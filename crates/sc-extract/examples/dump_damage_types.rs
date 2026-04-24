//! Quick dump of all 6 damage types from AmmoParams to find stun/thermal/biochem.
//!
//! ```bash
//! cargo run -p sc-extract --release --features ammoparams --example dump_damage_types
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

    let ammo_map = &snap.records.records.multi_feature.ammo_params;

    println!(
        "Record\tPhys\tEnergy\tDist\tThermal\tBiochem\tStun\tExpl Phys\tExpl Energy\tExpl Dist\tExpl Thermal\tExpl Biochem\tExpl Stun"
    );

    for (&guid, &handle) in ammo_map {
        let Some(ammo) = handle.get(pools) else {
            continue;
        };
        let name = record_names.get(&guid).copied().unwrap_or("?");

        let (d, ed) = extract_all_damage(ammo, pools);

        // Only print rows where any non-standard damage type is nonzero
        let has_interesting = d.thermal != 0.0
            || d.biochem != 0.0
            || d.stun != 0.0
            || ed.thermal != 0.0
            || ed.biochem != 0.0
            || ed.stun != 0.0
            || d.dist != 0.0
            || ed.dist != 0.0;

        if has_interesting {
            println!(
                "{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",
                name,
                d.phys,
                d.energy,
                d.dist,
                d.thermal,
                d.biochem,
                d.stun,
                ed.phys,
                ed.energy,
                ed.dist,
                ed.thermal,
                ed.biochem,
                ed.stun,
            );
        }
    }

    Ok(())
}

#[derive(Default)]
struct DmgSet {
    phys: f32,
    energy: f32,
    dist: f32,
    thermal: f32,
    biochem: f32,
    stun: f32,
}

fn read_damage(ptr: &DamageBasePtr, pools: &DataPools) -> DmgSet {
    match ptr {
        DamageBasePtr::DamageInfo(h) => {
            if let Some(d) = h.get(pools) {
                DmgSet {
                    phys: d.damage_physical,
                    energy: d.damage_energy,
                    dist: d.damage_distortion,
                    thermal: d.damage_thermal,
                    biochem: d.damage_biochemical,
                    stun: d.damage_stun,
                }
            } else {
                DmgSet::default()
            }
        }
        _ => DmgSet::default(),
    }
}

fn extract_all_damage(ammo: &AmmoParams, pools: &DataPools) -> (DmgSet, DmgSet) {
    let mut direct = DmgSet::default();
    let mut explosion = DmgSet::default();

    if let Some(proj_ptr) = &ammo.projectile_params {
        if let ProjectileParamsPtr::BulletProjectileParams(h) = proj_ptr {
            if let Some(bullet) = h.get(pools) {
                if let Some(dmg_ptr) = &bullet.damage {
                    direct = read_damage(dmg_ptr, pools);
                }
                if let Some(det) = bullet.detonation_params.and_then(|h| h.get(pools))
                    && let Some(expl) = det.explosion_params.and_then(|h| h.get(pools))
                        && let Some(dmg_ptr) = &expl.damage {
                            explosion = read_damage(dmg_ptr, pools);
                        }
            }
        }
    }

    (direct, explosion)
}
