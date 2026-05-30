//! Live validation of the v0.9.0 `Blueprints` surface against the
//! current install. Reports catalog stats and pretty-prints a few
//! sample blueprints with their full Recipe + Cost tree resolved.
//!
//! ```bash
//! cargo run -p sc-crafting --release --example blueprint_dump
//! ```

use sc_crafting::{Blueprint, Blueprints, Cost, Process, RecipeResult};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, LocaleMap};
use sc_items::Items;
use sc_resources::{CargoQuantity, Resources};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("install : {} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());
    let resources = Resources::build(datacore.records());
    let blueprints = Blueprints::build(&datacore, &items);
    let locale = &asset_data.locale;

    println!("\n=== Catalog stats ===");
    println!("blueprints total : {}", blueprints.len());
    let mut process_counts = std::collections::BTreeMap::<&'static str, usize>::new();
    let mut tier_count_dist = std::collections::BTreeMap::<usize, usize>::new();
    let mut with_recipe = 0;
    let mut with_costs = 0;
    let mut with_research = 0;
    let mut with_optional = 0;
    let mut with_results = 0;
    for bp in blueprints.iter() {
        let key = match &bp.process {
            Process::Creation { .. } => "Creation",
            Process::Other { .. } => "Other",
        };
        *process_counts.entry(key).or_default() += 1;
        *tier_count_dist.entry(bp.tiers.len()).or_default() += 1;
        for tier in &bp.tiers {
            if let Some(recipe) = &tier.recipe {
                with_recipe += 1;
                if let Some(costs) = &recipe.costs {
                    if costs.mandatory.is_some() {
                        with_costs += 1;
                    }
                    if !costs.optional.is_empty() {
                        with_optional += 1;
                    }
                }
                if !recipe.results.is_empty() {
                    with_results += 1;
                }
            }
            if tier.research.is_some() {
                with_research += 1;
            }
        }
    }
    println!("process: {:?}", process_counts);
    println!("tier counts: {:?}", tier_count_dist);
    println!("tiers with recipe          : {with_recipe}");
    println!("tiers with mandatory cost  : {with_costs}");
    println!("tiers with optional costs  : {with_optional}");
    println!("tiers with non-empty results: {with_results}");
    println!("tiers with research        : {with_research}");

    println!("\n=== Sample blueprints ===");
    let mut sampled = 0;
    // P4-AR (behr_rifle_ballistic_01)
    for bp in blueprints.iter() {
        if let Some(name) = bp.display_name(locale)
            && name.contains("P4-AR")
        {
            print_blueprint(bp, locale, &items, &resources);
            sampled += 1;
            break;
        }
    }
    // Any blueprint that has research
    for bp in blueprints.iter() {
        if sampled >= 3 {
            break;
        }
        if bp.tiers.iter().any(|t| t.research.is_some()) {
            print_blueprint(bp, locale, &items, &resources);
            sampled += 1;
        }
    }
    // Cargo: confirm the Select(Select(Resource)) shape resolves to Resources
    Ok(())
}

fn print_blueprint(
    bp: &Blueprint,
    locale: &LocaleMap,
    _items: &Items,
    resources: &Resources,
) {
    let name = bp.display_name(locale).unwrap_or("(unresolved)");
    println!("\n--- {name} ---");
    println!("  record_guid    : {}", bp.blueprint_record_guid);
    println!("  category       : {:?}", bp.category);
    match &bp.process {
        Process::Creation { entity_class } => {
            println!("  process        : Creation → entity={:?}", entity_class);
        }
        Process::Other { type_name, .. } => {
            println!("  process        : Other({type_name})");
        }
    }
    println!("  tiers          : {}", bp.tiers.len());
    for (i, tier) in bp.tiers.iter().enumerate() {
        println!("  tier[{i}]:");
        if let Some(recipe) = &tier.recipe {
            if let Some(d) = &recipe.craft_time {
                println!("    craft_time   : {:?} ({}s total)", d, d.to_seconds());
            }
            if let Some(costs) = &recipe.costs
                && let Some(mc) = &costs.mandatory
            {
                println!("    mandatory:");
                print_cost(mc, locale, resources, 6);
            }
            if !recipe.results.is_empty() {
                println!("    results      : {} entries", recipe.results.len());
                for r in &recipe.results {
                    match r {
                        RecipeResult::Item {
                            entity_class,
                            quantity,
                            tier,
                        } => println!(
                            "      Item entity={:?} qty={} tier={}",
                            entity_class, quantity, tier
                        ),
                        RecipeResult::Resource { resource, quantity } => println!(
                            "      Resource res={:?} qty={:?}",
                            resource, quantity
                        ),
                        RecipeResult::Other { type_name, .. } => {
                            println!("      Other({type_name})")
                        }
                    }
                }
            }
        }
        if tier.research.is_some() {
            println!("    research     : present (unlock + costs both empty in 4.8)");
        }
    }
}

fn print_cost(cost: &Cost, locale: &LocaleMap, resources: &Resources, indent: usize) {
    let pad = " ".repeat(indent);
    match cost {
        Cost::Resource(rc) => {
            let rname = rc
                .resource
                .and_then(|g| resources.get(&g))
                .and_then(|r| locale.resolve(&r.name_key))
                .unwrap_or("?");
            let scu = rc
                .quantity
                .as_ref()
                .and_then(CargoQuantity::to_scu)
                .map(|s| format!("{s} SCU"))
                .unwrap_or("?".into());
            println!(
                "{pad}Resource: {rname} (minQ={}) qty={scu}",
                rc.min_quality
            );
        }
        Cost::Item(ic) => {
            println!(
                "{pad}Item: entity={:?} qty={} minQ={}",
                ic.entity_class, ic.quantity, ic.min_quality
            );
        }
        Cost::Select { count, options } => {
            println!("{pad}Select count={count} options={}", options.len());
            for opt in options {
                print_cost(opt, locale, resources, indent + 2);
            }
        }
        Cost::Other { type_name, .. } => println!("{pad}Other({type_name})"),
    }
}
