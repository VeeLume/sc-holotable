//! Re-proves the rock spawn model against LIVE data and the iron asteroid
//! Valerie fractured on 2026-10-13 (iron 13.51 % at quality 664 + 67.01 % at
//! 325, scale 0.639):
//!
//! - `Mining_AsteroidCommon_Iron` (scale 0.6..1) rolls from a composition that
//!   lists iron twice, 9.7..15.7 % at quality scale 1 and 34.3..84.3 % at
//!   0.49 (= 325 / 664);
//! - provider entries tagged "Asteroid" spawn asteroid_01..03 (a third each),
//!   "Desert" entries the surface rock mesh;
//! - rolled compositions stay inside the parts' ranges.
//!
//! Then surveys the Golem with the Pitman (Focus III + Rieger-C3) on common
//! iron in Stanton's asteroid entries and on planet surfaces.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_spawn_probe
//! ```

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection, RecordPaths,
};
use sc_gathering::Providers;
use sc_items::Items;
use sc_items_mining::MiningModules;
use sc_items_ship_weapons::ShipWeapons;
use sc_mining::{
    Compositions, Elements, Laser, RockTypes, Rulesets, Setup, Shape, Shapes, Spawner,
};
use sc_resources::Resources;
use sc_tags::Tags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let resources = Resources::build(datacore.records());
    let items = Items::build(datacore.records());
    let tags = Tags::build(datacore.records());
    let providers = Providers::build(datacore.records(), &RecordPaths::build(&datacore));
    let db = datacore.db();
    // Probe fixtures are found by record name; the library joins by GUID.
    let name = |guid: &Guid| {
        db.record(guid)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_default()
    };

    let rulesets = Rulesets::build(&datacore, &resources);
    let elements = Elements::build(&datacore, &resources);
    let compositions = Compositions::build(&datacore);
    let rock_types = RockTypes::build(&datacore);
    let shapes = Shapes::build(&assets, &rock_types);

    let (iron_guid, iron) = rock_types
        .iter()
        .find(|(g, _)| name(g) == "Mining_AsteroidCommon_Iron")
        .ok_or("no Mining_AsteroidCommon_Iron")?;
    assert_eq!((iron.min_scale, iron.max_scale), (0.6, 1.0));
    let composition = compositions
        .get(&iron.composition.ok_or("no composition")?)
        .ok_or("composition")?;
    let iron_parts: Vec<_> = composition
        .parts
        .iter()
        .filter(|p| p.element.map(|e| name(&e)) == Some("Iron_Ore".into()))
        .collect();
    assert_eq!(iron_parts.len(), 2);
    assert!((iron_parts[0].min_percentage, iron_parts[0].max_percentage) == (9.7, 15.7));
    assert!((13.51..=15.7).contains(&13.51) && (34.3..=84.3).contains(&67.01));
    assert!((iron_parts[1].quality_scale - 325.0 / 664.0).abs() < 0.01);
    for seed in 0..500 {
        let entries = composition.roll(seed);
        let total: f32 = entries.iter().map(|e| e.fraction).sum();
        assert!(
            total <= 1.0 + 1e-6 && entries.len() >= 2,
            "seed {seed}: {entries:?}"
        );
    }

    // Provider geometry tags -> meshes.
    let file = |s: &Shape| s.path.rsplit('/').next().unwrap_or("").to_string();
    let entries_for = |preset: &Guid| {
        providers
            .iter()
            .flat_map(|(p, prov)| {
                prov.groups
                    .iter()
                    .flat_map(move |g| g.elements.iter().map(move |e| (p, e)))
            })
            .filter(|(_, e)| e.harvestable == Some(*preset))
            .collect::<Vec<_>>()
    };
    let mut asteroid_meshes = None;
    for (_, e) in entries_for(iron_guid) {
        let meshes: Vec<String> = iron
            .geometries_for(&tags, &e.geometry_tags)
            .iter()
            .filter_map(|g| shapes.get(&g.path).map(file))
            .collect();
        let tag_names: Vec<&str> = e
            .geometry_tags
            .iter()
            .filter_map(|t| tags.get(t).map(|n| n.name.as_str()))
            .collect();
        if tag_names == ["Asteroid"] {
            assert_eq!(
                meshes,
                ["asteroid_01.cgf", "asteroid_02.cgf", "asteroid_03.cgf"]
            );
            asteroid_meshes = Some(meshes);
        }
    }
    assert!(
        asteroid_meshes.is_some(),
        "no Asteroid-tagged common iron entry"
    );
    let (surface_guid, surface) = rock_types
        .iter()
        .find(|(g, _)| name(g) == "Mining_Common_Iron")
        .ok_or("no Mining_Common_Iron")?;
    let desert = entries_for(surface_guid)
        .into_iter()
        .find(|(_, e)| {
            e.geometry_tags
                .iter()
                .any(|t| tags.get(t).is_some_and(|n| n.name == "Desert"))
        })
        .ok_or("no Desert-tagged surface iron entry")?;
    let desert_meshes: Vec<String> = surface
        .geometries_for(&tags, &desert.1.geometry_tags)
        .iter()
        .filter_map(|g| shapes.get(&g.path).map(file))
        .collect();
    assert_eq!(desert_meshes, ["mining_rock_01_breakable.cgf"]);
    println!("asserts passed: common iron composition, geometry tags, roll bounds\n");

    // Survey.
    let weapons = ShipWeapons::build(&datacore, &items);
    let modules = MiningModules::build(&datacore, &items);
    let module = |n: &str| {
        modules
            .iter()
            .find(|(g, _)| name(g) == n)
            .map(|(_, m)| m.clone())
    };
    let mut pitman = Laser::new(
        weapons
            .iter()
            .find(|(g, _)| name(g) == "Mining_Laser_DRAK_Golem_S1")
            .and_then(|(_, w)| w.mining.clone())
            .ok_or("no Pitman")?,
    );
    pitman.modules = [
        module("Mining_Modules_Passive_Focus_MK3"),
        module("Mining_Modules_Passive_Rieger_MK3"),
    ]
    .into_iter()
    .flatten()
    .collect();
    let golem = Setup {
        lasers: vec![pitman],
        gadgets: vec![],
        activations: vec![],
    };
    for (label, (guid, rock_type), geometry_tags) in [
        (
            "asteroid common iron",
            (iron_guid, iron),
            entries_for(iron_guid)
                .into_iter()
                .map(|(_, e)| e.geometry_tags.clone())
                .find(|t| {
                    t.iter()
                        .any(|g| tags.get(g).is_some_and(|n| n.name == "Asteroid"))
                })
                .unwrap_or_default(),
        ),
        (
            "surface common iron (Desert)",
            (surface_guid, surface),
            desert.1.geometry_tags.clone(),
        ),
    ] {
        let ruleset = rulesets
            .get(&rock_type.ruleset.ok_or("no ruleset")?)
            .ok_or("ruleset")?;
        let meshes: Vec<&Shape> = rock_type
            .geometries_for(&tags, &geometry_tags)
            .iter()
            .filter_map(|g| shapes.get(&g.path))
            .collect();
        let spawner = Spawner {
            rock_type,
            ruleset,
            elements: &elements,
            compositions: &compositions,
            shapes: &meshes,
        };
        let survey = spawner.survey(&golem, 5000);
        println!("{label} ({}):\n  {survey:?}", name(guid));
    }
    Ok(())
}
