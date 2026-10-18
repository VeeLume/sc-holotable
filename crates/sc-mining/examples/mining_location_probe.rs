//! Ship-mining rocks per location: for each `HarvestableProviderPreset`, the
//! ship-mineable entries' rock types (scale range, meshes) and a survey of
//! the Golem with the Pitman (Focus III + Rieger-C3) on rocks pooled by the
//! entries' spawn shares. Optionally restricted to one resource's rock types.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_location_probe [rock-type-substring]
//! ```

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection, RecordPaths,
};
use sc_gathering::{GatheringMode, Providers};
use sc_items::Items;
use sc_items_mining::MiningModules;
use sc_items_ship_weapons::ShipWeapons;
use sc_mining::{
    Compositions, Elements, Laser, RockTypes, Rulesets, Setup, Shape, Shapes, Spawner, Survey,
};
use sc_resources::Resources;
use sc_tags::Tags;

const ROCKS_PER_LOCATION: f32 = 4000.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = std::env::args().nth(1).unwrap_or_default();
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let resources = Resources::build(datacore.records());
    let items = Items::build(datacore.records());
    let tags = Tags::build(datacore.records());
    let providers = Providers::build(datacore.records(), &RecordPaths::build(&datacore));
    let db = datacore.db();
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

    if filter == "--density" {
        // Asteroid-field cells (client code): 8192 m cubes; per element
        // max(0, P*5.5*(u+0.5)) cluster origins, P = share * groupProbability
        // / (1 + expected extra members), attempts only while the cell holds
        // fewer than 21 harvestables. Area modifiers are taken as 1.
        println!(
            "{:<28} {:>10} {:>12} {:>14}",
            "provider", "ship rocks", "all rocks", "spacing (21 cap)"
        );
        let mut rows: Vec<(String, f32, f32)> = providers
            .iter()
            .map(|(g, p)| {
                let ship: f32 = p
                    .groups
                    .iter()
                    .filter(|g| g.mode == GatheringMode::Ship)
                    .map(|g| g.group_probability * 5.5)
                    .sum();
                let all: f32 = p.groups.iter().map(|g| g.group_probability * 5.5).sum();
                (name(g), ship, all)
            })
            .collect();
        rows.sort_by(|a, b| b.1.total_cmp(&a.1));
        for (n, ship, all) in rows {
            let per_cell = all.min(21.0);
            let spacing = 8192.0 / per_cell.max(1e-3).cbrt();
            println!("{n:<28} {ship:>10.1} {all:>12.1} {spacing:>12.0} m");
        }
        return Ok(());
    }
    if filter == "--clusters" {
        // Cluster presets the ship-mining entries use: chance, sizes, spacing.
        let store = datacore.records();
        let mut seen = std::collections::BTreeMap::new();
        for (_, provider) in providers.iter() {
            for group in provider
                .groups
                .iter()
                .filter(|g| g.mode == GatheringMode::Ship)
            {
                for e in &group.elements {
                    let _ = e;
                }
            }
        }
        for (pg, h) in &store.records.harvestable.harvestable_provider_preset {
            let Some(preset) = h.get(&store.pools) else {
                continue;
            };
            for g in preset
                .harvestable_groups
                .iter()
                .filter_map(|g| g.get(&store.pools))
            {
                if !g.group_name.contains("SpaceShip") {
                    continue;
                }
                for e in g.harvestables.iter().filter_map(|e| e.get(&store.pools)) {
                    let Some(c) = e.clustering else { continue };
                    let Some(cp) = store
                        .records
                        .harvestable
                        .harvestable_cluster_preset
                        .get(&c)
                        .and_then(|h| h.get(&store.pools))
                    else {
                        continue;
                    };
                    let bands: Vec<String> = cp
                        .cluster_params_array
                        .iter()
                        .filter_map(|b| b.get(&store.pools))
                        .map(|b| {
                            format!(
                                "{}-{} w{} @{}..{}m",
                                b.min_size,
                                b.max_size,
                                b.relative_probability,
                                b.min_proximity,
                                b.max_proximity
                            )
                        })
                        .collect();
                    *seen
                        .entry(format!(
                            "{:<44} p{} {}",
                            name(&c),
                            cp.probability_of_clustering,
                            bands.join(" | ")
                        ))
                        .or_insert(0) += 1;
                    let _ = pg;
                }
            }
        }
        for (k, n) in seen {
            println!("{n:>3} {k}");
        }
        return Ok(());
    }
    let mut rows = Vec::new();
    for (guid, provider) in providers.iter() {
        let mut spawns = Vec::new();
        let mut ruleset = None;
        let mut kinds: Vec<String> = Vec::new();
        for group in provider
            .groups
            .iter()
            .filter(|g| g.mode == GatheringMode::Ship)
        {
            for e in &group.elements {
                let Some(rock_type) = e.harvestable.and_then(|h| rock_types.get(&h)) else {
                    continue;
                };
                let preset = name(&rock_type.guid);
                if !preset.contains(&filter) {
                    continue;
                }
                let Some(rs) = rock_type.ruleset.and_then(|r| rulesets.get(&r)) else {
                    continue;
                };
                ruleset = Some(rs);
                let meshes: Vec<&Shape> = rock_type
                    .geometries_for(&tags, &e.geometry_tags)
                    .iter()
                    .filter_map(|g| shapes.get(&g.path))
                    .collect();
                let mesh_names: Vec<&str> = meshes
                    .iter()
                    .map(|s| {
                        s.path
                            .rsplit('/')
                            .next()
                            .unwrap_or("")
                            .trim_end_matches(".cgf")
                    })
                    .collect();
                kinds.push(format!(
                    "{preset} {:.0}% s{}..{} [{}]",
                    e.share * 100.0,
                    rock_type.min_scale,
                    rock_type.max_scale,
                    mesh_names.join(",")
                ));
                let spawner = Spawner {
                    rock_type,
                    ruleset: rs,
                    elements: &elements,
                    compositions: &compositions,
                    shapes: &meshes,
                };
                let n = (ROCKS_PER_LOCATION * e.share).round() as u32;
                spawns.extend(spawner.spawns(n));
            }
        }
        let Some(ruleset) = ruleset else { continue };
        if spawns.is_empty() {
            continue;
        }
        rows.push((
            name(guid),
            Survey::of(&spawns, ruleset, &elements, &golem),
            kinds,
        ));
    }
    rows.sort_by(|a, b| a.1.breakable.total_cmp(&b.1.breakable));
    println!(
        "{:<28} {:>6} {:>6} {:>9} {:>9} {:>9} {:>6}",
        "provider", "rocks", "break", "mass p10", "p50", "p90", "whole"
    );
    for (provider, s, kinds) in &rows {
        println!(
            "{provider:<28} {:>6} {:>5.0}% {:>9.0} {:>9.0} {:>9.0} {:>5.0}%",
            s.rocks,
            s.breakable * 100.0,
            s.mass.p10,
            s.mass.p50,
            s.mass.p90,
            s.collectable_whole * 100.0
        );
        if !filter.is_empty() {
            for k in kinds {
                println!("    {k}");
            }
        }
    }
    Ok(())
}
