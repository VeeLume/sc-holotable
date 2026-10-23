//! Re-proves the loot model on LIVE 4.10.
//!
//! - Both generations: 161 v2 `LootTable`s and 59 `LootTableV3Record`s.
//! - Only items tagged `LootGeneration.CanGenerateAsLoot` can be picked: the
//!   "Antiques" v3 archetype picks 20 collectibles tagged below it, and not
//!   the 12 chess pieces that carry its tag but not the loot one.
//! - Lootables by kind; every table they name exists but one (a test
//!   corpse names a table no record has); how many tables no lootable names.
//! - Entries whose selector picks no item, by generation.
//! - Secondary groups add no items: v2 ones are rarity bands (the picks are
//!   narrowed to items in one band of each), v3 lootable layers
//!   manufacturer, rarity and faction weights.
//! - The generic pool filter (no faction, no special-event items) narrows
//!   what the tables can drop.
//!
//! `--who <record>` lists the entries and spawn-withs that pick a record.
//!
//! ```bash
//! cargo run -p sc-loot --release --example loot_probe
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_loot::{LootTables, Lootables, PoolFilter, Selector};
use sc_tags::Tags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let tags = Tags::build(datacore.records());
    let tables = LootTables::build(&datacore, &items, &tags);
    let lootables = Lootables::build(&datacore, &tables);
    let db = datacore.db();
    let name = |g: &Guid| {
        db.record(g)
            .and_then(|r| r.name())
            .map(|n| n.split_once('.').map_or(n, |(_, n)| n).to_string())
            .unwrap_or_else(|| g.to_string())
    };

    // Tables by generation.
    let mut generations: BTreeMap<String, usize> = BTreeMap::new();
    for t in tables.values() {
        *generations
            .entry(format!("{:?}", t.generation))
            .or_default() += 1;
    }
    println!("tables: {generations:?}");
    assert_eq!(generations.get("V2"), Some(&161));
    assert_eq!(generations.get("V3"), Some(&59));

    // The Antiques archetype: medals, Hadesian fragments, currency bars and
    // physical scrip carry tags below it. The 12 chess pieces carry the tag
    // itself but not LootGeneration.CanGenerateAsLoot, so the client's loot
    // item database leaves them out and no table can drop them.
    let antiques: HashSet<&Guid> = tables
        .values()
        .flat_map(|t| &t.archetypes)
        .filter(|a| {
            a.archetype
                .record
                .is_some_and(|r| name(&r) == "V3LootArchetype_Antiques")
        })
        .flat_map(|a| a.archetype.entries.iter().flat_map(|e| &e.items))
        .collect();
    let chess = |g: &Guid| {
        name(g)
            .to_ascii_lowercase()
            .starts_with("loot_game_chess_1_")
    };
    let pieces = antiques.iter().filter(|g| chess(g)).count();
    println!(
        "Antiques picks {} items, {pieces} chess pieces",
        antiques.len()
    );
    assert_eq!((pieces, antiques.len()), (0, 20));
    assert!(
        antiques
            .iter()
            .all(|g| name(g).starts_with("Carryable_1H_"))
    );

    // Lootables by kind, and the tables they name.
    let mut kinds: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut named: HashSet<Guid> = HashSet::new();
    let mut dangling: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (guid, l) in lootables.iter() {
        let k = kinds.entry(format!("{:?}", l.kind)).or_default();
        k.0 += 1;
        k.1 += l.configs.len();
        for t in l.configs.iter().flat_map(|c| c.tables()) {
            if tables.get(&t).is_some() {
                named.insert(t);
            } else {
                dangling.entry(t.to_string()).or_default().push(name(guid));
            }
        }
    }
    println!("lootables (kind: records, configs): {kinds:?}");
    println!("tables no record has, and who names them: {dangling:?}");
    assert_eq!(kinds.get("Entity"), Some(&(140, 140)));
    assert_eq!(kinds.get("Harvestable"), Some(&(136, 2088)));
    assert_eq!(
        dangling.values().flatten().collect::<Vec<_>>(),
        ["Corpse_Male_Gravity_Lootable_TEST"]
    );
    let unnamed = tables.guids().filter(|g| !named.contains(g)).count();
    println!(
        "{} tables named by a lootable, {unnamed} by none",
        named.len()
    );

    // Entries that pick nothing.
    let mut outcomes: BTreeMap<(String, String), usize> = BTreeMap::new();
    for t in tables.values() {
        for e in t.archetypes.iter().flat_map(|a| &a.archetype.entries) {
            let kind = match &e.selector {
                Selector::Tags(_) => "tags",
                Selector::Entities(_) => "entities",
                Selector::Other { .. } => "other",
            };
            let outcome = if e.items.is_empty() { "empty" } else { "picks" };
            *outcomes
                .entry((format!("{:?} {kind}", t.generation), outcome.into()))
                .or_default() += 1;
        }
    }
    println!("entries (generation selector, outcome): {outcomes:?}");

    // Secondary groups weight the picks; they are no drops.
    let rarity = tags
        .by_name("LootRarity")
        .first()
        .copied()
        .ok_or("no LootRarity tag")?;
    for t in tables.values() {
        for g in t.archetypes.iter().flat_map(|a| &a.archetype.secondary) {
            for e in &g.entries {
                let Selector::Tags(term) = &e.selector else {
                    panic!("v2 secondary {} has no tag selector", g.name);
                };
                assert!(
                    term.positive
                        .iter()
                        .all(|p| tags.is_descendant_of(p, &rarity)),
                    "v2 secondary {} is not a rarity band",
                    g.name
                );
            }
        }
    }
    let layers: HashSet<&str> = lootables
        .values()
        .flat_map(|l| &l.configs)
        .flat_map(|c| &c.secondary)
        .map(|g| g.name.as_str())
        .collect();
    println!("v3 secondary layers: {layers:?}");
    assert!(layers.iter().all(|n| {
        n.starts_with("V3ManufacturerWeights_") || n.starts_with("V3SecondaryChoices_")
    }));

    // Droppable items: tables named by some lootable.
    let droppable: HashSet<&Guid> = named
        .iter()
        .filter_map(|t| tables.get(t))
        .flat_map(|t| t.items())
        .collect();
    println!("{} items some lootable's table can drop", droppable.len());

    // The generic filter (no faction, no special-event items) narrows it.
    let filters: HashMap<String, &PoolFilter> = lootables
        .values()
        .flat_map(|l| &l.configs)
        .filter_map(|c| Some((name(&c.pool_filter_record?), c.pool_filter.as_ref()?)))
        .collect();
    let generic = filters
        .get("V3PoolFilter_Generic")
        .ok_or("no lootable uses V3PoolFilter_Generic")?;
    assert!(matches!(generic, PoolFilter::Sequence(_)));
    let kept = droppable
        .iter()
        .filter(|g| tables.admits(generic, g))
        .count();
    println!(
        "V3PoolFilter_Generic keeps {kept} of {} droppable items",
        droppable.len()
    );
    assert!(0 < kept && kept < droppable.len());

    // `--who <record>`: where a record is picked.
    if let Some(who) = std::env::args().skip_while(|a| a != "--who").nth(1) {
        for (g, t) in tables.iter() {
            for a in &t.archetypes {
                let arch = a.archetype.record.map(|r| name(&r));
                for e in &a.archetype.entries {
                    let hit =
                        |list: &[Guid]| list.iter().any(|i| name(i).eq_ignore_ascii_case(&who));
                    if hit(&e.items) {
                        println!(
                            "  {}: {arch:?} entry {:?} {:?}",
                            name(g),
                            e.name,
                            e.selector
                        );
                    }
                    for s in e.spawn_with.iter().filter(|s| hit(&s.items)) {
                        println!(
                            "  {}: {arch:?} spawn-with {:?} {:?}",
                            name(g),
                            s.name,
                            s.selector
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
