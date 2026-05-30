//! Validate `Resources` against live DCB: total count, name resolution,
//! and the refining graph (which resources have a refined_version target,
//! and what fraction of the catalog forms refining pairs).
//!
//! ```bash
//! cargo run -p sc-resources --release --example resource_dump
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
use sc_resources::Resources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let resources = Resources::build(datacore.records());

    println!("\nresources total : {}", resources.len());

    // Resolve all names; count any that fail to resolve.
    let mut named = 0;
    let mut unresolved = 0;
    let mut refining_pairs = 0;
    let mut sorted: Vec<_> = resources.all().collect();
    sorted.sort_by(|a, b| {
        let an = asset_data.locale.resolve(&a.name_key).unwrap_or("");
        let bn = asset_data.locale.resolve(&b.name_key).unwrap_or("");
        an.cmp(bn)
    });

    println!("\n=== full catalog ===");
    for r in &sorted {
        let nm = asset_data.locale.resolve(&r.name_key);
        match nm {
            Some(n) if !n.is_empty() => named += 1,
            _ => unresolved += 1,
        }
        let refined_name = r
            .refined_version
            .and_then(|g| resources.get(&g))
            .and_then(|next| asset_data.locale.resolve(&next.name_key));
        let suffix = match (&r.refined_version, refined_name) {
            (Some(g), Some(n)) => {
                refining_pairs += 1;
                format!("  →refined: {n} ({g})")
            }
            (Some(g), None) => format!("  →refined: ?? ({g})"),
            (None, _) => String::new(),
        };
        println!(
            "  {} ({}){suffix}",
            nm.unwrap_or("(unresolved)"),
            r.guid
        );
    }

    println!("\nname resolution : {named} ok / {unresolved} unresolved");
    println!("refining edges  : {refining_pairs} resources have a refined_version");

    Ok(())
}
