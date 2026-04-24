//! Boomtube deep dive part 2: launchParams, proximity trigger, pressure survey.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_boomtube2
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
    let ammo_map = &snap.records.records.multi_feature.ammo_params;

    // ── Part 1: Boomtube launcher details ─────────────────────────────
    println!("=== BOOMTUBE LAUNCH PARAMS ===\n");

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names.get(&guid).copied().unwrap_or("?");
        if name != "EntityClassDefinition.none_special_ballistic_01" {
            continue;
        }

        let wp = find_weapon(ecd, pools).unwrap();
        let action = &wp.fire_actions[0];
        if let SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) = action {
            let s = h.get(pools).unwrap();
            println!("FireSingle.name: {}", s.name);
            println!("FireSingle.fireRate: {}", s.fire_rate);
            println!(
                "FireSingle.launchParams present: {}",
                s.launch_params.is_some()
            );

            if let Some(ref lp) = s.launch_params {
                match lp {
                    SLauncherBasePtr::SProjectileLauncher(h) => {
                        if let Some(pl) = h.get(pools) {
                            println!("\n  SProjectileLauncher:");
                            println!("    pelletCount: {}", pl.pellet_count);
                            println!("    ammoCost: {}", pl.ammo_cost);
                            println!("    damageMultiplier: {}", pl.damage_multiplier);
                            println!("    soundRadius: {}", pl.sound_radius);
                            println!("    fireHelper: {}", pl.fire_helper);
                            println!("    muzzleHelper: {}", pl.muzzle_helper);
                            println!("    projectileType: {:?}", pl.projectile_type);
                            if let Some(sp) = pl.spread_params.and_then(|h| h.get(pools)) {
                                println!(
                                    "    spread: min={} max={} firstAttack={} attack={} decay={}",
                                    sp.min, sp.max, sp.first_attack, sp.attack, sp.decay
                                );
                            }
                        }
                    }
                    SLauncherBasePtr::SMissileLauncher(h) => {
                        if let Some(_ml) = h.get(pools) {
                            println!("  SMissileLauncher (unexpected)");
                        }
                    }
                    _ => {
                        println!("  Other launcher type");
                    }
                }
            }
        }

        // Also resolve the ammo and check detonation details
        if let Some(ammo_guid) = wp.ammo_container_record
            && let Some(&container_h) = ecd_map.get(&ammo_guid)
            && let Some(container_ecd) = container_h.get(pools)
            && let Some(ac) = find_ammo_container(container_ecd, pools)
            && let Some(ammo_guid2) = ac.ammo_params_record
            && let Some(&ammo_h) = ammo_map.get(&ammo_guid2)
            && let Some(ammo) = ammo_h.get(pools)
        {
            println!("\n  AmmoParams detail:");
            println!("    noBulletHits: {}", ammo.no_bullet_hits);
            println!("    quietRemoval: {}", ammo.quiet_removal);
            println!("    hitPoints: {}", ammo.hit_points);
            println!("    bulletType: {}", ammo.bullet_type);

            if let Some(ProjectileParamsPtr::BulletProjectileParams(h)) = &ammo.projectile_params
                && let Some(bullet) = h.get(pools)
            {
                println!(
                    "    keepAliveOnZeroDamage: {}",
                    bullet.keep_alive_on_zero_damage
                );
                // Proximity trigger
                if let Some(pt) = bullet.proximity_trigger_params.and_then(|h| h.get(pools)) {
                    println!(
                        "    proximityTrigger: armTime={} safeDistance={} radius={}",
                        pt.arm_time, pt.safe_distance, pt.radius
                    );
                } else {
                    println!("    proximityTrigger: None");
                }

                // Full detonation details
                if let Some(det) = bullet.detonation_params.and_then(|h| h.get(pools)) {
                    println!("    detonation:");
                    println!("      armTime: {}", det.arm_time);
                    println!("      safeDistance: {}", det.safe_distance);
                    println!("      destructDelay: {}", det.destruct_delay);
                    println!("      explodeOnImpact: {}", det.explode_on_impact);
                    println!(
                        "      explodeOnFinalImpact: {}",
                        det.explode_on_final_impact
                    );
                    println!("      explodeOnExpire: {}", det.explode_on_expire);
                    println!(
                        "      explodeOnTargetRange: {}",
                        det.explode_on_target_range
                    );
                }
            }
        }
        break;
    }

    // ── Part 2: Pressure survey across all ammo ──────────────────────
    println!("\n\n=== EXPLOSION PRESSURE SURVEY (all AmmoParams) ===\n");
    println!("Record\tPressure\tMinR\tMaxR\tExplPhys\tExplEnergy\tExplDist");

    let mut entries: Vec<(String, f32, f32, f32, f32, f32, f32)> = Vec::new();

    for (&guid, &handle) in ammo_map {
        let Some(ammo) = handle.get(pools) else {
            continue;
        };
        let name = record_names.get(&guid).copied().unwrap_or("?");

        if let Some(ProjectileParamsPtr::BulletProjectileParams(h)) = &ammo.projectile_params
            && let Some(bullet) = h.get(pools)
            && let Some(det) = bullet.detonation_params.and_then(|h| h.get(pools))
            && let Some(expl) = det.explosion_params.and_then(|h| h.get(pools))
            && expl.pressure != 0.0
        {
            let (ep, ee, ed) = if let Some(ref dp) = expl.damage {
                match dp {
                    DamageBasePtr::DamageInfo(dh) => dh
                        .get(pools)
                        .map(|d| (d.damage_physical, d.damage_energy, d.damage_distortion))
                        .unwrap_or_default(),
                    _ => (0.0, 0.0, 0.0),
                }
            } else {
                (0.0, 0.0, 0.0)
            };
            entries.push((
                name.to_string(),
                expl.pressure,
                expl.min_radius,
                expl.max_radius,
                ep,
                ee,
                ed,
            ));
        }
    }

    entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (name, pressure, min_r, max_r, ep, ee, ed) in &entries {
        println!(
            "{}\t{:.0}\t{:.1}\t{:.1}\t{:.0}\t{:.0}\t{:.0}",
            name.replace("AmmoParams.", ""),
            pressure,
            min_r,
            max_r,
            ep,
            ee,
            ed
        );
    }
    println!("\nTotal ammo with nonzero pressure: {}", entries.len());

    // ── Part 3: All weapons with launchParams ────────────────────────
    println!("\n\n=== ALL WEAPONS WITH SProjectileLauncher ===\n");
    println!("Record\tDisplayName\tPellets\tSpread Min\tSpread Max\tDmgMult\tProjType");

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names.get(&guid).copied().unwrap_or("?");
        let display = snap.display_names.get(&guid).unwrap_or("");

        let Some(wp) = find_weapon(ecd, pools) else {
            continue;
        };

        for action in &wp.fire_actions {
            let launcher = match action {
                SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
                    h.get(pools).and_then(|s| s.launch_params.as_ref())
                }
                SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
                    h.get(pools).and_then(|s| s.launch_params.as_ref())
                }
                _ => None,
            };

            if let Some(SLauncherBasePtr::SProjectileLauncher(h)) = launcher
                && let Some(pl) = h.get(pools)
            {
                let (sp_min, sp_max) = pl
                    .spread_params
                    .and_then(|h| h.get(pools))
                    .map(|sp| (sp.min, sp.max))
                    .unwrap_or((0.0, 0.0));
                // Skip CML launchers (countermeasures) — only show combat
                let short_name = name.replace("EntityClassDefinition.", "");
                if short_name.contains("CML") || short_name.contains("binoculars") {
                    continue;
                }
                println!(
                    "{}\t{}\t{}\t{:.2}\t{:.2}\t{:.2}\t{:?}",
                    short_name,
                    display,
                    pl.pellet_count,
                    sp_min,
                    sp_max,
                    pl.damage_multiplier,
                    pl.projectile_type
                );
            }
        }
    }

    Ok(())
}

fn find_weapon<'a>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SCItemWeaponComponentParams> {
    ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
        _ => None,
    })
}

fn find_ammo_container<'a>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SAmmoContainerComponentParams> {
    ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
        _ => None,
    })
}
