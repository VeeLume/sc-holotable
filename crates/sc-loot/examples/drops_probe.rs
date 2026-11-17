//! Where items drop (`Drops`) on the live game: every lootable config that
//! can yield an item, the container it fills, the places it sits and the
//! chance per fill. Items are picked by display name (a probe may scope by
//! name; the library joins by GUID). The second argument is the number of
//! fills per config (default 2,000).
//!
//! Asserts that chances are in (0, 1], that a fill's expected count is at
//! least its chance, and that every list is sorted most likely first.
//!
//! ```bash
//! cargo run -p sc-loot --release --example drops_probe -- "P4-AR" 2000
//! ```

use std::collections::HashSet;
use std::time::Instant;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_loot::{Drops, LootPlacements, LootTables, Lootables};
use sc_tags::Tags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let needle = std::env::args().nth(1).unwrap_or_else(|| "P4-AR".into());
    let runs: u32 = std::env::args()
        .nth(2)
        .and_then(|a| a.parse().ok())
        .unwrap_or(2_000);
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let tags = Tags::build(datacore.records());
    let tables = LootTables::build(&datacore, &items, &tags);
    let lootables = Lootables::build(&datacore, &tables);
    let placements = LootPlacements::cook(&assets, &lootables)?;
    let db = datacore.db();
    let name = |g: &Guid| {
        db.record(g)
            .and_then(|r| {
                r.name()
                    .map(|n| n.split_once('.').map_or(n, |(_, n)| n).to_string())
            })
            .unwrap_or_else(|| g.to_string())
    };

    let start = Instant::now();
    let drops = Drops::build(&tables, &lootables, &placements, &items, runs);
    let entries: usize = drops.values().map(Vec::len).sum();
    println!(
        "Drops::build: {} items, {entries} drops, {runs} fills per config, {:.1} s, {} KB as JSON",
        drops.len(),
        start.elapsed().as_secs_f64(),
        serde_json::to_vec(&drops)?.len() / 1024
    );
    for list in drops.values() {
        for d in list {
            assert!(d.chance > 0.0 && d.chance <= 1.0, "chance in (0, 1]");
            assert!(d.expected + 1e-9 >= d.chance, "expected >= chance");
        }
        assert!(
            list.windows(2).all(|w| w[0].chance >= w[1].chance),
            "most likely first"
        );
    }
    let placed_items = drops
        .values()
        .filter(|l| l.iter().any(|d| !drops.source(d).places.is_empty()))
        .count();
    println!("{placed_items} items drop somewhere placed on the star map");

    let mut matches: Vec<(Guid, String)> = items
        .iter()
        .filter_map(|(g, i)| {
            let shown = i.display_name(&data.locale)?;
            shown.contains(&needle).then(|| (*g, shown.to_string()))
        })
        .collect();
    matches.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{} items match {needle:?}", matches.len());

    for (item, shown) in matches.iter().take(4) {
        let list = drops.get(item).map_or(&[][..], Vec::as_slice);
        let placed: Vec<_> = list
            .iter()
            .filter(|d| !drops.source(d).places.is_empty())
            .collect();
        let places: HashSet<Guid> = placed
            .iter()
            .flat_map(|d| drops.source(d).places.iter().map(|(p, _)| *p))
            .collect();
        println!(
            "\n{shown} ({}): {} drops, {} placed at {} places, best placed {:.2} %",
            name(item),
            list.len(),
            placed.len(),
            places.len(),
            placed.first().map_or(0.0, |d| d.chance * 100.0)
        );
        for d in placed.iter().take(6) {
            let s = drops.source(d);
            let at: Vec<String> = s
                .places
                .iter()
                .take(3)
                .map(|(p, n)| format!("{} x{n}", name(p)))
                .collect();
            println!(
                "  {:>6.2} % ({:.3} each)  {} cfg {}  in {} ({:?} SCU)  at {}",
                d.chance * 100.0,
                d.expected,
                name(&s.lootable),
                s.config,
                s.container.as_ref().map_or_else(|| "-".into(), name),
                s.capacity,
                at.join(", ")
            );
        }
    }
    Ok(())
}
