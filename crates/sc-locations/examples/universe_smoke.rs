//! Smoke test for the joined `Universe` API — logical StarMap + physical socpak
//! placements, merged. Cooks both, joins, exercises every lookup + the graph.
//!
//! cargo run -p sc-locations --example universe_smoke --release

use sc_extract::{AssetConfig, AssetData, AssetSource, RecordCollection};
use sc_locations::{Locations, ObjectContainers, Universe};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("warn").init();

    let install = sc_discovery::discover_primary()?;
    println!("-> {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let ad = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &ad)?;

    let star_map = Locations::build(datacore.records());
    let containers = ObjectContainers::cook(&assets)?;
    println!(
        "-> StarMap {} locations, ObjectContainers {} placements",
        star_map.len(),
        containers.len()
    );

    // Serde round-trip (the persist path): the deserialized graph rebuilds its
    // indices and must resolve the same mission CRC.
    let bytes = serde_json::to_vec(&containers)?;
    let restored: ObjectContainers = serde_json::from_slice(&bytes)?;
    assert_eq!(
        containers.by_mission_crc(2273524489),
        restored.by_mission_crc(2273524489),
        "serde round-trip changed by_mission_crc"
    );
    assert_eq!(restored.len(), containers.len());
    println!("-> serde round-trip OK ({} bytes)", bytes.len());
    let u = Universe::join(star_map, containers);
    let loc = &ad.locale;

    // ── roots (logical graph) ────────────────────────────────────────────
    println!("\nroots:");
    for r in u.roots() {
        if let Some(n) = r.display_name(loc) {
            println!("  {n}  [{:?}]", r.kind());
        }
    }

    // ── global-position verification ─────────────────────────────────────
    // Hurston (system-global body) vs Everus Harbor (body-local placement):
    // Everus's GLOBAL should land ~1 body-radius from Hurston's center.
    println!("\nglobal-position check:");
    let hurston = u
        .by_name(loc, "Hurston")
        .into_iter()
        .find(|p| matches!(p.kind(), Some(k) if format!("{k:?}") == "Planet"));
    if let Some(h) = &hurston {
        println!("  Hurston   global={:?}", h.global_position());
    }
    for p in u.by_name(loc, "Everus Harbor") {
        let g = p.global_position();
        let d = match (hurston.as_ref().and_then(|h| h.global_position()), g) {
            (Some(a), Some(b)) => Some(dist(a, b) / 1000.0),
            _ => None,
        };
        println!(
            "  {:<22} local={:?} global={:?} dist_to_Hurston={:?} km",
            p.display_name(loc).unwrap_or("?"),
            p.position().map(round),
            g.map(round),
            d.map(|x| x.round()),
        );
    }

    // ── mission DataSet CRC → name (ARC-L3 ground truth) ─────────────────
    println!("\nby_mission_crc(2273524489):");
    if let Some(p) = u.by_mission_crc(2273524489) {
        println!(
            "  display_name={:?} own_name={:?} pos={:?}",
            p.display_name(loc),
            p.name(loc),
            p.position()
        );
    }

    // ── logical children + anchor-position fallback ──────────────────────
    println!("\nHurston children (name → position / anchor):");
    if let Some(h) = u.by_name(loc, "Hurston").into_iter().next() {
        for c in u.children(&h) {
            println!(
                "  {:<26} pos={:?}  anchor={:?}",
                c.display_name(loc).unwrap_or("<unnamed>"),
                c.position(),
                c.anchor_position(),
            );
        }
    }

    Ok(())
}

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}
fn round(p: [f64; 3]) -> [f64; 3] {
    [p[0].round(), p[1].round(), p[2].round()]
}
