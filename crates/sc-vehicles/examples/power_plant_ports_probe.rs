//! Re-proves that a port's unknown sub-type does not narrow what it takes
//! (LIVE 4.10.3): the Drake Marauder's two S1 power plant ports list
//! `<Type type="PowerPlant" subtypes="$editable"/>` in its vehicle XML, a
//! flag written into the sub-type list. They take what the Anvil Arrow's
//! S1 power plant port (no sub-types listed) takes, the stock LightBlossom
//! included. Lists every vehicle port that names a sub-type the bindings do
//! not know (also `""`, and item types such as `Misc` / `CargoGrid` written
//! as sub-types) and checks that every item the game's own loadouts put on
//! such a port passes the type check.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example power_plant_ports_probe
//! ```

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use sc_extract::generated::EItemSubType;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{Loadout, Misfit, Mounted, Port, PortContainers, Vehicles, fits};

const MARAUDER: &str = "16099337-179f-473e-8bc9-545e38ef0a32";
const ARROW: &str = "eaeb562d-d4ac-43bd-a843-e8a6d69fad82";

fn unknown_sub_types(p: &Port) -> Vec<&str> {
    p.types
        .iter()
        .flat_map(|t| &t.sub_types)
        .filter_map(|s| match s {
            EItemSubType::Unrecognized(v) => Some(v.as_str()),
            _ => None,
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let names: HashMap<Guid, &str> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?)))
        .collect();
    let name = |g: &Guid| names.get(g).copied().unwrap_or("?");

    // Every port naming a sub-type the bindings do not know, and the items
    // the game itself puts there (vehicle stock trees, item default
    // loadouts): their type check under the old rule (any listed sub-type
    // narrows) and under `fits`.
    let ports_of = |g: &Guid| -> &[Port] {
        match vehicles.get(g) {
            Some(v) => &v.ports,
            None => containers.get(g).map_or(&[][..], |c| &c.ports[..]),
        }
    };
    let mut stray = 0;
    for (g, v) in vehicles.iter() {
        for p in v.ports.iter().filter(|p| !unknown_sub_types(p).is_empty()) {
            stray += 1;
            println!("vehicle {} {}: {:?}", name(g), p.name, unknown_sub_types(p));
        }
    }
    for (g, c) in containers.iter() {
        if vehicles.get(g).is_none() {
            stray += c
                .ports
                .iter()
                .filter(|p| !unknown_sub_types(p).is_empty())
                .count();
        }
    }
    println!("{stray} vehicle and item ports name an unknown sub-type");
    // (unknown values, old type check, new type check) -> attachments.
    let mut seen: HashMap<(String, bool, bool), u32> = HashMap::new();
    let mut judge = |port: &Port, item: &Guid| {
        let (Some(item), unknown) = (items.get(item), unknown_sub_types(port)) else {
            return;
        };
        if unknown.is_empty() {
            return;
        }
        let old = port.types.iter().any(|t| {
            t.item_type == item.item_type
                && (t.sub_types.is_empty()
                    || item.item_sub_type == EItemSubType::UNDEFINED
                    || t.sub_types.contains(&item.item_sub_type))
        });
        let new = fits(port, item, &[]) != Err(Misfit::Type);
        *seen.entry((format!("{unknown:?}"), old, new)).or_default() += 1;
    };
    let mut stack: Vec<(&[Port], &Mounted)> = Vec::new();
    for v in vehicles.values() {
        stack.extend(v.stock.iter().map(|m| (&v.ports[..], m)));
    }
    while let Some((ports, m)) = stack.pop() {
        let Some(item) = m.item else { continue };
        if let Some(p) = ports.iter().find(|p| p.name.eq_ignore_ascii_case(&m.port)) {
            judge(p, &item);
        }
        let below = ports_of(&item);
        stack.extend(m.children.iter().map(|c| (below, c)));
    }
    for (g, c) in containers.iter() {
        for e in &c.default_loadout {
            if let (Some(item), Some(p)) = (
                e.item,
                ports_of(g)
                    .iter()
                    .find(|p| p.name.eq_ignore_ascii_case(&e.port)),
            ) {
                judge(p, &item);
            }
        }
    }
    let mut rows: Vec<_> = seen.into_iter().collect();
    rows.sort();
    for ((unknown, old, new), n) in &rows {
        println!("  {unknown}: type ok before {old}, now {new}: {n} attachments");
    }
    // No item the game puts on such a port is refused on type any more.
    assert!(rows.iter().all(|((_, _, new), _)| *new));

    // The Arrow's S1 power plant port: no sub-types listed.
    let arrow = Guid::from_str(ARROW)?;
    let arrow_loadout = Loadout::stock(&vehicles, &arrow).ok_or("no Arrow")?;
    let arrow_port = ["hardpoint_power_plant"];
    let reference: HashSet<Guid> = arrow_loadout
        .candidates(&arrow_port, &items, &vehicles, &containers)
        .into_iter()
        .collect();
    println!(
        "{} {}: {} candidates",
        name(&arrow),
        arrow_port[0],
        reference.len()
    );
    assert!(reference.len() > 1);

    let marauder = Guid::from_str(MARAUDER)?;
    let loadout = Loadout::stock(&vehicles, &marauder).ok_or("no Marauder")?;
    for path in [
        ["hardpoint_power_plant_top"],
        ["hardpoint_power_plant_bottom"],
    ] {
        let port = vehicles
            .get(&marauder)
            .and_then(|v| v.port(path[0]))
            .ok_or("no port")?;
        assert_eq!(unknown_sub_types(port), ["$editable"], "{}", path[0]);
        let stock = loadout.at(&path).and_then(|m| m.item).ok_or("empty port")?;
        let item = items.get(&stock).ok_or("stock item is no item")?;
        let check = loadout.check(&path, item, &items, &vehicles, &containers);
        let candidates: HashSet<Guid> = loadout
            .candidates(&path, &items, &vehicles, &containers)
            .into_iter()
            .collect();
        println!(
            "{} {}: stock {} {check:?}, {} candidates",
            name(&marauder),
            path[0],
            name(&stock),
            candidates.len()
        );
        assert_eq!(check, Ok(()));
        assert!(candidates.contains(&stock));
        assert_eq!(candidates, reference);
    }
    println!("asserts passed: Marauder power plant ports take what the Arrow's takes");
    Ok(())
}
