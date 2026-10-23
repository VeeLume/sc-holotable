//! Re-proves the phase-1 acquisition rule on LIVE 4.10: stock on player
//! vehicles, mount evidence and the vehicle listing.
//!
//! - The Centaurus is listed, and its stock CF-557 guns are obtainable from it.
//! - Vehicles: 221 base, 25 editions and 52 rewards obtainable, 849 variants
//!   unobtainable.
//! - Locked-only mounts are no proof: the Bengal turret's Slayer cannon (Size
//!   5 despite "S8") and the M4A, which shops sell, both sit only in locked
//!   ports and both read unknown.
//! - `SCItemPurchasableParams` is on every NPC-only gun too, so it marks no
//!   shop item.
//! - The Tempest CitCon missile sits in no stock loadout at all: unknown.
//! - Crafting: every item a starting blueprint makes reads Craft / Default;
//!   mission blueprint pools add more.
//!
//! ```bash
//! cargo run -p sc-acquisitions --release --example acquisitions_probe
//! cargo run -p sc-acquisitions --release --example acquisitions_probe -- --show BEHR_LaserCannon_S2
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_acquisitions::{Acquisitions, Listing, RecordCollection, Source, Status, Unlock};
use sc_crafting::{Blueprints, GlobalParams};
use sc_extract::generated::{DataForgeComponentParamsPtr, EItemType};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};
use sc_items::Items;
use sc_missions::Missions;
use sc_vehicles::{PortContainers, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let blueprints = Blueprints::build(&datacore, &items);
    let crafting = GlobalParams::build(&datacore).ok_or("no CraftingGlobalParams")?;
    let missions = Missions::build(&datacore);
    let acquisitions =
        Acquisitions::build(&vehicles, &containers, &blueprints, &crafting, &missions);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| {
            let n = r.name()?;
            let n = n.strip_prefix("EntityClassDefinition.").unwrap_or(n);
            Some((r.id(), n.to_string()))
        })
        .collect();
    let by_name = |n: &str| -> Option<Guid> {
        names
            .iter()
            .find(|(_, v)| v.eq_ignore_ascii_case(n))
            .map(|(g, _)| *g)
    };
    let name = |g: &Guid| names.get(g).map_or("?", String::as_str);
    println!("{} records classified", acquisitions.len());

    // The Centaurus and what it carries.
    let centaurus = by_name("RSI_Constellation_Mk5_Centaurus").ok_or("no Centaurus")?;
    let c = acquisitions
        .get(&centaurus)
        .ok_or("Centaurus not classified")?;
    assert_eq!(c.status(), Status::Obtainable);
    println!("Centaurus: {:?} {:?}", c.listing, c.sources);
    let carried: Vec<&Guid> = acquisitions
        .iter()
        .filter(|(_, a)| a.sources.contains(&Source::Stock { vehicle: centaurus }))
        .map(|(g, _)| g)
        .collect();
    println!("  carries {} distinct records as stock", carried.len());
    // CF-557 is Klaus & Werner's S5 laser repeater.
    assert!(
        carried
            .iter()
            .any(|g| name(g).eq_ignore_ascii_case("KLWE_LaserRepeater_S5")),
        "the Centaurus's stock CF-557 guns"
    );

    // Vehicles by listing and status.
    let mut ships: BTreeMap<(String, String), usize> = BTreeMap::new();
    for a in acquisitions.values().filter(|a| a.listing.is_some()) {
        let listing = a.listing.unwrap_or(Listing::Variant);
        *ships
            .entry((format!("{listing:?}"), format!("{:?}", a.status())))
            .or_default() += 1;
    }
    println!("vehicles (listing, status): {ships:?}");
    let count = |l: &str, s: &str| ships.get(&(l.into(), s.into())).copied();
    assert_eq!(count("Base", "Obtainable"), Some(221));
    assert_eq!(count("Edition", "Obtainable"), Some(25));
    assert_eq!(count("Reward", "Obtainable"), Some(52));
    assert_eq!(count("Variant", "Unobtainable"), Some(849));

    // Locked-only is a hint: the Bengal Slayer and the M4A alike sit only in
    // locked ports, yet a mission blueprint makes the M4A (shops sell it too).
    for (n, status) in [
        ("Bengal_Turret_BallisticCannon_S8", Status::Unknown),
        ("BEHR_LaserCannon_S2", Status::Obtainable),
    ] {
        let a = by_name(n)
            .and_then(|g| acquisitions.get(&g))
            .ok_or(format!("{n} carried by no stock"))?;
        println!(
            "{n}: {:?}, {} mounts, locked only {}, e.g. {} at {}",
            a.status(),
            a.mounts.len(),
            a.locked_only(),
            name(&a.mounts[0].vehicle),
            a.mounts[0].path,
        );
        assert_eq!(a.status(), status);
        assert!(a.locked_only());
        for s in &a.sources {
            if let Source::Craft { blueprint, unlock } = s {
                let missions = match unlock {
                    Unlock::Default => 0,
                    Unlock::Missions(m) => m.len(),
                };
                println!("  crafted from {blueprint} ({missions} missions grant it)");
            }
        }
    }

    // The Tempest CitCon: in no stock loadout.
    let citcon = by_name("MISL_S02_CS_FSKI_Tempest_CitCon").ok_or("no Tempest CitCon")?;
    assert!(acquisitions.get(&citcon).is_none());
    assert_eq!(acquisitions.status(&citcon), Status::Unknown);

    // SCItemPurchasableParams on guns, by status and locked-only.
    let store = datacore.records();
    let ecds = &store.records.multi_feature.entity_class_definition;
    let purchasable = |g: &Guid| {
        ecds.get(g)
            .and_then(|h| h.get(&store.pools))
            .is_some_and(|e| {
                e.components
                    .iter()
                    .any(|c| matches!(c, DataForgeComponentParamsPtr::SCItemPurchasableParams(_)))
            })
    };
    let mut guns: BTreeMap<(String, bool, bool), usize> = BTreeMap::new();
    for (guid, item) in items.iter() {
        if item.item_type != EItemType::WeaponGun {
            continue;
        }
        let a = acquisitions.get(guid);
        let locked = a.is_some_and(|a| a.locked_only());
        let status = format!("{:?}", acquisitions.status(guid));
        *guns.entry((status, locked, purchasable(guid))).or_default() += 1;
    }
    println!("guns (status, locked only, purchasable): {guns:?}");
    // Every locked-only gun without a source (Bengal, Vanduul, sentries)
    // carries it, so it marks no shop item.
    assert!(
        guns.get(&("Unknown".into(), true, true))
            .is_some_and(|&n| n > 0)
    );
    assert_eq!(guns.get(&("Unknown".into(), true, false)), None);

    // Crafting: every whitelisted blueprint's item reads Craft / Default.
    let defaults = &crafting.default_blueprint_whitelist;
    println!(
        "{} blueprints, {} start unlocked",
        blueprints.len(),
        defaults.len()
    );
    for bp in defaults {
        // Not every starting blueprint creates an item (dismantling).
        let Some(made) = blueprints.get(bp).and_then(|b| b.crafted_entity_guid()) else {
            println!("  default: {bp} creates no item");
            continue;
        };
        let a = acquisitions
            .get(&made)
            .ok_or("whitelisted item unclassified")?;
        println!("  default: {}", name(&made));
        assert!(a.sources.contains(&Source::Craft {
            blueprint: *bp,
            unlock: Unlock::Default
        }));
    }
    let mut crafts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut only_craft = 0;
    for a in acquisitions.values() {
        let mut craft = false;
        for s in &a.sources {
            if let Source::Craft { unlock, .. } = s {
                craft = true;
                let kind = match unlock {
                    Unlock::Default => "default",
                    Unlock::Missions(_) => "missions",
                };
                *crafts.entry(kind).or_default() += 1;
            }
        }
        if craft && a.sources.iter().all(|s| matches!(s, Source::Craft { .. })) {
            only_craft += 1;
        }
    }
    println!(
        "craft sources by unlock: {crafts:?}; {only_craft} records obtainable only by crafting"
    );
    let creating = blueprints
        .values()
        .filter(|b| b.crafted_entity_guid().is_some())
        .count();
    let unlocked: usize = crafts.values().sum();
    println!(
        "{creating} blueprints create an item; {} of them neither start unlocked nor are in a mission reward pool",
        creating - unlocked
    );
    assert!(crafts.get("missions").is_some_and(|&n| n > 0));

    // Every item record by status.
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (guid, _) in items.iter() {
        *counts
            .entry(format!("{:?}", acquisitions.status(guid)))
            .or_default() += 1;
    }
    println!("items by status: {counts:?}");

    // `--show <record>` lists one record's mounts.
    if let Some(show) = std::env::args().skip_while(|a| a != "--show").nth(1)
        && let Some(a) = by_name(&show).and_then(|g| acquisitions.get(&g))
    {
        println!("{show}: {:?} {:?}", a.status(), a.sources);
        for m in &a.mounts {
            println!(
                "  {} at {} editable={}",
                name(&m.vehicle),
                m.path,
                m.editable
            );
        }
    }
    Ok(())
}
