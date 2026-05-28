//! Validate the crafting catalog against live DCB: full catalog count,
//! pool count, and the P4-AR trigger case (present in the full catalog +
//! named via the baked key, but absent from any mission reward pool).
//!
//! ```bash
//! cargo run -p sc-crafting --release --example catalog_dump
//! ```

use sc_crafting::{BlueprintPoolRegistry, all_blueprints};
use sc_extract::{AssetConfig, AssetData, AssetSource};
use sc_items::ItemCache;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let items = ItemCache::build(&datacore);

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

    let reg = BlueprintPoolRegistry::build(&datacore, &items);
    println!("pools          : {}", reg.len());

    println!("P4-AR catalog entries (display_name needs only the locale now):");
    for b in &catalog {
        if let Some(n) = b.display_name(&asset_data.locale)
            && n.contains("P4-AR")
        {
            let in_pool = !reg.pools_containing_item(&b.blueprint_record_guid).is_empty();
            println!("  {n:?}  in_mission_pool={in_pool}");
        }
    }
    Ok(())
}
