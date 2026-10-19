//! The RSI Constellation Mk V Centaurus's interior ([`Interior`]): the
//! object containers its record places (offsets in the ship frame) and the brushes
//! their `.soc` files position, checked for sense: every transform is a
//! rotation times a scale (orthogonal axes), every brush geometry file is
//! in the archive, and the brushes sit inside the hull's collision shell.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example interior_probe
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{GeometryPaths, HullGeometry, Interior, PortContainers, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let paths = GeometryPaths::build(&datacore);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let with = vehicles
        .values()
        .filter(|v| !v.object_containers.is_empty())
        .count();
    println!(
        "{with} of {} vehicles place object containers",
        vehicles.len()
    );

    let (_, ship) = vehicles
        .iter()
        .find(|(g, _)| names.get(g).map(String::as_str) == Some("RSI_Constellation_Mk5_Centaurus"))
        .ok_or("no Centaurus")?;
    for oc in &ship.object_containers {
        println!(
            "  {} at {:?} rot {:?} bone {:?}",
            oc.file, oc.position, oc.rotation, oc.bone
        );
    }
    let interior = Interior::read(&assets, ship);
    for (f, why) in &interior.unread {
        println!("  unread {f}: {why}");
    }
    let mut per: BTreeMap<usize, usize> = BTreeMap::new();
    for b in &interior.brushes {
        *per.entry(b.container).or_default() += 1;
    }
    println!("{} brushes: {per:?}", interior.brushes.len());

    // Rotation times scale.
    let (mut bent, mut scaled, mut mirrored) = (0, 0, 0);
    for b in &interior.brushes {
        let m = b.transform;
        let row = |r: usize| [m[r][0], m[r][1], m[r][2]];
        let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let (r0, r1, r2) = (row(0), row(1), row(2));
        let det = r0[0] * (r1[1] * r2[2] - r1[2] * r2[1]) - r0[1] * (r1[0] * r2[2] - r1[2] * r2[0])
            + r0[2] * (r1[0] * r2[1] - r1[1] * r2[0]);
        // Columns (the brush's axes in the ship frame) orthogonal.
        let col = |c: usize| [m[0][c], m[1][c], m[2][c]];
        let (c0, c1, c2) = (col(0), col(1), col(2));
        let len = |v: [f32; 3]| dot(v, v).sqrt();
        let tol = 1e-3 * len(c0).max(len(c1)).max(len(c2)).powi(2);
        let ortho = dot(c0, c1).abs() < tol && dot(c1, c2).abs() < tol && dot(c0, c2).abs() < tol;
        if (len(c0) - 1.0).abs() > 1e-3
            || (len(c1) - 1.0).abs() > 1e-3
            || (len(c2) - 1.0).abs() > 1e-3
        {
            scaled += 1;
        }
        if det < 0.0 {
            mirrored += 1;
        }
        let _ = (r0, r1, r2);
        if !ortho {
            bent += 1;
            if bent <= 5 {
                println!("  not a rotation: {} {m:?}", b.geometry);
            }
        }
    }
    println!("{scaled} scaled, {mirrored} mirrored, {bent} skewed (not rotation x scale)");
    assert_eq!(bent, 0);

    // Geometry files in the archive.
    let mut unique: Vec<&str> = interior
        .brushes
        .iter()
        .map(|b| b.geometry.as_str())
        .collect();
    unique.sort_unstable();
    unique.dedup();
    let missing: Vec<&&str> = unique
        .iter()
        .filter(|p| assets.read_data(p).is_err())
        .collect();
    println!(
        "{} distinct geometry files, {} not in the archive",
        unique.len(),
        missing.len()
    );
    assert!(missing.is_empty());
    assert!(interior.unread.is_empty(), "{:?}", interior.unread);
    for p in missing.iter().take(5) {
        println!("  missing {p}");
    }

    // Inside the hull.
    let hull =
        HullGeometry::read(&assets, ship, &ship.stock, &vehicles, &paths).ok_or("no hull")?;
    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
    for p in hull.collision.iter().flat_map(|m| &m.positions) {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let outside: Vec<_> = interior
        .brushes
        .iter()
        .filter(|b| {
            (0..3).any(|k| b.transform[k][3] < lo[k] - 1.0 || b.transform[k][3] > hi[k] + 1.0)
        })
        .collect();
    println!(
        "hull shell {lo:.1?}..{hi:.1?}; {} of {} brush origins outside it",
        outside.len(),
        interior.brushes.len()
    );
    assert!(outside.is_empty());
    for b in outside.iter().take(5) {
        println!(
            "  outside: {} at {:?}",
            b.geometry,
            [b.transform[0][3], b.transform[1][3], b.transform[2][3]]
        );
    }
    Ok(())
}
