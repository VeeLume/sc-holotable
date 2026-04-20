//! Dump all Sequence weapons with their sequence entry structure.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_sequence
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

    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    let ammo_map = &snap.records.records.multi_feature.ammo_params;

    // Collect unique sequence patterns (skip skins)
    let mut seen_bases: std::collections::HashSet<String> = std::collections::HashSet::new();

    let mut entries: Vec<(String, String, String, SequenceInfo)> = Vec::new();

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else { continue };
        let name = record_names.get(&guid).copied().unwrap_or("?").replace("EntityClassDefinition.", "");
        let display = snap.display_names.get(&guid).unwrap_or("").to_string();

        let Some(wp) = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        }) else { continue };

        if wp.fire_actions.is_empty() { continue; }
        let action = &wp.fire_actions[0];
        let seq = match action {
            SWeaponActionParamsPtr::SWeaponActionSequenceParams(h) => h.get(pools),
            _ => continue,
        };
        let Some(seq) = seq else { continue };

        // Dedup — skip obvious skins
        let base = dedup_name(&name);
        if seen_bases.contains(&base) { continue; }
        seen_bases.insert(base);

        let item_def = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SAttachableComponentParams(h) => {
                h.get(pools).and_then(|a| a.attach_def.and_then(|h| h.get(pools)))
            }
            _ => None,
        });

        let size = item_def.map(|d| d.size).unwrap_or(0);
        let item_type = item_def.map(|d| format!("{:?}", d.r#type)).unwrap_or_default();

        // Extract sequence details
        let mut seq_entries = Vec::new();
        for entry_h in &seq.sequence_entries {
            if let Some(entry) = entry_h.get(pools) {
                let inner_action = entry.weapon_action.as_ref().map(|a| describe_action(a, pools));
                seq_entries.push(SeqEntry {
                    delay: entry.delay,
                    unit: format!("{:?}", entry.unit),
                    repetitions: entry.repetitions,
                    inner_action: inner_action.unwrap_or_else(|| "None".into()),
                });
            }
        }

        // Sustain model
        let connection = wp.connection_params.and_then(|h| h.get(pools));
        let has_heat = connection.and_then(|c| c.simplified_heat_params).and_then(|h| h.get(pools)).is_some();
        let has_regen = wp.weapon_regen_consumer_params.and_then(|h| h.get(pools)).is_some();
        let sustain = match (has_heat, has_regen) {
            (true, false) => "HEAT",
            (false, true) => "ENERGY",
            (true, true) => "BOTH",
            (false, false) => "none",
        };

        // Ammo resolution
        let ammo = resolve_ammo(ecd, wp, pools, ecd_map, ammo_map);
        let (dmg_phys, dmg_energy, dmg_dist) = ammo.map(|a| extract_damage(a, pools)).unwrap_or_default();

        let info = SequenceInfo {
            size,
            item_type,
            sustain: sustain.to_string(),
            mode: format!("{:?}", seq.mode),
            seq_entries,
            dmg_phys, dmg_energy, dmg_dist,
            num_fire_actions: wp.fire_actions.len(),
        };

        entries.push((name, display, sustain.to_string(), info));
    }

    entries.sort_by(|a, b| a.3.item_type.cmp(&b.3.item_type).then(a.3.size.cmp(&b.3.size)).then(a.0.cmp(&b.0)));

    // Print
    for (name, display, _, info) in &entries {
        println!("{} ({}) S{} {} {} actions={}", name, display, info.size, info.item_type, info.sustain, info.num_fire_actions);
        println!("  mode: {} | alpha: phys={:.1} energy={:.1} dist={:.1}", info.mode, info.dmg_phys, info.dmg_energy, info.dmg_dist);
        for (i, e) in info.seq_entries.iter().enumerate() {
            println!("  seq[{}]: delay={:.3} {} reps={} -> {}", i, e.delay, e.unit, e.repetitions, e.inner_action);
        }
        println!();
    }

    // Summary stats
    let mut mode_counts: HashMap<String, u32> = HashMap::new();
    let mut unit_counts: HashMap<String, u32> = HashMap::new();
    let mut inner_type_counts: HashMap<String, u32> = HashMap::new();
    let mut entry_count_hist: HashMap<usize, u32> = HashMap::new();

    for (_, _, _, info) in &entries {
        *mode_counts.entry(info.mode.clone()).or_default() += 1;
        *entry_count_hist.entry(info.seq_entries.len()).or_default() += 1;
        for e in &info.seq_entries {
            *unit_counts.entry(e.unit.clone()).or_default() += 1;
            let inner_short = e.inner_action.split_whitespace().next().unwrap_or("?").to_string();
            *inner_type_counts.entry(inner_short).or_default() += 1;
        }
    }

    println!("=== SUMMARY ({} unique base Sequence weapons) ===", entries.len());
    println!("\nSequence modes:");
    for (k, v) in &mode_counts { println!("  {}: {}", k, v); }
    println!("\nDelay units (across all entries):");
    for (k, v) in &unit_counts { println!("  {}: {}", k, v); }
    println!("\nInner action types:");
    let mut inner_sorted: Vec<_> = inner_type_counts.into_iter().collect();
    inner_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (k, v) in &inner_sorted { println!("  {}: {}", k, v); }
    println!("\nSequence entry counts:");
    let mut hist_sorted: Vec<_> = entry_count_hist.into_iter().collect();
    hist_sorted.sort();
    for (k, v) in &hist_sorted { println!("  {} entries: {} weapons", k, v); }

    Ok(())
}

struct SequenceInfo {
    size: i32,
    item_type: String,
    sustain: String,
    mode: String,
    seq_entries: Vec<SeqEntry>,
    dmg_phys: f32,
    dmg_energy: f32,
    dmg_dist: f32,
    num_fire_actions: usize,
}

struct SeqEntry {
    delay: f32,
    unit: String,
    repetitions: i32,
    inner_action: String,
}

fn describe_action(action: &SWeaponActionParamsPtr, pools: &DataPools) -> String {
    match action {
        SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
            h.get(pools).map(|r| format!("FireRapid rpm={} hps={:.2} spin={:.2}/{:.2}", r.fire_rate, r.heat_per_shot, r.spin_up_time, r.spin_down_time))
                .unwrap_or("FireRapid(?)".into())
        }
        SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
            h.get(pools).map(|s| format!("FireSingle rpm={} hps={:.2} launch={}", s.fire_rate, s.heat_per_shot, s.launch_params.is_some()))
                .unwrap_or("FireSingle(?)".into())
        }
        SWeaponActionParamsPtr::SWeaponActionFireBeamParams(_) => "FireBeam".into(),
        SWeaponActionParamsPtr::SWeaponActionSequenceParams(_) => "Sequence(nested!)".into(),
        SWeaponActionParamsPtr::SWeaponActionFireBurstParams(h) => {
            h.get(pools).map(|b| format!("FireBurst rpm={} shots={}", b.fire_rate, b.shot_count))
                .unwrap_or("FireBurst(?)".into())
        }
        _ => "Other".into(),
    }
}

fn dedup_name(name: &str) -> String {
    // Strip common skin suffixes to get base name
    let n = name.to_string();
    // For ship weapons (UPPERCASE), just return as-is (few duplicates)
    if n.starts_with(|c: char| c.is_uppercase()) { return n; }
    // For FPS, strip after the model number
    if let Some(pos) = n.find("_01_") {
        return n[..pos + 3].to_string();
    }
    if let Some(pos) = n.find("_02_") {
        return n[..pos + 3].to_string();
    }
    n
}

fn resolve_ammo<'a>(
    ecd: &EntityClassDefinition,
    wp: &SCItemWeaponComponentParams,
    pools: &'a DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<&'a AmmoParams> {
    if let Some(guid) = wp.ammo_container_record {
        if let Some(&ammo_h) = ammo_map.get(&guid) {
            if let Some(ammo) = ammo_h.get(pools) { return Some(ammo); }
        }
        if let Some(&container_h) = ecd_map.get(&guid) {
            if let Some(container_ecd) = container_h.get(pools) {
                let ac = container_ecd.components.iter().find_map(|c| match c {
                    DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
                    _ => None,
                });
                if let Some(ac) = ac {
                    if let Some(ammo_guid) = ac.ammo_params_record {
                        if let Some(&ammo_h) = ammo_map.get(&ammo_guid) {
                            return ammo_h.get(pools);
                        }
                    }
                }
            }
        }
    }
    let ac = ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
        _ => None,
    })?;
    let &ammo_h = ammo_map.get(&ac.ammo_params_record?)?;
    ammo_h.get(pools)
}

fn extract_damage(ammo: &AmmoParams, pools: &DataPools) -> (f32, f32, f32) {
    let mut p = 0.0f32; let mut e = 0.0f32; let mut d = 0.0f32;
    if let Some(ProjectileParamsPtr::BulletProjectileParams(h)) = &ammo.projectile_params {
        if let Some(bullet) = h.get(pools) {
            if let Some(DamageBasePtr::DamageInfo(dh)) = &bullet.damage {
                if let Some(di) = dh.get(pools) { p += di.damage_physical; e += di.damage_energy; d += di.damage_distortion; }
            }
            if let Some(det) = bullet.detonation_params.and_then(|h| h.get(pools)) {
                if let Some(expl) = det.explosion_params.and_then(|h| h.get(pools)) {
                    if let Some(DamageBasePtr::DamageInfo(dh)) = &expl.damage {
                        if let Some(di) = dh.get(pools) { p += di.damage_physical; e += di.damage_energy; d += di.damage_distortion; }
                    }
                }
            }
        }
    }
    (p, e, d)
}
