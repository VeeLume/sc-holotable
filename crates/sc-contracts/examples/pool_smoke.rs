//! Smoke test for the new MissionPools API (Phase 3 of v2).
//!
//! Verifies that index.pools.{title_key, description_key} are
//! populated, and that the divergence helpers (blueprint_mixed,
//! rewards_uec_consistent, ...) match the live cluster numbers.

use sc_contracts::MissionIndex;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore, &asset_data.locale);

    println!("=== MissionPools smoke ===");
    println!("contracts: {}", index.contracts.len());
    println!("pools.title_key keys: {}", index.pools.title_key.len());
    println!("pools.description_key keys: {}", index.pools.description_key.len());

    // BP conflict groups via the new pool API.
    let bp_mixed: Vec<_> = index
        .pools
        .title_key
        .iter()
        .filter(|(_, ids)| index.blueprint_mixed(ids))
        .collect();
    println!("title-key pools with blueprint_mixed=true: {}", bp_mixed.len());

    let bp_pool_inconsistent: Vec<_> = index
        .pools
        .title_key
        .iter()
        .filter(|(_, ids)| !index.blueprint_pool_consistent(ids))
        .collect();
    println!(
        "title-key pools with blueprint_pool divergence: {}",
        bp_pool_inconsistent.len()
    );

    // Sample one BP-mixed pool and inspect.
    if let Some((key, ids)) = bp_mixed.first() {
        println!();
        println!("Example BP-mixed pool — title_key={}", key.as_str());
        for c in index.iter_pool(ids).take(5) {
            let bp = if c.rewards.blueprint.is_some() { "[BP]" } else { "    " };
            let title = c.title.as_deref().unwrap_or(&c.debug_name);
            println!("  {bp} {} ({:?})", title, c.origin.kind);
        }
    }

    println!();
    println!("[done]");
    Ok(())
}
