//! Exploration: one rock type in one system, and how the single-laser ships
//! do on it. Pools the ship-mining entries whose rock-type name contains the
//! first argument (default `Savril`) over the providers whose name contains
//! the second (default `Nyx`), prints the rocks' bare values (mass, SCU,
//! resistance, instability, window), then searches every Prospector laser
//! (size 1) and the Golem's Pitman with every module multiset and every
//! gadget (or none) for the share of rocks each can break, and runs the
//! player model on the best ones for the share that is reasonably crackable.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_rocktype_probe [rock-type-substring] [provider-substring]
//! ```

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection, RecordPaths,
};
use sc_gathering::{GatheringMode, Providers};
use sc_items::Items;
use sc_items_mining::{Activation, MiningGadget, MiningGadgets, MiningModule, MiningModules};
use sc_items_ship_weapons::{MiningLaserStats, ShipWeapons};
use sc_mining::{
    Compositions, Elements, Laser, ModifiedRock, Player, RockTypes, Ruleset, Rulesets, Setup,
    Shape, Shapes, SimConfig, Spawn, Spawner, difficulty,
};
use sc_resources::Resources;
use sc_tags::Tags;

/// Rocks rolled per matching provider entry.
const ROCKS: u32 = 6000;
/// Breakable rocks per loadout the player model runs on, and attempts each.
const SIM_ROCKS: usize = 150;
const SIM_RUNS: u32 = 16;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = std::env::args().nth(1).unwrap_or_else(|| "Savril".into());
    let system = std::env::args().nth(2).unwrap_or_else(|| "Nyx".into());
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

    // ── the rocks ──────────────────────────────────────────────────────────
    let chosen: Vec<_> = providers
        .iter()
        .filter(|(g, _)| name(g).contains(&system))
        .collect();
    let mut spawns: Vec<Spawn> = Vec::new();
    let mut ruleset: Option<&Ruleset> = None;
    let mut matched = 0u64;
    for (pg, provider) in &chosen {
        for group in provider
            .groups
            .iter()
            .filter(|g| g.mode == GatheringMode::Ship)
        {
            let group_share: f32 = group.elements.iter().map(|e| e.share).sum();
            for e in &group.elements {
                let Some(rock_type) = e.harvestable.and_then(|h| rock_types.get(&h)) else {
                    continue;
                };
                if !name(&rock_type.guid).contains(&filter) {
                    continue;
                }
                let Some(rs) = rock_type.ruleset.and_then(|r| rulesets.get(&r)) else {
                    continue;
                };
                matched += 1;
                ruleset = Some(rs);
                let meshes: Vec<&Shape> = rock_type
                    .geometries_for(&tags, &e.geometry_tags)
                    .iter()
                    .filter_map(|g| shapes.get(&g.path))
                    .collect();
                println!(
                    "{} / {}: share {:.2} % of the group's {:.2} (group p {}), scale {}..{}, ruleset {}",
                    name(pg),
                    name(&rock_type.guid),
                    e.share * 100.0,
                    group_share,
                    group.group_probability,
                    rock_type.min_scale,
                    rock_type.max_scale,
                    rock_type.ruleset.map(|g| name(&g)).unwrap_or_default(),
                );
                let tag_names: Vec<String> = e
                    .geometry_tags
                    .iter()
                    .filter_map(|t| tags.get(t).map(|n| n.name.clone()))
                    .collect();
                println!("    entry geometry tags {tag_names:?}");
                for g in &rock_type.geometries {
                    println!(
                        "    geometry {} tags '{}' volume {:?}",
                        g.path,
                        g.tags,
                        shapes.get(&g.path).map(|s| s.volume.round())
                    );
                }
                println!(
                    "    spawns with {:?}",
                    meshes.iter().map(|s| s.path.as_str()).collect::<Vec<_>>()
                );
                if let Some(c) = rock_type.composition.and_then(|c| compositions.get(&c)) {
                    for p in &c.parts {
                        println!(
                            "    part {:<22} {:>5.1}..{:>5.1} % p {:.2} curve {} quality x{}",
                            p.element.map(|g| name(&g)).unwrap_or_default(),
                            p.min_percentage,
                            p.max_percentage,
                            p.probability,
                            p.curve_exponent,
                            p.quality_scale
                        );
                    }
                }
                let spawner = Spawner {
                    rock_type,
                    ruleset: rs,
                    elements: &elements,
                    compositions: &compositions,
                    shapes: &meshes,
                };
                // Every matching entry gets the same number of rocks: the
                // shares are compared per entry above.
                spawns.extend(
                    (0..u64::from(ROCKS)).filter_map(|i| spawner.spawn(i + 1_000_000 * matched)),
                );
            }
        }
    }
    let ruleset = ruleset.ok_or("no matching ship rocks")?;
    println!(
        "\n{} rocks from {matched} entries; ruleset decay/mass {}, default mass {}",
        spawns.len(),
        ruleset.decay_per_mass,
        ruleset.default_mass
    );
    let stat = |label: &str, values: Vec<f32>| {
        let mut v = values;
        v.sort_by(f32::total_cmp);
        let at = |q: f32| v[((v.len() - 1) as f32 * q).round() as usize];
        let mean = v.iter().sum::<f32>() / v.len() as f32;
        println!(
            "  {label:<16} min {:>10.2}  p10 {:>10.2}  p50 {:>10.2}  mean {:>10.2}  p90 {:>10.2}  max {:>10.2}",
            v[0],
            at(0.1),
            at(0.5),
            mean,
            at(0.9),
            v[v.len() - 1]
        );
    };
    let target = |s: &Spawn| -> f32 {
        s.rock
            .entries
            .iter()
            .filter(|e| name(&e.element).contains(&filter))
            .map(|e| e.fraction)
            .sum()
    };
    println!("bare rock values (what the scanner shows before a beam lands):");
    stat("mass t", spawns.iter().map(|s| s.rock.mass).collect());
    stat("SCU", spawns.iter().map(|s| s.rock.scu).collect());
    stat(
        "resistance %",
        spawns.iter().map(|s| s.rock.resistance * 100.0).collect(),
    );
    stat(
        "instability",
        spawns.iter().map(|s| s.rock.instability).collect(),
    );
    stat(
        "window low",
        spawns.iter().map(|s| s.rock.window.low).collect(),
    );
    stat(
        "window size",
        spawns.iter().map(|s| s.rock.window.size()).collect(),
    );
    stat(
        &format!("{filter} %"),
        spawns.iter().map(|s| target(s) * 100.0).collect(),
    );
    stat(
        &format!("{filter} SCU"),
        spawns.iter().map(|s| target(s) * s.rock.scu).collect(),
    );

    // ── the ships ──────────────────────────────────────────────────────────
    let weapons = ShipWeapons::build(&datacore, &items);
    let mut s1: Vec<(String, MiningLaserStats)> = weapons
        .iter()
        .filter(|(_, w)| w.size == 1)
        .filter_map(|(g, w)| w.mining.clone().map(|m| (name(g), m)))
        .collect();
    s1.sort_by(|a, b| a.0.cmp(&b.0));
    // The Pitman is the Golem's own laser; the Prospector takes the others.
    let (golem, prospector): (Vec<_>, Vec<_>) =
        s1.into_iter().partition(|(n, _)| n.contains("Golem"));
    for (n, l) in golem.iter().chain(&prospector) {
        println!(
            "laser {n}: {:.0} energy/s, slots {}, throttle min {}, modifiers R {:?} I {:?} W {:?}",
            l.fracture_beam().map_or(0.0, |b| b.energy_per_second),
            l.module_slots,
            l.throttle_minimum,
            l.modifiers.resistance.as_ref().map(|m| m.value),
            l.modifiers.laser_instability.as_ref().map(|m| m.value),
            l.modifiers
                .optimal_charge_window_size
                .as_ref()
                .map(|m| m.value),
        );
    }
    let modules: Vec<(String, MiningModule)> = MiningModules::build(&datacore, &items)
        .iter()
        .map(|(g, m)| (name(g).replace("Mining_Modules_", ""), m.clone()))
        .collect();
    let gadgets: Vec<(String, MiningGadget)> = MiningGadgets::build(&datacore, &items)
        .iter()
        .map(|(g, m)| (name(g).replace("Mining_Gadget_", ""), m.clone()))
        .collect();
    for (n, m) in &modules {
        println!(
            "module {n:<24} power x{:.2} R {:?} I {:?} {:?}",
            m.damage_multiplier(0),
            m.modifiers.resistance.as_ref().map(|m| m.value),
            m.modifiers.laser_instability.as_ref().map(|m| m.value),
            m.activation
        );
    }
    for (n, g) in &gadgets {
        println!(
            "gadget {n:<16} R {:?} I {:?} W {:?}",
            g.modifiers.resistance.as_ref().map(|m| m.value),
            g.modifiers.laser_instability.as_ref().map(|m| m.value),
            g.modifiers
                .optimal_charge_window_size
                .as_ref()
                .map(|m| m.value),
        );
    }

    let breakable = |setup: &Setup| -> Vec<usize> {
        spawns
            .iter()
            .enumerate()
            .filter(|(_, s)| ModifiedRock::new(ruleset, &s.rock, setup).is_breakable(ruleset))
            .map(|(i, _)| i)
            .collect()
    };
    // Fast path for the search: breakability is `mass < (1 - R') * P /
    // decayPerMass` with `R' = clamp(R * f + a, 0, 1)`, so a setup reduces to
    // its power and (f, a), read off two probe rocks of tiny resistance.
    let rocks: Vec<(f32, f32)> = spawns
        .iter()
        .map(|s| (s.rock.mass, s.rock.resistance))
        .collect();
    let probe = |r: f32| {
        let mut rock = spawns[0].rock.clone();
        rock.resistance = r;
        rock
    };
    let (probe_lo, probe_hi) = (probe(0.01), probe(0.02));
    let fast_share = |setup: &Setup| -> f32 {
        let lo = ModifiedRock::new(ruleset, &probe_lo, setup);
        let hi = ModifiedRock::new(ruleset, &probe_hi, setup);
        let f = (hi.resistance - lo.resistance) / 0.01;
        let a = lo.resistance - 0.01 * f;
        let k = lo.power() / ruleset.decay_per_mass;
        let n = rocks
            .iter()
            .filter(|(mass, r)| *mass < (1.0 - (r * f + a).clamp(0.0, 1.0)) * k)
            .count();
        n as f32 / rocks.len() as f32
    };
    let is_active = |m: &[usize]| {
        m.iter()
            .any(|&i| matches!(modules[i].1.activation, Activation::Active { .. }))
    };
    struct Row {
        share: f32,
        active: bool,
        label: String,
        setup: Setup,
    }
    let search = |lasers: &[(String, MiningLaserStats)]| -> Vec<Row> {
        let mut rows = Vec::new();
        for (ln, stats) in lasers {
            let slots = stats.module_slots as usize;
            let kits: Vec<Vec<usize>> = (0..=slots)
                .flat_map(|k| multisets(modules.len(), k))
                .collect();
            for kit in kits {
                for g in std::iter::once(None).chain((0..gadgets.len()).map(Some)) {
                    let mut laser = Laser::new(stats.clone());
                    laser.modules = kit.iter().map(|&i| modules[i].1.clone()).collect();
                    let setup = Setup {
                        lasers: vec![laser],
                        gadgets: g.map(|g| gadgets[g].1.clone()).into_iter().collect(),
                        activations: vec![],
                    };
                    let kit_names: Vec<&str> = kit.iter().map(|&i| modules[i].0.as_str()).collect();
                    rows.push(Row {
                        share: fast_share(&setup),
                        active: is_active(&kit),
                        label: format!(
                            "{} [{}] + {}",
                            ln.replace("Mining_Laser_", ""),
                            kit_names.join(", "),
                            g.map_or("no gadget", |g| gadgets[g].0.as_str())
                        ),
                        setup,
                    });
                }
            }
        }
        rows.sort_by(|a, b| b.share.total_cmp(&a.share));
        rows
    };
    // Reasonably crackable: breakable, and the player model fractures it
    // controlled in at least half the attempts within the sim's 180 s.
    // Returns (breakable, reasonable, heaviest breakable t, median fracture s,
    // mean danger).
    let evaluate = |setup: &Setup| -> (f32, f32, f32, Option<f32>, f32) {
        let hits = breakable(setup);
        let share = hits.len() as f32 / spawns.len() as f32;
        let heaviest = hits
            .iter()
            .map(|&i| spawns[i].rock.mass)
            .fold(0.0, f32::max);
        let step = (hits.len() / SIM_ROCKS).max(1);
        let picked: Vec<usize> = hits.iter().step_by(step).copied().collect();
        if picked.is_empty() {
            return (share, 0.0, heaviest, None, 0.0);
        }
        let (mut ok, mut danger_sum) = (0usize, 0.0f32);
        let mut times = Vec::new();
        for &i in &picked {
            let m = ModifiedRock::new(ruleset, &spawns[i].rock, setup);
            let d = difficulty(
                &m,
                ruleset,
                &Player::default(),
                &SimConfig::default(),
                SIM_RUNS,
            );
            if d.controlled >= 0.5 {
                ok += 1;
                danger_sum += d.mean_danger;
                if let Some(t) = d.median_time_s {
                    times.push(t);
                }
            }
        }
        times.sort_by(f32::total_cmp);
        (
            share,
            share * ok as f32 / picked.len() as f32,
            heaviest,
            times.get(times.len() / 2).copied(),
            if ok > 0 { danger_sum / ok as f32 } else { 0.0 },
        )
    };
    let line = |label: &str, setup: &Setup| {
        let (share, ok, heaviest, t, danger) = evaluate(setup);
        println!(
            "  breakable {:>5.1} %  reasonable {:>5.1} %  (heaviest {:>6.0} t, median fracture {:?} s, danger {:.2})  {label}",
            share * 100.0,
            ok * 100.0,
            heaviest,
            t.map(|t| t.round()),
            danger,
        );
    };
    let mid_mass = median(&spawns);
    let mid = spawns
        .iter()
        .min_by(|a, b| {
            (a.rock.mass - mid_mass)
                .abs()
                .total_cmp(&(b.rock.mass - mid_mass).abs())
        })
        .ok_or("no rocks")?;
    let mut best_passive = Vec::new();
    for (ship, lasers) in [("Golem", &golem), ("Prospector", &prospector)] {
        let rows = search(lasers);
        println!("\n{ship}: {} loadouts searched", rows.len());
        for (title, active) in [("passive modules", false), ("with active modules on", true)] {
            println!("{ship}, best by breakable share ({title}):");
            for row in rows.iter().filter(|r| r.active == active).take(6) {
                line(&row.label, &row.setup);
            }
        }
        for row in rows.iter().filter(|r| r.label.contains("[] + no gadget")) {
            println!("  bare: {:.1} % {}", row.share * 100.0, row.label);
        }
        for row in rows
            .iter()
            .filter(|r| !r.active)
            .take(1)
            .chain(rows.first())
        {
            let m = ModifiedRock::new(ruleset, &mid.rock, &row.setup);
            println!(
                "  on the median rock ({:.0} t, R {:.0} %): R {:.1} %, limit {:.0} t ({:.0} % of its mass), {:.0} energy/s | {}",
                mid.rock.mass,
                mid.rock.resistance * 100.0,
                m.resistance * 100.0,
                m.max_mass(ruleset),
                m.max_mass(ruleset) / mid.rock.mass * 100.0,
                m.power(),
                row.label
            );
        }
        let best = rows
            .into_iter()
            .find(|r| !r.active)
            .ok_or("no passive loadout")?;
        best_passive.push(best);
    }

    // ── crews: several ships on one rock ──────────────────────────────────
    let crew = |n_golem: usize, n_prosp: usize| {
        let mut lasers = Vec::new();
        let mut owner = 0;
        for (n, row) in [(n_golem, &best_passive[0]), (n_prosp, &best_passive[1])] {
            for _ in 0..n {
                let mut l = row.setup.lasers[0].clone();
                l.owner = owner;
                owner += 1;
                lasers.push(l);
            }
        }
        // One gadget on the rock: the Prospector kit's, else the Golem's.
        let gadgets = best_passive[1]
            .setup
            .gadgets
            .iter()
            .chain(&best_passive[0].setup.gadgets)
            .take(1)
            .cloned()
            .collect();
        Setup {
            lasers,
            gadgets,
            activations: vec![],
        }
    };
    println!(
        "\ncrews, each ship on its best passive kit, one gadget on the rock:\n  Golem: {}\n  Prospector: {}",
        best_passive[0].label, best_passive[1].label
    );
    for (g, p) in [
        (0, 2),
        (1, 1),
        (2, 0),
        (0, 3),
        (1, 2),
        (2, 2),
        (0, 4),
        (0, 6),
    ] {
        line(&format!("{g} Golem + {p} Prospector"), &crew(g, p));
    }
    Ok(())
}

fn median(spawns: &[Spawn]) -> f32 {
    let mut v: Vec<f32> = spawns.iter().map(|s| s.rock.mass).collect();
    v.sort_by(f32::total_cmp);
    v[v.len() / 2]
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
    rec(0, n, k, &mut cur, &mut out);
    out
}
