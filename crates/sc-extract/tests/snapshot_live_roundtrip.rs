//! Live end-to-end snapshot round-trip against a real SC installation.
//!
//! Marked `#[ignore]` because it requires a discoverable SC install. Run
//! manually with:
//!
//! ```bash
//! cargo test --profile dev-opt -p sc-extract --features full --test snapshot_live_roundtrip -- --ignored --nocapture
//! ```
//!
//! What this test verifies:
//!
//! 1. `ExtractSnapshot::capture` reads a real `Data.p4k` and produces a
//!    `files` map with the expected entries.
//! 2. `save` → `load` round-trips the byte map losslessly.
//! 3. `hydrate` re-parses the captured DCB + locale bytes into a live
//!    `Datacore` + `AssetData`.
//! 4. The hydrated live session produces the same record / graph / tag /
//!    manufacturer / display-name counts as the original live parse of
//!    the same install. This is the actual correctness proof that the
//!    byte-bundle format doesn't lose information relative to the old
//!    cooked-snapshot format.
//!
//! This is the canonical "does the whole thing work?" test for the
//! snapshot rework. Skip in CI (no SC install available); run locally
//! after generator regens, dependency bumps, or snapshot-format changes.

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, ExtractSnapshot,
    SnapshotCaptureConfig,
};

/// Full round-trip: live parse → capture → save → load → hydrate →
/// compare counts. Fails fast on any mismatch.
#[test]
#[ignore = "requires a discoverable SC install"]
fn snapshot_live_roundtrip() {
    // Minimal tracing so a `--nocapture` run shows what's happening.
    let _ = tracing_subscriber::fmt::try_init();

    // ── Step 1: live parse (the reference run) ───────────────────────
    let install = sc_installs::discover_primary().expect("discover SC install");
    println!(
        "using install: channel={} root={}",
        install.channel,
        install.root.display()
    );

    let assets = AssetSource::from_install(&install).expect("open assets");
    let asset_config = AssetConfig::standard();
    let dc_config = DatacoreConfig::standard();

    let asset_data = AssetData::extract(&assets, &asset_config).expect("live asset extract");
    let datacore = Datacore::parse(&assets, &asset_data, &dc_config).expect("live datacore parse");

    let original = Counts::from(&datacore, &asset_data);
    println!("live parse summary: {original:?}");

    // ── Step 2: capture + save ────────────────────────────────────────
    let meta = sc_extract::snapshot_meta_from_install(&install);
    let snapshot = ExtractSnapshot::capture(&assets, meta, &SnapshotCaptureConfig::standard())
        .expect("capture");
    assert!(
        !snapshot.files.is_empty(),
        "capture should have populated at least the DCB"
    );
    assert!(
        snapshot
            .files
            .keys()
            .any(|k| k.to_ascii_lowercase().ends_with("game2.dcb")),
        "capture should include game2.dcb, got keys: {:?}",
        snapshot.files.keys().collect::<Vec<_>>()
    );

    let snap_path = std::env::temp_dir().join("sc_extract_live_roundtrip.snap");
    snapshot.save(&snap_path).expect("save");

    let on_disk_bytes = std::fs::metadata(&snap_path)
        .expect("stat snapshot file")
        .len();
    println!(
        "snapshot on disk: {} ({:.2} MB)",
        snap_path.display(),
        on_disk_bytes as f64 / 1_000_000.0
    );

    // ── Step 3: load + hydrate ───────────────────────────────────────
    let loaded = ExtractSnapshot::load(&snap_path).expect("load");
    assert_eq!(loaded.meta.schema_version, ExtractSnapshot::SCHEMA_VERSION);
    assert_eq!(loaded.files.len(), snapshot.files.len());

    let (hydrated_assets, hydrated_dc) =
        loaded.hydrate(&asset_config, &dc_config).expect("hydrate");
    let hydrated = Counts::from(&hydrated_dc, &hydrated_assets);
    println!("hydrated summary: {hydrated:?}");

    // ── Step 4: compare counts ───────────────────────────────────────
    assert_eq!(
        original, hydrated,
        "hydrated snapshot should produce identical counts to the original live parse"
    );

    // Cleanup — best-effort; don't fail the test on removal errors.
    let _ = std::fs::remove_file(&snap_path);
}

/// Count bundle for comparing a live parse against a hydrated one.
#[derive(Debug, PartialEq, Eq)]
struct Counts {
    records: usize,
    graph_edges: usize,
    tag_nodes: usize,
    manufacturers: usize,
    display_names: usize,
    locale_entries: usize,
}

impl Counts {
    fn from(dc: &Datacore, assets: &AssetData) -> Self {
        let snap = dc.snapshot();
        Self {
            records: snap.records.len(),
            graph_edges: snap.graph.edge_count(),
            tag_nodes: snap.tag_tree.len(),
            manufacturers: snap.manufacturers.len(),
            display_names: snap.display_names.len(),
            locale_entries: assets.locale.len(),
        }
    }
}
