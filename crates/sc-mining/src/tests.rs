//! Unit tests on hand-built rulesets and elements (values from the LIVE 4.10
//! ship ruleset); the live-data checks are the `mining_rocks_probe` example.

use sc_extract::Guid;
use sc_items_mining::{
    Activation, BeamMultiplier, FloatModifier, MiningModifiers, MiningModule, ModifierKind,
};
use sc_items_ship_weapons::{Damage, MiningBeam, MiningLaserStats};

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
        danger_pool_factor: 560.0,
        blast: BlastParams {
            min_radius: 0.1,
            max_radius: 0.5,
            pressure: 25.0,
            damage: Damage {
                physical: 1.0,
                ..Damage::default()
            },
        },
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
            quality: None,
        },
        Entry {
            element: guid(1),
            fraction: 0.709,
            quality: None,
        },
        Entry {
            element: guid(2),
            fraction: 0.108,
            quality: None,
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
            quality: None,
        },
        Entry {
            element: guid(2),
            fraction: 0.5,
            quality: None,
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
        quality: None,
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
        quality: None,
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
        quality: None,
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
            activations: vec![],
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
            activations: vec![],
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
            activations: vec![],
        },
    );
    assert!(close(two_ships.instability, 200.0, 1e-4));
}

/// The two rocks captured on 2026-10-17 (Golem, ArcCorp moons): the Laranite
/// rock showed the HUD's striped blast zone, the copper rock none. LIVE
/// element explosion multipliers; volumes from the scanned masses.
#[test]
fn fracture_blast_captured_rocks() {
    let mut elements = Elements::new();
    for (n, explosion) in [(1, 200.0), (2, 80.0), (3, -36.0), (4, -20.0)] {
        let mut e = element(n, 0.0, 0.0, 1.0);
        e.explosion_multiplier = explosion;
        elements.insert(e);
    }
    let entry = |n, fraction| Entry {
        element: guid(n),
        fraction,
        quality: None,
    };
    let ruleset = ship();
    let laranite = Rock::from_volume(
        &ruleset,
        &elements,
        &[entry(1, 0.0727), entry(1, 0.7305), entry(2, 0.0706)],
        596.0,
    )
    .expect("rock");
    let copper = Rock::from_volume(
        &ruleset,
        &elements,
        &[entry(3, 0.0684), entry(4, 0.1150), entry(4, 0.7234)],
        1647.0,
    )
    .expect("rock");
    assert!(close(laranite.explosion_multiplier, 166.288, 1e-4));

    let clean = laranite.fracture_blast(&ruleset, Combined::NONE, 0.0);
    assert!(close(clean.radius_max, 40.8, 2e-3), "{clean:?}");
    assert!(close(clean.radius_min, 8.16, 2e-3), "{clean:?}");
    assert!(close(clean.damage.physical, 19.46, 2e-3), "{clean:?}");
    assert!(close(clean.pressure, 25.0 * 19.46, 2e-3), "{clean:?}");
    let full = laranite.fracture_blast(&ruleset, Combined::NONE, 1.0);
    assert!(close(full.radius_max, 177.5, 2e-3), "{full:?}");

    let copper_clean = copper.fracture_blast(&ruleset, Combined::NONE, 0.0);
    assert!(copper_clean.is_none(), "{copper_clean:?}");
    assert_eq!(copper_clean.damage.total(), 0.0);
    assert!(
        !copper
            .fracture_blast(&ruleset, Combined::NONE, 0.25)
            .is_none()
    );
}

/// Damage falls off with the squared distance between the radii, the push
/// linearly.
#[test]
fn fracture_blast_falloff() {
    let blast = FractureBlast {
        radius_min: 10.0,
        radius_max: 50.0,
        damage: Damage {
            physical: 100.0,
            ..Damage::default()
        },
        pressure: 200.0,
    };
    assert_eq!(blast.damage_falloff(5.0), 1.0);
    assert_eq!(blast.damage_falloff(10.0), 1.0);
    assert_eq!(blast.damage_falloff(50.0), 0.0);
    assert_eq!(blast.damage_falloff(80.0), 0.0);
    // d = 30: (900 - 100) / (2500 - 100) = 1/3 of the way.
    assert!(close(blast.damage_falloff(30.0), 2.0 / 3.0, 1e-6));
    assert!(close(blast.damage_at(30.0).physical, 200.0 / 3.0, 1e-6));
    assert!(close(blast.pressure_at(30.0), 100.0, 1e-6));
    assert_eq!(blast.pressure_at(60.0), 0.0);
}

/// A shatter-damage modifier scales the explosion multiplier; the setup folds
/// it like the other modifiers (Torpid +40 %, Brandt -30 %).
#[test]
fn fracture_blast_shatter_modifier() {
    let mut elements = Elements::new();
    let mut e = element(1, 0.0, 0.0, 1.0);
    e.explosion_multiplier = 200.0;
    elements.insert(e);
    let entries = [Entry {
        element: guid(1),
        fraction: 1.0,
        quality: None,
    }];
    let ruleset = ship();
    let rock = Rock::from_volume(&ruleset, &elements, &entries, 5122.499).expect("rock");
    let mods = |v| MiningModifiers {
        shatter_damage: multiplicative(v),
        ..Default::default()
    };
    let mined = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(1000.0, mods(40.0)), laser(1000.0, mods(-30.0))],
            gadgets: vec![],
            activations: vec![],
        },
    );
    assert!(close(mined.shatter_damage.factor, 1.4 * 0.7, 1e-6));
    let blast = rock.fracture_blast(&ruleset, mined.shatter_damage, 0.0);
    // u = 1, so the outer radius is (200 * 0.98 + 1) * maxRadius.
    assert!(close(blast.radius_max, 197.0 * 0.5, 1e-5), "{blast:?}");
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
        activations: vec![],
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
            activations: vec![],
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
        quality: None,
    }];
    let setup = Setup {
        lasers: vec![laser(2000.0, MiningModifiers::default())],
        gadgets: vec![],
        activations: vec![],
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
        quality: None,
    }];
    let rock = Rock::from_mass(&ruleset, &elements, &entries, 5000.0).expect("rock");
    let mined = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(4000.0, MiningModifiers::default())],
            gadgets: vec![],
            activations: vec![],
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

/// With the rock updating twice per hit, the 1.2-tick drain makes the ticks
/// alternate between 1.2x and 0.8x the mean gain, and the wave amplifies the
/// strong ones: the bar climbs just under the balance throttle, where even
/// hits leave it at zero.
#[test]
fn hit_queue_lets_wave_climb_under_balance() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 500.0, 0.3, 2.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.8,
        quality: None,
    }];
    let rock = Rock::from_mass(&ruleset, &elements, &entries, 5000.0).expect("rock");
    let mined = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(4000.0, MiningModifiers::default())],
            gadgets: vec![],
            activations: vec![],
        },
    );
    assert!(mined.instability > 10.0);
    let under = mined.balance_throttle(&ruleset).expect("holdable") * 0.99;
    let peak = |tick_s: f32| {
        let config = SimConfig {
            tick_s,
            max_time_s: 20.0,
            ..Default::default()
        };
        simulate(&mined, &ruleset, &mut Fixed(under), &config)
            .samples
            .iter()
            .map(|s| s.power)
            .fold(0.0, f32::max)
    };
    assert_eq!(peak(1.0 / 30.0), 0.0);
    assert!(peak(1.0 / 60.0) > 0.01, "peak {}", peak(1.0 / 60.0));
}

/// The queue delivers all hit time: without a wave, hits every other tick
/// charge the bar as fast as hits every tick, but for what is still queued
/// (at most one hit).
#[test]
fn hit_queue_conserves_hit_time() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 0.0, 0.3, 2.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.8,
        quality: None,
    }];
    let rock = Rock::from_mass(&ruleset, &elements, &entries, 5000.0).expect("rock");
    let mined = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![laser(4000.0, MiningModifiers::default())],
            gadgets: vec![],
            activations: vec![],
        },
    );
    assert_eq!(mined.instability, 0.0);
    let power_at = |hit_interval_s: f32| {
        let config = SimConfig {
            hit_interval_s,
            max_time_s: 2.0,
            ..Default::default()
        };
        let trace = simulate(&mined, &ruleset, &mut Fixed(0.5), &config);
        trace.samples.last().expect("ticks").power
    };
    let (even, paired) = (power_at(1.0 / 30.0), power_at(2.0 / 30.0));
    let one_hit = mined.gain_rate(&ruleset, 0.5) * 2.0 / 30.0;
    assert!(even > 0.0 && even < 1.0, "even {even}");
    assert!(
        paired <= even + 1e-6 && even - paired <= one_hit + 1e-6,
        "even {even}, paired {paired}, one hit {one_hit}"
    );
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
        activations: vec![],
    };
    let config = SimConfig::default();
    let score = |n| {
        let entries = [Entry {
            element: guid(n),
            fraction: 0.7,
            quality: None,
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
        base: false,
        pieces: vec![],
    };
    let shape = Shape {
        path: "rock.cgf".into(),
        volume: 1000.0,
        pieces: vec![
            Piece {
                volume: 600.0,
                base: false,
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

fn pieces(volumes: &[f32]) -> Shape {
    Shape {
        path: "rock.cgf".into(),
        volume: volumes.iter().sum(),
        pieces: volumes
            .iter()
            .map(|&v| Piece {
                volume: v,
                base: false,
                pieces: vec![],
            })
            .collect(),
    }
}

/// An overcharge keeps the first `round(n * (1 - danger))` pieces (client
/// code, LIVE 4.10.197): all of 6 up to 1/12, 5 up to 0.25, none at 1; a
/// base piece always stays.
#[test]
fn overcharge_keeps_first_pieces() {
    let six = pieces(&[1.0, 1.0, 1.0, 1.0, 1.0, 5.0]);
    for (danger, kept) in [
        (0.0, 6),
        (0.08, 6),
        (0.09, 5),
        (0.25, 5),
        (0.26, 4),
        (0.92, 0),
        (1.0, 0),
    ] {
        assert_eq!(six.kept_pieces(danger), kept, "danger {danger}");
    }
    assert!(close(six.yield_fraction(0.0), 1.0, 1e-6));
    // The last (biggest) piece goes first: 5 of 10 m³.
    assert!(close(six.yield_fraction(0.2), 0.5, 1e-6));
    assert_eq!(six.yield_fraction(1.0), 0.0);
    let seven = pieces(&[1.0; 7]);
    assert_eq!(seven.kept_pieces(0.0714), 7);
    assert_eq!(seven.kept_pieces(0.072), 6);
    assert_eq!(seven.kept_pieces(0.25), 5);

    let mut based = pieces(&[1.0, 1.0, 1.0]);
    based.pieces[2].base = true;
    assert_eq!(based.kept_pieces(1.0), 1);
    assert_eq!(based.kept(0.5), [true, false, true]);
}

/// A fracture at a danger pool spawns only the kept pieces, and each
/// element fills them at its parent share: the dropped pieces' ore is lost.
#[test]
fn overcharged_fracture_loses_ore() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 0.0, 0.0, 2.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.4,
        quality: None,
    }];
    let shape = pieces(&[100.0, 200.0, 300.0, 400.0]);
    let rock = Rock::from_volume(&ruleset, &elements, &entries, shape.volume).expect("rock");
    let spawn = Spawn {
        scale: 1.0,
        shape,
        rock,
    };
    let ore = |danger: f32| -> (usize, f32) {
        let children = spawn.fracture(&ruleset, &elements, Combined::NONE, danger, 7);
        let ore = children
            .iter()
            .map(|c| c.rock.volume * c.rock.entries.iter().map(|e| e.fraction).sum::<f32>())
            .sum();
        (children.len(), ore)
    };
    let (n, clean) = ore(0.0);
    assert_eq!(n, 4);
    assert!(close(clean, 0.4 * 1000.0, 1e-4), "clean {clean}");
    // round(4 * 0.6) = 2: the 100 and 200 m³ pieces, 40 % of 300 m³.
    let (n, over) = ore(0.4);
    assert_eq!(n, 2);
    assert!(close(over, 0.4 * 300.0, 1e-4), "overcharged {over}");
    assert!(close(over / clean, spawn.shape.yield_fraction(0.4), 1e-4));
    assert_eq!(ore(1.0).0, 0);
}

/// The throttle follows its setting as the client lerps it, each laser on
/// its own setting; the rock gets the sum of their energy.
#[test]
fn throttles_lerp_per_laser() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 0.0, 0.0, 2.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.8,
        quality: None,
    }];
    let rock = Rock::from_mass(&ruleset, &elements, &entries, 50_000.0).expect("rock");
    let mut fast = laser(1000.0, MiningModifiers::default());
    fast.stats.throttle_lerp_speed = 10.0;
    fast.stats.throttle_minimum = 0.1;
    let mut slow = laser(3000.0, MiningModifiers::default());
    slow.stats.throttle_lerp_speed = 6.5;
    let mined = ModifiedRock::new(
        &ruleset,
        &rock,
        &Setup {
            lasers: vec![fast, slow],
            ..Default::default()
        },
    );
    let l = mined.lasers[0];
    let dt = 1.0 / 30.0;
    // alpha = clamp(10/30, 0.1, 1): a third of the gap per tick.
    assert!(close(l.next_throttle(1.0, 0.4, dt), 0.8, 1e-6));
    // Below throttleMinimum per tick the step is throttleMinimum.
    assert!(close(l.next_throttle(1.0, 0.0, 0.001), 0.9, 1e-6));
    assert_eq!(l.next_throttle(0.4 + 5e-6, 0.4, dt), 0.4);

    /// Lead at 1 for a second, then 0.4; the support holds 0.5.
    struct Crew;
    impl Throttle for Crew {
        fn throttle(&mut self, view: &View<'_>, lasers: &mut [LaserControl]) {
            lasers[0].throttle = if view.time_s < 1.0 { 1.0 } else { 0.4 };
            lasers[1].throttle = 0.5;
        }
    }
    let trace = simulate(
        &mined,
        &ruleset,
        &mut Crew,
        &SimConfig {
            max_time_s: 3.0,
            ..Default::default()
        },
    );
    // Sample i is the tick that started at i / 30 s.
    let tick = |t: f32| (t * 30.0).round() as usize;
    assert_eq!(trace.lasers[tick(1.0) - 1][0].throttle, 1.0);
    assert!(close(trace.lasers[tick(1.0)][0].throttle, 0.8, 1e-5));
    assert!(close(trace.lasers[tick(2.5)][0].throttle, 0.4, 1e-5));
    assert!(trace.lasers.iter().all(|t| t[1].throttle == 0.5));
    assert!(close(
        trace.samples[tick(0.5)].energy,
        1000.0 + 1500.0,
        1e-5
    ));
    assert!(close(trace.samples[tick(2.5)].energy, 400.0 + 1500.0, 1e-4));
}

/// A controller moving a laser out past its optimal range cuts its power
/// by the beam's falloff, the same as a setup at that distance.
#[test]
fn distance_per_tick_follows_falloff() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 0.0, 0.0, 2.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.8,
        quality: None,
    }];
    let rock = Rock::from_mass(&ruleset, &elements, &entries, 50_000.0).expect("rock");
    let setup = Setup {
        lasers: vec![laser(2000.0, MiningModifiers::default())],
        ..Default::default()
    };
    let mined = ModifiedRock::new(&ruleset, &rock, &setup);
    let l = mined.lasers[0];
    let beam = &setup.lasers[0].stats.beams[0];
    for d in [0.0, 40.0, 42.5, 44.0, 50.0] {
        assert!(
            close(l.power_at(d), 2000.0 * beam.falloff(d), 1e-6),
            "{d} m"
        );
    }

    struct Drift;
    impl Throttle for Drift {
        fn throttle(&mut self, view: &View<'_>, lasers: &mut [LaserControl]) {
            lasers[0].throttle = 1.0;
            lasers[0].distance_m = 40.0 + view.time_s * 2.5;
        }
    }
    let config = SimConfig {
        max_time_s: 2.0,
        ..Default::default()
    };
    let trace = simulate(&mined, &ruleset, &mut Drift, &config);
    for (s, l) in trace.samples.iter().zip(&trace.lasers) {
        let want = 2000.0 * beam.falloff(l[0].distance_m);
        assert!((s.energy - want).abs() < 1e-2, "{} vs {want}", s.energy);
    }
    // Left alone, the distance stays the setup's: the old constant path.
    let held = simulate(&mined, &ruleset, &mut Fixed(1.0), &config);
    assert!(held.samples.iter().all(|s| s.energy == 2000.0));
}

fn surge(lifetime: f32, charges: i32) -> MiningModule {
    MiningModule {
        guid: guid(90),
        name_key: None,
        size: 1,
        grade: 1,
        activation: Activation::Active {
            duration_s: Some(lifetime),
            uses: Some(charges),
        },
        beam_multipliers: vec![BeamMultiplier {
            fire_action: 0,
            damage_multiplier: 1.5,
        }],
        modifiers: MiningModifiers::default(),
        inert_filter: None,
    }
}

/// An activation works over [start, start + lifetime); charges cap the uses
/// and a second activation while the first is on is ignored.
#[test]
fn activations_switch_modules() {
    let ruleset = ship();
    let mut elements = Elements::new();
    elements.insert(element(1, 0.0, 0.0, 2.0));
    let entries = [Entry {
        element: guid(1),
        fraction: 0.8,
        quality: None,
    }];
    let rock = Rock::from_mass(&ruleset, &elements, &entries, 50_000.0).expect("rock");
    let on = |start_s| ModuleActivation {
        laser: 0,
        module: surge(15.0, 2),
        start_s,
    };
    let setup = Setup {
        lasers: vec![laser(1000.0, MiningModifiers::default())],
        gadgets: vec![],
        activations: vec![on(5.0), on(10.0), on(30.0), on(60.0)],
    };
    let timeline = Timeline::new(&ruleset, &rock, &setup);
    let power = |t| timeline.at(t).power();
    assert_eq!(power(0.0), 1000.0);
    assert_eq!(power(4.99), 1000.0);
    assert_eq!(power(5.0), 1500.0);
    assert_eq!(power(19.99), 1500.0);
    assert_eq!(power(20.0), 1000.0); // the activation at 10 s was ignored
    assert_eq!(power(30.0), 1500.0);
    assert_eq!(power(45.0), 1000.0);
    assert_eq!(power(70.0), 1000.0); // out of charges
    // ModifiedRock::new takes the setup as fitted.
    assert_eq!(ModifiedRock::new(&ruleset, &rock, &setup).power(), 1000.0);

    let trace = simulate_timeline(
        &timeline,
        &ruleset,
        &mut Fixed(1.0),
        &SimConfig {
            max_time_s: 25.0,
            ..Default::default()
        },
    );
    let tick = |t: f32| (t * 30.0).round() as usize;
    assert_eq!(trace.samples[tick(4.0)].energy, 1000.0);
    assert_eq!(trace.samples[tick(6.0)].energy, 1500.0);
    assert_eq!(trace.samples[tick(21.0)].energy, 1000.0);
}
