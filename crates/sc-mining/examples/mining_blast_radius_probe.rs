//! Re-proves the fracture explosion ([`sc_mining::FractureBlast`]) against
//! the LIVE data and the two rocks captured on 2026-10-17 (Golem, Pitman,
//! ArcCorp moons): the Laranite rock (6,393 t) showed the HUD's striped blast
//! zone on the distance gauge, the copper rock (13,176 t) none. The client
//! (LIVE 4.10, `FUN_14441f220` for the HUD, `FUN_144511510` for the
//! explosion every fracture queues) scales `defaultExplosionParams` by the
//! rock's volume, composition, shatter modifier and danger pool.
//!
//! Prints the ship ruleset's blast constants, every element's
//! `elementExplosionMultiplier`, the shatter modifiers on lasers, modules and
//! gadgets, and both rocks' explosion over the danger pool.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_blast_radius_probe
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_mining::{MiningGadgets, MiningModules};
use sc_items_ship_weapons::ShipWeapons;
use sc_mining::{Combined, Elements, Entry, Rock, Rulesets};
use sc_resources::Resources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let resources = Resources::build(datacore.records());
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
        .ok_or("ship ruleset")?;
    println!(
        "MiningGlobalParamsShip: defaultVolume {} m3, dangerPoolFactor {}, {:?}",
        ship.default_volume, ship.danger_pool_factor, ship.blast
    );
    assert_eq!(ship.danger_pool_factor, 560.0);
    assert_eq!((ship.blast.min_radius, ship.blast.max_radius), (0.1, 0.5));
    assert_eq!(ship.blast.damage.physical, 1.0);

    println!("\nelementExplosionMultiplier:");
    let mut rows: Vec<_> = elements.iter().map(|(g, e)| (name(g), e)).collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    for (n, e) in &rows {
        println!("  {n:<40} {:>8.3}", e.explosion_multiplier);
    }
    let element = |record: &str| {
        elements
            .iter()
            .find(|(g, _)| name(g) == record)
            .map(|(g, _)| *g)
            .ok_or_else(|| format!("no element {record}"))
    };
    let entry = |element: Guid, pct: f32, quality: u16| Entry {
        element,
        fraction: pct / 100.0,
        quality: Some(quality),
    };
    let (laranite, tungsten) = (element("Laranite_Raw")?, element("Tungsten_Ore")?);
    let (tin, copper) = (element("Tin_Ore")?, element("Copper_Ore")?);
    let captures = [
        (
            "Laranite 6,393 t (stripes seen)",
            6393.0,
            vec![
                entry(laranite, 7.27, 510),
                entry(laranite, 73.05, 298),
                entry(tungsten, 7.06, 530),
            ],
            40.8,
        ),
        (
            "Copper 13,176 t (no stripes)",
            13176.0,
            vec![
                entry(tin, 6.84, 537),
                entry(copper, 11.50, 593),
                entry(copper, 72.34, 359),
            ],
            0.0,
        ),
    ];
    for (label, mass, entries, expected) in &captures {
        let rock = Rock::from_mass(ship, &elements, entries, *mass).ok_or("rock")?;
        println!(
            "\n{label}: V {:.0} m3, instability {:.2}, explosionMultiplier {:.3}",
            rock.volume, rock.instability, rock.explosion_multiplier
        );
        for pool in [0.0f32, 0.25, 0.5, 1.0] {
            let blast = rock.fracture_blast(ship, Combined::NONE, pool);
            println!(
                "  danger pool {pool:.2}: radius {:.1}-{:.1} m, damage {:.1} physical \
                 (item at 20 / 40 m: {:.1} / {:.1}), pressure {:.0}",
                blast.radius_min,
                blast.radius_max,
                blast.damage.physical,
                blast.damage_at(20.0).physical,
                blast.damage_at(40.0).physical,
                blast.pressure
            );
        }
        let clean = rock.fracture_blast(ship, Combined::NONE, 0.0);
        assert!(
            (clean.radius_max - expected).abs() < 0.1,
            "{label}: {clean:?}"
        );
    }

    let items = Items::build(datacore.records());
    let weapons = ShipWeapons::build(&datacore, &items);
    let modules = MiningModules::build(&datacore, &items);
    let gadgets = MiningGadgets::build(&datacore, &items);
    println!("\nshatterdamageModifier on lasers / modules / gadgets:");
    for (g, w) in weapons.iter() {
        if let Some(s) = w.mining.as_ref().and_then(|m| m.modifiers.shatter_damage) {
            println!("  laser  {:<40} {:?} {}", name(g), s.kind, s.value);
        }
    }
    for (g, m) in modules.iter() {
        if let Some(s) = m.modifiers.shatter_damage {
            println!("  module {:<40} {:?} {}", name(g), s.kind, s.value);
        }
    }
    for (g, m) in gadgets.iter() {
        if let Some(s) = m.modifiers.shatter_damage {
            println!("  gadget {:<40} {:?} {}", name(g), s.kind, s.value);
        }
    }
    println!("\nasserts passed: ship blast constants, Laranite 40.8 m, copper 0 m");
    Ok(())
}
