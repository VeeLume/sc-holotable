//! Re-proves the ship signature against the game's power screen (LIVE
//! 4.10.2) on Valerie's F7C-M Super Hornet Mk II: JS-400 plant, 3 × 7SA
//! shields, 2 × Bracer coolers (the right one switched off), SNSR7 radar,
//! Spectre quantum drive (0 in SCM), ComfortAir, Ardor-3 Salvaged × 2
//! (nose), Attrition-3 × 2 (ball turret), M6A × 2 (wings); SCM with weapons
//! and shields at 4. Its items emit 16,057 EM and 5,338 IR with thrusters at
//! 4; the armour multiplies both and the cross-section by 1.13, which gives
//! the screen's 18.1K EM, 6.0K IR and 9.0K CS (the largest axis, 8,040 ×
//! 1.13 = 9,085: the screen truncates). With thrusters off the items emit
//! 14,076 EM and 4,484 IR: 15.9K and 5.0K against the screen's 16.0K and
//! 5.0K (the EM reading is open).
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example signature_probe
//! ```

use std::collections::HashMap;

use sc_extract::generated::{EItemType, EMasterMode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::ShipComponents;
use sc_vehicles::{
    Defense, Loadout, NetworkParams, PortContainers, PowerNetwork, ResourceProfiles, ShipSignature,
    Vehicles,
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
    let profiles = ResourceProfiles::build(&datacore);
    let params = NetworkParams::build(&datacore).ok_or("no ItemResourceNetworkGlobal")?;
    let components = ShipComponents::build(&datacore, &items);
    // A probe names its fixture by record name; the lib joins by GUID.
    let by_name: HashMap<&str, Guid> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| {
            let n = r.name()?;
            Some((
                n.strip_prefix("EntityClassDefinition.").unwrap_or(n),
                r.id(),
            ))
        })
        .collect();
    let guid = |n: &str| by_name.get(n).copied().ok_or(format!("no record {n}"));

    let hornet = guid("ANVL_Hornet_F7CM_Mk2")?;
    let ship = vehicles.get(&hornet).ok_or("not a vehicle")?;
    let mut loadout = Loadout::stock(&vehicles, &hornet).ok_or("no stock loadout")?;
    for (path, item) in [
        (&["hardpoint_power_plant"][..], "POWR_AMRS_S02_JS400_SCItem"),
        (
            &["hardpoint_shield_generator_left"],
            "SHLD_BEHR_S01_7SA_SCItem",
        ),
        (
            &["hardpoint_shield_generator_right"],
            "SHLD_BEHR_S01_7SA_SCItem",
        ),
        (
            &["hardpoint_shield_generator_center"],
            "SHLD_BEHR_S01_7SA_SCItem",
        ),
        (&["hardpoint_radar"], "RADR_NAVE_S01_SNSR7"),
        (&["hardpoint_quantum_drive"], "QDRV_RACO_S01_Spectre_SCItem"),
        (
            &["hardpoint_quantum_drive", "hardpoint_Jump_Drive"],
            "JDRV_TARS_S01_Explorer_SCItem",
        ),
        (
            &["hardpoint_weapon_nose", "hardpoint_weapon_S1_left"],
            "NONE_LaserRepeater_S3",
        ),
        (
            &["hardpoint_weapon_nose", "hardpoint_weapon_S1_right"],
            "NONE_LaserRepeater_S3",
        ),
        (
            &[
                "hardpoint_weapon_center",
                "hardpoint_weapon_left",
                "hardpoint_class_2",
            ],
            "HRST_LaserRepeater_S3",
        ),
        (
            &[
                "hardpoint_weapon_center",
                "hardpoint_weapon_right",
                "hardpoint_class_2",
            ],
            "HRST_LaserRepeater_S3",
        ),
        (
            &["hardpoint_weapon_left_wing", "hardpoint_class_2"],
            "BEHR_LaserCannon_S4",
        ),
        (
            &["hardpoint_weapon_right_wing", "hardpoint_class_2"],
            "BEHR_LaserCannon_S4",
        ),
    ] {
        assert!(
            loadout.equip(path, Some(guid(item)?), &vehicles, &containers),
            "no port {path:?}"
        );
    }
    let defense = Defense::compute(ship, &loadout.mounted, &vehicles, &components);
    let armor = defense.armor.as_ref().ok_or("no armour")?;
    println!(
        "armour signals: EM {} IR {} CS {}",
        armor.signal_electromagnetic, armor.signal_infrared, armor.signal_cross_section
    );
    assert_eq!(
        (
            armor.signal_electromagnetic,
            armor.signal_infrared,
            armor.signal_cross_section
        ),
        (1.13, 1.13, 1.13)
    );

    let mut network = PowerNetwork::build(
        &loadout.mounted,
        containers.get(&hornet),
        &vehicles,
        &items,
        &profiles,
        &params,
    );
    assert!(network.set_on("hardpoint_cooler_right", false));
    let pool = |t: EItemType| network.pools.iter().position(|p| p.item_type == t);
    let (guns, shields, thrusters) = (
        pool(EItemType::WeaponGun).ok_or("no weapons pool")?,
        pool(EItemType::Shield).ok_or("no shield pool")?,
        pool(EItemType::FlightController).ok_or("no thruster pool")?,
    );
    let mode = EMasterMode::SCM;
    let mut states = Vec::new();
    for thrust in [0, 4] {
        let mut a = network.power_on(&mode);
        a.pool_targets[guns] = 4;
        a.pool_targets[shields] = 4;
        a.pool_targets[thrusters] = thrust;
        network.split_pools(&mut a);
        let used: f64 = network
            .consumers
            .iter()
            .zip(&a.granted)
            .filter(|(c, _)| c.draws(&mode))
            .map(|(_, g)| g)
            .sum();
        let readout = network.readout(&a, &mode);
        let sig = ShipSignature::compute(&readout, ship, Some(armor));
        println!(
            "thrusters {thrust}: used {used} of {}; items EM {:.0} IR {:.0}; ship EM {:.0} IR {:.0} CS {:?} (max {:?})",
            network.supply(),
            readout.em,
            readout.ir,
            sig.em,
            sig.ir,
            sig.cross_section,
            sig.cs()
        );
        states.push((used, readout.em, readout.ir, sig));
    }
    // The game: 7 and 3 of 21 left.
    assert!((states[0].0 - 14.0).abs() < 1e-9 && (states[1].0 - 18.0).abs() < 1e-9);
    // Starlume's sums for the same loadout.
    assert!((states[0].1 - 14_076.0).abs() < 1.0 && (states[0].2 - 4_484.0).abs() < 1.0);
    assert!((states[1].1 - 16_057.0).abs() < 1.0 && (states[1].2 - 5_338.0).abs() < 1.0);
    // The screen, in tenths of K, truncated.
    let shown = |v: f64| (v / 100.0).floor() as i64;
    let b = states[1].3;
    assert_eq!((shown(b.em), shown(b.ir)), (181, 60));
    assert_eq!(shown(f64::from(b.cs().ok_or("no CS")?)), 90);
    assert_eq!(shown(states[0].3.ir), 50);
    Ok(())
}
