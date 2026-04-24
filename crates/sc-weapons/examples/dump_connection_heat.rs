//! Dump SWeaponConnectionParams heat-related fields for selected ship
//! weapons, to figure out where heat-per-shot/heat-rate lives for Sequence
//! and scattergun weapons whose fire action has no `heatPerShot`.
//!
//! ```bash
//! cargo run -p sc-weapons --release --example dump_connection_heat
//! ```

use sc_extract::generated::*;
use sc_extract::{AssetConfig, AssetData, AssetSource, DatacoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("warn").init();

    let install = sc_installs::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;
    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;

    let targets = [
        // Charged — hunting for the missing cycle time on Singe
        "BANU_TachyonCannon_S1",
        "BANU_TachyonCannon_S3",
    ];

    for target in &targets {
        let Some((guid, handle)) = ecd_map.iter().find(|(g, _)| {
            db.records().iter().any(|r| {
                r.id == **g
                    && db
                        .record_name(r)
                        .map(|n| n.strip_prefix("EntityClassDefinition.").unwrap_or(n) == *target)
                        .unwrap_or(false)
            })
        }) else {
            println!("--- {target}: NOT FOUND ---");
            continue;
        };

        let Some(ecd) = handle.get(pools) else {
            continue;
        };

        // Find SCItemWeaponComponentParams on the entity
        let Some(wp) = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        }) else {
            println!("--- {target}: no weapon component ---");
            continue;
        };

        let _ = guid;
        println!("--- {target} ---");

        // Primary fire action
        let primary = wp.fire_actions.first();
        match primary {
            Some(SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h)) => {
                if let Some(r) = h.get(pools) {
                    println!(
                        "  fire_action: Rapid fire_rate={} heat_per_shot={}",
                        r.fire_rate, r.heat_per_shot
                    );
                }
            }
            Some(SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h)) => {
                if let Some(s) = h.get(pools) {
                    println!(
                        "  fire_action: Single fire_rate={} heat_per_shot={}",
                        s.fire_rate, s.heat_per_shot
                    );
                }
            }
            Some(SWeaponActionParamsPtr::SWeaponActionSequenceParams(_)) => {
                println!("  fire_action: Sequence (no heat_per_shot on this variant)");
            }
            Some(SWeaponActionParamsPtr::SWeaponActionFireChargedParams(h)) => {
                if let Some(c) = h.get(pools) {
                    println!("  fire_action: Charged");
                    println!("    charge_time         = {}", c.charge_time);
                    println!("    overcharge_time     = {}", c.overcharge_time);
                    println!("    overcharged_time    = {}", c.overcharged_time);
                    println!("    cooldown_time       = {}", c.cooldown_time);
                    println!(
                        "    auto_fire           = {}",
                        c.fire_automatically_on_full_charge
                    );
                    println!("    full_only           = {}", c.fire_only_on_full_charge);
                    println!("    interpolate_charge  = {}", c.interpolate_charge_bonus);
                    println!("    charge_automatically= {}", c.charge_automatically);
                }
            }
            Some(other) => {
                println!("  fire_action: {:?}", std::mem::discriminant(other));
            }
            None => {
                println!("  fire_action: none");
            }
        }

        // Connection params
        let Some(conn) = wp.connection_params.and_then(|h| h.get(pools)) else {
            println!("  NO connection_params");
            continue;
        };

        println!("  connection_params:");
        println!("    heat_rate_online        = {}", conn.heat_rate_online);
        println!(
            "    heat_reduce_when_fixed  = {}",
            conn.heat_reduce_when_overheat_is_fixed
        );
        println!("    lock_on_overheat        = {}", conn.lock_on_onverheat);
        println!(
            "    power_active_cooldown   = {}",
            conn.power_active_cooldown
        );

        // Heat params
        if let Some(heat) = conn.simplified_heat_params.and_then(|h| h.get(pools)) {
            println!("  simplified_heat_params:");
            println!(
                "    overheat_temperature         = {}",
                heat.overheat_temperature
            );
            println!(
                "    cooling_per_second           = {}",
                heat.cooling_per_second
            );
            println!(
                "    overheat_fix_time            = {}",
                heat.overheat_fix_time
            );
            println!(
                "    temperature_after_fix        = {}",
                heat.temperature_after_overheat_fix
            );
            println!(
                "    time_till_cooling_starts     = {}",
                heat.time_till_cooling_starts
            );
        } else {
            println!("  NO simplified_heat_params");
        }

        // SWeaponStats — noPower and heatStats slots
        if let Some(s) = conn.no_power_stats.and_then(|h| h.get(pools)) {
            println!(
                "  no_power_stats:  fire_rate={} dmg_mult={:.2} heat_gen_mult={:.2}",
                s.fire_rate, s.damage_multiplier, s.heat_generation_multiplier
            );
        }
        if let Some(s) = conn.heat_stats.and_then(|h| h.get(pools)) {
            println!(
                "  heat_stats:      fire_rate={} dmg_mult={:.2} heat_gen_mult={:.2}",
                s.fire_rate, s.damage_multiplier, s.heat_generation_multiplier
            );
        }

        println!();
    }

    Ok(())
}
