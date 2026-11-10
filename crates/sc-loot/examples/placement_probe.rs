//! Where lootables sit (LIVE 4.10): every object container walked for the
//! entities that place a lootable — corpses by their class, containers by
//! the harvestable slot config a `looseSubConfigBase` names — and each
//! socpak joined to the star-map places it belongs to. Prints the
//! placements by kind, how many reach a place, and the places with the
//! most loot entities. Asserts that at least 100 lootables are placed,
//! cave corpses (`Corpse_Male_Cave`) among them, and that most container
//! placements reach a place.
//!
//! ```bash
//! cargo run -p sc-loot --release --example placement_probe
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_loot::{LootPlacements, LootTables, Lootables, Placed};
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
    let mut kinds: BTreeMap<&str, (usize, usize, usize)> = BTreeMap::new();
    let mut per_place: HashMap<Guid, usize> = HashMap::new();
    for (_, list) in placements.iter() {
        for p in list {
            let kind = match p.placed {
                Placed::Entity => "entity",
                Placed::Container { .. } => "container",
            };
            let k = kinds.entry(kind).or_default();
            k.0 += 1;
            k.1 += p.count;
            if !p.places.is_empty() {
                k.2 += 1;
            }
            for (place, n) in &p.places {
                *per_place.entry(*place).or_default() += n * p.count;
            }
        }
    }
    println!(
        "{} of {} lootables placed",
        placements.len(),
        lootables.len()
    );
    for (k, (socpaks, entities, placed)) in &kinds {
        println!(
            "  {k}: {socpaks} socpak placements ({entities} entities), {placed} reach a place"
        );
    }
    let mut top: Vec<(&Guid, &usize)> = per_place.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1).then(a.0.to_string().cmp(&b.0.to_string())));
    println!("{} places hold loot; the most:", per_place.len());
    for (place, n) in top.iter().take(12) {
        println!(
            "  {n:>6}  {}  ({} lootables)",
            name(place),
            placements.at(place).len()
        );
    }
    let mut unreached: BTreeMap<String, usize> = BTreeMap::new();
    for (_, list) in placements.iter() {
        for p in list.iter().filter(|p| p.places.is_empty()) {
            let dir: Vec<&str> = p.socpak.split('/').take(5).collect();
            *unreached.entry(dir.join("/")).or_default() += p.count;
        }
    }
    if let Some(want) = std::env::args().skip_while(|a| a != "--socpak").nth(1) {
        for (g, list) in placements.iter() {
            for p in list.iter().filter(|p| p.socpak.contains(want.as_str())) {
                let at: Vec<String> = p
                    .places
                    .iter()
                    .map(|(g, n)| format!("{} x{n}", name(g)))
                    .collect();
                println!("  {} x{} {} -> {at:?}", p.socpak, p.count, name(g));
            }
        }
    }
    if std::env::args().any(|a| a == "--unreached") {
        for (g, list) in placements.iter() {
            for p in list.iter().filter(|p| p.places.is_empty()) {
                println!("  unreached {} x{} {}", p.socpak, p.count, name(g));
            }
        }
    }
    println!("entities in socpaks that reach no place, by directory:");
    for (d, n) in &unreached {
        println!("  {n:>5}  {d}");
    }
    assert!(placements.len() >= 100);
    let cave = db
        .all_records()
        .find(|r| r.name() == Some("EntityClassDefinition.Corpse_Male_Cave"))
        .map(|r| r.id())
        .ok_or("no Corpse_Male_Cave")?;
    assert!(placements.contains(&cave), "cave corpses are placed");
    let (socpaks, _, placed) = kinds.get("container").copied().unwrap_or_default();
    assert!(
        placed * 2 > socpaks,
        "most container placements reach a place"
    );
    Ok(())
}
