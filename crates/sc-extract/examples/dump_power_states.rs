//! Dump weapon power state modifiers + health/repair + secondary ammo + ammo empty + subtypes.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_power_states
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

    // Counters
    let mut has_no_power = 0u32;
    let mut has_underpower = 0u32;
    let mut has_overpower = 0u32;
    let mut has_overclock = 0u32;
    let mut has_heat_stats = 0u32;
    let mut has_overheat_locked = 0u32;
    let mut has_ranged_heat = 0u32;
    let mut has_on_ammo_empty = 0u32;
    let mut has_secondary_ammo = 0u32;

    // Health stats
    let mut health_values: Vec<(String, String, f32, i32)> = Vec::new(); // name, display, health, size

    // SubType counts
    let mut subtype_counts: HashMap<String, u32> = HashMap::new();

    // Power state details for weapons that have non-default modifiers
    let mut power_details: Vec<String> = Vec::new();

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

        // Dedup
        let base = dedup_name(&name);
        if seen.contains(&base) {
            continue;
        }
        seen.insert(base);

        let item_def = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SAttachableComponentParams(h) => h
                .get(pools)
                .and_then(|a| a.attach_def.and_then(|h| h.get(pools))),
            _ => None,
        });

        let size = item_def.map(|d| d.size).unwrap_or(0);
        let subtype = item_def
            .map(|d| format!("{:?}", d.sub_type))
            .unwrap_or("None".into());
        *subtype_counts.entry(subtype).or_default() += 1;

        // === Health ===
        let health = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SHealthComponentParams(h) => h.get(pools),
            _ => None,
        });
        if let Some(h) = health {
            health_values.push((name.clone(), display.clone(), h.health, size));
        }

        // === Connection params / power states ===
        let connection = wp.connection_params.and_then(|h| h.get(pools));
        if let Some(conn) = connection {
            let np = conn.no_power_stats.and_then(|h| h.get(pools));
            let up = conn.underpower_stats.and_then(|h| h.get(pools));
            let op = conn.overpower_stats.and_then(|h| h.get(pools));
            let oc = conn.overclock_stats.and_then(|h| h.get(pools));
            let hs = conn.heat_stats.and_then(|h| h.get(pools));
            let ol = conn.overheat_locked_stats.and_then(|h| h.get(pools));

            if np.is_some() {
                has_no_power += 1;
            }
            if up.is_some() {
                has_underpower += 1;
            }
            if op.is_some() {
                has_overpower += 1;
            }
            if oc.is_some() {
                has_overclock += 1;
            }
            if hs.is_some() {
                has_heat_stats += 1;
            }
            if ol.is_some() {
                has_overheat_locked += 1;
            }
            if !conn.ranged_heat_stats.is_empty() {
                has_ranged_heat += 1;
            }

            // Dump details if any non-default modifier has interesting values
            let any_interesting = [up, op, oc, hs, ol].iter().any(|s| {
                s.map(|stats| {
                    stats.damage_multiplier != 0.0
                        || stats.fire_rate != 0
                        || stats.pellets != 0
                        || stats.projectile_speed_multiplier != 0.0
                })
                .unwrap_or(false)
            });

            if any_interesting {
                let mut detail = format!("{} ({}) S{}", name, display, size);
                if let Some(s) = np {
                    detail += &format!(
                        "\n  noPower: rpm={} dmg={:.2} speed={:.2} pellets={}",
                        s.fire_rate, s.damage_multiplier, s.projectile_speed_multiplier, s.pellets
                    );
                }
                if let Some(s) = up {
                    detail += &format!(
                        "\n  underpower: rpm={} dmg={:.2} speed={:.2} pellets={}",
                        s.fire_rate, s.damage_multiplier, s.projectile_speed_multiplier, s.pellets
                    );
                }
                if let Some(s) = op {
                    detail += &format!(
                        "\n  overpower: rpm={} dmg={:.2} speed={:.2} pellets={}",
                        s.fire_rate, s.damage_multiplier, s.projectile_speed_multiplier, s.pellets
                    );
                }
                if let Some(s) = oc {
                    detail += &format!(
                        "\n  overclock: rpm={} dmg={:.2} speed={:.2} pellets={}",
                        s.fire_rate, s.damage_multiplier, s.projectile_speed_multiplier, s.pellets
                    );
                }
                if let Some(s) = hs {
                    detail += &format!(
                        "\n  heatStats: rpm={} dmg={:.2} speed={:.2} pellets={}",
                        s.fire_rate, s.damage_multiplier, s.projectile_speed_multiplier, s.pellets
                    );
                }
                if let Some(s) = ol {
                    detail += &format!(
                        "\n  overheatLocked: rpm={} dmg={:.2} speed={:.2} pellets={}",
                        s.fire_rate, s.damage_multiplier, s.projectile_speed_multiplier, s.pellets
                    );
                }
                if !conn.ranged_heat_stats.is_empty() {
                    for (i, rhs_h) in conn.ranged_heat_stats.iter().enumerate() {
                        if let Some(rhs) = rhs_h.get(pools) {
                            let range_str = rhs
                                .range
                                .and_then(|h| h.get(pools))
                                .map(|r| format!("{:.1}-{:.1}", r.minimum, r.maximum))
                                .unwrap_or("?".into());
                            let ws = rhs.weapon_stats.and_then(|h| h.get(pools));
                            let ws_str = ws
                                .map(|s| {
                                    format!(
                                        "rpm={} dmg={:.2} speed={:.2}",
                                        s.fire_rate,
                                        s.damage_multiplier,
                                        s.projectile_speed_multiplier
                                    )
                                })
                                .unwrap_or("none".into());
                            detail += &format!(
                                "\n  rangedHeat[{}]: range={} interp={} {}",
                                i, range_str, rhs.interpolate, ws_str
                            );
                        }
                    }
                }
                power_details.push(detail);
            }
        }

        // === Secondary ammo ===
        if !wp.secondary_ammo_containers.is_empty() {
            has_secondary_ammo += 1;
        }

        // === On ammo empty ===
        if wp.on_ammo_empty_params.is_some() {
            has_on_ammo_empty += 1;
        }
    }

    // === Print results ===

    println!("=== POWER STATE MODIFIER PRESENCE (unique base weapons) ===\n");
    println!("noPowerStats:       {has_no_power}");
    println!("underpowerStats:    {has_underpower}");
    println!("overpowerStats:     {has_overpower}");
    println!("overclockStats:     {has_overclock}");
    println!("heatStats:          {has_heat_stats}");
    println!("overheatLockedStats:{has_overheat_locked}");
    println!("rangedHeatStats:    {has_ranged_heat}");

    println!("\n=== INTERESTING POWER STATE DETAILS ===\n");
    for d in &power_details {
        println!("{d}\n");
    }

    println!("\n=== SECONDARY AMMO ===\n");
    println!("Weapons with secondaryAmmoContainers: {has_secondary_ammo}");

    println!("\n=== ON AMMO EMPTY ===\n");
    println!("Weapons with onAmmoEmptyParams: {has_on_ammo_empty}");

    println!("\n=== SUBTYPES ===\n");
    let mut st_sorted: Vec<_> = subtype_counts.into_iter().collect();
    st_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (st, c) in &st_sorted {
        println!("  {st}: {c}");
    }

    println!("\n=== HEALTH VALUES ===\n");
    // Group by ship (uppercase) vs FPS (lowercase)
    let ship_health: Vec<_> = health_values
        .iter()
        .filter(|(n, _, _, _)| n.starts_with(|c: char| c.is_uppercase()))
        .collect();
    let fps_health: Vec<_> = health_values
        .iter()
        .filter(|(n, _, _, _)| n.starts_with(|c: char| c.is_lowercase()))
        .collect();

    println!("Ship weapons with health ({}):", ship_health.len());
    let mut ship_by_size: HashMap<i32, Vec<f32>> = HashMap::new();
    for (_, _, h, s) in &ship_health {
        ship_by_size.entry(*s).or_default().push(*h);
    }
    let mut sizes: Vec<_> = ship_by_size.keys().copied().collect();
    sizes.sort();
    for s in &sizes {
        let vals = &ship_by_size[s];
        let min = vals.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        if min == max {
            println!("  S{s}: {min:.0} ({} weapons)", vals.len());
        } else {
            println!("  S{s}: {min:.0}-{max:.0} ({} weapons)", vals.len());
        }
    }

    println!("\nFPS weapons with health ({}):", fps_health.len());
    let mut fps_by_size: HashMap<i32, Vec<f32>> = HashMap::new();
    for (_, _, h, s) in &fps_health {
        fps_by_size.entry(*s).or_default().push(*h);
    }
    let mut sizes: Vec<_> = fps_by_size.keys().copied().collect();
    sizes.sort();
    for s in &sizes {
        let vals = &fps_by_size[s];
        let min = vals.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        if min == max {
            println!("  S{s}: {min:.0} ({} weapons)", vals.len());
        } else {
            println!("  S{s}: {min:.0}-{max:.0} ({} weapons)", vals.len());
        }
    }

    // Print outliers — weapons with unusual health
    println!("\nShip weapon health outliers (min/max per size):");
    for (name, display, h, s) in &ship_health {
        let vals = &ship_by_size[s];
        let min = vals.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        if *h == min || *h == max {
            if min != max {
                println!("  {} ({}) S{}: {:.0}", name, display, s, h);
            }
        }
    }

    Ok(())
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
