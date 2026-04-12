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
    /// Dump record path prefixes grouped by struct type, then exit.
    pub dump_paths: bool,
    /// Dump computed feature assignments and exit.
    pub dump_features: bool,
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

    if options.dump_paths {
        dump_record_paths(&db);
        return Ok(Summary {
            struct_count,
            enum_count,
            out_dir: options.out_dir.clone(),
        });
    }

    if options.dump_features {
        dump_feature_assignments(&db);
        return Ok(Summary {
            struct_count,
            enum_count,
            out_dir: options.out_dir.clone(),
        });
    }

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
    let emitted_names = emit::compute_emitted_struct_names(&db);
    let pool_fields = emit::compute_pool_field_names(&emitted_names);

    // Classify types into features.
    tracing::info!("classifying types into features");
    let reachable: std::collections::HashSet<usize> = emitted_names
        .keys()
        .map(|&idx| idx as usize)
        .collect();
    let feature_rules = features::FeatureRules::default();
    let feature_map = features::classify_features(&db, &reachable, &feature_rules);
    let feature_assignments = emit::compute_feature_assignment_map(&emitted_names, &feature_map);

    tracing::info!(
        features = feature_map.feature_names.len(),
        parent_features = feature_map.parent_features.len(),
        "feature classification complete"
    );

    // Build the list of all feature module names (core + leaf features).
    let mut all_features: Vec<String> = vec!["core".to_string()];
    for f in &feature_map.feature_names {
        if f != "core" && !all_features.contains(f) {
            all_features.push(f.clone());
        }
    }

    // Group struct indices by feature.
    let mut by_feature: std::collections::BTreeMap<String, Vec<u32>> = std::collections::BTreeMap::new();
    for (&struct_idx, feature) in &feature_assignments {
        by_feature.entry(feature.clone()).or_default().push(struct_idx);
    }
    // Sort indices within each feature for deterministic output.
    for indices in by_feature.values_mut() {
        indices.sort_unstable();
    }

    // Discover record types.
    let mut record_struct_indices: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for record in db.records() {
        if record.struct_index >= 0 && emitted_names.contains_key(&(record.struct_index as u32)) {
            record_struct_indices.insert(record.struct_index as u32);
        }
    }

    // Emit per-feature directories.
    let mut features_with_index: Vec<String> = Vec::new();

    for feature in &all_features {
        let mod_name = feature.replace('-', "_");
        let feature_dir = options.out_dir.join(&mod_name);
        std::fs::create_dir_all(&feature_dir).map_err(|source| Error::CreateDir {
            path: feature_dir.clone(),
            source,
        })?;

        let indices = by_feature.get(feature).cloned().unwrap_or_default();

        tracing::info!(feature = %feature, types = indices.len(), "emitting feature");

        // types.rs
        let types_src = emit::emit_feature_types(&db, feature, &indices, &emitted_names, &pool_fields);
        write_file(&feature_dir.join("types.rs"), &types_src)?;

        // pools.rs
        let pools_src = emit::emit_feature_pools(feature, &indices, &emitted_names, &pool_fields);
        write_file(&feature_dir.join("pools.rs"), &pools_src)?;

        // index.rs (only if this feature has record types)
        let feature_record_indices: Vec<u32> = indices
            .iter()
            .filter(|idx| record_struct_indices.contains(idx))
            .copied()
            .collect();
        let has_index = !feature_record_indices.is_empty();
        if has_index {
            if let Some(index_src) = emit::emit_feature_index(&db, feature, &feature_record_indices, &emitted_names) {
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

    tracing::info!("emitting data_pools.rs");
    write_file(&options.out_dir.join("data_pools.rs"), &emit::emit_top_data_pools(&all_features))?;

    tracing::info!("emitting record_index.rs");
    write_file(&options.out_dir.join("record_index.rs"), &emit::emit_top_record_index(&features_with_index))?;

    tracing::info!("emitting record_store.rs");
    write_file(&options.out_dir.join("record_store.rs"), &emit::emit_record_store(&db, &emitted_names, &pool_fields, &feature_assignments))?;

    tracing::info!("emitting metadata.rs");
    write_file(&options.out_dir.join("metadata.rs"), &emit::emit_metadata(&metadata))?;

    tracing::info!("emitting mod.rs");
    write_file(&options.out_dir.join("mod.rs"), &emit::emit_top_mod(&all_features, &feature_map))?;

    // Update Cargo.toml [features] sections.
    // 1. sc-extract-generated/Cargo.toml — leaf features
    // 2. sc-extract/Cargo.toml — forwarding features
    tracing::info!("updating Cargo.toml features");
    let generated_crate_dir = options
        .out_dir
        .parent() // src/
        .and_then(|p| p.parent()); // sc-extract-generated/

    if let Some(gen_dir) = generated_crate_dir {
        update_cargo_features(&gen_dir.join("Cargo.toml"), &all_features, &feature_map)?;

        // sc-extract lives alongside sc-extract-generated under crates/.
        let extract_cargo = gen_dir.parent() // crates/
            .map(|p| p.join("sc-extract").join("Cargo.toml"));
        if let Some(extract_path) = extract_cargo {
            if extract_path.exists() {
                update_sc_extract_features(&extract_path, &all_features, &feature_map)?;
            }
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

/// Remove any generator-owned files from the target directory so a fresh
/// generation doesn't leave orphan files around.
///
/// Only removes files matching our known output names:
/// - `types.rs` (legacy single-file layout)
/// - `types_*.rs` (any bucket naming)
/// - `enums.rs`
/// - `metadata.rs`
/// - `mod.rs`
///
/// Any other files in the directory are left alone.
fn clean_generated_dir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("could not enumerate {}: {e}", dir.display());
            return Ok(());
        }
    };

    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_file() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        let owned = name == "types.rs"
            || name == "enums.rs"
            || name == "metadata.rs"
            || name == "mod.rs"
            || name == "record_store.rs"
            || name == "data_pools.rs"
            || name == "record_index.rs"
            || (name.starts_with("types_") && name.ends_with(".rs"));

        if owned {
            let path = entry.path();
            if let Err(e) = std::fs::remove_file(&path) {
                tracing::warn!("failed to remove stale file {}: {e}", path.display());
            }
        }
    }
    Ok(())
}

/// Dump computed feature assignments — shows which types go into which feature.
fn dump_feature_assignments(db: &DataCoreDatabase) {
    use crate::emit;
    use crate::features::{self, FeatureAssignment, FeatureRules};

    let emitted_names = emit::compute_emitted_struct_names(db);
    let reachable: std::collections::HashSet<usize> = emitted_names
        .keys()
        .map(|&idx| idx as usize)
        .collect();

    let rules = FeatureRules::default();
    let feature_map = features::classify_features(db, &reachable, &rules);

    // Count types per category.
    let mut core_count = 0usize;
    let mut multi_count = 0usize;
    let mut single_count = 0usize;
    for (&struct_idx, _) in &emitted_names {
        match feature_map.struct_feature.get(struct_idx as usize).and_then(|a| a.as_ref()) {
            Some(FeatureAssignment::Core) => core_count += 1,
            Some(FeatureAssignment::Single(_)) => single_count += 1,
            Some(FeatureAssignment::Multi(_)) => multi_count += 1,
            None => {}
        }
    }

    println!("Feature classification results");
    println!("==============================");
    println!();
    println!("Rules: max_fields={}, min_fields={}, max_depth={}, core_threshold={}",
        rules.max_fields_per_feature, rules.min_fields_per_feature,
        rules.max_depth, rules.core_promotion_threshold);
    println!();
    println!("Total emitted types: {}", emitted_names.len());
    println!("Total compile cost: {} fields", feature_map.total_cost);
    println!();
    println!("  Core (always compiled): {core_count} types, {} fields",
        feature_map.feature_costs.get("core").copied().unwrap_or(0));
    println!("  Multi-feature (cfg any): {multi_count} types");
    println!("  Single-feature: {single_count} types");
    println!();

    // Leaf features sorted alphabetically.
    println!("Leaf features ({} total):", feature_map.feature_names.len());
    println!();
    println!("{:<55} {:>6} {:>8}", "feature", "types", "cost");
    println!("{:-<55} {:->6} {:->8}", "", "", "");

    for feature_name in &feature_map.feature_names {
        let cost = feature_map.feature_costs.get(feature_name).copied().unwrap_or(0);
        // Count exclusive types for this feature.
        let excl = feature_map.struct_feature.iter().flatten().filter(|a| {
            matches!(a, FeatureAssignment::Single(f) if f == feature_name)
        }).count();
        println!("{feature_name:<55} {excl:>6} {cost:>8}");
    }

    // Parent features.
    if !feature_map.parent_features.is_empty() {
        println!();
        println!("Parent features ({}):", feature_map.parent_features.len());
        println!();
        for (parent, children) in &feature_map.parent_features {
            println!("  {parent}");
            for child in children {
                println!("    - {child}");
            }
        }
    }
}

/// Dump record paths with adaptive depth analysis.
///
/// Shows how records cluster at various depths, and proposes auto-feature
/// boundaries based on configurable thresholds.
fn dump_record_paths(db: &DataCoreDatabase) {
    use std::collections::{BTreeMap, BTreeSet};

    // Thresholds for auto-feature boundary decisions
    const MIN_RECORDS_FOR_FEATURE: usize = 10;
    const MAX_RECORDS_BEFORE_SPLIT: usize = 2000;
    const MAX_DEPTH: usize = 8;

    /// Collect records grouped by prefix at a given depth.
    fn collect_at_depth(
        db: &DataCoreDatabase,
        depth: usize,
    ) -> BTreeMap<String, (usize, BTreeSet<String>)> {
        let mut by_prefix: BTreeMap<String, (usize, BTreeSet<String>)> = BTreeMap::new();
        for record in db.all_records() {
            let Some(file_name) = record.file_name() else { continue };
            let prefix: String = file_name
                .split('/')
                .take(depth)
                .collect::<Vec<_>>()
                .join("/");
            let type_name = db
                .struct_name(record.struct_index() as usize)
                .unwrap_or("<unknown>")
                .to_string();
            let entry = by_prefix.entry(prefix).or_insert_with(|| (0, BTreeSet::new()));
            entry.0 += 1;
            entry.1.insert(type_name);
        }
        by_prefix
    }

    // ── Depth 4 overview ──────────────────────────────────────────────
    let d4 = collect_at_depth(db, 4);

    println!("Record path prefixes (depth 4)");
    println!("==============================");
    println!();
    println!("{:<60} {:>8} {:>6}", "prefix", "records", "types");
    println!("{:-<60} {:->8} {:->6}", "", "", "");

    for (prefix, (count, types)) in &d4 {
        let marker = if *count > MAX_RECORDS_BEFORE_SPLIT {
            " [SPLIT]"
        } else if *count < MIN_RECORDS_FOR_FEATURE {
            " [tiny]"
        } else {
            ""
        };
        println!("{prefix:<60} {count:>8} {:>6}{marker}", types.len());
    }

    let total_records: usize = d4.values().map(|(c, _)| c).sum();
    let needs_split: Vec<_> = d4
        .iter()
        .filter(|(_, (c, _))| *c > MAX_RECORDS_BEFORE_SPLIT)
        .collect();

    println!();
    println!("Total records: {total_records}");
    println!("Total depth-4 groups: {}", d4.len());
    println!(
        "Groups needing split (>{MAX_RECORDS_BEFORE_SPLIT} records): {}",
        needs_split.len()
    );

    // ── Deep dive into large groups ───────────────────────────────────
    for (parent_prefix, (parent_count, _)) in &needs_split {
        println!();
        println!("Deep dive: {parent_prefix} ({parent_count} records)");
        println!("{:=<70}", "");

        // Try increasing depths until groups are small enough
        for depth in 5..=MAX_DEPTH {
            let at_depth = collect_at_depth(db, depth);
            // Filter to children of this parent
            let children: BTreeMap<_, _> = at_depth
                .iter()
                .filter(|(k, _)| k.starts_with(*parent_prefix))
                .collect();

            let max_child = children.values().map(|(c, _)| c).max().copied().unwrap_or(0);
            let child_count = children.len();

            println!();
            println!("  depth {depth}: {child_count} sub-groups (largest: {max_child} records)");

            if child_count <= 30 || depth == MAX_DEPTH {
                // Show them
                let mut sorted: Vec<_> = children.into_iter().collect();
                sorted.sort_by(|a, b| (b.1).0.cmp(&(a.1).0));
                for (prefix, (count, types)) in sorted.iter().take(40) {
                    let marker = if *count > MAX_RECORDS_BEFORE_SPLIT {
                        " [SPLIT]"
                    } else {
                        ""
                    };
                    // Show relative prefix (strip common parent)
                    let rel = prefix.strip_prefix(*parent_prefix).unwrap_or(prefix);
                    let rel = rel.strip_prefix('/').unwrap_or(rel);
                    println!("    {count:>8} types={:<3} {rel}{marker}", types.len());
                }
            }

            if max_child <= MAX_RECORDS_BEFORE_SPLIT {
                println!("  -> depth {depth} achieves target granularity");
                break;
            }
        }
    }

    // ── Proposed auto-feature summary ─────────────────────────────────
    println!();
    println!("Proposed auto-feature rules");
    println!("===========================");
    println!();
    println!("  MIN_RECORDS_FOR_FEATURE = {MIN_RECORDS_FOR_FEATURE}");
    println!("  MAX_RECORDS_BEFORE_SPLIT = {MAX_RECORDS_BEFORE_SPLIT}");
    println!("  MAX_DEPTH = {MAX_DEPTH}");
    println!();
    println!("Algorithm:");
    println!("  1. Start at depth 4");
    println!("  2. Groups with < {MIN_RECORDS_FOR_FEATURE} records: merge into parent or 'misc'");
    println!("  3. Groups with > {MAX_RECORDS_BEFORE_SPLIT} records: recurse deeper");
    println!("  4. Stop at depth {MAX_DEPTH} regardless");
    println!("  5. Each final group = one Cargo feature");
    println!();

    // Count how many would be features vs. tiny
    let features: Vec<_> = d4
        .iter()
        .filter(|(_, (c, _))| *c >= MIN_RECORDS_FOR_FEATURE && *c <= MAX_RECORDS_BEFORE_SPLIT)
        .collect();
    let tiny: Vec<_> = d4
        .iter()
        .filter(|(_, (c, _))| *c < MIN_RECORDS_FOR_FEATURE)
        .collect();

    println!(
        "At depth 4: {} direct features, {} need splitting, {} tiny (merge into misc)",
        features.len(),
        needs_split.len(),
        tiny.len()
    );
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
    out.push_str("full = [\n");
    for f in &leaf_features {
        let _ = writeln!(out, "    \"{f}\",");
    }
    out.push_str("]\n");

    // Parent features (aliases).
    for (parent, children) in &feature_map.parent_features {
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

    // Parent features forward to their children in sc-extract-generated.
    for (parent, children) in &feature_map.parent_features {
        let _ = write!(out, "{parent} = [");
        let valid: Vec<&String> = children.iter().filter(|c| all_features.contains(c)).collect();
        for (i, child) in valid.iter().enumerate() {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "\"sc-extract-generated/{child}\"");
        }
        out.push_str("]\n");
    }

    // Leaf features forward 1:1.
    out.push('\n');
    out.push_str("# Auto-generated: each feature forwards to sc-extract-generated.\n");
    for f in all_features {
        if f == "core" { continue; }
        let _ = writeln!(out, "{f} = [\"sc-extract-generated/{f}\"]");
    }

    write_file(cargo_path, &out)
}

fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{now}")
}
