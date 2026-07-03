//! Prove the hauling cargo manifest (commodity + SCU + box size) is in the DCB:
//! dump every HaulingOrderContent_Resource, resolving `resource` -> commodity name.
//!
//! ```bash
//! cargo run -p sc-missions --release --example hauling_dump
//! ```
use std::collections::BTreeMap;

use sc_extract::generated::{RecordLookup, ResourceType};
use sc_extract::{AssetConfig, AssetData, AssetSource, LocaleMap};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let pools = &store.pools;
    let locale: &LocaleMap = &asset_data.locale;

    let mut commodity_hist: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0usize;
    let mut samples = 0usize;
    for c in pools
        .contracts
        .hauling_order_content_resource
        .iter()
        .flatten()
    {
        total += 1;
        let name = c
            .resource
            .and_then(|g| ResourceType::lookup(&store.records, &g))
            .and_then(|h| h.get(pools))
            .and_then(|rt| locale.resolve(&rt.display_name))
            .unwrap_or("<unresolved>")
            .to_string();
        *commodity_hist.entry(name.clone()).or_default() += 1;
        if samples < 18 {
            samples += 1;
            println!(
                "  {name:<18} min_scu={:<6} max_scu={:<6} max_box={}",
                c.min_scu, c.max_scu, c.max_container_size
            );
        }
    }
    println!("\nHaulingOrderContent_Resource records: {total}");
    println!("distinct commodities: {}", commodity_hist.len());
    let mut v: Vec<_> = commodity_hist.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    for (k, n) in v.iter().take(40) {
        println!("  {n:>4}  {k}");
    }
    Ok(())
}
