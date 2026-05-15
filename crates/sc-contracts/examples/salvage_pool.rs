//! Find the salvage-target ECD pool. Hypothesis: the wreck ships
//! (C.O. Nomad, Drake Cutter, MISC Hull A, Aegis Gladius, RSI Zeus
//! Mk II ES, Aegis Retaliator, Crusader Mercury Star Runner) live as
//! `EntityClassDefinition`s tagged with `Missions ▸ MissionType ▸
//! Salvage ▸ AvailableToSalvage` (78 carriers per the role_investigation
//! audit). The slot query asks for that tag plus a size filter (Small /
//! Medium / Large / Huge) plus Legal / power-down state. If the
//! salvage ECDs aren't *also* tagged with the size+legality the slot
//! demands, we'll see the entity pool here but candidates=0 in the
//! contract dump — same data, different lens.
//!
//! Output:
//!   1. Every ECD whose tag set contains AvailableToSalvage, with its
//!      display name + full tag list. Lets us see the actual pool.
//!   2. Cross-reference: which size tags / legality tags ARE on these
//!      entities, so we can compare against what the slot demands.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example salvage_pool
//! ```

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let snap = datacore.snapshot();
    let store = datacore.records();
    let pools = &store.pools;
    let ecd_map = &store.records.multi_feature.entity_class_definition;
    let tree = &snap.tag_tree;
    let localized_items = &snap.localized_items;
    let locale = &asset_data.locale;

    let needle = "AvailableToSalvage";
    let needle_guids: Vec<Guid> = tree.by_name(needle).to_vec();
    println!(
        "=== AvailableToSalvage tag GUIDs ({}) ===",
        needle_guids.len()
    );
    for g in &needle_guids {
        let path = tree.path(g);
        println!("  {} :: {}", g, path.join(" ▸ "));
    }
    println!();

    // Walk every EntityClassDefinition record. Capture those whose tag
    // set contains an AvailableToSalvage-rooted tag (the literal
    // AvailableToSalvage GUID OR any descendant — Small / Medium /
    // Large / Huge etc. live under it).
    let salvage_root = needle_guids.first().copied();
    let salvage_descendants: HashSet<Guid> = match salvage_root {
        Some(root) => tree
            .descendants(&root)
            .map(|n| n.guid)
            .chain(std::iter::once(root))
            .collect(),
        None => HashSet::new(),
    };
    println!(
        "  AvailableToSalvage subtree (root + descendants): {} tag(s)",
        salvage_descendants.len()
    );
    println!();

    // Header for entity dump
    println!("=== ECDs whose tag set intersects the AvailableToSalvage subtree ===");
    let mut hits: Vec<(String, String, Guid, Vec<String>)> = Vec::new();
    for (&guid, handle) in ecd_map.iter() {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let intersects = ecd.tags.iter().any(|t| salvage_descendants.contains(t));
        if !intersects {
            continue;
        }
        // Resolve display name + collect full tag-name list.
        let display = localized_items
            .name_key(&guid)
            .and_then(|k| locale.resolve(k))
            .map(String::from)
            .unwrap_or_default();
        let record_name = datacore
            .db()
            .record(&guid)
            .and_then(|r| r.name().map(String::from))
            .unwrap_or_else(|| "<no name>".into());
        let mut tag_paths: Vec<String> = ecd
            .tags
            .iter()
            .filter_map(|t| {
                let path = tree.path(t);
                if path.is_empty() {
                    None
                } else {
                    Some(path.join(" ▸ "))
                }
            })
            .collect();
        tag_paths.sort();
        hits.push((record_name, display, guid, tag_paths));
    }
    hits.sort_by(|a, b| a.0.cmp(&b.0));
    println!(
        "  {} ECD(s) tagged AvailableToSalvage (or a descendant)\n",
        hits.len()
    );

    for (rec, display, _guid, tags) in &hits {
        println!("──────────────────────────────────────────────────");
        println!("  record:  {rec}");
        println!("  display: {display}");
        println!("  tags ({}):", tags.len());
        for t in tags {
            println!("    {t}");
        }
        println!();
    }

    // Now compute the cross-reference: of these salvage-target ECDs,
    // which slot-relevant tags do they actually carry? Specifically
    // compare to what the Adagio Lawful Salvage VeryEasy slot demands:
    //   AvailableToSalvage + Small + Legal + DisablePowerInteractions +
    //   EngineOff + IgnoreCrimes + IgnoreHostility + ItemPortsUnlocked +
    //   PoweredOff + Full Cargo + General + LowValue
    println!("=== Slot-tag coverage across salvage-target ECDs ===");
    let slot_tags: &[&str] = &[
        "AvailableToSalvage",
        "Small",
        "Medium",
        "Large",
        "Huge",
        "Legal",
        "Mixed",
        "DisablePowerInteractions",
        "EngineOff",
        "PoweredOff",
        "ItemPortsUnlocked",
        "IgnoreCrimes",
        "IgnoreHostility",
        "Full Cargo",
        "Half Cargo",
        "Empty Cargo",
        "LowValue",
        "MediumValue",
        "HighValue",
        "General",
    ];
    let mut coverage: HashMap<&str, usize> = HashMap::new();
    let total = hits.len();
    for (_, _, guid, _) in &hits {
        let Some(handle) = ecd_map.get(guid) else {
            continue;
        };
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let names: BTreeSet<String> = ecd
            .tags
            .iter()
            .filter_map(|t| tree.get(t).map(|n| n.name.clone()))
            .collect();
        for needle in slot_tags {
            if names.contains(*needle) {
                *coverage.entry(*needle).or_default() += 1;
            }
        }
    }
    let mut entries: Vec<_> = coverage.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    println!("  total salvage-target ECDs: {total}");
    for (tag, count) in &entries {
        println!("    {count:>3}/{total}  {tag}");
    }
    let zero: Vec<&&str> = slot_tags
        .iter()
        .filter(|t| !coverage.contains_key(*t))
        .collect();
    if !zero.is_empty() {
        println!("    --- not present on any salvage ECD: ---");
        for tag in zero {
            println!("    0/{total}  {tag}");
        }
    }

    // Also: let's count by size tier for clarity.
    println!();
    println!("=== Size-tier distribution among salvage-target ECDs ===");
    let mut by_size: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (rec, _disp, guid, _) in &hits {
        let Some(handle) = ecd_map.get(guid) else {
            continue;
        };
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let mut size = String::from("(no size tag)");
        for t in &ecd.tags {
            let path = tree.path(t);
            // Look for `... AvailableToSalvage ▸ <Size>`
            if path.windows(2).any(|w| w[0] == "AvailableToSalvage") {
                size = path.last().unwrap().to_string();
                break;
            }
        }
        by_size.entry(size).or_default().push(rec.clone());
    }
    for (size, recs) in &by_size {
        println!("  {size}: {} entities", recs.len());
        for rec in recs.iter().take(8) {
            println!("    - {rec}");
        }
        if recs.len() > 8 {
            println!("    ... +{} more", recs.len() - 8);
        }
    }
    let _ = (asset_data, snap, tree); // silence

    Ok(())
}
