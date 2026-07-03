//! Exhaustive socpak sweep for the location-CRC preimage. Across the Stanton
//! body object-containers, harvest every GUID / id-string / numeric attr, test
//! them all (class_crc, crc32c, crc32-IEEE, literal) against the location-CRC
//! target set, and dump the full attribute inventory (to spot undecoded fields).
//!
//! ```bash
//! cargo run -p sc-locations --release --example socpak_mine -- <targets.txt> [needle]
//! ```
use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

use sc_extract::object_container::{Socpak, decode};
use sc_extract::{AssetSource, Guid, class_crc};

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
fn do_crc(b: &[u8], t: &[u32; 256]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &x in b {
        c = t[((c ^ x as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}
fn as_guid(s: &str) -> Option<Guid> {
    Guid::from_str(s.trim().trim_matches(['{', '}'])).ok()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let targets: HashSet<u32> = std::fs::read_to_string(&args[0])?
        .lines()
        .filter_map(|l| l.split('\t').next().unwrap_or("").trim().parse().ok())
        .collect();
    let needle = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "/system/stanton/".into());
    eprintln!("targets: {}  needle: {needle:?}", targets.len());

    let c_tbl = table(0x82F6_3B78); // crc32c (Castagnoli)
    let i_tbl = table(0xEDB8_8320); // crc32-IEEE

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    // Body OCs: exactly one path segment under the needle (skip deep outpost sub-OCs for speed).
    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak")
                && l.find(&needle)
                    .map(|i| !l[i + needle.len()..].contains('/'))
                    .unwrap_or(false)
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("matched {} body-OC socpaks:", socpaks.len());
    for s in &socpaks {
        eprintln!("  {s}");
    }

    let mut guids: HashSet<Guid> = HashSet::new();
    let mut idstrings: HashSet<String> = HashSet::new();
    let mut numbers: HashSet<u32> = HashSet::new();
    let mut attr_inv: BTreeMap<String, usize> = BTreeMap::new();
    let mut num_attr_inv: BTreeMap<String, usize> = BTreeMap::new();
    let idkeys: HashSet<&str> = [
        "Name",
        "name",
        "pointName",
        "starmapRecord",
        "template",
        "actionArea",
        "locationActionArea",
        "objectContainer",
        "string",
        "tag",
        "TagId",
        "EntityClass",
        "entityClusterId",
    ]
    .into_iter()
    .collect();

    for (n, sp) in socpaks.iter().enumerate() {
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        eprintln!("[{}/{}] {sp} ({} members)", n + 1, socpaks.len(), pak.len());
        for i in 0..pak.len() {
            let nm = pak.name(i).unwrap_or_default().to_ascii_lowercase();
            if !(nm.ends_with(".soc") || nm.ends_with(".pla") || nm.ends_with(".entxml")) {
                continue;
            }
            let Ok(b) = pak.read(i) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            for node in root.descendants() {
                for (k, v) in &node.attrs {
                    *attr_inv.entry(format!("{}@{}", node.tag, k)).or_default() += 1;
                    if let Some(g) = as_guid(v) {
                        guids.insert(g);
                    } else if let Ok(num) = v.parse::<u64>() {
                        if num <= u32::MAX as u64 {
                            numbers.insert(num as u32);
                            *num_attr_inv
                                .entry(format!("{}@{}", node.tag, k))
                                .or_default() += 1;
                        }
                    }
                    if idkeys.contains(k.as_str()) && v.len() < 200 {
                        idstrings.insert(v.clone());
                    }
                }
            }
        }
    }
    eprintln!(
        "\nharvested: {} guids, {} id-strings, {} numbers",
        guids.len(),
        idstrings.len(),
        numbers.len()
    );

    let mut hits: Vec<String> = Vec::new();
    for g in &guids {
        let c = class_crc(g);
        if targets.contains(&c) {
            hits.push(format!("class_crc={c} <- GUID {g}"));
        }
    }
    for n in &numbers {
        if targets.contains(n) {
            hits.push(format!("LITERAL {n} present as a numeric attr value"));
        }
    }
    for s in &idstrings {
        let after = s.rsplit(['.', '/']).next().unwrap_or(s).to_string();
        for v in [
            s.clone(),
            s.to_lowercase(),
            s.to_uppercase(),
            s.trim_start_matches('@').to_lowercase(),
            after.clone(),
            after.to_lowercase(),
        ] {
            let b = v.as_bytes();
            let cc = do_crc(b, &c_tbl);
            if targets.contains(&cc) {
                hits.push(format!("crc32c({v:?})={cc}"));
            }
            let ci = do_crc(b, &i_tbl);
            if targets.contains(&ci) {
                hits.push(format!("crc32ieee({v:?})={ci}"));
            }
        }
    }
    eprintln!("\n==== HITS: {} ====", hits.len());
    for h in &hits {
        eprintln!("  {h}");
    }
    eprintln!("\n==== numeric-valued attrs (candidate id/hash fields) ====");
    for (k, n) in &num_attr_inv {
        eprintln!("  {n:>7}  {k}");
    }
    eprintln!("\n==== full attr inventory ({} keys) ====", attr_inv.len());
    for (k, n) in &attr_inv {
        eprintln!("  {n:>7}  {k}");
    }
    Ok(())
}
