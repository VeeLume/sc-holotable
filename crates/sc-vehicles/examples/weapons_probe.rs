//! Re-proves the weapon figures on the stock RSI Constellation Mk V
//! Centaurus (LIVE 4.10.197) against erkul's default for it: the weapon pool
//! at its full 4 of 12.2 requested (ratio 0.328) and the ship's engineering
//! buff (guns' `maxAmmoLoad` × 9). CF-557 (S5): 221 shots, 4.918/s regen,
//! sustained 341.7; CF-337 (S3): sustained 152.3. Prints the power-on state
//! (2 of 12.2) too.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example weapons_probe
//! ```

use std::collections::HashMap;

use sc_extract::generated::EMasterMode;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{
    ItemportModifiers, NetworkParams, PortContainers, PowerNetwork, ResourceProfiles, Vehicles,
    weapon_power,
};
use sc_weapons::{LoadoutContext, ShipWeapon, SustainKind, Weapons};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let profiles = ResourceProfiles::build(&datacore);
    let params = NetworkParams::build(&datacore).ok_or("no network params")?;
    let modifiers = ItemportModifiers::build(&datacore);
    let weapons = Weapons::build(&datacore, &items);
    let by_guid: HashMap<Guid, &ShipWeapon> = weapons.ships.iter().map(|w| (w.guid, w)).collect();
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let ship = *names
        .iter()
        .find(|(_, n)| *n == "RSI_Constellation_Mk5_Centaurus")
        .ok_or("no Centaurus")?
        .0;
    let v = vehicles.get(&ship).ok_or("no vehicle")?;
    let network = PowerNetwork::build(
        &v.stock,
        containers.get(&ship),
        &vehicles,
        &items,
        &profiles,
        &params,
    );
    let weapons_pool = network
        .pools
        .iter()
        .position(|p| p.item_type == sc_extract::generated::EItemType::WeaponGun)
        .ok_or("no weapon pool")?;
    let mode = EMasterMode::SCM;
    let mut allocation = network.power_on(&mode);

    let mut check = |label: &str, target: i64| -> Result<Vec<(String, f32, f32)>, String> {
        allocation.pool_targets[weapons_pool] = target;
        network.split_pools(&mut allocation);
        let powers = weapon_power(
            &network,
            &allocation,
            &mode,
            &v.stock,
            &vehicles,
            &modifiers,
        );
        println!("\n{label}: weapon pool target {target}");
        let mut out = Vec::new();
        for (i, power) in powers {
            let c = &network.consumers[i];
            let w = by_guid.get(&c.item).ok_or("gun not in Weapons")?;
            let p = w.powered(&power);
            let (shots, regen) = match &p.sustain {
                SustainKind::Energy(e) => (e.max_ammo_load, e.max_regen_per_sec),
                _ => (0.0, 0.0),
            };
            let sustained = p.sustained_dps().unwrap_or(0.0);
            let eff = w
                .effective_dps(&LoadoutContext {
                    window_seconds: 60.0,
                    power: Some(power),
                })
                .unwrap_or(0.0);
            println!(
                "  {:<40} ratio {:.4} x{} -> {shots} shots, {regen:.3}/s, sustained {sustained:.1}, 60 s {eff:.1}",
                names.get(&c.item).map_or("?", String::as_str),
                power.ratio,
                power.max_ammo_load_multiplier,
            );
            out.push((
                names.get(&c.item).cloned().unwrap_or_default(),
                shots,
                sustained,
            ));
        }
        Ok(out)
    };

    let erkul = check("erkul default (weapons first)", 4)?;
    let s5 = erkul
        .iter()
        .find(|(n, ..)| n == "KLWE_LaserRepeater_S5")
        .ok_or("no S5")?;
    assert_eq!(s5.1, 221.0);
    assert!((s5.2 - 341.7).abs() < 0.1, "S5 sustained {}", s5.2);
    let s3 = erkul
        .iter()
        .find(|(n, ..)| n == "KLWE_LaserRepeater_S3")
        .ok_or("no S3")?;
    assert!((s3.2 - 152.3).abs() < 0.1, "S3 sustained {}", s3.2);
    check("power-on", 2)?;

    // A sequence gun: the BEHR ballistic repeater fires 2 entries at 750
    // over inner Singles at 500, so 750 rpm, and heats 0.8 per shot:
    // 10/s, overheating after 10 s (erkul: 125 shots).
    let behr = weapons
        .ships
        .iter()
        .find(|w| w.record_name.ends_with("BEHR_BallisticRepeater_S3"))
        .ok_or("no BEHR repeater")?;
    println!(
        "
BEHR repeater: {:?} rpm, burst {:?}, overheat after {:?} s",
        behr.burst_rpm(),
        behr.burst_dps(),
        behr.time_to_overheat()
    );
    assert_eq!(behr.burst_rpm(), Some(750.0));
    assert!((behr.time_to_overheat().unwrap_or(0.0) - 10.0).abs() < 1e-3);
    println!("\nasserts passed: Centaurus guns at erkul's 4 / 12.2 with the x9 buff");
    Ok(())
}
