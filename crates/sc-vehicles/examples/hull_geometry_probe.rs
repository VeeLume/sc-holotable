//! Explores how a hull's geometry lines up with its damage parts and ports
//! (RSI Constellation Mk V Centaurus, LIVE 4.10.197): which vehicle-XML
//! parts have a mesh node of the same name and how many triangles each
//! carries, and where the hardpoint helpers sit.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example hull_geometry_probe
//! ```

use std::collections::HashMap;

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, Guid, MeshGeometry, RecordCollection,
};
use sc_items::Items;
use sc_vehicles::{GeometryPaths, HullGeometry, PortContainers, Vehicles, apply};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), r.name()?.rsplit('.').next()?.to_string())))
        .collect();
    let ship_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "RSI_Constellation_Mk5_Centaurus".into());
    let (_, ship) = vehicles
        .iter()
        .find(|(g, _)| names.get(g) == Some(&ship_name))
        .ok_or("no ship")?;
    let path = ship.geometry.as_deref().ok_or("no geometry")?;
    let g = MeshGeometry::read(&assets, path)?;
    println!(
        "{path}: {} nodes, {} vertices, {} triangles, {} submeshes",
        g.nodes.len(),
        g.positions.len(),
        g.indices.len() / 3,
        g.submeshes.len()
    );

    // Triangles per node.
    let mut tris: HashMap<usize, u32> = HashMap::new();
    for s in &g.submeshes {
        if let Some(n) = s.node {
            *tris.entry(n).or_default() += s.index_count / 3;
        }
    }
    let by_name: HashMap<String, usize> = g
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.name.to_ascii_lowercase(), i))
        .collect();

    println!("\ndamage parts (damageMax > 0) vs mesh nodes:");
    let mut matched = 0;
    for p in ship.hull.iter().filter(|p| p.damage_max > 0.0) {
        let node = by_name.get(&p.name.to_ascii_lowercase());
        if node.is_some() {
            matched += 1;
        }
        println!(
            "  {:<40} {:>8} HP  node {:?}  triangles {}",
            p.name,
            p.damage_max,
            node,
            node.and_then(|n| tris.get(n)).copied().unwrap_or(0)
        );
    }
    println!(
        "{matched} of {} damage parts have a node",
        ship.hull.iter().filter(|p| p.damage_max > 0.0).count()
    );

    // Which nodes carry the triangles: top 25.
    let mut heavy: Vec<(&usize, &u32)> = tris.iter().collect();
    heavy.sort_by(|a, b| b.1.cmp(a.1));
    println!("\nnodes with the most triangles:");
    for (n, t) in heavy.iter().take(25) {
        let node = &g.nodes[**n];
        let parent = node.parent.map(|p| g.nodes[p].name.as_str()).unwrap_or("-");
        println!("  {:<48} {t:>7}  parent {parent}", node.name);
    }

    // Node names that look like physics proxies or LODs, and the node
    // properties of a few damage parts.
    let proxies: Vec<&str> = g
        .nodes
        .iter()
        .map(|n| n.name.as_str())
        .filter(|n| {
            let l = n.to_ascii_lowercase();
            l.contains("proxy") || l.contains("phys") || l.contains("lod") || l.contains("hitbox")
        })
        .take(20)
        .collect();
    println!("\nproxy / lod-like nodes: {proxies:?}");
    let mats: std::collections::BTreeSet<u16> = g.submeshes.iter().map(|s| s.material).collect();
    println!("material ids: {mats:?} (mtl {:?})", g.material);
    for n in g
        .nodes
        .iter()
        .filter(|n| !n.properties.trim().is_empty())
        .take(8)
    {
        println!("  props {}: {:?}", n.name, n.properties.trim());
    }

    // Hardpoint helpers.
    let helpers: Vec<&str> = g
        .nodes
        .iter()
        .filter(|n| n.name.starts_with("hardpoint_"))
        .map(|n| n.name.as_str())
        .collect();
    let ports: Vec<&str> = ship.ports.iter().map(|p| p.name.as_str()).collect();
    let missing: Vec<&&str> = ports
        .iter()
        .filter(|p| !by_name.contains_key(&p.to_ascii_lowercase()))
        .collect();
    println!(
        "\n{} hardpoint helpers, {} ports, {} ports without a helper: {missing:?}",
        helpers.len(),
        ports.len(),
        missing.len()
    );

    // The library's HullGeometry: parts and placements.
    let paths = GeometryPaths::build(&datacore);
    let hull =
        HullGeometry::read(&assets, ship, &ship.stock, &vehicles, &paths).ok_or("no hull")?;
    let triangles: usize = hull.parts.iter().map(|p| p.indices.len() / 3).sum();
    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
    for p in &hull.parts {
        for v in &p.positions {
            for k in 0..3 {
                lo[k] = lo[k].min(v[k]);
                hi[k] = hi[k].max(v[k]);
            }
        }
    }
    println!(
        "
HullGeometry: {} parts, {triangles} triangles, hull box {lo:?} .. {hi:?}; {} placements",
        hull.parts.len(),
        hull.placements.len()
    );
    assert_eq!(triangles, g.indices.len() / 3);
    let inside = |p: [f32; 3]| (0..3).all(|k| p[k] >= lo[k] && p[k] <= hi[k]);
    for pl in &hull.placements {
        let n = names.get(&pl.item).map_or("?", String::as_str);
        let keep = [
            "POWR",
            "COOL",
            "SHLD",
            "QDRV",
            "RADR",
            "LFSP",
            "KLWE",
            "Mount_Gimbal",
            "MISL",
            "MRCK",
        ]
        .iter()
        .any(|k| n.contains(k));
        if !keep || n.contains("MISL") && !pl.path.ends_with("_01") {
            continue;
        }
        let at = apply(&pl.transform, [0.0; 3]);
        let size = pl
            .bounds
            .map(|b| std::array::from_fn::<f32, 3, _>(|k| b.max[k] - b.min[k]));
        println!(
            "  {:<60} {:<36} at ({:+7.2}, {:+7.2}, {:+7.2}) size {:?}{}",
            pl.path,
            n,
            at[0],
            at[1],
            at[2],
            size.map(|s| s.map(|v| (v * 100.0).round() / 100.0)),
            if inside(at) { "" } else { "  OUTSIDE HULL BOX" }
        );
    }
    Ok(())
}
