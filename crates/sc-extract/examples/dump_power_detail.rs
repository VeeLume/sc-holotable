//! Detailed power state multiplier dump per ship weapon.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example dump_power_detail
//! ```

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{
    AssetConfig, AssetData, AssetSource, DataPools, DatacoreConfig, Guid,
};

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

    let mfg_names: HashMap<Guid, String> = snap.manufacturers.all()
        .map(|mfg| {
            let resolved = mfg.name_key.as_deref()
                .and_then(|k| asset_data.locale.resolve(&sc_extract::LocaleKey::new(k)))
                .map(|s| s.to_string())
                .unwrap_or_else(|| mfg.code.clone());
            (mfg.guid, resolved)
        }).collect();

    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // TSV header
    println!("Name\tDisplay\tMfg\tS\tType\tSustain\tnoPwr.dmg\tnoPwr.rateM\tunder.dmg\tunder.rateM\tover.rateM\tclock.rateM\theat.dmg\theat.rateM\theat.ammoCostM\theat.speedM");

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else { continue };
        let name = record_names.get(&guid).copied().unwrap_or("?").replace("EntityClassDefinition.", "");

        // Only ship weapons (uppercase)
        if !name.starts_with(|c: char| c.is_uppercase()) { continue; }
        // Skip CMLs
        if name.contains("CML") { continue; }

        let Some(wp) = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        }) else { continue };

        let base = dedup_name(&name);
        if seen.contains(&base) { continue; }
        seen.insert(base);

        let display = snap.display_names.get(&guid).unwrap_or("").to_string();

        let item_def = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SAttachableComponentParams(h) =>
                h.get(pools).and_then(|a| a.attach_def.and_then(|h| h.get(pools))),
            _ => None,
        });

        let size = item_def.map(|d| d.size).unwrap_or(0);
        let item_type = item_def.map(|d| format!("{:?}", d.r#type)).unwrap_or_default();
        let mfg = item_def.and_then(|d| d.manufacturer).and_then(|g| mfg_names.get(&g)).map(|s| s.as_str()).unwrap_or("");

        let connection = wp.connection_params.and_then(|h| h.get(pools));
        let has_heat = connection.and_then(|c| c.simplified_heat_params).and_then(|h| h.get(pools)).is_some();
        let has_regen = wp.weapon_regen_consumer_params.and_then(|h| h.get(pools)).is_some();
        let sustain = match (has_heat, has_regen) {
            (true, false) => "HEAT", (false, true) => "ENERGY", (true, true) => "BOTH", (false, false) => "none",
        };

        let conn = connection;

        let np = conn.and_then(|c| c.no_power_stats.and_then(|h| h.get(pools)));
        let up = conn.and_then(|c| c.underpower_stats.and_then(|h| h.get(pools)));
        let op = conn.and_then(|c| c.overpower_stats.and_then(|h| h.get(pools)));
        let oc = conn.and_then(|c| c.overclock_stats.and_then(|h| h.get(pools)));
        let hs = conn.and_then(|c| c.heat_stats.and_then(|h| h.get(pools)));

        println!("{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",
            name, display, mfg, size, item_type, sustain,
            np.map(|s| s.damage_multiplier).unwrap_or(f32::NAN),
            np.map(|s| s.fire_rate_multiplier).unwrap_or(f32::NAN),
            up.map(|s| s.damage_multiplier).unwrap_or(f32::NAN),
            up.map(|s| s.fire_rate_multiplier).unwrap_or(f32::NAN),
            op.map(|s| s.fire_rate_multiplier).unwrap_or(f32::NAN),
            oc.map(|s| s.fire_rate_multiplier).unwrap_or(f32::NAN),
            hs.map(|s| s.damage_multiplier).unwrap_or(f32::NAN),
            hs.map(|s| s.fire_rate_multiplier).unwrap_or(f32::NAN),
            hs.map(|s| s.ammo_cost_multiplier).unwrap_or(f32::NAN),
            hs.map(|s| s.projectile_speed_multiplier).unwrap_or(f32::NAN),
        );
    }

    Ok(())
}

fn dedup_name(name: &str) -> String {
    if name.starts_with(|c: char| c.is_uppercase()) { return name.to_string(); }
    if let Some(pos) = name.find("_01_") { return name[..pos + 3].to_string(); }
    if let Some(pos) = name.find("_02_") { return name[..pos + 3].to_string(); }
    name.to_string()
}
