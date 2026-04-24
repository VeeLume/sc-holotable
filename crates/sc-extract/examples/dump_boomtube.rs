//! Deep dive into the Boomtube rocket launcher structure.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_boomtube
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

    // Find the base Boomtube
    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names.get(&guid).copied().unwrap_or("?");
        if !name.contains("none_special_ballistic_01")
            || name.contains("_tint")
            || name.contains("_collector")
            || name.contains("Prop")
        {
            continue;
        }

        println!("=== {} ===", name);
        println!("GUID: {}", guid);
        println!("Components: {}", ecd.components.len());

        // List all component types
        println!("\nComponent types:");
        for (i, comp) in ecd.components.iter().enumerate() {
            let type_name = match comp {
                DataForgeComponentParamsPtr::SCItemWeaponComponentParams(_) => {
                    "SCItemWeaponComponentParams"
                }
                DataForgeComponentParamsPtr::SAttachableComponentParams(_) => {
                    "SAttachableComponentParams"
                }
                DataForgeComponentParamsPtr::SAmmoContainerComponentParams(_) => {
                    "SAmmoContainerComponentParams"
                }
                DataForgeComponentParamsPtr::SHealthComponentParams(_) => "SHealthComponentParams",
                _ => "other",
            };
            if type_name != "other" {
                println!("  [{}] {}", i, type_name);
            }
        }

        // Weapon params
        let wp = find_weapon_component(ecd, pools);
        if let Some(wp) = wp {
            println!("\n--- SCItemWeaponComponentParams ---");
            println!("ammoContainerRecord: {:?}", wp.ammo_container_record);
            println!("fireActions count: {}", wp.fire_actions.len());

            for (i, action) in wp.fire_actions.iter().enumerate() {
                println!("\n  fireAction[{}]:", i);
                match action {
                    SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
                        if let Some(s) = h.get(pools) {
                            println!("    type: FireSingle");
                            println!("    name: {}", s.name);
                            println!("    fireRate: {} rpm", s.fire_rate);
                            println!("    heatPerShot: {}", s.heat_per_shot);
                            println!("    launchParams: {}", s.launch_params.is_some());
                            println!("    explosionParams: {}", s.explosion_params.is_some());
                            if let Some(expl) = s.explosion_params.and_then(|h| h.get(pools)) {
                                println!("    explosion minRadius: {}", expl.min_radius);
                                println!("    explosion maxRadius: {}", expl.max_radius);
                                if let Some(dmg_ptr) = &expl.damage {
                                    print_damage("    explosion damage", dmg_ptr, pools);
                                }
                            }
                            // Check launch params
                            if let Some(ref lp) = s.launch_params {
                                println!(
                                    "    launchParams variant: {:?}",
                                    std::mem::discriminant(lp)
                                );
                            }
                        }
                    }
                    other => {
                        println!("    type: {:?}", std::mem::discriminant(other));
                    }
                }
            }

            // Connection params
            if let Some(conn) = wp.connection_params.and_then(|h| h.get(pools)) {
                println!("\n--- SWeaponConnectionParams ---");
                println!(
                    "simplifiedHeatParams: {}",
                    conn.simplified_heat_params.is_some()
                );
                println!("rangedHeatStats count: {}", conn.ranged_heat_stats.len());
                if let Some(stats) = conn.no_power_stats.and_then(|h| h.get(pools)) {
                    println!("noPowerStats.pellets: {}", stats.pellets);
                    println!("noPowerStats.fireRate: {}", stats.fire_rate);
                    println!("noPowerStats.damageMultiplier: {}", stats.damage_multiplier);
                }
            }

            // Regen
            if let Some(regen) = wp.weapon_regen_consumer_params.and_then(|h| h.get(pools)) {
                println!("\n--- SWeaponRegenConsumerParams ---");
                println!("maxAmmoLoad: {}", regen.max_ammo_load);
                println!("regenerationCooldown: {}", regen.regeneration_cooldown);
                println!(
                    "regenerationCostPerBullet: {}",
                    regen.regeneration_cost_per_bullet
                );
                println!("maxRegenPerSec: {}", regen.max_regen_per_sec);
            }
        }

        // Ammo container on weapon entity
        let ac = find_ammo_container(ecd, pools);
        if let Some(ac) = ac {
            println!("\n--- SAmmoContainerComponentParams (on weapon entity) ---");
            println!("maxAmmoCount: {}", ac.max_ammo_count);
            println!("initialAmmoCount: {}", ac.initial_ammo_count);
            println!("ammoParamsRecord: {:?}", ac.ammo_params_record);
            if let Some(ammo_guid) = ac.ammo_params_record {
                let ammo_name = record_names.get(&ammo_guid).copied().unwrap_or("?");
                println!("  -> resolves to: {}", ammo_name);
            }
        }

        // Follow ammo chain
        println!("\n--- Ammo resolution ---");
        // Path 1: weapon's ammoContainerRecord
        if let Some(ref_guid) = wp.and_then(|w| w.ammo_container_record) {
            let ref_name = record_names.get(&ref_guid).copied().unwrap_or("?");
            println!("ammoContainerRecord -> {}", ref_name);

            // Is it an AmmoParams directly?
            if let Some(&ammo_h) = ammo_map.get(&ref_guid) {
                if let Some(ammo) = ammo_h.get(pools) {
                    println!("  (direct AmmoParams hit!)");
                    dump_ammo(ammo, pools, &record_names, ammo_map);
                }
            } else if let Some(&container_h) = ecd_map.get(&ref_guid) {
                // Two-hop: it's an entity
                println!("  (entity — following two-hop chain)");
                if let Some(container_ecd) = container_h.get(pools) {
                    println!("  container components: {}", container_ecd.components.len());

                    if let Some(inner_ac) = find_ammo_container(container_ecd, pools) {
                        println!("  inner SAmmoContainerComponentParams:");
                        println!("    maxAmmoCount: {}", inner_ac.max_ammo_count);
                        println!("    ammoParamsRecord: {:?}", inner_ac.ammo_params_record);
                        if let Some(ammo_guid) = inner_ac.ammo_params_record {
                            let ammo_name = record_names.get(&ammo_guid).copied().unwrap_or("?");
                            println!("    -> resolves to: {}", ammo_name);

                            if let Some(&ammo_h) = ammo_map.get(&ammo_guid)
                                && let Some(ammo) = ammo_h.get(pools) {
                                    dump_ammo(ammo, pools, &record_names, ammo_map);
                                }
                        }
                    }

                    // Check for weapon params on the container (secondary ammo?)
                    if let Some(inner_wp) = find_weapon_component(container_ecd, pools) {
                        println!("  container also has SCItemWeaponComponentParams!");
                        println!("    fireActions: {}", inner_wp.fire_actions.len());
                    }
                }
            }
        }

        println!();
    }

    Ok(())
}

fn dump_ammo(
    ammo: &AmmoParams,
    pools: &DataPools,
    _names: &HashMap<Guid, &str>,
    _ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) {
    println!("\n  --- AmmoParams ---");
    println!("  speed: {}", ammo.speed);
    println!("  lifetime: {}", ammo.lifetime);
    println!("  size: {}", ammo.size);
    println!("  ammoCategory: {:?}", ammo.ammo_category);
    println!("  spawnType: {:?}", ammo.spawn_type);
    println!("  inheritVelocity: {}", ammo.inherit_velocity);
    println!("  hitPoints: {}", ammo.hit_points);

    if let Some(ref proj_ptr) = ammo.projectile_params {
        match proj_ptr {
            ProjectileParamsPtr::BulletProjectileParams(h) => {
                if let Some(bullet) = h.get(pools) {
                    println!("  projectileParams: BulletProjectileParams");
                    if let Some(ref dmg_ptr) = bullet.damage {
                        print_damage("    direct damage", dmg_ptr, pools);
                    }
                    println!("    impactRadius: {}", bullet.impact_radius);
                    println!(
                        "    detonationParams: {}",
                        bullet.detonation_params.is_some()
                    );
                    if let Some(det) = bullet.detonation_params.and_then(|h| h.get(pools)) {
                        println!("    detonation armTime: {}", det.arm_time);
                        println!("    detonation explodeOnImpact: {}", det.explode_on_impact);
                        println!("    detonation explodeOnExpire: {}", det.explode_on_expire);
                        if let Some(expl) = det.explosion_params.and_then(|h| h.get(pools)) {
                            println!("    explosion minRadius: {}", expl.min_radius);
                            println!("    explosion maxRadius: {}", expl.max_radius);
                            println!("    explosion pressure: {}", expl.pressure);
                            if let Some(ref dmg_ptr) = expl.damage {
                                print_damage("    explosion damage", dmg_ptr, pools);
                            }
                        }
                    }

                    // Check for additional projectiles (sub-munitions)
                    // This field might be on BulletProjectileParams or on AmmoParams
                }
            }
            ProjectileParamsPtr::ProjectileParams(h) => {
                println!("  projectileParams: ProjectileParams (base)");
                if let Some(p) = h.get(pools) {
                    println!("    detonationParams: {}", p.detonation_params.is_some());
                    if let Some(det) = p.detonation_params.and_then(|h| h.get(pools)) {
                        println!("    detonation explodeOnImpact: {}", det.explode_on_impact);
                        if let Some(expl) = det.explosion_params.and_then(|h| h.get(pools)) {
                            println!("    explosion minRadius: {}", expl.min_radius);
                            println!("    explosion maxRadius: {}", expl.max_radius);
                            if let Some(ref dmg_ptr) = expl.damage {
                                print_damage("    explosion damage", dmg_ptr, pools);
                            }
                        }
                    }
                }
            }
            _other => {
                println!("  projectileParams: other variant");
            }
        }
    } else {
        println!("  projectileParams: None");
    }

    // Check for additional projectiles field on AmmoParams
    // The struct had no such field directly, but let's check if there's something
    // in the geometry or physics controller params
    println!(
        "  geometryResourceParams: {}",
        ammo.geometry_resource_params.is_some()
    );
    println!(
        "  physicsControllerParams: {}",
        ammo.physics_controller_params.is_some()
    );
}

fn print_damage(label: &str, ptr: &DamageBasePtr, pools: &DataPools) {
    match ptr {
        DamageBasePtr::DamageInfo(h) => {
            if let Some(d) = h.get(pools) {
                println!(
                    "{}: phys={:.2} energy={:.2} dist={:.2} thermal={:.2} biochem={:.2} stun={:.2}",
                    label,
                    d.damage_physical,
                    d.damage_energy,
                    d.damage_distortion,
                    d.damage_thermal,
                    d.damage_biochemical,
                    d.damage_stun
                );
            }
        }
        DamageBasePtr::DamageParams(h) => {
            if let Some(_d) = h.get(pools) {
                println!("{}: DamageParams (has damage_info handle)", label);
                // DamageParams wraps DamageInfo — check its structure
            }
        }
        DamageBasePtr::DamageBase(_) => {
            println!("{}: DamageBase (empty base)", label);
        }
        DamageBasePtr::Unknown {
            struct_index,
            instance_index,
        } => {
            println!(
                "{}: Unknown(si={}, ii={})",
                label, struct_index, instance_index
            );
        }
    }
}

fn find_weapon_component<'a>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SCItemWeaponComponentParams> {
    for comp in &ecd.components {
        if let DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) = comp {
            return h.get(pools);
        }
    }
    None
}

fn find_ammo_container<'a>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SAmmoContainerComponentParams> {
    for comp in &ecd.components {
        if let DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) = comp {
            return h.get(pools);
        }
    }
    None
}
