//! Re-proves the projectile hit pipeline on the stock RSI Constellation Mk
//! V Centaurus (LIVE 4.10.197) against the cases worked by hand from the
//! client code (Firefly note "Projectile hit on a ship"):
//!
//! - full shields: a CF-557 shot (98.19 energy) is all taken by the face;
//!   a BEHR S3 shot (45 physical) leaves 15.19 on the face and passes 18.56;
//! - shields down, armour intact: both are deflected (380 / 531), 0 damage;
//! - an ESPR S5 cannon shot (950 physical): 665 past the armour, 538.65
//!   wear, 176.69 on the hull part, 0.036 m penetration;
//! - the same with the armour gone: 1.7 m penetration; no power plant lies
//!   within 1.7 m of the hull's collision shell (the nearest 1.73 m), the
//!   radar does (1.39 m) and takes 950 × (1 − s / 1.7) × the component
//!   multiplier × its resistance on its best axis;
//! - the collision shell is closed: vertical rays over the power-plant
//!   bays all hit it (the render mesh is open there), so no internal
//!   component reads 0 exposure.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example hits_probe
//! ```

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::{PerDamageType, ShipComponents};
use sc_vehicles::{
    ComponentHealths, Condition, DamageRules, Defense, GeometryPaths, HullGeometry, PortContainers,
    Raycaster, Shot, Target, Vehicles, apply,
};
use sc_weapons::{ShipWeapon, Weapons};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let components = ShipComponents::build(&datacore, &items);
    let healths = ComponentHealths::build(&datacore);
    let paths = GeometryPaths::build(&datacore);
    let weapons = Weapons::build(&datacore, &items);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let (_, ship) = vehicles
        .iter()
        .find(|(g, _)| names.get(g).map(String::as_str) == Some("RSI_Constellation_Mk5_Centaurus"))
        .ok_or("no Centaurus")?;
    let geometry =
        HullGeometry::read(&assets, ship, &ship.stock, &vehicles, &paths).ok_or("no hull")?;
    let raycaster = Raycaster::new(&geometry.collision);
    let defense = Defense::compute(ship, &ship.stock, &vehicles, &components);
    let target = Target {
        vehicle: ship,
        geometry: &geometry,
        raycaster: &raycaster,
        defense: &defense,
        items: &items,
        healths: &healths,
        rules: DamageRules::default(),
    };
    let gun = |n: &str| -> Result<&ShipWeapon, String> {
        weapons
            .ships
            .iter()
            .find(|w| w.record_name.ends_with(n))
            .ok_or(format!("no weapon {n}"))
    };
    let shot = |w: &ShipWeapon, origin: [f32; 3], at: [f32; 3]| -> Shot {
        let d = w.damage.as_ref();
        Shot {
            origin,
            direction: std::array::from_fn(|k| at[k] - origin[k]),
            damage: PerDamageType {
                physical: d.map_or(0.0, |d| d.physical),
                energy: d.map_or(0.0, |d| d.energy),
                distortion: d.map_or(0.0, |d| d.distortion),
                thermal: d.map_or(0.0, |d| d.thermal),
                biochemical: d.map_or(0.0, |d| d.biochemical),
                stun: d.map_or(0.0, |d| d.stun),
            },
            penetration: w
                .penetration_m
                .map(|p| (p, w.penetration_radius_m.unwrap_or([0.0; 2]))),
        }
    };
    let near = |a: f32, b: f32, tol: f32| (a - b).abs() <= tol;
    let cf557 = gun("KLWE_LaserRepeater_S5")?;
    let behr = gun("BEHR_BallisticRepeater_S3")?;
    let espr = gun("ESPR_BallisticCannon_S5")?;
    println!(
        "ESPR S5: {:?}, penetration {:?} m radius {:?}",
        espr.damage, espr.penetration_m, espr.penetration_radius_m
    );

    // A shot from 200 m ahead at the nose.
    let (from, to) = ([0.0, 200.0, 0.0], [0.0, 0.0, 0.0]);
    let full = Condition::default();
    let r = target.hit(&shot(cf557, from, to), &full);
    println!("\nCF-557, full shields: {r:?}");
    assert!(near(r.shield_taken, 98.19, 0.01) && r.passed.energy < 1e-3);
    let r = target.hit(&shot(behr, from, to), &full);
    println!(
        "BEHR S3, full shields: taken {} passed {:?}",
        r.shield_taken, r.passed.physical
    );
    assert!(near(r.shield_taken, 15.19, 0.01) && near(r.passed.physical, 18.5625, 0.01));

    let down = Condition {
        shield_ratio: 0.0,
        ..Condition::default()
    };
    for w in [cf557, behr] {
        let r = target.hit(&shot(w, from, to), &down);
        println!(
            "{} shields down: part {:?} hull {}",
            w.record_name, r.part, r.hull_damage
        );
        assert!(r.part.is_some() && r.hull_damage == 0.0 && r.armor_wear == 0.0);
    }
    let r = target.hit(&shot(espr, from, to), &down);
    println!("\nESPR S5, armour intact: {r:#?}");
    assert!(near(r.after_armor.physical, 665.0, 0.01));
    assert!(near(r.armor_wear, 538.65, 0.01));
    assert!(near(r.hull_damage, 176.69, 0.05), "hull {}", r.hull_damage);
    assert!(near(r.depth, 0.0356, 0.0005));

    // Armour gone: aim at each power plant from outside, along its port axis
    // directions, and report what the penetration reaches.
    let bare = Condition {
        armor_ratio: 0.0,
        shield_ratio: 0.0,
        ..Condition::default()
    };
    let mut checked = 0;
    for (i, pl) in geometry.placements.iter().enumerate() {
        if !names.get(&pl.item).is_some_and(|n| n.starts_with("POWR_")) {
            continue;
        }
        let Some(b) = pl.bounds else { continue };
        let centre = apply(
            &pl.transform,
            std::array::from_fn(|k| 0.5 * (b.min[k] + b.max[k])),
        );
        for dir in [
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ] {
            let origin = std::array::from_fn(|k| centre[k] - dir[k] * 100.0);
            let r = target.hit(&shot(espr, origin, centre), &bare);
            let reached: Vec<String> = r
                .components
                .iter()
                .map(|c| {
                    format!(
                        "{} @ {:.2} m: {:.1}",
                        names.get(&c.item).map_or("?", String::as_str),
                        c.depth,
                        c.damage
                    )
                })
                .collect();
            println!(
                "  {} from {dir:?}: part {:?}, depth {:.2} m, hull {:.1}, reached {reached:?}",
                pl.path,
                r.part.as_ref().map(|p| p.1.as_str()),
                r.depth,
                r.hull_damage
            );
            assert!(near(r.depth, 1.7, 1e-4));
            assert!(near(r.hull_damage, 950.0, 0.01));
            if let Some(c) = r.components.iter().find(|c| c.placement == i) {
                // On the axis (aimed at its centre): 950 x (1 - s/1.7) x 0.85.
                assert!(near(c.damage, 950.0 * (1.0 - c.depth / 1.7) * 0.85, 0.05));
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 0, "a power plant within 1.7 m of the shell");

    // The radar is within reach: on its best axis the penetration reaches it.
    let (ri, rpl) = geometry
        .placements
        .iter()
        .enumerate()
        .find(|(_, p)| names.get(&p.item).is_some_and(|n| n.starts_with("RADR_")))
        .ok_or("no radar")?;
    let b = rpl.bounds.ok_or("radar without bounds")?;
    let centre = apply(
        &rpl.transform,
        std::array::from_fn(|k| 0.5 * (b.min[k] + b.max[k])),
    );
    let best = target.exposure(ri, 2000).remove(0);
    assert!(
        best.depth > 1.3 && best.depth < 1.7,
        "radar depth {}",
        best.depth
    );
    let origin = std::array::from_fn(|k| centre[k] - best.direction[k] * 100.0);
    let r = target.hit(&shot(espr, origin, centre), &bare);
    let c = r
        .components
        .iter()
        .find(|c| c.placement == ri)
        .ok_or("the radar is not reached")?;
    let resist = healths
        .get(&rpl.item)
        .and_then(|h| h.damage_multiplier)
        .map_or(1.0, |m| m.physical);
    let want = 950.0 * (1.0 - c.depth / 1.7) * ship.penetration.component_multiplier * resist;
    println!(
        "radar at {:.2} m: {:.1} (950 x (1 - s/1.7) x {} x {resist} = {want:.1})",
        c.depth, c.damage, ship.penetration.component_multiplier
    );
    assert!(near(c.damage, want, 0.05));

    // Exposure: for the key components, the shallowest attack angles.
    println!("\nexposure (hull surface to component centre, 2000 directions):");
    for (i, pl) in geometry.placements.iter().enumerate() {
        let n = names.get(&pl.item).map_or("?", String::as_str);
        if !["POWR", "COOL", "SHLD", "QDRV", "RADR", "LFSP"]
            .iter()
            .any(|k| n.starts_with(k))
        {
            continue;
        }
        let e = target.exposure(i, 2000);
        let Some(best) = e.first() else { continue };
        assert!(best.depth > 0.0, "{} reads 0 exposure", pl.path);
        let reachable = e.iter().filter(|x| x.depth <= 1.7).count();
        println!(
            "  {:<32} {:<34} best {:.2} m from {:?} through {:?}; {reachable} of {} directions within 1.7 m",
            pl.path,
            n,
            best.depth,
            best.direction.map(|c| (c * 100.0).round() / 100.0),
            best.part,
            e.len()
        );
    }
    // From each power plant's centre outward along its shallowest direction:
    // how far to the first hull triangle, and which part.
    for (i, pl) in geometry.placements.iter().enumerate() {
        if !names.get(&pl.item).is_some_and(|n| n.starts_with("POWR_")) {
            continue;
        }
        let Some(b) = pl.bounds else { continue };
        let centre = apply(
            &pl.transform,
            std::array::from_fn(|k| 0.5 * (b.min[k] + b.max[k])),
        );
        let e = target.exposure(i, 2000);
        for x in e.iter().take(3) {
            let back = x.direction.map(|c| -c);
            let out = raycaster.cast(centre, back);
            println!(
                "  {} centre {centre:?} toward {:?}: first hull at {:?} m ({:?}); outward along +dir: {:?}",
                pl.path,
                back.map(|c| (c * 100.0).round() / 100.0),
                out.map(|h| (h.distance * 100.0).round() / 100.0),
                out.map(|h| geometry.collision[h.part].part.clone()),
                raycaster.cast(centre, x.direction).map(|h| (
                    (h.distance * 100.0).round() / 100.0,
                    geometry.collision[h.part].part.clone()
                ))
            );
        }
    }

    // Openings in the render mesh: vertical rays from below and above on a
    // 1 m grid around the power plants (x +-6..12, y -9..0).
    for (label, z0, dz) in [("from below", -50.0f32, 1.0f32), ("from above", 50.0, -1.0)] {
        let mut row = String::new();
        let mut misses = 0;
        for y in (-9..=0).rev() {
            for x in -12..=12 {
                let hit = raycaster.cast([x as f32, y as f32, z0], [0.0, 0.0, dz]);
                row.push(if hit.is_some() { '#' } else { '.' });
                misses += usize::from(hit.is_none());
            }
            row.push('\n');
        }
        println!(
            "\nvertical rays {label} (x -12..12 across, y 0..-9 down), {misses} misses:\n{row}"
        );
        assert_eq!(misses, 0, "the collision shell is open {label}");
    }
    println!(
        "asserts passed: shield, deflection, armour, hull reduction, penetration, closed shell"
    );
    Ok(())
}
