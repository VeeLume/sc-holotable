//! Dump all FireBeam weapons.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_beam
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

    let mfg_names: HashMap<Guid, String> = snap
        .manufacturers
        .all()
        .map(|mfg| {
            let resolved = mfg
                .name_key
                .as_deref()
                .and_then(|k| asset_data.locale.resolve(&sc_extract::LocaleKey::new(k)))
                .map(|s| s.to_string())
                .unwrap_or_else(|| mfg.code.clone());
            (mfg.guid, resolved)
        })
        .collect();

    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;

    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    println!(
        "Name\tDisplay\tManufacturer\tS\tType\tSubType\tSustain\tToggle\tDPS_Phys\tDPS_Energy\tDPS_Dist\tDPS_Therm\tDPS_Bio\tDPS_Stun\tFullRange\tZeroRange\tHPS\tMinDraw\tMaxDraw\tHitRadius\tHitType\tAmmoType\tActions"
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

        // Find FireBeam in any fire action slot
        let mut beam = None;
        for action in &wp.fire_actions {
            if let SWeaponActionParamsPtr::SWeaponActionFireBeamParams(h) = action {
                beam = h.get(pools);
                break;
            }
        }
        let Some(beam) = beam else { continue };

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
        let item_subtype = item_def
            .map(|d| format!("{:?}", d.sub_type))
            .unwrap_or_default();
        let mfg = item_def
            .and_then(|d| d.manufacturer)
            .and_then(|g| mfg_names.get(&g))
            .map(|s| s.as_str())
            .unwrap_or("");

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

        // DPS from beam's damagePerSecond
        let (dp, de, dd, dt, db_val, ds) = beam
            .damage_per_second
            .as_ref()
            .map(|ptr| match ptr {
                DamageBasePtr::DamageInfo(h) => h
                    .get(pools)
                    .map(|d| {
                        (
                            d.damage_physical,
                            d.damage_energy,
                            d.damage_distortion,
                            d.damage_thermal,
                            d.damage_biochemical,
                            d.damage_stun,
                        )
                    })
                    .unwrap_or_default(),
                _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            })
            .unwrap_or_default();

        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{:.1}\t{:.1}\t{:.1}\t{:.1}\t{:.0}\t{:.0}\t{:.2}\t{:.1}\t{:.1}\t{:.2}\t{}\t{:?}\t{}",
            name,
            display,
            mfg,
            size,
            item_type,
            item_subtype,
            sustain,
            beam.toggle,
            dp,
            de,
            dd,
            dt,
            db_val,
            ds,
            beam.full_damage_range,
            beam.zero_damage_range,
            beam.heat_per_second,
            beam.min_energy_draw,
            beam.max_energy_draw,
            beam.hit_radius,
            beam.hit_type,
            beam.ammo_type,
            wp.fire_actions.len(),
        );
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
