//! Re-proves the quality roll against scanned rocks (LIVE 4.10, 2026-10-13):
//! every scanned quality is a possible outcome of its rock type's roll. The
//! Aslarite rock read 741 / 287 (two Aslarite entries) / 588 (Agricium) /
//! 622 (Titanium); the iron asteroid 664 / 325. Quality comes out in a few
//! fixed steps (the resources' quantization bands), so the odds per step are
//! printed, and checked against the exact odds (`QualityRoll::odds`) within
//! one percentage point per step.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_quality_probe
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_mining::{Compositions, Elements, QualityRoll, RockTypes};
use sc_resources::{Quality, Resources};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let resources = Resources::build(datacore.records());
    let catalog = Quality::build(datacore.records());
    let db = datacore.db();
    let name = |guid: &Guid| {
        db.record(guid)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_default()
    };
    let elements = Elements::build(&datacore, &resources);
    let compositions = Compositions::build(&datacore);
    let rock_types = RockTypes::build(&datacore);
    let roll = QualityRoll {
        elements: &elements,
        resources: &resources,
        catalog: &catalog,
        locations: &[],
    };

    for (rock_type, scanned) in [
        (
            "Mining_AsteroidUncommon_Aslarite",
            vec![741u16, 287, 588, 622],
        ),
        ("Mining_AsteroidCommon_Iron", vec![664, 325]),
    ] {
        let composition = rock_types
            .iter()
            .find(|(g, _)| name(g) == rock_type)
            .and_then(|(_, r)| r.composition)
            .and_then(|c| compositions.get(&c))
            .ok_or(format!("no {rock_type}"))?;
        // Every part of these two compositions is always in, so entries
        // follow part order.
        let mut odds: Vec<BTreeMap<u16, u32>> = vec![BTreeMap::new(); composition.parts.len()];
        let runs = 20_000;
        for seed in 0..runs {
            let entries = composition.roll_with_quality(seed, &roll);
            assert_eq!(entries.len(), composition.parts.len());
            for (k, e) in entries.iter().enumerate() {
                *odds[k].entry(e.quality.ok_or("no quality")?).or_default() += 1;
            }
        }
        println!("{rock_type}:");
        for (k, (o, want)) in odds.iter().zip(&scanned).enumerate() {
            let steps: Vec<String> = o
                .iter()
                .map(|(q, n)| format!("{q} {:.1}%", *n as f32 / runs as f32 * 100.0))
                .collect();
            let element = composition.parts[k]
                .element
                .map(|e| name(&e))
                .unwrap_or_default();
            println!("  {element:<14} scanned {want}: {}", steps.join(", "));
            assert!(
                o.contains_key(want),
                "{rock_type} entry {k}: {want} is not an outcome"
            );
            // The exact odds agree with the draws step by step.
            let part = &composition.parts[k];
            let exact = roll
                .odds(&part.element.ok_or("no element")?, part.quality_scale)
                .ok_or("no odds")?;
            for &(q, p) in exact.steps() {
                let drawn = o.get(&q).copied().unwrap_or(0) as f64 / runs as f64;
                assert!(
                    (drawn - p).abs() < 0.01,
                    "{rock_type} entry {k} Q{q}: exact {p:.4}, drawn {drawn:.4}"
                );
            }
            let at_700 = composition
                .chance_at_quality(&part.element.ok_or("no element")?, 700, &roll, 2000)
                .ok_or("no chance")?;
            println!(
                "  {element:<14} exact: Q700+ {:.1}% of rolls, {:.1}% of rocks",
                exact.at_least(700) * 100.0,
                at_700 * 100.0
            );
        }
    }
    println!(
        "asserts passed: every scanned quality is an outcome of the roll, \
         and the exact odds match the draws within 1 point"
    );
    Ok(())
}
