//! Re-proves the fracture tree read from the rock meshes (LIVE 4.10): every
//! ship rock mesh has a `main` node and `child_NN` pieces, and the iron
//! asteroid Valerie fractured on 2026-10-13 (9.41 SCU, 1344 m³) is
//! `asteroid_03` at scale 0.639: its six pieces predict the scanned 254 / 252 /
//! 235 / 217 / 190 / 186 m³ within 0.5 %. Then prints each mesh's tree.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_mesh_probe
//! ```

use sc_extract::{AssetSource, Mesh};
use sc_mining::Shape;

const MESHES: [&str; 4] = [
    "objects/gameplay/mining/asteroids/asteroid_01.cgf",
    "objects/gameplay/mining/asteroids/asteroid_02.cgf",
    "objects/gameplay/mining/asteroids/asteroid_03.cgf",
    "objects/gameplay/mining/rocks/mining_rock_01_breakable.cgf",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let mut shapes = Vec::new();
    for path in MESHES {
        let shape = Shape::from_mesh(path, &Mesh::read(&assets, path)?).ok_or("no main node")?;
        shapes.push(shape);
    }
    let main: Vec<f32> = shapes.iter().map(|s| s.volume).collect();
    for (got, want) in main.iter().zip([4953.2, 4652.4, 5143.2, 8460.1]) {
        assert!((got - want).abs() < 0.5, "main volume {got} vs {want}");
    }
    assert_eq!(
        shapes.iter().map(|s| s.pieces.len()).collect::<Vec<_>>(),
        [6, 6, 6, 7]
    );

    let iron = &shapes[2];
    let volume: f32 = 9.41 / 0.007;
    let scale = (volume / iron.volume).cbrt();
    let mut predicted: Vec<f32> = iron.scaled(scale).pieces.iter().map(|p| p.volume).collect();
    predicted.sort_by(|a, b| b.total_cmp(a));
    for (p, s) in predicted
        .iter()
        .zip([254.0, 252.0, 235.0, 217.0, 190.0, 186.0])
    {
        assert!((p - s).abs() / s < 5e-3, "piece {p} vs scanned {s}");
    }
    println!(
        "asserts passed: main volumes, piece counts, iron asteroid pieces (scale {scale:.4})\n"
    );

    for shape in &shapes {
        println!("{}: main {:.1} m3", shape.path, shape.volume);
        for piece in &shape.pieces {
            let sub: Vec<String> = piece
                .pieces
                .iter()
                .map(|p| format!("{:.0}", p.volume))
                .collect();
            println!("  piece {:>7.1} m3 -> [{}]", piece.volume, sub.join(", "));
        }
    }
    Ok(())
}
