//! Re-proves the ship systems inputs against spviewer (LIVE 4.10.197) on
//! the stock RSI Constellation Mk V Centaurus: two Sedulity power plants
//! (19 segments each) share 24, the coolers convert 4 segments into 41
//! coolant, the nominal EM per component (plant 9,410, cooler 1,980,
//! shield 1,800, radar 3,200, life support 625), and the ship's power pools
//! (weapons fixed at 4, shields dynamic for 2 generators). Then the
//! client's power-on distribution in SCM and Navigation: coolers 4 each,
//! flight controller 3 of 6, shields 2 + 2 (off in Navigation), guns 2 of
//! 12.2, the quantum drive off in SCM, the rest to radar and life support.
//! The engineering screen's cooling bar at power-on in SCM: 33.65 coolant
//! asked of 82 made (41 %). Prints each mounted item's online deltas, both
//! allocations and their readouts.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example ship_systems_probe
//! ```

use std::collections::HashMap;

use sc_extract::generated::{EItemType, EMasterMode, ResourceNetworkResource};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{
    NetworkParams, PoolSize, PortContainers, PowerNetwork, ResourceProfiles, ResourceUnits,
    Vehicles, network_items, plant_output,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let profiles = ResourceProfiles::build(&datacore);
    let network = NetworkParams::build(&datacore).ok_or("no ItemResourceNetworkGlobal")?;
    println!("{} resource profiles; network {network:?}", profiles.len());
    // Signature functionality curves (none on LIVE 4.10 ship items, so the
    // readout maps the ratio linearly).
    let curved: Vec<&Guid> = profiles
        .iter()
        .filter(|(_, p)| {
            p.states.iter().any(|s| {
                [s.em.as_ref(), s.ir.as_ref()]
                    .into_iter()
                    .flatten()
                    .any(|e| !e.curve.is_empty())
            })
        })
        .map(|(g, _)| g)
        .collect();
    println!("{} profiles with a signature curve", curved.len());
    assert!(
        curved.is_empty(),
        "signature curves appeared: the readout needs them"
    );
    assert_eq!(network.plant_modifiers, [0, 1, 2, 3, 4]);
    assert!(network.distribution.pools_fill_min);
    assert_eq!(network.distribution.weapons, 0.5);

    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), short(r.name()?))))
        .collect();
    let (guid, ship) = vehicles
        .iter()
        .find(|(g, _)| names.get(g).map(String::as_str) == Some("RSI_Constellation_Mk5_Centaurus"))
        .ok_or("no Centaurus")?;

    let pools = &containers.get(guid).ok_or("no container")?.power_pools;
    println!("\npools: {pools:?}");
    assert!(
        pools
            .iter()
            .any(|p| p.item_type == EItemType::WeaponGun && p.size == PoolSize::Fixed(4))
    );
    assert!(
        pools.iter().any(|p| p.item_type == EItemType::Shield
            && p.size == PoolSize::Dynamic { max_items: Some(2) })
    );

    // The docked Skylark (with its own plant) runs its own network.
    let mounted = network_items(&ship.stock, &vehicles);
    let mut plants = Vec::new();
    let mut em = HashMap::new();
    println!();
    for (path, item) in &mounted {
        let Some(online) = profiles.get(item).and_then(|p| p.online()) else {
            continue;
        };
        let name = names.get(item).map_or("?", String::as_str);
        let deltas: Vec<String> = online
            .deltas
            .iter()
            .map(|d| {
                let side = |a: &Option<sc_vehicles::ResourceAmount>| {
                    a.as_ref()
                        .map(|a| format!("{}={:?}", a.resource.as_dcb_str(), a.per_second))
                };
                format!(
                    "{:?} {}->{} min{}{}",
                    d.kind,
                    side(&d.consumes).unwrap_or_default(),
                    side(&d.generates).unwrap_or_default(),
                    d.minimum_fraction,
                    d.dynamic
                        .as_ref()
                        .map(|x| format!(" dyn {x:?}"))
                        .unwrap_or_default()
                )
            })
            .collect();
        println!(
            "{path:<60} {name:<40} EM {:>6} | {}",
            online.em.as_ref().map_or(0.0, |e| e.nominal),
            deltas.join(" | ")
        );
        if let Some(e) = &online.em {
            em.insert(name.to_string(), e.nominal);
        }
        if let Some(ResourceUnits::Segments(units)) = online
            .generation(&ResourceNetworkResource::Power)
            .and_then(|d| d.generates.as_ref())
            .map(|a| a.per_second)
        {
            let size = items.get(item).map_or(0, |i| i.size);
            let own = profiles.get(item).and_then(|p| p.plant_modifier);
            plants.push((units, network.plant_modifier(size, own)));
        }
        if name == "COOL_JUST_S02_Snowfall_SCItem" {
            let conv = online
                .generation(&ResourceNetworkResource::Coolant)
                .ok_or("cooler")?;
            assert_eq!(
                conv.consumes.as_ref().map(|a| a.per_second),
                Some(ResourceUnits::Segments(4))
            );
            assert_eq!(
                conv.generates.as_ref().map(|a| a.per_second.standard()),
                Some(41.0)
            );
        }
    }

    let online = plants.len() as i32;
    let supply: i32 = plants
        .iter()
        .map(|&(u, m)| plant_output(u, online, m))
        .sum();
    println!("\nplants {plants:?} -> supply {supply}");
    assert_eq!(plants, [(19, 2), (19, 2)]);
    assert_eq!(supply, 24);
    for (name, want) in [
        ("POWR_JUST_S02_Sedulity_SCItem", 9410.0),
        ("COOL_JUST_S02_Snowfall_SCItem", 1980.0),
        ("SHLD_BEHR_S03_5CA_SCItem", 1800.0),
        ("RADR_WLOP_S03_Lephari", 3200.0),
        ("LFSP_TYDT_S02_ComfortAirPlus", 625.0),
    ] {
        assert_eq!(em.get(name), Some(&want), "{name}");
    }
    // The power-on distribution in both modes.
    let network = PowerNetwork::build(
        &ship.stock,
        containers.get(guid),
        &vehicles,
        &items,
        &profiles,
        &network,
    );
    assert_eq!(network.supply(), 24);
    let pool = |t: EItemType| {
        network
            .pools
            .iter()
            .position(|p| p.item_type == t)
            .ok_or("pool")
    };
    let (fc, shields, weapons) = (
        pool(EItemType::FlightController)?,
        pool(EItemType::Shield)?,
        pool(EItemType::WeaponGun)?,
    );
    assert_eq!((network.pool_size(fc), network.pool_minimum(fc)), (6, 1));
    assert_eq!(
        (network.pool_size(shields), network.pool_minimum(shields)),
        (8, 2)
    );
    assert_eq!(
        (network.pool_size(weapons), network.pool_minimum(weapons)),
        (4, 1)
    );
    for mode in [EMasterMode::SCM, EMasterMode::Navigation] {
        let a = network.power_on(&mode);
        let ratios = network.ratios(&a, &mode);
        println!(
            "\npower-on {} (supply {}, left {}):",
            mode.as_dcb_str(),
            a.supply,
            a.remaining
        );
        let mut by_type: HashMap<&str, f64> = HashMap::new();
        for ((c, g), r) in network.consumers.iter().zip(&a.granted).zip(&ratios) {
            println!(
                "  {:<60} {:<22} {g:>6.3} / {:>5.2}  ratio {r:.3}{}",
                c.path,
                c.item_type.as_dcb_str(),
                c.request as f64 / 1e6,
                if c.draws(&mode) {
                    ""
                } else {
                    "  (off in this mode)"
                }
            );
            if c.draws(&mode) {
                *by_type.entry(c.item_type.as_dcb_str()).or_default() += g;
            }
        }
        for (p, t) in network.pools.iter().zip(&a.pool_targets) {
            println!("  pool {} target {t}", p.item_type.as_dcb_str());
        }
        let readout = network.readout(&a, &mode);
        for i in readout
            .items
            .iter()
            .filter(|i| i.coolant_asked > 0.0 || i.coolant_made > 0.0)
        {
            println!(
                "  {:<40} ratio {:.3}  coolant made {:>5.2} asked {:>5.2}  EM {:>7.1}",
                i.path, i.ratio, i.coolant_made, i.coolant_asked, i.em
            );
        }
        println!(
            "  cooling {:.2} / {:.2} = {:.1} %, EM {:.0}, IR {:.0}, shield {:.0}/s",
            readout.coolant_asked,
            readout.coolant_made,
            100.0 * readout.cooling().unwrap_or(0.0),
            readout.em,
            readout.ir,
            readout.shield_made
        );
        assert_eq!(a.remaining, 0);
        assert_eq!(readout.coolant_made, 82.0);
        if mode == EMasterMode::SCM {
            // Plants 12 + 12, radar 6 × 1, shields 2 × 4 × (0.5 × 0.7), life
            // support 2 × (0.5 × 0.85); one cooler's IR carries it all.
            assert!((readout.coolant_asked - 33.65).abs() < 1e-4);
            assert!((readout.ir - 8710.0 * 33.65 / 41.0).abs() < 0.5);
            // Each 5CA at 2 of 4 (0.5 x 0.7): 19,000 x 0.35.
            assert!((readout.shield_made - 2.0 * 19_000.0 * 0.35).abs() < 1.0);
        }
        assert_eq!(by_type.get("Cooler"), Some(&8.0));
        assert_eq!(by_type.get("FlightController"), Some(&3.0));
        let radar_ls = by_type.get("Radar").unwrap_or(&0.0)
            + by_type.get("LifeSupportGenerator").unwrap_or(&0.0);
        match mode {
            EMasterMode::SCM => {
                assert_eq!(a.pool_targets[shields], 4);
                assert_eq!(by_type.get("Shield"), Some(&4.0));
                assert_eq!(by_type.get("QuantumDrive"), None);
                assert_eq!(radar_ls, 7.0);
            }
            _ => {
                assert_eq!(by_type.get("Shield"), None);
                assert_eq!(by_type.get("QuantumDrive"), Some(&3.0));
                assert_eq!(radar_ls, 8.0);
            }
        }
    }

    println!("asserts passed: Centaurus supply, cooler conversion, nominal EM, pools, power-on");
    Ok(())
}

fn short(name: &str) -> String {
    name.rsplit('.').next().unwrap_or(name).to_string()
}
