//! Find where Adagio Lawful Salvage missions source their wreck ship
//! and the template encounter description seen in the contract UI.
//!
//! The user's screenshots show:
//!   - Per-claim target ships: C.O. Nomad, RSI Zeus Mk II ES,
//!     Drake Cutter, Aegis Retaliator, Crusader Mercury Star Runner,
//!     MISC Hull A, Aegis Gladius — these are the actual wrecks.
//!   - A fixed "Encounter" text identical across every claim:
//!     "2× Avenger Stalker, 2× Hurricane, 1× (600i, C2 Hercules
//!     Starlifter, Carrack, Corsair, M2 Hercules Starlifter,
//!     Reclaimer, Retaliator, Starfarer, Starfarer Gemini)
//!      Encounter  1× Vulture"
//!     This is a template description, not per-mission spawn data.
//!
//! This dig:
//!   1. Search for any record whose name/id matches the wreck-ship names
//!      shown in the UI — to find the source pool.
//!   2. Search for any record whose name contains "Salvage" + "Adagio"
//!      to find the contract template's spawn pool (if any).
//!   3. Walk the Stanton1 Locality (the one Adagio uses) and look at
//!      each location's record fields for aiSpawnTags / dataLinks.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example salvage_source
//! ```

use std::collections::HashSet;

use sc_contracts::{MissionIndex, raw};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

const WRECK_NAMES: &[&str] = &[
    "Nomad", "Zeus", "Cutter", "Retaliator", "Mercury", "Hull_A",
    "Gladius", "Hurricane", "Stalker", "Vulture",
];

const TEMPLATE_NEEDLES: &[&str] = &[
    "AdagioSalvage", "Adagio_Salvage", "Adagio_Lawful", "Salvage_Stanton",
    "SalvageContractor", "SalvageRights", "SalvagePool",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore, &asset_data.locale);
    let db = datacore.db();

    // ── §1 Search records by name for wreck-ship and template hints ──────────
    println!("=== §1 Records whose name matches a wreck-ship or template needle ===");
    let mut wreck_hits: Vec<(String, String, Guid)> = Vec::new();
    let mut template_hits: Vec<(String, String, Guid)> = Vec::new();
    for record in db.records() {
        let name = match record.name() {
            Some(n) => n,
            None => continue,
        };
        let inst = record.as_instance();
        let type_name = inst.type_name().unwrap_or("?");
        let id = record.id();
        let lname = name.to_lowercase();

        for needle in WRECK_NAMES {
            if lname.contains(&needle.to_lowercase()) && lname.contains("salvag") {
                wreck_hits.push((name.to_string(), type_name.to_string(), id));
                break;
            }
        }
        for needle in TEMPLATE_NEEDLES {
            if lname.contains(&needle.to_lowercase()) {
                template_hits.push((name.to_string(), type_name.to_string(), id));
                break;
            }
        }
    }
    if !wreck_hits.is_empty() {
        println!("  wreck-ship matches in salvage-named records:");
        wreck_hits.sort();
        for (name, type_name, _) in wreck_hits.iter().take(40) {
            println!("    [{type_name:<32}] {name}");
        }
        if wreck_hits.len() > 40 {
            println!("    ... +{} more", wreck_hits.len() - 40);
        }
    } else {
        println!("  (no records match a wreck-ship name within a salvage-named record)");
    }
    println!();
    if !template_hits.is_empty() {
        println!("  Adagio / Salvage template-related records ({}):", template_hits.len());
        template_hits.sort();
        for (name, type_name, _) in template_hits.iter().take(80) {
            println!("    [{type_name:<32}] {name}");
        }
    }
    println!();

    // ── §2 Walk one Adagio mission's handler/template for spawn refs ─────────
    println!("=== §2 Adagio Stanton Lawful Salvage — handler / template walk ===");
    let target = index
        .contracts
        .iter()
        .find(|c| c.debug_name == "Adaigo_Stanton1_Lawful_Salvage_VeryEasy");
    if let Some(c) = target {
        println!("  contract:  {}", c.debug_name);
        println!("  generator: {} (id {})", c.origin.source_debug_name, c.origin.generator_id);
        println!("  parent:    {:?}", c.origin.subcontract_of);

        // Walk the parent CareerContract via raw and dump every Reference
        // field at top level so we can see what records the contract /
        // its handler / its template point at.
        if let Some(parent_id) = c.origin.subcontract_of {
            walk_parent_chain(&datacore, parent_id);
        } else {
            walk_parent_chain(&datacore, c.id);
        }
    } else {
        println!("  (couldn't find Adaigo_Stanton1_Lawful_Salvage_VeryEasy)");
    }
    println!();

    // ── §3 Stanton1 Adagio locality — look at each location's full record ────
    println!("=== §3 Adagio Stanton1 Locality — per-location record fingerprint ===");
    let locality_guid: Guid = match parse_guid("939dad8b-a944-4baa-a6b7-163ec203adad") {
        Some(g) => g,
        None => return Err("bad guid literal".into()),
    };
    if let Some(view) = index.localities.get(&locality_guid) {
        println!("  locality: {} ({} location(s))", view.name, view.locations.len());
        for loc in view.locations.iter().take(8) {
            let body = if !loc.body_display_name.is_empty() {
                loc.body_display_name.as_str()
            } else {
                loc.body.as_deref().unwrap_or("")
            };
            println!(
                "  ── {} :: {} (body={body})",
                loc.record_name, loc.display_name
            );
            if let Some(rec) = db.record(&loc.guid) {
                let inst = rec.as_instance();
                let mut fields: Vec<String> = inst
                    .properties()
                    .map(|p| p.name.to_string())
                    .collect();
                fields.sort();
                fields.dedup();
                println!("    fields: {}", fields.join(", "));

                // Look for any embedded reference to a Pool / SpawnTags.
                let interesting = ["dataLink", "missionData", "aiSpawnTags", "spawnPool"];
                for f in &interesting {
                    if let Some(v) = inst.get(f) {
                        match v {
                            raw::Value::Reference(Some(r)) => {
                                let ref_name = db
                                    .record(&r.guid)
                                    .and_then(|r| r.name().map(String::from))
                                    .unwrap_or_else(|| "<unresolved>".into());
                                println!("    {f}: {} ({ref_name})", r.guid);
                            }
                            raw::Value::Class { struct_index, .. } => {
                                let ty = db.struct_name(struct_index as usize).unwrap_or("?");
                                println!("    {f}: <inline {ty}>");
                            }
                            raw::Value::StrongPointer(Some(r)) | raw::Value::WeakPointer(Some(r)) => {
                                let ty = db.struct_name(r.struct_index as usize).unwrap_or("?");
                                println!("    {f}: <pointer {ty}>");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    } else {
        println!("  (Stanton1 locality not in index)");
    }
    println!();

    // ── §4 Hint: dump all top-level Reference fields on Adagio handler/template ─
    let _ = WRECK_NAMES; // referenced
    Ok(())
}

fn walk_parent_chain(datacore: &Datacore, parent_id: Guid) {
    let db = datacore.db();
    let Some(parent_rec) = db.record(&parent_id) else {
        println!("    raw walk: <no parent record for {parent_id}>");
        return;
    };
    let parent_inst = parent_rec.as_instance();
    let parent_name = parent_rec.name().unwrap_or("<no name>");
    let parent_type = parent_inst.type_name().unwrap_or("?");
    println!(
        "  parent record: {parent_name} (type {parent_type}, id {parent_id})"
    );

    // Get template ref
    if let Some(v) = parent_inst.get("template")
        && let raw::Value::Reference(Some(r)) = v
    {
        let template_id = r.guid;
        if let Some(template_rec) = db.record(&template_id) {
            let template_name = template_rec.name().unwrap_or("?");
            let template_inst = template_rec.as_instance();
            let template_type = template_inst.type_name().unwrap_or("?");
            println!(
                "  template:      {template_name} (type {template_type}, id {template_id})"
            );

            // Dump every Reference field on the template — these are the
            // pools / records the template binds to. Filter to the ones
            // that look spawn-relevant.
            dump_references(&template_inst, "    template", db);
        }
    }

    // Dump every Reference field on the parent contract itself.
    dump_references(&parent_inst, "    parent  ", db);
}

fn dump_references(inst: &raw::Instance<'_>, prefix: &str, db: &raw::DataCoreDatabase) {
    let mut printed = HashSet::new();
    for prop in inst.properties() {
        let name = prop.name;
        if !printed.insert(name) {
            continue;
        }
        match prop.value {
            raw::Value::Reference(Some(r)) => {
                let ref_name = db
                    .record(&r.guid)
                    .and_then(|x| x.name().map(String::from))
                    .unwrap_or_else(|| "<unresolved>".into());
                println!("{prefix}.{name} = {} ({ref_name})", r.guid);
            }
            raw::Value::StrongPointer(Some(r)) | raw::Value::WeakPointer(Some(r)) => {
                let ty = db.struct_name(r.struct_index as usize).unwrap_or("?");
                println!("{prefix}.{name} = <ptr {ty}>");
            }
            _ => {}
        }
    }
}

fn parse_guid(s: &str) -> Option<Guid> {
    use std::str::FromStr;
    Guid::from_str(s).ok()
}
