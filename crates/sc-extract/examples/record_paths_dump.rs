//! Validate [`RecordPaths`] against the live DCB: total count, the top-level
//! layout, and the two motivating subtrees (manufacturers + scitems) whose
//! path prefixes are the "classification axis" finding 3 is about.
//!
//! ```bash
//! cargo run -p sc-extract --release --example record_paths_dump --features full
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, RecordPaths};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;

    let paths = RecordPaths::build(&datacore);
    println!("RecordPaths entries : {}", paths.len());

    println!("-- top-level segments (roots) --");
    for r in paths.roots() {
        let n = paths.under(r).count();
        println!("  {n:>7}  {r}");
    }

    // Find the scitemmanufacturer prefix and break down its immediate children
    // — these child segments ARE the manufacturer "kind" (genuine vs paint vs
    // logo) that the deferred manufacturer-kind question wanted.
    let mfr_prefix = find_prefix(&paths, "scitemmanufacturer");
    if let Some(prefix) = &mfr_prefix {
        println!("-- {prefix} children (manufacturer kinds) --");
        for kind in paths.children(prefix) {
            let sub = format!("{prefix}/{kind}");
            println!("  {:>5}  {kind}", paths.under(&sub).count());
        }
    } else {
        println!("(no scitemmanufacturer prefix found)");
    }

    // scitem subtree: count leaves grouped by their category segment
    // (cooler, weapon, shield, ...) — the item-category win.
    let item_prefix = find_prefix(&paths, "scitem");
    if let Some(prefix) = &item_prefix {
        println!("-- {prefix} children (item categories, top 25) --");
        let mut by_cat: BTreeMap<String, usize> = BTreeMap::new();
        for cat in paths.children(prefix) {
            let sub = format!("{prefix}/{cat}");
            by_cat.insert(cat.to_string(), paths.under(&sub).count());
        }
        let mut v: Vec<_> = by_cat.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        for (cat, n) in v.iter().take(25) {
            println!("  {n:>6}  {cat}");
        }
    }

    // Spot-check round-trip: pick one manufacturer leaf, resolve guid -> ref,
    // confirm its path is addressable back via at().
    if let Some(prefix) = &mfr_prefix
        && let Some(first) = paths.under(prefix).next()
        && let Some(r) = paths.get(first)
    {
        println!("-- spot check --");
        println!("  guid    : {:?}", r.guid);
        println!("  name    : {}", r.name);
        println!("  type    : {:?}", paths.type_name(r.struct_index));
        println!("  is_main : {}", r.is_main);
        println!("  path    : {}", r.path);
        assert!(
            paths.at(&r.path).contains(&r.guid),
            "at(path) must round-trip"
        );
        println!("  at(path) round-trips ✓");
    }

    Ok(())
}

/// Find the first full trie path whose final segment equals `leaf` by a
/// breadth-first descent. Returns e.g.
/// `libs/foundry/records/scitemmanufacturer` for `"scitemmanufacturer"`.
fn find_prefix(paths: &RecordPaths, leaf: &str) -> Option<String> {
    let mut frontier: Vec<String> = paths.roots().map(String::from).collect();
    while !frontier.is_empty() {
        let mut next = Vec::new();
        for path in &frontier {
            let last = path.rsplit('/').next().unwrap_or(path);
            if last == leaf {
                return Some(path.clone());
            }
            for child in paths.children(path) {
                next.push(format!("{path}/{child}"));
            }
        }
        frontier = next;
    }
    None
}
