//! Re-proves [`Site::markers`] on the installed build: for every provider
//! whose fields resolve to a system's star (rings and belts in unnamed
//! segments), the quantum targets in each field's segment.
//!
//! Asserts that the Glaciem Ring and the Keeger Belt in Nyx each have
//! markers that quantum travel targets (LIVE 4.10: Levski among the ring's,
//! the People's Service Stations among the belt's), and prints the sites
//! whose own segment has none (one Glaciem segment holds only a QV
//! Logistics Station; the Aaron Halo's field sits outside any segment).
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
            let ring = record(provider);
            if ring == "HPP_Nyx_GlaciemRing" || ring == "HPP_Nyx_KeegerBelt" {
                assert!(!reachable.is_empty(), "{ring}: no quantum marker at all");
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 2, "the Glaciem Ring and the Keeger Belt");
    println!("\nthe Glaciem Ring and the Keeger Belt each have quantum markers");
    Ok(())
}
