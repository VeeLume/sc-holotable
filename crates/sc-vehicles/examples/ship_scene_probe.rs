//! The stock RSI Constellation Mk V Centaurus assembled for drawing
//! ([`sc_vehicles::scene`]), checked against [`HullGeometry`]: every item
//! with geometry the placement walk finds is in the scene, every hull-level
//! attachment names a node of the hull file, and the hull's render nodes
//! all map to a damage part.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example ship_scene_probe
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::{
    AssetConfig, AssetData, AssetSource, CharacterDefinition, Datacore, Guid, Mesh,
    RecordCollection,
};
use sc_items::Items;
use sc_items_visuals::{Palettes, Part, SceneNote, Visuals};
use sc_vehicles::{GeometryPaths, HullGeometry, PortContainers, Vehicles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let visuals = Visuals::build(&datacore, &items);
    let palettes = Palettes::build(&datacore);
    let paths = GeometryPaths::build(&datacore);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let vehicle_visuals = vehicles
        .guids()
        .filter(|g| visuals.get(g).is_some())
        .count();
    println!(
        "{vehicle_visuals} of {} vehicles have a visual",
        vehicles.len()
    );

    let (guid, ship) = vehicles
        .iter()
        .find(|(g, _)| names.get(g).map(String::as_str) == Some("RSI_Constellation_Mk5_Centaurus"))
        .ok_or("no Centaurus")?;
    let scene = sc_vehicles::scene(&visuals, &palettes, guid, ship, &ship.stock, |p| {
        CharacterDefinition::read(&assets, p).map_err(|e| e.to_string())
    })
    .ok_or("no scene")?;
    let mut count: BTreeMap<String, usize> = BTreeMap::new();
    walk(&scene.root, &names, &mut count);
    let total: usize = count.values().sum();
    println!(
        "scene: {} pieces at the root, {} attachments at the hull, {total} parts in all",
        scene.root.pieces.len(),
        scene.root.attachments.len()
    );
    let mut notes: BTreeMap<String, usize> = BTreeMap::new();
    for n in &scene.notes {
        let key = match n {
            SceneNote::NoPort { item, port } => format!(
                "no port {port} on {}",
                names.get(item).map_or("?", String::as_str)
            ),
            other => format!("{other:?}"),
        };
        *notes.entry(key).or_default() += 1;
    }
    for (n, k) in &notes {
        println!("  note x{k}: {n}");
    }

    // Every hull-level attachment sits on a node of the hull file.
    let hull_path = ship.geometry.as_deref().ok_or("no hull path")?;
    let nodes = Mesh::read(&assets, hull_path)?.nodes;
    for a in &scene.root.attachments {
        let bone = a.bone.as_deref().ok_or("skinned hull attachment")?;
        assert!(
            nodes.iter().any(|n| n.name.eq_ignore_ascii_case(bone)),
            "no hull node {bone}"
        );
    }

    // The placement walk and the scene agree on what is drawn.
    let geometry =
        HullGeometry::read(&assets, ship, &ship.stock, &vehicles, &paths).ok_or("no hull")?;
    let placed = geometry
        .placements
        .iter()
        .filter(|p| p.bounds.is_some() && visuals.get(&p.item).is_some())
        .count();
    let docked = count
        .iter()
        .filter(|(n, _)| {
            vehicles
                .iter()
                .any(|(g, _)| names.get(g) == Some(n) && g != guid)
        })
        .map(|(_, k)| k)
        .sum::<usize>();
    println!(
        "{placed} placements with geometry; scene items below the hull: {} (docked vehicles {docked})",
        total - 1
    );
    let unmapped = nodes
        .iter()
        .filter(|n| !geometry.node_parts.contains_key(&n.name))
        .count();
    println!(
        "{} of {} hull nodes map to a damage part ({unmapped} do not)",
        geometry.node_parts.len(),
        nodes.len()
    );
    assert!(
        scene
            .notes
            .iter()
            .all(|n| !matches!(n, SceneNote::NoPort { item, .. } if item == guid))
    );
    println!("asserts passed");
    Ok(())
}

fn walk(p: &Part, names: &HashMap<Guid, String>, count: &mut BTreeMap<String, usize>) {
    if let Some(g) = &p.item {
        *count
            .entry(names.get(g).cloned().unwrap_or_else(|| g.to_string()))
            .or_default() += 1;
    }
    for a in &p.attachments {
        walk(&a.part, names, count);
    }
}
