//! Re-proves the mesh side of what an overcharge drops (LIVE 4.10.197).
//!
//! Client code (`CBreakableManager::BreakIntoPieces`): a fracture keeps every
//! piece whose user properties say `base`, plus the first `round(n * (1 -
//! danger))` of the others in subobject order, and the dropped pieces' ore is
//! lost. This probe checks, on every ship rock mesh, that the pieces
//! [`Shape`] reads are `child_01`, `child_02`, … in node order (the order the
//! client walks, taken to be the node order: an inference until an
//! overcharged fracture's pieces are scanned in game), that none is `base`,
//! and the ore lost per dropped piece. Then it prints the danger thresholds.
//!
//! ```bash
//! cargo run -p sc-mining --release --example mining_overcharge_probe
//! ```

use sc_extract::{AssetSource, Mesh};
use sc_mining::Shape;

/// Mesh, then the ore lost when the last 1 / 2 / 3 pieces drop, %.
const MESHES: [(&str, [f32; 3]); 4] = [
    (
        "objects/gameplay/mining/asteroids/asteroid_01.cgf",
        [6.9, 20.8, 42.2],
    ),
    (
        "objects/gameplay/mining/asteroids/asteroid_02.cgf",
        [11.3, 29.9, 45.7],
    ),
    (
        "objects/gameplay/mining/asteroids/asteroid_03.cgf",
        [19.0, 36.6, 52.9],
    ),
    (
        "objects/gameplay/mining/rocks/mining_rock_01_breakable.cgf",
        [23.5, 35.6, 45.4],
    ),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    for (path, lost) in MESHES {
        let mesh = Mesh::read(&assets, path)?;
        let shape = Shape::from_mesh(path, &mesh).ok_or("no main node")?;

        // The breakable siblings of `main`, in node order, by name.
        let main = mesh
            .nodes
            .iter()
            .position(|n| n.name == "main")
            .ok_or("no main")?;
        let parent = mesh.nodes[main].parent;
        let children: Vec<&str> = mesh
            .nodes
            .iter()
            .filter(|n| n.parent == parent && n.name.starts_with("child_"))
            .map(|n| n.name.as_str())
            .collect();
        let expected: Vec<String> = (1..=children.len())
            .map(|i| format!("child_{i:02}"))
            .collect();
        assert_eq!(children, expected, "{path}: pieces not in child order");
        assert_eq!(shape.pieces.len(), children.len(), "{path}");
        for (piece, name) in shape.pieces.iter().zip(&children) {
            let node = mesh.nodes.iter().find(|n| n.name == *name).ok_or("node")?;
            assert_eq!(piece.volume, node.volume, "{path}: {name} out of order");
            assert!(!piece.base, "{path}: {name} is a base piece");
        }

        let n = shape.pieces.len();
        for (dropped, want) in (1..=3).zip(lost) {
            // The danger pool that keeps exactly n - dropped pieces.
            let danger = (dropped as f32 - 0.25) / n as f32;
            assert_eq!(shape.kept_pieces(danger), n - dropped, "{path}");
            let got = 100.0 * (1.0 - shape.yield_fraction(danger));
            assert!(
                (got - want).abs() < 0.05,
                "{path}: lost {got:.2} % vs {want}"
            );
        }

        let thresholds: Vec<String> = (0..n)
            .rev()
            .map(|k| {
                // Kept while n * (1 - d) >= k + 0.5; read the yield just
                // inside, clear of float rounding at the boundary.
                let up_to = 1.0 - (k as f32 + 0.5) / n as f32;
                format!(
                    "{} kept up to danger {up_to:.4}: {:.1} % of the ore",
                    k + 1,
                    100.0 * shape.yield_fraction(up_to - 1e-4)
                )
            })
            .collect();
        println!("{path}: {n} pieces\n  {}", thresholds.join("\n  "));
    }
    println!("\nasserts passed: child order, no base pieces, ore lost per dropped piece");
    Ok(())
}
