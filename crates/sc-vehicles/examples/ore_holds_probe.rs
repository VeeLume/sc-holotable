//! Re-proves [`Storage::holds`] and [`Storage::capacity_for`] on the stock
//! loadouts of the mining and salvage ships.
//!
//! Asserts the ore holds players know: Prospector 32 SCU (4 pods of 8), MOLE
//! 96 (8 of 12), Golem 32 (2 of 16), and that a fuel tank is no hold.
//! Prints the ROCs and the salvage holds (Reclaimer, Vulture) with what
//! they take.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example ore_holds_probe
//! ```

use std::collections::HashMap;

use sc_extract::generated::EItemType;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_resources::Resources;
use sc_vehicles::{PortContainers, Storage, Vehicles};

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
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let n = |g: &Guid| names.get(g).cloned().unwrap_or_else(|| g.to_string());
    let storage = |record: &str| {
        let (_, v) = vehicles
            .iter()
            .find(|(g, _)| names.get(g).map(String::as_str) == Some(record))?;
        Some(Storage::compute(
            v,
            &v.stock,
            &[],
            &vehicles,
            &items,
            &containers,
        ))
    };

    // An ore the Prospector's pods take through a group, not by name.
    let prospector = storage("MISC_Prospector").ok_or("no Prospector")?;
    let pod = &prospector
        .holds
        .iter()
        .find(|h| h.scu() > 0.0)
        .ok_or("no pod")?
        .container;
    let ore = resources
        .values()
        .filter(|r| !pod.resources.contains(&r.guid) && pod.accepts(r))
        .min_by_key(|r| n(&r.guid))
        .ok_or("no ore the pods take by group")?;
    let taken = resources.values().filter(|r| pod.accepts(r)).count();
    println!("ore: {} (the pods take {taken} resources)", n(&ore.guid));

    for (ship, expect) in [
        ("MISC_Prospector", Some(32.0)),
        ("ARGO_MOLE", Some(96.0)),
        ("DRAK_Golem", Some(32.0)),
        ("GRIN_ROC", None),
        ("GRIN_ROC_DS", None),
        ("AEGS_Reclaimer", None),
        ("DRAK_Vulture", None),
    ] {
        let Some(s) = storage(ship) else {
            println!("{ship}: not a vehicle");
            continue;
        };
        let ore_scu = s.capacity_for(ore);
        println!(
            "\n{ship}: {} holds, {ore_scu} SCU of {}",
            s.holds.len(),
            n(&ore.guid)
        );
        for h in s.holds.iter().filter(|h| h.scu() > 0.0) {
            let takes: Vec<String> = h
                .container
                .groups
                .iter()
                .chain(&h.container.resources)
                .map(n)
                .collect();
            println!("  {:<44} {:>6} SCU  takes {takes:?}", n(&h.item), h.scu());
        }
        assert!(
            s.holds.iter().all(|h| !matches!(
                items.get(&h.item).map(|i| &i.item_type),
                Some(
                    EItemType::FuelTank | EItemType::QuantumFuelTank | EItemType::ExternalFuelTank
                )
            )),
            "{ship}: a fuel tank counted as a hold"
        );
        if let Some(e) = expect {
            assert_eq!(ore_scu, e, "{ship}");
        }
    }
    Ok(())
}
