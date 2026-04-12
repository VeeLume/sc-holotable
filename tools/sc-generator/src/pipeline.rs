//! Orchestration — opens the P4K, extracts and parses the DCB, and drives
//! the `emit` module to write generated Rust files.

use std::path::{Path, PathBuf};
use std::time::Instant;

use svarog_datacore::DataCoreDatabase;
use svarog_p4k::P4kArchive;

use crate::emit;

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

    // Remove stale files from previous runs so bucket renames don't leave
    // orphan files hanging around (e.g. `types_a.rs` from an earlier
    // first-letter bucketing when we've switched to two-letter bucketing).
    //
    // Scope is limited to generator-owned filenames: `types_*.rs`, the
    // legacy single-file `types.rs`, `enums.rs`, `metadata.rs`, and `mod.rs`.
    // We deliberately do NOT blow away the whole directory — if a user
    // dropped a README or similar there, it stays.
    clean_generated_dir(&options.out_dir)?;

    // Compute the shared struct-name dedup table once; every emission
    // stage consumes it so their views of "which types exist" stay
    // consistent.
    let emitted_names = emit::compute_emitted_struct_names(&db);
    let pool_fields = emit::compute_pool_field_names(&emitted_names);
    // Bucket assignments are the shared grouping used by the type bucket
    // files AND by the `DataPools` / `RecordIndex` sub-struct splits, so
    // every stage has the same idea of which bucket a type belongs to.
    let bucket_assignments = emit::compute_bucket_assignments(&emitted_names);

    // Emit files.
    tracing::info!("emitting type buckets");
    let buckets = emit::emit_types(&db, &emitted_names, &pool_fields, &bucket_assignments);
    for (bucket, source) in &buckets {
        let file_name = format!("types_{bucket}.rs");
        write_file(&options.out_dir.join(&file_name), source)?;
    }

    tracing::info!("emitting enums.rs");
    let enums_src = emit::emit_enums(&db);
    write_file(&options.out_dir.join("enums.rs"), &enums_src)?;

    tracing::info!("emitting data_pools.rs");
    let data_pools_src = emit::emit_data_pools(&emitted_names, &pool_fields, &bucket_assignments);
    write_file(&options.out_dir.join("data_pools.rs"), &data_pools_src)?;

    tracing::info!("emitting record_index.rs");
    let record_index_src = emit::emit_record_index(&db, &emitted_names, &bucket_assignments);
    write_file(&options.out_dir.join("record_index.rs"), &record_index_src)?;

    tracing::info!("emitting record_store.rs");
    let record_store_src = emit::emit_record_store(&db, &emitted_names, &bucket_assignments);
    write_file(&options.out_dir.join("record_store.rs"), &record_store_src)?;

    tracing::info!("emitting metadata.rs");
    let metadata_src = emit::emit_metadata(&metadata);
    write_file(&options.out_dir.join("metadata.rs"), &metadata_src)?;

    tracing::info!("emitting mod.rs");
    let bucket_names: Vec<String> = buckets.keys().cloned().collect();
    let mod_src = emit::emit_mod_file(&bucket_names);
    write_file(&options.out_dir.join("mod.rs"), &mod_src)?;

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

fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{now}")
}
