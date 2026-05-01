//! Three investigations on ship-spawn descriptions:
//!
//! 1. **`SpawnDescription_Ship.name_property` distribution.** What does
//!    the recursive `Handle<MissionProperty>` field point at? Walk every
//!    instance in the pool, classify the value-pointer kind, sample a
//!    few non-empty cases.
//! 2. **F7C_Hornet broken-tag trace.** Find the specific slot the user
//!    spotted with `other: F7C_Hornet` + `candidates=0`. Print the
//!    tag's tag-tree path so we can see why it landed in the `other`
//!    bucket and what namespace it lives in.
//! 3. **Empty-candidate census.** Walk every ship encounter slot, count
//!    those whose resolved candidate list is empty, group by contract
//!    family + variable_name. Drives in-game validation: which missions
//!    are likely to have a non-spawning slot.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example spawn_dig
//! ```

use std::collections::BTreeMap;

use sc_contracts::{Encounter, MissionIndex, raw};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore, &asset_data.locale);

    section_1_name_property(&datacore);
    section_2_f7c_hornet_trace(&index);
    section_3_empty_candidate_census(&index);

    Ok(())
}

// ── §1 SpawnDescription_Ship.name_property distribution ──────────────────────

fn section_1_name_property(datacore: &Datacore) {
    println!("=== §1 SpawnDescription_Ship.name_property distribution ===");
    let pools = &datacore.records().pools;
    let db = datacore.db();

    let mut total = 0usize;
    let mut with_name_prop = 0usize;
    let mut value_kinds: BTreeMap<&'static str, usize> = BTreeMap::new();

    // For sample dumps:
    let mut sample_ainame: Vec<(String, String, String)> = Vec::new(); // (var_name, ext_token, name)
    let mut sample_other: Vec<(String, String, &'static str)> = Vec::new();

    for sd in pools.multi_feature.spawn_description_ship.iter().flatten() {
        total += 1;
        let Some(handle) = sd.name_property.as_ref() else {
            continue;
        };
        with_name_prop += 1;
        let Some(prop) = handle.get(pools) else {
            *value_kinds.entry("<unresolved handle>").or_default() += 1;
            continue;
        };
        let kind = match &prop.value {
            None => "<no value>",
            Some(ptr) => poly_kind(ptr),
        };
        *value_kinds.entry(kind).or_default() += 1;

        // Sample collection — dig the actual content for AIName since
        // that's the most commonly-claimed semantic.
        if kind == "AIName" {
            // Walk raw to extract the .name field on the AIName value.
            let raw_inst = db_instance_for_mission_property(db, prop, datacore);
            if let Some(name) = raw_inst.and_then(|i| {
                i.get_instance("value").and_then(|v| v.get_str("name").map(String::from))
            }) {
                if sample_ainame.len() < 8 {
                    sample_ainame.push((
                        prop.mission_variable_name.clone(),
                        prop.extended_text_token.clone(),
                        name,
                    ));
                }
            }
        } else if kind != "<no value>" && sample_other.len() < 6 {
            sample_other.push((
                prop.mission_variable_name.clone(),
                prop.extended_text_token.clone(),
                kind,
            ));
        }
    }

    println!("  total SpawnDescription_Ship: {total}");
    println!(
        "  with name_property set:      {with_name_prop} ({:.1}%)",
        100.0 * with_name_prop as f32 / total.max(1) as f32
    );
    println!("  value-kind distribution:");
    let mut entries: Vec<_> = value_kinds.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for (k, c) in &entries {
        println!("    {c:>5}  {k}");
    }
    if !sample_ainame.is_empty() {
        println!("  sample AIName values:");
        for (var, ext, name) in &sample_ainame {
            println!("    var=\"{var}\" ext=\"{ext}\" name=\"{name}\"");
        }
    }
    if !sample_other.is_empty() {
        println!("  sample non-AIName name_property values:");
        for (var, ext, kind) in &sample_other {
            println!("    var=\"{var}\" ext=\"{ext}\" value_kind={kind}");
        }
    }
    println!();
}

fn poly_kind(p: &sc_extract::generated::BaseMissionPropertyValuePtr) -> &'static str {
    use sc_extract::generated::BaseMissionPropertyValuePtr as P;
    match p {
        P::BaseMissionPropertyValue(_) => "Base",
        P::MissionPropertyValue_AIName(_) => "AIName",
        P::MissionPropertyValue_Boolean(_) => "Boolean",
        P::MissionPropertyValue_Integer(_) => "Integer",
        P::MissionPropertyValue_Float(_) => "Float",
        P::MissionPropertyValue_StringHash(_) => "StringHash",
        P::MissionPropertyValue_Object(_) => "Object",
        P::MissionPropertyValue_Tags(_) => "Tags",
        P::MissionPropertyValue_Location(_) => "Location",
        P::MissionPropertyValue_Locations(_) => "Locations",
        P::MissionPropertyValue_Organization(_) => "Organization",
        P::MissionPropertyValue_MissionItem(_) => "MissionItem",
        P::MissionPropertyValue_CombinedDataSetEntries(_) => "CombinedDataSetEntries",
        P::MissionPropertyValue_ShipSpawnDescriptions(_) => "ShipSpawnDescriptions",
        P::MissionPropertyValue_NPCSpawnDescriptions(_) => "NPCSpawnDescriptions",
        P::MissionPropertyValue_EntitySpawnDescriptions(_) => "EntitySpawnDescriptions",
        P::MissionPropertyValue_TimeTrialRace(_) => "TimeTrialRace",
        P::MissionPropertyValue_HaulingOrders(_) => "HaulingOrders",
        P::MissionPropertyValue_Reward(_) => "Reward",
        P::MissionPropertyValue_DeliveryOrder(_) => "DeliveryOrder",
        P::MissionPropertyValue_StarMapLocation(_) => "StarMapLocation",
        P::Unknown { .. } => "Unknown",
    }
}

/// Best-effort raw lookup of the underlying svarog Instance for a
/// MissionProperty (which we need to read AIName.name since the
/// generator drops record-name fields). Returns None when the type
/// isn't directly reachable as a top-level record (most cases).
fn db_instance_for_mission_property<'a>(
    _db: &'a raw::DataCoreDatabase,
    _prop: &sc_extract::generated::MissionProperty,
    _datacore: &'a Datacore,
) -> Option<raw::Instance<'a>> {
    // MissionProperty isn't a top-level record (it's nested inside a
    // contract's paramOverrides). The generated MissionProperty struct
    // gave us the variable name but not the inline value's name field.
    // Skipped here; AIName resolution would need a parallel raw walk.
    None
}

// ── §2 F7C_Hornet broken-tag trace ───────────────────────────────────────────

fn section_2_f7c_hornet_trace(index: &MissionIndex) {
    println!("=== §2 F7C_Hornet broken-tag trace ===");
    let needle = "F7C_Hornet";
    let tree = &index.tag_tree;

    // Tag tree lookup: where does F7C_Hornet live?
    let tag_guids = tree.by_name(needle);
    if tag_guids.is_empty() {
        println!("  no tag named '{needle}' in the tree.");
    } else {
        println!("  '{needle}' lives at:");
        for g in tag_guids {
            let path = tree.path(g);
            println!("    {} :: {}", g, path.join(" ▸ "));
        }
    }
    println!();

    // Find every slot that has F7C_Hornet anywhere (positive or negative).
    let mut hits = 0usize;
    let mut sample: Vec<(String, String, String, usize, &'static str)> = Vec::new();
    for c in &index.contracts {
        for enc in &c.encounters {
            if let Encounter::Ships(s) = enc {
                for phase in &s.phases {
                    for (slot_idx, slot) in phase.slots.iter().enumerate() {
                        for (bag_label, bag) in [
                            ("positive", &slot.positive),
                            ("negative", &slot.negative),
                            ("markup", &slot.markup),
                            ("entity", &slot.entity),
                        ] {
                            if bag.contains_name(needle) {
                                hits += 1;
                                if sample.len() < 12 {
                                    sample.push((
                                        c.debug_name.clone(),
                                        s.variable_name.clone(),
                                        phase.name.clone(),
                                        slot_idx,
                                        bag_label,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("  '{needle}' tag appears in {hits} slot(s) across all bags.");
    for (dn, var, phase, slot_idx, where_) in &sample {
        println!("    [{where_}] {dn} → {var} / phase \"{phase}\" / slot {slot_idx}");
    }
    println!();
}

// ── §3 Empty-candidate census ────────────────────────────────────────────────

fn section_3_empty_candidate_census(index: &MissionIndex) {
    println!("=== §3 Ship-encounter slots with candidates=0 ===");

    let mut total_slots = 0usize;
    let mut empty_slots = 0usize;

    // (mission_family, variable_name) → count
    let mut by_family_var: BTreeMap<(String, String), usize> = BTreeMap::new();
    // For each (family, var) cluster, surface a sample mission that hits it
    let mut sample_per_family_var: BTreeMap<(String, String), (String, Vec<String>)> =
        BTreeMap::new();

    for c in &index.contracts {
        for enc in &c.encounters {
            if let Encounter::Ships(s) = enc {
                for phase in &s.phases {
                    for slot in &phase.slots {
                        total_slots += 1;
                        if slot.candidates.is_empty() {
                            empty_slots += 1;
                            let key = (family_prefix(&c.debug_name), s.variable_name.clone());
                            *by_family_var.entry(key.clone()).or_default() += 1;
                            sample_per_family_var
                                .entry(key)
                                .and_modify(|(_, names)| {
                                    if names.len() < 3 {
                                        names.push(slot_first_positive(slot));
                                    }
                                })
                                .or_insert_with(|| {
                                    (c.debug_name.clone(), vec![slot_first_positive(slot)])
                                });
                        }
                    }
                }
            }
        }
    }

    println!(
        "  empty / total: {empty_slots} / {total_slots} ({:.1}%)",
        100.0 * empty_slots as f32 / total_slots.max(1) as f32
    );
    println!();
    println!("  by mission family × variable_name (top 30):");
    let mut entries: Vec<_> = by_family_var.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for ((family, var), count) in entries.iter().take(30) {
        let sample = sample_per_family_var
            .get(&(family.clone(), var.clone()))
            .map(|(dn, _)| dn.as_str())
            .unwrap_or("?");
        println!("    {count:>4}  {family} / {var}");
        println!("           sample contract: {sample}");
    }
    if entries.len() > 30 {
        println!("  ... +{} more (family, var) combinations", entries.len() - 30);
    }
}

/// "First positive tag" snapshot for diagnostic display — gives the
/// reader a sense of *what the slot was trying to find*.
fn slot_first_positive(slot: &sc_contracts::ShipSlot) -> String {
    let names: Vec<&str> = slot.positive.names.iter().map(|s| s.as_str()).collect();
    if names.is_empty() {
        "(no positive tags)".to_string()
    } else {
        names.join(",")
    }
}

/// Trim difficulty / region suffixes for family grouping.
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
            ) || matches!(**p, "Stanton" | "Pyro" | "Nyx")
                || p.chars().all(char::is_numeric)
        })
        .count();
    parts[..parts.len() - drop_tail].join("_")
}
