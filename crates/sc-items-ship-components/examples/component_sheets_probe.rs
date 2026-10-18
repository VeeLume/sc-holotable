//! Re-proves the per-kind sheets on the stock RSI Constellation Mk V
//! Centaurus's components (LIVE 4.10.197): the 5CA shield generator, the
//! ship's shield emitter (quadrant faces), its hull armour (damage
//! multiplier physical 0.7 / energy 0.5, deflection 531 / 380, 25,740 HP
//! taking physical × 0.81 and energy × 1.21), the Khaos quantum drive and a
//! main thruster. Prints each sheet.
//!
//! ```bash
//! cargo run -p sc-items-ship-components --release --example component_sheets_probe
//! ```

use std::collections::HashMap;

use sc_extract::generated::{EFaceType, EThrusterType};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::{ShipComponentKind, ShipComponentStats, ShipComponents};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let components = ShipComponents::build(&datacore, &items);
    let by_name: HashMap<String, Guid> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.name()?.rsplit('.').next()?.to_string(), r.id())))
        .collect();
    let get = |name: &str| -> Result<&ShipComponentStats, String> {
        by_name
            .get(name)
            .and_then(|g| components.get(g))
            .ok_or_else(|| format!("no component {name}"))
    };
    let mut kinds: HashMap<String, usize> = HashMap::new();
    for c in components.values() {
        *kinds.entry(format!("{:?}", c.kind)).or_default() += 1;
    }
    println!("{} components: {kinds:?}", components.len());

    let shield = get("SHLD_BEHR_S03_5CA_SCItem")?;
    println!("\n5CA {:#?}", shield.shield);
    let s = shield.shield.as_ref().ok_or("5CA sheet")?;
    assert_eq!(s.max_health, 100_000.0);
    assert!(s.resistance.is_some() && s.absorption.is_some());

    let controller = get("Controller_Shield_RSI_Constellation_Mk5")?;
    println!("\nemitter {:?}", controller.emitter);
    let e = controller.emitter.as_ref().ok_or("emitter")?;
    assert_eq!(e.face_type, EFaceType::Quadrant);
    assert_eq!(e.max_reallocation, 1.0);
    assert_eq!(e.reconfiguration_cooldown, 2.5);
    assert!(!e.use_power_for_absorption);

    let armour = get("ARMR_RSI_Constellation_Mk5_Centaurus")?;
    println!("\narmour {:#?}\nhealth {:?}", armour.armor, armour.health);
    assert_eq!(armour.kind, ShipComponentKind::Armor);
    let a = armour.armor.as_ref().ok_or("armour sheet")?;
    let m = a.damage_multiplier.ok_or("damage multiplier")?;
    assert_eq!((m.physical, m.energy), (0.7, 0.5));
    let d = a.deflection.ok_or("deflection")?;
    assert_eq!((d.physical, d.energy), (531.0, 380.0));
    let h = armour.health.as_ref().ok_or("armour health")?;
    assert_eq!(h.health, 25_740.0);
    let hm = h.damage_multiplier.ok_or("armour resistances")?;
    assert_eq!((hm.physical, hm.energy), (0.81, 1.21));

    let qd = get("QDRV_RSI_S02_Khaos_SCItem")?;
    println!("\nKhaos {:#?}", qd.quantum);
    let q = qd.quantum.as_ref().ok_or("quantum sheet")?;
    assert_eq!(
        Some(q.travel.drive_speed / 1_000_000.0),
        qd.quantum_drive_speed
    );
    assert_eq!(Some(q.fuel_requirement), qd.quantum_fuel_requirement);

    let thruster = get("RSI_Constellation_Mk5_Thruster_Main_Top_Right")?;
    println!("\nmain thruster {:?}", thruster.thruster);
    assert_eq!(thruster.kind, ShipComponentKind::MainThruster);
    let t = thruster.thruster.as_ref().ok_or("thruster sheet")?;
    assert_eq!(t.thruster_type, EThrusterType::Main);
    assert!(t.thrust_capacity > 0.0);

    println!("\nasserts passed: shield, emitter, armour, quantum drive, thruster sheets");
    Ok(())
}
