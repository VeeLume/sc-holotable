//! Explore the `StarMapObject` hierarchy from the loose probe DCB.
//!
//! The `dcbraw/` XML export empties reference fields (parent/type), so the
//! relationships only survive in the binary `Game2.dcb`. This parses that via
//! an in-memory [`AssetSource`] and walks the typed [`sc_locations::Locations`]
//! hierarchy — correct kinds + parent/children — to answer "how is a system
//! laid out, and where do asteroid bases sit."
//!
//! ```bash
//! cargo run -p sc-locations --example dump_location_tree
//! cargo run -p sc-locations --example dump_location_tree -- 286cb603-b4ae-4279-80a1-d4505fee1916  # Pyro subtree
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};
use sc_locations::{Location, LocationKind, Locations};

const DCB: &str = "target/probe-resources/dcbfile/Data/Game2.dcb";
/// Pyro solar system (`starmap/pu/pyrosolarsystem.xml`).
const PYRO: &str = "286cb603-b4ae-4279-80a1-d4505fee1916";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read(DCB)?;
    let mut files = BTreeMap::new();
    files.insert("Data/Game2.dcb".to_string(), bytes);
    let assets = AssetSource::from_snapshot(files, "probe");
    let asset_data = AssetData::extract(&assets, &AssetConfig::minimal())?;
    let dc = Datacore::parse(&assets, &asset_data)?;
    let locs = Locations::build(dc.records());
    eprintln!("total locations: {}\n", locs.len());

    // ── System overview: every SolarSystem + its full subtree size ──────────
    println!("== solar systems (subtree size) ==");
    let mut systems: Vec<(&Guid, &Location)> = locs
        .iter()
        .filter(|(_, l)| matches!(l.kind, LocationKind::SolarSystem))
        .collect();
    systems.sort_by_key(|(_, l)| format!("{:?}", l.name_key));
    for (guid, loc) in &systems {
        let mut tally = BTreeMap::new();
        walk(&locs, guid, 0, usize::MAX, &mut tally, &mut 0);
        let total: usize = tally.values().sum();
        println!("  {:?}  ({total} in subtree)  {guid}", loc.name_key);
    }

    let _ = PYRO;

    // ── How is hierarchy actually encoded? parent populated or not? ─────────
    let with_parent = locs.iter().filter(|(_, l)| l.parent.is_some()).count();
    println!(
        "\n== parent linkage: {with_parent}/{} locations have a parent ==",
        locs.len()
    );

    // ── Enumerate locations by name substring (default "pyro") ──────────────
    let needle = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "pyro".to_string());
    let needle_lc = needle.to_lowercase();
    let mut hits: Vec<&Location> = locs
        .iter()
        .map(|(_, l)| l)
        .filter(|l| {
            format!("{:?}", l.name_key)
                .to_lowercase()
                .contains(&needle_lc)
        })
        .collect();
    hits.sort_by_key(|l| format!("{:?}", l.name_key));

    let mut tally: BTreeMap<String, usize> = BTreeMap::new();
    for l in &hits {
        *tally.entry(format!("{:?}", l.kind)).or_default() += 1;
    }
    println!(
        "\n== {} locations whose name contains \"{needle}\" — kind tally ==",
        hits.len()
    );
    let mut rows: Vec<(&String, &usize)> = tally.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1));
    for (kind, n) in rows {
        println!("  {n:>5}  {kind}");
    }

    // Structural skeleton: drop the bulk asteroid/outpost POIs to see the
    // planet/moon/lagrange spine and what each hangs off.
    let bulk = ["Asteroid", "AsteroidValidQt", "Outpost", "OutpostInvalidQt"];
    let mut skel: Vec<&&Location> = hits
        .iter()
        .filter(|l| !bulk.contains(&format!("{:?}", l.kind).as_str()))
        .collect();
    skel.sort_by_key(|l| (format!("{:?}", l.kind), format!("{:?}", l.name_key)));
    println!("\n== structural skeleton (non-asteroid/outpost): kind | name | parent ==");
    for l in skel {
        let parent = l
            .parent
            .and_then(|p| locs.get(&p))
            .map(|p| format!("{:?}", p.name_key))
            .unwrap_or_else(|| "—".to_string());
        println!("  {:?}  {:?}  parent={parent}", l.kind, l.name_key);
    }
    Ok(())
}

/// Pre-order walk: tally every descendant by kind; print the tree only down to
/// `print_depth` (with a line cap) so deep systems stay readable.
fn walk(
    locs: &Locations,
    guid: &Guid,
    depth: usize,
    print_depth: usize,
    tally: &mut BTreeMap<String, usize>,
    printed: &mut usize,
) {
    let Some(loc) = locs.get(guid) else { return };
    *tally.entry(format!("{:?}", loc.kind)).or_default() += 1;

    if depth <= print_depth && *printed < 400 {
        let kids = locs.children_of(guid).count();
        let suffix = if kids > 0 {
            format!(" [{kids} children]")
        } else {
            String::new()
        };
        println!(
            "{}{:?}  {:?}{suffix}",
            "  ".repeat(depth),
            loc.kind,
            loc.name_key
        );
        *printed += 1;
    }

    let mut kids: Vec<&Location> = locs.children_of(guid).collect();
    kids.sort_by_key(|c| format!("{:?}", c.name_key));
    for c in kids {
        walk(locs, &c.guid, depth + 1, print_depth, tally, printed);
    }
}
