//! Exploration: which single-laser loadout breaks the most rocks. Every
//! mining laser of the given size with every multiset of passive modules for
//! its slots, scored on rocks pooled from the ship-mining entries of the
//! providers whose name contains the filter (all by default).
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_loadout_probe [laser-size] [provider-substring]
//! ```

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection, RecordPaths,
};
use sc_gathering::{GatheringMode, Providers};
use sc_items::Items;
use sc_items_mining::{Activation, MiningModule, MiningModules};
use sc_items_ship_weapons::ShipWeapons;
use sc_mining::{
    Compositions, Elements, Laser, ModifiedRock, RockTypes, Ruleset, Rulesets, Setup, Shape,
    Shapes, Spawn, Spawner,
};
use sc_resources::Resources;
use sc_tags::Tags;

const ROCKS: f32 = 3000.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size: i32 = std::env::args().nth(1).map_or(Ok(1), |s| s.parse())?;
    let filter = std::env::args().nth(2).unwrap_or_default();
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

    // Rocks, pooled over the matching providers by spawn share.
    let chosen: Vec<_> = providers
        .iter()
        .filter(|(g, _)| name(g).contains(&filter))
        .collect();
    let per_provider = ROCKS / chosen.len().max(1) as f32;
    let mut spawns: Vec<Spawn> = Vec::new();
    let mut ruleset: Option<&Ruleset> = None;
    for (_, provider) in &chosen {
        for group in provider
            .groups
            .iter()
            .filter(|g| g.mode == GatheringMode::Ship)
        {
            for e in &group.elements {
                let Some(rock_type) = e.harvestable.and_then(|h| rock_types.get(&h)) else {
                    continue;
                };
                let Some(rs) = rock_type.ruleset.and_then(|r| rulesets.get(&r)) else {
                    continue;
                };
                ruleset = Some(rs);
                let meshes: Vec<&Shape> = rock_type
                    .geometries_for(&tags, &e.geometry_tags)
                    .iter()
                    .filter_map(|g| shapes.get(&g.path))
                    .collect();
                let spawner = Spawner {
                    rock_type,
                    ruleset: rs,
                    elements: &elements,
                    compositions: &compositions,
                    shapes: &meshes,
                };
                let n = (per_provider * e.share).round() as u32;
                spawns.extend(spawner.spawns(n));
            }
        }
    }
    let ruleset = ruleset.ok_or("no ship rocks")?;
    println!(
        "{} providers, {} rocks, laser size {size}",
        chosen.len(),
        spawns.len()
    );

    let weapons = ShipWeapons::build(&datacore, &items);
    let lasers: Vec<(String, Laser)> = weapons
        .iter()
        .filter(|(_, w)| w.size == size)
        .filter_map(|(g, w)| w.mining.clone().map(|m| (name(g), Laser::new(m))))
        .collect();
    let modules: Vec<(String, MiningModule)> = MiningModules::build(&datacore, &items)
        .iter()
        .filter(|(_, m)| m.activation == Activation::Passive)
        .map(|(g, m)| (name(g), m.clone()))
        .collect();
    println!("{} lasers, {} passive modules", lasers.len(), modules.len());

    let score = |setup: &Setup| {
        spawns
            .iter()
            .filter(|s| ModifiedRock::new(ruleset, &s.rock, setup).is_breakable(ruleset))
            .count() as f32
            / spawns.len().max(1) as f32
    };
    let mut results: Vec<(f32, String)> = Vec::new();
    for (laser_name, laser) in &lasers {
        let slots = laser.stats.module_slots as usize;
        for combo in multisets(modules.len(), slots) {
            let mut l = laser.clone();
            l.modules = combo.iter().map(|&i| modules[i].1.clone()).collect();
            let setup = Setup {
                lasers: vec![l],
                gadgets: vec![],
            };
            let names: Vec<&str> = combo
                .iter()
                .map(|&i| modules[i].0.trim_start_matches("Mining_Modules_Passive_"))
                .collect();
            results.push((
                score(&setup),
                format!("{laser_name} + [{}]", names.join(", ")),
            ));
        }
        let bare = Setup {
            lasers: vec![laser.clone()],
            gadgets: vec![],
        };
        results.push((score(&bare), format!("{laser_name} (no modules)")));
    }
    results.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (s, label) in results.iter().take(15) {
        println!("{:>5.1}%  {label}", s * 100.0);
    }
    println!("...");
    for (s, label) in results.iter().filter(|(_, l)| l.contains("(no modules)")) {
        println!("{:>5.1}%  {label}", s * 100.0);
    }
    Ok(())
}

/// Every multiset of `k` indices out of `n` (non-decreasing sequences).
fn multisets(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut cur = Vec::with_capacity(k);
    fn rec(start: usize, n: usize, k: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        for i in start..n {
            cur.push(i);
            rec(i, n, k, cur, out);
            cur.pop();
        }
    }
    if k > 0 {
        rec(0, n, k, &mut cur, &mut out);
    }
    out
}
