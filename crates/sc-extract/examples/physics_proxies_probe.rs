//! Physics proxies ([`PhysicsProxies`]) of the RSI Constellation Mk V hull
//! and two components, read straight from the archive.
//!
//! Re-proves the `STATOBJ_PHYSICS` layout (LIVE 4.10): the hull has 36
//! slot-0 proxies (22 trimeshes, 14 boxes) on its damage-part nodes; every
//! trimesh is closed (each edge shared by exactly two triangles) and
//! encloses the volume its header states; every box's volume is
//! 8 × hx × hy × hz; `geo_body`'s proxy has 2,808 triangles. The Sakura
//! Sun S3 power plant's box spans 1.496 × 1.078 × 0.955 half extents.
//!
//! ```bash
//! cargo run -p sc-extract --release --example physics_proxies_probe
//! ```

use std::collections::HashMap;

use sc_extract::{AssetSource, Mesh, PhysicsProxies, ProxyKind, ProxyShape};

const HULL: &str =
    "objects/spaceships/ships/rsi/constellation_mk5/exterior/rsi_constellation_mk5.cga";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;

    let nodes = Mesh::read(&assets, HULL)?.nodes;
    let proxies = PhysicsProxies::read(&assets, HULL)?.proxies;
    let default: Vec<_> = proxies
        .iter()
        .filter(|p| p.kind == ProxyKind::Default)
        .collect();
    let (mut meshes, mut boxes) = (0, 0);
    for p in &default {
        let name = p.node.map_or("<root>", |n| nodes[n].name.as_str());
        let (pos, idx) = p.shape.triangles();
        let enclosed = volume(&pos, &idx).abs();
        match &p.shape {
            ProxyShape::Mesh { .. } => {
                meshes += 1;
                let open = open_edges(&idx);
                println!(
                    "  {name:36} mesh {:5} tris, {enclosed:9.2} m3 (stated {:9.2}), {open} open edges",
                    idx.len() / 3,
                    p.volume
                );
                assert_eq!(open, 0, "{name} is not closed");
                assert!((enclosed - p.volume).abs() <= 1e-3 * p.volume.max(1.0));
                if name == "geo_body" {
                    assert_eq!(idx.len() / 3, 2808);
                }
            }
            ProxyShape::Box { half, .. } => {
                boxes += 1;
                println!(
                    "  {name:36} box  half {half:.3?}, {enclosed:9.3} m3 (stated {:9.3})",
                    p.volume
                );
                assert!((8.0 * half[0] * half[1] * half[2] - p.volume).abs() <= 1e-3);
            }
            ProxyShape::Other { geometry_type } => println!("  {name:36} type {geometry_type}"),
        }
    }
    println!(
        "{HULL}: {} proxies ({} slot 0: {meshes} meshes, {boxes} boxes)",
        proxies.len(),
        default.len()
    );
    assert_eq!((default.len(), meshes, boxes), (36, 22, 14));

    let plant = "objects/spaceships/power_plants/large_component/sasu/powr_sasu_s03_pl01.cga";
    let p = PhysicsProxies::read(&assets, plant)?;
    for x in &p.proxies {
        println!("{plant}: {:?} {:?}", x.kind, x.shape);
    }
    let Some(ProxyShape::Box { axes, half, .. }) = p.proxies.first().map(|x| &x.shape) else {
        return Err("no box on the power plant".into());
    };
    // The box is rotated in its node; along the node's axes it spans the
    // render bounds' 1.496 x 1.078 x 0.955 half extents.
    let extent: [f32; 3] =
        std::array::from_fn(|k| (0..3).map(|a| half[a] * axes[a][k].abs()).sum());
    println!("  node-frame half extents {extent:.3?}");
    let want = [1.496, 1.078, 0.955];
    assert!((0..3).all(|k| (extent[k] - want[k]).abs() < 1e-3));
    println!("asserts passed");
    Ok(())
}

/// Volume of a closed triangle list (divergence theorem).
fn volume(p: &[[f32; 3]], idx: &[u32]) -> f32 {
    idx.chunks(3)
        .map(|t| {
            let [a, b, c] = [p[t[0] as usize], p[t[1] as usize], p[t[2] as usize]];
            f64::from(
                a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
                    + a[2] * (b[0] * c[1] - b[1] * c[0]),
            ) / 6.0
        })
        .sum::<f64>() as f32
}

/// Edges not shared by exactly two triangles.
fn open_edges(idx: &[u32]) -> usize {
    let mut uses: HashMap<(u32, u32), u32> = HashMap::new();
    for t in idx.chunks(3) {
        for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
            *uses.entry((a.min(b), a.max(b))).or_default() += 1;
        }
    }
    uses.values().filter(|&&n| n != 2).count()
}
