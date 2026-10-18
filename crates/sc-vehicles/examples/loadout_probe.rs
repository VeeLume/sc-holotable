//! Re-proves loadout editing on the RSI Constellation Mk V Centaurus (LIVE
//! 4.10.197): the S5 gimbals are locked while the guns on them take S5
//! weapons (the stock CF-557 among them), the thrusters are locked, and two
//! QuadraCell MT power plants (21 segments each) make 13 + 13 = 26, as erkul
//! shows for that loadout. Prints the candidate counts.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example loadout_probe
//! ```

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{
    Loadout, NetworkParams, PortContainers, PowerNetwork, Refusal, ResourceProfiles, Vehicles,
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
    let network = NetworkParams::build(&datacore).ok_or("no network params")?;
    let by_name: HashMap<String, Guid> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.name()?.rsplit('.').next()?.to_string(), r.id())))
        .collect();
    let guid = |n: &str| by_name.get(n).copied().ok_or(format!("no record {n}"));
    let names: HashMap<Guid, &str> = by_name.iter().map(|(n, g)| (*g, n.as_str())).collect();

    let ship = guid("RSI_Constellation_Mk5_Centaurus")?;
    let mut loadout = Loadout::stock(&vehicles, &ship).ok_or("no stock loadout")?;
    let item = |n: &str| guid(n).and_then(|g| items.get(&g).ok_or(format!("no item {n}")));

    // Gimbal locked, the gun on it free.
    let gimbal = ["hardpoint_gun_s5_front_left"];
    let gun = ["hardpoint_gun_s5_front_left", "hardpoint_class_2"];
    let cf557 = item("KLWE_LaserRepeater_S5")?;
    assert_eq!(
        loadout.check(
            &gimbal,
            item("Mount_Gimbal_S5")?,
            &items,
            &vehicles,
            &containers
        ),
        Err(Refusal::Locked)
    );
    assert_eq!(
        loadout.check(&gun, cf557, &items, &vehicles, &containers),
        Ok(())
    );
    let guns = loadout.candidates(&gun, &items, &vehicles, &containers);
    println!("S5 gun on the gimbal: {} candidates", guns.len());
    assert!(guns.contains(&guid("KLWE_LaserRepeater_S5")?));
    assert!(
        guns.iter()
            .all(|g| items.get(g).is_some_and(|i| i.size == 5))
    );
    assert_eq!(
        loadout.check(
            &["hardpoint_thruster_main_top_right"],
            cf557,
            &items,
            &vehicles,
            &containers
        ),
        Err(Refusal::Locked)
    );

    // Two QuadraCell MT plants: 13 + 13.
    let plants = [["hardpoint_power_plant_01"], ["hardpoint_power_plant_02"]];
    let quadracell = guid("POWR_AEGS_S02_QuadracellMT_SCItem")?;
    for p in &plants {
        let candidates = loadout.candidates(p, &items, &vehicles, &containers);
        println!(
            "{}: {} candidates, stock {}",
            p[0],
            candidates.len(),
            loadout
                .at(p)
                .and_then(|m| m.item)
                .and_then(|g| names.get(&g).copied())
                .unwrap_or("-")
        );
        assert!(candidates.contains(&quadracell));
        assert!(loadout.equip(p, Some(quadracell), &vehicles, &containers));
    }
    let net = PowerNetwork::build(
        &loadout.mounted,
        containers.get(&ship),
        &vehicles,
        &items,
        &profiles,
        &network,
    );
    println!("supply with 2 x QuadraCell MT: {}", net.supply());
    assert_eq!(net.supply(), 26);

    // Emptying a port drops what it carried.
    assert!(loadout.equip(&gimbal, None, &vehicles, &containers));
    assert!(loadout.at(&gun).is_none());
    println!("asserts passed: locks, candidates, equip, QuadraCell 26");
    Ok(())
}
