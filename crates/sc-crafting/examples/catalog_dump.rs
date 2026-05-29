//! Validate the crafting catalog against live DCB: full catalog count +
//! the P4-AR trigger case (present in the full catalog, named via the
//! baked key). Pools are a missions concern — see sc-missions.
//!
//! ```bash
//! cargo run -p sc-crafting --release --example catalog_dump
//! ```

use sc_crafting::all_blueprints;
use sc_extract::{AssetConfig, AssetData, AssetSource};
use sc_items::Items;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());

    let catalog = all_blueprints(&datacore, &items);
    let craftable = catalog
        .iter()
        .filter(|b| b.crafted_entity_guid.is_some())
        .count();
    println!(
        "all_blueprints : {} ({} with crafted_entity)",
        catalog.len(),
        craftable
    );

    println!("P4-AR catalog entries (display_name needs only the locale now):");
    for b in &catalog {
        if let Some(n) = b.display_name(&asset_data.locale)
            && n.contains("P4-AR")
        {
            println!("  {n:?}");
        }
    }
    Ok(())
}
