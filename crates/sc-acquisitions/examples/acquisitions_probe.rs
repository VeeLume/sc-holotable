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
//!
//! ```bash
//! cargo run -p sc-acquisitions --release --example acquisitions_probe
//! cargo run -p sc-acquisitions --release --example acquisitions_probe -- --show BEHR_LaserCannon_S2
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_acquisitions::{Acquisitions, Listing, RecordCollection, Source, Status};
use sc_extract::generated::{DataForgeComponentParamsPtr, EItemType};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};
use sc_items::Items;
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
    let acquisitions = Acquisitions::build(&vehicles, &containers);
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

    // Locked-only is a hint: the Bengal Slayer and the shop M4A alike.
    for n in ["Bengal_Turret_BallisticCannon_S8", "BEHR_LaserCannon_S2"] {
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
        assert_eq!(a.status(), Status::Unknown);
        assert!(a.locked_only());
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
    // The 13 NPC-only guns (Bengal, Vanduul, sentries; the M4A among them)
    // all carry it.
    assert_eq!(guns.get(&("Unknown".into(), true, true)), Some(&13));
    assert_eq!(guns.get(&("Unknown".into(), true, false)), None);

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
