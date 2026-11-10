//! Where lootables come from: for every lootable record, what references it
//! (record type, two hops up), summarised by type chain.
//!
//! ```bash
//! cargo run -p sc-loot --release --example where_dig
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_loot::{LootTables, LootableKind, Lootables};
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
    let db = datacore.db();
    let graph = sc_extract::ReferenceGraph::from_database(db);
    let ty = |g: &Guid| {
        db.record(g)
            .and_then(|r| r.type_name().map(String::from))
            .unwrap_or_else(|| "?".into())
    };
    let name = |g: &Guid| {
        db.record(g)
            .and_then(|r| r.name().map(String::from))
            .unwrap_or_else(|| "?".into())
    };
    let mut chains: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut unreferenced: BTreeMap<String, usize> = BTreeMap::new();
    for (g, l) in lootables.iter() {
        let kind = match l.kind {
            LootableKind::Entity => "entity",
            LootableKind::Harvestable => "harvestable",
        };
        let up = graph.incoming(g);
        if up.is_empty() {
            *unreferenced.entry(format!("{kind} {}", ty(g))).or_default() += 1;
        }
        for r in up {
            let up2: Vec<String> = graph.incoming(r).iter().map(&ty).collect();
            let mut up2s = up2.clone();
            up2s.sort();
            up2s.dedup();
            chains
                .entry(format!("{kind} {} <- {} <- {:?}", ty(g), ty(r), up2s))
                .or_default()
                .push(format!("{} <- {}", name(g), name(r)));
        }
    }
    println!("unreferenced lootables: {unreferenced:?}");
    for (chain, ex) in &chains {
        println!("{:>5}  {chain}", ex.len());
        for e in ex.iter().take(2) {
            println!("         e.g. {e}");
        }
    }
    if let Some(who) = std::env::args().skip_while(|a| a != "--refs").nth(1) {
        let g = db
            .all_records()
            .find(|r| r.name().is_some_and(|n| n.ends_with(&who)))
            .map(|r| r.id())
            .ok_or("no record")?;
        for r in graph.incoming(&g) {
            let up: Vec<String> = graph.incoming(r).iter().map(&name).collect();
            println!("  {} <- {up:?}", name(r));
        }
    }
    Ok(())
}
