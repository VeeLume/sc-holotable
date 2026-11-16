//! Re-proves `Travel::order` on the installed build with the stock RSI
//! Constellation Mk V Centaurus.
//!
//! A shopping run from Everus Harbor and back, two minutes per arrival:
//! part A at Area18 or Lorville, part B at Lorville or New Babbage, a drop at
//! Ruin Station in Pyro after part B. Asserts:
//! - A and B share Lorville (one arrival instead of two wins);
//! - the drop comes after B, across the jump;
//! - the plan is exact and its total adds up;
//! - an open start costs the first stop no travel.
//!
//! ```bash
//! cargo run -p sc-travel --release --example tour_plan
//! ```

use std::collections::HashMap;
use std::time::Instant;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::ShipComponents;
use sc_locations::{Bodies, JumpLinks, Locations, ObjectContainers, Universe, Waypoints};
use sc_travel::{Plan, Tour, Travel};
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
    let drive = vehicles
        .iter()
        .find(|(g, _)| names.get(g).map(String::as_str) == Some("RSI_Constellation_Mk5_Centaurus"))
        .and_then(|(_, v)| Quantum::compute(&v.stock, &vehicles, &components, &profiles))
        .ok_or("no Centaurus drive")?;

    let label = |g: &Guid| {
        locations
            .get(g)
            .and_then(|l| l.display_name(&data.locale))
            .unwrap_or("?")
            .to_string()
    };
    let place = |n: &str| {
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
    let show = |title: &str, plan: &Plan| {
        println!(
            "\n{title}: {:.0} s, {:.1} Gm, {:.3} SCU, exact {}, estimate {}",
            plan.seconds(),
            plan.metres() / 1e9,
            plan.fuel(),
            plan.exact,
            plan.is_estimate()
        );
        for v in &plan.visits {
            println!(
                "  at {:>6.0} s  stop {} {:<16} travel {:>5.0} s, overhead {:>4.0} s, dwell {:>4.0} s",
                v.at,
                v.stop.index(),
                label(&v.place),
                v.route.seconds(),
                v.overhead,
                v.dwell
            );
        }
        if let Some(f) = &plan.finish {
            println!("  finish: {:.0} s", f.seconds());
        }
    };

    let everus = place("Everus Harbor")?;
    let (area18, lorville, babbage) = (place("Area18")?, place("Lorville")?, place("New Babbage")?);
    let ruin = place("Ruin Station")?;

    let mut tour = Tour::new(everus);
    tour.overhead(120.0).end_at_start();
    let a = tour.visit_any(&[area18, lorville]).dwell(300.0).id();
    let b = tour.visit_any(&[lorville, babbage]).dwell(300.0).id();
    let drop = tour.visit(ruin).dwell(120.0).id();
    tour.require(b, drop);

    let t = Instant::now();
    let plan = travel.order(&tour, &drive, |_| true).ok_or("no plan")?;
    println!("planned in {:?}", t.elapsed());
    show("shopping run", &plan);
    let at = |id| plan.visits.iter().position(|v| v.stop == id).unwrap();
    let place_of = |id| plan.visits[at(id)].place;
    assert!(plan.exact);
    assert_eq!(place_of(a), lorville, "part A shares Lorville");
    assert_eq!(place_of(b), lorville, "part B shares Lorville");
    assert!(at(b) < at(drop), "the drop comes after part B");
    assert!(
        plan.visits[at(drop)]
            .route
            .hops
            .iter()
            .any(|h| h.kind == sc_travel::HopKind::Tunnel)
    );
    let sum: f64 = plan
        .visits
        .iter()
        .map(|v| v.route.seconds() + v.overhead + v.dwell)
        .sum::<f64>()
        + plan.finish.as_ref().map_or(0.0, |f| f.seconds());
    assert!((plan.seconds() - sum).abs() < 1e-6);

    // Open start: the first stop costs nothing to reach.
    let mut open = Tour::anywhere();
    open.overhead(120.0);
    open.visit(area18).dwell(60.0);
    open.visit(babbage).dwell(60.0);
    let plan = travel
        .order(&open, &drive, |_| true)
        .ok_or("no open plan")?;
    show("open start", &plan);
    assert!(plan.visits[0].route.hops.is_empty() && plan.visits[0].at == 0.0);
    assert_eq!(plan.visits[0].overhead, 0.0);
    Ok(())
}
