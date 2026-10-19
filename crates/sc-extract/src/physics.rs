//! A geometry file's physics proxies: the closed collision shapes the
//! engine physicalizes, per node ([`PhysicsProxies`]).
//!
//! The render mesh ([`crate::MeshGeometry`]) is not closed everywhere (the
//! Constellation Mk V's belly under its power plants is open); the proxies
//! are. Read from the client (LIVE 4.10.197: loader `FUN_140653d30`,
//! `CStatObj::Physicalize_0x900` `FUN_146f32e50`, `CTriMesh::Load`
//! `FUN_146417410`) and checked on the Centaurus: every trimesh proxy is
//! closed and encloses exactly the volume its header states.
//!
//! Layout: the `STATOBJ_PHYSICS` chunk (`0x58DE1772`) of the geometry file
//! itself (not its `m` companion) starts with ten `u32` slot counts and
//! their total (+0x28); from +0x50 one 8-byte entry per block (`u32` size,
//! `u16` node, `u16` proxy number), then the blocks, slot by slot. Slots
//! 0–3 hold physics geometry ([`ProxyKind`]); slot 4 a cluster mesh (the
//! render mesh again), 7 a distance field; those are not read. A geometry
//! block is `u32` version 2, a 64-byte header (inertia, orientation, origin,
//! volume at +48), the geometry type at +68 and its data from +72:
//!
//! - box (0): `f32 × 9` basis (rows are the box axes), `i32` oriented,
//!   centre, half extents, in the node's frame;
//! - trimesh (1): `i32` vertex / triangle counts, valency, flags, map
//!   count, `u8` has-foreign-ids, has-ids, then the `u16` map, optional
//!   `u16` foreign ids and `u8` material ids per triangle, vertices
//!   (`f32 × 3`, absent with flag 1) and `u16` indices (absent with flag 2).
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let path = "objects/spaceships/ships/rsi/constellation_mk5/exterior/rsi_constellation_mk5.cga";
//! let proxies = sc_extract::PhysicsProxies::read(&assets, path)?;
//! for p in proxies.proxies.iter().filter(|p| p.kind.hits_rays()) {
//!     println!("node {:?}: {:.1} m3", p.node, p.volume);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::mesh::{chunk, parse_nodes, read_data};
use crate::{AssetSource, Error, Result};

const STATOBJ_PHYSICS: u32 = 0x58DE_1772;
const SLOTS: usize = 10;
const TABLE: usize = 0x50;

/// Every physics proxy of one geometry file.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhysicsProxies {
    pub proxies: Vec<PhysicsProxy>,
}

/// One collision shape, in its node's frame.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicsProxy {
    /// Index into the file's nodes ([`crate::MeshGeometry::nodes`]); `None`
    /// for the root object.
    pub node: Option<usize>,
    pub kind: ProxyKind,
    pub shape: ProxyShape,
    /// m³, as the file states it.
    pub volume: f32,
}

/// The physics slot a proxy sits in, which decides what it collides with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyKind {
    /// Slot 0: collides with everything, rays included.
    Default,
    /// Slot 1: no collision, rays only.
    NoCollide,
    /// Slot 2: obstructs (sight), no rays.
    Obstruct,
    /// Slot 3 (`0x1003`, meaning not read).
    Other,
}

impl ProxyKind {
    /// Whether ray queries (projectiles) test it: the `geom_colltype_ray`
    /// flag the client sets for slots 0 and 1.
    pub fn hits_rays(self) -> bool {
        matches!(self, ProxyKind::Default | ProxyKind::NoCollide)
    }
}

/// A proxy's geometry.
#[derive(Debug, Clone, PartialEq)]
pub enum ProxyShape {
    /// A box: `centre + Σ s_i × half[i] × axes[i]` for `s_i` in ±1.
    Box {
        axes: [[f32; 3]; 3],
        centre: [f32; 3],
        half: [f32; 3],
    },
    /// A closed triangle mesh.
    Mesh {
        positions: Vec<[f32; 3]>,
        indices: Vec<u32>,
    },
    /// A shape this reader does not decode (sphere, cylinder, capsule, …,
    /// or a mesh whose vertices live elsewhere).
    Other { geometry_type: i32 },
}

impl ProxyShape {
    /// The shape as a triangle list (a box as 12 triangles); empty for
    /// [`ProxyShape::Other`].
    pub fn triangles(&self) -> (Vec<[f32; 3]>, Vec<u32>) {
        match self {
            ProxyShape::Box { axes, centre, half } => {
                let positions = (0..8)
                    .map(|i| {
                        let s = [
                            if i & 1 == 0 { -1.0 } else { 1.0 },
                            if i & 2 == 0 { -1.0 } else { 1.0 },
                            if i & 4 == 0 { -1.0 } else { 1.0 },
                        ];
                        std::array::from_fn(|k| {
                            centre[k] + (0..3).map(|a| s[a] * half[a] * axes[a][k]).sum::<f32>()
                        })
                    })
                    .collect();
                let faces = [
                    [0, 2, 3, 1],
                    [4, 5, 7, 6],
                    [0, 1, 5, 4],
                    [2, 6, 7, 3],
                    [0, 4, 6, 2],
                    [1, 3, 7, 5],
                ];
                let indices = faces
                    .iter()
                    .flat_map(|f| [f[0], f[1], f[2], f[0], f[2], f[3]])
                    .collect();
                (positions, indices)
            }
            ProxyShape::Mesh { positions, indices } => (positions.clone(), indices.clone()),
            ProxyShape::Other { .. } => (Vec::new(), Vec::new()),
        }
    }
}

impl PhysicsProxies {
    /// Read `path`'s proxies from the archive (the geometry file itself).
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        Self::parse(&read_data(assets, path)?)
    }

    /// Parse a geometry file; one without a physics chunk has no proxies.
    pub fn parse(geometry: &[u8]) -> Result<Self> {
        let Ok(d) = chunk(geometry, STATOBJ_PHYSICS) else {
            return Ok(Self::default());
        };
        let nodes = parse_nodes(geometry).map_or(0, |n| n.len());
        let counts: Vec<usize> = (0..SLOTS)
            .map(|i| u32_at(d, i * 4).map(|c| c as usize))
            .collect::<Result<_>>()?;
        let total = u32_at(d, 0x28)? as usize;
        if counts.iter().sum::<usize>() != total {
            return Err(bad("slot counts do not add up"));
        }
        let mut at = TABLE + total * 8;
        let mut proxies = Vec::new();
        let mut entry = 0;
        for (slot, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                let e = TABLE + entry * 8;
                entry += 1;
                let size = u32_at(d, e)? as usize;
                let node = u16_at(d, e + 4)? as usize;
                let block = d
                    .get(at..at.checked_add(size).ok_or_else(|| bad("overflow"))?)
                    .ok_or_else(|| bad("block past chunk end"))?;
                at += size;
                let kind = match slot {
                    0 => ProxyKind::Default,
                    1 => ProxyKind::NoCollide,
                    2 => ProxyKind::Obstruct,
                    3 => ProxyKind::Other,
                    _ => continue,
                };
                let Some((shape, volume)) = geometry_block(block)? else {
                    continue;
                };
                proxies.push(PhysicsProxy {
                    node: (node < nodes).then_some(node),
                    kind,
                    shape,
                    volume,
                });
            }
        }
        Ok(Self { proxies })
    }
}

/// A slot 0–3 block: its shape and stated volume; `None` for an unknown
/// block version.
fn geometry_block(b: &[u8]) -> Result<Option<(ProxyShape, f32)>> {
    if u32_at(b, 0)? != 2 {
        return Ok(None);
    }
    let volume = f32_at(b, 48)?;
    let geometry_type = u32_at(b, 68)? as i32;
    let q = 72;
    let shape = match geometry_type {
        0 => {
            let v3 = |p: usize| -> Result<[f32; 3]> {
                Ok([f32_at(b, p)?, f32_at(b, p + 4)?, f32_at(b, p + 8)?])
            };
            ProxyShape::Box {
                axes: [v3(q)?, v3(q + 12)?, v3(q + 24)?],
                centre: v3(q + 40)?,
                half: v3(q + 52)?,
            }
        }
        1 => {
            let n_verts = u32_at(b, q)? as usize;
            let n_tris = u32_at(b, q + 4)? as usize;
            let flags = u32_at(b, q + 12)?;
            let n_map = u32_at(b, q + 16)? as usize;
            let has_foreign = *b.get(q + 20).ok_or_else(|| bad("truncated"))? != 0;
            let has_ids = *b.get(q + 21).ok_or_else(|| bad("truncated"))? != 0;
            if flags & 3 != 0 {
                return Ok(Some((ProxyShape::Other { geometry_type }, volume)));
            }
            let mut p = q + 22 + 2 * n_map;
            if has_foreign {
                p += 2 * n_tris;
            }
            if has_ids {
                p += n_tris;
            }
            let positions = (0..n_verts)
                .map(|i| {
                    let o = p + i * 12;
                    Ok([f32_at(b, o)?, f32_at(b, o + 4)?, f32_at(b, o + 8)?])
                })
                .collect::<Result<Vec<_>>>()?;
            p += 12 * n_verts;
            let indices = (0..3 * n_tris)
                .map(|i| {
                    let v = u32::from(u16_at(b, p + 2 * i)?);
                    if v as usize >= n_verts {
                        return Err(bad("index out of range"));
                    }
                    Ok(v)
                })
                .collect::<Result<Vec<_>>>()?;
            ProxyShape::Mesh { positions, indices }
        }
        _ => ProxyShape::Other { geometry_type },
    };
    Ok(Some((shape, volume)))
}

fn bad(what: &str) -> Error {
    Error::Mesh(format!("physics: {what}"))
}

fn u32_at(d: &[u8], p: usize) -> Result<u32> {
    d.get(p..p + 4)
        .map(|b| u32::from_le_bytes(b.try_into().expect("4 bytes")))
        .ok_or_else(|| bad("truncated"))
}

fn u16_at(d: &[u8], p: usize) -> Result<u16> {
    d.get(p..p + 2)
        .map(|b| u16::from_le_bytes(b.try_into().expect("2 bytes")))
        .ok_or_else(|| bad("truncated"))
}

fn f32_at(d: &[u8], p: usize) -> Result<f32> {
    u32_at(d, p).map(f32::from_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Signed volume of a closed triangle list.
    fn volume(p: &[[f32; 3]], idx: &[u32]) -> f32 {
        idx.chunks(3)
            .map(|t| {
                let [a, b, c] = [p[t[0] as usize], p[t[1] as usize], p[t[2] as usize]];
                (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
                    + a[2] * (b[0] * c[1] - b[1] * c[0]))
                    / 6.0
            })
            .sum()
    }

    #[test]
    fn a_box_is_closed_and_outward() {
        let s = ProxyShape::Box {
            axes: [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            centre: [1.0, 2.0, 3.0],
            half: [1.0, 2.0, 0.5],
        };
        let (p, i) = s.triangles();
        assert_eq!((p.len(), i.len()), (8, 36));
        assert!((volume(&p, &i) - 8.0).abs() < 1e-4, "{}", volume(&p, &i));
    }

    #[test]
    fn no_chunk_no_proxies() {
        let mut f = b"#ivo".to_vec();
        f.extend([0u8; 4]);
        f.extend(0u32.to_le_bytes());
        f.extend(16u32.to_le_bytes());
        assert_eq!(
            PhysicsProxies::parse(&f).unwrap(),
            PhysicsProxies::default()
        );
    }
}
