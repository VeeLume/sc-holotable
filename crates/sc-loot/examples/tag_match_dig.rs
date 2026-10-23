//! Does a loot selector's tag match an item that carries the tag itself, or
//! also one carrying a descendant tag? Counts, per selector term of both
//! generations, the items each rule matches. LIVE 4.10: 65 terms pick no
//! item exactly but do through the tree ("Sustenance" 0 / 282), so the
//! rule is the tree.
//!
//! ```bash
//! cargo run -p sc-loot --release --example tag_match_dig
//! ```

use std::collections::{HashMap, HashSet};

use sc_extract::generated::*;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_tags::Tags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let store = datacore.records();
    let pools = &store.pools;
    let items = Items::build(store);
    let tags = Tags::build(store);
    let ecds = &store.records.multi_feature.entity_class_definition;
    let item_tags: HashMap<Guid, &[Guid]> = items
        .guids()
        .filter_map(|g| Some((*g, &ecds.get(g)?.get(pools)?.tags[..])))
        .collect();
    println!(
        "{} items, {} with tags",
        items.len(),
        item_tags.values().filter(|t| !t.is_empty()).count()
    );

    // Every selector term: (generation, name, positive, negative).
    let mut terms: Vec<(&str, String, Vec<Guid>, Vec<Guid>)> = Vec::new();
    for h in store
        .records
        .multi_feature
        .loot_archetype_v3_record
        .values()
    {
        let Some(LootArchetypeV3_NoRefPtr::LootArchetypeV3(a)) =
            h.get(pools).and_then(|r| r.loot_archetype.as_ref())
        else {
            continue;
        };
        for e in a
            .get(pools)
            .iter()
            .flat_map(|a| &a.entries)
            .filter_map(|e| e.get(pools))
        {
            if let Some(LootArchetypeV3Selector_BasePtr::LootArchetypeV3Selector_Tags(t)) =
                &e.selector
                && let Some(term) = t.get(pools).and_then(|t| t.tags).and_then(|t| t.get(pools))
            {
                terms.push((
                    "v3",
                    e.name.clone(),
                    term.positive_tags.clone(),
                    term.negative_tags.clone(),
                ));
            }
        }
    }
    for h in store.records.multi_feature.loot_archetype.values() {
        let Some(a) = h.get(pools) else { continue };
        let excluded: Vec<Guid> = a
            .excluded_tags
            .and_then(|t| t.get(pools))
            .map(|t| t.tags.clone())
            .unwrap_or_default();
        let primary = a.primary_or_group.and_then(|g| g.get(pools));
        for e in primary
            .iter()
            .flat_map(|g| &g.entries)
            .filter_map(|e| e.get(pools))
        {
            let mut pos: Vec<Guid> = e.tag.into_iter().collect();
            let mut neg = excluded.clone();
            if let Some(extra) = e.additional_tags.and_then(|t| t.get(pools)) {
                pos.extend(&extra.positive_tags);
                neg.extend(&extra.negative_tags);
            }
            terms.push(("v2", e.name.clone(), pos, neg));
        }
    }
    println!("{} selector terms", terms.len());

    let direct = |t: &[Guid], want: &Guid| t.contains(want);
    let below = |t: &[Guid], want: &Guid| {
        t.iter()
            .any(|x| x == want || tags.is_descendant_of(x, want))
    };
    let mut stats: HashMap<(&str, &str), usize> = HashMap::new();
    let mut shown = 0;
    let mut seen = HashSet::new();
    for (generation, name, pos, neg) in &terms {
        if pos.is_empty() || !seen.insert((pos.clone(), neg.clone())) {
            continue;
        }
        let count = |m: &dyn Fn(&[Guid], &Guid) -> bool| {
            item_tags
                .values()
                .filter(|t| pos.iter().all(|p| m(t, p)) && !neg.iter().any(|n| m(t, n)))
                .count()
        };
        let (d, b) = (count(&direct), count(&below));
        let class = match (d > 0, b > d) {
            (true, false) => "same",
            (true, true) => "more below",
            (false, true) => "only below",
            (false, false) => "none",
        };
        *stats.entry((generation, class)).or_default() += 1;
        if class == "only below" && shown < 12 {
            shown += 1;
            let names: Vec<String> = pos.iter().map(|p| tags.path(p).join("/")).collect();
            println!("  only below: {generation} {name} {names:?} direct {d} below {b}");
        }
    }
    let mut stats: Vec<_> = stats.into_iter().collect();
    stats.sort();
    println!("distinct terms by (generation, rule outcome): {stats:?}");
    // 65 terms (14 v3, 51 v2) pick nothing exactly but do through the tree.
    let only_below = |g: &str| {
        stats
            .iter()
            .find(|((gen_, class), _)| *gen_ == g && *class == "only below")
            .map_or(0, |(_, n)| *n)
    };
    assert_eq!((only_below("v3"), only_below("v2")), (14, 51));
    Ok(())
}
