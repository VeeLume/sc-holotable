//! Re-proves [`Crew`] on LIVE 4.10: the seat roles of ships whose layout
//! is known (the Constellation Mk5's two remote-turret gunners, the
//! Cutlass Black's copilot on the tail turret, the MOLE's three mining
//! cabs, the Hammerhead's six manned turrets, the Caterpillar's pilot in its
//! docked command module), and how often the crew
//! count matches the crew size the ship record states. Prints every listed
//! ship.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example crew_probe
//! ```
//!
//! `CREW_SHOW=DRAK_Caterpillar,MISC_Freelancer` also prints those ships'
//! seats and roles.

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};
use sc_items::Items;
use sc_vehicles::{Crew, PortContainers, Role, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();

    // Every port is mounted once (a port the hull and the entity both
    // define, as the Caterpillar's docking module, is one port).
    for (g, v) in vehicles.listed() {
        let mut seen = std::collections::HashSet::new();
        for p in &v.stock {
            assert!(
                seen.insert(p.port.to_ascii_lowercase()),
                "{:?} {}",
                names.get(g),
                p.port
            );
        }
    }

    let remote = Role::RemoteTurret { turrets: vec![] };
    let mut all: Vec<(String, i32, Crew)> = vehicles
        .listed()
        .map(|(g, v)| {
            let crew = Crew::compute(v, &v.stock, &vehicles, &items, &containers);
            (names.get(g).cloned().unwrap_or_default(), v.crew_size, crew)
        })
        .collect();
    all.sort_by(|a, b| a.0.cmp(&b.0));
    let mut same = 0;
    for (n, size, c) in &all {
        same += usize::from(c.size() as i32 == *size);
        println!(
            "{n:<44} crew {:>2} (record {size:>2}) pilot {} copilot {} manned {:>2} remote {:>2} mining {} salvage {} passengers {:>2}",
            c.size(),
            c.count(&Role::Pilot),
            c.count(&Role::Copilot),
            c.count(&Role::MannedTurret),
            c.count(&remote),
            c.count(&Role::Mining),
            c.count(&Role::Salvage),
            c.stations.len() - c.size(),
        );
    }
    println!("crew = record crew size on {same} of {} listed", all.len());
    if let Ok(list) = std::env::var("CREW_SHOW") {
        for n in list.split(',') {
            for (_, _, c) in all.iter().filter(|(name, _, _)| name == n) {
                for s in &c.stations {
                    println!("  {n} {} {:?}", s.port, s.roles);
                }
            }
        }
    }

    let get = |n: &str| {
        all.iter()
            .find(|(name, _, _)| name == n)
            .map(|(_, _, c)| c)
            .ok_or(format!("no {n}"))
    };
    let mk5 = get("RSI_Constellation_Mk5_Centaurus")?;
    for s in &mk5.stations {
        println!("  Centaurus {} {:?}", s.port, s.roles);
    }
    assert_eq!(mk5.size(), 4);
    assert_eq!((mk5.count(&Role::Pilot), mk5.count(&Role::Copilot)), (1, 1));
    assert_eq!(mk5.count(&remote), 2);

    let cutlass = get("DRAK_Cutlass_Black")?;
    for s in &cutlass.stations {
        println!("  Cutlass {} {:?}", s.port, s.roles);
    }
    let copilot = cutlass
        .stations
        .iter()
        .find(|s| s.has(&remote))
        .ok_or("no remote turret seat")?;
    assert!(copilot.has(&Role::Salvage), "{copilot:?}");

    let mole = get("ARGO_MOLE")?;
    assert_eq!(mole.count(&Role::Mining), 4);
    assert_eq!(mole.count(&Role::MannedTurret), 3);

    // Flown from its command module, a docked vehicle.
    let caterpillar = get("DRAK_Caterpillar")?;
    assert_eq!(caterpillar.count(&Role::Pilot), 1);

    let hammerhead = get("AEGS_Hammerhead")?;
    assert_eq!(hammerhead.count(&Role::MannedTurret), 6);
    assert_eq!(hammerhead.count(&Role::Pilot), 1);
    Ok(())
}
