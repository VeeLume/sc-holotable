//! End-to-end product-stats verification: reproduce scmdb's P6-LR
//! (behr_sniper_ballistic_01) PRODUCT STATS at Q750 through the full pipeline
//! — Blueprints + GameplayProperties (sc-crafting) × FpsWeapons base stats
//! (sc-items-fps-weapons).
//!
//! scmdb @ Q750: Physical Damage 100→110 (+10%), Recoil Pitch 1.550→1.240
//! (−20%), Recoil Yaw 0.440→0.352 (−20%), Recoil Smooth 0.090→0.072 (−20%).
//!
//! ```bash
//! cargo run -p sc-crafting --release --example product_stats
//! ```

use sc_crafting::{Blueprints, GameplayProperties};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};
use sc_items::Items;
use sc_items_armor::Armor;
use sc_items_fps_weapons::FpsWeapons;
use sc_items_ship_components::ShipComponents;
use sc_items_ship_weapons::ShipWeapons;

const Q: i32 = 750;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());
    let weapons = FpsWeapons::build(&datacore, &items);
    let armor = Armor::build(&datacore, &items);
    let ship = ShipComponents::build(&datacore, &items);
    let ship_wpn = ShipWeapons::build(&datacore, &items);
    let gp = GameplayProperties::build(&datacore);
    let blueprints = Blueprints::build(&datacore, &items);
    let locale = &asset_data.locale;
    let db = datacore.db();

    let find = |target: &str| -> Option<Guid> {
        db.records_by_type("EntityClassDefinition")
            .find(|r| r.name().map(|n| n.ends_with(target)).unwrap_or(false))
            .map(|r| r.id())
    };
    let show = |label: &str, stats: &[sc_crafting::ProductStat]| {
        println!("\n=== {label} ===");
        println!("  {:<22} {:>10} {:>10} {:>9}", "stat", "base", "modified", "pct");
        for ps in stats {
            let label = gp
                .get(&ps.gameplay_property)
                .and_then(|p| locale.resolve(&p.property_name_key))
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("{:?}", ps.stat));
            let base = ps.base.map(|b| format!("{b:.3}")).unwrap_or_else(|| "—".into());
            let modified = ps.modified.map(|m| format!("{m:.3}")).unwrap_or_else(|| "—".into());
            println!("  {label:<22} {base:>10} {modified:>10} {:>+8.2}%", ps.pct_change());
        }
    };

    // Weapons (FpsWeapons source) @ Q750.
    for target in ["behr_sniper_ballistic_01", "klwe_pistol_energy_01"] {
        if let Some(guid) = find(target) {
            show(&format!("{target} @ Q{Q}"), &blueprints.product_stats(guid, &gp, &weapons, Q));
        }
    }
    // Armor (Armor source) @ Q750 — MacFlex Arms.
    for target in ["rsi_deckcrew_armor_light_arms_01_01_01"] {
        if let Some(guid) = find(target) {
            show(&format!("{target} @ Q{Q}"), &blueprints.product_stats(guid, &gp, &armor, Q));
        }
    }
    // ADP Core @ Q0 — reproduce scmdb: physical mitigation 0.40 → 0.34 (−15%).
    if let Some(guid) = find("cds_legacy_armor_heavy_core_01_01_01") {
        show("ADP Core (cds_legacy_armor_heavy_core_01_01_01) @ Q0", &blueprints.product_stats(guid, &gp, &armor, 0));
    }
    // Ship components (ShipComponents source) @ Q750 — Zephyr QD, Steward shield,
    // Charger power plant, Kelvid cooler.
    for target in [
        "QDRV_RACO_S01_Zephyr_SCItem",
        "SHLD_BASL_S01_Steward_SCItem",
        "POWR_AEGS_S01_Charger_SCItem",
        "COOL_WCPR_S00_Kelvid_SCItem",
        "RADR_GRNP_S00_Prevenir",
    ] {
        if let Some(guid) = find(target) {
            show(&format!("{target} @ Q{Q}"), &blueprints.product_stats(guid, &gp, &ship, Q));
        }
    }
    // Ship weapons (ShipWeapons source) @ Q750 — mining laser (by record name)
    // + C-788 cannon (by display name).
    if let Some(guid) = find("Mining_Laser_THCN_Impact_S1") {
        show(&format!("Impact Mining Laser @ Q{Q}"), &blueprints.product_stats(guid, &gp, &ship_wpn, Q));
    }
    if let Some(guid) = blueprints
        .iter()
        .find(|bp| bp.display_name(locale).map(|n| n.contains("C-788")).unwrap_or(false))
        .and_then(|bp| bp.crafted_entity_guid())
    {
        show(&format!("C-788 Cannon @ Q{Q}"), &blueprints.product_stats(guid, &gp, &ship_wpn, Q));
    }
    Ok(())
}
