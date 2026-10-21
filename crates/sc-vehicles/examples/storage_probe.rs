//! Re-proves [`Storage`] on LIVE 4.10: cargo SCU against erkul (the
//! Constellation Mk V Centaurus's 240, the Freelancer's 66, the Cutlass
//! Black's 46), the Hammerhead's grid behind its cargo door's XML loadout,
//! and the lockers and racks the interiors place (the Centaurus's five suit
//! lockers, the Corsair's eight). Prints every listed ship's totals.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example storage_probe
//! ```

use std::collections::HashMap;
use std::time::Instant;

use sc_extract::generated::{EItemSubType, EItemType};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};
use sc_items::Items;
use sc_vehicles::{Interior, PortContainers, Storage, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();

    let t = Instant::now();
    let mut all: Vec<(String, Storage)> = Vec::new();
    for (g, v) in vehicles.listed() {
        let interior = Interior::read(&assets, v);
        let s = Storage::compute(
            v,
            &v.stock,
            &interior.entities,
            &vehicles,
            &items,
            &containers,
        );
        all.push((names.get(g).cloned().unwrap_or_default(), s));
    }
    println!("{} listed vehicles read in {:?}", all.len(), t.elapsed());
    all.sort_by(|a, b| a.0.cmp(&b.0));
    for (n, s) in &all {
        let slots: usize = s.weapon_racks.iter().map(|r| r.slots.len()).sum();
        println!(
            "{n:<44} cargo {:>6} ship inv {:>6} lockers {:>2} ({:>5.2} SCU) suit {:>2} racks {:>2} ({slots} slots)",
            s.cargo_scu(),
            s.ship_inventory.map_or("-".into(), |v| format!("{v}")),
            s.lockers.len(),
            s.locker_scu(),
            s.suit_lockers.len(),
            s.weapon_racks.len(),
        );
    }

    let get = |n: &str| all.iter().find(|(name, _)| name == n).map(|(_, s)| s);
    let centaurus = get("RSI_Constellation_Mk5_Centaurus").ok_or("no Centaurus")?;
    assert_eq!(centaurus.cargo_scu(), 240.0);
    assert_eq!(centaurus.ship_inventory, Some(1.5));
    assert_eq!(centaurus.suit_lockers.len(), 5);
    // The Perseus-type 19-slot rack takes a launcher, rifles, sidearms and
    // gadgets.
    let big = centaurus
        .weapon_racks
        .iter()
        .flat_map(|r| &r.slots)
        .any(|s| s.takes(&EItemType::WeaponPersonal, &EItemSubType::Large));
    assert!(big, "no slot takes a large weapon");
    assert_eq!(
        get("MISC_Freelancer").ok_or("no Freelancer")?.cargo_scu(),
        66.0
    );
    assert_eq!(
        get("DRAK_Cutlass_Black").ok_or("no Cutlass")?.cargo_scu(),
        46.0
    );
    let hammerhead = get("AEGS_Hammerhead").ok_or("no Hammerhead")?;
    assert!(hammerhead.cargo_scu() > 0.0, "Hammerhead grid not found");
    assert_eq!(
        get("DRAK_Corsair").ok_or("no Corsair")?.suit_lockers.len(),
        8
    );

    for rack in centaurus.weapon_racks.iter().take(3) {
        println!("\n{}:", names.get(&rack.item).map_or("?", String::as_str));
        for s in &rack.slots {
            let accepts: Vec<String> = s
                .accepts
                .iter()
                .map(|t| {
                    let subs: Vec<&str> = t.sub_types.iter().map(|s| s.as_dcb_str()).collect();
                    format!("{}{subs:?}", t.item_type.as_dcb_str())
                })
                .collect();
            println!(
                "  {} S{}-{}: {}",
                s.port,
                s.min_size,
                s.max_size,
                accepts.join(" ")
            );
        }
    }
    println!(
        "\nasserts passed: Centaurus 240 / Freelancer 66 / Cutlass 46 SCU, Hammerhead grid, suit lockers"
    );
    Ok(())
}
