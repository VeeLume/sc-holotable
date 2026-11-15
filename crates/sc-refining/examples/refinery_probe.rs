//! Where refining and crafting happen (LIVE 4.10, 2026-11-15).
//!
//! Prints the refining processes, the refinery classes and the star-map
//! places object containers put them at. Asserts that every speed x quality
//! pair has a process, that refineries reach at least 15 places, and that
//! no object container places a crafter (a class with
//! `CrafterComponentParams`: the item fabricators and crafting benches are
//! base-building items a player places, so crafting has no star-map place).
//!
//! ```bash
//! cargo run -p sc-refining --release --example refinery_probe
//! ```

use std::collections::HashSet;

use sc_extract::generated::DataForgeComponentParamsPtr;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_locations::{ObjectContainers, normalize_socpak_path};
use sc_refining::{
    Refineries, RefineryPlacements, RefiningProcesses, RefiningQuality, RefiningSpeed,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let db = datacore.db();
    let name = |g: &Guid| {
        db.record(g)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_else(|| g.to_string())
    };

    let processes = RefiningProcesses::build(&datacore);
    println!("refining processes:");
    for p in processes.values() {
        println!(
            "  {:<32} {:<7} {:<9} {}",
            name(&p.guid),
            p.speed.as_dcb_str(),
            p.quality.as_dcb_str(),
            p.name.as_str()
        );
    }
    for speed in [
        RefiningSpeed::Slow,
        RefiningSpeed::Normal,
        RefiningSpeed::Fast,
    ] {
        for quality in [
            RefiningQuality::Wasteful,
            RefiningQuality::Normal,
            RefiningQuality::Careful,
        ] {
            assert!(
                processes
                    .values()
                    .any(|p| p.speed == speed && p.quality == quality),
                "no {speed:?} / {quality:?} process"
            );
        }
    }

    let refineries = Refineries::build(&datacore);
    let placements = RefineryPlacements::cook(&assets, &refineries)?;
    for (class, list) in placements.iter() {
        println!("{} ({class}):", name(class));
        for p in list {
            let places: Vec<String> = p.places.iter().map(|(g, _)| name(g)).collect();
            if places.is_empty() {
                println!("  {:>2}  (no place) {}", p.count, p.socpak);
            } else {
                println!("  {:>2}  {}", p.count, places.join(", "));
            }
        }
    }
    let reached = placements.places().count();
    println!("places with a refinery: {reached}");
    assert!(!refineries.is_empty(), "no refinery class");
    assert!(reached >= 15, "refineries reach only {reached} places");

    // Crafters: the classes exist, object containers place none of them.
    let store = datacore.records();
    let crafters: HashSet<Guid> = store
        .records
        .multi_feature
        .entity_class_definition
        .iter()
        .filter(|(_, h)| {
            h.get(&store.pools).is_some_and(|ecd| {
                ecd.components
                    .iter()
                    .any(|c| matches!(c, DataForgeComponentParamsPtr::CrafterComponentParams(_)))
            })
        })
        .map(|(g, _)| *g)
        .collect();
    println!(
        "crafter classes: {}",
        crafters.iter().map(&name).collect::<Vec<_>>().join(", ")
    );
    assert!(!crafters.is_empty(), "no crafter class");
    let roots: Vec<String> = assets
        .find(|n| n.to_ascii_lowercase().ends_with(".socpak"))
        .map(|e| normalize_socpak_path(e.name))
        .collect();
    let (_, placed) = ObjectContainers::walk(&assets, &roots, |_, _, root| {
        root.find_all("Entity")
            .filter_map(|e| e.attr("EntityClassGUID")?.parse::<Guid>().ok())
            .filter(|g| crafters.contains(g))
            .collect::<Vec<_>>()
    })?;
    let placed: usize = placed.values().map(Vec::len).sum();
    println!("crafters placed by object containers: {placed}");
    assert_eq!(placed, 0, "an object container places a crafter");
    println!("asserts passed");
    Ok(())
}
