//! Data exploration for the sc-weapons crate design.
//!
//! Parses real DCB data and dumps every weapon record's structure so we
//! can verify assumptions before encoding them into types.
//!
//! Usage:
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example explore_weapons
//!
//! # With reference graph (traces cross-record references):
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example explore_weapons -- --all
//!
//! # Verbose output (heat/regen details per weapon):
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example explore_weapons -- -v
//!
//! # Explicit P4K path:
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example explore_weapons -- "C:/Games/StarCitizen/LIVE/Data.p4k"
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use sc_extract::generated::*;
use sc_extract::{
    AssetConfig, AssetData, AssetSource, DataPools, DatacoreConfig, Guid, RecordStore,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let use_all = args.iter().any(|a| a == "--all");
    let explicit_path: Option<PathBuf> = args.iter().find(|a| !a.starts_with('-')).map(Into::into);

    // ── Parse ────────────────────────────────────────────────────────────
    let t0 = Instant::now();

    let (assets, _meta) = if let Some(p4k_path) = &explicit_path {
        let assets = AssetSource::open(p4k_path)?;
        let meta = sc_extract::SnapshotMeta {
            schema_version: sc_extract::ExtractSnapshot::SCHEMA_VERSION,
            game_version: "unknown".into(),
            build_id: "unknown".into(),
            extracted_at: sc_extract::current_timestamp(),
        };
        (assets, meta)
    } else {
        let install = sc_installs::discover_primary()?;
        println!(
            "Found {} v{} at {}",
            install.channel,
            install.short_version(),
            install.root.display()
        );
        let assets = AssetSource::from_install(&install)?;
        let meta = sc_extract::snapshot_meta_from_install(&install);
        (assets, meta)
    };

    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let dc_config = if use_all {
        println!("Using DatacoreConfig::all() (includes reference graph)");
        DatacoreConfig::all()
    } else {
        DatacoreConfig::standard()
    };
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &dc_config)?;

    let parse_secs = t0.elapsed().as_secs_f64();
    println!("Parse complete in {parse_secs:.1}s\n");

    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;

    // ── Build GUID → record name map from the raw DB ─────────────────────
    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| {
            let name = db.record_name(r)?;
            Some((r.id, name))
        })
        .collect();

    // ── Walk EntityClassDefinition records ────────────────────────────────
    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    println!("EntityClassDefinition records: {}", ecd_map.len());

    let mut ship_weapons: Vec<WeaponInfo> = Vec::new();
    let mut fps_weapons: Vec<WeaponInfo> = Vec::new();
    let mut other_weapons: Vec<WeaponInfo> = Vec::new();
    let mut no_weapon_component = 0u32;

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };

        let record_name = record_names.get(&guid).copied().unwrap_or("<unknown>");

        // Find SAttachableComponentParams → SItemDefinition
        let item_def =
            find_attachable(ecd, pools).and_then(|a| a.attach_def.and_then(|h| h.get(pools)));

        // Find SCItemWeaponComponentParams
        let weapon_params = find_weapon_params(ecd, pools);

        if weapon_params.is_none() {
            no_weapon_component += 1;
            continue;
        }

        let weapon_params = weapon_params.unwrap();
        let info = build_weapon_info(guid, record_name, ecd, item_def, weapon_params, pools);

        match (&info.item_type, &info.item_subtype) {
            (Some(t), Some(st))
                if *t == EItemType::WeaponGun
                    && matches!(st, EItemSubType::Gun | EItemSubType::Rocket) =>
            {
                ship_weapons.push(info);
            }
            (Some(t), Some(st))
                if *t == EItemType::WeaponPersonal && *st != EItemSubType::Gadget =>
            {
                fps_weapons.push(info);
            }
            _ => {
                other_weapons.push(info);
            }
        }
    }

    // Sort by record name for stable output
    ship_weapons.sort_by(|a, b| a.name.cmp(&b.name));
    fps_weapons.sort_by(|a, b| a.name.cmp(&b.name));
    other_weapons.sort_by(|a, b| a.name.cmp(&b.name));

    println!("  without SCItemWeaponComponentParams: {no_weapon_component}");
    println!(
        "  ship weapons (WeaponGun + Gun/Rocket): {}",
        ship_weapons.len()
    );
    println!(
        "  FPS weapons (WeaponPersonal, !Gadget): {}",
        fps_weapons.len()
    );
    println!(
        "  other (has weapon component but different type/subtype): {}",
        other_weapons.len()
    );

    // ── Ship weapons ─────────────────────────────────────────────────────
    println!("\n{}", "=".repeat(80));
    println!("SHIP WEAPONS ({} total)", ship_weapons.len());
    println!("{}", "=".repeat(80));
    print_weapon_table(&ship_weapons, verbose);

    // ── FPS weapons ──────────────────────────────────────────────────────
    println!("\n{}", "=".repeat(80));
    println!("FPS WEAPONS ({} total)", fps_weapons.len());
    println!("{}", "=".repeat(80));
    print_weapon_table(&fps_weapons, verbose);

    // ── Other weapon-component entities ──────────────────────────────────
    if !other_weapons.is_empty() {
        println!("\n{}", "=".repeat(80));
        println!(
            "OTHER WEAPON-COMPONENT ENTITIES ({} total)",
            other_weapons.len()
        );
        println!("{}", "=".repeat(80));
        for w in &other_weapons {
            println!(
                "  {:<60} type={:<20} sub={:<20}",
                w.name,
                w.item_type
                    .as_ref()
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(|| "None".into()),
                w.item_subtype
                    .as_ref()
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(|| "None".into()),
            );
        }
    }

    // ── Ammo reference chain summary ─────────────────────────────────────
    println!("\n{}", "=".repeat(80));
    println!("AMMO REFERENCE CHAIN ANALYSIS");
    println!("{}", "=".repeat(80));
    analyze_ammo_chain(&ship_weapons, &fps_weapons, pools, &snap.records);

    // ── Heat vs energy classification ────────────────────────────────────
    println!("\n{}", "=".repeat(80));
    println!("SUSTAIN MODEL CLASSIFICATION (ship weapons only)");
    println!("{}", "=".repeat(80));
    analyze_sustain_models(&ship_weapons);

    // ── Fire action variants ─────────────────────────────────────────────
    println!("\n{}", "=".repeat(80));
    println!("FIRE ACTION VARIANTS");
    println!("{}", "=".repeat(80));
    analyze_fire_actions(&ship_weapons, &fps_weapons);

    Ok(())
}

// ── Types ────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct WeaponInfo<'a> {
    #[allow(dead_code)]
    guid: Guid,
    name: &'a str,
    item_type: Option<EItemType>,
    item_subtype: Option<EItemSubType>,
    size: Option<i32>,
    grade: Option<i32>,
    manufacturer: Option<Guid>,
    fire_rate_rpm: Option<i32>,
    heat_per_shot: Option<f32>,
    fire_action_type: Option<String>,
    has_simplified_heat: bool,
    has_regen_consumer: bool,
    ammo_container_ref: Option<Guid>,
    regen_info: Option<RegenInfo>,
    heat_info: Option<HeatInfo>,
    num_fire_actions: usize,
    num_components: usize,
}

#[derive(Debug)]
struct RegenInfo {
    regeneration_cooldown: f32,
    regeneration_cost_per_bullet: f32,
    max_ammo_load: f32,
    max_regen_per_sec: f32,
    requested_ammo_load: f32,
}

#[derive(Debug)]
struct HeatInfo {
    overheat_temperature: f32,
    cooling_per_second: f32,
    overheat_fix_time: f32,
    temperature_after_overheat_fix: f32,
    time_till_cooling_starts: f32,
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn find_attachable<'a>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SAttachableComponentParams> {
    for comp in &ecd.components {
        match comp {
            DataForgeComponentParamsPtr::SAttachableComponentParams(h) => {
                return h.get(pools);
            }
            _ => continue,
        }
    }
    None
}

fn find_weapon_params<'a>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SCItemWeaponComponentParams> {
    for comp in &ecd.components {
        match comp {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => {
                return h.get(pools);
            }
            _ => continue,
        }
    }
    None
}

fn find_ammo_container<'a>(
    ecd: &'a EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SAmmoContainerComponentParams> {
    for comp in &ecd.components {
        match comp {
            DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => {
                return h.get(pools);
            }
            _ => continue,
        }
    }
    None
}

fn build_weapon_info<'a>(
    guid: Guid,
    name: &'a str,
    ecd: &EntityClassDefinition,
    item_def: Option<&SItemDefinition>,
    wp: &SCItemWeaponComponentParams,
    pools: &DataPools,
) -> WeaponInfo<'a> {
    // Fire rate from the first fire action
    let (fire_rate_rpm, fire_action_type, heat_per_shot_from_action) = extract_fire_info(wp, pools);

    // Heat params from connection_params → simplified_heat_params
    let connection = wp.connection_params.and_then(|h| h.get(pools));
    let heat_params = connection
        .and_then(|c| c.simplified_heat_params)
        .and_then(|h| h.get(pools));

    // Regen consumer params
    let regen = wp.weapon_regen_consumer_params.and_then(|h| h.get(pools));

    // Ammo container ref — check both on weapon component and on the
    // SAmmoContainerComponentParams component
    let ammo_ref = wp
        .ammo_container_record
        .or_else(|| find_ammo_container(ecd, pools).and_then(|ac| ac.ammo_params_record));

    WeaponInfo {
        guid,
        name,
        item_type: item_def.map(|d| d.r#type.clone()),
        item_subtype: item_def.map(|d| d.sub_type.clone()),
        size: item_def.map(|d| d.size),
        grade: item_def.map(|d| d.grade),
        manufacturer: item_def.and_then(|d| d.manufacturer),
        fire_rate_rpm,
        heat_per_shot: heat_per_shot_from_action,
        fire_action_type,
        has_simplified_heat: heat_params.is_some(),
        has_regen_consumer: regen.is_some(),
        ammo_container_ref: ammo_ref,
        regen_info: regen.map(|r| RegenInfo {
            regeneration_cooldown: r.regeneration_cooldown,
            regeneration_cost_per_bullet: r.regeneration_cost_per_bullet,
            max_ammo_load: r.max_ammo_load,
            max_regen_per_sec: r.max_regen_per_sec,
            requested_ammo_load: r.requested_ammo_load,
        }),
        heat_info: heat_params.map(|h| HeatInfo {
            overheat_temperature: h.overheat_temperature,
            cooling_per_second: h.cooling_per_second,
            overheat_fix_time: h.overheat_fix_time,
            temperature_after_overheat_fix: h.temperature_after_overheat_fix,
            time_till_cooling_starts: h.time_till_cooling_starts,
        }),
        num_fire_actions: wp.fire_actions.len(),
        num_components: ecd.components.len(),
    }
}

fn extract_fire_info(
    wp: &SCItemWeaponComponentParams,
    pools: &DataPools,
) -> (Option<i32>, Option<String>, Option<f32>) {
    if wp.fire_actions.is_empty() {
        return (None, None, None);
    }

    let action = &wp.fire_actions[0];
    match action {
        SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
            let p = h.get(pools);
            (
                p.map(|p| p.fire_rate),
                Some("FireRapid".into()),
                p.map(|p| p.heat_per_shot),
            )
        }
        SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
            let p = h.get(pools);
            (p.map(|p| p.fire_rate), Some("FireSingle".into()), None)
        }
        SWeaponActionParamsPtr::SWeaponActionFireBeamParams(_) => {
            (None, Some("FireBeam".into()), None)
        }
        #[cfg(any(feature = "entities-scitem-ships", feature = "entities-scitem-weapons",))]
        SWeaponActionParamsPtr::SWeaponActionFireHealingBeamParams(_) => {
            (None, Some("HealingBeam".into()), None)
        }
        SWeaponActionParamsPtr::SWeaponActionSequenceParams(_) => {
            (None, Some("Sequence".into()), None)
        }
        _ => (None, Some(format!("Other({action:?})")), None),
    }
}

fn print_weapon_table(weapons: &[WeaponInfo], verbose: bool) {
    if weapons.is_empty() {
        println!("  (none)");
        return;
    }

    for w in weapons {
        let sustain = match (w.has_simplified_heat, w.has_regen_consumer) {
            (true, false) => "HEAT",
            (false, true) => "ENERGY",
            (true, true) => "BOTH!",
            (false, false) => "none",
        };

        println!(
            "  {:<55} S{} G{} {:>6}rpm {:>8} {:>10} ammo={} comps={}",
            w.name,
            w.size.unwrap_or(-1),
            w.grade.unwrap_or(-1),
            w.fire_rate_rpm.unwrap_or(0),
            w.fire_action_type.as_deref().unwrap_or("?"),
            sustain,
            if w.ammo_container_ref.is_some() {
                "yes"
            } else {
                "no "
            },
            w.num_components,
        );

        if verbose {
            if let Some(ref h) = w.heat_info {
                println!(
                    "    HEAT: overheat={:.1} cool/s={:.1} fix_time={:.1}s \
                     after_fix={:.1} cool_delay={:.1}s",
                    h.overheat_temperature,
                    h.cooling_per_second,
                    h.overheat_fix_time,
                    h.temperature_after_overheat_fix,
                    h.time_till_cooling_starts,
                );
            }
            if let Some(hps) = w.heat_per_shot {
                println!("    heat_per_shot (on fire action): {hps:.4}");
            }
            if let Some(ref r) = w.regen_info {
                println!(
                    "    REGEN: cooldown={:.1}s cost/bullet={:.2} max_load={:.0} \
                     max_regen/s={:.1} requested={:.0}",
                    r.regeneration_cooldown,
                    r.regeneration_cost_per_bullet,
                    r.max_ammo_load,
                    r.max_regen_per_sec,
                    r.requested_ammo_load,
                );
            }
        }
    }
}

fn analyze_ammo_chain(
    ship: &[WeaponInfo],
    fps: &[WeaponInfo],
    pools: &DataPools,
    store: &RecordStore,
) {
    let all: Vec<&WeaponInfo> = ship.iter().chain(fps.iter()).collect();
    let with_ammo = all
        .iter()
        .filter(|w| w.ammo_container_ref.is_some())
        .count();
    let without_ammo = all.len() - with_ammo;

    println!("  weapons with ammo reference: {with_ammo}");
    println!("  weapons without ammo reference: {without_ammo}");

    // Try to resolve some ammo references against the ammo pool
    let ammo_map = &store.records.multi_feature.ammo_params;
    println!("  AmmoParams records in pool: {}", ammo_map.len());

    let mut resolved = 0u32;
    let mut unresolved = 0u32;
    let mut projectile_types: HashMap<String, u32> = HashMap::new();

    for w in &all {
        let Some(ammo_guid) = w.ammo_container_ref else {
            continue;
        };
        if let Some(&ammo_handle) = ammo_map.get(&ammo_guid) {
            if let Some(ammo) = ammo_handle.get(pools) {
                resolved += 1;
                let proj_type = match &ammo.projectile_params {
                    Some(ProjectileParamsPtr::BulletProjectileParams(_)) => "Bullet",
                    Some(ProjectileParamsPtr::ProjectileParams(_)) => "ProjectileBase",
                    Some(ProjectileParamsPtr::Unknown { .. }) => "Unknown",
                    None => "None",
                    #[allow(unreachable_patterns)]
                    Some(_) => "Other",
                };
                *projectile_types.entry(proj_type.into()).or_default() += 1;
            } else {
                unresolved += 1;
            }
        } else {
            // GUID points to a record not in the ammo pool — maybe it's an
            // ammo container entity, not an AmmoParams directly
            unresolved += 1;
        }
    }

    println!("  ammo resolved to AmmoParams: {resolved}");
    println!("  ammo unresolved: {unresolved}");
    println!("  projectile_params types:");
    let mut sorted: Vec<_> = projectile_types.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (t, c) in &sorted {
        println!("    {t}: {c}");
    }
}

fn analyze_sustain_models(ship: &[WeaponInfo]) {
    let heat_only = ship
        .iter()
        .filter(|w| w.has_simplified_heat && !w.has_regen_consumer)
        .count();
    let energy_only = ship
        .iter()
        .filter(|w| !w.has_simplified_heat && w.has_regen_consumer)
        .count();
    let both = ship
        .iter()
        .filter(|w| w.has_simplified_heat && w.has_regen_consumer)
        .count();
    let neither = ship
        .iter()
        .filter(|w| !w.has_simplified_heat && !w.has_regen_consumer)
        .count();

    println!("  heat only (ballistic model): {heat_only}");
    println!("  energy only (regen/capacitor model): {energy_only}");
    println!("  both heat AND energy: {both}");
    println!("  neither: {neither}");

    if both > 0 {
        println!("\n  Weapons with BOTH heat and energy (unexpected):");
        for w in ship
            .iter()
            .filter(|w| w.has_simplified_heat && w.has_regen_consumer)
        {
            println!("    {}", w.name);
        }
    }

    if neither > 0 {
        println!("\n  Weapons with NEITHER heat nor energy:");
        for w in ship
            .iter()
            .filter(|w| !w.has_simplified_heat && !w.has_regen_consumer)
        {
            println!(
                "    {:<55} {}",
                w.name,
                w.fire_action_type.as_deref().unwrap_or("?")
            );
        }
    }
}

fn analyze_fire_actions(ship: &[WeaponInfo], fps: &[WeaponInfo]) {
    let mut action_counts: HashMap<String, u32> = HashMap::new();
    let mut multi_action = 0u32;

    for w in ship.iter().chain(fps.iter()) {
        if let Some(ref t) = w.fire_action_type {
            *action_counts.entry(t.clone()).or_default() += 1;
        }
        if w.num_fire_actions > 1 {
            multi_action += 1;
        }
    }

    println!("  primary fire action types:");
    let mut sorted: Vec<_> = action_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (t, c) in &sorted {
        println!("    {t}: {c}");
    }
    println!("  weapons with >1 fire action: {multi_action}");
}
