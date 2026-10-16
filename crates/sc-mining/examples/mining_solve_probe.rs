//! Exploration: can a given scanned rock be cracked, and with what? The rock
//! is the Quantainium Valerie scanned on 2026-10-14 (127,487 t, 54.29 SCU,
//! Quantainium 10.31 % + 68.19 %, Beryl 9.46 %, scanner 59 % / 1238.97,
//! IMPOSSIBLE) with a MOLE (three Helix S2 lasers) and a Sabir on it.
//! Prints the rock's own values, then ranks loadouts by how far under the
//! mass limit they get it, with the balance throttle and the wave size.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_solve_probe
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_mining::{Activation, MiningGadget, MiningGadgets, MiningModule, MiningModules};
use sc_items_ship_weapons::ShipWeapons;
use sc_mining::{Elements, Entry, Laser, ModifiedRock, Rock, Rulesets, Setup};
use sc_resources::Resources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let resources = Resources::build(datacore.records());
    let items = Items::build(datacore.records());
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
    let ship = rulesets
        .iter()
        .find(|(g, _)| name(g) == "MiningGlobalParamsShip")
        .map(|(_, r)| r)
        .ok_or("ruleset")?;
    let el = |n: &str| {
        elements
            .iter()
            .find(|(g, _)| name(g) == n)
            .map(|(g, _)| *g)
            .ok_or(format!("no {n}"))
    };
    let entries = [
        Entry {
            element: el("Quantainium_Raw")?,
            fraction: 0.1031,
            quality: Some(514),
        },
        Entry {
            element: el("Quantainium_Raw")?,
            fraction: 0.6819,
            quality: Some(344),
        },
        Entry {
            element: el("Beryl_Raw")?,
            fraction: 0.0946,
            quality: Some(547),
        },
    ];
    let rock =
        Rock::from_volume(ship, &elements, &entries, ship.volume_from_scu(54.29)).ok_or("rock")?;
    println!(
        "rock: {:.0} t (scanned 127,487), {:.0} m3, resistance {:.1} %, instability {:.2}, window {:.3}..{:.3}",
        rock.mass,
        rock.volume,
        rock.resistance * 100.0,
        rock.instability,
        rock.window.low,
        rock.window.high
    );

    let weapons = ShipWeapons::build(&datacore, &items);
    let helix = weapons
        .iter()
        .find(|(g, _)| name(g) == "Mining_Laser_THCN_Helix_S2")
        .and_then(|(_, w)| w.mining.clone())
        .ok_or("no Helix S2")?;
    println!(
        "Helix S2: {} energy/s, modifiers {:?}, slots {}",
        helix.fracture_beam().map_or(0.0, |b| b.energy_per_second),
        helix.modifiers,
        helix.module_slots
    );
    let modules: Vec<(String, MiningModule)> = MiningModules::build(&datacore, &items)
        .iter()
        .map(|(g, m)| (name(g).replace("Mining_Modules_", ""), m.clone()))
        .collect();
    let gadgets: Vec<(String, MiningGadget)> = MiningGadgets::build(&datacore, &items)
        .iter()
        .map(|(g, m)| (name(g).replace("Mining_Gadget_", ""), m.clone()))
        .collect();
    for (n, g) in &gadgets {
        println!("gadget {n}: {:?}", g.modifiers);
    }

    let setup = |mods: &[usize], gadget: Option<usize>, lasers: usize| Setup {
        lasers: (0..lasers)
            .map(|_| {
                let mut l = Laser::new(helix.clone());
                l.modules = mods.iter().map(|&i| modules[i].1.clone()).collect();
                l
            })
            .collect(),
        gadgets: gadget.map(|g| gadgets[g].1.clone()).into_iter().collect(),
    };
    let report = |label: String, s: &Setup| {
        let m = ModifiedRock::new(ship, &rock, s);
        (
            m.max_mass(ship) / rock.mass,
            format!(
                "{label}: limit {:.0} t ({:.0} %), R {:.1} %, I {:.0}, window {:.3}, balance {:?}",
                m.max_mass(ship),
                m.max_mass(ship) / rock.mass * 100.0,
                m.resistance * 100.0,
                m.instability,
                m.window.size(),
                m.balance_throttle(ship).map(|t| (t * 100.0).round())
            ),
        )
    };
    let sabir = gadgets.iter().position(|(n, _)| n.contains("Sabir"));
    for lasers in [1, 3] {
        for g in [None, sabir] {
            let label = format!(
                "{lasers} Helix S2 bare{}",
                if g.is_some() { " + Sabir" } else { "" }
            );
            println!("{}", report(label, &setup(&[], g, lasers)).1);
        }
    }

    // Hand-picked scenarios: fewest Surges, and a second ship.
    let idx = |n: &str| {
        modules
            .iter()
            .position(|(m, _)| m == n)
            .ok_or(format!("no module {n}"))
    };
    let (surge, rieger) = (idx("Active_Surge")?, idx("Passive_Rieger_MK3")?);
    let helix_s1 = weapons
        .iter()
        .find(|(g, _)| name(g) == "Mining_Laser_THCN_Helix_S1")
        .and_then(|(_, w)| w.mining.clone())
        .ok_or("no Helix S1")?;
    let laser = |stats: &sc_items_ship_weapons::MiningLaserStats, mods: &[usize], owner: u32| {
        let mut l = Laser::new(stats.clone());
        l.modules = mods.iter().map(|&i| modules[i].1.clone()).collect();
        l.owner = owner;
        l
    };
    let rr = [rieger, rieger, rieger];
    let focus = idx("Passive_Focus_MK3")?;
    let ff = [focus, focus, focus];
    let sabir_on = || {
        sabir
            .map(|g| gadgets[g].1.clone())
            .into_iter()
            .collect::<Vec<_>>()
    };
    let scenarios: Vec<(&str, Setup)> = vec![
        (
            "MOLE as kitted: 3 Focus III per laser, Sabir",
            Setup {
                lasers: (0..3).map(|_| laser(&helix, &ff, 0)).collect(),
                gadgets: sabir_on(),
            },
        ),
        (
            "MOLE as kitted, no Sabir",
            Setup {
                lasers: (0..3).map(|_| laser(&helix, &ff, 0)).collect(),
                gadgets: vec![],
            },
        ),
        (
            "MOLE: 2 Rieger + 1 Focus per laser, Sabir",
            Setup {
                lasers: (0..3)
                    .map(|_| laser(&helix, &[rieger, rieger, focus], 0))
                    .collect(),
                gadgets: sabir_on(),
            },
        ),
        (
            "MOLE: 3 Rieger per laser, Sabir",
            Setup {
                lasers: (0..3).map(|_| laser(&helix, &rr, 0)).collect(),
                gadgets: sabir_on(),
            },
        ),
        (
            "MOLE: 1 laser Surge+2 Rieger, 2 lasers 3 Rieger, Sabir",
            Setup {
                lasers: vec![
                    laser(&helix, &[surge, rieger, rieger], 0),
                    laser(&helix, &rr, 0),
                    laser(&helix, &rr, 0),
                ],
                gadgets: sabir.map(|g| gadgets[g].1.clone()).into_iter().collect(),
            },
        ),
        (
            "MOLE: every laser Surge+2 Rieger, Sabir",
            Setup {
                lasers: (0..3)
                    .map(|_| laser(&helix, &[surge, rieger, rieger], 0))
                    .collect(),
                gadgets: sabir.map(|g| gadgets[g].1.clone()).into_iter().collect(),
            },
        ),
        (
            "two MOLEs, 3 Rieger each laser, Sabir",
            Setup {
                lasers: (0..6).map(|i| laser(&helix, &rr, (i / 3) as u32)).collect(),
                gadgets: sabir.map(|g| gadgets[g].1.clone()).into_iter().collect(),
            },
        ),
        (
            "MOLE (3 Rieger) + Prospector (Helix S1, 2 Rieger), Sabir",
            Setup {
                lasers: (0..3)
                    .map(|_| laser(&helix, &rr, 0))
                    .chain(std::iter::once(laser(&helix_s1, &[rieger, rieger], 1)))
                    .collect(),
                gadgets: sabir.map(|g| gadgets[g].1.clone()).into_iter().collect(),
            },
        ),
    ];
    println!();
    for (label, s) in &scenarios {
        println!("{}", report(label.to_string(), s).1);
        let m = ModifiedRock::new(ship, &rock, s);
        if m.is_breakable(ship) {
            let d = sc_mining::difficulty(
                &m,
                ship,
                &sc_mining::Player::default(),
                &sc_mining::SimConfig::default(),
                200,
            );
            println!(
                "    to window {:?} s at full throttle; player model: controlled {:.0} %, overcharged {:.0} %, median {:?} s, danger at fracture {:.2}",
                m.time_to_window_s(ship).map(|t| t.round()),
                d.controlled * 100.0,
                d.overcharged * 100.0,
                d.median_time_s.map(|t| t.round()),
                d.mean_danger
            );
        }
    }

    // MOLE + two Prospectors (Arbor S1, Helix S1): every module multiset on
    // each Prospector laser, MOLE as kitted (Focus) or on Riegers, Sabir.
    let arbor = weapons
        .iter()
        .find(|(g, _)| name(g) == "Mining_Laser_GRIN_Arbor_S1")
        .and_then(|(_, w)| w.mining.clone())
        .ok_or("no Arbor S1")?;
    let multisets = |k: usize| {
        let mut out: Vec<Vec<usize>> = vec![vec![]];
        for _ in 0..k {
            out = out
                .into_iter()
                .flat_map(|v| {
                    let start = v.last().copied().unwrap_or(0);
                    (start..modules.len()).map(move |i| {
                        let mut w = v.clone();
                        w.push(i);
                        w
                    })
                })
                .collect();
        }
        out
    };
    let names = |m: &[usize]| {
        m.iter()
            .map(|&i| modules[i].0.as_str())
            .collect::<Vec<_>>()
            .join("+")
    };
    let is_active = |m: &[usize]| {
        m.iter()
            .any(|&i| matches!(modules[i].1.activation, Activation::Active { .. }))
    };
    println!(
        "
Arbor S1 slots {}, Helix S1 slots {}",
        arbor.module_slots, helix_s1.module_slots
    );
    let mut fleet = Vec::new();
    for (mole_label, mole_mods) in [
        ("MOLE Focus kit", ff),
        ("MOLE 3 Rieger", rr),
        ("MOLE 2 Rieger+Focus", [rieger, rieger, focus]),
        ("MOLE Rieger+2 Focus", [rieger, focus, focus]),
    ] {
        for am in multisets(arbor.module_slots as usize) {
            for hm in multisets(helix_s1.module_slots as usize) {
                let s = Setup {
                    lasers: (0..3)
                        .map(|_| laser(&helix, &mole_mods, 0))
                        .chain([laser(&arbor, &am, 1), laser(&helix_s1, &hm, 2)])
                        .collect(),
                    gadgets: sabir_on(),
                };
                let active = is_active(&am) || is_active(&hm);
                let label = format!(
                    "{mole_label} | Arbor [{}] | Helix S1 [{}]",
                    names(&am),
                    names(&hm)
                );
                fleet.push((active, report(label, &s)));
            }
        }
        for (active, title) in [(false, "passive"), (true, "with active modules")] {
            println!(
                "
best fleet ({mole_label}, Prospectors {title}):"
            );
            let mut rows: Vec<_> = fleet
                .iter()
                .filter(|(a, r)| *a == active && r.1.starts_with(mole_label))
                .collect();
            rows.sort_by(|a, b| b.1.0.total_cmp(&a.1.0));
            for (_, (_, line)) in rows.iter().take(4) {
                println!("  {line}");
            }
        }
    }
    // Inventory search: MOLE slots from {Focus, Rieger (max 2 in total),
    // Surge, Stampede}; Prospectors from {nothing, Surge, Stampede}.
    let stampede = idx("Active_Stampede")?;
    for i in [surge, stampede, rieger, focus] {
        println!(
            "module {}: x{} power, {:?}, {:?}",
            modules[i].0,
            modules[i].1.damage_multiplier(0),
            modules[i].1.activation,
            modules[i].1.modifiers
        );
    }
    let pool = [focus, rieger, surge, stampede];
    let mut kits: Vec<Vec<usize>> = Vec::new(); // per-laser multisets
    for a in 0..4 {
        for b in a..4 {
            for c in b..4 {
                kits.push(vec![pool[a], pool[b], pool[c]]);
            }
        }
    }
    let count = |v: &[usize], m: usize| v.iter().filter(|&&i| i == m).count();
    let mut found = Vec::new();
    for x in 0..kits.len() {
        for y in x..kits.len() {
            for z in y..kits.len() {
                let mole = [&kits[x], &kits[y], &kits[z]];
                let riegers: usize = mole.iter().map(|k| count(k, rieger)).sum();
                if riegers > 2 {
                    continue;
                }
                for am in [vec![], vec![surge], vec![stampede]] {
                    for hm in [
                        vec![],
                        vec![surge],
                        vec![stampede],
                        vec![surge, surge],
                        vec![surge, stampede],
                        vec![stampede, stampede],
                    ] {
                        let s = Setup {
                            lasers: mole
                                .iter()
                                .map(|k| laser(&helix, k, 0))
                                .chain([laser(&arbor, &am, 1), laser(&helix_s1, &hm, 2)])
                                .collect(),
                            gadgets: sabir_on(),
                        };
                        let m = ModifiedRock::new(ship, &rock, &s);
                        if !m.is_breakable(ship) {
                            continue;
                        }
                        let all: Vec<usize> = mole
                            .iter()
                            .flat_map(|k| k.iter().copied())
                            .chain(am.iter().copied())
                            .chain(hm.iter().copied())
                            .collect();
                        let actives = count(&all, surge) + count(&all, stampede);
                        let label = format!(
                            "MOLE [{}] / [{}] / [{}] | Arbor [{}] | Helix S1 [{}] ({} Surge, {} Stampede)",
                            names(mole[0]),
                            names(mole[1]),
                            names(mole[2]),
                            names(&am),
                            names(&hm),
                            count(&all, surge),
                            count(&all, stampede)
                        );
                        found.push((
                            m.window.size(),
                            actives,
                            m.max_mass(ship),
                            m.instability,
                            m.balance_throttle(ship),
                            label,
                        ));
                    }
                }
            }
        }
    }
    println!(
        "
{} breakable fleets; widest window first:",
        found.len()
    );
    found.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    for (w, _, limit, inst, bal, label) in found.iter().take(8) {
        println!(
            "  window {w:.3} limit {limit:.0} t ({:.0} %) I {inst:.0} balance {:?} | {label}",
            limit / rock.mass * 100.0,
            bal.map(|t| (t * 100.0).round())
        );
    }
    println!(
        "
most margin first:"
    );
    let mut by_margin: Vec<_> = found.iter().collect();
    by_margin.sort_by(|a, b| b.2.total_cmp(&a.2));
    for (w, _, limit, inst, bal, label) in by_margin.iter().take(5) {
        println!(
            "  limit {limit:.0} t ({:.0} %) window {w:.3} I {inst:.0} balance {:?} | {label}",
            limit / rock.mass * 100.0,
            bal.map(|t| (t * 100.0).round())
        );
    }
    println!(
        "
widest window at a margin of at least 125 % / 150 %:"
    );
    for floor in [1.25f32, 1.5] {
        if let Some((w, _, limit, inst, bal, label)) = found
            .iter()
            .filter(|f| f.2 >= floor * rock.mass)
            .max_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)))
        {
            // Climb at full throttle without the wave (the wave only speeds it up):
            // net = gain - decay, gain = decay * limit / mass.
            let net = 0.02 * (limit / rock.mass - 1.0);
            println!(
                "  >= {:.0} %: limit {limit:.0} t ({:.0} %) window {w:.3} I {inst:.0} balance {:?}, about {:.0} s to the window without the wave | {label}",
                floor * 100.0,
                limit / rock.mass * 100.0,
                bal.map(|t| (t * 100.0).round()),
                rock.window.centre() / net
            );
        }
    }
    for n in 0..=3 {
        if let Some((w, _, limit, inst, bal, label)) = found
            .iter()
            .filter(|f| f.1 == n)
            .max_by(|a, b| a.0.total_cmp(&b.0))
        {
            println!(
                "fewest actives = {n}: window {w:.3} limit {limit:.0} t ({:.0} %) I {inst:.0} balance {:?} | {label}",
                limit / rock.mass * 100.0,
                bal.map(|t| (t * 100.0).round())
            );
        }
    }

    // The fleet as set up on 2026-10-14.
    let rieger2 = idx("Passive_Rieger_MK2")?;
    let now = Setup {
        lasers: vec![
            laser(&helix, &ff, 0),
            laser(&helix, &ff, 0),
            laser(&helix, &[rieger, rieger, stampede], 0),
            laser(&helix_s1, &[focus, surge], 1),
            laser(&helix_s1, &[stampede, rieger2], 2),
        ],
        gadgets: sabir_on(),
    };
    let (_, line) = report("FLEET NOW".into(), &now);
    println!(
        "
{line}"
    );
    let m = ModifiedRock::new(ship, &rock, &now);
    println!(
        "FLEET NOW detail: power {:.0}/s, net climb at full throttle {:.4}/s ({:?} s to the window without the wave)",
        m.power(),
        m.net_rate(ship, 1.0),
        m.time_to_window_s(ship).map(|t| t.round())
    );
    let d = sc_mining::difficulty(
        &m,
        ship,
        &sc_mining::Player::default(),
        &sc_mining::SimConfig::default(),
        300,
    );
    println!(
        "FLEET NOW sim (player model): controlled {:.0} %, overcharged {:.0} %, timed out {:.0} %, median {:?} s, danger at fracture {:.2}, time in window {:.0} %",
        d.controlled * 100.0,
        d.overcharged * 100.0,
        d.timed_out * 100.0,
        d.median_time_s.map(|t| t.round()),
        d.mean_danger,
        d.in_window * 100.0
    );
    let mut times = Vec::new();
    for seed in 0..300 {
        let t = sc_mining::simulate(
            &m,
            ship,
            &mut sc_mining::Fixed(1.0),
            &sc_mining::SimConfig {
                seed,
                record: false,
                ..Default::default()
            },
        );
        if let sc_mining::Outcome::Fractured { time_s, .. } = t.outcome {
            times.push(time_s);
        }
    }
    times.sort_by(f32::total_cmp);
    println!(
        "FLEET NOW sim (full throttle held): fractured {} / 300, median {:?} s",
        times.len(),
        times.get(times.len() / 2).map(|t| t.round())
    );
    let without_surge = Setup {
        lasers: vec![
            laser(&helix, &ff, 0),
            laser(&helix, &ff, 0),
            laser(&helix, &[rieger, rieger, stampede], 0),
            laser(&helix_s1, &[focus], 1),
            laser(&helix_s1, &[stampede, rieger2], 2),
        ],
        gadgets: sabir_on(),
    };
    println!(
        "{}",
        report("FLEET NOW, Surge off".into(), &without_surge).1
    );
    let no_actives = Setup {
        lasers: vec![
            laser(&helix, &ff, 0),
            laser(&helix, &ff, 0),
            laser(&helix, &[rieger, rieger], 0),
            laser(&helix_s1, &[focus], 1),
            laser(&helix_s1, &[rieger2], 2),
        ],
        gadgets: sabir_on(),
    };
    println!(
        "{}",
        report("FLEET NOW, no actives running".into(), &no_actives).1
    );

    // Bare Prospectors for reference.
    let bare = Setup {
        lasers: (0..3)
            .map(|_| laser(&helix, &ff, 0))
            .chain([laser(&arbor, &[], 1), laser(&helix_s1, &[], 2)])
            .collect(),
        gadgets: sabir_on(),
    };
    println!(
        "
{}",
        report("MOLE Focus kit + bare Prospectors".into(), &bare).1
    );

    // Every multiset of 3 modules (same on all three lasers) x every gadget.
    let mut results = Vec::new();
    let n = modules.len();
    for a in 0..n {
        for b in a..n {
            for c in b..n {
                for g in std::iter::once(None).chain((0..gadgets.len()).map(Some)) {
                    let s = setup(&[a, b, c], g, 3);
                    let names: Vec<&str> =
                        [a, b, c].iter().map(|&i| modules[i].0.as_str()).collect();
                    let label = format!(
                        "3 Helix S2 [{}] + {}",
                        names.join(", "),
                        g.map_or("no gadget", |g| gadgets[g].0.as_str())
                    );
                    results.push(report(label, &s));
                }
            }
        }
    }
    results.sort_by(|a, b| b.0.total_cmp(&a.0));
    println!("\nbest by mass limit (all lasers on the rock, active modules counted as active):");
    for (_, line) in results.iter().take(12) {
        println!("  {line}");
    }
    let passive = |l: &str| {
        !modules.iter().any(|(n, m)| {
            matches!(m.activation, Activation::Active { .. }) && l.contains(n.as_str())
        })
    };
    println!("\nbest with passive modules only:");
    for (_, line) in results.iter().filter(|(_, l)| passive(l)).take(8) {
        println!("  {line}");
    }
    Ok(())
}
