//! Validate the typed `Tags` against the live DCB: total count + a few
//! sample paths (should match the prior raw-walk count of ~18,313).
//!
//! ```bash
//! cargo run -p sc-tags --release --example tag_dump
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource};
use sc_tags::Tags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;

    let tags = Tags::build(datacore.records());
    println!("tag nodes : {}", tags.len());
    println!("roots     : {}", tags.roots().count());

    // Sample a few deepest paths to confirm hierarchy is wired.
    let mut by_depth: Vec<(usize, String)> = tags
        .iter()
        .map(|n| {
            let path = tags.path(&n.guid);
            (path.len(), path.join(" > "))
        })
        .collect();
    by_depth.sort_by(|a, b| b.0.cmp(&a.0));
    println!("deepest paths:");
    for (_d, p) in by_depth.iter().take(8) {
        println!("  {p}");
    }
    Ok(())
}
