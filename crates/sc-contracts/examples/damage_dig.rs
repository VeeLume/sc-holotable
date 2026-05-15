//! Investigate `initial_damage_settings` GUIDs on pre-damaged spawn
//! slots. Hypothesis: when a salvage / cargo-recovery slot reports
//! candidates=0 from the tag query, the engine actually instantiates
//! the wreck via the damage-settings record — which embeds the ship's
//! entity class directly. That would explain why the static spawn
//! query has zero candidates yet the mission spawns a salvage target
//! in-game.
//!
//! Walks every ship slot with a populated `initial_damage_settings`
//! GUID, resolves the underlying record via raw svarog, and reports
//! the record type + any `entityClass` / `entity_class` field on it.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example damage_dig
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_contracts::{Encounter, MissionIndex};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);
    let db = datacore.db();

    // Collect every distinct initial_damage_settings GUID + which
    // mission family / variable_name uses it.
    let mut by_guid: HashMap<Guid, Vec<(String, String, bool)>> = HashMap::new();
    for c in &index.contracts {
        for enc in &c.encounters {
            if let Encounter::Ships(s) = enc {
                for phase in &s.phases {
                    for slot in &phase.slots {
                        if let Some(g) = slot.initial_damage_settings {
                            by_guid.entry(g).or_default().push((
                                family_prefix(&c.debug_name),
                                s.variable_name.clone(),
                                slot.candidates.is_empty(),
                            ));
                        }
                    }
                }
            }
        }
    }

    println!(
        "=== {} distinct initial_damage_settings GUIDs across {} usages ===",
        by_guid.len(),
        by_guid.values().map(|v| v.len()).sum::<usize>()
    );
    println!();

    for (guid, usages) in &by_guid {
        let zero_count = usages.iter().filter(|(_, _, e)| *e).count();
        let total = usages.len();
        println!("── {guid}");

        // Resolve via raw svarog. Ask the database what record type this is
        // and whether the record has an entity_class / entityClass field.
        match db.record(guid) {
            None => {
                println!("    (no record for this GUID)");
            }
            Some(record) => {
                let inst = record.as_instance();
                let type_name = inst.type_name().unwrap_or("<unknown>");
                let record_name = record.name().unwrap_or("<unnamed>");
                println!("    record: {record_name} (type {type_name})");

                // Try to find an embedded entity-class reference.
                let mut found_entity = None;
                for field in &[
                    "entityClass",
                    "entity_class",
                    "shipReference",
                    "shipEntity",
                    "vehicle",
                    "entity",
                ] {
                    if let Some(v) = inst.get(field)
                        && let Some(rec_ref) = v.as_record_ref()
                    {
                        found_entity = Some((field.to_string(), rec_ref.guid));
                        break;
                    }
                }
                if let Some((field, ent_guid)) = &found_entity {
                    let ent_name = db
                        .record(ent_guid)
                        .and_then(|r| r.name().map(String::from))
                        .unwrap_or_else(|| "<unresolved>".to_string());
                    println!("    {field}: {ent_guid} ({ent_name})");
                } else {
                    let mut fields: Vec<String> =
                        inst.properties().map(|p| p.name.to_string()).collect();
                    fields.sort();
                    fields.dedup();
                    println!("    fields: {}", fields.join(", "));
                }
            }
        }

        println!("    used by {total} slot(s); {zero_count} have candidates=0");
        let mut family_var: BTreeMap<(String, String), (usize, usize)> = BTreeMap::new();
        for (fam, var, empty) in usages {
            let entry = family_var.entry((fam.clone(), var.clone())).or_default();
            entry.0 += 1;
            if *empty {
                entry.1 += 1;
            }
        }
        let mut entries: Vec<_> = family_var.iter().collect();
        entries.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        for ((fam, var), (cnt, empty)) in entries.iter().take(6) {
            println!("      {cnt:>4} ({empty} empty)  {fam} / {var}");
        }
        if entries.len() > 6 {
            println!("      ... +{} more", entries.len() - 6);
        }
        println!();
    }

    Ok(())
}

fn family_prefix(s: &str) -> String {
    let parts: Vec<&str> = s.split('_').collect();
    if parts.len() <= 2 {
        return s.to_string();
    }
    let drop_tail = parts
        .iter()
        .rev()
        .take_while(|p| {
            matches!(
                **p,
                "VeryEasy"
                    | "Easy"
                    | "Moderate"
                    | "Medium"
                    | "Hard"
                    | "VeryHard"
                    | "Extreme"
                    | "Super"
                    | "Hardest"
                    | "Rehire"
                    | "Intro"
            ) || matches!(**p, "Stanton" | "Pyro" | "Nyx")
                || p.chars().all(char::is_numeric)
        })
        .count();
    parts[..parts.len() - drop_tail].join("_")
}
