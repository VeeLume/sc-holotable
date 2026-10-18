//! Re-proves the per-kind sheets on the stock RSI Constellation Mk V
//! Centaurus's components (LIVE 4.10.197): the 5CA shield generator, the
//! ship's shield emitter (quadrant faces), its hull armour (damage
//! multiplier physical 0.7 / energy 0.5, deflection 531 / 380, 25,740 HP
//! taking physical × 0.81 and energy × 1.21), the Khaos quantum drive, a
//! main thruster and its tanks (46.8 SCU hydrogen, 8.6 SCU quantum). Prints
//! each sheet.
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

    // The Centaurus's flight blade (legacy IFCS, as spviewer shows it):
    // SCM 170, boost 365 / 191, NAV 1075, pitch / roll / yaw 22 / 35 / 18.
    let blade = get("Controller_Flight_RSI_Constellation_Mk5")?;
    println!("\nflight {:#?}", blade.flight);
    let f = blade.flight.as_ref().ok_or("flight sheet")?;
    assert_eq!((f.scm_speed, f.boost_speed_forward), (170.0, 365.0));
    assert_eq!((f.boost_speed_backward, f.max_speed), (191.0, 1075.0));
    assert_eq!(f.max_angular_velocity, [22.0, 35.0, 18.0]);
    let ab = f.afterburner.as_ref().ok_or("afterburner")?;
    assert_eq!(ab.accel_multiplier_positive, [1.35, 1.5, 1.3]);
    assert_eq!(ab.capacitor_max, 20.0);

    // The Lephari radar: IR / EM / CS detection, and the Centaurus's
    // countermeasure launchers.
    let radar = get("RADR_WLOP_S03_Lephari")?;
    let r = radar.radar.as_ref().ok_or("radar sheet")?;
    println!("\nLephari {r:#?}");
    assert_eq!(r.detection.len(), 8);
    assert_eq!(
        (r.detection[0].sensitivity, r.detection[0].piercing),
        (0.95, 0.25)
    );
    assert!(r.detection[0].passive && !r.detection[2].passive);
    let launchers: Vec<(&String, &ShipComponentStats)> = by_name
        .iter()
        .filter(|(n, _)| n.contains("CML") || n.contains("Countermeasure"))
        .filter_map(|(n, g)| Some((n, components.get(g)?)))
        .filter(|(_, c)| c.kind == ShipComponentKind::CountermeasureLauncher)
        .take(4)
        .collect();
    for (n, c) in &launchers {
        println!("  {n}: {:?}", c.ammo);
    }
    assert!(
        launchers
            .iter()
            .any(|(_, c)| c.ammo.as_ref().is_some_and(|a| a.max > 0))
    );

    // Tank capacities; the tanks without one are outpost props and
    // refinery parts (8 in LIVE 4.10), listed below.
    let tanks: Vec<&ShipComponentStats> = components
        .values()
        .filter(|c| {
            matches!(
                c.kind,
                ShipComponentKind::FuelTank | ShipComponentKind::QuantumFuelTank
            )
        })
        .collect();
    let empty = tanks
        .iter()
        .filter(|c| c.capacity.as_ref().and_then(|q| q.to_scu()).unwrap_or(0.0) <= 0.0)
        .count();
    println!("\n{} tanks, {empty} without a capacity", tanks.len());
    for (name, guid) in &by_name {
        let Some(c) = components.get(guid) else {
            continue;
        };
        let scu = c.capacity.as_ref().and_then(|q| q.to_scu());
        let tank = matches!(
            c.kind,
            ShipComponentKind::FuelTank | ShipComponentKind::QuantumFuelTank
        );
        if (name.contains("Constellation_Mk5") && scu.is_some())
            || (tank && scu.unwrap_or(0.0) <= 0.0)
        {
            println!("  {name}: {scu:?} SCU");
        }
    }
    let scu = |name: &str| get(name).map(|c| c.capacity.as_ref().and_then(|q| q.to_scu()));
    assert_eq!(scu("HTNK_RSI_Constellation_Mk5_Centaurus")?, Some(46.8));
    assert_eq!(scu("QTNK_RSI_Constellation_Mk5_Centaurus")?, Some(8.6));

    println!(
        "\nasserts passed: shield, emitter, armour, quantum drive, thruster, flight, tank sheets"
    );
    Ok(())
}
