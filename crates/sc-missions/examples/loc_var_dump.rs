//! Decode the mission location-VARIABLE declaration surface and test it against
//! the mission DataSet location CRCs — the structure sc-missions never modeled.
//!
//! Walks `VariableDeclaration_LocationEntities` (subsumptionVariableName + the
//! candidate `locationEntities` pool) and `ModuleLocationEntities_Static`
//! (debugName + entityDeclarations GUIDs), prints samples, and tests every string
//! (crc32c + crc32-IEEE, ±case) + every entityDeclaration GUID (class_crc) against
//! the 209 DataSet CRC targets. Confirmed pairs to watch: 1991385875 (ARC-L1,
//! bracket RestStop_Stanton_3_L1) and 950566234 (Rayari Kaltag).
//!
//! ```bash
//! cargo run -p sc-missions --release --example loc_var_dump -- <loc_targets.txt> [loc_resolved.tsv]
//! ```
use std::collections::{HashMap, HashSet};

use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, class_crc};

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
    let names: HashMap<u32, String> = args
        .get(1)
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|t| {
            t.lines()
                .filter_map(|l| {
                    let mut it = l.split('\t');
                    Some((
                        it.next()?.trim().parse().ok()?,
                        it.next().unwrap_or("").to_string(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    let lbl = |t: u32| names.get(&t).cloned().unwrap_or_else(|| "?".into());
    let c_tbl = table(0x82F6_3B78);
    let i_tbl = table(0xEDB8_8320);
    eprintln!("targets: {}", targets.len());

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let pools = &store.pools;

    let mut strings: HashSet<String> = HashSet::new();
    let mut guids: HashSet<[u8; 16]> = HashSet::new();

    // VariableDeclaration_LocationEntities — subsumptionVariableName.
    let mut subs: Vec<String> = Vec::new();
    for v in pools
        .missiondata
        .variable_declaration_location_entities
        .iter()
        .flatten()
    {
        if !v.subsumption_variable_name.is_empty() {
            subs.push(v.subsumption_variable_name.clone());
            strings.insert(v.subsumption_variable_name.clone());
        }
    }
    // ModuleLocationEntities_Static — debugName + entityDeclarations.
    let mut debugs: Vec<String> = Vec::new();
    let mut ndecl = 0usize;
    for m in pools
        .missiondata
        .module_location_entities_static
        .iter()
        .flatten()
    {
        if !m.debug_name.is_empty() {
            debugs.push(m.debug_name.clone());
            strings.insert(m.debug_name.clone());
        }
        for g in &m.entity_declarations {
            ndecl += 1;
            guids.insert(*g.as_bytes());
        }
    }

    eprintln!(
        "VariableDeclaration_LocationEntities: {} (subsumption names: {} distinct)",
        pools
            .missiondata
            .variable_declaration_location_entities
            .iter()
            .flatten()
            .count(),
        subs.iter().collect::<HashSet<_>>().len()
    );
    eprintln!(
        "ModuleLocationEntities_Static: {} (debug names: {} distinct, entityDeclarations: {})",
        pools
            .missiondata
            .module_location_entities_static
            .iter()
            .flatten()
            .count(),
        debugs.iter().collect::<HashSet<_>>().len(),
        ndecl
    );
    eprintln!("\n-- sample subsumptionVariableName --");
    for s in subs.iter().collect::<HashSet<_>>().into_iter().take(25) {
        eprintln!("    {s}");
    }
    eprintln!("-- sample debugName --");
    for s in debugs.iter().collect::<HashSet<_>>().into_iter().take(25) {
        eprintln!("    {s}");
    }

    // Test.
    let mut hits: Vec<String> = Vec::new();
    for s in &strings {
        let after = s.rsplit(['.', '/', '|']).next().unwrap_or(s).to_string();
        for v in [
            s.clone(),
            s.to_lowercase(),
            s.to_uppercase(),
            after.clone(),
            after.to_lowercase(),
        ] {
            let b = v.as_bytes();
            for (fnname, t) in [("crc32c", crc(b, &c_tbl)), ("ieee", crc(b, &i_tbl))] {
                if targets.contains(&t) {
                    hits.push(format!("{fnname}({v:?}) = {t}  [{}]", lbl(t)));
                }
            }
        }
    }
    for g in &guids {
        let guid = Guid::from_bytes(*g);
        let cc = class_crc(&guid);
        if targets.contains(&cc) {
            hits.push(format!(
                "class_crc(entityDecl {guid}) = {cc}  [{}]",
                lbl(cc)
            ));
        }
        for (fnname, t) in [
            ("crc32c", crc(&g[..], &c_tbl)),
            ("ieee", crc(&g[..], &i_tbl)),
        ] {
            if targets.contains(&t) {
                hits.push(format!(
                    "{fnname}(entityDecl bytes {guid}) = {t}  [{}]",
                    lbl(t)
                ));
            }
        }
    }

    eprintln!(
        "\n==== distinct strings tested: {}  guids: {} ====",
        strings.len(),
        guids.len()
    );
    println!("==== HITS: {} ====", hits.len());
    let mut seen = HashSet::new();
    for h in &hits {
        if seen.insert(h.clone()) {
            println!("  {h}");
        }
    }
    Ok(())
}
