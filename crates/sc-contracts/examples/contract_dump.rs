//! Dump everything we know about one specific mission, including
//! tag-tree paths, raw GUIDs, and registry resolutions. Filter by a
//! debug-name substring on the command line.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example contract_dump -- FoxwellEnforcement_ShipAmbush_Stanton_VeryEasy
//! ```

use sc_contracts::{Encounter, MissionIndex, raw};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid, TagTree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let needle: String = std::env::args()
        .nth(1)
        .ok_or("usage: contract_dump <debug_name_substring>")?;

    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore, &asset_data.locale);

    let needle_lower = needle.to_lowercase();
    let matches: Vec<&_> = index
        .contracts
        .iter()
        .filter(|c| c.debug_name.to_lowercase().contains(&needle_lower))
        .collect();

    if matches.is_empty() {
        eprintln!("no contract whose debug_name contains '{needle}'");
        return Ok(());
    }
    println!("=== {} match(es) ===\n", matches.len());

    for c in &matches {
        println!("══════════════════════════════════════════════════════════════");
        println!("{}", c.debug_name);
        println!("══════════════════════════════════════════════════════════════");
        println!("  id:                {}", c.id);
        println!("  origin.kind:       {:?}", c.origin.kind);
        println!("  origin.source:     {}", c.origin.source_debug_name);
        println!("  origin.generator:  {}", c.origin.generator_id);
        if let Some(parent) = c.origin.subcontract_of {
            println!("  origin.subcontract_of: {parent}");
        }
        println!("  title:             {:?}", c.title);
        println!("  title_key:         {:?}", c.title_key.as_ref().map(|k| k.as_str()));
        println!("  description:       {:?}", c.description.as_deref());
        println!("  description_key:   {:?}", c.description_key.as_ref().map(|k| k.as_str()));
        println!("  shareable:         {}", c.shareable);
        println!("  illegal_flag:      {}", c.illegal_flag);
        println!("  availability:      {:?}", c.availability);
        println!("  rewards:           {:?}", c.rewards);
        println!("  prerequisites ({}):", c.prerequisites.len());
        for p in &c.prerequisites {
            println!("    - {p:?}");
        }
        if !c.mission_span.is_empty() {
            println!("  mission_span ({}):", c.mission_span.len());
            for g in &c.mission_span {
                if let Some(view) = index.localities.get(g) {
                    println!("    - {} :: {}", view.name, view.region_label);
                } else {
                    println!("    - <unresolved {g}>");
                }
            }
        }

        // Ship-pool sibling preview via the title-key pool.
        if let Some(key) = c.title_key.as_ref()
            && let Some(ids) = index.pools.title_key.get(key)
            && ids.len() > 1
        {
            println!("  title_key pool ({} member(s)):", ids.len());
            for &sib_id in ids.iter().take(8) {
                if sib_id == c.id {
                    continue;
                }
                if let Some(sib) = index.get(sib_id) {
                    println!("    - {} ({:?})", sib.debug_name, sib.origin.kind);
                }
            }
        }

        println!();
        println!("Encounters ({}):", c.encounters.len());
        for enc in &c.encounters {
            dump_encounter(enc, &index.tag_tree);
        }

        // Walk the raw MissionProperty graph attached to the contract +
        // its template/handler, so we can see properties that didn't make
        // it into the typed Encounter list.
        println!();
        println!("Raw MissionProperty walk:");
        dump_raw_properties(&datacore, c.id, &index.tag_tree);
        println!();
    }

    Ok(())
}

fn dump_encounter(enc: &Encounter, tree: &TagTree) {
    let kind = match enc {
        Encounter::Ships(_) => "ships",
        Encounter::Npcs(_) => "npcs",
        Encounter::Entities(_) => "entities",
        Encounter::Unknown { .. } => "unknown",
    };
    println!(
        "  ▸ [{kind}] {}  (ext_token={:?})",
        enc.variable_name(),
        enc.extended_text_token()
    );
    match enc {
        Encounter::Ships(s) => {
            for phase in &s.phases {
                println!("      phase {:?}, {} slot(s)", phase.name, phase.slots.len());
                for (i, slot) in phase.slots.iter().enumerate() {
                    println!(
                        "        slot {i}: concurrent={} weight={:.2} candidates={}",
                        slot.concurrent,
                        slot.weight,
                        slot.candidates.len()
                    );
                    if slot.initial_damage_settings.is_some() {
                        println!("          [pre-damaged]");
                    }
                    if slot.include_location_ai_spawn_tags {
                        println!("          [+location-tags]");
                    }
                    dump_bag("positive", &slot.positive, tree);
                    dump_bag("negative", &slot.negative, tree);
                    dump_bag("markup", &slot.markup, tree);
                    dump_bag("entity", &slot.entity, tree);
                    let ships: Vec<&str> = slot
                        .candidates
                        .iter()
                        .map(|c| c.display_name.as_str())
                        .filter(|n| !n.is_empty())
                        .collect();
                    if !ships.is_empty() {
                        println!("          ships:    {}", ships.join(", "));
                    }
                }
            }
        }
        Encounter::Npcs(s) => {
            for phase in &s.phases {
                println!("      phase {:?}, {} slot(s)", phase.name, phase.slots.len());
                for (i, slot) in phase.slots.iter().enumerate() {
                    println!(
                        "        slot {i}: priority={} weight={:.2} allied_marker={} critical={} faction_override={:?}",
                        slot.priority,
                        slot.weight,
                        slot.mission_allied_marker,
                        slot.is_critical,
                        slot.faction_override,
                    );
                    dump_bag("identifier", &slot.identifier_tags, tree);
                }
            }
        }
        Encounter::Entities(s) => {
            for phase in &s.phases {
                println!("      phase {:?}, {} slot(s)", phase.name, phase.slots.len());
                for (i, slot) in phase.slots.iter().enumerate() {
                    println!(
                        "        slot {i}: amount={} weight={:.2}",
                        slot.amount, slot.weight
                    );
                    dump_bag("positive", &slot.positive, tree);
                    dump_bag("negative", &slot.negative, tree);
                    dump_bag("markup", &slot.markup, tree);
                    dump_bag("entity", &slot.entity, tree);
                }
            }
        }
        Encounter::Unknown { variable_name, raw_guid } => {
            println!("      raw_guid: {raw_guid} (variable_name={variable_name})");
        }
    }
}

fn dump_bag(label: &str, bag: &sc_contracts::TagBag, tree: &TagTree) {
    if bag.is_empty() {
        return;
    }
    println!("          {label} ({}):", bag.len());
    for (guid, name) in bag.iter() {
        let path = tree.path(guid);
        let path_s = if path.is_empty() {
            String::new()
        } else {
            format!("  [{}]", path.join(" ▸ "))
        };
        println!("            - {name}{path_s}");
    }
}

/// Walk the raw record's `paramOverrides.propertyOverrides` directly
/// so we can see properties whose poly variant the Encounter enum
/// doesn't model (or that v2 chose to skip). This catches cases like
/// "the Allies wave is actually pointing at a turret-archetype tag
/// that's also referenced from somewhere else".
fn dump_raw_properties(datacore: &Datacore, contract_id: Guid, tree: &TagTree) {
    let db = datacore.db();
    let Some(record) = db.record(&contract_id) else {
        println!("  <no record for {contract_id}>");
        return;
    };
    let inst = record.as_instance();
    walk_property_overrides(&inst, "  contract.paramOverrides", db, tree);

    // Dive into sub_contracts too for completeness.
    if let Some(arr) = inst.get_array("sub_contracts") {
        for (i, v) in arr.enumerate() {
            if let raw::Value::StrongPointer(Some(r)) | raw::Value::WeakPointer(Some(r)) = &v {
                let sub = db.instance(r.struct_index, r.instance_index);
                walk_property_overrides(
                    &sub,
                    &format!("  sub_contracts[{i}]"),
                    db,
                    tree,
                );
            }
        }
    }
}

fn walk_property_overrides(
    inst: &raw::Instance<'_>,
    label: &str,
    db: &raw::DataCoreDatabase,
    tree: &TagTree,
) {
    let Some(po) = inst.get_instance("paramOverrides") else {
        return;
    };
    let Some(prop_arr) = po.get_array("propertyOverrides") else {
        return;
    };
    let mut count = 0usize;
    for prop_v in prop_arr {
        let prop_inst = match prop_v {
            raw::Value::Class { struct_index, data } => {
                raw::Instance::from_inline_data(db, struct_index, data)
            }
            raw::Value::ClassRef(r) => db.instance(r.struct_index, r.instance_index),
            _ => continue,
        };
        count += 1;
        let var_name = prop_inst.get_str("missionVariableName").unwrap_or("?");
        let ext_token = prop_inst.get_str("extendedTextToken").unwrap_or("");
        let value_type = prop_inst
            .get_instance("value")
            .and_then(|v| v.type_name())
            .unwrap_or("<no value>");
        println!(
            "{label}.propertyOverrides[{count}] var='{var_name}' ext='{ext_token}' value={value_type}"
        );

        // For ship/npc/entity spawn variants, pull tag GUIDs from the
        // first option for visual confirmation.
        if let Some(value) = prop_inst.get_instance("value") {
            if value.type_name() == Some("MissionPropertyValue_NPCSpawnDescriptions") {
                if let Some(arr) = value.get_array("spawnDescriptions") {
                    for v in arr {
                        if let raw::Value::Class { struct_index, data } = v {
                            let g = raw::Instance::from_inline_data(db, struct_index, data);
                            let name = g.get_str("Name").unwrap_or("?");
                            let opt_count = g
                                .get_array("options")
                                .map(|i| i.count())
                                .unwrap_or(0);
                            println!("    npc-group '{name}' options={opt_count}");
                        }
                    }
                }
            }
        }
        let _ = (tree, ext_token); // currently unused but kept for future expansion
    }
    if count == 0 {
        println!("{label}.propertyOverrides: <empty>");
    }
}
