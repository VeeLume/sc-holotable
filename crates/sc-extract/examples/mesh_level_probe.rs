//! The game's own detail levels against the full mesh on real files: a level
//! must draw only nodes the full mesh has, keep skin joints inside the full
//! skin's bone list, and report the level it came from; asking past the last
//! level gives the coarsest. Paths are archive paths; without any, a capital
//! hull, a component, a weapon and a skin.
//!
//! ```bash
//! cargo run -p sc-extract --release --example mesh_level_probe
//! ```

use std::collections::BTreeSet;

use sc_extract::{AssetSource, MeshGeometry, Skeleton};

const DEFAULT: &[&str] = &[
    "Objects/Spaceships/Ships/AEGS/Idris_Frigate/Exteriors/AEGS_Idris.cga",
    "Objects/Spaceships/Ships/AEGS/Gladius/AEGS_Gladius.cga",
    "objects/fps_weapons/weapons_v7/behr/rifle/p8ar/brfl_fps_behr_p8ar_citcon_body.cgf",
    "objects/characters/human/female_v2/armor/syfb/f_syfb_flightsuit_01.skin",
];

/// Far past the last level any file has.
const BEYOND: u8 = 20;

fn nodes_drawn(g: &MeshGeometry) -> BTreeSet<usize> {
    g.submeshes.iter().filter_map(|s| s.node).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let paths: Vec<&str> = if args.is_empty() {
        DEFAULT.to_vec()
    } else {
        args.iter().map(String::as_str).collect()
    };
    for path in paths {
        println!("\n{path}");
        let full = MeshGeometry::read(&assets, path)?;
        assert_eq!(full.level, 0, "a plain read is the full mesh");
        let bones = path
            .to_ascii_lowercase()
            .ends_with(".skin")
            .then(|| Skeleton::read(&assets, path))
            .transpose()?
            .map(|s| s.bones.len());
        let drawn = nodes_drawn(&full);
        let triangles = |g: &MeshGeometry| g.indices.len() / 3;
        println!("  level 0: {:>8} triangles", triangles(&full));

        let mut last = 0;
        for asked in 1..=BEYOND {
            let g = MeshGeometry::read_level(&assets, path, asked)?;
            assert!(g.level <= asked, "asked {asked}, got coarser {}", g.level);
            if g.level == last {
                continue;
            }
            last = g.level;
            assert_eq!(g.level, asked, "levels have no gaps here");
            assert_eq!(g.nodes, full.nodes, "level {asked}: nodes differ");
            assert!(
                nodes_drawn(&g).is_subset(&drawn),
                "level {asked}: draws a node the full mesh does not"
            );
            assert!(
                triangles(&g) < triangles(&full),
                "level {asked} is not coarser"
            );
            assert_eq!(g.uvs.len(), g.positions.len(), "level {asked}: uvs");
            if let Some(bones) = bones {
                assert_eq!(
                    g.bone_weights.len(),
                    g.positions.len(),
                    "level {asked}: weights"
                );
                let top = g
                    .bone_weights
                    .iter()
                    .flat_map(|w| w.joints.iter().zip(w.weights))
                    .filter(|(_, weight)| *weight > 0)
                    .map(|(joint, _)| usize::from(*joint))
                    .max();
                assert!(top < Some(bones), "level {asked}: joint {top:?} of {bones}");
            }
            let shape = MeshGeometry::read_shape_level(&assets, path, asked)?;
            assert_eq!(shape.level, asked);
            assert_eq!(
                shape.positions, g.positions,
                "level {asked}: shape read differs"
            );
            assert!(shape.uvs.is_empty() && shape.bone_weights.is_empty());
            println!(
                "  level {asked}: {:>8} triangles, {} of {} nodes drawn",
                triangles(&g),
                nodes_drawn(&g).len(),
                drawn.len()
            );
        }
        assert_eq!(
            MeshGeometry::read_level(&assets, path, BEYOND)?.level,
            last,
            "past the end gives the coarsest"
        );
    }
    println!("\nevery level lines up with its full mesh");
    Ok(())
}
