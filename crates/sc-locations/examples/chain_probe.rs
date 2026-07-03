//! Resolve each `locationSuperGUID` containment-chain component against the DCB
//! record set and the StarMapObject set. Answers: is the marker bound to the
//! main StarMapObject, or a sub-entity? Does the chain contain the StarMapObject
//! GUID (giving an offline hash->name bridge for the historical 199)?
//!
//! ```bash
//! cargo run -p sc-locations --release --example chain_probe -- <triples.tsv>
//! ```
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, RecordPaths};
use sc_locations::{Locations, RecordCollection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("triples.tsv arg");
    let text = std::fs::read_to_string(&path)?;

    let mut rows: HashMap<u32, Vec<Vec<Guid>>> = HashMap::new();
    let mut name: HashMap<u32, String> = HashMap::new();
    for line in text.lines() {
        let mut it = line.split('\t');
        let (Some(h), Some(sg), nm) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let Ok(hash) = h.parse::<u32>() else { continue };
        let comps: Vec<Guid> = sg
            .split('.')
            .filter_map(|g| Guid::from_str(g).ok())
            .collect();
        if comps.is_empty() {
            continue;
        }
        rows.entry(hash).or_default().push(comps);
        name.entry(hash)
            .or_insert_with(|| nm.unwrap_or("").to_string());
    }

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locs = Locations::build(datacore.records());
    let paths = RecordPaths::build(&datacore);
    let locale = &asset_data.locale;

    // Per hash: constant components (present in EVERY row), in deepest-last order
    // from a representative row. Resolve each against DCB + StarMapObject.
    let mut star_bridge = 0usize; // chain contains the leaf StarMapObject GUID
    let mut dcb_any = 0usize;
    let mut samples = 0usize;
    // position-from-end -> how many hashes have a DCB record / StarMapObject there
    let mut pos_star: HashMap<i64, usize> = HashMap::new();
    let mut pos_dcb: HashMap<i64, usize> = HashMap::new();
    let mut pos_total: HashMap<i64, usize> = HashMap::new();

    for (&hash, vecs) in &rows {
        let nrows = vecs.len();
        let mut count: HashMap<Guid, usize> = HashMap::new();
        for v in vecs {
            for g in v.iter().collect::<HashSet<_>>() {
                *count.entry(*g).or_default() += 1;
            }
        }
        let rep = &vecs[0];
        let mut found_star = false;
        let mut found_dcb = false;
        let mut chain_repr = Vec::new();
        for (i, g) in rep.iter().enumerate() {
            let pos = -((rep.len() as i64) - 1 - i as i64) - 1; // -1 deepest
            let constant = count.get(g).copied().unwrap_or(0) == nrows;
            let loc = locs.get(g);
            let rec = paths.get(g);
            *pos_total.entry(pos).or_default() += 1;
            if loc.is_some() && constant {
                *pos_star.entry(pos).or_default() += 1;
            }
            if rec.is_some() && constant {
                *pos_dcb.entry(pos).or_default() += 1;
            }
            if loc.is_some() && constant {
                found_star = true;
            }
            if rec.is_some() && constant {
                found_dcb = true;
            }
            let tag = if let Some(l) = loc {
                format!(
                    "STAR[{}]={:?}",
                    l.kind.as_dcb_str(),
                    l.display_name(locale).unwrap_or("")
                )
            } else if let Some(r) = rec {
                format!("REC[{}]={:?}", r.struct_index, r.name)
            } else {
                "??".into()
            };
            chain_repr.push(format!("{}{}", if constant { "" } else { "~" }, tag));
        }
        if found_star {
            star_bridge += 1;
        }
        if found_dcb {
            dcb_any += 1;
        }
        if samples < 22 {
            samples += 1;
            eprintln!(
                "hash {hash:<11} {:?}\n   chain(deepest->root): {}",
                name.get(&hash),
                chain_repr
                    .iter()
                    .rev()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("  >  ")
            );
        }
    }

    eprintln!("\n==== SUMMARY ====");
    eprintln!("hashes: {}", rows.len());
    eprintln!("chain contains the leaf StarMapObject (constant): {star_bridge}");
    eprintln!("chain contains any DCB record (constant): {dcb_any}");
    let fmt = |m: &HashMap<i64, usize>| {
        let mut v: Vec<_> = m.iter().map(|(k, c)| (*k, *c)).collect();
        v.sort();
        v
    };
    eprintln!("pos-from-end totals:        {:?}", fmt(&pos_total));
    eprintln!("pos-from-end is-StarMapObj: {:?}", fmt(&pos_star));
    eprintln!("pos-from-end is-DCB-record: {:?}", fmt(&pos_dcb));
    Ok(())
}
