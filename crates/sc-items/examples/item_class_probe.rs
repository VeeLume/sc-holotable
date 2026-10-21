//! Re-proves [`ItemClass`] against the shipped `global.ini` (LIVE 4.10):
//! ship components (cooler, power plant, quantum drive, radar, shield) name
//! one of the five classes when they have a `Class:` line (over 300 do),
//! and personal weapons name their weapon class there. 14 named components
//! have no such line at all (the Mauler set, "Main Powerplant",
//! Holdstrong, Frontline, Exotherm): the probe lists them.
//!
//! ```bash
//! cargo run -p sc-items --release --example item_class_probe
//! ```

use std::collections::BTreeMap;

use sc_extract::generated::EItemType;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection, is_placeholder};
use sc_items::{ItemClass, Items};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let locale = &data.locale;

    let components = [
        EItemType::Cooler,
        EItemType::PowerPlant,
        EItemType::QuantumDrive,
        EItemType::Radar,
        EItemType::Shield,
    ];
    let mut classes: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let (mut classless, mut other) = (Vec::new(), Vec::new());
    for (_, item) in items.iter() {
        let class = item.class(locale);
        let t = item.item_type.as_dcb_str().to_string();
        if let Some(c) = &class {
            *classes
                .entry(t.clone())
                .or_default()
                .entry(format!("{c:?}"))
                .or_default() += 1;
        }
        if !components.contains(&item.item_type) {
            continue;
        }
        let name = item.display_name(locale).filter(|n| !is_placeholder(n));
        match (&class, name) {
            (None, Some(n)) => classless.push(format!("{t}: {n}")),
            (Some(ItemClass::Other(v)), _) => other.push(format!("{t}: class {v}")),
            _ => {}
        }
    }
    for (t, c) in &classes {
        println!("{t}: {c:?}");
    }
    classless.sort();
    println!("named components without a Class line: {classless:#?}");
    let classed: usize = components
        .iter()
        .filter_map(|t| classes.get(t.as_dcb_str()))
        .flat_map(|c| c.values())
        .sum();
    assert!(other.is_empty(), "{other:?}");
    assert!(classed >= 300, "{classed} components classed");
    let weapons = classes.get("WeaponPersonal").ok_or("no weapon classes")?;
    assert!(weapons.contains_key("Other(\"Ballistic\")"));
    println!("asserts passed: {classed} components classed, all in the five classes");
    Ok(())
}
