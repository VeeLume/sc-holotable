//! Dump all FireCharged weapons.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_charged
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

    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    println!(
        "Name\tDisplay\tS\tType\tSustain\tChargeTime\tOverchargeTime\tOverchargedTime\tCooldown\tAutoCharge\tAutoFire\tFullOnly\tInterpolate\tInnerAction\tInnerRPM\tMaxChargeMod\tPhys\tEnergy\tDist\tSpeed\tLifetime\tActions"
    );

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

        // Find FireCharged — check all fire actions, not just first
        let mut charged_idx = None;
        for (i, action) in wp.fire_actions.iter().enumerate() {
            if matches!(
                action,
                SWeaponActionParamsPtr::SWeaponActionFireChargedParams(_)
            ) {
                charged_idx = Some(i);
                break;
            }
        }
        let Some(idx) = charged_idx else { continue };

        let charged = match &wp.fire_actions[idx] {
            SWeaponActionParamsPtr::SWeaponActionFireChargedParams(h) => h.get(pools),
            _ => None,
        };
        let Some(charged) = charged else { continue };

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
        let item_type = item_def
            .map(|d| format!("{:?}", d.r#type))
            .unwrap_or_default();

        let connection = wp.connection_params.and_then(|h| h.get(pools));
        let has_heat = connection
            .and_then(|c| c.simplified_heat_params)
            .and_then(|h| h.get(pools))
            .is_some();
        let has_regen = wp
            .weapon_regen_consumer_params
            .and_then(|h| h.get(pools))
            .is_some();
        let sustain = match (has_heat, has_regen) {
            (true, false) => "HEAT",
            (false, true) => "ENERGY",
            (true, true) => "BOTH",
            (false, false) => "none",
        };

        // Inner action
        let inner_desc = charged
            .weapon_action
            .as_ref()
            .map(|a| describe_action(a, pools))
            .unwrap_or("None".into());

        // Max charge modifier
        let charge_mod = charged.max_charge_modifier.and_then(|h| h.get(pools));
        let mod_desc = charge_mod
            .map(|m| {
                let mut parts = Vec::new();
                if m.damage_multiplier != 1.0 && m.damage_multiplier != 0.0 {
                    parts.push(format!("dmg={:.2}x", m.damage_multiplier));
                }
                if m.fire_rate != 0 {
                    parts.push(format!("rpm={}", m.fire_rate));
                }
                if m.projectile_speed_multiplier != 1.0 && m.projectile_speed_multiplier != 0.0 {
                    parts.push(format!("speed={:.2}x", m.projectile_speed_multiplier));
                }
                if m.pellets != 0 {
                    parts.push(format!("pellets={}", m.pellets));
                }
                if m.ammo_cost != 0 {
                    parts.push(format!("ammoCost={}", m.ammo_cost));
                }
                if m.ammo_cost_multiplier != 1.0 && m.ammo_cost_multiplier != 0.0 {
                    parts.push(format!("ammoCostMult={:.2}", m.ammo_cost_multiplier));
                }
                if m.heat_generation_multiplier != 1.0 && m.heat_generation_multiplier != 0.0 {
                    parts.push(format!("heatMult={:.2}", m.heat_generation_multiplier));
                }
                if m.charge_time_multiplier != 1.0 && m.charge_time_multiplier != 0.0 {
                    parts.push(format!("chargeMult={:.2}", m.charge_time_multiplier));
                }
                if parts.is_empty() {
                    "default".into()
                } else {
                    parts.join(" ")
                }
            })
            .unwrap_or("none".into());

        // Ammo
        let ammo = resolve_ammo(ecd, wp, pools, ecd_map, ammo_map);
        let (p, e, d, spd, lt) = ammo
            .map(|a| {
                let (dp, de, dd) = extract_damage(a, pools);
                (dp, de, dd, a.speed, a.lifetime)
            })
            .unwrap_or_default();

        println!(
            "{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{}\t{}\t{}\t{}\t{}\t\t{}\t{:.1}\t{:.1}\t{:.1}\t{:.0}\t{:.1}\t{}",
            name,
            display,
            size,
            item_type,
            sustain,
            charged.charge_time,
            charged.overcharge_time,
            charged.overcharged_time,
            charged.cooldown_time,
            charged.charge_automatically,
            charged.fire_automatically_on_full_charge,
            charged.fire_only_on_full_charge,
            charged.interpolate_charge_bonus,
            inner_desc,
            mod_desc,
            p,
            e,
            d,
            spd,
            lt,
            wp.fire_actions.len(),
        );
    }

    Ok(())
}

fn describe_action(action: &SWeaponActionParamsPtr, pools: &DataPools) -> String {
    match action {
        SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => h
            .get(pools)
            .map(|r| format!("FireRapid(rpm={} hps={:.1})", r.fire_rate, r.heat_per_shot))
            .unwrap_or("FireRapid(?)".into()),
        SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => h
            .get(pools)
            .map(|s| format!("FireSingle(rpm={} hps={:.1})", s.fire_rate, s.heat_per_shot))
            .unwrap_or("FireSingle(?)".into()),
        SWeaponActionParamsPtr::SWeaponActionFireBeamParams(_) => "FireBeam".into(),
        SWeaponActionParamsPtr::SWeaponActionSequenceParams(_) => "Sequence".into(),
        _ => "Other".into(),
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

fn resolve_ammo<'a>(
    ecd: &EntityClassDefinition,
    wp: &SCItemWeaponComponentParams,
    pools: &'a DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<&'a AmmoParams> {
    if let Some(guid) = wp.ammo_container_record {
        if let Some(&h) = ammo_map.get(&guid)
            && let Some(a) = h.get(pools)
        {
            return Some(a);
        }
        if let Some(&ch) = ecd_map.get(&guid)
            && let Some(ce) = ch.get(pools)
        {
            let ac = ce.components.iter().find_map(|c| match c {
                DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
                _ => None,
            });
            if let Some(ac) = ac
                && let Some(g2) = ac.ammo_params_record
                && let Some(&h2) = ammo_map.get(&g2)
            {
                return h2.get(pools);
            }
        }
    }
    let ac = ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
        _ => None,
    })?;
    let &h = ammo_map.get(&ac.ammo_params_record?)?;
    h.get(pools)
}

fn extract_damage(ammo: &AmmoParams, pools: &DataPools) -> (f32, f32, f32) {
    let mut p = 0.0f32;
    let mut e = 0.0f32;
    let mut d = 0.0f32;
    if let Some(ProjectileParamsPtr::BulletProjectileParams(h)) = &ammo.projectile_params
        && let Some(b) = h.get(pools)
    {
        if let Some(DamageBasePtr::DamageInfo(dh)) = &b.damage
            && let Some(di) = dh.get(pools)
        {
            p += di.damage_physical;
            e += di.damage_energy;
            d += di.damage_distortion;
        }
        if let Some(det) = b.detonation_params.and_then(|h| h.get(pools))
            && let Some(expl) = det.explosion_params.and_then(|h| h.get(pools))
            && let Some(DamageBasePtr::DamageInfo(dh)) = &expl.damage
            && let Some(di) = dh.get(pools)
        {
            p += di.damage_physical;
            e += di.damage_energy;
            d += di.damage_distortion;
        }
    }
    (p, e, d)
}
