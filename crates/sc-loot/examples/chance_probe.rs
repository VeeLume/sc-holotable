//! Drop chances on LIVE 4.10. Coverage first: how many droppable items have
//! an occupancy volume (`AttachDef.inventoryOccupancyVolume`) and how many
//! slot configs name a container whose capacity is known. Then, for one
//! lootable (`--who <record name>`, default an outlaw outpost slot preset),
//! each config's pick shares and 20,000 simulated fills into its container.
//! Asserts that every config's shares add up to its chance to generate,
//! that every droppable item has a volume, that a known container capacity
//! bounds a fill, and that no item's chance exceeds its expected count.
//!
//! ```bash
//! cargo run -p sc-loot --release --example chance_probe -- --who LootGeneration_SlotPreset_ColonialOutpost_Outlaw_Small
//! ```

use std::collections::HashSet;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_loot::{LootPlacements, LootTables, Lootables, MAX_RESULTS, Placed};
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
            .and_then(|r| {
                r.name()
                    .map(|n| n.split_once('.').map_or(n, |(_, n)| n).to_string())
            })
            .unwrap_or_else(|| g.to_string())
    };
    let display = |g: &Guid| {
        items
            .get(g)
            .and_then(|i| i.display_name(&data.locale))
            .map_or_else(|| name(g), str::to_string)
    };

    // Coverage.
    let droppable: HashSet<Guid> = lootables
        .values()
        .flat_map(|l| l.configs.iter().flat_map(|c| tables.drops(c)))
        .collect();
    let sized = droppable
        .iter()
        .filter(|g| items.get(g).is_some_and(|i| i.occupancy.is_some()))
        .count();
    println!(
        "{} droppable items, {sized} with an occupancy volume",
        droppable.len()
    );
    assert_eq!(sized, droppable.len());
    let capacity = |c: &Option<Guid>| {
        c.and_then(|g| items.get(&g))
            .and_then(|i| i.inventory.as_ref())
            .and_then(|inv| inv.scu())
    };
    let (mut slots, mut known) = (0, 0);
    let mut sums_ok = true;
    for l in lootables.values() {
        for c in &l.configs {
            if c.container.is_some() {
                slots += 1;
                if capacity(&c.container).is_some() {
                    known += 1;
                }
            }
            let sum: f64 = tables.shares(c).iter().map(|(_, p)| p).sum();
            if sum > 0.0 && (sum - f64::from(c.chance_to_generate.clamp(0.0, 1.0))).abs() > 1e-6 {
                sums_ok = false;
            }
        }
    }
    println!("{slots} slot configs name a container, {known} with a known capacity");
    assert!(sums_ok, "shares add up to the chance to generate");

    // One lootable in detail.
    let who = std::env::args()
        .skip_while(|a| a != "--who")
        .nth(1)
        .unwrap_or_else(|| "LootGeneration_SlotPreset_ColonialOutpost_Outlaw_Small".into());
    let (guid, lootable) = lootables
        .iter()
        .find(|(g, _)| name(g) == who)
        .ok_or("no such lootable")?;
    println!("\n{who} ({guid}): {} configs", lootable.configs.len());
    // A slot that names no container fills the entity placing the config.
    let placements = LootPlacements::cook(&assets, &lootables)?;
    let mut hosts: Vec<Guid> = placements
        .get(guid)
        .into_iter()
        .flatten()
        .filter_map(|p| match p.placed {
            Placed::Container { entity } => entity,
            Placed::Entity => None,
        })
        .collect();
    hosts.sort_by_key(Guid::to_string);
    hosts.dedup();
    for h in &hosts {
        println!("  placed in {} ({:?} SCU)", name(h), capacity(&Some(*h)));
    }
    let host = hosts.first().copied();
    for (n, c) in lootable.configs.iter().enumerate().take(4) {
        let cap = capacity(&c.container.or(host));
        println!(
            "config {n}: container {} ({cap:?} SCU), generate {:.2}, limit {}, fullness {:?}, {:?} / {:?}",
            c.container.as_ref().map_or_else(|| "-".into(), name),
            c.chance_to_generate,
            c.total_results_limit,
            c.fullness,
            c.pruning,
            c.fullness_mode,
        );
        for (item, p) in tables.shares(c).iter().take(5) {
            println!("    share {:>6.2} %  {}", p * 100.0, display(item));
        }
        let odds = tables.simulate(
            c,
            cap,
            |g| items.get(g).and_then(|i| i.occupancy),
            20_000,
            42,
        );
        println!(
            "  simulated: generated {:.1} %, {:.2} results per fill, {} items seen",
            odds.generated * 100.0,
            odds.results,
            odds.drops.len()
        );
        if cap.is_some() {
            assert!(
                odds.results < f64::from(MAX_RESULTS),
                "a known capacity bounds the fill"
            );
        }
        for d in odds.drops.iter().take(5) {
            println!(
                "    {:>6.2} % per fill, {:.3} each  {}",
                d.chance * 100.0,
                d.expected,
                display(&d.item)
            );
            assert!(d.expected + 1e-9 >= d.chance && d.chance <= 1.0);
        }
    }
    Ok(())
}
