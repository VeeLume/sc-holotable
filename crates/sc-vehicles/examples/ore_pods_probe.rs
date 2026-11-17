//! Re-proves ore-pod swaps on the mining ships: each stock pod port's
//! [`Loadout::candidates`] that hold ore, their capacity and whether a
//! blueprint crafts them, and [`Storage::capacity_for`] on a loadout
//! [`Loadout::equip`]ped with the largest pod in every port.
//!
//! Asserts (LIVE 4.10): the Prospector's ports take the Argo pod (12 SCU)
//! and the crafted MISC Enhanced Ore Pod (15 SCU), 60 SCU with four of
//! those; the MOLE's take the Enhanced pod too, 120 SCU; the Golem's take
//! the crafted GOLEM MC-4 Ore Pod (20 SCU), 40 SCU. Candidates also list
//! template and stowed (0 SCU) pods; this probe shows only named pods that
//! hold something, as a consumer would.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example ore_pods_probe
//! ```

use std::collections::HashMap;

use sc_crafting::Blueprints;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_resources::Resources;
use sc_vehicles::{Loadout, Place, PortContainers, Storage, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let resources = Resources::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let blueprints = Blueprints::build(&datacore, &items);
    let records: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let shown = |g: &Guid| items.get(g).and_then(|i| i.display_name(&data.locale));

    // An ore the stock Prospector pod takes through a group.
    let prospector = vehicles
        .iter()
        .find(|(g, _)| records.get(g).map(String::as_str) == Some("MISC_Prospector"))
        .ok_or("no Prospector")?;
    let stock_pod = prospector
        .1
        .stock
        .iter()
        .filter_map(|m| m.item)
        .find_map(|g| {
            items
                .get(&g)?
                .resource_container
                .as_ref()
                .filter(|r| r.scu() > Some(0.0))
        })
        .ok_or("no pod")?;
    let ore = resources
        .values()
        .filter(|r| !stock_pod.resources.contains(&r.guid) && stock_pod.accepts(r))
        .min_by_key(|r| r.guid.to_string())
        .ok_or("no ore")?;
    let scu = |g: &Guid| {
        items
            .get(g)
            .and_then(|i| i.resource_container.as_ref())
            .filter(|rc| rc.accepts(ore))
            .and_then(|rc| rc.scu())
    };

    for (ship, stock_scu, best_scu, must_take) in [
        (
            "MISC_Prospector",
            32.0,
            60.0,
            &["Argo Ore Pod", "MISC Enhanced Ore Pod"][..],
        ),
        ("ARGO_MOLE", 96.0, 120.0, &["MISC Enhanced Ore Pod"][..]),
        ("DRAK_Golem", 32.0, 40.0, &["GOLEM MC-4 Ore Pod"][..]),
    ] {
        let (guid, v) = vehicles
            .iter()
            .find(|(g, _)| records.get(g).map(String::as_str) == Some(ship))
            .ok_or(ship)?;
        let stock = Loadout::stock(&vehicles, guid).ok_or("no loadout")?;
        let storage =
            |l: &Loadout| Storage::compute(v, &l.mounted, &[], &vehicles, &items, &containers);
        let held = storage(&stock);
        println!("\n{ship}: stock {} SCU", held.capacity_for(ore));
        assert_eq!(held.capacity_for(ore), stock_scu, "{ship} stock");

        let mut best = stock.clone();
        for hold in held.holds.iter().filter(|h| h.container.accepts(ore)) {
            let Place::Port(path) = &hold.place else {
                continue;
            };
            let path: Vec<&str> = path.split('/').collect();
            let options: Vec<(Guid, f64)> = stock
                .candidates(&path, &items, &vehicles, &containers)
                .into_iter()
                .filter(|g| shown(g).is_some())
                .filter_map(|g| Some((g, scu(&g).filter(|s| *s > 0.0)?)))
                .collect();
            if hold.scu() == 0.0 {
                continue; // a stowed pod
            }
            let names: Vec<String> = options
                .iter()
                .map(|(g, s)| {
                    let crafted = if blueprints.for_crafted_entity(g).is_some() {
                        ", crafted"
                    } else {
                        ""
                    };
                    format!("{} {s} SCU{crafted}", shown(g).unwrap_or("?"))
                })
                .collect();
            println!("  {:<26} takes {names:?}", path.join("/"));
            for name in must_take {
                assert!(
                    options.iter().any(|(g, _)| shown(g) == Some(name)),
                    "{ship} {path:?} does not take {name}"
                );
            }
            let (largest, _) = options
                .iter()
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .ok_or("no option")?;
            best.equip(&path, Some(*largest), &vehicles, &containers);
        }
        let best_held = storage(&best).capacity_for(ore);
        println!("  largest pod in every port: {best_held} SCU");
        assert_eq!(best_held, best_scu, "{ship} largest pods");
    }
    Ok(())
}
