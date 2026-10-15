//! Render inputs of two inventory items, read straight from the archive:
//! geometry nodes and streams ([`MeshGeometry`]), the `.mtl` the submeshes
//! index into ([`Material`]), and the textures it names ([`Texture`]).
//!
//! Re-proves the reader layouts against real files: the Aegis Regulus power
//! plant (`.cga`, material named by its `MtlName` chunk, `LayerBlend_V2` +
//! `Illum`) and the PIPS energy can (`.cga`, material from the DCB path,
//! `Illum`), plus the skeletons two weapons attach their parts to.
//!
//! ```bash
//! cargo run -p sc-extract --release --example render_assets_probe
//! ```

use sc_extract::{AssetSource, Material, MeshGeometry, Skeleton, Texture};

const ITEMS: &[(&str, Option<&str>)] = &[
    (
        "objects/spaceships/power_plants/small_component/aegs/powr_aegs_s01_pl03.cga",
        None,
    ),
    (
        "objects/buildingsets/human/universal/prop/dressing/can/can_drink_3_energy_a.cga",
        Some("objects/buildingsets/human/universal/prop/dressing/can/can_drink_3_a.mtl"),
    ),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;

    for chr in [
        "objects/fps_weapons/weapons_v7/behr/rifle/p4ar/brfl_fps_behr_p4ar.chr",
        "objects/fps_weapons/weapons_v7/gmni/pistol/lh86/bpst_fps_gmni_lh86.chr",
    ] {
        let s = Skeleton::read(&assets, chr)?;
        println!("\n== {chr}: {} bones", s.bones.len());
        for name in ["root", "magAttach", "sight_attachment", "bullet"] {
            if let Some(b) = s.bone(name) {
                println!("  {name:16} pos {:.4?} rot {:.4?}", b.position, b.rotation);
            }
        }
    }

    for &(path, mtl_override) in ITEMS {
        println!("\n== {path}");
        let g = MeshGeometry::read(&assets, path)?;
        println!(
            "vertices {}  normals {}  tangents {}  uvs {}  colors {}  triangles {}",
            g.positions.len(),
            g.normals.len(),
            g.tangents.len(),
            g.uvs.len(),
            g.colors.len(),
            g.indices.len() / 3
        );
        let (lo, hi) = bounds(&g.positions);
        println!("bounds {lo:.3?} .. {hi:.3?}");
        let uv_range = g.uvs.iter().fold([f32::MAX, f32::MIN], |a, uv| {
            [a[0].min(uv[0].min(uv[1])), a[1].max(uv[0].max(uv[1]))]
        });
        println!("uv range {uv_range:.3?}");
        let bad_normals = g
            .normals
            .iter()
            .filter(|n| ((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt() - 1.0).abs() > 0.01)
            .count();
        println!("non-unit normals {bad_normals}");
        let max_index = g.indices.iter().copied().max().unwrap_or(0);
        println!(
            "max index {max_index} (< vertices: {})",
            (max_index as usize) < g.positions.len()
        );
        println!("material chunk {:?}", g.material);
        println!("nodes {}", g.nodes.len());
        for (i, n) in g.nodes.iter().enumerate().take(12) {
            let t = n.transform;
            println!(
                "  [{i}] {:28} parent {:?} t=({:.3},{:.3},{:.3})",
                n.name, n.parent, t[0][3], t[1][3], t[2][3]
            );
        }

        let mtl_path = match mtl_override {
            Some(p) => p.to_string(),
            None => format!("{}.mtl", g.material.clone().ok_or("no material")?),
        };
        let mtl = Material::read(&assets, &mtl_path)?;
        let mut used: Vec<u16> = g.submeshes.iter().map(|s| s.material).collect();
        used.sort_unstable();
        used.dedup();
        println!(
            "submeshes {} using materials {used:?} of {}",
            g.submeshes.len(),
            mtl.submaterials.len()
        );
        for &m in &used {
            let Some(s) = mtl.submaterials.get(m as usize) else {
                println!("  [{m}] MISSING");
                continue;
            };
            println!("  [{m}] {:20} {:14} {:?}", s.name, s.shader, s.features);
            for t in &s.textures {
                let tex = Texture::read(&assets, &t.file);
                match tex {
                    Ok(tex) => println!(
                        "      {:9} {:56} {}x{} {:?} mips {} alpha {} srgb {}",
                        t.slot,
                        short(&t.file),
                        tex.width(),
                        tex.height(),
                        tex.format(),
                        tex.mip_count(),
                        tex.alpha.as_ref().map_or(0, Vec::len),
                        tex.is_srgb()
                    ),
                    Err(e) => println!("      {:9} {:56} ERR {e}", t.slot, short(&t.file)),
                }
            }
            for l in &s.layers {
                println!(
                    "      layer {:12} {} tint {:.2?} palette {} tiling {}",
                    l.name,
                    short(&l.path),
                    l.tint,
                    l.palette_tint,
                    l.uv_tiling
                );
            }
        }
    }
    Ok(())
}

fn bounds(p: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    p.iter()
        .fold(([f32::MAX; 3], [f32::MIN; 3]), |(lo, hi), v| {
            (
                std::array::from_fn(|a| lo[a].min(v[a])),
                std::array::from_fn(|a| hi[a].max(v[a])),
            )
        })
}

fn short(p: &str) -> &str {
    p.rsplit('/').next().unwrap_or(p)
}
