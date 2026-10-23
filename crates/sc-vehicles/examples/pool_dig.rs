//! A ship's weapons pool, member by member: the Online Power request and
//! minimum of each gun, the running capacity truncated after each member
//! (what caps a dynamic pool) against the total truncated once, and what
//! `split_pools` grants for pool targets 2 to 4, which must be exactly the
//! target. The F7C-M Super Hornet Mk II's pool is fixed at 4 with two
//! gatlings asking 0.1 each: the running capacity does not cap it.
//! `--guns "A,B"` prints the Online Power request of guns by display name
//! (Ardor-3 and Attrition-3 0.75, M6A 1.6: Valerie's six ask 6.2, whose
//! running capacity would be 2 although the game allows 4).
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example pool_dig -- ANVL_Hornet_F7CM_Mk2
//! ```

use std::collections::HashMap;

use sc_extract::generated::{EItemType, EMasterMode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{NetworkParams, PortContainers, PowerNetwork, ResourceProfiles, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wanted = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "ANVL_Hornet_F7CM_Mk2".into());
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let profiles = ResourceProfiles::build(&datacore);
    let params = NetworkParams::build(&datacore).ok_or("no ItemResourceNetworkGlobal")?;
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
    let candidates: Vec<&String> = names
        .values()
        .filter(|n| {
            n.to_ascii_lowercase()
                .contains(&wanted.to_ascii_lowercase())
        })
        .collect();
    println!("matching records: {candidates:?}");
    let (guid, ship) = vehicles
        .iter()
        .find(|(g, _)| {
            names
                .get(g)
                .is_some_and(|n| n.eq_ignore_ascii_case(&wanted))
        })
        .ok_or("no such vehicle")?;
    let network = PowerNetwork::build(
        &ship.stock,
        containers.get(guid),
        &vehicles,
        &items,
        &profiles,
        &params,
    );
    println!("supply {}", network.supply());
    let p = network
        .pools
        .iter()
        .position(|p| p.item_type == EItemType::WeaponGun)
        .ok_or("no weapons pool")?;
    let pool = &network.pools[p];
    println!(
        "pool {:?}, size {}, minimum {}",
        pool.size,
        network.pool_size(p),
        network.pool_minimum(p)
    );
    let (mut running, mut total) = (0.0f64, 0.0f64);
    for &c in &pool.members {
        let m = &network.consumers[c];
        let request = m.request as f64 / 1e6;
        running = (running + request).trunc();
        total += request;
        println!(
            "  {:<40} request {request:.4} minimum {:.4} -> capacity {running}",
            names.get(&m.item).map_or("?", String::as_str),
            m.minimum as f64 / 1e6,
        );
    }
    println!(
        "capacity per member {running}, total {total:.4} -> trunc {}",
        total.trunc()
    );
    // `--guns "A,B"`: the Online Power request of guns by display name.
    if let Some(list) = std::env::args().skip_while(|a| a != "--guns").nth(1) {
        for want in list.split(',') {
            for (g, item) in items.iter() {
                let display = item
                    .name_key
                    .as_ref()
                    .and_then(|k| data.locale.resolve(k.as_str()))
                    .unwrap_or("");
                if !display
                    .to_ascii_lowercase()
                    .contains(&want.trim().to_ascii_lowercase())
                {
                    continue;
                }
                let request = profiles
                    .get(g)
                    .and_then(|p| p.online())
                    .and_then(|o| {
                        o.consumption(&sc_extract::generated::ResourceNetworkResource::Power)
                    })
                    .and_then(|d| d.consumes.as_ref())
                    .map(|a| a.per_second.standard());
                println!(
                    "  {display:<20} {:<36} S{} power request {request:?}",
                    names.get(g).map_or("?", String::as_str),
                    item.size
                );
            }
        }
    }
    let mut a = network.power_on(&EMasterMode::SCM);
    println!("power-on target {}", a.pool_targets[p]);
    for t in 2..=4 {
        a.pool_targets[p] = t;
        network.split_pools(&mut a);
        let granted: f64 = pool.members.iter().map(|&c| a.granted[c]).sum();
        println!("target {t}: granted {granted:.4}");
        assert!((granted - f64::from(t as i32)).abs() < 1e-9, "target {t}");
    }
    Ok(())
}
