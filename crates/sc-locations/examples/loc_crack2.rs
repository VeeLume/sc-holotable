//! Targeted crack: with the confirmed pair DataSet-CRC `1991385875` ↔
//! `StarMapObject.RR_ARC_L1` (and `950566234` ↔ Rayari Kaltag), walk the full
//! StarMapObject HIERARCHY (the location + its parent chain — the Lagrange/station
//! separation the client clearly understands) and test every guid + name + a set
//! of composites against the mission DataSet location CRCs.
//!
//! ```bash
//! cargo run -p sc-locations --release --example loc_crack2 -- <loc_targets.txt> <loc_resolved.tsv>
//! ```
use std::collections::{HashMap, HashSet};

use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, LocaleMap, class_crc};
use sc_locations::Locations;

fn table(poly: u32) -> [u32; 256] {
    let mut t = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ poly } else { c >> 1 };
        }
        t[i as usize] = c;
    }
    t
}
fn crc(b: &[u8], t: &[u32; 256]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &x in b {
        c = t[((c ^ x as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let targets: HashSet<u32> = std::fs::read_to_string(&args[0])?
        .lines()
        .filter_map(|l| l.split('\t').next()?.trim().parse().ok())
        .collect();
    let names: HashMap<u32, String> = std::fs::read_to_string(&args[1])?
        .lines()
        .filter_map(|l| {
            let mut it = l.split('\t');
            Some((
                it.next()?.trim().parse().ok()?,
                it.next().unwrap_or("").to_string(),
            ))
        })
        .collect();
    let lbl = |t: u32| names.get(&t).cloned().unwrap_or_else(|| "?".into());
    let c_tbl = table(0x82F6_3B78); // crc32c
    let i_tbl = table(0xEDB8_8320); // crc32-IEEE

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let locale: &LocaleMap = &asset_data.locale;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let locs = Locations::build(store);

    let mut hits: Vec<String> = Vec::new();
    let mut test_str = |s: &str, ctx: &str, hits: &mut Vec<String>| {
        if s.is_empty() {
            return;
        }
        for v in [
            s.to_string(),
            s.to_lowercase(),
            s.to_uppercase(),
            s.trim_start_matches('@').to_string(),
            s.trim_start_matches('@').to_lowercase(),
        ] {
            for (fnname, t) in [
                ("crc32c", crc(v.as_bytes(), &c_tbl)),
                ("ieee", crc(v.as_bytes(), &i_tbl)),
            ] {
                if targets.contains(&t) {
                    hits.push(format!("{fnname}({v:?}) = {t}  [{}]  <{ctx}>", lbl(t)));
                }
            }
        }
    };
    let test_guid = |g: &Guid, ctx: &str, hits: &mut Vec<String>| {
        let cc = class_crc(g);
        if targets.contains(&cc) {
            hits.push(format!("class_crc({g}) = {cc}  [{}]  <{ctx}>", lbl(cc)));
        }
        let b = *g.as_bytes();
        let mut rb = b;
        rb.reverse();
        for (form, by) in [("bytes", &b[..]), ("rev", &rb[..])] {
            for (fnname, t) in [("crc32c", crc(by, &c_tbl)), ("ieee", crc(by, &i_tbl))] {
                if targets.contains(&t) {
                    hits.push(format!("{fnname}({form} {g}) = {t}  [{}]  <{ctx}>", lbl(t)));
                }
            }
        }
    };

    // RR_ARC_L1 (DataSet 1991385875) via its known class_crc (field-19 = 979247948).
    let Some(start) = locs.by_crc(979247948) else {
        eprintln!("RR_ARC_L1 not found by class_crc 979247948");
        return Ok(());
    };
    let label = "RR_ARC_L1 (DataSet 1991385875)";
    eprintln!("=== {label} ===");
    // Chain = the location itself + its ancestors (Lagrange / planet / system).
    let mut chain: Vec<&sc_locations::Location> = vec![start];
    chain.extend(locs.ancestors(&start.guid));

    for (depth, loc) in chain.iter().enumerate() {
        let nm = loc.display_name(locale).unwrap_or("<no name>");
        eprintln!("  [{depth}] {}  {nm:?}  kind={:?}", loc.guid, loc.kind);
        test_guid(&loc.guid, &format!("depth{depth} guid"), &mut hits);
        test_str(nm, &format!("depth{depth} name"), &mut hits);
        if let Some(k) = &loc.name_key {
            test_str(k.as_ref(), &format!("depth{depth} name_key"), &mut hits);
        }
        for ck in &loc.callouts {
            test_str(ck.as_ref(), &format!("depth{depth} callout"), &mut hits);
        }
        if let Some(j) = loc.jurisdiction {
            test_guid(&j, &format!("depth{depth} jurisdiction"), &mut hits);
        }
    }
    // Composites across adjacent hierarchy levels (station + Lagrange/parent).
    for w in chain.windows(2) {
        let (child, parent) = (w[0], w[1]);
        let cn = child.display_name(locale).unwrap_or("");
        let pn = parent.display_name(locale).unwrap_or("");
        for comp in [
            format!("{pn} {cn}"),
            format!("{pn}{cn}"),
            format!("{pn}_{cn}"),
            format!("{cn} {pn}"),
            format!("{cn}_{pn}"),
        ] {
            test_str(&comp, "composite name", &mut hits);
        }
        let mut cat = child.guid.as_bytes().to_vec();
        cat.extend_from_slice(parent.guid.as_bytes());
        for (fnname, t) in [("crc32c", crc(&cat, &c_tbl)), ("ieee", crc(&cat, &i_tbl))] {
            if targets.contains(&t) {
                hits.push(format!(
                    "{fnname}(child++parent guids) = {t}  [{}]  <composite guid>",
                    lbl(t)
                ));
            }
        }
    }

    println!("\n==== HITS: {} ====", hits.len());
    let mut seen = HashSet::new();
    for h in &hits {
        if seen.insert(h.clone()) {
            println!("  {h}");
        }
    }
    Ok(())
}
