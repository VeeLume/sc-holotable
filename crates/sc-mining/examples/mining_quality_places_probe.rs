//! Does one provider give the same quality odds at every place it serves?
//! (LIVE 4.10, 2026-11-15.) A `HarvestableProviderPreset` often serves
//! several places (reused asteroid fields); the quality roll takes a location
//! override from the rock's place and its parents
//! ([`QualityRoll::locations_at`]). For every provider with more than one
//! place and every element its rocks can hold, this groups the places by
//! [`QualityRoll::source`] and asserts:
//!
//! - places with the same source get the same exact odds (distinct
//!   sources may still share odds: every `Rockcracker` cluster is its own
//!   override location with one distribution);
//! - the Keeger belt splits: the 30 placed `Rockcracker` clusters carry
//!   their own Savrilium and Torite overrides, its other places take the
//!   default; no other provider splits;
//! - the `PyroSolarSystem` override (most resources) lies on no place's
//!   chain: Pyro's bodies are children of `PyroStar`, and only the
//!   `SSolarSystem` record links the system location.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_quality_places_probe
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection, RecordPaths,
};
use sc_gathering::{ProviderLocations, Providers};
use sc_locations::Locations;
use sc_mining::{Compositions, Elements, QualityRoll, QualitySource, RockTypes};
use sc_resources::{LocationOverrideRef, Quality, Resources};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let store = datacore.records();
    let resources = Resources::build(store);
    let catalog = Quality::build(store);
    let elements = Elements::build(&datacore, &resources);
    let compositions = Compositions::build(&datacore);
    let rock_types = RockTypes::build(&datacore);
    let providers = Providers::build(store, &RecordPaths::build(&datacore));
    let places = ProviderLocations::cook(&assets, store)?;
    let locations = Locations::build(store);
    let db = datacore.db();
    let name = |guid: &Guid| {
        db.record(guid)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_else(|| guid.to_string())
    };

    // Every location an override names, with how many resources name it.
    let mut named: HashMap<Guid, usize> = HashMap::new();
    for (_, resource) in resources.iter() {
        let entries = match resource
            .quality
            .as_ref()
            .and_then(|w| w.location_override.as_ref())
        {
            Some(LocationOverrideRef::Inline(e)) => e.as_slice(),
            Some(LocationOverrideRef::Record(g)) => catalog
                .location_override(g)
                .map_or(&[][..], |r| r.entries.as_slice()),
            _ => &[],
        };
        for loc in entries.iter().filter_map(|e| e.location) {
            *named.entry(loc).or_default() += 1;
        }
    }
    let chains: HashSet<Guid> = providers
        .iter()
        .flat_map(|(g, _)| places.locations_of(g))
        .flat_map(|p| QualityRoll::locations_at(&locations, p))
        .collect();
    let mut unreached: Vec<(String, usize)> = named
        .iter()
        .filter(|(g, _)| !chains.contains(g))
        .map(|(g, n)| (name(g), *n))
        .collect();
    unreached.sort();
    println!(
        "{} override locations, {} on no provider place's chain",
        named.len(),
        unreached.len()
    );
    for (n, count) in unreached.iter().filter(|(n, _)| !n.contains("Rockcracker")) {
        println!("  unreached: {n} ({count} resources)");
    }
    let pyro = unreached
        .iter()
        .find(|(n, _)| n == "PyroSolarSystem")
        .ok_or("PyroSolarSystem is reached now: recheck the Pyro gap")?;
    assert!(pyro.1 > 30, "PyroSolarSystem names {} resources", pyro.1);

    let mut splits: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new();
    let mut multi = 0;
    for (pg, provider) in providers.iter() {
        let at = places.locations_of(pg);
        if at.len() < 2 {
            continue;
        }
        multi += 1;
        let held: HashSet<Guid> = provider
            .groups
            .iter()
            .flat_map(|g| &g.elements)
            .filter_map(|e| e.harvestable.and_then(|h| rock_types.get(&h)))
            .filter_map(|r| r.composition.and_then(|c| compositions.get(&c)))
            .flat_map(|c| c.parts.iter().filter_map(|p| p.element))
            .collect();
        let chains: Vec<Vec<Guid>> = at
            .iter()
            .map(|p| QualityRoll::locations_at(&locations, p))
            .collect();
        for element in &held {
            let mut groups: Vec<(Option<QualitySource>, Vec<usize>)> = Vec::new();
            for (k, ids) in chains.iter().enumerate() {
                let source = roll(&elements, &resources, &catalog, ids).source(element);
                match groups.iter_mut().find(|(s, _)| *s == source) {
                    Some((_, ks)) => ks.push(k),
                    None => groups.push((source, vec![k])),
                }
            }
            let odds: Vec<_> = groups
                .iter()
                .map(|(_, ks)| {
                    let first =
                        roll(&elements, &resources, &catalog, &chains[ks[0]]).odds(element, 1.0);
                    for &k in &ks[1..] {
                        assert_eq!(
                            roll(&elements, &resources, &catalog, &chains[k]).odds(element, 1.0),
                            first
                        );
                    }
                    first
                })
                .collect();
            // Distinct sources can share odds (each Rockcracker cluster is
            // its own override location, all with one distribution).
            let mut distinct = Vec::new();
            for o in &odds {
                if !distinct.contains(&o) {
                    distinct.push(o);
                }
            }
            if distinct.len() > 1 {
                let overridden = groups
                    .iter()
                    .filter(|(s, _)| matches!(s, Some(QualitySource::Override(_))))
                    .map(|(_, ks)| ks.len())
                    .sum();
                splits
                    .entry(format!("{} ({} places)", name(pg), at.len()))
                    .or_default()
                    .push((name(element), overridden));
            }
        }
    }
    println!("{multi} providers serve more than one place");
    for (provider, elements) in &splits {
        for (element, overridden) in elements {
            println!("  {provider}: {element} overridden at {overridden} places");
        }
    }
    assert_eq!(splits.len(), 1, "providers splitting on quality");
    let keeger = splits
        .iter()
        .find(|(p, _)| p.starts_with("HPP_Nyx_KeegerBelt"))
        .ok_or("the Keeger belt no longer splits")?;
    let mut split_elements: Vec<&str> = keeger.1.iter().map(|(e, _)| e.as_str()).collect();
    split_elements.sort();
    assert_eq!(split_elements, ["Savrilium_Ore", "Torite_Ore"]);
    assert!(keeger.1.iter().all(|(_, n)| *n == 30));
    println!(
        "asserts passed: one source, one odds; only the Keeger belt splits \
         (30 Rockcracker clusters), PyroSolarSystem is on no chain"
    );
    Ok(())
}

fn roll<'a>(
    elements: &'a Elements,
    resources: &'a Resources,
    catalog: &'a Quality,
    locations: &'a [Guid],
) -> QualityRoll<'a> {
    QualityRoll {
        elements,
        resources,
        catalog,
        locations,
    }
}
