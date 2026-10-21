//! Re-proves the flight figures against spviewer (LIVE 4.10.197) on the
//! stock RSI Constellation Mk V Centaurus: mass 1,630,397 kg (hull
//! 1,240,000 + loadout 371,180 + the docked Skylark's 14,868.5 hull, its
//! panels included, and 4,349 loadout), SCM 170 / boost 365 /
//! NAV 1075, pitch / roll / yaw 22 / 35 / 18 (boosted 33 / 70 / 27), and
//! the accelerations spviewer rounds to forward 4.9, back 2.0, strafe 2.5
//! G (boosted forward 7.4); and its defences: hull 170,500, armour 25,740
//! HP at 0.7 / 0.5, 2 x 5CA = 200,000 HP and 38,000/s, quadrant faces;
//! hydrogen 0.5925 SCU/min on the mains from 46.8 SCU; the Khaos at 0.022
//! SCU per GM, range 390.9 GM (spviewer), 31.92 GM in 170.6 s (the client's
//! envelope; spviewer's older model shows 2:46).
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example flight_probe
//! ```

use std::collections::HashMap;

use sc_extract::generated::EFaceType;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::ShipComponents;
use sc_vehicles::{
    Defense, Flight, PortContainers, Quantum, ResourceProfiles, Vehicles, port_axes,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
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
    // Defences: hull, armour, 2 x 5CA combined, quadrant faces.
    let d = Defense::compute(ship, &ship.stock, &vehicles, &components);
    println!("{d:#?}");
    assert_eq!(d.hull_health, 170_500.0);
    assert_eq!(d.armor_health, Some(25_740.0));
    let m = d
        .armor
        .as_ref()
        .and_then(|a| a.damage_multiplier)
        .ok_or("armour")?;
    assert_eq!((m.physical, m.energy), (0.7, 0.5));
    let s = d.shield.as_ref().ok_or("shield")?;
    assert_eq!(
        (s.generators, s.max_health, s.max_regen),
        (2, 200_000.0, 38_000.0)
    );
    assert_eq!((s.damaged_regen_delay, s.downed_regen_delay), (6.05, 12.1));
    assert_eq!(
        d.emitter.as_ref().map(|e| &e.face_type),
        Some(&EFaceType::Quadrant)
    );
    // Hydrogen: 4 mains of 19.75 MN at 1.25e-6 SCU / 10 kN / s.
    let h = flight.hydrogen_burn;
    println!("hydrogen {h:?}, tanks {} SCU", flight.hydrogen_capacity);
    assert!(near(h.main, 0.5925, 0.0005) && near(h.retro, 0.2378, 0.0005));
    assert!(near(flight.hydrogen_capacity, 46.8, 1e-4));

    // Quantum: Khaos, 22 micro-SCU per Mm, 8.6 SCU tank.
    let q = Quantum::compute(&ship.stock, &vehicles, &components, &profiles).ok_or("no QD")?;
    println!(
        "quantum: {:.6} SCU/Mm, {} SCU, range {:.2} GM, 31.92 GM in {:.2} s",
        q.fuel_per_mm,
        q.fuel_capacity,
        q.range() / 1e9,
        q.time(31.92e9)
    );
    assert!(near(q.fuel_per_mm, 22e-6, 1e-12) && near(q.fuel_capacity, 8.6, 1e-4));
    assert!(near(q.range() / 1e9, 390.909, 0.001));
    assert!(near(q.time(31.92e9), 170.58, 0.01));

    println!(
        "asserts passed: Centaurus mass, speeds, rotation, accelerations, defences, fuel, quantum"
    );
    Ok(())
}
