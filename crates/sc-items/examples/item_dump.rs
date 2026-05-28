//! Validate the typed `ItemCache` against the live DCB: total count,
//! inventory-item count, and a P4-AR spot check (confirms the typed walk
//! covers the same ground the old raw `localized_items` walk did).
//!
//! ```bash
//! cargo run -p sc-items --release --example item_dump
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, DatacoreConfig};
use sc_items::ItemCache;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

    let items = ItemCache::build(&datacore);
    let inv = items.iter().filter(|(_, it)| it.is_inventory_item()).count();
    println!("ItemCache entries : {}", items.len());
    println!("inventory items   : {inv}");

    // Typed Type distribution (top 30 by count) — confirm how NOITEM/UNDEFINED decode.
    let mut by_type: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for (_g, it) in items.iter() {
        *by_type.entry(format!("{:?}", it.item_type)).or_default() += 1;
    }
    let mut v: Vec<_> = by_type.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    println!("-- typed Type distribution (top 30) --");
    for (t, n) in v.iter().take(30) {
        println!("  {n:>6}  {t}");
    }

    println!("P4-AR matches:");
    let mut hits = 0;
    for (_g, it) in items.iter() {
        if let Some(n) = it.display_name(&asset_data.locale)
            && n.contains("P4-AR")
        {
            println!("  {n:?}  type={:?} sub={:?}", it.item_type, it.item_sub_type);
            hits += 1;
        }
    }
    println!("({hits} P4-AR display-name hits)");
    Ok(())
}
