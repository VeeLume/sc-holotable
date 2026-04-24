//! Benchmark runner — exercises the full sc-extract API surface.

use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Instant;

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, ExtractSnapshot, Guid,
    SnapshotCaptureConfig, SnapshotMeta,
};

use crate::output::BenchResult;

/// Configuration for a benchmark run.
pub struct RunConfig {
    /// Explicit P4K path, or None for auto-discovery.
    pub p4k_path: Option<PathBuf>,
    /// Include reference graph (DatacoreConfig::all()).
    pub include_graph: bool,
    /// Skip AssetData extraction (datacore-only, no display names).
    pub skip_assets: bool,
}

/// Run all benchmark phases and collect results.
pub fn run(config: &RunConfig) -> Result<BenchResult, Box<dyn std::error::Error>> {
    let mut r = BenchResult::default();

    // ── Phase 1: asset open ──────────────────────────────────────────
    let t = Instant::now();
    let (assets, meta) = open_assets(config)?;
    r.asset_open_time = t.elapsed().as_secs_f64();

    // ── Phase 2: asset data extraction ───────────────────────────────
    let t = Instant::now();
    let asset_data = if config.skip_assets {
        AssetData::default()
    } else {
        AssetData::extract(&assets, &AssetConfig::standard())?
    };
    r.asset_extract_time = t.elapsed().as_secs_f64();
    r.locale_entries = asset_data.locale.len();

    // ── Phase 3: datacore parse ──────────────────────────────────────
    let dc_config = if config.include_graph {
        DatacoreConfig::all()
    } else {
        DatacoreConfig::standard()
    };

    let t = Instant::now();
    let datacore = Datacore::parse(&assets, &asset_data, &dc_config)?;
    r.parse_time = t.elapsed().as_secs_f64();

    let snap = datacore.snapshot();
    r.records = snap.records.len();
    r.manufacturers = snap.manufacturers.len();
    r.display_names = snap.display_names.len();
    r.tag_nodes = snap.tag_tree.len();
    r.graph_edges = snap.graph.edge_count();

    // ── Phase 4: tag tree exercise ───────────────────────────────────
    let t = Instant::now();
    exercise_tags(snap, &mut r);
    r.tag_exercise_time = t.elapsed().as_secs_f64();

    // ── Phase 5: manufacturer exercise ───────────────────────────────
    let t = Instant::now();
    exercise_manufacturers(snap, &mut r);
    r.manufacturer_exercise_time = t.elapsed().as_secs_f64();

    // ── Phase 6: display name exercise ───────────────────────────────
    let t = Instant::now();
    exercise_display_names(snap, &mut r);
    r.display_name_exercise_time = t.elapsed().as_secs_f64();

    // ── Phase 7: locale exercise ─────────────────────────────────────
    let t = Instant::now();
    exercise_locale(&asset_data, &mut r);
    r.locale_exercise_time = t.elapsed().as_secs_f64();

    // ── Phase 8: graph exercise ──────────────────────────────────────
    if config.include_graph {
        let t = Instant::now();
        exercise_graph(snap, &mut r);
        r.graph_exercise_time = t.elapsed().as_secs_f64();
    }

    // ── Phase 9: filter predicates ───────────────────────────────────
    let t = Instant::now();
    exercise_filters(&datacore, &asset_data, &mut r);
    r.filter_exercise_time = t.elapsed().as_secs_f64();

    // ── Phase 10: raw escape hatch ───────────────────────────────────
    let t = Instant::now();
    exercise_raw_queries(&datacore, &mut r);
    r.raw_query_time = t.elapsed().as_secs_f64();

    // ── Phase 11: snapshot round-trip ─────────────────────────────────
    exercise_snapshot(&assets, &meta, &dc_config, config, &mut r)?;

    Ok(r)
}

fn open_assets(
    config: &RunConfig,
) -> Result<(AssetSource, SnapshotMeta), Box<dyn std::error::Error>> {
    if let Some(p4k_path) = &config.p4k_path {
        let assets = AssetSource::open(p4k_path)?;
        let meta = SnapshotMeta {
            schema_version: ExtractSnapshot::SCHEMA_VERSION,
            game_version: "unknown".into(),
            build_id: "unknown".into(),
            extracted_at: sc_extract::current_timestamp(),
        };
        Ok((assets, meta))
    } else {
        let install = sc_installs::discover_primary()?;
        let assets = AssetSource::from_install(&install)?;
        let meta = sc_extract::snapshot_meta_from_install(&install);
        Ok((assets, meta))
    }
}

fn exercise_tags(snap: &sc_extract::DatacoreSnapshot, r: &mut BenchResult) {
    let tree = &snap.tag_tree;

    // Count root nodes.
    r.tag_roots = tree.roots().count();

    // Measure max depth by sampling ancestry on all nodes (the tree is
    // small enough — ~18k nodes — that this is cheap).
    let mut max_depth = 0usize;
    for node in tree.iter() {
        let depth = tree.ancestors(&node.guid).count();
        if depth > max_depth {
            max_depth = depth;
        }
    }
    r.tag_max_depth = max_depth;

    // Exercise by_name on a known tag category.
    let _manufacturer_tags = tree.by_name("Manufacturer");

    // Exercise is_descendant_of: pick the first root and its first child,
    // verify the child is a descendant.
    if let Some(root) = tree.roots().next() {
        if let Some(child_guid) = root.children.first() {
            let _is_desc = tree.is_descendant_of(child_guid, &root.guid);
        }
        // Exercise path().
        let _path = tree.path(&root.guid);
    }
}

fn exercise_manufacturers(snap: &sc_extract::DatacoreSnapshot, r: &mut BenchResult) {
    let mfrs = &snap.manufacturers;

    // Count manufacturers with a name_key.
    r.manufacturers_with_name_key = mfrs.all().filter(|m| m.name_key.is_some()).count();

    // Exercise by_code lookups on known manufacturer codes.
    let _aegs = mfrs.by_code("AEGS");
    let _gats = mfrs.by_code("GATS");
    let _behr = mfrs.by_code("BEHR");
    let _drak = mfrs.by_code("DRAK");
}

fn exercise_display_names(snap: &sc_extract::DatacoreSnapshot, r: &mut BenchResult) {
    r.display_names_non_empty = snap
        .display_names
        .iter()
        .filter(|(_, name)| !name.is_empty())
        .count();
}

fn exercise_locale(asset_data: &AssetData, r: &mut BenchResult) {
    // Try resolving a known locale key pattern.
    let _hit = asset_data.locale.resolve("@ui_MainMenu");
    let _contains = asset_data.locale.contains_key("@ui_MainMenu");

    // Count is already set from phase 2 (r.locale_entries).
    let _ = r;
}

fn exercise_graph(snap: &sc_extract::DatacoreSnapshot, r: &mut BenchResult) {
    let graph = &snap.graph;
    r.graph_sources = graph.source_count();
    r.graph_targets = graph.target_count();

    // 2-hop reachability: pick up to 10 source records, follow their
    // outgoing edges one hop, then follow those targets' outgoing edges,
    // and count unique reachable GUIDs at depth 2.
    let mut depth2: HashSet<Guid> = HashSet::new();
    let mut sampled = 0;

    // We need to iterate over sources. The graph doesn't expose a
    // source iterator directly, so sample by checking outgoing on records
    // from the record store that have outgoing edges.
    // Since we can't iterate the graph's source set directly, use an
    // approach that works: iterate display_names keys (which are record
    // GUIDs) and check for outgoing edges.
    for (guid, _) in snap.display_names.iter() {
        let outgoing = graph.outgoing(guid);
        if outgoing.is_empty() {
            continue;
        }
        for target in outgoing {
            for hop2 in graph.outgoing(target) {
                depth2.insert(*hop2);
            }
        }
        sampled += 1;
        if sampled >= 10 {
            break;
        }
    }
    r.graph_depth2_reachable = depth2.len();
}

fn exercise_filters(datacore: &Datacore, asset_data: &AssetData, r: &mut BenchResult) {
    let db = datacore.db();

    let mut total_entities = 0usize;
    let mut playable_weapons = 0usize;
    let mut playable_ships = 0usize;

    for record in db.records_by_type("EntityClassDefinition") {
        total_entities += 1;
        let inst = record.as_instance();
        let class_name = record.name().unwrap_or("");

        // Resolve display name for this entity.
        let display_name = sc_extract::resolve_entity_display_name(&inst, db, &asset_data.locale);

        // Extract AttachDef fields for weapon filter.
        let (type_name, sub_type, size) = extract_attach_def_fields(&inst, db);

        // Check if the entity has a VehicleComponentParams for ship filter.
        let has_vehicle = has_component_of_type(&inst, db, "VehicleComponentParams");

        if sc_extract::is_playable_weapon(
            &type_name,
            &sub_type,
            display_name.as_deref(),
            class_name,
            size,
        ) {
            playable_weapons += 1;
        }

        if sc_extract::is_playable_ship(class_name, display_name.as_deref(), has_vehicle) {
            playable_ships += 1;
        }
    }

    r.total_entities = total_entities;
    r.playable_weapons = playable_weapons;
    r.playable_ships = playable_ships;
}

/// Extract AttachDef.Type, AttachDef.SubType, and AttachDef.Size from an
/// EntityClassDefinition's SAttachableComponentParams component.
fn extract_attach_def_fields(
    inst: &sc_extract::Instance<'_>,
    db: &sc_extract::DataCoreDatabase,
) -> (String, String, Option<i32>) {
    let Some(components) = inst.get_array("Components") else {
        return (String::new(), String::new(), None);
    };

    for value in components {
        let Some(component) = value_to_instance(&value, db) else {
            continue;
        };
        if component.type_name() != Some("SAttachableComponentParams") {
            continue;
        }
        let Some(attach_def) = component.get_instance("AttachDef") else {
            continue;
        };
        let type_name = attach_def.get_str("Type").unwrap_or("").to_string();
        let sub_type = attach_def.get_str("SubType").unwrap_or("").to_string();
        let size = attach_def.get_i32("Size");
        return (type_name, sub_type, size);
    }

    (String::new(), String::new(), None)
}

/// Check if an EntityClassDefinition has a component of the given type name.
fn has_component_of_type(
    inst: &sc_extract::Instance<'_>,
    db: &sc_extract::DataCoreDatabase,
    component_type: &str,
) -> bool {
    let Some(components) = inst.get_array("Components") else {
        return false;
    };
    for value in components {
        let Some(component) = value_to_instance(&value, db) else {
            continue;
        };
        if component.type_name() == Some(component_type) {
            return true;
        }
    }
    false
}

/// Convert a svarog Value to an Instance (same helper as display_names.rs).
fn value_to_instance<'a>(
    value: &sc_extract::svarog_datacore::Value<'a>,
    db: &'a sc_extract::DataCoreDatabase,
) -> Option<sc_extract::Instance<'a>> {
    use sc_extract::svarog_datacore::Value;
    match value {
        Value::Class { struct_index, data } => Some(sc_extract::Instance::from_inline_data(
            db,
            *struct_index,
            data,
        )),
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}

fn exercise_raw_queries(datacore: &Datacore, r: &mut BenchResult) {
    let db = datacore.db();
    r.raw_entity_count = db.records_by_type("EntityClassDefinition").count();
    r.raw_manufacturer_count = db.records_by_type("SCItemManufacturer").count();
    r.raw_tag_count = db.records_by_type("Tag").count();
    r.raw_ammo_count = db.records_by_type("AmmoParams").count();
}

fn exercise_snapshot(
    assets: &AssetSource,
    meta: &SnapshotMeta,
    dc_config: &DatacoreConfig,
    config: &RunConfig,
    r: &mut BenchResult,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut capture_config = SnapshotCaptureConfig::standard();
    if config.skip_assets {
        capture_config.archive_locale = false;
    }

    let t = Instant::now();
    let snapshot = ExtractSnapshot::capture(assets, meta.clone(), &capture_config)?;
    r.snapshot_capture_time = t.elapsed().as_secs_f64();

    let snapshot_path = std::env::temp_dir().join("sc_bench_smoke.snap");

    let t = Instant::now();
    snapshot.save(&snapshot_path)?;
    r.snapshot_save_time = t.elapsed().as_secs_f64();

    let size = std::fs::metadata(&snapshot_path)?.len();
    r.snapshot_size_mb = size as f64 / 1_000_000.0;

    let t = Instant::now();
    let loaded = ExtractSnapshot::load(&snapshot_path)?;
    r.snapshot_load_time = t.elapsed().as_secs_f64();

    let hydrate_asset_config = if config.skip_assets {
        AssetConfig::minimal()
    } else {
        AssetConfig::standard()
    };

    let t = Instant::now();
    let (_hydrated_assets, hydrated_dc) = loaded.hydrate(&hydrate_asset_config, dc_config)?;
    r.snapshot_hydrate_time = t.elapsed().as_secs_f64();
    r.snapshot_hydrated_records = hydrated_dc.snapshot().records.len();

    // Cleanup — best-effort.
    let _ = std::fs::remove_file(&snapshot_path);

    Ok(())
}
