//! Compares [`Bodies`] (and the other system-level containers) with
//! starmap.space's container list, joined by the placement's
//! `EntityCryGUID` (starmap.space's `GUID`).
//!
//! Fetch the lists once (filtered per system, cached; see
//! <https://starmap.space/api/docs.php> for the fair-use note) and pass them:
//!
//! ```bash
//! curl -A "sc-holotable-probe/1.0" -o oc_Stanton.json "https://starmap.space/api/v4/oc/index.php?system=Stanton"
//! curl -A "sc-holotable-probe/1.0" -o oc_Pyro.json "https://starmap.space/api/v4/oc/index.php?system=Pyro"
//! cargo run -p sc-locations --release --example starmap_bodies -- oc_Stanton.json oc_Pyro.json
//! ```
//!
//! Prints, per container: position difference, the bearing about the system
//! origin on both sides, and for bodies radius, OM radius, day length.
//! Asserts that at least 90 % join and every joined container sits within
//! 1e-6 of its distance (LIVE 4.10: 92 of 94, worst ~1e-8). Pyro only
//! agrees with its root rotation applied (−85.23°, `pyrostar` in
//! `pyrosystem.xml`).

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
use sc_locations::{Bodies, Locations, ObjectContainers};
use serde_json::Value;

fn num(v: &Value, key: &str) -> Option<f64> {
    match v.get(key)? {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<String> = std::env::args().skip(1).collect();
    if files.is_empty() {
        return Err("pass the starmap.space oc_*.json files".into());
    }
    let mut theirs: HashMap<String, Value> = HashMap::new();
    for f in &files {
        let list: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(f)?)?;
        for c in list {
            if let Some(g) = c.get("GUID").and_then(Value::as_str) {
                theirs.insert(g.to_ascii_lowercase(), c);
            }
        }
    }

    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let locations = Locations::build(datacore.records());
    let bodies = Bodies::build(&assets, &locations)?;
    let systems = ObjectContainers::systems(&assets, 0)?;

    let bearing = |x: f64, y: f64| y.atan2(x).to_degrees().rem_euclid(360.0);
    let mut rows = Vec::new();
    let mut matched = 0;
    let mut worst: f64 = 0.0;
    for (id, p) in systems.iter() {
        if !p.socpak().ends_with("system.socpak") {
            continue;
        }
        let Some(c) = theirs.get(p.entity_guid()) else {
            continue;
        };
        let Some(global) = systems.global_position(id) else {
            continue;
        };
        matched += 1;
        let t = [
            num(c, "XCoord").unwrap_or(f64::NAN),
            num(c, "YCoord").unwrap_or(f64::NAN),
            num(c, "ZCoord").unwrap_or(f64::NAN),
        ];
        let off =
            ((global[0] - t[0]).powi(2) + (global[1] - t[1]).powi(2) + (global[2] - t[2]).powi(2))
                .sqrt();
        let name = c
            .get("ObjectContainer")
            .and_then(Value::as_str)
            .unwrap_or("?");
        let system = c.get("System").and_then(Value::as_str).unwrap_or("?");
        // Relative: a float32 quaternion leaves ~1e-8 of the distance.
        let dist = global.iter().map(|v| v * v).sum::<f64>().sqrt();
        worst = worst.max(off / dist.max(1.0));
        let body = p.starmap_guid().and_then(|g| bodies.get(&g));
        let extra = body.map_or(String::new(), |b| {
            format!(
                "  r {:>8.0}/{:<8.0} OM {:>9.0}/{:<9.0} day {:.2}/{:.2}",
                b.radius,
                num(c, "BodyRadius").unwrap_or(f64::NAN),
                b.om_radius().unwrap_or(f64::NAN),
                num(c, "OrbitalMarkerRadius").unwrap_or(f64::NAN),
                b.rotation_period_h,
                num(c, "RotationSpeedX").unwrap_or(f64::NAN),
            )
        });
        rows.push((
            system.to_string(),
            name.to_string(),
            format!(
                "{system:<8} {name:<28} off {:>15.0} m  bearing {:>7.2}/{:<7.2}{extra}",
                off,
                bearing(global[0], global[1]),
                bearing(t[0], t[1]),
            ),
        ));
    }
    rows.sort();
    for (_, _, line) in &rows {
        println!("{line}");
    }
    println!(
        "\nmatched {matched} of {} starmap.space containers, worst offset {worst:.1e} of the distance",
        theirs.len()
    );
    assert!(
        matched * 10 >= theirs.len() * 9,
        "joined too few containers"
    );
    assert!(
        worst < 1e-6,
        "a container sits elsewhere: {worst:.1e} of its distance"
    );
    Ok(())
}
