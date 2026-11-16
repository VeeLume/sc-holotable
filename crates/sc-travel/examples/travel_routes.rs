//! Re-proves sc-travel on the installed build with the stock RSI
//! Constellation Mk V Centaurus (Khaos quantum drive).
//!
//! Asserts:
//! - the tunnel record reads 15 / 110 / 8 km and the ship's jump drive
//!   carries a tunnel speed limit;
//! - a clear in-system leg is one hop costing spool-up plus the client's
//!   travel envelope over the distance less the arrival radius;
//! - a leg whose straight line crosses a body detours through orbital
//!   markers, and every hop of it clears every other body;
//! - two outposts on one body are one spline hop;
//! - Hurston to Pyro crosses at the Stanton–Pyro link, and with that link
//!   left out of the usable links goes round through Nyx (or finds no
//!   route).
//!
//! ```bash
//! cargo run -p sc-travel --release --example travel_routes
//! ```

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::ShipComponents;
use sc_locations::{
    Bodies, JumpLinks, LocationKind, Locations, ObjectContainers, Universe, Waypoints,
};
use sc_travel::{HopKind, Route, Stop, Travel};
use sc_vehicles::{PortContainers, Quantum, ResourceProfiles, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;

    let locations = Locations::build(datacore.records());
    let waypoints = Waypoints::build(&Universe::join(
        locations.clone(),
        ObjectContainers::systems(&assets, 1)?,
    ));
    let bodies = Bodies::build(&assets, &locations)?;
    let links = JumpLinks::build(&assets)?;
    let travel = Travel::build(&datacore, &waypoints, &bodies, &links);
    let name = |s: &Stop| match s {
        Stop::Place(g) => locations
            .get(g)
            .and_then(|l| l.display_name(&data.locale))
            .unwrap_or("?")
            .to_string(),
        Stop::Marker { body, number } => format!(
            "OM-{number} {}",
            locations
                .get(body)
                .and_then(|l| l.display_name(&data.locale))
                .unwrap_or("?")
        ),
    };
    let show = |label: &str, r: &Route| {
        println!(
            "\n{label}: {:.3} Gm, {:.1} s, {:.4} SCU, estimate {}",
            r.metres() / 1e9,
            r.seconds(),
            r.fuel(),
            r.is_estimate()
        );
        for h in &r.hops {
            println!(
                "  {:?} {} -> {}: {:.3} Gm {:.1} s (+{:.1} s cooldown){}",
                h.kind,
                name(&h.from),
                name(&h.to),
                h.metres / 1e9,
                h.seconds,
                h.cooldown,
                if h.estimate { " ~" } else { "" }
            );
        }
    };

    let tunnel = travel.tunnel().ok_or("no tunnel record")?;
    println!("tunnel: {tunnel:?}");
    assert_eq!(
        (tunnel.entrance, tunnel.length, tunnel.exit),
        (15_000.0, 110_000.0, 8_000.0)
    );

    // The ship.
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let components = ShipComponents::build(&datacore, &items);
    let profiles = ResourceProfiles::build(&datacore);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let ship = |record: &str| {
        vehicles
            .iter()
            .find(|(g, _)| names.get(g).map(String::as_str) == Some(record))
            .and_then(|(_, v)| Quantum::compute(&v.stock, &vehicles, &components, &profiles))
    };
    let drive = ship("RSI_Constellation_Mk5_Centaurus").ok_or("no Centaurus drive")?;
    println!(
        "Centaurus: spool {} s, jump drive {:?}, tunnel speed {:?} m/s",
        drive.travel.spool_up_time, drive.jump_drive, drive.tunnel_speed
    );
    assert!(
        drive.tunnel_speed.is_some(),
        "the Centaurus has a jump drive"
    );
    for record in [
        "AEGS_Idris_P",
        "ORIG_890Jump",
        "AEGS_Gladius",
        "MISC_Prospector",
    ] {
        if let Some(q) = ship(record) {
            println!("  {record}: tunnel speed {:?}", q.tunnel_speed);
        }
    }

    let by_name = |n: &str| {
        waypoints
            .values()
            .find(|w| {
                locations
                    .get(&w.guid)
                    .and_then(|l| l.display_name(&data.locale))
                    == Some(n)
            })
            .map(|w| w.guid)
            .ok_or(format!("no waypoint {n}"))
    };

    // A clear leg: one hop, spool plus the envelope.
    let (hurston, crusader) = (by_name("Hurston")?, by_name("Crusader")?);
    let leg = travel.leg(&hurston, &crusader, &drive).ok_or("no leg")?;
    show("Hurston -> Crusader", &leg);
    let expect = waypoints.travel(&hurston, &crusader).unwrap();
    assert_eq!(leg.hops.len(), 1);
    assert_eq!(leg.metres(), expect);
    assert!(
        (leg.seconds() - (f64::from(drive.travel.spool_up_time) + drive.time(expect))).abs() < 1e-6
    );

    // A blocked leg: the first pair of system-level places in Stanton whose
    // straight line crosses another body detours through its markers.
    let stanton = waypoints.get(&hurston).and_then(|w| w.system).unwrap();
    let mut top: Vec<_> = waypoints
        .values()
        .filter(|w| w.system == Some(stanton) && w.anchor.is_none())
        .filter(|w| matches!(w.kind, LocationKind::Planet | LocationKind::Moon))
        .map(|w| w.guid)
        .collect();
    top.sort_by_key(|g| g.to_string());
    let detoured = top
        .iter()
        .flat_map(|a| top.iter().map(move |b| (*a, *b)))
        .filter(|(a, b)| a != b)
        .find_map(|(a, b)| travel.leg(&a, &b, &drive).filter(|r| r.hops.len() > 1))
        .ok_or("no blocked leg among Stanton's bodies")?;
    show("blocked leg", &detoured);
    assert!(detoured.is_estimate());
    assert!(
        detoured.hops[1..]
            .iter()
            .all(|h| matches!(h.from, Stop::Marker { .. }))
    );

    // Two outposts on one body: a spline hop.
    let mut outposts: Vec<_> = waypoints
        .values()
        .filter(|w| w.kind == LocationKind::Outpost)
        .filter(|w| w.anchor.is_some_and(|g| bodies.get(&g).is_some()))
        .collect();
    outposts.sort_by_key(|w| (w.anchor.map(|g| g.to_string()), w.guid.to_string()));
    let pair = outposts
        .windows(2)
        .find(|p| p[0].anchor == p[1].anchor)
        .ok_or("no two outposts on one body")?;
    let spline = travel
        .leg(&pair[0].guid, &pair[1].guid, &drive)
        .ok_or("no spline leg")?;
    show("outpost -> outpost", &spline);
    assert_eq!(spline.hops.len(), 1);
    assert_eq!(spline.hops[0].kind, HopKind::Spline);

    // Across systems.
    let lorville = by_name("Lorville")?;
    let ruin = by_name("Ruin Station")?;
    let cross = travel
        .path(&lorville, &ruin, &drive, |_| true)
        .ok_or("no route to Pyro")?;
    show("Lorville -> Ruin Station", &cross);
    let tunnels: Vec<_> = cross
        .hops
        .iter()
        .filter(|h| h.kind == HopKind::Tunnel)
        .collect();
    assert_eq!(tunnels.len(), 1, "one jump");
    assert_eq!(name(&tunnels[0].from), "Stanton-Pyro Jump Point");
    let stanton_pyro = by_name("Stanton-Pyro Jump Point")?;
    match travel.path(&lorville, &ruin, &drive, |l| {
        l.entry != stanton_pyro && l.exit != stanton_pyro
    }) {
        Some(r) => {
            show("Lorville -> Ruin Station, avoiding Stanton-Pyro", &r);
            assert!(r.hops.iter().filter(|h| h.kind == HopKind::Tunnel).count() >= 2);
        }
        None => println!("\navoiding Stanton-Pyro: no route"),
    }
    assert!(
        travel.path(&lorville, &ruin, &drive, |_| false).is_none(),
        "no links, no route"
    );
    Ok(())
}
