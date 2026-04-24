//! Deep dump of ALL SWeaponStats fields for power state modifiers on ship weapons.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example dump_power_deep
//! ```

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{AssetConfig, AssetData, AssetSource, DataPools, DatacoreConfig, Guid};

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
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Track which fields are ever non-default across all weapons
    let mut field_has_nondefault: HashMap<String, Vec<(String, f32)>> = HashMap::new();

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names
            .get(&guid)
            .copied()
            .unwrap_or("?")
            .replace("EntityClassDefinition.", "");

        // Only ship weapons
        if !name.starts_with(|c: char| c.is_uppercase()) {
            continue;
        }

        let Some(wp) = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        }) else {
            continue;
        };

        let base = dedup_name(&name);
        if seen.contains(&base) {
            continue;
        }
        seen.insert(base.clone());

        let connection = wp.connection_params.and_then(|h| h.get(pools));
        let Some(conn) = connection else { continue };

        let slots: Vec<(&str, Option<&SWeaponStats>)> = vec![
            ("noPower", conn.no_power_stats.and_then(|h| h.get(pools))),
            (
                "underpower",
                conn.underpower_stats.and_then(|h| h.get(pools)),
            ),
            ("overpower", conn.overpower_stats.and_then(|h| h.get(pools))),
            ("overclock", conn.overclock_stats.and_then(|h| h.get(pools))),
            ("heat", conn.heat_stats.and_then(|h| h.get(pools))),
        ];

        for (slot_name, stats_opt) in &slots {
            let Some(s) = stats_opt else { continue };
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.fireRate"),
                s.fire_rate as f32,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.fireRateMultiplier"),
                s.fire_rate_multiplier,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.damageMultiplier"),
                s.damage_multiplier,
                1.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.damageOverTimeMultiplier"),
                s.damage_over_time_multiplier,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.projectileSpeedMultiplier"),
                s.projectile_speed_multiplier,
                1.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.pellets"),
                s.pellets as f32,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.burstShots"),
                s.burst_shots as f32,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.ammoCost"),
                s.ammo_cost as f32,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.ammoCostMultiplier"),
                s.ammo_cost_multiplier,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.heatGenerationMultiplier"),
                s.heat_generation_multiplier,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.soundRadiusMultiplier"),
                s.sound_radius_multiplier,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.chargeTimeMultiplier"),
                s.charge_time_multiplier,
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.useAlternateProjectileVisuals"),
                if s.use_alternate_projectile_visuals {
                    1.0
                } else {
                    0.0
                },
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.useAugmentedRealityProjectiles"),
                if s.use_augmented_reality_projectiles {
                    1.0
                } else {
                    0.0
                },
                0.0,
            );
            check_field(
                &mut field_has_nondefault,
                &base,
                &format!("{slot_name}.disableMisfire"),
                if s.disable_misfire { 1.0 } else { 0.0 },
                0.0,
            );

            // Check modifier sub-structs
            if let Some(regen) = s.regen_modifier.and_then(|h| h.get(pools)) {
                check_field(
                    &mut field_has_nondefault,
                    &base,
                    &format!("{slot_name}.regenModifier.present"),
                    1.0,
                    0.0,
                );
            }
            if let Some(spread) = s.spread_modifier.and_then(|h| h.get(pools)) {
                check_field(
                    &mut field_has_nondefault,
                    &base,
                    &format!("{slot_name}.spreadModifier.present"),
                    1.0,
                    0.0,
                );
            }
            if let Some(aim) = s.aim_modifier.and_then(|h| h.get(pools)) {
                check_field(
                    &mut field_has_nondefault,
                    &base,
                    &format!("{slot_name}.aimModifier.present"),
                    1.0,
                    0.0,
                );
            }
            if let Some(recoil) = s.recoil_modifier.and_then(|h| h.get(pools)) {
                check_field(
                    &mut field_has_nondefault,
                    &base,
                    &format!("{slot_name}.recoilModifier.present"),
                    1.0,
                    0.0,
                );
            }
        }
    }

    println!("=== SHIP WEAPON POWER STATE FIELDS WITH NON-DEFAULT VALUES ===\n");

    let mut sorted_fields: Vec<_> = field_has_nondefault
        .iter()
        .filter(|(_, weapons)| !weapons.is_empty())
        .collect();
    sorted_fields.sort_by_key(|(field, _)| field.clone());

    for (field, weapons) in &sorted_fields {
        let values: Vec<f32> = weapons.iter().map(|(_, v)| *v).collect();
        let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        println!(
            "{}: {} weapons (range {:.3}-{:.3})",
            field,
            weapons.len(),
            min,
            max
        );
        if weapons.len() <= 10 {
            for (name, val) in weapons.iter() {
                println!("  {}: {:.3}", name, val);
            }
        }
    }

    Ok(())
}

fn check_field(
    map: &mut HashMap<String, Vec<(String, f32)>>,
    weapon: &str,
    field: &str,
    value: f32,
    default: f32,
) {
    if (value - default).abs() > 0.001 {
        map.entry(field.to_string())
            .or_default()
            .push((weapon.to_string(), value));
    }
}

fn dedup_name(name: &str) -> String {
    if name.starts_with(|c: char| c.is_uppercase()) {
        return name.to_string();
    }
    if let Some(pos) = name.find("_01_") {
        return name[..pos + 3].to_string();
    }
    if let Some(pos) = name.find("_02_") {
        return name[..pos + 3].to_string();
    }
    name.to_string()
}
