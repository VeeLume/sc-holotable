//! List ALL component types on a few representative weapons to find power draw.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example dump_weapon_components
//! ```

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{
    AssetConfig, AssetData, AssetSource, DataPools, DatacoreConfig, Guid,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let install = sc_installs::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;

    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;

    // Target weapons: M7A (energy cannon), Buzzsaw (ballistic repeater), Bulldog (energy repeater)
    let targets = [
        "EntityClassDefinition.BEHR_LaserCannon_S7",
        "EntityClassDefinition.BEHR_BallisticRepeater_S1",
        "EntityClassDefinition.KLWE_LaserRepeater_S1",
        "EntityClassDefinition.GATS_BallisticGatling_S1",
        "EntityClassDefinition.GATS_BallisticGatling_Mounted_S1",
    ];

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else { continue };
        let name = record_names.get(&guid).copied().unwrap_or("?");

        if !targets.contains(&name) { continue; }

        let display = snap.display_names.get(&guid).unwrap_or("");
        println!("=== {} ({}) ===", name.replace("EntityClassDefinition.", ""), display);
        println!("Components: {}", ecd.components.len());

        // Use the raw DB to get component type names since the poly enum only gives us known variants
        // Actually let's just match all known variants
        for (i, comp) in ecd.components.iter().enumerate() {
            let type_name = component_type_name(comp);
            println!("  [{}] {}", i, type_name);

            // For specific types, print details
            match comp {
                DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => {
                    if let Some(wp) = h.get(pools) {
                        let conn = wp.connection_params.and_then(|h| h.get(pools));
                        println!("       powerActiveCooldown: {}", conn.map(|c| c.power_active_cooldown).unwrap_or(0.0));
                        println!("       heatRateOnline: {}", conn.map(|c| c.heat_rate_online).unwrap_or(0.0));
                    }
                }
                _ => {}
            }
        }
        println!();
    }

    // Check raw DB for component names on multiple weapons
    println!("=== RAW DB component types ===");
    // First print a few record names to check format
    let mut count = 0;
    for record in db.records_by_type("EntityClassDefinition") {
        let name = record.name().unwrap_or("");
        if name.contains("BEHR_LaserCannon") || name.contains("GATS_BallisticGatling") || name.contains("KLWE_LaserRepeater") {
            if count < 3 {
                println!("Found record: '{}'", name);
                count += 1;
            }
        }
    }
    for record in db.records_by_type("EntityClassDefinition") {
        let name = record.name().unwrap_or("");
        if !["BEHR_LaserCannon_S7", "KLWE_LaserRepeater_S1", "GATS_BallisticGatling_S1"].contains(&name) { continue; }

        let inst = record.as_instance();
        if let Some(components) = inst.get_array("Components") {
            for (i, val) in components.enumerate() {
                let type_name = match &val {
                    sc_extract::svarog_datacore::Value::StrongPointer(Some(r)) => {
                        db.struct_name(r.struct_index as usize).unwrap_or("?")
                    }
                    sc_extract::svarog_datacore::Value::Class { struct_index, .. } => {
                        db.struct_name(*struct_index as usize).unwrap_or("?")
                    }
                    _ => "?",
                };
                println!("  [{}] {}", i, type_name);
            }
        }
        println!();
    }

    Ok(())
}

fn component_type_name(comp: &DataForgeComponentParamsPtr) -> &'static str {
    match comp {
        DataForgeComponentParamsPtr::SCItemWeaponComponentParams(_) => "SCItemWeaponComponentParams",
        DataForgeComponentParamsPtr::SAttachableComponentParams(_) => "SAttachableComponentParams",
        DataForgeComponentParamsPtr::SAmmoContainerComponentParams(_) => "SAmmoContainerComponentParams",
        DataForgeComponentParamsPtr::SHealthComponentParams(_) => "SHealthComponentParams",
        DataForgeComponentParamsPtr::DataForgeComponentParams(_) => "DataForgeComponentParams(base)",
        _ => "other (check raw DB)",
    }
}
