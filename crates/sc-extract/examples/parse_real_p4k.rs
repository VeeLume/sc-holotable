//! End-to-end smoke test against a real Star Citizen installation.
//!
//! Exercises every step of the staged API and verifies that the live
//! [`sc_extract::Datacore`] session keeps the [`DataCoreDatabase`] alive
//! for raw queries after high-level parsing.
//!
//! Usage:
//!
//! ```bash
//! # Auto-discover primary installation (standard config, no graph):
//! cargo run -p sc-extract --profile dev-opt --example parse_real_p4k
//!
//! # Explicit path:
//! cargo run -p sc-extract --profile dev-opt --example parse_real_p4k -- "C:/Games/StarCitizen/LIVE/Data.p4k"
//!
//! # Full extraction including reference graph:
//! cargo run -p sc-extract --profile dev-opt --example parse_real_p4k -- --all
//!
//! # Pattern 4: datacore-only (empty AssetData → no display names):
//! cargo run -p sc-extract --profile dev-opt --example parse_real_p4k -- --no-assets
//! ```

use std::path::PathBuf;
use std::time::Instant;

use sc_extract::{
    AssetConfig, AssetData, AssetSource, DatacoreConfig, ExtractSnapshot, SnapshotMeta,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let use_all = args.iter().any(|a| a == "--all");
    let skip_assets = args.iter().any(|a| a == "--no-assets");
    let explicit_path: Option<PathBuf> = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(Into::into);

    let dc_config = if use_all {
        println!("-> using DatacoreConfig::all() (includes reference graph)");
        DatacoreConfig::all()
    } else {
        println!("-> using DatacoreConfig::standard() (graph skipped)");
        DatacoreConfig::standard()
    };

    // ── Step 1: open assets (always needed — DCB lives inside the P4K) ────
    let t0 = Instant::now();
    let (assets, meta) = if let Some(p4k_path) = &explicit_path {
        println!("-> opening {}", p4k_path.display());
        let assets = AssetSource::open(p4k_path)?;
        let meta = SnapshotMeta {
            schema_version: ExtractSnapshot::SCHEMA_VERSION,
            game_version: "unknown".into(),
            build_id: "unknown".into(),
            extracted_at: sc_extract::current_timestamp(),
        };
        (assets, meta)
    } else {
        let install = sc_installs::discover_primary()?;
        println!(
            "-> found {} v{} at {}",
            install.channel,
            install.short_version(),
            install.root.display()
        );
        let assets = AssetSource::from_install(&install)?;
        let meta = sc_extract::snapshot_meta_from_install(&install);
        (assets, meta)
    };

    // ── Step 2: asset data (locale etc.) ──────────────────────────────────
    let asset_data = if skip_assets {
        println!("-> skipping AssetData (pattern 4 — display names will be empty)");
        AssetData::default()
    } else {
        println!("-> extracting asset data (locale)");
        AssetData::extract(&assets, &AssetConfig::standard())?
    };

    // ── Step 3: datacore parse ────────────────────────────────────────────
    println!("-> parsing datacore");
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &dc_config)?;

    let parse_secs = t0.elapsed().as_secs_f64();

    let snap_view = datacore.snapshot();

    println!();
    println!("Datacore summary");
    println!("----------------");
    println!("  game_version   : {}", meta.game_version);
    println!("  build_id       : {}", meta.build_id);
    println!("  extracted_at   : {}", meta.extracted_at);
    println!("  records        : {}", snap_view.records.len());
    println!("  locale entries : {}", asset_data.locale.len());
    println!("  manufacturers  : {}", snap_view.manufacturers.len());
    println!("  display names  : {}", snap_view.display_names.len());
    println!("  tag nodes      : {}", snap_view.tag_tree.len());
    println!("  graph edges    : {}", snap_view.graph.edge_count());
    println!("  parse time     : {parse_secs:.2}s");

    // ── Raw escape hatch: db() still works after parse ────────────────────
    println!();
    println!("Raw escape hatch");
    println!("----------------");
    let db = datacore.db();
    let entity_count = db.records_by_type("EntityClassDefinition").count();
    println!("  db.records_by_type(\"EntityClassDefinition\") : {entity_count}");

    // ── Snapshot round-trip ───────────────────────────────────────────────
    println!();
    println!("Snapshot round-trip");
    println!("-------------------");
    let dc_snapshot = datacore.into_snapshot();
    let expected_records = dc_snapshot.records.len();

    let snapshot = ExtractSnapshot {
        meta,
        asset_data: Some(asset_data),
        datacore: Some(dc_snapshot),
    };

    let snapshot_path = std::env::temp_dir().join("sc_extract_smoke.snap");
    let t1 = Instant::now();
    snapshot.save(&snapshot_path)?;
    let save_secs = t1.elapsed().as_secs_f64();
    let size = std::fs::metadata(&snapshot_path)?.len();
    println!("  snapshot file  : {}", snapshot_path.display());
    println!("  size on disk   : {:.2} MB", size as f64 / 1_000_000.0);
    println!("  save time      : {save_secs:.2}s");

    let t2 = Instant::now();
    let loaded = ExtractSnapshot::load(&snapshot_path)?;
    let load_secs = t2.elapsed().as_secs_f64();
    println!("  load time      : {load_secs:.2}s");
    let loaded_records = loaded
        .datacore
        .as_ref()
        .map(|d| d.records.len())
        .unwrap_or(0);
    println!("  loaded records : {loaded_records}");
    assert_eq!(loaded_records, expected_records);

    // ── Asset access smoke check ──────────────────────────────────────────
    println!();
    println!("Asset access");
    println!("------------");
    match assets.default_profile_xml() {
        Ok(bytes) => println!("  defaultProfile : {} bytes", bytes.len()),
        Err(e) => println!("  defaultProfile : not found ({e})"),
    }

    println!();
    println!("Done.");
    Ok(())
}
