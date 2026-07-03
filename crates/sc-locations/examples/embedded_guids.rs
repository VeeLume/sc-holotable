//! Harvest every embedded `Value::Guid` (raw 16-byte GUID values) AND every
//! `Value::Reference` target GUID from EVERY DCB record's full instance tree,
//! then class_crc each against the mission location_id target set.
//!
//! Rationale: the proto says location_id = "crc of DF records guid". Top-level
//! record GUIDs (115k) all fail class_crc. But `Value::Guid` fields embedded
//! INSIDE records are a SEPARATE set — they are NOT enumerated record ids, NOT
//! socpak ASCII guids, and the ReferenceGraph deliberately ignores them. They
//! can point at object-container / entity GUIDs that live outside the DCB.
//!
//! ```bash
//! cargo run -p sc-locations --release --example embedded_guids -- <loc_targets.txt>
//! ```
use std::collections::{HashMap, HashSet};

use sc_extract::{
    AssetConfig, AssetData, AssetSource, DataCoreDatabase, Guid, Instance, RecordPaths, Value,
    class_crc,
};

/// Walk one instance tree, pushing every raw Guid + Reference target into `out`,
/// tagged with the field name where it was found.
fn walk<'a>(db: &'a DataCoreDatabase, root: Instance<'a>, out: &mut Vec<(Guid, String)>) {
    let mut worklist: Vec<Instance<'a>> = vec![root];
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    while let Some(inst) = worklist.pop() {
        for prop in inst.properties() {
            match prop.value {
                Value::Guid(g) => out.push((g, prop.name.to_string())),
                Value::Reference(Some(r)) => out.push((r.guid, format!("ref:{}", prop.name))),
                Value::Class { struct_index, data } => {
                    worklist.push(Instance::from_inline_data(db, struct_index, data));
                }
                Value::ClassRef(r)
                | Value::StrongPointer(Some(r))
                | Value::WeakPointer(Some(r)) => {
                    if visited.insert((r.struct_index, r.instance_index)) {
                        worklist.push(db.instance(r.struct_index, r.instance_index));
                    }
                }
                Value::Array(_) => {
                    if let Some(arr) = inst.get_array(prop.name) {
                        for elem in arr {
                            match elem {
                                Value::Guid(g) => out.push((g, prop.name.to_string())),
                                Value::Reference(Some(r)) => {
                                    out.push((r.guid, format!("ref:{}", prop.name)))
                                }
                                Value::Class { struct_index, data } => {
                                    worklist.push(Instance::from_inline_data(
                                        db,
                                        struct_index,
                                        data,
                                    ));
                                }
                                Value::ClassRef(r)
                                | Value::StrongPointer(Some(r))
                                | Value::WeakPointer(Some(r)) => {
                                    if visited.insert((r.struct_index, r.instance_index)) {
                                        worklist
                                            .push(db.instance(r.struct_index, r.instance_index));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets_path = std::env::args().nth(1).expect("loc_targets.txt arg");
    let targets: HashSet<u32> = std::fs::read_to_string(&targets_path)?
        .lines()
        .filter_map(|l| {
            l.split('\t')
                .next()
                .and_then(|s| s.trim().parse::<u32>().ok())
        })
        .collect();
    eprintln!("targets: {}", targets.len());

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();
    let paths = RecordPaths::build(&datacore);

    let mut all_guids: HashMap<Guid, (String, String)> = HashMap::new(); // guid -> (record_name, field)
    let mut total = 0usize;
    for record in db.all_records() {
        let rec_name = record.name().unwrap_or("?").to_string();
        let mut found = Vec::new();
        walk(db, record.as_instance(), &mut found);
        for (g, field) in found {
            total += 1;
            all_guids.entry(g).or_insert((rec_name.clone(), field));
        }
    }
    eprintln!(
        "embedded GUID occurrences: {total}; distinct: {}",
        all_guids.len()
    );

    // How many distinct embedded guids are NOT themselves top-level records?
    let mut non_record = 0usize;
    let mut hits = 0usize;
    for (g, (rec, field)) in &all_guids {
        let is_rec = paths.get(g).is_some();
        if !is_rec {
            non_record += 1;
            // Dump non-record embedded guids to stdout for the offline battery.
            println!("{g}\t{rec}\t{field}");
        }
        let crc = class_crc(g);
        if targets.contains(&crc) {
            hits += 1;
            eprintln!("HIT crc={crc} guid={g} in record={rec} field={field}");
        }
    }
    eprintln!("distinct embedded guids NOT a top-level record: {non_record}");
    eprintln!("class_crc HITS against location targets: {hits}");
    Ok(())
}
