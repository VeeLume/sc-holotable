//! Unit tests on hand-built rulesets and elements (values from the LIVE 4.10
//! ship ruleset); the live-data checks are the `mining_rocks_probe` example.

use sc_extract::Guid;
use sc_items_mining::{FloatModifier, MiningModifiers, ModifierKind};
use sc_items_ship_weapons::{MiningBeam, MiningLaserStats};

use crate::*;

fn ship() -> Ruleset {
    Ruleset {
        guid: Guid::default(),
        power_capacity_per_mass: 10.0,
        decay_per_mass: 0.2,
        optimal_window_size: 0.1,
        optimal_window_factor: 0.75,
        optimal_window_max_size: 0.5,
        resistance_curve_factor: 0.6,
        optimal_window_thinness_curve_factor: 0.7,
        instability_curve_factor: 1.0,
        instability_wave_period: 3.0,
        instability_wave_variance: 1.0,
        default_volume: 5122.499,
        default_mass: 100.0,
        controlled_breaking_fill_rate: 0.5,
        controlled_breaking_fill_rate_danger: 0.7,
        controlled_breaking_decay_rate: 0.5,
        danger_breaking_fill_rate: 0.3,
        danger_breaking_fill_rate_exponent: 2.0,
        danger_breaking_decay_rate: 0.1,
        absorbable_volume_threshold: 330.0,
        modifier_persistence_time: 0.25,
        scu_per_volume: 0.007,
        hit_consistency: HitConsistency {
            history_window: 4.0,
            standard_deviation_multiplier: 10.0,
            time_exponent: 2.0,
            min_deviation: 0.1,
            extraction_magnitude: 1.0,
            max_effect_on_instability: 0.5,
        },
        waste_resource: None,
        inert_density: Some(1.0),
    }
}

fn guid(n: u8) -> Guid {
    let mut s = String::from("00000000-0000-0000-0000-0000000000");
    s.push_str(&format!("{n:02x}"));
    s.parse().expect("guid")
}

fn element(n: u8, instability: f32, resistance: f32, density: f32) -> Element {
    Element {
        guid: guid(n),
        resource: None,
        instability,
        resistance,
        window_midpoint: 0.5,
        window_midpoint_randomness: 0.0,
        window_thinness: 0.0,
        explosion_multiplier: 0.0,
        cluster_factor: 0.0,
        density: Some(density),
    }
}

fn multiplicative(value: f32) -> Option<FloatModifier> {
    Some(FloatModifier {
        kind: ModifierKind::Multiplicative,
        value,
        show_in_ui: true,
    })
}

fn laser(power: f32, modifiers: MiningModifiers) -> Laser {
    Laser::new(MiningLaserStats {
        beams: vec![MiningBeam {
            fire_action: 0,
            hit_type: "ElectricArc".into(),
            energy_per_second: power,
            full_damage_range: 40.0,
            zero_damage_range: 45.0,
        }],
        throttle_minimum: 0.0,
        throttle_lerp_speed: 1.0,
        uses_power_throttle: false,
        modifiers,
        inert_filter: None,
        module_slots: 0,
    })
}

fn close(a: f32, b: f32, tolerance: f32) -> bool {
    (a - b).abs() <= tolerance * b.abs().max(1e-6)
}

/// SCMDB's worked example (scmdb.net): Sileron 18.3 % + 70.9 %, Taranite
/// 10.8 %, 69,259 t: volume 12,974.4 m³, resistance 0.652, instability 2157.
#[test]
fn scmdb_worked_example() {
    let mut elements = Elements::new();
    elements.insert(element(1, 870.0, 0.6, 4.75));
    elements.insert(element(2, 700.0, 0.5, 10.2));
    let entries = [
        Entry {
            element: guid(1),
            fraction: 0.183,
        },
        Entry {
            element: guid(1),
            fraction: 0.709,
        },
        Entry {
            element: guid(2),
            fraction: 0.108,
        },
    ];
    let rock = Rock::from_mass(&ship(), &elements, &entries, 69259.0).expect("rock");
    assert!(close(rock.volume, 12974.4, 1e-3), "{}", rock.volume);
    assert!(
        (rock.resistance - 0.652).abs() < 1e-3,
        "{}",
        rock.resistance
    );
    assert!(
        close(rock.instability, 2157.0, 1e-3),
        "{}",
        rock.instability
    );
}

/// Negative-resistance elements pull resistance down, after the positive ones.
#[test]
fn resistance_sign_split() {
    let mut elements = Elements::new();
    elements.insert(element(1, 0.0, -0.4, 1.0));
    elements.insert(element(2, 0.0, 0.5, 1.0));
    let entries = [
        Entry {
            element: guid(1),
            fraction: 0.5,
        },
        Entry {
            element: guid(2),
            fraction: 0.5,
        },
    ];
    let rock = Rock::from_volume(&ship(), &elements, &entries, 1000.0).expect("rock");
    let w = 0.5f32.powf(0.6);
    let expected = 0.5 * w * (1.0 - 0.4 * w);
    assert!((rock.resistance - expected).abs() < 1e-6);
}

/// The window narrows with thinness and its modifier scales the width around
/// the centre, capped at the ruleset's maximum.
#[test]
fn window_size_and_modifier() {
    let mut elements = Elements::new();
    let mut thin = element(1, 0.0, 0.0, 1.0);
    thin.window_thinness = 2.0;
    elements.insert(thin);
    let entries = [Entry {
        element: guid(1),
        fraction: 1.0,
    }];
    let rock = Rock::from_volume(&ship(), &elements, &entries, 1000.0).expect("rock");
    assert!(close(rock.window.size(), 0.025, 1e-4));

    let w = Window::around(0.5, 0.1);
    assert!(close(w.scaled(1.9404, 0.5).size(), 0.19404, 1e-4));
    assert!(close(w.scaled(10.0, 0.5).size(), 0.5, 1e-6));
    assert!(close(w.scaled(1.9404, 0.5).centre(), 0.5, 1e-6));
}

/// Mass, SCU and the inert density.
#[test]
fn mass_and_scu() {
    let mut elements = Elements::new();
    elements.insert(element(1, 0.0, 0.0, 4.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.25,
    }];
    let rock = Rock::from_volume(&ship(), &elements, &entries, 1000.0).expect("rock");
    assert!(close(rock.mass, 1000.0 * (0.25 * 4.0 + 0.75), 1e-6));
    assert!(close(rock.scu, 7.0, 1e-6));
    assert!(!rock.is_absorbable(&ship()));
}

/// Modifiers multiply across lasers and gadgets; two lasers on one ship halve
/// the instability, two ships double it.
#[test]
fn modifiers_and_owners() {
    let mut elements = Elements::new();
    elements.insert(element(1, 100.0, 0.5, 1.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 1.0,
    }];
    let ruleset = ship();
    let rock = Rock::from_volume(&ruleset, &elements, &entries, 5122.499).expect("rock");
    let mods = MiningModifiers {
        resistance: multiplicative(25.0),
        laser_instability: multiplicative(35.0),
        ..Default::default()
    };
    let one = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(1000.0, mods)],
            gadgets: vec![],
        },
    );
    assert!(close(one.resistance, rock.resistance * 1.25, 1e-6));
    assert!(close(one.instability, 135.0, 1e-4));

    let same_ship = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(1000.0, mods), laser(1000.0, mods)],
            gadgets: vec![],
        },
    );
    assert!(close(
        same_ship.instability,
        100.0 * 1.35 * 1.35 / 2.0,
        1e-4
    ));

    let mut second = laser(1000.0, MiningModifiers::default());
    second.owner = 1;
    let two_ships = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(1000.0, MiningModifiers::default()), second],
            gadgets: vec![],
        },
    );
    assert!(close(two_ships.instability, 200.0, 1e-4));
}

/// Mort13's Breakability Chart: max mass = 5 * (1 - R) * E; the Pitman's +25 %
/// resistance reaches zero at 80 %.
#[test]
fn breakability_threshold() {
    let ruleset = ship();
    let rock = |resistance| Rock {
        entries: vec![],
        volume: 1000.0,
        mass: 1000.0,
        resistance,
        instability_unscaled: 0.0,
        instability: 0.0,
        window: Window::around(0.5, 0.1),
        window_midpoint_spread: 0.0,
        explosion_multiplier: 0.0,
        scu: 7.0,
    };
    let setup = Setup {
        lasers: vec![laser(16638.75, MiningModifiers::default())],
        gadgets: vec![],
    };
    let r0 = ModifiedRock::new(&ruleset, &rock(0.0), &setup);
    assert!(close(r0.max_mass(&ruleset), 83193.75, 1e-6));

    let pitman = MiningModifiers {
        resistance: multiplicative(25.0),
        ..Default::default()
    };
    let r80 = ModifiedRock::new(
        &ruleset,
        &rock(0.8),
        &Setup {
            lasers: vec![laser(4631.0, pitman)],
            gadgets: vec![],
        },
    );
    assert!(r80.max_mass(&ruleset).abs() < 1e-3);
}

/// The simulation agrees with the closed form: just under the mass limit the
/// bar rises, just over it the bar never leaves zero.
#[test]
fn simulation_matches_threshold() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 300.0, 0.0, 1.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 1.0,
    }];
    let setup = Setup {
        lasers: vec![laser(2000.0, MiningModifiers::default())],
        gadgets: vec![],
    };
    let limit = 5.0 * 2000.0;
    let config = SimConfig {
        max_time_s: 20.0,
        ..Default::default()
    };
    for (mass, rises) in [(limit * 0.98, true), (limit * 1.02, false)] {
        let rock = Rock::from_mass(&ruleset, &elements, &entries, mass).expect("rock");
        let mined = ModifiedRock::new(&ruleset, &rock, &setup);
        assert_eq!(mined.is_breakable(&ruleset), rises);
        let trace = simulate(&mined, &ruleset, &mut Fixed(1.0), &config);
        let peak = trace.samples.iter().map(|s| s.power).fold(0.0, f32::max);
        assert_eq!(peak > 0.0, rises, "mass {mass}: peak {peak}");
    }
}

/// At the balance throttle the bar holds still, wave or not (the wave only
/// amplifies a gain).
#[test]
fn balance_throttle_holds() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 500.0, 0.3, 2.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.8,
    }];
    let rock = Rock::from_mass(&ruleset, &elements, &entries, 5000.0).expect("rock");
    let mined = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(4000.0, MiningModifiers::default())],
            gadgets: vec![],
        },
    );
    let balance = mined.balance_throttle(&ruleset).expect("holdable");
    assert!(mined.net_rate(&ruleset, balance).abs() < 1e-6);
    let below = simulate(
        &mined,
        &ruleset,
        &mut Fixed(balance * 0.99),
        &SimConfig {
            max_time_s: 10.0,
            ..Default::default()
        },
    );
    assert!(below.samples.iter().all(|s| s.power == 0.0));
}

/// Hit consistency: 1 after 4 s within 0.1 m; ramps as (t/4)^2; 0.2 at 0.5 m.
#[test]
fn hit_consistency_values() {
    let p = ship().hit_consistency;
    assert!(close(hit_consistency(&p, 4.0, 0.05), 1.0, 1e-6));
    assert!(close(hit_consistency(&p, 2.0, 0.1), 0.25, 1e-6));
    assert!(close(hit_consistency(&p, 10.0, 0.5), 0.2, 1e-6));
}

/// Extraction speed does not depend on the rock: (1 + c) * 0.013 * E * 0.007.
#[test]
fn extraction_rate() {
    let mut l = laser(3900.0, MiningModifiers::default());
    l.stats.beams.push(MiningBeam {
        fire_action: 1,
        hit_type: "Extraction".into(),
        energy_per_second: 1295.0,
        full_damage_range: 40.0,
        zero_damage_range: 45.0,
    });
    let ruleset = ship();
    assert!(close(
        l.extraction_scu_per_s(&ruleset, 0.0),
        0.013 * 1295.0 * 0.007,
        1e-6
    ));
    assert!(close(
        l.extraction_scu_per_s(&ruleset, 1.0),
        2.0 * 0.013 * 1295.0 * 0.007,
        1e-6
    ));
}

/// A reasonable player fractures an easy rock cleanly; the same rock with
/// huge instability overcharges more often.
#[test]
fn difficulty_orders_rocks() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 5.0, 0.2, 3.0));
    elements.insert(element(2, 3000.0, 0.2, 3.0));
    let setup = Setup {
        lasers: vec![laser(4000.0, MiningModifiers::default())],
        gadgets: vec![],
    };
    let config = SimConfig::default();
    let score = |n| {
        let entries = [Entry {
            element: guid(n),
            fraction: 0.7,
        }];
        let rock = Rock::from_mass(&ruleset, &elements, &entries, 8000.0).expect("rock");
        let mined = ModifiedRock::new(&ruleset, &rock, &setup);
        difficulty(&mined, &ruleset, &Player::default(), &config, 40)
    };
    let easy = score(1);
    let wild = score(2);
    assert!(easy.controlled > 0.9 && easy.mean_danger < 0.05, "{easy:?}");
    assert!(wild.overcharged > easy.overcharged, "{easy:?} vs {wild:?}");
}

/// The composition roll: every part with probability 1 is in, shares stay in
/// range, the total never passes 100 %, and missing parts are forced up to
/// the minimum count.
#[test]
fn composition_roll() {
    let part = |n, min, max, probability| Part {
        element: Some(guid(n)),
        min_percentage: min,
        max_percentage: max,
        probability,
        curve_exponent: 1.0,
        quality_scale: 1.0,
    };
    let composition = Composition {
        guid: Guid::default(),
        deposit_name: Default::default(),
        minimum_distinct_elements: 2,
        // The forced parts always fit (a forced part that would pass 100 %
        // stops the forcing, as in the client).
        parts: vec![
            part(1, 30.0, 70.0, 1.0),
            part(2, 10.0, 15.0, 0.01),
            part(3, 10.0, 15.0, 0.01),
        ],
    };
    for seed in 0..200 {
        let entries = composition.roll(seed);
        assert!(entries.len() >= 2, "{entries:?}");
        assert_eq!(entries[0].element, guid(1));
        assert!((0.3..=0.7).contains(&entries[0].fraction));
        assert!(entries.iter().map(|e| e.fraction).sum::<f32>() <= 1.0 + 1e-6);
    }
}

/// Fracture tree: scaling by s multiplies every volume by s³; cracks count
/// the rock and each uncollectable piece.
#[test]
fn shape_cracks() {
    let leaf = |v| Piece {
        volume: v,
        pieces: vec![],
    };
    let shape = Shape {
        path: "rock.cgf".into(),
        volume: 1000.0,
        pieces: vec![
            Piece {
                volume: 600.0,
                pieces: vec![leaf(300.0), leaf(290.0)],
            },
            leaf(300.0),
        ],
    };
    let ruleset = ship();
    assert_eq!(shape.cracks_to_collect(&ruleset), Some(2));
    assert_eq!(shape.collectable_pieces(&ruleset), [300.0, 290.0, 300.0]);
    let small = shape.scaled(0.5);
    assert!((small.volume - 125.0).abs() < 1e-3);
    assert_eq!(small.cracks_to_collect(&ruleset), Some(0));
    let big = shape.scaled(1.2);
    assert_eq!(big.cracks_to_collect(&ruleset), None); // 300 * 1.728 m³ leaves
}
