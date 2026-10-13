//! Re-proves the mining model against the LIVE data and in-game scans
//! (Golem with the Pitman, Focus III + Rieger-C3, 2026-10-13):
//!
//! - aluminium rock A (35.71 SCU): window 0.100, instability 5.75 with the laser;
//! - aluminium rock B (36.60 SCU): instability 2.87 / 3.87, window 0.194 firing;
//! - Beryl (8403 t, 28.18 SCU): resistance 65 % with the laser, instability
//!   230.69, IMPOSSIBLE (limit 7980 t);
//! - Aslarite (6445 t, 20.52 SCU): resistance 48 % / 60 %, instability
//!   301.19 / 406.61 (the scanner truncates resistance to whole percent);
//! - Mort13's Breakability Chart: Helix II with three Surge modules,
//!   16,638.75 energy/s, 83,194 t at 0 % resistance, 57.76 % of that at 100 %.
//!
//! Then prints the setup's view of each rock and our difficulty.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_rocks_probe
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_mining::MiningModules;
use sc_items_ship_weapons::ShipWeapons;
use sc_mining::{
    Elements, Entry, Laser, ModifiedRock, Player, Rock, Ruleset, Rulesets, Setup, SimConfig,
    Window, difficulty,
};
use sc_resources::Resources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
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
    // Probe fixtures are looked up by record name; the library joins by GUID.
    fn find<'a, T>(
        c: &'a impl RecordCollection<Item = T>,
        name: &impl Fn(&Guid) -> String,
        wanted: &str,
    ) -> Result<&'a T, String> {
        c.iter()
            .find(|(g, _)| name(g) == wanted)
            .map(|(_, v)| v)
            .ok_or(format!("no {wanted}"))
    }

    let rulesets = Rulesets::build(&datacore, &resources);
    let elements = Elements::build(&datacore, &resources);
    let ship = find(&rulesets, &name, "MiningGlobalParamsShip")?;
    let weapons = ShipWeapons::build(&datacore, &items);
    let modules = MiningModules::build(&datacore, &items);
    let laser_named = |n: &str| -> Result<Laser, String> {
        find(&weapons, &name, n)?
            .mining
            .clone()
            .map(Laser::new)
            .ok_or(format!("{n} is no mining laser"))
    };
    let module = |n: &str| find(&modules, &name, n).cloned();

    let mut pitman = laser_named("Mining_Laser_DRAK_Golem_S1")?;
    pitman.modules = vec![
        module("Mining_Modules_Passive_Focus_MK3")?,
        module("Mining_Modules_Passive_Rieger_MK3")?,
    ];
    assert!(close(pitman.fracture_power(), 4631.25, 1e-6));
    let golem = Setup {
        lasers: vec![pitman],
        gadgets: vec![],
    };
    let el = |n: &str| find(&elements, &name, n).map(|e| e.guid);
    let entries = |parts: &[(&str, f32)]| -> Result<Vec<Entry>, String> {
        parts
            .iter()
            .map(|&(n, pct)| {
                Ok(Entry {
                    element: el(n)?,
                    fraction: pct / 100.0,
                    quality: None,
                })
            })
            .collect()
    };
    let by_scu = |parts: &[(&str, f32)], scu: f32| -> Result<Rock, String> {
        Rock::from_volume(ship, &elements, &entries(parts)?, ship.volume_from_scu(scu))
            .ok_or("no density".into())
    };

    // Rock A.
    let a = by_scu(
        &[
            ("Corundum_Raw", 8.56),
            ("Aluminium_Ore", 62.66),
            ("Aluminium_Ore", 4.89),
        ],
        35.71,
    )?;
    let a_mined = ModifiedRock::new(ship, &a, &golem);
    assert!(close(a.mass, 12281.0, 2e-3), "A mass {}", a.mass);
    assert!(
        close(a.window.size(), 0.100, 1e-4),
        "A window {:?}",
        a.window
    );
    assert!(
        close(a_mined.instability, 5.756, 2e-3),
        "A inst {}",
        a_mined.instability
    );

    // Rock B.
    let b = by_scu(
        &[
            ("Corundum_Raw", 5.62),
            ("Aluminium_Ore", 60.46),
            ("Aluminium_Ore", 4.66),
        ],
        36.60,
    )?;
    let b_mined = ModifiedRock::new(ship, &b, &golem);
    // B's mass (11,986 t on the scanner) is not checked: 0.7 % off with the
    // transcribed aluminium / inert split, while its instability (volume and
    // corundum only) matches.
    assert!(
        close(b.instability, 2.868, 2e-3),
        "B inst {}",
        b.instability
    );
    assert!(
        close(b_mined.instability, 3.872, 2e-3),
        "B inst {}",
        b_mined.instability
    );
    assert!(
        close(b_mined.window.size(), 0.194, 2e-3),
        "B window {:?}",
        b_mined.window
    );

    // Beryl.
    let beryl = by_scu(&[("Beryl_Raw", 5.93), ("Beryl_Raw", 56.17)], 28.18)?;
    let beryl_mined = ModifiedRock::new(ship, &beryl, &golem);
    assert!(close(beryl.mass, 8403.0, 3e-3), "Beryl mass {}", beryl.mass);
    assert_eq!(percent(beryl_mined.resistance), 65);
    assert!(
        close(beryl_mined.instability, 230.69, 2e-3),
        "Beryl inst {}",
        beryl_mined.instability
    );
    assert!(
        close(beryl_mined.max_mass(ship), 7980.0, 2e-3),
        "Beryl limit {}",
        beryl_mined.max_mass(ship)
    );
    assert!(!beryl_mined.is_breakable(ship));

    // Aslarite.
    let aslarite = by_scu(
        &[
            ("Aslarite_Raw", 4.93),
            ("Aslarite_Raw", 68.94),
            ("Agricium_Ore", 2.56),
            ("Titanium_Ore", 2.24),
        ],
        20.52,
    )?;
    let aslarite_mined = ModifiedRock::new(ship, &aslarite, &golem);
    assert!(
        close(aslarite.mass, 6445.0, 2e-3),
        "Aslarite mass {}",
        aslarite.mass
    );
    assert_eq!(percent(aslarite.resistance), 48);
    assert_eq!(percent(aslarite_mined.resistance), 60);
    assert!(
        close(aslarite.instability, 301.19, 2e-3),
        "Aslarite inst {}",
        aslarite.instability
    );
    assert!(
        close(aslarite_mined.instability, 406.61, 2e-3),
        "Aslarite inst {}",
        aslarite_mined.instability
    );

    // Mort13's chart.
    let mut helix = laser_named("Mining_Laser_THCN_Helix_S2")?;
    let surge = module("Mining_Modules_Active_Surge")?;
    helix.modules = vec![surge.clone(), surge.clone(), surge];
    let helix = Setup {
        lasers: vec![helix],
        gadgets: vec![],
    };
    let bare = |resistance| Rock {
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
    let r0 = ModifiedRock::new(ship, &bare(0.0), &helix).max_mass(ship);
    let r100 = ModifiedRock::new(ship, &bare(1.0), &helix).max_mass(ship);
    assert!(close(r0, 83193.75, 1e-5), "Helix II {r0}");
    assert!(
        close(r100 / r0, 0.5776, 1e-3),
        "Helix II at 100 % {}",
        r100 / r0
    );
    println!("asserts passed: rocks A, B, Beryl, Aslarite; Mort13 Helix II\n");

    for (label, rock) in [
        ("A", &a),
        ("B", &b),
        ("Beryl", &beryl),
        ("Aslarite", &aslarite),
    ] {
        report(label, ship, rock, &golem);
    }
    Ok(())
}

fn report(label: &str, ruleset: &Ruleset, rock: &Rock, setup: &Setup) {
    let mined = ModifiedRock::new(ruleset, rock, setup);
    println!(
        "{label}: {:.0} t, {:.0} m3, {:.2} SCU | R {:.3} -> {:.3}, I {:.2} -> {:.2}, window {:.3}..{:.3} -> {:.3}..{:.3}",
        rock.mass,
        rock.volume,
        rock.scu,
        rock.resistance,
        mined.resistance,
        rock.instability,
        mined.instability,
        rock.window.low,
        rock.window.high,
        mined.window.low,
        mined.window.high
    );
    println!(
        "  limit {:.0} t, balance throttle {:?}, to window {:?} s, in window {:?} s, overcharges at min {}",
        mined.max_mass(ruleset),
        mined.balance_throttle(ruleset),
        mined.time_to_window_s(ruleset),
        mined.time_in_window_s(ruleset),
        mined.overcharges_at_minimum(ruleset)
    );
    if mined.is_breakable(ruleset) {
        let d = difficulty(
            &mined,
            ruleset,
            &Player::default(),
            &SimConfig::default(),
            200,
        );
        println!("  difficulty {d:?}");
    }
}

/// Whole percent, truncated, as the scanner shows resistance.
fn percent(x: f32) -> i32 {
    (x * 100.0 + 1e-4).floor() as i32
}

fn close(a: f32, b: f32, tolerance: f32) -> bool {
    (a - b).abs() <= tolerance * b.abs().max(1e-6)
}
