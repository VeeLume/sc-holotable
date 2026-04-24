//! Dump all HealingBeam weapons.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example dump_healing
//! ```

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{AssetConfig, AssetData, AssetSource, DatacoreConfig, Guid};

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

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names
            .get(&guid)
            .copied()
            .unwrap_or("?")
            .replace("EntityClassDefinition.", "");
        let display = snap.display_names.get(&guid).unwrap_or("").to_string();

        let Some(wp) = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        }) else {
            continue;
        };

        // Collect all HealingBeam actions
        let mut has_healing = false;
        for action in &wp.fire_actions {
            if let SWeaponActionParamsPtr::SWeaponActionFireHealingBeamParams(h) = action
                && let Some(hb) = h.get(pools) {
                    if !has_healing {
                        println!("{} ({})", name, display);
                        println!("  fireActions: {}", wp.fire_actions.len());
                        has_healing = true;
                    }
                    println!("  HealingBeam:");
                    println!("    name: {}", hb.name);
                    println!("    healingMode: {:?}", hb.healing_mode);
                    println!("    externalHealingMode: {}", hb.external_healing_mode);
                    println!("    toggle: {}", hb.toggle);
                    println!("    maxDistance: {}", hb.max_distance);
                    println!("    maxSensorDistance: {}", hb.max_sensor_distance);
                    println!("    mSCUPerSec: {}", hb.m_scuper_sec);
                    println!("    ammoPerMSCU: {}", hb.ammo_per_mscu);
                    println!("    medicalAmmoType: {:?}", hb.medical_ammo_type);
                    println!("    wearPerSec: {}", hb.wear_per_sec);
                }
        }

        // Also show non-healing actions on the same weapon
        if has_healing {
            for (i, action) in wp.fire_actions.iter().enumerate() {
                let desc = match action {
                    SWeaponActionParamsPtr::SWeaponActionFireHealingBeamParams(_) => continue,
                    SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => h
                        .get(pools)
                        .map(|r| format!("[{}] FireRapid rpm={}", i, r.fire_rate))
                        .unwrap_or_default(),
                    SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => h
                        .get(pools)
                        .map(|s| format!("[{}] FireSingle rpm={}", i, s.fire_rate))
                        .unwrap_or_default(),
                    SWeaponActionParamsPtr::SWeaponActionFireBeamParams(_) => {
                        format!("[{}] FireBeam", i)
                    }
                    _ => format!("[{}] Other", i),
                };
                if !desc.is_empty() {
                    println!("  other action: {}", desc);
                }
            }
            println!();
        }
    }

    Ok(())
}
