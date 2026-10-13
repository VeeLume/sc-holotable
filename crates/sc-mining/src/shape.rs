//! A rock's fracture tree, read from its mesh.

use sc_extract::Mesh;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::Ruleset;

/// The engine's name for a breakable mesh's intact object; its siblings are
/// the pieces it breaks into. A CryEngine breakable convention with no typed
/// counterpart in the data, so it is matched by name (and warned on when
/// missing).
const MAIN_NODE: &str = "main";
/// The user property that keeps a node out of the break (effect meshes).
const NON_BREAKABLE: &str = "nonBreakable";

/// A rock mesh's fracture tree: the intact volume and the pieces it breaks
/// into, each with the pieces it breaks into in turn.
///
/// Client code and data (LIVE 4.10): a fracture spawns one child rock per
/// piece of the breakable mesh (`BreakableComponentParams.numPiecesOverride`
/// is 0 on every ship rock), each a mineable of its own with the piece's
/// volume times the parent's scale³. Ship rock meshes have 6 or 7 pieces,
/// each with 2 to 8 sub-pieces of 56 to 325 m³ at scale 1, so below the 330 m³
/// collectable threshold at any spawn scale: at most two cracks to
/// collectable pieces. An overcharged fracture spawns fewer pieces; which
/// ones is not modelled.
///
/// Volumes are the render mesh's enclosed volume. The client takes the
/// physics geometry's; the render volumes reproduce scanned fragments to
/// 0.3 % (`mining_mesh_probe`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    /// The geometry file.
    pub path: String,
    /// m³ of the intact rock at scale 1.
    pub volume: f32,
    pub pieces: Vec<Piece>,
}

/// One piece of a [`Shape`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Piece {
    /// m³.
    pub volume: f32,
    /// What this piece breaks into (empty: it does not break further).
    pub pieces: Vec<Piece>,
}

impl Shape {
    /// The fracture tree of a breakable mesh; `None` without a `main` node.
    pub fn from_mesh(path: &str, mesh: &Mesh) -> Option<Self> {
        let Some(main) = mesh.nodes.iter().position(|n| n.name == MAIN_NODE) else {
            warn!(path, "rock mesh without a '{MAIN_NODE}' node");
            return None;
        };
        let breakable = |i: usize| {
            let n = &mesh.nodes[i];
            n.volume > 0.0 && !n.properties.lines().any(|l| l.trim() == NON_BREAKABLE)
        };
        fn piece(mesh: &Mesh, i: usize, breakable: &dyn Fn(usize) -> bool) -> Piece {
            Piece {
                volume: mesh.nodes[i].volume,
                pieces: mesh
                    .children(i)
                    .filter(|&c| breakable(c))
                    .map(|c| piece(mesh, c, breakable))
                    .collect(),
            }
        }
        let siblings: Vec<usize> = match mesh.nodes[main].parent {
            Some(parent) => mesh.children(parent).collect(),
            None => (0..mesh.nodes.len())
                .filter(|&i| mesh.nodes[i].parent.is_none())
                .collect(),
        };
        Some(Self {
            path: path.to_string(),
            volume: mesh.nodes[main].volume,
            pieces: siblings
                .into_iter()
                .filter(|&i| i != main && breakable(i))
                .map(|i| piece(mesh, i, &breakable))
                .collect(),
        })
    }

    /// The tree at spawn scale `scale` (volumes times `scale³`).
    pub fn scaled(&self, scale: f32) -> Self {
        let k = scale.powi(3);
        fn scale_piece(p: &Piece, k: f32) -> Piece {
            Piece {
                volume: p.volume * k,
                pieces: p.pieces.iter().map(|c| scale_piece(c, k)).collect(),
            }
        }
        Self {
            path: self.path.clone(),
            volume: self.volume * k,
            pieces: self.pieces.iter().map(|p| scale_piece(p, k)).collect(),
        }
    }

    /// Fractures needed until every piece can be collected directly (the
    /// rock itself counts; 0 when the whole rock is collectable). `None`
    /// when some piece is too big to collect and does not break further.
    pub fn cracks_to_collect(&self, ruleset: &Ruleset) -> Option<u32> {
        cracks(self.volume, &self.pieces, ruleset)
    }

    /// The pieces that end up collectable after every needed fracture, m³.
    pub fn collectable_pieces(&self, ruleset: &Ruleset) -> Vec<f32> {
        let mut out = Vec::new();
        leaves(self.volume, &self.pieces, ruleset, &mut out);
        out
    }
}

fn cracks(volume: f32, pieces: &[Piece], ruleset: &Ruleset) -> Option<u32> {
    if ruleset.is_absorbable(volume) {
        return Some(0);
    }
    if pieces.is_empty() {
        return None;
    }
    let mut n = 1;
    for p in pieces {
        n += cracks(p.volume, &p.pieces, ruleset)?;
    }
    Some(n)
}

fn leaves(volume: f32, pieces: &[Piece], ruleset: &Ruleset, out: &mut Vec<f32>) {
    if ruleset.is_absorbable(volume) || pieces.is_empty() {
        if ruleset.is_absorbable(volume) {
            out.push(volume);
        }
        return;
    }
    for p in pieces {
        leaves(p.volume, &p.pieces, ruleset, out);
    }
}
