//! Re-proves [`Waypoints`] on LIVE 4.10: the system object containers
//! alone (depth 0: bodies, Lagrange and jump points; depth 1: the stations
//! on them) position every quantum destination, a fraction of a full cook.
//! Checks the positions against the data's own orbits (Ariel sits its
//! `OrbitalRadius` from Hurston, to 4 m) and prints the waypoints per
//! system and some routes (Hurston–Crusader 31.924 Gm).
//!
//! ```bash
//! cargo run -p sc-locations --release --example waypoints_probe
//! ```

use std::collections::BTreeMap;
use std::time::Instant;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_locations::{LocationKind, Locations, ObjectContainers, Universe, Waypoints};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let locale = &data.locale;

    let t = Instant::now();
    let containers = ObjectContainers::systems(&assets, 1)?;
    println!(
        "systems(1): {} placements in {:?}",
        containers.len(),
        t.elapsed()
    );
    let universe = Universe::join(Locations::build(datacore.records()), containers);
    let waypoints = Waypoints::build(&universe);
    println!("{} waypoints", waypoints.len());

    let name = |g: &Guid| {
        waypoints
            .get(g)
            .and_then(|w| w.display_name(locale))
            .unwrap_or("?")
            .to_string()
    };
    let mut by_system: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    for (_, w) in waypoints.iter() {
        let system = w
            .system
            .and_then(|s| universe.star_map().get(&s))
            .and_then(|l| l.display_name(locale))
            .unwrap_or("-")
            .to_string();
        by_system
            .entry(system)
            .or_default()
            .entry(w.kind.as_dcb_str().to_string())
            .or_default()
            .push(w.display_name(locale).unwrap_or("?").to_string());
    }
    for (system, kinds) in &by_system {
        println!("{system}");
        for (kind, names) in kinds {
            let mut names = names.clone();
            names.sort();
            names.dedup();
            let shown = if names.len() > 12 {
                &names[..12]
            } else {
                &names[..]
            };
            println!("  {kind:<30} {:>3}  {}", names.len(), shown.join(", "));
        }
    }

    let find = |n: &str, kind: LocationKind| -> Result<Guid, String> {
        waypoints
            .iter()
            .find(|(_, w)| w.kind == kind && w.display_name(locale) == Some(n))
            .map(|(g, _)| *g)
            .ok_or(format!("no {n}"))
    };
    let hurston = find("Hurston", LocationKind::Planet)?;
    let crusader = find("Crusader", LocationKind::Planet)?;
    let microtech = find("microTech", LocationKind::Planet)?;
    // The locale spells Ariel "Arial".
    let ariel = find("Arial", LocationKind::Moon)?;
    let gm = |a: &Guid, b: &Guid| waypoints.distance(a, b).map_or(f64::NAN, |d| d / 1e9);
    println!("Hurston - Crusader {:.3} Gm", gm(&hurston, &crusader));
    println!("Crusader - microTech {:.3} Gm", gm(&crusader, &microtech));
    for (g, w) in waypoints.iter() {
        if matches!(w.kind, LocationKind::Anomaly | LocationKind::JumpPoint)
            && w.system == waypoints.get(&microtech).and_then(|m| m.system)
        {
            println!("microTech - {} {:.3} Gm", name(g), gm(&microtech, g));
        }
    }
    // Ariel's `OrbitalRadius` is 52,658,856 m.
    let ariel_orbit = waypoints.distance(&hurston, &ariel).ok_or("Ariel")?;
    println!("Hurston - Ariel {ariel_orbit:.0} m");
    assert!((ariel_orbit - 52_658_856.0).abs() < 10.0);
    // Each system has its own frame.
    let monox = find("Monox", LocationKind::Planet)?;
    assert_eq!(waypoints.distance(&hurston, &monox), None);
    let h = waypoints.get(&hurston).ok_or("Hurston")?;
    assert_eq!(h.quantum.arrival_radius, 1_050_000.0);
    assert!(waypoints.travel(&crusader, &hurston) < waypoints.distance(&crusader, &hurston));
    Ok(())
}
