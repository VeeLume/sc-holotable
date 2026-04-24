//! Detailed dump of all FireSingle weapons with full ammo/damage resolution.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_fire_single
//! ```

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{AssetConfig, AssetData, AssetSource, DataPools, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let explicit_path: Option<std::path::PathBuf> = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(std::path::PathBuf::from);

    let (assets, _meta) = if let Some(p4k_path) = &explicit_path {
        let a = AssetSource::open(p4k_path)?;
        let m = sc_extract::SnapshotMeta {
            schema_version: sc_extract::ExtractSnapshot::SCHEMA_VERSION,
            game_version: "unknown".into(),
            build_id: "unknown".into(),
            extracted_at: sc_extract::current_timestamp(),
        };
        (a, m)
    } else {
        let install = sc_installs::discover_primary()?;
        let a = AssetSource::from_install(&install)?;
        let m = sc_extract::snapshot_meta_from_install(&install);
        (a, m)
    };

    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let dc_config = DatacoreConfig::standard();
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &dc_config)?;

    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;

    // Build GUID → record name map
    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    // Build manufacturer GUID → resolved name map
    let mfg_names: HashMap<Guid, String> = snap
        .manufacturers
        .all()
        .map(|mfg| {
            let resolved = mfg
                .name_key
                .as_deref()
                .and_then(|k| asset_data.locale.resolve(sc_extract::LocaleKey::new(k)))
                .map(|s| s.to_string())
                .unwrap_or_else(|| mfg.code.clone());
            (mfg.guid, resolved)
        })
        .collect();

    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    let ammo_map = &snap.records.records.multi_feature.ammo_params;

    let mut rows: Vec<WeaponRow> = Vec::new();

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };

        let wp = find_component::<SCItemWeaponComponentParams>(ecd, pools);
        let Some(wp) = wp else { continue };

        // Only FireSingle primary action
        if wp.fire_actions.is_empty() {
            continue;
        }
        let action = &wp.fire_actions[0];
        let single = match action {
            SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => h.get(pools),
            _ => continue,
        };
        let Some(single) = single else { continue };
        let has_launch_params = single.launch_params.is_some();
        let has_explosion_on_action = single.explosion_params.is_some();

        let record_name = record_names.get(&guid).copied().unwrap_or("?");
        let display_name = snap.display_names.get(&guid).unwrap_or("");

        let item_def = find_component::<SAttachableComponentParams>(ecd, pools)
            .and_then(|a| a.attach_def.and_then(|h| h.get(pools)));

        let mfg_name = item_def
            .and_then(|d| d.manufacturer)
            .and_then(|g| mfg_names.get(&g))
            .map(|s| s.as_str())
            .unwrap_or("");

        let connection = wp.connection_params.and_then(|h| h.get(pools));
        let heat = connection
            .and_then(|c| c.simplified_heat_params)
            .and_then(|h| h.get(pools));
        let regen = wp.weapon_regen_consumer_params.and_then(|h| h.get(pools));

        // Resolve ammo: two-hop chain
        let ammo = resolve_ammo(ecd, wp, pools, ecd_map, ammo_map);

        // Extract damage from ammo
        let (
            dmg_phys,
            dmg_energy,
            dmg_dist,
            ammo_speed,
            ammo_lifetime,
            pen_base,
            pen_near,
            pen_far,
            expl_min,
            expl_max,
        ) = ammo
            .map(|a| extract_ammo_data(a, pools))
            .unwrap_or_default();

        // SWeaponStats from connection (pellets, spread)
        let stats = connection.and_then(|c| {
            // noPowerStats is the baseline stats
            c.no_power_stats.and_then(|h| h.get(pools))
        });

        let pellets = stats.map(|s| s.pellets).unwrap_or(0);

        // Overheat calc
        let (max_rounds_overheat, max_time_overheat) = if let Some(h) = heat {
            if single.heat_per_shot > 0.0 {
                let rounds = (h.overheat_temperature / single.heat_per_shot).floor();
                let time = rounds / (single.fire_rate as f32 / 60.0);
                (Some(rounds as i32), Some(time))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        // Ammo container for magazine size
        let ammo_container = find_component::<SAmmoContainerComponentParams>(ecd, pools);
        let mag_size = ammo_container.map(|ac| ac.max_ammo_count);

        rows.push(WeaponRow {
            record_name: record_name.to_string(),
            display_name: display_name.to_string(),
            manufacturer: mfg_name.to_string(),
            size: item_def.map(|d| d.size).unwrap_or(0),
            item_type: item_def
                .map(|d| format!("{:?}", d.r#type))
                .unwrap_or_default(),
            item_subtype: item_def
                .map(|d| format!("{:?}", d.sub_type))
                .unwrap_or_default(),
            rpm: single.fire_rate,
            dmg_phys,
            dmg_energy,
            dmg_dist,
            ammo_speed,
            ammo_lifetime,
            mag_size,
            pellets,
            pen_base,
            pen_near,
            pen_far,
            expl_min,
            expl_max,
            spin_up: 0.0,
            spin_down: 0.0,
            heat_per_shot: single.heat_per_shot,
            has_launch_params,
            has_explosion_on_action,
            cool_delay: heat.map(|h| h.time_till_cooling_starts),
            cool_per_sec: heat.map(|h| h.cooling_per_second),
            overheat_temp: heat.map(|h| h.overheat_temperature),
            overheat_fix: heat.map(|h| h.overheat_fix_time),
            after_fix: heat.map(|h| h.temperature_after_overheat_fix),
            max_rounds_overheat,
            max_time_overheat,
            regen_max_ammo: regen.map(|r| r.max_ammo_load),
            regen_max_per_sec: regen.map(|r| r.max_regen_per_sec),
            regen_cooldown: regen.map(|r| r.regeneration_cooldown),
            regen_cost: regen.map(|r| r.regeneration_cost_per_bullet),
            num_fire_actions: wp.fire_actions.len(),
            sustain: match (heat.is_some(), regen.is_some()) {
                (true, false) => "HEAT",
                (false, true) => "ENERGY",
                (true, true) => "BOTH",
                (false, false) => "none",
            }
            .to_string(),
        });
    }

    rows.sort_by(|a, b| {
        // Ship (uppercase) first, then FPS (lowercase)
        let a_ship = a.record_name.starts_with(|c: char| c.is_uppercase());
        let b_ship = b.record_name.starts_with(|c: char| c.is_uppercase());
        b_ship
            .cmp(&a_ship)
            .then(a.size.cmp(&b.size))
            .then(a.record_name.cmp(&b.record_name))
    });

    // Print TSV header
    println!(
        "{}",
        [
            "Record",
            "Display Name",
            "Manufacturer",
            "S",
            "Type",
            "SubType",
            "RPM",
            "Phys",
            "Energy",
            "Dist",
            "Speed",
            "Lifetime",
            "Mag",
            "Pellets",
            "Pen Base",
            "Pen Near",
            "Pen Far",
            "Expl Min",
            "Expl Max",
            "SpinUp",
            "SpinDown",
            "Sustain",
            "HPS",
            "CoolDelay",
            "Cool/s",
            "OHTemp",
            "OHFix",
            "AfterFix",
            "OHRounds",
            "OHTime",
            "CapAmmo",
            "CapRegen/s",
            "CapCool",
            "CapCost",
            "Actions",
            "Launch",
            "ExplOnAction",
        ]
        .join("\t")
    );

    for r in &rows {
        println!(
            "{}",
            [
                r.record_name.replace("EntityClassDefinition.", ""),
                r.display_name.clone(),
                r.manufacturer.clone(),
                r.size.to_string(),
                r.item_type.clone(),
                r.item_subtype.clone(),
                r.rpm.to_string(),
                fmt_f32(r.dmg_phys),
                fmt_f32(r.dmg_energy),
                fmt_f32(r.dmg_dist),
                fmt_f32(r.ammo_speed),
                fmt_f32(r.ammo_lifetime),
                r.mag_size.map(|v| v.to_string()).unwrap_or_default(),
                if r.pellets > 0 {
                    r.pellets.to_string()
                } else {
                    String::new()
                },
                fmt_f32(r.pen_base),
                fmt_f32(r.pen_near),
                fmt_f32(r.pen_far),
                fmt_f32(r.expl_min),
                fmt_f32(r.expl_max),
                fmt_f32(r.spin_up),
                fmt_f32(r.spin_down),
                r.sustain.clone(),
                fmt_f32(r.heat_per_shot),
                fmt_opt(r.cool_delay),
                fmt_opt(r.cool_per_sec),
                fmt_opt(r.overheat_temp),
                fmt_opt(r.overheat_fix),
                fmt_opt(r.after_fix),
                r.max_rounds_overheat
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                r.max_time_overheat
                    .map(|v| format!("{v:.2}"))
                    .unwrap_or_default(),
                r.regen_max_ammo
                    .map(|v| format!("{v:.0}"))
                    .unwrap_or_default(),
                r.regen_max_per_sec
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or_default(),
                r.regen_cooldown
                    .map(|v| format!("{v:.1}"))
                    .unwrap_or_default(),
                r.regen_cost.map(|v| format!("{v:.2}")).unwrap_or_default(),
                r.num_fire_actions.to_string(),
                if r.has_launch_params { "yes" } else { "" }.to_string(),
                if r.has_explosion_on_action { "yes" } else { "" }.to_string(),
            ]
            .join("\t")
        );
    }

    eprintln!("\n{} FireSingle weapons dumped", rows.len());

    Ok(())
}

// ── Types ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
struct WeaponRow {
    record_name: String,
    display_name: String,
    manufacturer: String,
    size: i32,
    item_type: String,
    item_subtype: String,
    rpm: i32,
    dmg_phys: f32,
    dmg_energy: f32,
    dmg_dist: f32,
    ammo_speed: f32,
    ammo_lifetime: f32,
    mag_size: Option<i32>,
    pellets: i32,
    pen_base: f32,
    pen_near: f32,
    pen_far: f32,
    expl_min: f32,
    expl_max: f32,
    spin_up: f32,
    spin_down: f32,
    sustain: String,
    heat_per_shot: f32,
    cool_delay: Option<f32>,
    cool_per_sec: Option<f32>,
    overheat_temp: Option<f32>,
    overheat_fix: Option<f32>,
    after_fix: Option<f32>,
    max_rounds_overheat: Option<i32>,
    max_time_overheat: Option<f32>,
    regen_max_ammo: Option<f32>,
    regen_max_per_sec: Option<f32>,
    regen_cooldown: Option<f32>,
    regen_cost: Option<f32>,
    has_launch_params: bool,
    has_explosion_on_action: bool,
    num_fire_actions: usize,
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn find_component<'a, T: 'static>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a T> {
    for comp in &ecd.components {
        // Try each known component variant
        let ptr = try_extract_component::<T>(comp, pools);
        if ptr.is_some() {
            return ptr;
        }
    }
    None
}

fn try_extract_component<'a, T: 'static>(
    comp: &'a DataForgeComponentParamsPtr,
    pools: &'a DataPools,
) -> Option<&'a T> {
    use std::any::TypeId;
    if TypeId::of::<T>() == TypeId::of::<SCItemWeaponComponentParams>()
        && let DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) = comp
    {
        let r = h.get(pools)?;
        // SAFETY: we just checked TypeId matches
        return Some(unsafe { &*(r as *const SCItemWeaponComponentParams as *const T) });
    }
    if TypeId::of::<T>() == TypeId::of::<SAttachableComponentParams>()
        && let DataForgeComponentParamsPtr::SAttachableComponentParams(h) = comp
    {
        let r = h.get(pools)?;
        return Some(unsafe { &*(r as *const SAttachableComponentParams as *const T) });
    }
    if TypeId::of::<T>() == TypeId::of::<SAmmoContainerComponentParams>()
        && let DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) = comp
    {
        let r = h.get(pools)?;
        return Some(unsafe { &*(r as *const SAmmoContainerComponentParams as *const T) });
    }
    None
}

fn resolve_ammo<'a>(
    ecd: &EntityClassDefinition,
    wp: &SCItemWeaponComponentParams,
    pools: &'a DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<&'a AmmoParams> {
    // Path 1: weapon's ammoContainerRecord → AmmoParams directly
    if let Some(guid) = wp.ammo_container_record {
        if let Some(&ammo_h) = ammo_map.get(&guid)
            && let Some(ammo) = ammo_h.get(pools)
        {
            return Some(ammo);
        }
        // Path 2: two-hop — ammoContainerRecord → EntityClassDefinition → SAmmoContainerComponentParams → ammoParamsRecord
        if let Some(&container_h) = ecd_map.get(&guid)
            && let Some(container_ecd) = container_h.get(pools)
        {
            let ammo_comp = find_component::<SAmmoContainerComponentParams>(container_ecd, pools);
            if let Some(ac) = ammo_comp
                && let Some(ammo_guid) = ac.ammo_params_record
                && let Some(&ammo_h) = ammo_map.get(&ammo_guid)
            {
                return ammo_h.get(pools);
            }
        }
    }

    // Path 3: ammo container on the weapon entity itself
    let ac = find_component::<SAmmoContainerComponentParams>(ecd, pools)?;
    let ammo_guid = ac.ammo_params_record?;
    let &ammo_h = ammo_map.get(&ammo_guid)?;
    ammo_h.get(pools)
}

fn extract_ammo_data(
    ammo: &AmmoParams,
    pools: &DataPools,
) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    let speed = ammo.speed;
    let lifetime = ammo.lifetime;

    let mut phys = 0.0f32;
    let mut energy = 0.0f32;
    let mut dist = 0.0f32;
    let pen_base = 0.0f32;
    let pen_near = 0.0f32;
    let pen_far = 0.0f32;
    let mut expl_min = 0.0f32;
    let mut expl_max = 0.0f32;

    if let Some(proj_ptr) = &ammo.projectile_params {
        if let ProjectileParamsPtr::BulletProjectileParams(h) = proj_ptr {
            if let Some(bullet) = h.get(pools) {
                // Direct damage
                if let Some(dmg_ptr) = &bullet.damage
                    && let DamageBasePtr::DamageInfo(dh) = dmg_ptr
                    && let Some(d) = dh.get(pools)
                {
                    phys += d.damage_physical;
                    energy += d.damage_energy;
                    dist += d.damage_distortion;
                }
                // Penetration
                if let Some(pen_h) = bullet.detonation_params.and_then(|h| h.get(pools))
                    && let Some(expl) = pen_h.explosion_params.and_then(|h| h.get(pools))
                {
                    expl_min = expl.min_radius;
                    expl_max = expl.max_radius;
                    // Explosion damage
                    if let Some(dmg_ptr) = &expl.damage
                        && let DamageBasePtr::DamageInfo(dh) = dmg_ptr
                        && let Some(d) = dh.get(pools)
                    {
                        phys += d.damage_physical;
                        energy += d.damage_energy;
                        dist += d.damage_distortion;
                    }
                }
                // Penetration params are on BulletProjectileParams level
                // Actually they're on the ammo level
            }
        }
    }

    // Penetration is on AmmoParams.projectileParams.BulletProjectileParams
    // But the AmmoPenetrationParams is actually on a different path
    // Check if there's a penetrationParams on the bullet
    // Actually from our struct exploration, penetrationParams is a field
    // that doesn't exist on BulletProjectileParams. It's in a different struct.
    // Let's skip pen for now and verify the path later.

    (
        phys, energy, dist, speed, lifetime, pen_base, pen_near, pen_far, expl_min, expl_max,
    )
}

fn fmt_f32(v: f32) -> String {
    if v == 0.0 {
        String::new()
    } else {
        format!("{v:.2}")
    }
}

fn fmt_opt(v: Option<f32>) -> String {
    v.map(|v| {
        if v == 0.0 {
            String::new()
        } else {
            format!("{v:.2}")
        }
    })
    .unwrap_or_default()
}
