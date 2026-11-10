//! What a lootable's v3 secondary layers do to the items its table picks,
//! if a layer keeps only the items one of its entries matches (client
//! `MakeArchetypeDecisionTree`): per layer, whether it has a catch-all
//! entry (no positive tag), and how many picks survive all layers, testing
//! entries against the item's tags with ancestors and without. Asserts:
//! only the `V3SecondaryChoices_Rarity_*` layers lack a catch-all (so only
//! they drop items), no v2 config carries layers, and ancestors change
//! nothing.
//!
//! ```bash
//! cargo run -p sc-loot --release --example secondary_dig
//! ```

use std::collections::{BTreeMap, HashSet};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_loot::{Generation, LootTables, Lootables, Selector};
use sc_tags::Tags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let tags = Tags::build(datacore.records());
    let tables = LootTables::build(&datacore, &items, &tags);
    let lootables = Lootables::build(&datacore, &tables);
    let store = datacore.records();
    let own = |g: &Guid| -> HashSet<Guid> {
        store
            .records
            .multi_feature
            .entity_class_definition
            .get(g)
            .and_then(|h| h.get(&store.pools))
            .map(|e| e.tags.iter().copied().collect())
            .unwrap_or_default()
    };
    let mut layers: BTreeMap<String, (bool, usize)> = BTreeMap::new();
    let (mut configs, mut before, mut after_closure, mut after_own) = (0, 0usize, 0usize, 0usize);
    let mut v2_with_layers = 0;
    for (_, l) in lootables.iter() {
        for c in &l.configs {
            if c.secondary.is_empty() {
                continue;
            }
            for g in &c.secondary {
                let catch_all = g
                    .entries
                    .iter()
                    .any(|e| matches!(&e.selector, Selector::Tags(t) if t.positive.is_empty()));
                layers
                    .entry(g.name.clone())
                    .or_insert((catch_all, g.entries.len()));
            }
            let Some(t) = c.tables().find_map(|t| tables.get(&t)) else {
                continue;
            };
            if t.generation == Generation::V2 {
                v2_with_layers += 1;
                continue;
            }
            configs += 1;
            let picks: HashSet<&Guid> = t
                .archetypes
                .iter()
                .flat_map(|a| a.archetype.entries.iter().flat_map(|e| &e.items))
                .collect();
            before += picks.len();
            let survives = |i: &Guid, closure: bool| {
                let mine = own(i);
                c.secondary.iter().all(|g| {
                    g.entries.iter().any(|e| match &e.selector {
                        Selector::Tags(term) if closure => tables.selects(term, i),
                        Selector::Tags(term) => {
                            term.positive.iter().all(|p| mine.contains(p))
                                && !term.negative.iter().any(|n| mine.contains(n))
                        }
                        _ => true,
                    })
                })
            };
            after_closure += picks.iter().filter(|i| survives(i, true)).count();
            after_own += picks.iter().filter(|i| survives(i, false)).count();
        }
    }
    println!("layers (catch-all, entries):");
    for (n, (c, e)) in &layers {
        println!("  {n:<45} catch-all {c:<5} {e} entries");
    }
    println!(
        "{configs} v3 configs with layers ({v2_with_layers} v2 ones): picks {before}, \
         surviving with ancestor tags {after_closure}, with own tags {after_own}"
    );
    // Only the rarity layers lack a catch-all, so only they filter; their
    // entries use leaf tags, so ancestors make no difference.
    let filtering: Vec<&String> = layers
        .iter()
        .filter(|(_, (c, _))| !c)
        .map(|(n, _)| n)
        .collect();
    assert!(!filtering.is_empty());
    assert!(
        filtering
            .iter()
            .all(|n| n.starts_with("V3SecondaryChoices_Rarity_"))
    );
    assert_eq!(v2_with_layers, 0, "a v2 config carries layers: unused");
    assert!(after_closure < before);
    assert_eq!(after_closure, after_own);
    Ok(())
}
