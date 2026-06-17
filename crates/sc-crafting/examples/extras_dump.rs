//! Live validation of the Categories / GlobalParams / GameplayProperties
//! types added on top of the v0.9.0 sc-crafting core.
//!
//! ```bash
//! cargo run -p sc-crafting --release --example extras_dump
//! ```

use sc_crafting::{
    Categories, DisplayTransformation, DistributionRef, GameplayProperties, GlobalParams, Quality,
    QualityDistributionShape,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordPaths};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("install : {} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let paths = RecordPaths::build(&datacore);
    let locale = &asset_data.locale;

    // -- Categories --
    let cats = Categories::build(&paths);
    println!("\n=== Categories ===");
    println!("count          : {}", cats.len());
    println!("database_guid  : {:?}", cats.database_guid);
    for c in cats.iter() {
        println!("  {} ({})", c.name, c.guid);
    }

    // -- GlobalParams --
    println!("\n=== GlobalParams ===");
    match GlobalParams::build(&datacore) {
        None => println!("(no CraftingGlobalParams record found)"),
        Some(gp) => {
            println!(
                "refining_multiplier             : {}",
                gp.refining_quality_unit_multiplier
            );
            println!(
                "default_composition_quality     : {}",
                gp.default_composition_quality
            );
            println!(
                "dismantle blacklist resources   : {}",
                gp.dismantle_blacklist_resources.len()
            );
            println!(
                "dismantle blacklist entityclass : {}",
                gp.dismantle_blacklist_entity_classes.len()
            );
            println!(
                "default whitelist               : {} blueprints unlocked at start",
                gp.default_blueprint_whitelist.len()
            );
            println!("  (with resolved paths:)");
            for g in &gp.default_blueprint_whitelist {
                let path = paths.get(g).map(|rp| rp.path.as_str()).unwrap_or("?");
                println!("    {g} → {path}");
            }
            println!(
                "default_selection_is_non_whitelist : {}",
                gp.default_selection_is_non_whitelist
            );
        }
    }

    // -- GameplayProperties --
    let gpp = GameplayProperties::build(&datacore);
    println!("\n=== GameplayProperties ===");
    println!("count : {}", gpp.len());
    let mut with_transform = 0;
    let mut with_overrides = 0;
    for p in gpp.iter() {
        if p.display_transformation.is_some() {
            with_transform += 1;
        }
        if !p.name_overrides.is_empty() {
            with_overrides += 1;
        }
    }
    println!("with transformation: {with_transform}");
    println!("with name overrides: {with_overrides}");

    println!("\n  (sample 8 with transformations):");
    let mut shown = 0;
    for p in gpp.iter() {
        if p.display_transformation.is_none() {
            continue;
        }
        if shown >= 8 {
            break;
        }
        let name = locale.resolve(&p.property_name_key).unwrap_or("?");
        let unit = locale.resolve(&p.unit_format_key).unwrap_or("?");
        let t = format_transform(p.display_transformation.as_ref().unwrap());
        println!("    {name:<40} unit={unit:<12} → {t}");
        shown += 1;
    }

    println!("\n  (sample with name_overrides):");
    for p in gpp.iter() {
        if p.name_overrides.is_empty() {
            continue;
        }
        let name = locale.resolve(&p.property_name_key).unwrap_or("?");
        println!("    {name}:");
        for o in &p.name_overrides {
            let n = locale.resolve(&o.property_name_key).unwrap_or("?");
            println!("      override → {n:?} when {:?}", o.condition);
        }
    }

    // -- Quality --
    let quality = Quality::build(&datacore);
    println!("\n=== Quality ===");
    println!(
        "distributions: {}  location_overrides: {}  quantizations: {}",
        quality.distributions().count(),
        quality.location_overrides().count(),
        quality.quantizations().count(),
    );

    // tally distribution shapes
    let mut normal = 0usize;
    let mut other = 0usize;
    for d in quality.distributions() {
        match &d.shape {
            Some(QualityDistributionShape::Normal { .. }) => normal += 1,
            _ => other += 1,
        }
    }
    println!("  distribution shapes: Normal={normal}  Other={other}");
    if let Some(d) = quality.distributions().next() {
        println!("  sample distribution: {} → {:?}", d.guid, d.shape);
    }

    // location override stats + reference resolution
    let total_entries: usize = quality.location_overrides().map(|o| o.entries.len()).sum();
    let mut inline = 0usize;
    let mut record_refs = 0usize;
    let mut resolved_refs = 0usize;
    for o in quality.location_overrides() {
        for e in &o.entries {
            match &e.distribution {
                Some(DistributionRef::Inline(_)) => inline += 1,
                Some(DistributionRef::Record(g)) => {
                    record_refs += 1;
                    if quality.distribution(g).is_some() {
                        resolved_refs += 1;
                    }
                }
                _ => {}
            }
        }
    }
    println!("  location override entries: {total_entries}");
    println!("    inline distributions  : {inline}");
    println!("    record-ref'd          : {record_refs} ({resolved_refs} resolve in our catalog)");

    // quantization band stats
    let total_bands: usize = quality.quantizations().map(|q| q.bands.len()).sum();
    println!("  quantization bands total: {total_bands}");
    if let Some(q) = quality.quantizations().next() {
        println!(
            "  sample quantization {} → {} bands:",
            q.guid,
            q.bands.len()
        );
        for b in q.bands.iter().take(3) {
            println!(
                "    Band {{ start={}, end={}, mapped_value={} }}",
                b.start, b.end, b.mapped_value
            );
        }
    }

    Ok(())
}

fn format_transform(t: &DisplayTransformation) -> String {
    use DisplayTransformation::*;
    match t {
        Scale { factor } => format!("Scale(×{factor})"),
        ConvertFactorToPercentChange => "ConvertFactorToPercentChange".into(),
        ConvertFactorToNegatedPercentChange => "ConvertFactorToNegatedPercentChange".into(),
        ConvertValueToFactorOfBaseValue => "ConvertValueToFactorOfBaseValue".into(),
        Sequence(items) => format!(
            "Sequence([{}])",
            items
                .iter()
                .map(format_transform)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Other { type_name, .. } => format!("Other({type_name})"),
    }
}
