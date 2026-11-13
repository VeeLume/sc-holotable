//! Every stock vehicle's power pools, checked against the pool rules in
//! `PowerNetwork`:
//!
//! - fixed pools whose members ask a fractional total below the pool size,
//!   where a full target exceeds what the members ask (each member is then
//!   capped at its request; before v0.48 `split_pools` over-granted them);
//! - dynamic pools with targets that fall between sums of member minimums:
//!   the client lets a player set them (it does not snap), the pool takes
//!   them, and the extra segments power nothing until the next minimum fits;
//! - power-on in SCM never leaves negative free supply.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example power_pool_sweep
//! ```

use std::collections::HashMap;

use sc_extract::generated::EMasterMode;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{
    NetworkParams, PoolSize, PortContainers, PowerNetwork, ResourceProfiles, Vehicles,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let profiles = ResourceProfiles::build(&datacore);
    let params = NetworkParams::build(&datacore).ok_or("no ItemResourceNetworkGlobal")?;
    // Record names for the printout only.
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| {
            let n = r.name()?;
            Some((
                r.id(),
                n.strip_prefix("EntityClassDefinition.")
                    .unwrap_or(n)
                    .to_string(),
            ))
        })
        .collect();

    let mode = EMasterMode::SCM;
    let (mut ships, mut fixed_over, mut between) = (0, 0, 0);
    let mut sorted: Vec<_> = vehicles.iter().collect();
    sorted.sort_by_key(|(g, _)| names.get(g).cloned().unwrap_or_default());
    for (guid, ship) in sorted {
        let network = PowerNetwork::build(
            &ship.stock,
            containers.get(guid),
            &vehicles,
            &items,
            &profiles,
            &params,
        );
        if network.pools.is_empty() {
            continue;
        }
        ships += 1;
        let name = names.get(guid).map_or("?", String::as_str);
        let a = network.power_on(&mode);
        assert!(
            network.free_supply(&a, &mode) >= 0,
            "{name}: power-on overspends"
        );
        for (p, pool) in network.pools.iter().enumerate() {
            let asked: f64 = pool
                .members
                .iter()
                .filter(|&&c| network.consumers[c].on)
                .map(|&c| network.consumers[c].request as f64 / 1e6)
                .sum();
            let size = network.pool_size(p);
            let label = pool.item_type.as_dcb_str();
            match pool.size {
                PoolSize::Fixed(_) => {
                    if asked.fract() > 1e-9 && asked < size as f64 {
                        fixed_over += 1;
                        println!(
                            "{name:<44} {label:<20} fixed: asks {asked:.3} of {size}, a full target gives {:.3}x",
                            size as f64 / asked
                        );
                    }
                }
                _ => {
                    let mut probe = a.clone();
                    let dead: Vec<i64> = (1..=size)
                        .filter(|&t| {
                            probe.pool_targets[p] = t;
                            network.split_pools(&mut probe);
                            let got: f64 = pool.members.iter().map(|&c| probe.granted[c]).sum();
                            got + 1e-9 < t as f64
                        })
                        .collect();
                    if !dead.is_empty() {
                        between += 1;
                        println!(
                            "{name:<44} {label:<20} dynamic {size}: targets {dead:?} grant less than the target"
                        );
                    }
                }
            }
        }
    }
    println!(
        "\n{ships} vehicles with pools; {fixed_over} fixed pools ask a fractional total below their size; {between} dynamic pools have targets that grant less than they take"
    );
    Ok(())
}
