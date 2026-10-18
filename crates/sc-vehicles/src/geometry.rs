//! A loadout in 3D: the hull mesh and its collision shell by damage part,
//! and where each mounted item sits, in the ship's frame (x right, y
//! forward, z up, metres).

use std::collections::{BTreeMap, HashMap};

use sc_extract::{AssetSource, Datacore, Guid, MeshGeometry, PhysicsProxies, RecordCollection};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::{Mounted, Vehicle, Vehicles};

/// A 3 × 4 row-major transform: rotation / scale in the first three
/// columns, translation in the fourth.
pub type Transform = [[f32; 4]; 3];

const IDENTITY: Transform = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
];

/// Every entity's root geometry path
/// (`SGeometryResourceParams.Geometry.Geometry.Geometry.path`), keyed by
/// `EntityClassDefinition` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeometryPaths {
    by_guid: HashMap<Guid, String>,
}

impl GeometryPaths {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let by_guid = store
            .records
            .multi_feature
            .entity_class_definition
            .iter()
            .filter_map(|(&guid, h)| {
                let ecd = h.get(&store.pools)?;
                Some((guid, crate::vehicle::geometry(ecd, &store.pools)?))
            })
            .collect();
        Self { by_guid }
    }
}

impl RecordCollection for GeometryPaths {
    type Item = String;

    fn get(&self, guid: &Guid) -> Option<&String> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &String)> + '_ {
        self.by_guid.iter()
    }
}

/// The hull and the mounted items of one loadout in the ship frame.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HullGeometry {
    /// The hull's render mesh grouped by damage part: what to draw. It is
    /// not closed everywhere (the Constellation Mk V's belly under its
    /// power plants is open).
    pub parts: Vec<PartMesh>,
    /// The hull's ray-colliding physics proxies grouped by damage part: what
    /// shots hit. Closed shells, a little proud of the render skin (0.18 m
    /// on the Centaurus body).
    pub collision: Vec<PartMesh>,
    /// Each hull mesh node (by name, as the geometry file spells it) under
    /// a damage part, to that part's name: how a renderer drawing the hull
    /// file itself colours it by part.
    pub node_parts: BTreeMap<String, String>,
    /// Where each mounted item sits, outermost first (docked vehicles
    /// left out).
    pub placements: Vec<Placement>,
}

/// The triangles of one hull damage part (a vehicle-XML `Part` with
/// `damageMax`), ship frame: render mesh or collision shell.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PartMesh {
    /// The vehicle-XML part (and mesh node) name; empty for geometry under
    /// no damage part.
    pub part: String,
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

/// A mounted item in the ship frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Placement {
    /// Port path from the hull (`turret/gimbal/gun`).
    pub path: String,
    pub item: Guid,
    /// The item's frame in the ship frame: the port helper it sits on.
    pub transform: Transform,
    /// The item mesh's bounds in its own frame; `None` without geometry.
    pub bounds: Option<Aabb>,
}

/// An axis-aligned box.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    fn grow(&mut self, p: [f32; 3]) {
        self.min = std::array::from_fn(|i| self.min[i].min(p[i]));
        self.max = std::array::from_fn(|i| self.max[i].max(p[i]));
    }

    /// The eight corners.
    pub fn corners(&self) -> [[f32; 3]; 8] {
        let (a, b) = (self.min, self.max);
        std::array::from_fn(|i| {
            [
                if i & 1 == 0 { a[0] } else { b[0] },
                if i & 2 == 0 { a[1] } else { b[1] },
                if i & 4 == 0 { a[2] } else { b[2] },
            ]
        })
    }
}

impl HullGeometry {
    /// Read `vehicle`'s hull geometry and place `loadout`'s items.
    ///
    /// Each hull node's triangles, render and physics proxy alike, go to
    /// the nearest node above it (itself included) named after a damage
    /// part: the engine names a damage part's mesh node after the
    /// vehicle-XML `Part` (`node=Part` in the node properties). The proxies
    /// kept are those ray queries test ([`sc_extract::ProxyKind::hits_rays`]). An item sits on the helper node named after its
    /// port, in the hull or in the item above it: the engine's own attach
    /// rule, joined by name within one mesh.
    pub fn read(
        assets: &AssetSource,
        vehicle: &Vehicle,
        loadout: &[Mounted],
        vehicles: &Vehicles,
        paths: &GeometryPaths,
    ) -> Option<Self> {
        let path = vehicle.geometry.as_deref()?;
        let hull = match MeshGeometry::read(assets, path) {
            Ok(g) => g,
            Err(error) => {
                warn!(path, %error, "hull geometry unreadable");
                return None;
            }
        };
        let world = world_transforms(&hull);
        let damage_parts: std::collections::HashSet<String> = vehicle
            .hull
            .iter()
            .filter(|p| p.damage_max > 0.0)
            .map(|p| p.name.to_ascii_lowercase())
            .collect();
        // Each node's damage part: itself or its nearest such ancestor.
        let owner: Vec<Option<usize>> = (0..hull.nodes.len())
            .map(|mut i| {
                loop {
                    if damage_parts.contains(&hull.nodes[i].name.to_ascii_lowercase()) {
                        return Some(i);
                    }
                    i = hull.nodes[i].parent?;
                }
            })
            .collect();
        let mut parts: HashMap<Option<usize>, PartMesh> = HashMap::new();
        for s in &hull.submeshes {
            let Some(node) = s.node else {
                continue;
            };
            let key = owner[node];
            let mesh = parts.entry(key).or_insert_with(|| PartMesh {
                part: key.map(|k| hull.nodes[k].name.clone()).unwrap_or_default(),
                ..PartMesh::default()
            });
            let m = &world[node];
            let range = s.first_index as usize..(s.first_index + s.index_count) as usize;
            let mut remap: HashMap<u32, u32> = HashMap::new();
            for &i in &hull.indices[range] {
                let next = mesh.positions.len() as u32;
                let j = *remap.entry(i).or_insert_with(|| {
                    mesh.positions.push(apply(m, hull.positions[i as usize]));
                    next
                });
                mesh.indices.push(j);
            }
        }
        let mut parts: Vec<PartMesh> = parts.into_values().collect();
        parts.sort_by(|a, b| a.part.cmp(&b.part));

        let proxies = PhysicsProxies::read(assets, path).unwrap_or_else(|error| {
            warn!(path, %error, "hull physics proxies unreadable");
            PhysicsProxies::default()
        });
        let mut collision: HashMap<Option<usize>, PartMesh> = HashMap::new();
        for p in proxies.proxies.iter().filter(|p| p.kind.hits_rays()) {
            let key = p.node.and_then(|n| owner[n]);
            let mesh = collision.entry(key).or_insert_with(|| PartMesh {
                part: key.map(|k| hull.nodes[k].name.clone()).unwrap_or_default(),
                ..PartMesh::default()
            });
            let m = p.node.map_or(&IDENTITY, |n| &world[n]);
            let (positions, indices) = p.shape.triangles();
            let base = mesh.positions.len() as u32;
            mesh.positions
                .extend(positions.into_iter().map(|v| apply(m, v)));
            mesh.indices.extend(indices.into_iter().map(|i| base + i));
        }
        let mut collision: Vec<PartMesh> = collision.into_values().collect();
        collision.sort_by(|a, b| a.part.cmp(&b.part));
        if collision.is_empty() {
            warn!(
                path,
                "no physics proxies; shots fall back to the render mesh"
            );
            collision = parts.clone();
        }

        let node_parts = owner
            .iter()
            .enumerate()
            .filter_map(|(i, o)| Some((hull.nodes[i].name.clone(), hull.nodes[(*o)?].name.clone())))
            .collect();

        let mut placements = Vec::new();
        let mut meshes: HashMap<String, Option<ItemMesh>> = HashMap::new();
        let helpers = helper_map(&hull, &world);
        place(
            loadout,
            "",
            &IDENTITY,
            &helpers,
            vehicles,
            paths,
            assets,
            &mut meshes,
            &mut placements,
        );
        debug!(
            parts = parts.len(),
            collision = collision.len(),
            placements = placements.len(),
            "hull geometry"
        );
        Some(Self {
            parts,
            collision,
            node_parts,
            placements,
        })
    }
}

/// An item mesh's bounds and helper nodes, in its own frame.
struct ItemMesh {
    bounds: Option<Aabb>,
    helpers: HashMap<String, Transform>,
}

#[allow(clippy::too_many_arguments)]
fn place(
    level: &[Mounted],
    prefix: &str,
    parent: &Transform,
    helpers: &HashMap<String, Transform>,
    vehicles: &Vehicles,
    paths: &GeometryPaths,
    assets: &AssetSource,
    meshes: &mut HashMap<String, Option<ItemMesh>>,
    out: &mut Vec<Placement>,
) {
    for m in level {
        let Some(item) = m.item else {
            continue;
        };
        if vehicles.get(&item).is_some() {
            continue;
        }
        let path = format!("{prefix}{}", m.port);
        let local = helpers.get(&m.port.to_ascii_lowercase());
        if local.is_none() {
            debug!(port = %m.port, "no helper node; placed at its holder");
        }
        let transform = mul(parent, local.unwrap_or(&IDENTITY));
        let mesh = paths.get(&item).and_then(|p| {
            meshes
                .entry(p.to_ascii_lowercase())
                .or_insert_with(|| read_item(assets, p))
                .as_ref()
        });
        let bounds = mesh.and_then(|m| m.bounds);
        let child_helpers = mesh.map(|m| m.helpers.clone()).unwrap_or_default();
        out.push(Placement {
            path: path.clone(),
            item,
            transform,
            bounds,
        });
        place(
            &m.children,
            &format!("{path}/"),
            &transform,
            &child_helpers,
            vehicles,
            paths,
            assets,
            meshes,
            out,
        );
    }
}

fn read_item(assets: &AssetSource, path: &str) -> Option<ItemMesh> {
    let g = MeshGeometry::read(assets, path).ok()?;
    let world = world_transforms(&g);
    let mut bounds: Option<Aabb> = None;
    for s in &g.submeshes {
        let m = s.node.map_or(&IDENTITY, |n| &world[n]);
        let range = s.first_index as usize..(s.first_index + s.index_count) as usize;
        for &i in &g.indices[range] {
            let p = apply(m, g.positions[i as usize]);
            match &mut bounds {
                Some(b) => b.grow(p),
                None => bounds = Some(Aabb { min: p, max: p }),
            }
        }
    }
    Some(ItemMesh {
        bounds,
        helpers: helper_map(&g, &world),
    })
}

/// Every node's transform relative to the file's root.
fn world_transforms(g: &MeshGeometry) -> Vec<Transform> {
    let mut out: Vec<Option<Transform>> = vec![None; g.nodes.len()];
    for i in 0..g.nodes.len() {
        let mut chain = vec![i];
        let mut n = i;
        while let Some(p) = g.nodes[n].parent {
            if out[p].is_some() {
                break;
            }
            chain.push(p);
            n = p;
        }
        for &c in chain.iter().rev() {
            let local = g.nodes[c].transform;
            out[c] = Some(match g.nodes[c].parent {
                Some(p) => mul(out[p].as_ref().unwrap_or(&IDENTITY), &local),
                None => local,
            });
        }
    }
    out.into_iter().map(|t| t.unwrap_or(IDENTITY)).collect()
}

/// Lower-case node name to its transform, for the nodes ports attach to.
fn helper_map(g: &MeshGeometry, world: &[Transform]) -> HashMap<String, Transform> {
    g.nodes
        .iter()
        .zip(world)
        .map(|(n, w)| (n.name.to_ascii_lowercase(), *w))
        .collect()
}

/// `a × b`.
pub fn mul(a: &Transform, b: &Transform) -> Transform {
    let mut o = [[0.0; 4]; 3];
    for r in 0..3 {
        for c in 0..3 {
            o[r][c] = a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c];
        }
        o[r][3] = a[r][0] * b[0][3] + a[r][1] * b[1][3] + a[r][2] * b[2][3] + a[r][3];
    }
    o
}

/// `m` applied to point `p`.
pub fn apply(m: &Transform, p: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|r| m[r][0] * p[0] + m[r][1] * p[1] + m[r][2] * p[2] + m[r][3])
}
