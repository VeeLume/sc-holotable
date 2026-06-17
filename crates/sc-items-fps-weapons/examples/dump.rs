//! Build the FPS-weapon stat sheet and verify the P6-LR against scmdb's
//! known values, then print a few samples.
//!
//! ```bash
//! cargo run -p sc-items-fps-weapons --release --example dump
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
use sc_items::Items;
use sc_items_fps_weapons::FpsWeapons;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());
    let weapons = FpsWeapons::build(&datacore, &items);
    let db = datacore.db();

    println!("FPS weapons indexed: {}", weapons.len());

    // ── verify P6-LR (behr_sniper_ballistic_01) vs scmdb @ base ──
    let target = "behr_sniper_ballistic_01";
    let p6 = weapons
        .iter()
        .find(|(g, _)| {
            db.record(g)
                .and_then(|r| r.name())
                .map(|n| n.ends_with(target))
                .unwrap_or(false)
        })
        .map(|(_, s)| s);
    println!(
        "\n=== {target} (expect: fire 55, dmg.phys 100, speed 725, spread 11/21, recoil 1.55/0.44/0.09, mag 8) ==="
    );
    match p6 {
        Some(s) => {
            println!("  fire_rate     = {:?}", s.fire_rate);
            println!("  damage        = {:?}", s.damage);
            println!("  ammo_speed    = {:?}", s.ammo_speed);
            println!("  ammo_lifetime = {:?}", s.ammo_lifetime);
            println!("  spread        = {:?} / {:?}", s.spread_min, s.spread_max);
            println!(
                "  recoil p/y/s  = {:?} / {:?} / {:?}",
                s.recoil_pitch, s.recoil_yaw, s.recoil_smooth
            );
            println!("  mag_size      = {:?}", s.mag_size);
        }
        None => println!("  !! not found"),
    }

    // ── a few more samples across families ──
    println!("\n=== samples ===");
    let mut shown = 0;
    for (guid, s) in weapons.iter() {
        let name = db.record(guid).and_then(|r| r.name()).unwrap_or("?");
        if name.ends_with("_01") {
            println!(
                "  {name:<42} fire={:?} dmg={:?} mag={:?} recoil_p={:?}",
                s.fire_rate,
                s.damage.map(|d| d.total()),
                s.mag_size,
                s.recoil_pitch,
            );
            shown += 1;
            if shown >= 12 {
                break;
            }
        }
    }
    Ok(())
}
