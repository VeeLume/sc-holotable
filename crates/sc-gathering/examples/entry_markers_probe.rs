//! Re-proves [`Site::markers`] on the installed build: for every provider
//! whose fields resolve to a system's star (rings and belts), the quantum
//! targets in each field's segment or inside its [`Ring`](sc_gathering::Ring).
//!
//! Asserts that the Glaciem Ring and the Keeger Belt in Nyx each have
//! markers that quantum travel targets (LIVE 4.10: Levski among the ring's,
//! the People's Service Stations among the belt's), and that the Aaron Halo,
//! outside any segment, carries its ring shape and the Wikelo Emporium Selo
//! Station inside it. Prints the sites without a marker of their own (one
//! Glaciem segment holds only a QV Logistics Station).
//!
//! ```bash
//! cargo run -p sc-gathering --release --example entry_markers_probe
//! ```

use std::collections::BTreeSet;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_gathering::{ProviderLocations, Site};
use sc_locations::{Locations, ObjectContainers, Universe, Waypoints};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let locations = Locations::build(datacore.records());
    let places = ProviderLocations::cook(&assets, datacore.records())?;
    let waypoints = Waypoints::build(&Universe::join(
        locations.clone(),
        ObjectContainers::systems(&assets, 1)?,
    ));
    let record = |g: &Guid| {
        datacore
            .db()
            .record(g)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_else(|| g.to_string())
    };
    let name = |g: &Guid| {
        locations
            .get(g)
            .and_then(|l| l.display_name(&data.locale))
            .unwrap_or("(unnamed)")
            .to_string()
    };

    let stars: Vec<Guid> = locations
        .iter()
        .filter(|(g, l)| l.parent.is_none() && !places.providers_at(g).is_empty())
        .map(|(g, _)| *g)
        .collect();
    let mut checked = 0;
    for star in stars {
        println!("\n== {}", name(&star));
        for provider in places.providers_at(&star) {
            let at_star: Vec<&Site> = places
                .sites_of(provider)
                .iter()
                .filter(|s| s.place == star)
                .collect();
            let reachable: BTreeSet<String> = at_star
                .iter()
                .flat_map(|s| &s.markers)
                .filter(|g| waypoints.get(g).is_some())
                .map(name)
                .collect();
            println!(
                "  {:<24} {} sites at the star, quantum markers: {:?}",
                record(provider),
                at_star.len(),
                reachable
            );
            for s in &at_star {
                let quantum = s
                    .markers
                    .iter()
                    .filter(|g| waypoints.get(g).is_some())
                    .count();
                if quantum == 0 {
                    let named: Vec<String> = s.markers.iter().map(name).collect();
                    println!(
                        "    site at {:?}: no quantum marker; named in segment: {named:?}",
                        s.position.map(|p| p.map(|v| (v / 1e6).round()))
                    );
                }
            }
            for s in at_star.iter().filter_map(|s| s.ring) {
                println!(
                    "    ring {:.4}-{:.4} Gm, {:.1} Mm deep",
                    s.inner_radius / 1e9,
                    s.outer_radius / 1e9,
                    s.depth / 1e6
                );
            }
            let field = record(provider);
            if field == "HPP_Nyx_GlaciemRing" || field == "HPP_Nyx_KeegerBelt" {
                assert!(!reachable.is_empty(), "{field}: no quantum marker at all");
                checked += 1;
            }
            if field == "HPP_AaronHalo" {
                assert!(
                    at_star.iter().any(|s| s.ring.is_some()),
                    "the Aaron Halo's ring shape"
                );
                assert!(
                    reachable.contains("Wikelo Emporium Selo Station"),
                    "the Wikelo Selo Station marks the Aaron Halo"
                );
                checked += 1;
            }
        }
    }
    assert_eq!(
        checked, 3,
        "the Glaciem Ring, the Keeger Belt, the Aaron Halo"
    );
    println!("\nthe Glaciem Ring, the Keeger Belt and the Aaron Halo each have quantum markers");

    // Every site: how quantum travel gets there. Its place itself, else the
    // nearest anchor that is a quantum target, else a marker.
    let (mut own, mut anchored, mut marked, mut none) = (0, 0, 0, Vec::new());
    let mut providers: Vec<Guid> = locations
        .iter()
        .flat_map(|(g, _)| places.providers_at(g).to_vec())
        .collect();
    providers.sort_by_key(Guid::to_string);
    providers.dedup();
    for provider in providers {
        for s in places.sites_of(&provider) {
            if waypoints.get(&s.place).is_some() {
                own += 1;
            } else if s.anchors.iter().any(|g| waypoints.get(g).is_some()) {
                anchored += 1;
            } else if s.markers.iter().any(|g| waypoints.get(g).is_some()) {
                marked += 1;
            } else {
                none.push(format!("{} at {}", record(&provider), name(&s.place)));
            }
        }
    }
    none.sort();
    none.dedup();
    println!(
        "sites: {own} at a quantum target, {anchored} under one, {marked} with a marker, {} with none: {none:?}",
        none.len()
    );
    assert!(anchored > 0, "no site reached through an anchor");
    Ok(())
}
