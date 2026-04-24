//! Orchestration — opens the P4K, extracts and parses the DCB, and drives
//! the `emit` module to write generated Rust files.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use svarog_datacore::DataCoreDatabase;
use svarog_p4k::P4kArchive;

use crate::emit;
use crate::features;

/// Command-line options for a single generator run.
#[derive(Debug, Clone)]
pub struct RunOptions {
    pub p4k: PathBuf,
    pub out_dir: PathBuf,
    pub check_only: bool,
    pub dump_refs: bool,
}

/// Summary of a completed generator run. Printed on success.
#[derive(Debug, Clone)]
pub struct Summary {
    pub struct_count: usize,
    pub enum_count: usize,
    pub out_dir: PathBuf,
}

/// Errors produced during a generator run.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to open P4K archive at {path}: {source}")]
    P4kOpen {
        path: PathBuf,
        #[source]
        source: svarog_p4k::Error,
    },

    #[error("Game2.dcb not found inside P4K archive")]
    DcbNotFound,

    #[error("failed to read Game2.dcb from P4K: {0}")]
    DcbRead(#[source] svarog_p4k::Error),

    #[error("failed to parse Game2.dcb: {0}")]
    DcbParse(#[source] svarog_datacore::Error),

    #[error("failed to write generated file {path}: {source}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to create output directory {path}: {source}")]
    CreateDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// Run the generator end-to-end: open P4K, extract DCB, emit files.
pub fn run(options: &RunOptions) -> Result<Summary> {
    let start = Instant::now();

    tracing::info!(path = %options.p4k.display(), "opening P4K archive");
    let archive = P4kArchive::open(&options.p4k).map_err(|source| Error::P4kOpen {
        path: options.p4k.clone(),
        source,
    })?;

    tracing::info!("locating Game2.dcb inside archive");
    let dcb_bytes = extract_dcb(&archive)?;
    tracing::info!(bytes = dcb_bytes.len(), "extracted Game2.dcb");

    tracing::info!("parsing DataCore");
    let db = DataCoreDatabase::parse(&dcb_bytes).map_err(Error::DcbParse)?;

    let struct_count = db.struct_definitions().len();
    let enum_count = db.enum_definitions().len();
    tracing::info!(struct_count, enum_count, "DataCore parsed");

    // Read version metadata from the install directory next to Data.p4k.
    let metadata = load_metadata(&options.p4k, struct_count, enum_count);

    if options.check_only {
        tracing::info!("--check mode: skipping file writes");
        return Ok(Summary {
            struct_count,
            enum_count,
            out_dir: options.out_dir.clone(),
        });
    }

    // Ensure output directory exists.
    std::fs::create_dir_all(&options.out_dir).map_err(|source| Error::CreateDir {
        path: options.out_dir.clone(),
        source,
    })?;

    // Clean the output directory completely — feature-scoped layout replaces
    // the old alphabetical bucket layout.
    if options.out_dir.exists() {
        std::fs::remove_dir_all(&options.out_dir).map_err(|source| Error::CreateDir {
            path: options.out_dir.clone(),
            source,
        })?;
    }
    std::fs::create_dir_all(&options.out_dir).map_err(|source| Error::CreateDir {
        path: options.out_dir.clone(),
        source,
    })?;

    // Compute shared tables.
    //
    // The inherited-property cache is built once up front and passed to
    // every later phase that needs to read fields. Reachability BFS,
    // polymorphic base detection, feature classification, closure walks,
    // and per-struct emit all read from this single table instead of
    // re-walking parent chains each time.
    tracing::info!("building inherited-property cache");
    let property_cache = emit::build_property_cache(&db);

    let descendants = emit::compute_descendants(&db);
    // Reachability is a pipeline-level fact — computed once here and
    // shared by `compute_emitted_struct_names` (via `reachable`) and the
    // feature classifier (via `reachable_indices` derived from
    // `emitted_names`). Previously this BFS was hidden inside
    // `compute_emitted_struct_names`.
    let reachable_structs =
        emit::compute_reachable_struct_indices(&db, &descendants, &property_cache);
    let emitted_names = emit::compute_emitted_struct_names(&db, &reachable_structs);
    let pool_fields = emit::compute_pool_field_names(&emitted_names);
    let poly_bases = emit::compute_poly_bases(&emitted_names, &descendants, &property_cache);
    tracing::info!(
        poly_bases = poly_bases.len(),
        "computed polymorphic base set"
    );

    // Role-based promotion: polymorphic bases whose *full* inherited
    // property set is empty are compiled unconditionally in `core`.
    // Using the full inheritance-resolved field count (not just `own`)
    // guarantees the emitted struct body is truly empty and carries no
    // transitive references to other types, which is what makes
    // unconditional compilation safe (no cascade). In practice most of
    // the 336 empty-own bases also have empty-full, so the resulting set
    // should be within ~5% of 336. See `docs/feature-gating-v2.md`
    // Decision 4.
    let promoted: std::collections::HashSet<u32> = poly_bases
        .iter()
        .copied()
        .filter(|&idx| property_cache[idx as usize].is_empty())
        .collect();
    tracing::info!(
        promoted = promoted.len(),
        "computed role-based unconditional promotion set (empty poly bases, full inheritance)"
    );

    // Classify types into features.
    tracing::info!("classifying types into features");
    let reachable: std::collections::HashSet<usize> =
        emitted_names.keys().map(|&idx| idx as usize).collect();
    tracing::info!("building GUID → instance lookup for Reference resolution");
    let guid_lookup = crate::closure::build_guid_lookup(&db);
    let feature_rules = features::FeatureRules::default();
    let mut dangling_refs: std::collections::HashSet<svarog_common::CigGuid> =
        std::collections::HashSet::new();
    let feature_map = features::classify_features(
        &db,
        &reachable,
        &guid_lookup,
        &promoted,
        &feature_rules,
        &property_cache,
        &mut dangling_refs,
    );
    if !dangling_refs.is_empty() {
        tracing::warn!(
            dangling_ref_count = dangling_refs.len(),
            "some Reference fields pointed at GUIDs not present in the record map; \
             see earlier per-GUID warnings. svarog's own export walker also skips \
             these, so generation continues."
        );
    }
    let feature_assignments = emit::compute_feature_assignment_map(&emitted_names, &feature_map);
    let feature_cfgs = emit::compute_feature_cfg_map(&emitted_names, &feature_map);
    let multi_count = feature_cfgs.values().filter(|c| c.is_some()).count();
    tracing::info!(
        multi_assigned_types = multi_count,
        "computed per-type cfg map (Multi assignments)"
    );

    tracing::info!(
        features = feature_map.feature_names.len(),
        parent_features = feature_map.parent_features.len(),
        "feature classification complete"
    );

    // Build the list of all feature module names. Three synthetic
    // modules ride alongside the real feature names:
    //
    // - `core` — unconditional, role-promoted types only
    // - `multi-feature` — types shared across multiple real features;
    //   module is unconditional but each struct is individually gated
    //   on `#[cfg(any(feature = "f1", feature = "f2", ...))]`
    // - `dormant` — schema-reachable types never observed in record
    //   walks; whole module is gated on `#[cfg(feature = "dormant")]`
    //
    // The pipeline pre-reserves directory slots for all three; the
    // emission loop below skips any slot that ends up with zero types.
    let mut all_features: Vec<String> = vec![
        "core".to_string(),
        emit::MULTI_FEATURE_MODULE.to_string(),
        emit::DORMANT_MODULE.to_string(),
    ];
    for f in &feature_map.feature_names {
        if !all_features.contains(f) {
            all_features.push(f.clone());
        }
    }

    // Group struct indices by feature.
    let mut by_feature: std::collections::BTreeMap<String, Vec<u32>> =
        std::collections::BTreeMap::new();
    for (&struct_idx, feature) in &feature_assignments {
        by_feature
            .entry(feature.clone())
            .or_default()
            .push(struct_idx);
    }
    // Sort indices within each feature for deterministic output.
    for indices in by_feature.values_mut() {
        indices.sort_unstable();
    }

    // Discover record types.
    let mut record_struct_indices: std::collections::HashSet<u32> =
        std::collections::HashSet::new();
    for record in db.records() {
        if record.struct_index >= 0 && emitted_names.contains_key(&(record.struct_index as u32)) {
            record_struct_indices.insert(record.struct_index as u32);
        }
    }

    // Emit per-feature directories.
    //
    // `features_emitted` is the subset of `all_features` that actually
    // produced a module. We pass that list to the top-level composition
    // functions (`emit_top_data_pools`, `emit_top_mod`, etc.) so they
    // don't reference directories we didn't write.
    let mut features_with_index: Vec<String> = Vec::new();
    let mut features_emitted: Vec<String> = Vec::new();

    for feature in &all_features {
        let indices = by_feature.get(feature).cloned().unwrap_or_default();

        // Skip feature dirs that end up with zero types. `core` is the
        // only directory we always emit: it's referenced by the top-level
        // mod.rs doc comment and must exist even when pruning leaves it
        // empty (which in practice never happens — there's always at
        // least one role-promoted type).
        if indices.is_empty() && feature != "core" {
            continue;
        }

        let mod_name = feature.replace('-', "_");
        let feature_dir = options.out_dir.join(&mod_name);
        std::fs::create_dir_all(&feature_dir).map_err(|source| Error::CreateDir {
            path: feature_dir.clone(),
            source,
        })?;

        features_emitted.push(feature.clone());

        tracing::info!(feature = %feature, types = indices.len(), "emitting feature");

        // types.rs
        let types_src = emit::emit_feature_types(
            &db,
            feature,
            &indices,
            &emitted_names,
            &pool_fields,
            &poly_bases,
            &feature_cfgs,
            &property_cache,
        );
        write_file(&feature_dir.join("types.rs"), &types_src)?;

        // pools.rs
        let pools_src = emit::emit_feature_pools(
            feature,
            &indices,
            &emitted_names,
            &pool_fields,
            &feature_cfgs,
        );
        write_file(&feature_dir.join("pools.rs"), &pools_src)?;

        // index.rs (only if this feature has record types)
        let feature_record_indices: Vec<u32> = indices
            .iter()
            .filter(|idx| record_struct_indices.contains(idx))
            .copied()
            .collect();
        let has_index = !feature_record_indices.is_empty();
        if has_index {
            if let Some(index_src) = emit::emit_feature_index(
                &db,
                feature,
                &feature_record_indices,
                &emitted_names,
                &feature_cfgs,
            ) {
                write_file(&feature_dir.join("index.rs"), &index_src)?;
            }
            features_with_index.push(feature.clone());
        }

        // mod.rs
        let mod_src = emit::emit_feature_mod(feature, has_index);
        write_file(&feature_dir.join("mod.rs"), &mod_src)?;
    }

    // Top-level files.
    tracing::info!("emitting enums.rs");
    write_file(&options.out_dir.join("enums.rs"), &emit::emit_enums(&db))?;

    tracing::info!("emitting poly_enums.rs");
    write_file(
        &options.out_dir.join("poly_enums.rs"),
        &emit::emit_poly_enums_file(
            &db,
            &poly_bases,
            &emitted_names,
            &descendants,
            &feature_assignments,
            &feature_cfgs,
        ),
    )?;

    tracing::info!("emitting data_pools.rs");
    write_file(
        &options.out_dir.join("data_pools.rs"),
        &emit::emit_top_data_pools(&features_emitted),
    )?;

    tracing::info!("emitting record_index.rs");
    write_file(
        &options.out_dir.join("record_index.rs"),
        &emit::emit_top_record_index(&features_with_index),
    )?;

    tracing::info!("emitting record_store.rs");
    write_file(
        &options.out_dir.join("record_store.rs"),
        &emit::emit_record_store(
            &db,
            &emitted_names,
            &pool_fields,
            &feature_assignments,
            &feature_cfgs,
        ),
    )?;

    tracing::info!("emitting metadata.rs");
    write_file(
        &options.out_dir.join("metadata.rs"),
        &emit::emit_metadata(&metadata),
    )?;

    tracing::info!("emitting mod.rs");
    write_file(
        &options.out_dir.join("mod.rs"),
        &emit::emit_top_mod(&features_emitted, &feature_map),
    )?;

    // Update Cargo.toml [features] sections.
    // 1. sc-extract-generated/Cargo.toml — leaf features
    // 2. sc-extract/Cargo.toml — forwarding features
    tracing::info!("updating Cargo.toml features");
    let generated_crate_dir = options
        .out_dir
        .parent() // src/
        .and_then(|p| p.parent()); // sc-extract-generated/

    // Cargo features list excludes the synthetic `multi-feature` module
    // (it's always compiled, not a real feature) but includes `dormant`
    // (a real opt-in feature gating the `dormant` module). `core` is
    // also stripped by `update_cargo_features` itself.
    let cargo_features: Vec<String> = all_features
        .iter()
        .filter(|f| f.as_str() != emit::MULTI_FEATURE_MODULE)
        .cloned()
        .collect();

    if let Some(gen_dir) = generated_crate_dir {
        update_cargo_features(&gen_dir.join("Cargo.toml"), &cargo_features, &feature_map)?;

        // sc-extract lives alongside sc-extract-generated under crates/.
        let extract_cargo = gen_dir
            .parent() // crates/
            .map(|p| p.join("sc-extract").join("Cargo.toml"));
        if let Some(extract_path) = extract_cargo
            && extract_path.exists()
        {
            update_sc_extract_features(&extract_path, &cargo_features, &feature_map)?;
        }
    }

    tracing::info!(elapsed_ms = start.elapsed().as_millis(), "write complete");

    Ok(Summary {
        struct_count,
        enum_count,
        out_dir: options.out_dir.clone(),
    })
}

/// Metadata baked into the generated `metadata.rs` file.
#[derive(Debug, Clone)]
pub struct Metadata {
    pub game_version: String,
    pub game_branch: String,
    pub build_id: String,
    pub changelist: Option<String>,
    pub generated_at: String,
    pub generator_version: String,
    pub schema_version: u32,
    pub struct_count: usize,
    pub enum_count: usize,
}

/// Read `build_manifest.id` from next to `Data.p4k` if possible.
fn load_metadata(p4k: &Path, struct_count: usize, enum_count: usize) -> Metadata {
    let install = p4k.parent().unwrap_or(Path::new("."));
    let manifest = sc_installs::read_build_manifest(install).ok();

    Metadata {
        game_version: manifest
            .as_ref()
            .map(|m| m.version.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        game_branch: manifest
            .as_ref()
            .map(|m| m.branch.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        build_id: manifest
            .as_ref()
            .map(|m| m.build_id.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        changelist: manifest.as_ref().and_then(|m| m.changelist.clone()),
        generated_at: current_timestamp(),
        generator_version: env!("CARGO_PKG_VERSION").to_string(),
        schema_version: 1,
        struct_count,
        enum_count,
    }
}

/// Locate Game2.dcb in the P4K archive and return its raw bytes.
fn extract_dcb(archive: &P4kArchive) -> Result<Vec<u8>> {
    let entry = archive
        .iter()
        .find(|e| e.name.to_ascii_lowercase().ends_with("game2.dcb"))
        .ok_or(Error::DcbNotFound)?;

    archive.read(&entry).map_err(Error::DcbRead)
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    std::fs::write(path, contents).map_err(|source| Error::WriteFile {
        path: path.to_path_buf(),
        source,
    })
}

/// Update the [features] section in Cargo.toml.
///
/// Reads the existing Cargo.toml, strips any existing [features] section
/// (and everything after it), then appends the generated features.
fn update_cargo_features(
    cargo_path: &Path,
    all_features: &[String],
    feature_map: &crate::features::FeatureMap,
) -> Result<()> {
    let content = std::fs::read_to_string(cargo_path).map_err(|source| Error::WriteFile {
        path: cargo_path.to_path_buf(),
        source,
    })?;

    // Strip existing [features] section if present.
    let base = if let Some(pos) = content.find("\n[features]") {
        content[..pos + 1].to_string()
    } else {
        let mut s = content.clone();
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s
    };

    let mut out = base;
    out.push_str("[features]\n");
    out.push_str("default = []\n");

    // `full` enables all leaf features.
    let leaf_features: Vec<&String> = all_features.iter().filter(|f| *f != "core").collect();
    let mut written: std::collections::HashSet<String> = std::collections::HashSet::new();
    written.insert("default".to_string());
    written.insert("full".to_string());
    written.insert("dormant".to_string());
    out.push_str("full = [\n");
    for f in &leaf_features {
        let _ = writeln!(out, "    \"{f}\",");
    }
    out.push_str("]\n");

    // `dormant` forwards to `full` so every observed cross-reference
    // is in scope when dormant types compile. See
    // `docs/feature-gating-v2.md` Decision 5 for rationale.
    out.push_str("dormant = [\"full\"]\n");

    // Parent features (aliases). Skip any that collide with a leaf name
    // — the leaf definition below is authoritative, and Cargo rejects
    // duplicate keys in [features].
    for (parent, children) in &feature_map.parent_features {
        if leaf_features.contains(&parent) {
            continue;
        }
        if !written.insert(parent.clone()) {
            continue;
        }
        let _ = write!(out, "{parent} = [");
        let valid_children: Vec<&String> = children
            .iter()
            .filter(|c| leaf_features.contains(c) || feature_map.parent_features.contains_key(*c))
            .collect();
        for (i, child) in valid_children.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            let _ = write!(out, "\"{child}\"");
        }
        out.push_str("]\n");
    }

    // Individual leaf features (empty deps — they just gate cfg).
    out.push('\n');
    out.push_str("# Auto-generated leaf features. Each gates a set of DCB types.\n");
    for f in &leaf_features {
        if !written.insert((*f).clone()) {
            continue;
        }
        let _ = writeln!(out, "{f} = []");
    }

    write_file(cargo_path, &out)
}

/// Update sc-extract's Cargo.toml to forward features to sc-extract-generated.
fn update_sc_extract_features(
    cargo_path: &Path,
    all_features: &[String],
    feature_map: &crate::features::FeatureMap,
) -> Result<()> {
    let content = std::fs::read_to_string(cargo_path).map_err(|source| Error::WriteFile {
        path: cargo_path.to_path_buf(),
        source,
    })?;

    // Strip existing [features] section.
    let base = if let Some(pos) = content.find("\n[features]") {
        content[..pos + 1].to_string()
    } else {
        let mut s = content.clone();
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s
    };

    let mut out = base;
    out.push_str("[features]\n");
    out.push_str("default = []\n");

    // `full` forwards to sc-extract-generated/full
    out.push_str("full = [\"sc-extract-generated/full\"]\n");
    // `dormant` forwards to sc-extract-generated/dormant (which forwards to full).
    out.push_str("dormant = [\"sc-extract-generated/dormant\"]\n");

    let mut written: std::collections::HashSet<String> = std::collections::HashSet::new();
    written.insert("default".to_string());
    written.insert("full".to_string());
    written.insert("dormant".to_string());

    // Parent features forward to their children in sc-extract-generated.
    // Skip names that collide with a leaf so the leaf forward wins.
    for (parent, children) in &feature_map.parent_features {
        if all_features.iter().any(|f| f == parent) {
            continue;
        }
        if !written.insert(parent.clone()) {
            continue;
        }
        let _ = write!(out, "{parent} = [");
        let valid: Vec<&String> = children
            .iter()
            .filter(|c| all_features.contains(c))
            .collect();
        for (i, child) in valid.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            let _ = write!(out, "\"sc-extract-generated/{child}\"");
        }
        out.push_str("]\n");
    }

    // Leaf features forward 1:1.
    out.push('\n');
    out.push_str("# Auto-generated: each feature forwards to sc-extract-generated.\n");
    for f in all_features {
        if f == "core" {
            continue;
        }
        if !written.insert(f.clone()) {
            continue;
        }
        let _ = writeln!(out, "{f} = [\"sc-extract-generated/{f}\"]");
    }

    write_file(cargo_path, &out)
}

/// Diagnostic: iterate the raw Reference value array and report how
/// many entries resolve to a known record, how many are "null" per
/// svarog's definition (all-zero GUID), and how many are neither —
/// grouped by `instance_index` to see if there is a sentinel pattern
/// svarog's `is_null()` check is missing.
///
/// Motivation: svarog's `DataCoreReference::is_null()` only checks
/// `record_id.is_empty()` (all 16 bytes zero). If the DCB actually
/// marks null references with `instance_index == -1` and leaves the
/// GUID slot uninitialized (or populated with stale bytes), svarog
/// will parse them as non-null and we will see them as "dangling"
/// references. This diagnostic confirms or refutes that theory.
pub fn run_dump_refs(options: &RunOptions) -> Result<()> {
    let archive = P4kArchive::open(&options.p4k).map_err(|source| Error::P4kOpen {
        path: options.p4k.clone(),
        source,
    })?;
    let dcb_bytes = extract_dcb(&archive)?;
    let db = DataCoreDatabase::parse(&dcb_bytes).map_err(Error::DcbParse)?;

    let counts = db.pool_counts();
    let total = counts.reference_count;

    // Build the same record map the walker uses so we can ask
    // "would this ref resolve?" for every entry in the value array.
    let mut record_guids: std::collections::HashSet<svarog_common::CigGuid> =
        std::collections::HashSet::with_capacity(db.all_records().count());
    for record in db.all_records() {
        record_guids.insert(record.id());
    }

    // Histogram: map (instance_index, guid_class) → count
    // where guid_class ∈ { Zero, InMap, NotInMap }.
    let mut by_idx_zero: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
    let mut by_idx_inmap: std::collections::BTreeMap<i32, usize> =
        std::collections::BTreeMap::new();
    let mut by_idx_dangling: std::collections::BTreeMap<i32, usize> =
        std::collections::BTreeMap::new();

    for i in 0..total {
        let Some(entry) = db.reference_value(i) else {
            continue;
        };
        let instance_index = entry.instance_index;
        let guid = entry.record_id;
        if guid.is_empty() {
            *by_idx_zero.entry(instance_index).or_insert(0) += 1;
        } else if record_guids.contains(&guid) {
            *by_idx_inmap.entry(instance_index).or_insert(0) += 1;
        } else {
            *by_idx_dangling.entry(instance_index).or_insert(0) += 1;
        }
    }

    // Also count unique dangling GUIDs so we can compare against the
    // walker's per-run dedup count.
    let mut dangling_guids_unique: std::collections::HashSet<svarog_common::CigGuid> =
        std::collections::HashSet::new();
    let mut inmap_unique: std::collections::HashSet<svarog_common::CigGuid> =
        std::collections::HashSet::new();
    for i in 0..total {
        let Some(entry) = db.reference_value(i) else {
            continue;
        };
        if entry.record_id.is_empty() {
            continue;
        }
        if record_guids.contains(&entry.record_id) {
            inmap_unique.insert(entry.record_id);
        } else {
            dangling_guids_unique.insert(entry.record_id);
        }
    }

    let zero_total: usize = by_idx_zero.values().sum();
    let inmap_total: usize = by_idx_inmap.values().sum();
    let dangling_total: usize = by_idx_dangling.values().sum();

    println!("Reference value array histogram");
    println!("  total entries:                 {total}");
    println!("  records in map:                {}", record_guids.len());
    println!();
    println!("  zero GUID (svarog 'null'):     {zero_total}");
    println!("  non-zero GUID, resolves:       {inmap_total}");
    println!(
        "  non-zero GUID, DANGLING:       {dangling_total}  (unique guids: {})",
        dangling_guids_unique.len()
    );
    println!("  non-zero GUID, in-map unique:  {}", inmap_unique.len());
    println!();

    println!("Breakdown by instance_index:");
    println!("  (only indices with non-zero counts shown)");
    let mut all_idx: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();
    all_idx.extend(by_idx_zero.keys().copied());
    all_idx.extend(by_idx_inmap.keys().copied());
    all_idx.extend(by_idx_dangling.keys().copied());
    println!();
    println!("    idx │   zero   │  in-map  │ dangling │");
    println!("  ──────┼──────────┼──────────┼──────────┤");
    for idx in &all_idx {
        let z = by_idx_zero.get(idx).copied().unwrap_or(0);
        let m = by_idx_inmap.get(idx).copied().unwrap_or(0);
        let d = by_idx_dangling.get(idx).copied().unwrap_or(0);
        println!("  {idx:>5} │ {z:>8} │ {m:>8} │ {d:>8} │");
    }

    // Sample some dangling refs and their raw bytes so we can eyeball them.
    println!();
    println!("First 10 dangling entries (instance_index, guid):");
    let mut shown = 0usize;
    for i in 0..total {
        if shown >= 10 {
            break;
        }
        let Some(entry) = db.reference_value(i) else {
            continue;
        };
        if entry.record_id.is_empty() {
            continue;
        }
        if record_guids.contains(&entry.record_id) {
            continue;
        }
        let idx = entry.instance_index;
        let guid = entry.record_id;
        println!("  [{i:>6}] instance_index={idx:>11}  guid={guid:?}");
        shown += 1;
    }

    Ok(())
}

fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{now}")
}
