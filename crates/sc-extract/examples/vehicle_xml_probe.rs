//! Re-proves the vehicle XML reader against the reference numbers (LIVE
//! 4.10.197): the RSI Constellation Mk V Centaurus has 170,500 hull HP over
//! 31 parts, a 45,000 HP vital `geo_body`, the 1 / 12 / 14 / 4 part split by
//! depth below it (erkul's vital / secondary / breakable / subpart), a hull
//! mass of 1,240,000 kg (spviewer), and its stock gun, shield, power plant
//! and cooler hardpoints. Then it reads every ship's XML with its
//! modification and reports what could not be read.
//!
//! ```bash
//! cargo run -p sc-extract --release --example vehicle_xml_probe
//! ```

use std::collections::BTreeMap;

use sc_extract::generated::EItemType;
use sc_extract::{AssetConfig, AssetData, AssetSource, VehicleDefinition};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();

    // Every vehicle entity: (record, definition path, modification).
    let mut vehicles = Vec::new();
    for rec in db.records_by_type("EntityClassDefinition") {
        let Some(v) = components(db, &rec.as_instance())
            .into_iter()
            .find(|c| c.type_name() == Some("VehicleComponentParams"))
        else {
            continue;
        };
        let path = v
            .get_str("vehicleDefinition")
            .unwrap_or_default()
            .to_string();
        if path.is_empty() {
            continue;
        }
        let modification = v.get_str("modification").unwrap_or_default().to_string();
        vehicles.push((short(rec.name().unwrap_or("?")), path, modification));
    }

    let (_, path, modification) = vehicles
        .iter()
        .find(|(name, _, _)| name == "RSI_Constellation_Mk5_Centaurus")
        .ok_or("no Centaurus record")?;
    let ship = VehicleDefinition::read(&assets, path, modification)?;
    assert_eq!(ship.hull_health(), 170_500.0);
    // 31 parts carry HP (1 + 12 + 14 + 4); erkul's '32 parts' also counts
    // something without HP, likely the root part.
    assert_eq!(ship.parts.iter().filter(|p| p.damage_max > 0.0).count(), 31);
    assert_eq!(ship.hull_mass(), Some(1_240_000.0));
    let body = ship
        .parts
        .iter()
        .position(|p| p.name == "geo_body")
        .ok_or("no geo_body")?;
    assert_eq!(ship.parts[body].damage_max, 45_000.0);
    let mut by_depth: BTreeMap<u32, (u32, f32)> = BTreeMap::new();
    for (i, part) in ship.parts.iter().enumerate() {
        if part.damage_max <= 0.0 || i == body {
            continue;
        }
        let mut depth = 0;
        let mut at = part.parent;
        while let Some(p) = at {
            depth += 1;
            if p == body {
                let slot = by_depth.entry(depth).or_default();
                slot.0 += 1;
                slot.1 += part.damage_max;
                break;
            }
            at = ship.parts[p].parent;
        }
    }
    assert_eq!(
        by_depth.values().map(|&(n, _)| n).collect::<Vec<_>>(),
        [12, 14, 4]
    );
    assert_eq!(by_depth[&1].1, 115_300.0);
    assert_eq!(by_depth[&3].1, 8_000.0);
    for (port, ty, size) in [
        ("hardpoint_gun_s5_front_left", EItemType::WeaponGun, 5),
        ("hardpoint_shield_generator_01", EItemType::Shield, 3),
        ("hardpoint_power_plant_01", EItemType::PowerPlant, 2),
        ("hardpoint_cooler_01", EItemType::Cooler, 2),
    ] {
        let p = ship.part(port).and_then(|p| p.port.as_ref()).ok_or(port)?;
        assert!(
            p.types.iter().any(|t| t.item_type == ty),
            "{port}: {:?}",
            p.types
        );
        assert!(p.min_size <= size && size <= p.max_size, "{port}: {p:?}");
    }
    let flags = &ship
        .part("hardpoint_gun_s5_front_left")
        .and_then(|p| p.port.as_ref())
        .ok_or("gun port")?
        .flags;
    assert!(flags.iter().any(|f| f == "$uneditable"), "{flags:?}");
    println!(
        "asserts passed: Centaurus hull 170,500 over 31 parts, body 45,000, split 1/12/14/4, \
         mass 1,240,000, stock hardpoints"
    );

    // Every ship record's XML with its modification.
    let (mut ok, mut failed) = (0, Vec::new());
    let mut patched = 0;
    for (name, path, modification) in &vehicles {
        match VehicleDefinition::read(&assets, path, modification) {
            Ok(def) => {
                ok += 1;
                if !modification.is_empty() {
                    patched += 1;
                }
                if def.parts.is_empty() {
                    failed.push(format!("{name}: no parts"));
                }
            }
            Err(e) => failed.push(format!("{name} ({path} / '{modification}'): {e}")),
        }
    }
    println!(
        "{} vehicle records: {ok} read ({patched} with a modification), {} failed",
        vehicles.len(),
        failed.len()
    );
    for f in &failed {
        println!("  {f}");
    }
    Ok(())
}

fn components<'a>(db: &'a DataCoreDatabase, ent: &Instance<'a>) -> Vec<Instance<'a>> {
    ent.get_array("Components")
        .into_iter()
        .flatten()
        .filter_map(|v| to_instance(db, &v))
        .collect()
}

fn to_instance<'a>(db: &'a DataCoreDatabase, v: &Value<'a>) -> Option<Instance<'a>> {
    match v {
        Value::Class { struct_index, data } => {
            Some(Instance::from_inline_data(db, *struct_index, data))
        }
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}

fn short(name: &str) -> String {
    name.rsplit('.').next().unwrap_or(name).to_string()
}
