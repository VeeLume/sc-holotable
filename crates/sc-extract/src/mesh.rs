//! IVO geometry files (`.cgf` / `.cga`): the node tree and the volume each
//! node's mesh encloses ([`Mesh`]), or everything needed to draw it
//! ([`MeshGeometry`]).
//!
//! A geometry file holds the node tree (`NodeMeshCombo` chunk, `0x70697FDA`)
//! and names a companion (`x.cgf` → `x.cgfm`) whose `IvoSkin2` chunk
//! (`0xB8757777`) carries the triangles. Each submesh there names its node
//! (high 16 bits of its first word), so the volume a node's triangles enclose
//! follows from the divergence theorem. That is what breakable rocks need:
//! their `child_NN` nodes are the pieces they break into (see `sc-mining`).
//!
//! Layout (StarBreaker's reader, MIT, cross-checked on the mining meshes):
//! `IvoSkin2` = flags `u32`, mesh info (vertex / index / submesh counts at
//! +8 / +12 / +16, model bbox, scaling bbox at +48 / +60), submeshes from +168
//! at 48 bytes (first index, index count, page base at +4 / +8 / +16), then
//! tagged streams: positions (`0x91329AE9` / `0xB3A70D5E`; 16-byte quantised
//! `i16` over the scaling box, or 20-byte `f32`) and indices (`0xEECDC168`,
//! `u16` plus the submesh page base, or `u32`).
//!
//! Render streams ([`MeshGeometry`]): each vertex also carries a BGRA colour
//! and a half-float UV pair (bytes 8..16 of a quantised vertex, 12..20 of a
//! float one). Normals come from `0x38A581FE` (`f32 × 3`, when 12 bytes wide),
//! else `0x9CF3F615` (one `u32`, 15-15-1-1 packed unit vector), else from the
//! tangent frame: `0xB95E9A1B` is an SNorm `i16 × 4` quaternion (normal =
//! column 2, negated when `w < 0`), `0xEE057252` two packed unit vectors
//! (tangent, bitangent; bit 31 of the first is the bitangent sign). A
//! submesh's low 16 bits are its index into the `.mtl` submaterials; the
//! `MtlName` chunk (`0x83353333`, 128 bytes) names that `.mtl` without its
//! extension. Node transforms are the `BoneToWorld` 3 × 4 at node entry +80,
//! relative to the parent node.
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let mesh = sc_extract::Mesh::read(&assets, "objects/gameplay/mining/asteroids/asteroid_01.cgf")?;
//! for node in &mesh.nodes {
//!     println!("{} {:.1} m3", node.name, node.volume);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::{AssetSource, Error, Result};

const IVO_MAGIC: &[u8; 4] = b"#ivo";
const NODE_MESH_COMBO: u32 = 0x7069_7FDA;
const IVO_SKIN2: u32 = 0xB875_7777;
const POSITIONS: u32 = 0x9132_9AE9;
const POSITIONS2: u32 = 0xB3A7_0D5E;
const INDICES: u32 = 0xEECD_C168;
const NORMALS_PACKED: u32 = 0x9CF3_F615;
const NORMALS_FLOAT: u32 = 0x38A5_81FE;
const TANGENTS_QUAT: u32 = 0xB95E_9A1B;
const TANGENTS_PACKED: u32 = 0xEE05_7252;
const MTL_NAME: u32 = 0x8335_3333;

/// A geometry file's node tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub nodes: Vec<MeshNode>,
}

/// One node of a [`Mesh`].
#[derive(Debug, Clone, PartialEq)]
pub struct MeshNode {
    pub name: String,
    /// Index into [`Mesh::nodes`].
    pub parent: Option<usize>,
    /// The node's user properties (`key=value` lines, e.g. `nonBreakable`).
    pub properties: String,
    /// m³ enclosed by the node's own triangles, in the file's units; 0 for
    /// a node without geometry.
    pub volume: f32,
    /// Placement relative to the parent node, 3 × 4 row-major (rotation /
    /// scale in the first three columns, translation in the fourth).
    pub transform: [[f32; 4]; 3],
}

/// Everything needed to draw a geometry file: its nodes, the vertex and
/// index streams, and which `.mtl` submaterial each part uses.
///
/// Vertex positions are in their node's space; place a submesh with the
/// chain of [`MeshNode::transform`]s from its node up to the root. A file
/// without a node tree (`.skin`) has no nodes and every submesh's
/// [`Submesh::node`] is `None`.
///
/// ```no_run
/// let install = sc_discovery::discover_primary()?;
/// let assets = sc_extract::AssetSource::from_install(&install)?;
/// let g = sc_extract::MeshGeometry::read(
///     &assets,
///     "objects/spaceships/power_plants/small_component/aegs/powr_aegs_s01_pl03.cga",
/// )?;
/// println!("{} vertices, material {:?}", g.positions.len(), g.material);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MeshGeometry {
    pub nodes: Vec<MeshNode>,
    /// The `.mtl` the submeshes index into, as an archive path without its
    /// extension (`MtlName` chunk); `None` when the file names none.
    pub material: Option<String>,
    pub positions: Vec<[f32; 3]>,
    /// Unit normals, one per vertex; empty when the file has none.
    pub normals: Vec<[f32; 3]>,
    /// Unit tangent plus bitangent sign in `w`; empty when the file has none.
    pub tangents: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    /// RGBA vertex colours; empty when the file has none.
    pub colors: Vec<[u8; 4]>,
    /// Triangle list, absolute vertex indices.
    pub indices: Vec<u32>,
    pub submeshes: Vec<Submesh>,
}

/// A run of triangles drawn with one material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Submesh {
    /// Index into the `.mtl` file's submaterials.
    pub material: u16,
    /// Index into [`MeshGeometry::nodes`].
    pub node: Option<usize>,
    /// First entry in [`MeshGeometry::indices`].
    pub first_index: u32,
    pub index_count: u32,
}

impl Mesh {
    /// Read `path` and its companion stream file from the archive.
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        let geometry = read_data(assets, path)?;
        let streams = read_data(assets, &format!("{path}m")).ok();
        Self::parse(&geometry, streams.as_deref())
    }

    /// Parse a geometry file and, if it has one, its companion streams file.
    /// Without streams every volume is 0.
    pub fn parse(geometry: &[u8], streams: Option<&[u8]>) -> Result<Self> {
        let mut nodes = parse_nodes(geometry)?;
        let volumes = match streams {
            Some(s) => node_volumes(s)?,
            None => node_volumes(geometry).unwrap_or_default(),
        };
        for (i, v) in volumes {
            if let Some(node) = nodes.get_mut(i) {
                node.volume += v as f32;
            }
        }
        Ok(Self { nodes })
    }

    /// The nodes whose parent is `index`.
    pub fn children(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        self.nodes
            .iter()
            .enumerate()
            .filter(move |(_, n)| n.parent == Some(index))
            .map(|(i, _)| i)
    }
}

impl MeshGeometry {
    /// Read `path` and its companion stream file (`x.cga` → `x.cgam`,
    /// `x.skin` → `x.skinm`) from the archive.
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        let geometry = read_data(assets, path)?;
        let streams = read_data(assets, &format!("{path}m")).ok();
        Self::parse(&geometry, streams.as_deref())
    }

    /// Parse a geometry file and, if it has one, its companion streams file.
    pub fn parse(geometry: &[u8], streams: Option<&[u8]>) -> Result<Self> {
        let nodes = match parse_nodes(geometry) {
            Ok(nodes) => nodes,
            Err(_) if chunk(geometry, NODE_MESH_COMBO).is_err() => Vec::new(),
            Err(e) => return Err(e),
        };
        let material = chunk(geometry, MTL_NAME).ok().and_then(|d| {
            let name = d.get(..128.min(d.len()))?.split(|&b| b == 0).next()?;
            let name = String::from_utf8_lossy(name).trim().to_string();
            (!name.is_empty()).then_some(name)
        });
        let mut g = parse_render_streams(streams.unwrap_or(geometry))?;
        g.nodes = nodes;
        g.material = material;
        for s in &mut g.submeshes {
            s.node = s.node.filter(|&n| n < g.nodes.len());
        }
        Ok(g)
    }
}

/// Read an archive file by its DataCore-style path (`objects/…`), with or
/// without a `Data/` prefix.
pub(crate) fn read_data(assets: &AssetSource, path: &str) -> Result<Vec<u8>> {
    let path = path.trim_start_matches(['/', '\\']);
    let bare = ["Data/", "data/", "Data\\", "data\\"]
        .iter()
        .find_map(|p| path.strip_prefix(p))
        .unwrap_or(path);
    if let Some(b) = assets.try_read(bare)? {
        return Ok(b);
    }
    assets.read(&format!("Data\\{bare}"))
}

fn bad(what: &str) -> Error {
    Error::Mesh(what.to_string())
}

struct Reader<'a> {
    d: &'a [u8],
    p: usize,
}

impl<'a> Reader<'a> {
    fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        let b = self
            .d
            .get(self.p..self.p.checked_add(n).ok_or_else(|| bad("overflow"))?)
            .ok_or_else(|| bad("truncated"))?;
        self.p += n;
        Ok(b)
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.bytes(4)?.try_into().expect("4 bytes"),
        ))
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(
            self.bytes(2)?.try_into().expect("2 bytes"),
        ))
    }
    fn skip(&mut self, n: usize) -> Result<()> {
        self.bytes(n).map(|_| ())
    }
}

/// `(type, offset)` of every chunk in an IVO file.
fn chunks(file: &[u8]) -> Result<Vec<(u32, usize)>> {
    if file.get(..4) != Some(IVO_MAGIC.as_slice()) {
        return Err(bad("not an IVO file"));
    }
    let mut r = Reader { d: file, p: 8 };
    let count = r.u32()? as usize;
    let table = r.u32()? as usize;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let mut e = Reader {
            d: file,
            p: table + i * 16,
        };
        let ty = e.u32()?;
        e.skip(4)?;
        let off = u64::from_le_bytes(e.bytes(8)?.try_into().expect("8 bytes")) as usize;
        out.push((ty, off));
    }
    Ok(out)
}

/// The chunk of `ty`, up to the next chunk.
fn chunk(file: &[u8], ty: u32) -> Result<&[u8]> {
    let all = chunks(file)?;
    let (_, off) = *all
        .iter()
        .find(|(t, _)| *t == ty)
        .ok_or_else(|| bad("chunk missing"))?;
    let end = all
        .iter()
        .map(|&(_, o)| o)
        .filter(|&o| o > off)
        .min()
        .unwrap_or(file.len());
    file.get(off..end).ok_or_else(|| bad("chunk out of range"))
}

/// Names, parents and properties from the `NodeMeshCombo` chunk.
fn parse_nodes(file: &[u8]) -> Result<Vec<MeshNode>> {
    let d = chunk(file, NODE_MESH_COMBO)?;
    let mut r = Reader { d, p: 0 };
    let _ = r.u32()?;
    let n = r.u32()? as usize;
    let _ = r.u32()?;
    let n_unknown = r.u32()? as usize;
    let n_material = r.u32()? as usize;
    let names_len = r.u32()? as usize;
    r.skip(8)?;
    let mut parents = Vec::with_capacity(n);
    let mut transforms = Vec::with_capacity(n);
    for _ in 0..n {
        // metadata (32), world-to-bone (48), bone-to-world (48), scale + id
        // (20), parent, geometry type, bbox (24), tail (32)
        r.skip(32 + 48)?;
        let mut m = [[0.0f32; 4]; 3];
        for row in &mut m {
            for v in row.iter_mut() {
                *v = f32::from_bits(r.u32()?);
            }
        }
        transforms.push(m);
        r.skip(20)?;
        let parent = r.u16()?;
        r.skip(2 + 24 + 32)?;
        parents.push((parent != 0xFFFF).then_some(parent as usize));
    }
    r.skip(32 + 2 * n_unknown + 2 * n_material)?;
    let names = strings(r.bytes(names_len)?, n);
    let properties = strings(&d[r.p..], n);
    Ok((0..n)
        .map(|i| MeshNode {
            name: names[i].lines().next().unwrap_or("").to_string(),
            parent: parents[i].filter(|&p| p < n && p != i),
            properties: properties[i].replace('\r', ""),
            volume: 0.0,
            transform: transforms[i],
        })
        .collect())
}

/// `n` NUL-terminated strings (missing ones empty).
fn strings(b: &[u8], n: usize) -> Vec<String> {
    let mut out: Vec<String> = b
        .split(|&c| c == 0)
        .take(n)
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .collect();
    out.resize(n, String::new());
    out
}

/// Volume enclosed by each node's submeshes in an `IvoSkin2` chunk.
fn node_volumes(file: &[u8]) -> Result<Vec<(usize, f64)>> {
    let d = chunk(file, IVO_SKIN2)?;
    let g32 = |p: usize| -> Result<u32> { Reader { d, p }.u32() };
    let gf = |p: usize| -> Result<f32> { Ok(f32::from_bits(g32(p)?)) };
    let vertices = g32(8)? as usize;
    let index_count = g32(12)? as usize;
    let submesh_count = g32(16)? as usize;
    let mut lo = [0.0f32; 3];
    let mut hi = [0.0f32; 3];
    for a in 0..3 {
        lo[a] = gf(48 + a * 4)?;
        hi[a] = gf(60 + a * 4)?;
    }
    let extra = g32(76)? as usize;

    let mut submeshes = Vec::with_capacity(submesh_count);
    for i in 0..submesh_count {
        let b = 168 + i * 48;
        submeshes.push((
            (g32(b)? >> 16) as usize,
            g32(b + 4)? as usize,
            g32(b + 8)? as usize,
            g32(b + 16)?,
        ));
    }

    let mut p = 168 + submesh_count * 48 + extra * 4;
    let (mut positions, mut indices) = (None, None);
    while p + 8 <= d.len() && (positions.is_none() || indices.is_none()) {
        let tag = g32(p)?;
        if tag == 0 {
            p += 4;
            continue;
        }
        let elem = g32(p + 4)? as usize;
        let start = p + 8;
        let count = if tag == INDICES {
            index_count
        } else {
            vertices
        };
        let size = elem * count;
        let known = matches!(tag, POSITIONS | POSITIONS2 | INDICES);
        let Some(s) = d.get(start..start + size) else {
            if known {
                return Err(bad("stream past chunk end"));
            }
            break;
        };
        match (tag, elem) {
            (POSITIONS | POSITIONS2, 16) => {
                positions = Some(
                    s.as_chunks::<16>()
                        .0
                        .iter()
                        .map(|v| {
                            std::array::from_fn(|a| {
                                let q =
                                    i16::from_le_bytes([v[a * 2], v[a * 2 + 1]]) as f32 / 32767.0;
                                q * ((hi[a] - lo[a]) / 2.0).max(1.0) + (hi[a] + lo[a]) / 2.0
                            })
                        })
                        .collect::<Vec<[f32; 3]>>(),
                )
            }
            (POSITIONS | POSITIONS2, 20) => {
                positions = Some(
                    s.as_chunks::<20>()
                        .0
                        .iter()
                        .map(|v| {
                            std::array::from_fn(|a| {
                                f32::from_le_bytes(v[a * 4..a * 4 + 4].try_into().expect("4 bytes"))
                            })
                        })
                        .collect::<Vec<[f32; 3]>>(),
                )
            }
            (POSITIONS | POSITIONS2, _) => return Err(bad("unknown position format")),
            (INDICES, 2) => {
                let mut v: Vec<u32> = s
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|&b| u16::from_le_bytes(b) as u32)
                    .collect();
                for &(_, first, n, base) in &submeshes {
                    for x in v.iter_mut().skip(first).take(n) {
                        *x += base;
                    }
                }
                indices = Some(v);
            }
            (INDICES, _) => {
                indices = Some(
                    s.as_chunks::<4>()
                        .0
                        .iter()
                        .map(|&b| u32::from_le_bytes(b))
                        .collect(),
                )
            }
            _ => {}
        }
        p = start + size.next_multiple_of(8);
    }
    let positions = positions.ok_or_else(|| bad("no positions"))?;
    let indices = indices.ok_or_else(|| bad("no indices"))?;

    Ok(submeshes
        .iter()
        .map(|&(node, first, n, _)| {
            let tris = indices.get(first..first + n).unwrap_or(&[]);
            (node, enclosed_volume(&positions, tris))
        })
        .collect())
}

/// Vertex and index streams of an `IvoSkin2` chunk; `nodes` and `material`
/// are left empty for the caller.
fn parse_render_streams(file: &[u8]) -> Result<MeshGeometry> {
    let d = chunk(file, IVO_SKIN2)?;
    let g32 = |p: usize| -> Result<u32> { Reader { d, p }.u32() };
    let gf = |p: usize| -> Result<f32> { Ok(f32::from_bits(g32(p)?)) };
    let vertices = g32(8)? as usize;
    let index_count = g32(12)? as usize;
    let submesh_count = g32(16)? as usize;
    let mut lo = [0.0f32; 3];
    let mut hi = [0.0f32; 3];
    for a in 0..3 {
        lo[a] = gf(48 + a * 4)?;
        hi[a] = gf(60 + a * 4)?;
    }
    let extra = g32(76)? as usize;

    let mut submeshes = Vec::with_capacity(submesh_count);
    let mut page_bases = Vec::with_capacity(submesh_count);
    for i in 0..submesh_count {
        let b = 168 + i * 48;
        let w = g32(b)?;
        submeshes.push(Submesh {
            material: (w & 0xFFFF) as u16,
            node: Some((w >> 16) as usize),
            first_index: g32(b + 4)?,
            index_count: g32(b + 8)?,
        });
        page_bases.push(g32(b + 16)?);
    }

    let mut g = MeshGeometry {
        nodes: Vec::new(),
        material: None,
        positions: Vec::new(),
        normals: Vec::new(),
        tangents: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        submeshes,
    };
    let mut packed_normals: Option<Vec<[f32; 3]>> = None;
    let mut frames: Option<Vec<([f32; 3], [f32; 4])>> = None;

    let mut p = 168 + submesh_count * 48 + extra * 4;
    while p + 8 <= d.len() {
        let tag = g32(p)?;
        if tag == 0 {
            p += 4;
            continue;
        }
        let elem = g32(p + 4)? as usize;
        let start = p + 8;
        let count = if tag == INDICES {
            index_count
        } else {
            vertices
        };
        let size = elem * count;
        let Some(s) = d.get(start..start + size) else {
            if matches!(tag, POSITIONS | POSITIONS2 | INDICES) {
                return Err(bad("stream past chunk end"));
            }
            break;
        };
        match (tag, elem) {
            (POSITIONS | POSITIONS2, 16) => {
                for v in s.as_chunks::<16>().0 {
                    g.positions.push(std::array::from_fn(|a| {
                        let q = i16::from_le_bytes([v[a * 2], v[a * 2 + 1]]) as f32 / 32767.0;
                        q * ((hi[a] - lo[a]) / 2.0).max(1.0) + (hi[a] + lo[a]) / 2.0
                    }));
                    g.colors.push([v[10], v[9], v[8], v[11]]);
                    g.uvs.push(half2(&v[12..16]));
                }
            }
            (POSITIONS | POSITIONS2, 20) => {
                for v in s.as_chunks::<20>().0 {
                    g.positions.push(std::array::from_fn(|a| {
                        f32::from_le_bytes(v[a * 4..a * 4 + 4].try_into().expect("4 bytes"))
                    }));
                    g.colors.push([v[14], v[13], v[12], v[15]]);
                    g.uvs.push(half2(&v[16..20]));
                }
            }
            (POSITIONS | POSITIONS2, _) => return Err(bad("unknown position format")),
            (INDICES, 2) => {
                g.indices = s
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|&b| u16::from_le_bytes(b) as u32)
                    .collect();
                for (sm, &base) in g.submeshes.iter().zip(&page_bases) {
                    let first = sm.first_index as usize;
                    for x in g
                        .indices
                        .iter_mut()
                        .skip(first)
                        .take(sm.index_count as usize)
                    {
                        *x += base;
                    }
                }
            }
            (INDICES, _) => {
                g.indices = s
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|&b| u32::from_le_bytes(b))
                    .collect()
            }
            (NORMALS_FLOAT, 12) => {
                g.normals = s
                    .as_chunks::<12>()
                    .0
                    .iter()
                    .map(|v| {
                        normalize(std::array::from_fn(|a| {
                            f32::from_le_bytes(v[a * 4..a * 4 + 4].try_into().expect("4 bytes"))
                        }))
                    })
                    .collect()
            }
            (NORMALS_PACKED, 4) => {
                packed_normals = Some(
                    s.as_chunks::<4>()
                        .0
                        .iter()
                        .map(|&b| packed_unit_vector(u32::from_le_bytes(b)))
                        .collect(),
                )
            }
            (TANGENTS_QUAT, 8) => {
                frames = Some(
                    s.as_chunks::<8>()
                        .0
                        .iter()
                        .map(|v| {
                            qtangent(std::array::from_fn(|a| {
                                i16::from_le_bytes([v[a * 2], v[a * 2 + 1]]) as f32 / 32767.0
                            }))
                        })
                        .collect(),
                )
            }
            (TANGENTS_PACKED, 8) => {
                frames = Some(
                    s.as_chunks::<8>()
                        .0
                        .iter()
                        .map(|v| {
                            let t = u32::from_le_bytes(v[0..4].try_into().expect("4 bytes"));
                            let b = u32::from_le_bytes(v[4..8].try_into().expect("4 bytes"));
                            packed_frame(t, b)
                        })
                        .collect(),
                )
            }
            _ => {}
        }
        p = start + size.next_multiple_of(8);
    }
    if g.positions.is_empty() {
        return Err(bad("no positions"));
    }
    if g.indices.is_empty() {
        return Err(bad("no indices"));
    }
    if g.normals.is_empty() {
        if let Some(n) = packed_normals {
            g.normals = n;
        } else if let Some(f) = &frames {
            g.normals = f.iter().map(|(n, _)| *n).collect();
        }
    }
    if let Some(f) = frames {
        g.tangents = f.into_iter().map(|(_, t)| t).collect();
    }
    Ok(g)
}

/// Two little-endian IEEE half floats.
fn half2(b: &[u8]) -> [f32; 2] {
    [
        half(u16::from_le_bytes([b[0], b[1]])),
        half(u16::from_le_bytes([b[2], b[3]])),
    ]
}

fn half(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = (h >> 10) & 0x1F;
    let frac = (h & 0x3FF) as f32;
    sign * match exp {
        0 => frac * 2f32.powi(-24),
        31 if frac == 0.0 => f32::INFINITY,
        31 => f32::NAN,
        _ => (1.0 + frac / 1024.0) * 2f32.powi(exp as i32 - 15),
    }
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 1e-8 {
        v.map(|c| c / l)
    } else {
        [0.0, 0.0, 1.0]
    }
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// 15-15-1-1 packed unit vector: x, y biased by 0x3FFF, bit 30 = z negative.
fn packed_unit_vector(v: u32) -> [f32; 3] {
    let x = ((v & 0x7FFF) as i32 - 0x3FFF) as f32 / 16383.0;
    let y = (((v >> 15) & 0x7FFF) as i32 - 0x3FFF) as f32 / 16383.0;
    let z = (1.0 - x * x - y * y).max(0.0).sqrt();
    normalize([x, y, if (v >> 30) & 1 != 0 { -z } else { z }])
}

/// Normal and tangent from packed tangent + bitangent; bit 31 of the
/// tangent word is the bitangent sign.
fn packed_frame(t: u32, b: u32) -> ([f32; 3], [f32; 4]) {
    let (tv, bv) = (packed_unit_vector(t), packed_unit_vector(b));
    let sign = if t & 0x8000_0000 != 0 { -1.0 } else { 1.0 };
    let n = cross(tv, bv).map(|c| c * sign);
    (normalize(n), [tv[0], tv[1], tv[2], sign])
}

/// Normal and tangent from an SNorm quaternion (CryEngine `SPipQTangents`):
/// normal = column 2, negated when `w < 0`; tangent = column 0.
fn qtangent([x, y, z, w]: [f32; 4]) -> ([f32; 3], [f32; 4]) {
    let flip = if w < 0.0 { -1.0 } else { 1.0 };
    let n = [
        2.0 * (x * z + w * y),
        2.0 * (y * z - w * x),
        1.0 - 2.0 * (x * x + y * y),
    ]
    .map(|c| c * flip);
    let t = normalize([
        1.0 - 2.0 * (y * y + z * z),
        2.0 * (x * y + w * z),
        2.0 * (x * z - w * y),
    ]);
    (normalize(n), [t[0], t[1], t[2], flip])
}

/// Volume enclosed by triangles (divergence theorem); triangles with an
/// out-of-range index are skipped.
fn enclosed_volume(positions: &[[f32; 3]], indices: &[u32]) -> f64 {
    let at = |i: u32| positions.get(i as usize).map(|p| p.map(f64::from));
    indices
        .as_chunks::<3>()
        .0
        .iter()
        .filter_map(|t| Some((at(t[0])?, at(t[1])?, at(t[2])?)))
        .map(|(a, b, c)| {
            a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
                + a[2] * (b[0] * c[1] - b[1] * c[0])
        })
        .sum::<f64>()
        / 6.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_cube_volume() {
        let p = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ];
        // outward-facing triangles
        let t = [
            0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7,
            6, 3, 0, 4, 3, 4, 7,
        ];
        assert!((enclosed_volume(&p, &t) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_non_ivo() {
        assert!(Mesh::parse(b"CrCh....", None).is_err());
    }
}
