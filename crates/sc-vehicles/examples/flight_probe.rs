//! Re-proves the flight figures against spviewer (LIVE 4.10.197) on the
//! stock RSI Constellation Mk V Centaurus: mass 1,630,397 kg (hull
//! 1,240,000 + loadout 371,180 + the docked Skylark's 14,868.5 hull, its
//! panels included, and 4,349 loadout), SCM 170 / boost 365 /
//! NAV 1075, pitch / roll / yaw 22 / 35 / 18 (boosted 33 / 70 / 27), and
//! the accelerations spviewer rounds to forward 4.9, back 2.0, strafe 2.5
//! G (boosted forward 7.4).
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example flight_probe
//! ```

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::ShipComponents;
use sc_vehicles::{Flight, PortContainers, Vehicles, port_axes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let components = ShipComponents::build(&datacore, &items);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let (_, ship) = vehicles
        .iter()
        .find(|(g, _)| names.get(g).map(String::as_str) == Some("RSI_Constellation_Mk5_Centaurus"))
        .ok_or("no Centaurus")?;

    let axes = port_axes(&assets, ship);
    println!("{} hull helpers", axes.len());
    let flight = Flight::compute(ship, &ship.stock, &vehicles, &items, &components, &axes)
        .ok_or("no flight controller")?;
    println!("{flight:#?}");

    // Mass split: own items, docked hulls, docked items.
    let (mut own, mut docked_hull, mut docked_items) = (0.0f64, 0.0f64, 0.0f64);
    let mut stack: Vec<(&sc_vehicles::Mounted, bool)> =
        ship.stock.iter().map(|m| (m, false)).collect();
    while let Some((m, docked)) = stack.pop() {
        let mut child_docked = docked;
        if let Some(g) = m.item {
            if let Some(v) = vehicles.get(&g) {
                docked_hull += f64::from(v.hull_mass().unwrap_or(0.0));
                child_docked = true;
            } else {
                let kg = f64::from(items.get(&g).and_then(|i| i.mass).unwrap_or(0.0));
                if docked {
                    docked_items += kg;
                } else {
                    own += kg;
                }
            }
        }
        stack.extend(m.children.iter().map(|c| (c, child_docked)));
    }
    println!(
        "mass: hull {:?}, items {own}, docked hull {docked_hull}, docked items {docked_items}",
        ship.hull_mass()
    );
    assert_eq!(ship.hull_mass(), Some(1_240_000.0));
    assert_eq!(docked_hull, 14_868.5);

    let near = |a: f64, b: f64, tol: f64| (a - b).abs() <= tol;
    // Stage 0 summed rounded item masses: within 5 kg.
    assert!(near(flight.mass, 1_630_394.5, 5.0), "mass {}", flight.mass);
    assert_eq!(
        (
            flight.scm_speed,
            flight.boost_speed_forward,
            flight.max_speed
        ),
        (170.0, 365.0, 1075.0)
    );
    assert_eq!(flight.angular_velocity, [22.0, 35.0, 18.0]);
    assert_eq!(flight.angular_velocity_boosted, [33.0, 70.0, 27.0]);
    let a = flight.acceleration;
    for (got, want) in [
        (a.forward, 4.94),
        (a.backward, 1.98),
        (a.left, 2.48),
        (a.right, 2.48),
        (a.up, 2.01),
        (a.down, 1.98),
    ] {
        assert!(near(got, want, 0.005), "{got} vs {want}");
    }
    let b = flight.acceleration_boosted;
    for (got, want) in [
        (b.forward, 7.41),
        (b.backward, 2.38),
        (b.right, 3.34),
        (b.up, 2.61),
        (b.down, 2.38),
    ] {
        assert!(near(got, want, 0.005), "boosted {got} vs {want}");
    }
    println!("asserts passed: Centaurus mass, speeds, rotation, accelerations");
    Ok(())
}
