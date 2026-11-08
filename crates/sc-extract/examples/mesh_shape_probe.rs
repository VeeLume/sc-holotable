//! `MeshGeometry::read_shape` against `read` on real files: everything a
//! shape read keeps must equal the full read's, and what it drops must be
//! empty. Paths are archive paths; without any, a small component and a
//! capital ship's hull.
//!
//! ```bash
//! cargo run -p sc-extract --release --example mesh_shape_probe
//! ```

use std::time::Instant;

use sc_extract::{AssetSource, MeshGeometry};

const DEFAULT: &[&str] = &[
    "objects/spaceships/power_plants/small_component/aegs/powr_aegs_s01_pl03.cga",
    "Objects/Spaceships/Ships/AEGS/Idris_Frigate/Exteriors/AEGS_Idris.cga",
];

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
        // Read once first, so both timings below find the file cached.
        MeshGeometry::read(&assets, path)?;
        let t = Instant::now();
        let full = MeshGeometry::read(&assets, path)?;
        let full_time = t.elapsed();
        let t = Instant::now();
        let shape = MeshGeometry::read_shape(&assets, path)?;
        let shape_time = t.elapsed();

        assert_eq!(shape.positions, full.positions, "positions differ");
        assert_eq!(shape.normals, full.normals, "normals differ");
        assert_eq!(shape.indices, full.indices, "indices differ");
        assert_eq!(shape.submeshes, full.submeshes, "submeshes differ");
        assert_eq!(shape.nodes, full.nodes, "nodes differ");
        assert_eq!(shape.material, full.material, "material differs");
        assert!(shape.uvs.is_empty() && shape.colors.is_empty());
        assert!(shape.tangents.is_empty() && shape.bone_weights.is_empty());
        println!(
            "{:.2} M vertices: read {full_time:.1?}, read_shape {shape_time:.1?}  {path}",
            full.positions.len() as f64 / 1e6
        );
    }
    println!("every kept field equals the full read");
    Ok(())
}
