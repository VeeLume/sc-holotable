//! Print items assembled for drawing ([`Visuals::scene`]) by record name: the
//! part tree with each part's geometry, material, skeleton and attach bone,
//! plus the notes. The defaults cover a weapon skin's `.cdf` assembly (its
//! material replaces the assembly's own, the cartridge keeps its own), a rack
//! with its default loadout, and the player actor (a bare `.chr`, body and
//! head skinned on).
//!
//! ```bash
//! cargo run -p sc-items-visuals --release --example scene_probe -- [record name]...
//! ```

use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, CharacterDefinition, Datacore, Guid};
use sc_items::Items;
use sc_items_visuals::{Body, Palettes, Part, Visuals};

const DEFAULT: &[&str] = &[
    "behr_rifle_ballistic_01_sf01",
    "PlayerDeco_Gunrack_4P_3R_lowtech_1_a",
    "Player",
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

    let mut names: HashMap<Guid, String> = HashMap::new();
    let mut by_name: HashMap<String, Guid> = HashMap::new();
    for r in dc.db().all_records() {
        let name = r.name().unwrap_or_default();
        let short = name.split_once('.').map_or(name, |(_, n)| n).to_string();
        by_name.insert(short.to_ascii_lowercase(), r.id());
        names.insert(r.id(), short);
    }

    for w in &wanted {
        let Some(guid) = by_name.get(&w.to_ascii_lowercase()) else {
            println!("\n== {w}: no record");
            continue;
        };
        let Some(scene) = visuals.scene(&palettes, guid, None, Body::Female, |path| {
            CharacterDefinition::read(&assets, path).map_err(|e| e.to_string())
        }) else {
            println!("\n== {w}: no geometry");
            continue;
        };
        println!("\n== {w} (character: {})", scene.is_character());
        print_part(&scene.root, &names, "", 0);
        for n in &scene.notes {
            println!("  note: {n:?}");
        }
    }
    Ok(())
}

fn print_part(part: &Part, names: &HashMap<Guid, String>, bone: &str, depth: usize) {
    let pad = "  ".repeat(depth + 1);
    let label = match (&part.item, &part.cdf_attachment) {
        (Some(g), _) => names.get(g).cloned().unwrap_or_else(|| g.to_string()),
        (None, Some(a)) => format!("[cdf] {a}"),
        (None, None) => "?".into(),
    };
    println!(
        "{pad}{label}{bone} skeleton {:?} palette {:?}",
        part.skeleton,
        part.palette.as_ref().map(|p| &p.name)
    );
    for p in &part.pieces {
        println!("{pad}  piece {} material {:?}", p.geometry, p.material);
    }
    for a in &part.attachments {
        let on = match &a.bone {
            Some(b) => format!(" on {b}"),
            None => " (skinned)".into(),
        };
        print_part(&a.part, names, &on, depth + 1);
    }
}
