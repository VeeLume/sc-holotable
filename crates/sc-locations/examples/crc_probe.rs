//! Probe: does the dead log's `locationHash` == class_crc of one of its
//! `locationSuperGUID` containment-chain components? And does that GUID resolve
//! to a DCB record / StarMapObject?
//!
//! Input: a TSV `hash \t superguid(dot-joined) \t name` (the historical seed).
//! ```bash
//! cargo run -p sc-locations --release --example crc_probe -- <triples.tsv>
//! ```
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, RecordPaths, class_crc};
use sc_locations::{Locations, RecordCollection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("triples.tsv arg");
    let text = std::fs::read_to_string(&path)?;

    // hash -> list of component-vectors (one per row)
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
    eprintln!("distinct hashes: {}", rows.len());

    // Build holotable indices for resolution.
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locs = Locations::build(datacore.records());
    let paths = RecordPaths::build(&datacore);
    let locale = &asset_data.locale;

    let mut matched = 0usize;
    let mut pos_hist: HashMap<i64, usize> = HashMap::new(); // position from END (-1 = last)
    let mut in_dcb = 0usize;
    let mut is_location = 0usize;
    let mut samples = 0usize;

    for (&hash, vecs) in &rows {
        // distinct GUIDs that appear in EVERY row (constant), with their positions
        // (count from end so chains of different length align on the deepest).
        let nrows = vecs.len();
        let mut count: HashMap<Guid, usize> = HashMap::new();
        for v in vecs {
            let mut seen = HashSet::new();
            for g in v {
                if seen.insert(*g) {
                    *count.entry(*g).or_default() += 1;
                }
            }
        }
        // find the GUID whose class_crc == hash
        let mut hit: Option<(Guid, i64, bool)> = None; // (guid, pos_from_end, constant)
        for v in vecs {
            for (i, g) in v.iter().enumerate() {
                if class_crc(g) == hash {
                    let pos_from_end = (v.len() as i64) - 1 - (i as i64);
                    let constant = count.get(g).copied().unwrap_or(0) == nrows;
                    hit = Some((*g, -(pos_from_end + 1), constant));
                    break;
                }
            }
            if hit.is_some() {
                break;
            }
        }
        if let Some((g, pos, constant)) = hit {
            matched += 1;
            *pos_hist.entry(pos).or_default() += 1;
            let rec = paths.get(&g);
            let loc = locs.get(&g);
            if rec.is_some() {
                in_dcb += 1;
            }
            if loc.is_some() {
                is_location += 1;
            }
            if samples < 25 {
                samples += 1;
                let rinfo = rec
                    .map(|r| {
                        format!(
                            "REC name={:?} type_idx={} path={}",
                            r.name, r.struct_index, r.path
                        )
                    })
                    .unwrap_or_else(|| "NOT-a-DCB-record".into());
                let linfo = loc.and_then(|l| l.display_name(locale)).unwrap_or("");
                eprintln!(
                    "hash {hash:<11} pos{pos} const={constant} guid={g}\n   name={:?}\n   {rinfo}  loc_name={linfo:?}",
                    name.get(&hash)
                );
            }
        } else if samples < 25 {
            // no component matched — report the constant deepest GUID for inspection
            eprintln!(
                "hash {hash:<11} NO-COMPONENT-MATCH  name={:?}",
                name.get(&hash)
            );
        }
    }

    eprintln!("\n==== SUMMARY ====");
    eprintln!("hashes total: {}", rows.len());
    eprintln!("hashes where class_crc(some superGUID component) == hash: {matched}");
    eprintln!("  of those, matched GUID IS a DCB record: {in_dcb}");
    eprintln!("  of those, matched GUID IS a StarMapObject location: {is_location}");
    let mut ph: Vec<_> = pos_hist.into_iter().collect();
    ph.sort();
    eprintln!("position-from-end histogram (-1=deepest/last component): {ph:?}");
    Ok(())
}
