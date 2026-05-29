//! Validate the umbrella's coordination build-context + HolotableSnapshot
//! against the live DCB: build all foundations in one bundled pass, confirm
//! counts match independent builds, then round-trip the snapshot.
//!
//! ```bash
//! cargo run -p sc-holotable --release --example foundations_snapshot --features full
//! ```

use sc_holotable::asset::{
    AssetConfig, AssetData, AssetSource, Datacore, RecordPaths, snapshot_meta_from_install,
};
use sc_holotable::items::ItemCache;
use sc_holotable::manufacturers::ManufacturerRegistry;
use sc_holotable::tags::TagTree;
use sc_holotable::{HolotableSnapshot, build_foundations};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();

    // Independent builds (baseline).
    let items = ItemCache::build(store);
    let tags = TagTree::build(store);
    let mfrs = ManufacturerRegistry::build(store);
    let paths = RecordPaths::build(&datacore);

    // One bundled pass via the umbrella build-context.
    let f = build_foundations(&datacore);

    println!(
        "items         : sep {:>6}  bundle {:>6}",
        items.len(),
        f.items.len()
    );
    println!(
        "tags          : sep {:>6}  bundle {:>6}",
        tags.len(),
        f.tags.len()
    );
    println!(
        "manufacturers : sep {:>6}  bundle {:>6}",
        mfrs.len(),
        f.manufacturers.len()
    );
    println!(
        "paths         : sep {:>6}  bundle {:>6}",
        paths.len(),
        f.paths.len()
    );
    assert_eq!(items.len(), f.items.len(), "items mismatch");
    assert_eq!(tags.len(), f.tags.len(), "tags mismatch");
    assert_eq!(mfrs.len(), f.manufacturers.len(), "manufacturers mismatch");
    assert_eq!(paths.len(), f.paths.len(), "paths mismatch");

    // HolotableSnapshot round-trip (all four cooked indices).
    let snap = HolotableSnapshot::from_foundations(&f);
    let path = std::env::temp_dir().join("holotable_example.cook");
    snap.save(snapshot_meta_from_install(&install), &path)?;
    let loaded = HolotableSnapshot::load(&path)?;
    assert_eq!(loaded.items.as_ref().map(ItemCache::len), Some(f.items.len()));
    assert_eq!(
        loaded.paths.as_ref().map(RecordPaths::len),
        Some(f.paths.len())
    );
    assert_eq!(loaded.tags.as_ref().map(TagTree::len), Some(f.tags.len()));
    assert_eq!(
        loaded.manufacturers.as_ref().map(ManufacturerRegistry::len),
        Some(f.manufacturers.len())
    );
    let size = std::fs::metadata(&path)?.len();
    let _ = std::fs::remove_file(&path);
    println!(
        "HolotableSnapshot round-trip ✓  ({:.2} MB on disk)",
        size as f64 / 1_000_000.0
    );

    Ok(())
}
