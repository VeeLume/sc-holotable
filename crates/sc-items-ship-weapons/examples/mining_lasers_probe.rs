//! Re-proves the mining-laser stats against the LIVE data: the Arbor S1
//! (fracture 2340 / extraction 1850 energy/s, 60 / 180 m, throttle minimum
//! 0.05, instability -35 %, window +40 %, resistance +25 %) and the module slot
//! counts Mort13's laser setup shows (Helix II 3, Helix I 2, Pitman = Golem S1
//! 2). Then lists every mining laser with its beams' hit types.
//!
//! ```bash
//! cargo run -p sc-items-ship-weapons --release --example mining_lasers_probe
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_weapons::{ShipWeaponStats, ShipWeapons};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let db = datacore.db();
    let name = |guid: &Guid| {
        db.record(guid)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_default()
    };

    let weapons = ShipWeapons::build(&datacore, &items);
    let lasers: Vec<(String, &ShipWeaponStats)> = weapons
        .iter()
        .filter(|(_, w)| w.mining.is_some())
        .map(|(g, w)| (name(g), w))
        .collect();
    let laser = |n: &str| {
        lasers
            .iter()
            .find(|(k, _)| k == n)
            .and_then(|(_, w)| w.mining.as_ref())
            .ok_or(format!("no laser {n}"))
    };

    let arbor = laser("Mining_Laser_GRIN_Arbor_S1")?;
    assert_eq!(arbor.beams.len(), 2);
    assert_eq!(arbor.beams[0].energy_per_second, 2340.0);
    assert_eq!(arbor.beams[1].energy_per_second, 1850.0);
    assert_eq!(
        (
            arbor.beams[0].full_damage_range,
            arbor.beams[0].zero_damage_range
        ),
        (60.0, 180.0)
    );
    assert_eq!(
        arbor.fracture_beam().map(|b| b.energy_per_second),
        Some(2340.0)
    );
    assert_eq!(
        arbor.extraction_beam().map(|b| b.energy_per_second),
        Some(1850.0)
    );
    assert_eq!(arbor.throttle_minimum, 0.05);
    assert_eq!(
        arbor.modifiers.laser_instability.map(|m| m.value),
        Some(-35.0)
    );
    assert_eq!(
        arbor.modifiers.optimal_charge_window_size.map(|m| m.value),
        Some(40.0)
    );
    assert_eq!(arbor.modifiers.resistance.map(|m| m.value), Some(25.0));
    assert_eq!(laser("Mining_Laser_THCN_Helix_S2")?.module_slots, 3);
    assert_eq!(laser("Mining_Laser_THCN_Helix_S1")?.module_slots, 2);
    assert_eq!(laser("Mining_Laser_DRAK_Golem_S1")?.module_slots, 2);
    println!("asserts passed: Arbor S1, module slots\n");

    let mut rows = lasers.clone();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    for (n, w) in rows {
        let m = w.mining.as_ref().expect("filtered");
        let beams: Vec<String> = m
            .beams
            .iter()
            .map(|b| {
                format!(
                    "[{}] {} {}/s {}..{}m",
                    b.fire_action,
                    b.hit_type,
                    b.energy_per_second,
                    b.full_damage_range,
                    b.zero_damage_range
                )
            })
            .collect();
        println!(
            "{n:<40} S{} slots={} thrMin={:<5} {}",
            w.size,
            m.module_slots,
            m.throttle_minimum,
            beams.join(" | ")
        );
    }
    Ok(())
}
