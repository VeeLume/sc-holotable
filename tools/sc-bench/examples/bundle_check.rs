//! Validate the bundled walk against separate builds: build `Items`,
//! `Tags`, and `RecordPaths` both the standalone way and as one fused
//! [`BundledWalk`], then assert the outputs are identical.
//!
//! ```bash
//! cargo run -p sc-bench --release --example bundle_check
//! ```

use sc_extract::RecordCollection;
use sc_extract::{
    AssetConfig, AssetData, AssetSource, BundledWalk, RecordPaths, RecordPathsBuilder,
};
use sc_items::{Items, ItemsBuilder};
use sc_tags::{Tags, TagsBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();

    // Baseline: three independent builds (today's path).
    let items_sep = Items::build(store);
    let tags_sep = Tags::build(store);
    let paths_sep = RecordPaths::build(&datacore);

    // One fused pass over db.all_records().
    let (items_b, tags_b, paths_b) = BundledWalk::new(&datacore).run((
        ItemsBuilder::default(),
        TagsBuilder::default(),
        RecordPathsBuilder::default(),
    ));

    println!(
        "items  : sep {:>6}  bundle {:>6}",
        items_sep.len(),
        items_b.len()
    );
    println!(
        "tags   : sep {:>6}  bundle {:>6}",
        tags_sep.len(),
        tags_b.len()
    );
    println!(
        "paths  : sep {:>6}  bundle {:>6}",
        paths_sep.len(),
        paths_b.len()
    );

    assert_eq!(items_sep.len(), items_b.len(), "Items count mismatch");
    assert_eq!(tags_sep.len(), tags_b.len(), "Tags count mismatch");
    assert_eq!(paths_sep.len(), paths_b.len(), "RecordPaths count mismatch");

    // Content equality, not just counts.
    let item_mismatch = items_sep
        .iter()
        .filter(|(g, it)| items_b.get(g) != Some(*it))
        .count();
    assert_eq!(item_mismatch, 0, "Items content differs ({item_mismatch})");

    let path_mismatch = paths_sep
        .iter()
        .filter(|r| paths_b.get(&r.guid) != Some(*r))
        .count();
    assert_eq!(
        path_mismatch, 0,
        "RecordPaths content differs ({path_mismatch})"
    );

    // Tags: parent links must match too (the finish-time pass-2).
    let tag_mismatch = tags_sep
        .values()
        .filter(|n| tags_b.get(&n.guid) != Some(*n))
        .count();
    assert_eq!(tag_mismatch, 0, "Tags content differs ({tag_mismatch})");

    println!("bundled walk matches separate builds (counts + content) ✓");
    Ok(())
}
