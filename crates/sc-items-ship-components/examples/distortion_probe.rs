//! Where `SDistortionParams` lives and what it holds: every entity that
//! carries the component, counted by item type, then the distortion sheet
//! per ship-component kind (pool, decay delay, decay rate ranges) and the
//! S2 power plants side by side.
//!
//! ```bash
//! cargo run -p sc-items-ship-components --release --example distortion_probe
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::generated::DataForgeComponentParamsPtr;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::{Distortion, ShipComponentKind, ShipComponents};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let components = ShipComponents::build(&datacore, &items);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();

    // Every entity carrying the component, whatever its item type.
    let store = datacore.records();
    let mut carriers: BTreeMap<String, usize> = BTreeMap::new();
    let mut outside = Vec::new();
    for (guid, h) in &store.records.multi_feature.entity_class_definition {
        let Some(ecd) = h.get(&store.pools) else {
            continue;
        };
        let has = ecd
            .components
            .iter()
            .any(|c| matches!(c, DataForgeComponentParamsPtr::SDistortionParams(_)));
        if !has {
            continue;
        }
        let ty = items.get(guid).map_or("<not an item>".to_string(), |i| {
            format!("{:?}", i.item_type)
        });
        *carriers.entry(ty.clone()).or_default() += 1;
        if components.get(guid).is_none() {
            outside.push(ty);
        }
    }
    println!("\nentities carrying SDistortionParams, by item type:");
    for (ty, n) in &carriers {
        println!("  {ty:<28} {n}");
    }
    let mut outside_counts: BTreeMap<String, usize> = BTreeMap::new();
    for ty in outside {
        *outside_counts.entry(ty).or_default() += 1;
    }
    println!("  of which outside ShipComponents: {outside_counts:?}");

    // Per kind: how many carry it, and the value ranges.
    let mut kinds: BTreeMap<String, (usize, Vec<&Distortion>)> = BTreeMap::new();
    for c in components.values() {
        let e = kinds.entry(format!("{:?}", c.kind)).or_default();
        e.0 += 1;
        if let Some(d) = &c.distortion {
            e.1.push(d);
        }
    }
    println!(
        "\nper kind: with/total, maximum, decay delay, decay rate, recovery, warning, power@max, only@max"
    );
    for (kind, (total, ds)) in &kinds {
        let range = |f: fn(&Distortion) -> f32| {
            let lo = ds.iter().map(|d| f(d)).fold(f32::INFINITY, f32::min);
            let hi = ds.iter().map(|d| f(d)).fold(f32::NEG_INFINITY, f32::max);
            if ds.is_empty() {
                "-".to_string()
            } else {
                format!("{lo}..{hi}")
            }
        };
        let only = ds.iter().filter(|d| d.power_change_only_at_max).count();
        println!(
            "  {kind:<22} {:>4}/{total:<4} max {:<14} delay {:<10} rate {:<12} rec {:<10} warn {:<10} pwr {:<8} only {only}",
            ds.len(),
            range(|d| d.maximum),
            range(|d| d.decay_delay),
            range(|d| d.decay_rate),
            range(|d| d.recovery_ratio),
            range(|d| d.warning_ratio),
            range(|d| d.power_ratio_at_max),
        );
    }

    // Components of a carrying kind without the sheet, and the drain time
    // (maximum / decay rate) and delay per kind and size.
    let carrying: Vec<ShipComponentKind> = components
        .values()
        .filter(|c| c.distortion.is_some())
        .map(|c| c.kind)
        .collect();
    println!("\nwithout a sheet, in a carrying kind:");
    let mut shape: BTreeMap<(String, i32), BTreeMap<String, usize>> = BTreeMap::new();
    for (g, c) in components.iter() {
        if !carrying.contains(&c.kind) {
            continue;
        }
        match &c.distortion {
            None => println!(
                "  {:?} {}",
                c.kind,
                names.get(g).map_or("?", |n| n.as_str())
            ),
            Some(d) => {
                let drain = d.maximum / d.decay_rate;
                let free_form = matches!(
                    c.kind,
                    ShipComponentKind::Radar | ShipComponentKind::JumpDrive
                );
                if !free_form && (drain - 15.0).abs() > 0.01 {
                    println!(
                        "  off the 15 s drain: {:?} S{} {} {d:?}",
                        c.kind,
                        c.size,
                        names.get(g).map_or("?", |n| n.as_str())
                    );
                }
                let key = format!("delay {} drain {drain:.2}", d.decay_delay);
                *shape
                    .entry((format!("{:?}", c.kind), c.size))
                    .or_default()
                    .entry(key)
                    .or_default() += 1;
            }
        }
    }
    println!("\ndelay and drain time (maximum / decay rate) per kind and size:");
    for ((kind, size), m) in &shape {
        println!("  {kind:<22} S{size} {m:?}");
    }

    let mut plants: Vec<(&String, _)> = components
        .iter()
        .filter(|(_, c)| c.kind == ShipComponentKind::PowerPlant && c.size == 2)
        .filter_map(|(g, c)| Some((names.get(g)?, c)))
        .collect();
    plants.sort_by_key(|(n, _)| n.as_str());
    println!("\nS2 power plants:");
    for (name, c) in plants {
        println!("  {name:<40} grade {} {:?}", c.grade, c.distortion);
    }

    // Only these kinds carry the sheet.
    for c in components.values() {
        if c.distortion.is_some() {
            assert!(
                matches!(
                    c.kind,
                    ShipComponentKind::Cooler
                        | ShipComponentKind::PowerPlant
                        | ShipComponentKind::QuantumDrive
                        | ShipComponentKind::Shield
                        | ShipComponentKind::Radar
                        | ShipComponentKind::LifeSupportGenerator
                        | ShipComponentKind::JumpDrive
                ),
                "{:?} carries a distortion sheet",
                c.kind
            );
        }
    }
    // starcitizen.tools, 4.10.2-LIVE: JS-400 "Distortion 8,500",
    // TurboDrive 6,500.
    let by_name: HashMap<&str, &Guid> = names.iter().map(|(g, n)| (n.as_str(), g)).collect();
    let sheet = |name: &str| -> Result<&Distortion, String> {
        by_name
            .get(name)
            .and_then(|g| components.get(g))
            .and_then(|c| c.distortion.as_ref())
            .ok_or_else(|| format!("no distortion sheet on {name}"))
    };
    let js400 = sheet("POWR_AMRS_S02_JS400_SCItem")?;
    assert_eq!((js400.maximum, js400.decay_delay), (8500.0, 3.0));
    assert!((js400.maximum / js400.decay_rate - 15.0).abs() < 1e-3);
    assert_eq!(js400.power_ratio_at_max, 0.0);
    assert!(js400.power_change_only_at_max);
    assert_eq!(sheet("POWR_AMRS_S02_TurboDrive_SCItem")?.maximum, 6500.0);

    println!("\nasserts passed: carrying kinds, JS-400 and TurboDrive sheets");
    Ok(())
}
