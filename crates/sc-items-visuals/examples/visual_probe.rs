//! Print the render recipe of items by record name: look (geometry, material,
//! palette entries), ports, default loadout, inventory orientation, light rig
//! and grid cell. The defaults are the items sc-item-render was calibrated on
//! against in-game screenshots.
//!
//! ```bash
//! cargo run -p sc-items-visuals --release --example visual_probe -- [record name]...
//! ```

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_visuals::{InventoryView, Palettes, Visuals};

const DEFAULT: &[&str] = &[
    "behr_rifle_ballistic_01_sf01",
    "gmni_pistol_ballistic_01_pink_cian01",
    "behr_sniper_ballistic_01",
    "hdgw_rifle_ballistic_01",
    "POWR_AMRS_S02_JS400_SCItem",
    "can_drink_3_energy_a",
    "PlayerDeco_Gunrack_4P_3R_lowtech_1_a",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let wanted: Vec<String> = if args.is_empty() {
        DEFAULT.iter().map(|s| s.to_string()).collect()
    } else {
        args
    };
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let dc = Datacore::parse(&assets, &data)?;
    let items = Items::build(dc.records());
    let visuals = Visuals::build(&dc, &items);
    let palettes = Palettes::build(&dc);
    let view = InventoryView::build(&dc);
    println!(
        "{} visuals, {} palettes, default orientation {:?}",
        visuals.len(),
        palettes.len(),
        view.default_orientation
    );

    let mut names: HashMap<Guid, String> = HashMap::new();
    let mut by_name: HashMap<String, Guid> = HashMap::new();
    for r in dc.db().all_records() {
        let name = r.name().unwrap_or_default();
        let short = name.split_once('.').map_or(name, |(_, n)| n).to_string();
        by_name.insert(short.to_ascii_lowercase(), r.id());
        names.insert(r.id(), short);
    }
    let name = |g: &Guid| names.get(g).cloned().unwrap_or_else(|| g.to_string());

    for w in &wanted {
        let Some(guid) = by_name.get(&w.to_ascii_lowercase()) else {
            println!("\n== {w}: no record");
            continue;
        };
        let (Some(item), Some(v)) = (items.get(guid), visuals.get(guid)) else {
            println!("\n== {w}: no item / visual");
            continue;
        };
        println!(
            "\n== {w} [{}/{}] grid {:?} tags '{}'",
            item.item_type.as_dcb_str(),
            item.item_sub_type.as_dcb_str(),
            v.grid,
            v.geometry_tags
        );
        match v.look(&[]) {
            Some(look) => {
                println!("  geometry {}", look.geometry);
                println!("  material {:?}", look.material);
                if let Some(p) = &look.palette {
                    let pal = palettes.resolve(p);
                    println!("  palette  {} {:?}", name(&p.record), p.child);
                    for (i, e) in pal.iter().flat_map(|p| p.entries.iter()).enumerate() {
                        println!(
                            "    {} tint {:?} spec {:?} gloss {}",
                            ['A', 'B', 'C'][i],
                            e.tint,
                            e.spec,
                            e.glossiness
                        );
                    }
                }
            }
            None => println!("  no look"),
        }
        for p in &v.ports {
            println!("  port {} -> {:?} {:?}", p.name, p.helper, p.offset);
        }
        let mut stack: Vec<(usize, &sc_items_visuals::LoadoutEntry)> =
            v.default_loadout.iter().rev().map(|e| (1, e)).collect();
        while let Some((depth, e)) = stack.pop() {
            println!(
                "  {}loadout {} = {:?}",
                "  ".repeat(depth - 1),
                e.port,
                e.item.as_ref().map(name)
            );
            stack.extend(e.children.iter().rev().map(|c| (depth + 1, c)));
        }
        println!("  orientation {:?}", view.orientation(item, v));
        if let Some(g) = view.light_group(item, v) {
            println!("  light group {}", name(&g));
            for l in view.rig(&g).iter().flat_map(|r| r.lights.iter()) {
                println!(
                    "    {} {} pos {:?} rot {:?} i {} r {} fov {} shadow {}",
                    l.name,
                    l.kind,
                    l.position,
                    l.rotation,
                    l.intensity,
                    l.radius,
                    l.fov,
                    l.casts_shadows
                );
            }
        }
    }
    Ok(())
}
