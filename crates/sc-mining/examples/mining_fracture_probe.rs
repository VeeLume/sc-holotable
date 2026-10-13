//! Re-proves the fracture split on the iron asteroid Valerie fractured on
//! 2026-10-13: asteroid_03 at scale 0.6394, iron 13.51 % (quality 664) +
//! 67.01 % (325), inert 19.46 %. Its six pieces came out nearly pure: one
//! 100 % inert, the others 95.9..100 % iron. The client's random stream is
//! process-wide, so the check is statistical: every element's volume is
//! conserved exactly, and fractures like the observed one are common.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_fracture_probe
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, Mesh, RecordCollection};
use sc_mining::{Combined, Elements, Entry, Rock, Rulesets, Shape, Spawn};
use sc_resources::Resources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
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
    let iron = elements
        .iter()
        .find(|(g, _)| name(g) == "Iron_Ore")
        .map(|(g, _)| *g)
        .ok_or("iron")?;

    let path = "objects/gameplay/mining/asteroids/asteroid_03.cgf";
    let scale = 0.6394;
    let shape = Shape::from_mesh(path, &Mesh::read(&assets, path)?)
        .ok_or("shape")?
        .scaled(scale);
    let entries = [
        Entry {
            element: iron,
            fraction: 0.1351,
            quality: Some(664),
        },
        Entry {
            element: iron,
            fraction: 0.6701,
            quality: Some(325),
        },
    ];
    let rock = Rock::from_volume(ship, &elements, &entries, shape.volume).ok_or("rock")?;
    let parent = Spawn { scale, shape, rock };

    let runs = 20_000;
    let (mut pure_inert, mut all_near_pure) = (0u32, 0u32);
    for seed in 0..runs {
        let pieces = parent.fracture(ship, &elements, Combined::NONE, seed);
        assert_eq!(pieces.len(), 6);
        let total: f32 = pieces.iter().map(|p| p.rock.volume).sum();
        for (k, e) in entries.iter().enumerate() {
            let poured: f32 = pieces
                .iter()
                .map(|p| {
                    p.rock
                        .entries
                        .iter()
                        .filter(|x| x.quality == e.quality)
                        .map(|x| x.fraction * p.rock.volume)
                        .sum::<f32>()
                })
                .sum();
            assert!(
                (poured - e.fraction * total).abs() < 0.05,
                "entry {k}: {poured} vs {}",
                e.fraction * total
            );
        }
        let iron_share = |p: &Spawn| p.rock.entries.iter().map(|x| x.fraction).sum::<f32>();
        if pieces.iter().any(|p| iron_share(p) < 0.01) {
            pure_inert += 1;
        }
        if pieces
            .iter()
            .all(|p| iron_share(p) < 0.01 || iron_share(p) > 0.9)
        {
            all_near_pure += 1;
        }
        if seed < 3 {
            let rows: Vec<String> = pieces
                .iter()
                .map(|p| {
                    let parts: Vec<String> = p
                        .rock
                        .entries
                        .iter()
                        .map(|x| format!("{:.1}%q{}", x.fraction * 100.0, x.quality.unwrap_or(0)))
                        .collect();
                    format!("{:.0} m3 [{}]", p.rock.volume, parts.join(" "))
                })
                .collect();
            println!("sample: {}", rows.join(" | "));
        }
    }
    let share = |n: u32| n as f32 / runs as f32 * 100.0;
    println!(
        "{runs} fractures: a pure-inert piece in {:.1} %, every piece pure iron or pure inert (as observed) in {:.1} %",
        share(pure_inert),
        share(all_near_pure)
    );
    assert!(share(pure_inert) > 10.0 && share(all_near_pure) > 5.0);
    println!("asserts passed: conservation, observed pattern is common");
    Ok(())
}
