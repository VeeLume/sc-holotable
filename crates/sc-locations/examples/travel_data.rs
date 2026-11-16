//! Re-proves the travel data on the installed build: [`Bodies`],
//! [`JumpLinks`] and the waypoints' anchor frames.
//!
//! Asserts:
//! - the orbital-marker radius (`sqrt(2) * obstruction + 10 km`) reproduces
//!   Murphy's Navigation Tool (Stanton 4.0, Pyro 4.2 tables) to 50 m on the
//!   bodies whose obstruction has not changed since;
//! - every live body turns about `0,0,1` and has a radius and an atmosphere;
//! - Stanton–Pyro and Pyro–Nyx link both ways;
//! - two outposts on one body are exactly as far apart as their authored
//!   offsets say, and a station on a body is anchored to it.
//!
//! ```bash
//! cargo run -p sc-locations --release --example travel_data
//! ```

use std::time::Instant;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_locations::{
    Bodies, JumpLinks, LocationKind, Locations, ObjectContainers, Universe, Waypoints,
};

/// Murphy's Navigation Tool v2.742, OM radius in m, by body container id.
/// Arial, Aberdeen and Magda are left out: their obstruction rose after the
/// 4.0 table.
const MURPHY_OM: &[(&str, f64)] = &[
    ("stanton1", 1_436_814.0),
    ("stanton1d", 475_276.7),
    ("stanton2", 10_673_165.7),
    ("stanton2a", 383_352.9),
    ("stanton2b", 432_850.7),
    ("stanton2c", 458_306.1),
    ("stanton3", 1_151_455.6),
    ("stanton3a", 331_027.3),
    ("stanton3b", 415_879.9),
    ("stanton4", 1_439_593.4),
    ("stanton4a", 355_068.7),
    ("stanton4b", 492_487.6),
    ("stanton4c", 316_884.2),
    ("pyro1", 603_546.0),
    ("pyro2", 770_140.0),
    ("pyro3", 606_657.0),
    ("pyro4", 475_842.0),
    ("pyro5", 11_161_073.5),
    ("pyro5a", 571_683.3),
    ("pyro5b", 532_988.9),
    ("pyro5c", 625_183.3),
    ("pyro5d", 570_368.0),
    ("pyro5e", 674_213.1),
    ("pyro5f", 656_054.5),
    ("pyro6", 424_365.0),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let locations = Locations::build(datacore.records());
    let name = |g: &Guid| {
        locations
            .get(g)
            .and_then(|l| l.display_name(&data.locale))
            .unwrap_or("?")
            .to_string()
    };

    let t = Instant::now();
    let bodies = Bodies::build(&assets, &locations)?;
    println!("\nbodies: {} in {:?}", bodies.len(), t.elapsed());
    let mut sorted: Vec<_> = bodies.values().collect();
    sorted.sort_by(|a, b| a.short_id().cmp(b.short_id()));
    let mut checked = 0;
    for b in &sorted {
        let om = b.om_radius();
        let murphy = MURPHY_OM
            .iter()
            .find(|(id, _)| *id == b.short_id())
            .map(|m| m.1);
        println!(
            "  {:<10} {:<12} r={:>10.0} day={:>5.2} h atmo={:>6} axis={:?} OM={:>11.0} murphy={}",
            b.short_id(),
            name(&b.guid),
            b.radius,
            b.rotation_period_h,
            b.atmosphere_height
                .map_or("-".into(), |a| format!("{a:.0}")),
            b.axis,
            om.unwrap_or(f64::NAN),
            murphy.map_or("-".into(), |m| format!("{m:.0}")),
        );
        assert_eq!(
            b.axis,
            [0.0, 0.0, 1.0],
            "{} turns about another axis",
            b.short_id()
        );
        assert!(
            b.radius > 0.0 && b.atmosphere_height.is_some(),
            "{}",
            b.short_id()
        );
        if let (Some(m), Some(om)) = (murphy, om) {
            assert!(
                (om - m).abs() < 50.0,
                "{}: OM {om} vs Murphy {m}",
                b.short_id()
            );
            checked += 1;
        }
    }
    assert_eq!(checked, MURPHY_OM.len(), "a reference body is missing");

    let t = Instant::now();
    let links = JumpLinks::build(&assets)?;
    println!("\njump links: {} in {:?}", links.len(), t.elapsed());
    for l in links.values() {
        let back = links.get(&l.exit).map(|r| r.exit == l.entry);
        println!(
            "  {} -> {}  (links back: {back:?})",
            name(&l.entry),
            name(&l.exit)
        );
        assert_eq!(back, Some(true), "{} is one-way", name(&l.entry));
    }
    assert!(links.len() >= 4, "Stanton–Pyro and Pyro–Nyx both ways");

    let universe = Universe::join(locations.clone(), ObjectContainers::systems(&assets, 1)?);
    let waypoints = Waypoints::build(&universe);
    let anchored = waypoints.values().filter(|w| w.anchor.is_some()).count();
    println!("\nwaypoints: {} ({anchored} anchored)", waypoints.len());
    // Two outposts on one body: the distance is the authored offsets'.
    let (a, b) = {
        let mut on_body: Vec<_> = waypoints
            .values()
            .filter(|w| w.kind == LocationKind::Outpost)
            .filter(|w| w.anchor.is_some_and(|g| bodies.get(&g).is_some()))
            .collect();
        on_body.sort_by_key(|w| (w.anchor.map(|g| g.to_string()), w.guid.to_string()));
        let pair = on_body
            .windows(2)
            .find(|p| p[0].anchor == p[1].anchor)
            .expect("two outposts on one body");
        (pair[0], pair[1])
    };
    let (p, q) = (a.local.unwrap(), b.local.unwrap());
    let exact = ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt();
    let d = waypoints.distance(&a.guid, &b.guid).unwrap();
    println!(
        "  {} - {} on {}: {d:.0} m (offsets {exact:.0} m)",
        name(&a.guid),
        name(&b.guid),
        name(&a.anchor.unwrap())
    );
    assert_eq!(d, exact);
    // A station placed in a body container is anchored to that body.
    let station = waypoints
        .values()
        .find(|w| {
            matches!(w.kind, LocationKind::Manmade)
                && w.anchor.is_some_and(|g| bodies.get(&g).is_some())
        })
        .expect("a station on a body");
    println!(
        "  {} sits on {}, {:.0} m from its centre",
        name(&station.guid),
        name(&station.anchor.unwrap()),
        waypoints
            .distance(&station.guid, &station.anchor.unwrap())
            .unwrap()
    );
    Ok(())
}
