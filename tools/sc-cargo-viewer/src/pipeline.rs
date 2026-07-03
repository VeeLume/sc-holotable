//! Data pipeline: ship → cargo grids + interior wall boxes + per-face-cell
//! distance-to-wall. Silent port of the validated
//! `crates/sc-extract/examples/cargo_grid_walls.rs` (see
//! `docs/ship-cargo-grids.md` §Tier C for the format discoveries this encodes:
//! NMC hierarchy composition, bottom-anchored grid boxes, `IncludedObjects` +
//! entity placements, fixed-offset model AABBs).

use std::collections::{BTreeMap, HashMap, HashSet};

use sc_extract::object_container::{Socpak, XmlNode, decode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

pub const CELL_M: f32 = 1.25;
pub const MAX_PROBE_M: f32 = 7.5;

// ── math (3×4 row-major) ─────────────────────────────────────────────────────

pub type Mat34 = [[f32; 4]; 3];
pub type Aabb = ([f32; 3], [f32; 3]);

const IDENTITY: Mat34 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
];

fn mat_mul(a: &Mat34, b: &Mat34) -> Mat34 {
    let mut o = [[0.0f32; 4]; 3];
    for r in 0..3 {
        for c in 0..3 {
            o[r][c] = a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c];
        }
        o[r][3] = a[r][0] * b[0][3] + a[r][1] * b[1][3] + a[r][2] * b[2][3] + a[r][3];
    }
    o
}

fn mat_apply(m: &Mat34, p: [f32; 3]) -> [f32; 3] {
    let mut o = [0.0f32; 3];
    for r in 0..3 {
        o[r] = m[r][0] * p[0] + m[r][1] * p[1] + m[r][2] * p[2] + m[r][3];
    }
    o
}

fn mat_translation(m: &Mat34) -> [f32; 3] {
    [m[0][3], m[1][3], m[2][3]]
}

fn mat_from_pos_quat_scale(pos: [f32; 3], q: [f32; 4], s: [f32; 3]) -> Mat34 {
    let (w, x, y, z) = (q[0], q[1], q[2], q[3]);
    let r = [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
        ],
        [
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
        ],
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ];
    [
        [r[0][0] * s[0], r[0][1] * s[1], r[0][2] * s[2], pos[0]],
        [r[1][0] * s[0], r[1][1] * s[1], r[1][2] * s[2], pos[1]],
        [r[2][0] * s[0], r[2][1] * s[1], r[2][2] * s[2], pos[2]],
    ]
}

fn mat_from_pos_euler_deg(pos: [f32; 3], rot_deg: [f32; 3]) -> Mat34 {
    let (sx, cx) = rot_deg[0].to_radians().sin_cos();
    let (sy, cy) = rot_deg[1].to_radians().sin_cos();
    let (sz, cz) = rot_deg[2].to_radians().sin_cos();
    [
        [
            cy * cz,
            sx * sy * cz - cx * sz,
            cx * sy * cz + sx * sz,
            pos[0],
        ],
        [
            cy * sz,
            sx * sy * sz + cx * cz,
            cx * sy * sz - sx * cz,
            pos[1],
        ],
        [-sy, sx * cy, cx * cy, pos[2]],
    ]
}

fn transform_aabb(m: &Mat34, mn: [f32; 3], mx: [f32; 3]) -> Aabb {
    let mut omn = [f32::INFINITY; 3];
    let mut omx = [f32::NEG_INFINITY; 3];
    for xi in [mn[0], mx[0]] {
        for yi in [mn[1], mx[1]] {
            for zi in [mn[2], mx[2]] {
                let p = mat_apply(m, [xi, yi, zi]);
                for a in 0..3 {
                    omn[a] = omn[a].min(p[a]);
                    omx[a] = omx[a].max(p[a]);
                }
            }
        }
    }
    (omn, omx)
}

// ── public scene model ───────────────────────────────────────────────────────

/// One face cell of a grid, with its measured distance to the nearest wall.
pub struct FaceCell {
    pub corners: [[f32; 3]; 4],
    pub dist: f32,
    /// Face axis (0=X, 1=Y, 2=Z) and outward direction (±1) — the outward
    /// normal, used for view-dependent culling in the renderer.
    pub axis: usize,
    pub dir: i32,
}

pub struct GridView {
    pub port: String,
    pub min: [f32; 3],
    pub max: [f32; 3],
    pub scu: i64,
    pub cells: Vec<FaceCell>,
}

pub struct WallView {
    pub min: [f32; 3],
    pub max: [f32; 3],
    #[allow(dead_code)]
    pub cgf: String,
}

pub struct ShipScene {
    #[allow(dead_code)]
    pub entity: String,
    pub grids: Vec<GridView>,
    pub walls: Vec<WallView>,
    pub total_scu: i64,
    pub info: String,
}

/// Owns the parsed game data; build scenes per ship.
pub struct Holo {
    assets: AssetSource,
    datacore: Datacore,
}

impl Holo {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let install = sc_discovery::discover_primary()?;
        eprintln!("install: {} v{}", install.channel, install.short_version());
        let assets = AssetSource::from_install(&install)?;
        eprintln!("parsing DCB (~30s release)…");
        let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
        let datacore = Datacore::parse(&assets, &asset_data)?;
        eprintln!("DCB parsed.");
        Ok(Self { assets, datacore })
    }

    /// Every entity class carrying a top-level `VehicleComponentParams` —
    /// the typed "is a vehicle" census (no name matching).
    pub fn list_ships(&self) -> Vec<String> {
        let db = self.datacore.db();
        let mut out = Vec::new();
        for rec in db.records_by_type("EntityClassDefinition") {
            let inst = rec.as_instance();
            let Some(comps) = inst.get_array("Components") else {
                continue;
            };
            let is_vehicle = comps.into_iter().any(|c| {
                value_to_instance(db, &c)
                    .and_then(|ci| ci.type_name().map(|t| t == "VehicleComponentParams"))
                    .unwrap_or(false)
            });
            if is_vehicle {
                let n = rec.name().unwrap_or("?");
                out.push(n.rsplit('.').next().unwrap_or(n).to_string());
            }
        }
        out.sort();
        out.dedup();
        out
    }

    /// Build the full scene (grids + walls + face-cell distances) for a ship.
    pub fn build_scene(&self, ship: &str) -> Result<ShipScene, String> {
        let db = self.datacore.db();
        let rec = shortest_entity(db, ship).ok_or_else(|| format!("no entity '{ship}'"))?;
        let inst = rec.as_instance();
        let entity = rec.name().unwrap_or("?").to_string();

        let hull = first_cga(db, &inst).ok_or("no hull .cga")?;
        let hull_bytes = read_p4k(&self.assets, &hull).map_err(|e| e.to_string())?;
        let nmc_nodes = nmc::parse(&hull_bytes).ok_or("NMC parse failed (CrCh-format hull?)")?;
        let worlds = nmc::world_transforms(&nmc_nodes);
        let node_world: BTreeMap<String, Mat34> = nmc_nodes
            .iter()
            .zip(worlds.iter())
            .map(|(n, w)| (n.name.to_lowercase(), *w))
            .collect();

        // grids — recursive over mounted items (cargo elevators/modules)
        let mut collector = GridCollector {
            holo: self,
            nmc_cache: HashMap::new(),
            loadout_cache: HashMap::new(),
            grids: Vec::new(),
            notes: Vec::new(),
        };
        collector
            .nmc_cache
            .insert(hull.to_lowercase(), Some(node_world.clone()));
        let mut visited = HashSet::new();
        collector.descend(
            &inst,
            &IDENTITY,
            std::slice::from_ref(&hull),
            "",
            0,
            &[],
            &mut visited,
        );
        let mut raw_grids = collector.grids;
        let notes = collector.notes;

        // walls + near-grid triangle soup
        let refs = object_container_refs(db, &inst);
        let mut walls: Vec<WallView> = Vec::new();
        let mut aabb_cache: HashMap<String, Vec<Aabb>> = HashMap::new();
        let mut soup: Vec<[[f32; 3]; 3]> = Vec::new();
        let mut tri_cache: HashMap<String, Option<std::rc::Rc<TriMesh>>> = HashMap::new();
        let grid_bounds: Vec<([f32; 3], [f32; 3])> =
            raw_grids.iter().map(|g| (g.min, g.max)).collect();
        let near_grid = move |mn: [f32; 3], mx: [f32; 3]| -> bool {
            grid_bounds.iter().any(|(gmin, gmax)| {
                (0..3).all(|a| mn[a] <= gmax[a] + MAX_PROBE_M && mx[a] >= gmin[a] - MAX_PROBE_M)
            })
        };
        for r in &refs {
            let helper = r
                .bone_name
                .as_ref()
                .and_then(|b| node_world.get(&b.to_lowercase()))
                .copied();
            let offset_m = mat_from_pos_euler_deg(r.offset_pos, r.offset_rot_deg);
            let section_world = compose_container(helper, &offset_m);
            let (placements, oc_grids) = read_section_placements(&self.assets, db, &r.file_name);
            for og in &oc_grids {
                let w = mat_mul(&section_world, &og.local);
                let lmin = [
                    og.off[0] - og.dims[0] / 2.0,
                    og.off[1] - og.dims[1] / 2.0,
                    og.off[2],
                ];
                let lmax = [
                    og.off[0] + og.dims[0] / 2.0,
                    og.off[1] + og.dims[1] / 2.0,
                    og.off[2] + og.dims[2],
                ];
                let (wmin, wmax) = transform_aabb(&w, lmin, lmax);
                raw_grids.push(RawGrid {
                    port: format!("oc:{}", og.label),
                    min: wmin,
                    max: wmax,
                });
            }
            for pl in placements {
                let boxes = aabb_cache
                    .entry(pl.cgf.to_lowercase())
                    .or_insert_with(|| cgf_sub_boxes(&self.assets, &pl.cgf))
                    .clone();
                let world_m = mat_mul(&section_world, &pl.local);
                let mut placement_near = false;
                for (mn, mx) in boxes {
                    let (wmn, wmx) = transform_aabb(&world_m, mn, mx);
                    placement_near |= near_grid(wmn, wmx);
                    walls.push(WallView {
                        min: wmn,
                        max: wmx,
                        cgf: pl.cgf.clone(),
                    });
                }
                if placement_near
                    && let Some(tm) = tri_cache
                        .entry(pl.cgf.to_lowercase())
                        .or_insert_with(|| {
                            cgf_triangles(&self.assets, &pl.cgf).map(std::rc::Rc::new)
                        })
                        .clone()
                {
                    for t in tm.indices.chunks_exact(3) {
                        soup.push([
                            mat_apply(&world_m, tm.positions[t[0] as usize]),
                            mat_apply(&world_m, tm.positions[t[1] as usize]),
                            mat_apply(&world_m, tm.positions[t[2] as usize]),
                        ]);
                    }
                }
            }
        }
        // hull skin (bay ceilings/outer boundary), near-grid triangles only
        if let Some(tm) = cgf_triangles(&self.assets, &hull) {
            for t in tm.indices.chunks_exact(3) {
                let a = tm.positions[t[0] as usize];
                let b = tm.positions[t[1] as usize];
                let c = tm.positions[t[2] as usize];
                let mn = [
                    a[0].min(b[0]).min(c[0]),
                    a[1].min(b[1]).min(c[1]),
                    a[2].min(b[2]).min(c[2]),
                ];
                let mx = [
                    a[0].max(b[0]).max(c[0]),
                    a[1].max(b[1]).max(c[1]),
                    a[2].max(b[2]).max(c[2]),
                ];
                if near_grid(mn, mx) {
                    soup.push([a, b, c]);
                }
            }
        }

        // face cells + SCU
        let mut grids = Vec::new();
        let mut total_scu = 0i64;
        for g in &raw_grids {
            let cells_dims = [
                (((g.max[0] - g.min[0]) / CELL_M).round() as i64).max(0),
                (((g.max[1] - g.min[1]) / CELL_M).round() as i64).max(0),
                (((g.max[2] - g.min[2]) / CELL_M).round() as i64).max(0),
            ];
            let scu = cells_dims[0] * cells_dims[1] * cells_dims[2];
            total_scu += scu;
            let mut cells = Vec::new();
            for (axis, dir) in [(0, -1), (0, 1), (1, -1), (1, 1), (2, -1), (2, 1)] {
                cells.extend(face_cells(g, axis, dir, &walls, &soup));
            }
            grids.push(GridView {
                port: g.port.clone(),
                min: g.min,
                max: g.max,
                scu,
                cells,
            });
        }

        let mut info = format!(
            "{} grids · {} SCU · {} wall boxes · {}k tris · {} sections",
            grids.len(),
            total_scu,
            walls.len(),
            soup.len() / 1000,
            refs.len()
        );
        if !notes.is_empty() {
            info.push_str(&format!(" · ⚠ {}", notes.join("; ")));
        }
        Ok(ShipScene {
            entity,
            grids,
            walls,
            total_scu,
            info,
        })
    }
}

// ── grid collection ──────────────────────────────────────────────────────────

struct RawGrid {
    port: String,
    min: [f32; 3],
    max: [f32; 3],
}

/// Grid collector. Grids reach a ship four ways (all handled):
///  1. inline manual loadout entries, 2. port default items,
///  3. mounted items' own loadouts (elevators/bay doors, via Reference),
///  4. XML-file loadouts (`SItemPortLoadoutXMLParams.loadoutPath`).
///
/// Port helpers resolve against the item's geometries, then ancestors', then
/// fall back to the item origin (engine default when no helper exists —
/// Hammerhead lift / Constellation bay).
/// A port's explicit DCB attachment (`SItemPortDef.AttachmentImplementation`):
/// anchor node name + offset — the authoritative port placement (the Idris'
/// 25 grid ports carry explicit offsets anchored on the hull root).
#[derive(Clone)]
struct PortAttach {
    helper: String,
    offset: Mat34,
}

struct GridCollector<'h> {
    holo: &'h Holo,
    nmc_cache: HashMap<String, Option<BTreeMap<String, Mat34>>>,
    loadout_cache: HashMap<String, Vec<(String, String)>>,
    grids: Vec<RawGrid>,
    notes: Vec<String>,
}

impl<'h> GridCollector<'h> {
    fn mounts<'a>(
        &mut self,
        inst: &Instance<'a>,
    ) -> (
        Vec<(String, svarog_datacore::Record<'a>)>,
        HashMap<String, PortAttach>,
    )
    where
        'h: 'a,
    {
        let db = self.holo.datacore.db();
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        let mut xml_paths: Vec<String> = Vec::new();
        let mut attach: HashMap<String, PortAttach> = HashMap::new();
        walk(db, inst, 22, &mut 600_000, &mut |ci| {
            match ci.type_name() {
                Some("SItemPortLoadoutXMLParams") => {
                    if let Some(p) = ci.get_str("loadoutPath").filter(|s| !s.is_empty()) {
                        xml_paths.push(p.to_string());
                    }
                    return;
                }
                Some("SItemPortDef") => {
                    if let Some(pname) = ci.get_str("Name").filter(|s| !s.is_empty())
                        && let Some(ai) = ci.get_instance("AttachmentImplementation")
                        && let Some(hn) = ai.get_instance("Helper")
                        && let Some(h) = hn.get_instance("Helper")
                    {
                        let helper = h.get_str("Name").unwrap_or("").to_string();
                        let off = h.get_instance("Offset");
                        let pos = off
                            .as_ref()
                            .and_then(|o| o.get_instance("Position"))
                            .map(vec3)
                            .unwrap_or([0.0; 3]);
                        let rot = off
                            .as_ref()
                            .and_then(|o| o.get_instance("Rotation"))
                            .map(|r| {
                                if r.get("x").is_some() {
                                    vec3(r)
                                } else {
                                    r.get_instance("Rotation").map(vec3).unwrap_or([0.0; 3])
                                }
                            })
                            .unwrap_or([0.0; 3]);
                        if !helper.is_empty() || pos != [0.0; 3] || rot != [0.0; 3] {
                            attach.insert(
                                pname.to_lowercase(),
                                PortAttach {
                                    helper,
                                    offset: mat_from_pos_euler_deg(pos, rot),
                                },
                            );
                        }
                    }
                    return;
                }
                _ => {}
            }
            let (port_f, cls_f, ref_f) = match ci.type_name() {
                Some("SItemPortLoadoutEntryParams") => {
                    ("itemPortName", "entityClassName", "entityClassReference")
                }
                Some("SItemPortDefaultItemDef") => ("itemPort", "", "entityClass"),
                _ => return,
            };
            let cls = if cls_f.is_empty() {
                ""
            } else {
                ci.get_str(cls_f).unwrap_or("")
            };
            let Some(rec) = ci
                .get(ref_f)
                .and_then(|va| va.as_record_ref())
                .and_then(|r| db.record(&r.guid))
                .or_else(|| {
                    if cls.is_empty() {
                        None
                    } else {
                        entity_by_short(db, cls)
                    }
                })
            else {
                return;
            };
            let port = ci.get_str(port_f).unwrap_or("").to_string();
            if port.is_empty() || !seen.insert(port.clone()) {
                return;
            }
            out.push((port, rec));
        });
        for path in xml_paths {
            for (port, item) in self.xml_loadout(&path) {
                if !seen.insert(port.clone()) {
                    continue;
                }
                if let Some(rec) = entity_by_short(db, &item) {
                    out.push((port, rec));
                }
            }
        }
        (out, attach)
    }

    fn xml_loadout(&mut self, path: &str) -> Vec<(String, String)> {
        if let Some(v) = self.loadout_cache.get(&path.to_lowercase()) {
            return v.clone();
        }
        let v = read_p4k(&self.holo.assets, path)
            .ok()
            .and_then(|b| decode(&b).ok().flatten())
            .map(|root| {
                root.find_all("Item")
                    .filter_map(|it| {
                        Some((
                            it.attr("portName")?.to_string(),
                            it.attr("itemName")?.to_string(),
                        ))
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        self.loadout_cache.insert(path.to_lowercase(), v.clone());
        v
    }

    fn has_grid(&mut self, inst: &Instance, depth: u8) -> bool {
        let db = self.holo.datacore.db();
        let (mounts, _) = self.mounts(inst);
        for (_, rec) in mounts {
            let gi = rec.as_instance();
            if matches!(grid_box(db, &gi), Some((_, _, true))) {
                return true;
            }
            if depth < 1 && self.has_grid(&gi, depth + 1) {
                return true;
            }
        }
        false
    }

    fn geometries(&self, inst: &Instance) -> Vec<String> {
        let db = self.holo.datacore.db();
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        walk(db, inst, 10, &mut 150_000, &mut |ci| {
            for p in ci.properties() {
                if let Some(s) = p.value.as_str() {
                    let l = s.to_lowercase();
                    if (l.ends_with(".cga") || l.ends_with(".cgf"))
                        && seen.insert(l)
                        && out.len() < 6
                    {
                        out.push(s.to_string());
                    }
                }
            }
        });
        out
    }

    fn nmc_for(&mut self, cga: &str) -> Option<BTreeMap<String, Mat34>> {
        self.nmc_cache
            .entry(cga.to_lowercase())
            .or_insert_with(|| {
                let bytes = read_p4k(&self.holo.assets, cga).ok()?;
                let nodes = nmc::parse(&bytes)?;
                let worlds = nmc::world_transforms(&nodes);
                Some(
                    nodes
                        .iter()
                        .zip(worlds.iter())
                        .map(|(n, w)| (n.name.to_lowercase(), *w))
                        .collect(),
                )
            })
            .clone()
    }

    #[allow(clippy::too_many_arguments)]
    fn descend(
        &mut self,
        inst: &Instance,
        base: &Mat34,
        geoms: &[String],
        chain: &str,
        depth: u8,
        fallbacks: &[(BTreeMap<String, Mat34>, Mat34)],
        visited: &mut HashSet<String>,
    ) {
        let db = self.holo.datacore.db();
        let maps: Vec<BTreeMap<String, Mat34>> =
            geoms.iter().filter_map(|g| self.nmc_for(g)).collect();
        let lookup = |port: &str| -> Option<(Mat34, Mat34)> {
            let p = port.to_lowercase();
            for m in &maps {
                if let Some(n) = m.get(&p) {
                    return Some((*n, *base));
                }
            }
            for (m, b) in fallbacks.iter().rev() {
                if let Some(n) = m.get(&p) {
                    return Some((*n, *b));
                }
            }
            None
        };
        let (mounts, attach) = self.mounts(inst);
        // explicit DCB attachment (anchor node ∘ offset) → NMC node named
        // after the port → item origin.
        let resolve_port = |port: &str| -> Option<(Mat34, Mat34)> {
            if let Some(pa) = attach.get(&port.to_lowercase()) {
                let (anchor, anchor_base) = if pa.helper.is_empty() {
                    (IDENTITY, *base)
                } else {
                    lookup(&pa.helper).unwrap_or((IDENTITY, *base))
                };
                return Some((mat_mul(&anchor, &pa.offset), anchor_base));
            }
            lookup(port)
        };
        for (port, rec) in mounts {
            let key = if chain.is_empty() {
                port.clone()
            } else {
                format!("{chain}/{port}")
            };
            let gi = rec.as_instance();
            if let Some((dims, off, true)) = grid_box(db, &gi) {
                // no attachment + no node anywhere → item origin (engine default)
                let (node, node_base) = resolve_port(&port).unwrap_or((IDENTITY, *base));
                let w = mat_mul(&node_base, &node);
                let lmin = [off[0] - dims[0] / 2.0, off[1] - dims[1] / 2.0, off[2]];
                let lmax = [
                    off[0] + dims[0] / 2.0,
                    off[1] + dims[1] / 2.0,
                    off[2] + dims[2],
                ];
                let (wmin, wmax) = transform_aabb(&w, lmin, lmax);
                self.grids.push(RawGrid {
                    port: key,
                    min: wmin,
                    max: wmax,
                });
            } else if depth < 2 {
                if !visited.insert(format!("{}", rec.id())) {
                    continue;
                }
                if !self.has_grid(&gi, 0) {
                    continue;
                }
                let Some((node, node_base)) = resolve_port(&port) else {
                    self.notes
                        .push(format!("item at '{key}' has grids but no parent node"));
                    continue;
                };
                let child_base = mat_mul(&node_base, &node);
                let child_geoms = self.geometries(&gi);
                let mut fb = fallbacks.to_vec();
                for m in maps.clone() {
                    fb.push((m, *base));
                }
                self.descend(
                    &gi,
                    &child_base,
                    &child_geoms,
                    &key,
                    depth + 1,
                    &fb,
                    visited,
                );
            }
        }
    }
}

fn grid_box(db: &DataCoreDatabase, gi: &Instance) -> Option<([f32; 3], [f32; 3], bool)> {
    let icc = find_component(db, gi, "SCItemInventoryContainerComponentParams")?;
    let cg = icc
        .get("containerParams")
        .and_then(|v| v.as_record_ref())
        .map(|r| r.guid)?;
    let c = db.record(&cg)?.as_instance();
    let dims = c
        .get_instance("interiorDimensions")
        .map(vec3)
        .unwrap_or([0.0; 3]);
    let it = c.get_instance("inventoryType");
    let open = it
        .as_ref()
        .and_then(|i| i.type_name())
        .map(|t| t.starts_with("InventoryOpen"))
        .unwrap_or(false);
    let off = it
        .and_then(|i| i.get_instance("gridPosOffset"))
        .map(vec3)
        .unwrap_or([0.0; 3]);
    Some((dims, off, open))
}

/// Face cells with world-space quad corners + distance to nearest wall box.
///
/// Distance = min over (a) boxes ahead of the face (exact) and (b) enclosing
/// shells the face is embedded in (upper bound to their far side — shell-built
/// ships like the Freelancer would otherwise read falsely open). The bottom
/// face (−Z) is 0 by construction: grids are bottom-anchored on the deck.
fn face_cells(
    g: &RawGrid,
    axis: usize,
    dir: i32,
    walls: &[WallView],
    soup: &[[[f32; 3]; 3]],
) -> Vec<FaceCell> {
    let face = if dir < 0 { g.min[axis] } else { g.max[axis] };
    let (u, v) = match axis {
        0 => (1, 2),
        1 => (0, 2),
        _ => (0, 1),
    };
    let nu = (((g.max[u] - g.min[u]) / CELL_M).round() as i32).max(1);
    let nv = (((g.max[v] - g.min[v]) / CELL_M).round() as i32).max(1);
    let floor = axis == 2 && dir < 0;
    let mut out = Vec::with_capacity((nu * nv) as usize);

    // per-face slab prune of the triangle soup (triangle mode)
    let mut slab_min = g.min;
    let mut slab_max = g.max;
    if dir > 0 {
        slab_min[axis] = face - 0.3;
        slab_max[axis] = face + MAX_PROBE_M;
    } else {
        slab_min[axis] = face - MAX_PROBE_M;
        slab_max[axis] = face + 0.3;
    }
    let face_tris: Vec<&[[f32; 3]; 3]> = if floor {
        Vec::new()
    } else {
        soup.iter()
            .filter(|t| {
                (0..3).all(|a| {
                    let lo = t[0][a].min(t[1][a]).min(t[2][a]);
                    let hi = t[0][a].max(t[1][a]).max(t[2][a]);
                    hi >= slab_min[a] && lo <= slab_max[a]
                })
            })
            .collect()
    };
    let use_tris = !face_tris.is_empty();
    let mut ndir = [0.0f32; 3];
    ndir[axis] = dir as f32;

    for iu in 0..nu {
        for iv in 0..nv {
            let u0 = g.min[u] + iu as f32 * CELL_M;
            let u1 = g.min[u] + (iu + 1) as f32 * CELL_M;
            let v0 = g.min[v] + iv as f32 * CELL_M;
            let v1 = g.min[v] + (iv + 1) as f32 * CELL_M;
            let best = if floor {
                0.0
            } else if use_tris {
                // conservative min over a 5-ray bundle per cell
                let cu = (u0 + u1) / 2.0;
                let cv = (v0 + v1) / 2.0;
                let s = CELL_M * 0.3;
                let mut worst = MAX_PROBE_M;
                for (du, dv) in [(0.0, 0.0), (-s, -s), (s, -s), (-s, s), (s, s)] {
                    let mut orig = [0.0f32; 3];
                    orig[axis] = face - dir as f32 * 0.05;
                    orig[u] = cu + du;
                    orig[v] = cv + dv;
                    let mut hit = MAX_PROBE_M;
                    for t in &face_tris {
                        if let Some(d) = ray_tri(orig, ndir, t[0], t[1], t[2])
                            && d >= 0.0
                            && d - 0.05 < hit
                        {
                            hit = (d - 0.05).max(0.0);
                        }
                    }
                    if hit < worst {
                        worst = hit;
                    }
                }
                worst
            } else {
                let (pu0, pu1, pv0, pv1) = (u0 + 0.15, u1 - 0.15, v0 + 0.15, v1 - 0.15);
                let mut best = MAX_PROBE_M;
                for w in walls {
                    if w.max[u] <= pu0 || w.min[u] >= pu1 || w.max[v] <= pv0 || w.min[v] >= pv1 {
                        continue;
                    }
                    let d = if dir > 0 {
                        w.min[axis] - face
                    } else {
                        face - w.max[axis]
                    };
                    if d > -0.3 {
                        if d < best {
                            best = d.max(0.0);
                        }
                    } else if w.min[axis] < face && w.max[axis] > face {
                        let ds = if dir > 0 {
                            w.max[axis] - face
                        } else {
                            face - w.min[axis]
                        };
                        if ds > 0.0 && ds < best {
                            best = ds;
                        }
                    }
                }
                best
            };
            let mut corners = [[0.0f32; 3]; 4];
            for (k, (cu, cv)) in [(u0, v0), (u1, v0), (u1, v1), (u0, v1)].iter().enumerate() {
                corners[k][axis] = face;
                corners[k][u] = *cu;
                corners[k][v] = *cv;
            }
            out.push(FaceCell {
                corners,
                dist: best,
                axis,
                dir,
            });
        }
    }
    out
}

/// Möller–Trumbore ray/triangle intersection (distance along unit `dir`).
fn ray_tri(orig: [f32; 3], dir: [f32; 3], v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> Option<f32> {
    let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
    let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
    let pv = [
        dir[1] * e2[2] - dir[2] * e2[1],
        dir[2] * e2[0] - dir[0] * e2[2],
        dir[0] * e2[1] - dir[1] * e2[0],
    ];
    let det = e1[0] * pv[0] + e1[1] * pv[1] + e1[2] * pv[2];
    if det.abs() < 1e-9 {
        return None;
    }
    let inv = 1.0 / det;
    let tv = [orig[0] - v0[0], orig[1] - v0[1], orig[2] - v0[2]];
    let uu = (tv[0] * pv[0] + tv[1] * pv[1] + tv[2] * pv[2]) * inv;
    if !(0.0..=1.0).contains(&uu) {
        return None;
    }
    let qv = [
        tv[1] * e1[2] - tv[2] * e1[1],
        tv[2] * e1[0] - tv[0] * e1[2],
        tv[0] * e1[1] - tv[1] * e1[0],
    ];
    let vv = (dir[0] * qv[0] + dir[1] * qv[1] + dir[2] * qv[2]) * inv;
    if vv < 0.0 || uu + vv > 1.0 {
        return None;
    }
    Some((e2[0] * qv[0] + e2[1] * qv[1] + e2[2] * qv[2]) * inv)
}

// ── DCB object-container refs ────────────────────────────────────────────────

struct OcRef {
    file_name: String,
    bone_name: Option<String>,
    offset_pos: [f32; 3],
    offset_rot_deg: [f32; 3],
}

fn object_container_refs(db: &DataCoreDatabase, ship: &Instance) -> Vec<OcRef> {
    let mut out = Vec::new();
    walk(db, ship, 22, &mut 3_000_000, &mut |ci| {
        if ci.type_name() != Some("SVehicleObjectContainerParams") {
            return;
        }
        let Some(file) = ci.get_str("fileName") else {
            return;
        };
        if !file.to_lowercase().ends_with(".socpak") {
            return;
        }
        let bone = ci
            .get_str("boneName")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        let offset = ci.get_instance("Offset");
        let pos = offset
            .as_ref()
            .and_then(|o| o.get_instance("Position"))
            .map(vec3)
            .unwrap_or([0.0; 3]);
        let rot = offset
            .as_ref()
            .and_then(|o| o.get_instance("Rotation"))
            .map(|r| {
                if r.get("x").is_some() {
                    vec3(r)
                } else {
                    r.get_instance("Rotation").map(vec3).unwrap_or([0.0; 3])
                }
            })
            .unwrap_or([0.0; 3]);
        out.push(OcRef {
            file_name: file.to_string(),
            bone_name: bone,
            offset_pos: pos,
            offset_rot_deg: rot,
        });
    });
    out
}

fn compose_container(helper: Option<Mat34>, offset: &Mat34) -> Mat34 {
    match helper {
        Some(h) => {
            let th = mat_translation(&h);
            let to = mat_translation(offset);
            let dup = (0..3).all(|a| (th[a] - to[a]).abs() < 0.015);
            if dup { *offset } else { mat_mul(&h, offset) }
        }
        None => *offset,
    }
}

// ── socpak placements ────────────────────────────────────────────────────────

struct Placement {
    cgf: String,
    local: Mat34,
}

/// A cargo-grid entity placed directly in an interior object container.
struct OcGrid {
    label: String,
    local: Mat34,
    dims: [f32; 3],
    off: [f32; 3],
}

fn read_section_placements(
    assets: &AssetSource,
    db: &DataCoreDatabase,
    socpak_path: &str,
) -> (Vec<Placement>, Vec<OcGrid>) {
    let Ok(bytes) = read_p4k(assets, socpak_path) else {
        return (Vec::new(), Vec::new());
    };
    let Ok(mut pak) = Socpak::open(bytes) else {
        return (Vec::new(), Vec::new());
    };
    let mut out = Vec::new();
    let mut oc_grids = Vec::new();
    for i in 0..pak.len() {
        let Some(name) = pak.name(i) else { continue };
        if !name.to_lowercase().ends_with(".soc") {
            continue;
        }
        let Ok(soc) = pak.read(i) else { continue };
        for (ty, data) in crch_chunks(&soc) {
            if ty == 0x0010 {
                out.extend(parse_included_objects(data));
            }
        }
        if let Ok(Some(root)) = decode(&soc) {
            for ent in root.find_all("Entity") {
                if let Some(g) = entity_oc_grid(db, ent) {
                    oc_grids.push(g);
                    continue;
                }
                if let Some(pl) = entity_placement(db, ent) {
                    out.push(pl);
                }
            }
        }
    }
    (out, oc_grids)
}

/// Is this OC entity a cargo grid (open InventoryContainer on its class)?
fn entity_oc_grid(db: &DataCoreDatabase, ent: &XmlNode) -> Option<OcGrid> {
    let rec = ent
        .attr("EntityClassGUID")
        .and_then(|g| g.parse::<svarog_common::CigGuid>().ok())
        .and_then(|g| db.record(&g))
        .or_else(|| ent.attr("EntityClass").and_then(|n| entity_by_short(db, n)))?;
    let (dims, off, open) = grid_box(db, &rec.as_instance())?;
    if !open {
        return None;
    }
    let pos = parse_csv3(ent.attr("Pos").unwrap_or("0,0,0"));
    let q = parse_csv4(ent.attr("Rotate").unwrap_or("1,0,0,0"));
    let label = ent
        .attr("Name")
        .map(|s| s.to_string())
        .or_else(|| {
            rec.name()
                .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
        })
        .unwrap_or_else(|| "oc_grid".into());
    Some(OcGrid {
        label,
        local: mat_from_pos_quat_scale(pos, q, [1.0, 1.0, 1.0]),
        dims,
        off,
    })
}

fn entity_placement(db: &DataCoreDatabase, ent: &XmlNode) -> Option<Placement> {
    let pos = parse_csv3(ent.attr("Pos").unwrap_or("0,0,0"));
    let q = parse_csv4(ent.attr("Rotate").unwrap_or("1,0,0,0"));
    let scale = parse_csv3(ent.attr("Scale").unwrap_or("1,1,1"));
    let mut geom: Option<String> = None;
    for pdc in ent.children_named("PropertiesDataCore") {
        for n in pdc.descendants() {
            if n.tag == "Geometry"
                && let Some(p) = n.attr("path")
                && !p.is_empty()
            {
                geom = Some(p.to_string());
                break;
            }
        }
    }
    if geom.is_none()
        && let Some(gs) = ent.attr("EntityClassGUID")
        && let Ok(guid) = gs.parse::<svarog_common::CigGuid>()
        && let Some(rec) = db.record(&guid)
    {
        geom = first_geometry_path(db, &rec.as_instance());
    }
    let geom = geom?;
    let l = geom.to_lowercase();
    if !(l.ends_with(".cgf") || l.ends_with(".cga")) {
        return None;
    }
    Some(Placement {
        cgf: geom,
        local: mat_from_pos_quat_scale(pos, q, scale),
    })
}

fn crch_chunks(soc: &[u8]) -> Vec<(u16, &[u8])> {
    if soc.len() < 16 || &soc[..4] != b"CrCh" {
        return Vec::new();
    }
    let rd32 = |p: usize| u32::from_le_bytes(soc[p..p + 4].try_into().unwrap());
    let count = rd32(8) as usize;
    let table = rd32(12) as usize;
    let mut out = Vec::new();
    for i in 0..count {
        let base = table + i * 16;
        if base + 16 > soc.len() {
            break;
        }
        let ty = u16::from_le_bytes(soc[base..base + 2].try_into().unwrap());
        let size = rd32(base + 8) as usize;
        let off = rd32(base + 12) as usize;
        if off + size <= soc.len() {
            out.push((ty, &soc[off..off + size]));
        }
    }
    out
}

fn parse_included_objects(data: &[u8]) -> Vec<Placement> {
    const STR_LEN: usize = 256;
    let rd_u32 = |p: usize| -> Option<u32> {
        data.get(p..p + 4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
    };
    let rd_u16 = |p: usize| -> Option<u16> {
        data.get(p..p + 2)
            .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
    };
    let rd_f64 = |p: usize| -> Option<f64> {
        data.get(p..p + 8)
            .map(|b| f64::from_le_bytes(b.try_into().unwrap()))
    };
    let rd_str = |p: usize| -> Option<String> {
        let s = data.get(p..p + STR_LEN)?;
        let end = s.iter().position(|&b| b == 0).unwrap_or(STR_LEN);
        Some(String::from_utf8_lossy(&s[..end]).into_owned())
    };

    let mut off = 4usize;
    let Some(num_cgfs) = rd_u32(off) else {
        return Vec::new();
    };
    off += 4;
    let mut cgf_paths = Vec::with_capacity(num_cgfs as usize);
    for _ in 0..num_cgfs {
        let Some(s) = rd_str(off) else {
            return Vec::new();
        };
        cgf_paths.push(s);
        off += STR_LEN;
    }
    let Some(num_materials) = rd_u16(off) else {
        return Vec::new();
    };
    let Some(num_palettes) = rd_u16(off + 2) else {
        return Vec::new();
    };
    off += 4;
    off += (num_materials as usize + num_palettes as usize) * STR_LEN;
    off += 28;
    let Some(len_objects) = rd_u32(off) else {
        return Vec::new();
    };
    off += 4;
    let objects_end = (off + len_objects as usize).min(data.len());

    let mut out = Vec::new();
    while off + 4 <= objects_end {
        let Some(ty) = rd_u32(off) else { break };
        match ty {
            1 => {
                if off + 168 > data.len() {
                    break;
                }
                let Some(id) = rd_u16(off + 60) else { break };
                let mut local = [[0.0f32; 4]; 3];
                let mut ok = true;
                for (r, row) in local.iter_mut().enumerate() {
                    for (c, cell) in row.iter_mut().enumerate() {
                        match rd_f64(off + 64 + (r * 4 + c) * 8) {
                            Some(v) => *cell = v as f32,
                            None => ok = false,
                        }
                    }
                }
                let unknown3 = rd_f64(off + 160).map(|v| v.to_bits()).unwrap_or(1);
                let base = if unknown3 == 0 { 168 + 16 } else { 168 };
                let mut end = off + base;
                while end + 4 <= objects_end {
                    match rd_u32(end) {
                        Some(0) => end += 4,
                        _ => break,
                    }
                }
                if ok && let Some(cgf) = cgf_paths.get(id as usize) {
                    out.push(Placement {
                        cgf: cgf.clone(),
                        local,
                    });
                }
                off = end;
            }
            7 => off += 152,
            0x10 => off += 136,
            _ => off += 4,
        }
        if off > objects_end {
            break;
        }
    }
    out
}

// ── CGF model AABB ───────────────────────────────────────────────────────────

/// A decoded triangle mesh in model space (IVO `IvoSkin2` streams from the
/// `.cgfm`/`.cgam` companion — see `docs/ship-cargo-grids.md` §Tier C).
struct TriMesh {
    positions: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

fn cgf_triangles(assets: &AssetSource, path: &str) -> Option<TriMesh> {
    let companion = format!("{path}m");
    let bytes = read_p4k(assets, &companion)
        .or_else(|_| read_p4k(assets, path))
        .ok()?;
    if bytes.len() < 16 || &bytes[..4] != b"#ivo" {
        return None;
    }
    let rd32 = |p: usize| -> Option<u32> {
        bytes
            .get(p..p + 4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
    };
    let count = rd32(8)? as usize;
    let table = rd32(12)? as usize;
    let mut skin: Option<(usize, usize)> = None;
    let mut offsets: Vec<usize> = Vec::new();
    for i in 0..count {
        let base = table + i * 16;
        let off = u64::from_le_bytes(bytes.get(base + 8..base + 16)?.try_into().ok()?) as usize;
        offsets.push(off);
        if rd32(base)? == 0xB8757777 {
            skin = Some((off, bytes.len()));
        }
    }
    let (off, mut end) = skin?;
    for &o in &offsets {
        if o > off && o < end {
            end = o;
        }
    }
    let d = bytes.get(off..end)?;
    let g32 = |p: usize| -> Option<u32> {
        d.get(p..p + 4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
    };
    let gf = |p: usize| -> Option<f32> { Some(f32::from_bits(g32(p)?)) };
    let num_vertices = g32(8)? as usize;
    let num_indices = g32(12)? as usize;
    let num_submeshes = g32(16)? as usize;
    let mut scale_min = [0.0f32; 3];
    let mut scale_max = [0.0f32; 3];
    for a in 0..3 {
        scale_min[a] = gf(4 + 44 + a * 4)?;
        scale_max[a] = gf(4 + 56 + a * 4)?;
    }
    let extra_count = g32(4 + 72)? as usize;
    if num_vertices == 0 || num_indices == 0 || num_vertices > 4_000_000 {
        return None;
    }
    let sub_off = 168;
    let mut subs: Vec<(u32, u32, u32)> = Vec::with_capacity(num_submeshes);
    for i in 0..num_submeshes {
        let b = sub_off + i * 48;
        subs.push((g32(b + 4)?, g32(b + 8)?, g32(b + 16)?));
    }
    let mut p = sub_off + num_submeshes * 48 + extra_count * 4;
    const IVOVERTSUVS: u32 = 0x91329AE9;
    const IVOVERTSUVS2: u32 = 0xB3A70D5E;
    const IVOINDICES: u32 = 0xEECDC168;
    let mut positions: Option<Vec<[f32; 3]>> = None;
    let mut indices: Option<Vec<u32>> = None;
    while p + 8 <= d.len() {
        let tag = g32(p)?;
        if tag == 0 {
            p += 4;
            continue;
        }
        let elem = g32(p + 4)? as usize;
        let start = p + 8;
        let size;
        match tag {
            IVOVERTSUVS | IVOVERTSUVS2 => {
                size = elem * num_vertices;
                let s = d.get(start..start + size)?;
                let mut v = Vec::with_capacity(num_vertices);
                if elem == 16 {
                    for i in 0..num_vertices {
                        let b = &s[i * 16..];
                        let q = [
                            u16::from_le_bytes(b[0..2].try_into().unwrap()),
                            u16::from_le_bytes(b[2..4].try_into().unwrap()),
                            u16::from_le_bytes(b[4..6].try_into().unwrap()),
                        ];
                        let mut o = [0.0f32; 3];
                        for a in 0..3 {
                            let snorm = q[a] as i16 as f32 / 32767.0;
                            let he = ((scale_max[a] - scale_min[a]) / 2.0).max(1.0);
                            let c = (scale_max[a] + scale_min[a]) / 2.0;
                            o[a] = snorm * he + c;
                        }
                        v.push(o);
                    }
                } else if elem == 20 {
                    for i in 0..num_vertices {
                        let b = &s[i * 20..];
                        v.push([
                            f32::from_le_bytes(b[0..4].try_into().unwrap()),
                            f32::from_le_bytes(b[4..8].try_into().unwrap()),
                            f32::from_le_bytes(b[8..12].try_into().unwrap()),
                        ]);
                    }
                } else {
                    return None;
                }
                positions = Some(v);
            }
            IVOINDICES => {
                size = elem * num_indices;
                let s = d.get(start..start + size)?;
                let mut v = Vec::with_capacity(num_indices);
                if elem == 2 {
                    for i in 0..num_indices {
                        v.push(u16::from_le_bytes(s[i * 2..i * 2 + 2].try_into().unwrap()) as u32);
                    }
                    for &(first, n, base) in &subs {
                        if base != 0 {
                            for k in first..first + n {
                                if let Some(x) = v.get_mut(k as usize) {
                                    *x += base;
                                }
                            }
                        }
                    }
                } else {
                    for i in 0..num_indices {
                        v.push(u32::from_le_bytes(s[i * 4..i * 4 + 4].try_into().unwrap()));
                    }
                }
                indices = Some(v);
            }
            _ => {
                size = elem * num_vertices;
                if start + size > d.len() {
                    break;
                }
            }
        }
        p = start + size;
        let rem = size % 8;
        if rem != 0 {
            p += 8 - rem;
        }
        if positions.is_some() && indices.is_some() {
            break;
        }
    }
    let positions = positions?;
    let mut indices = indices?;
    indices.retain(|&i| (i as usize) < positions.len());
    let tri_count = indices.len() / 3;
    indices.truncate(tri_count * 3);
    Some(TriMesh { positions, indices })
}

/// Model-space SUB-boxes: one per NMC geometry node (per-node bbox, node-local,
/// composed through the node's world transform — decomposes monolithic shells
/// into floor/wall/ceiling panels). Falls back to the single model AABB.
fn cgf_sub_boxes(assets: &AssetSource, path: &str) -> Vec<Aabb> {
    if let Ok(bytes) = read_p4k(assets, path)
        && let Some(nodes) = nmc::parse(&bytes)
        && !nodes.is_empty()
    {
        let worlds = nmc::world_transforms(&nodes);
        let boxes: Vec<Aabb> = nodes
            .iter()
            .zip(worlds.iter())
            .filter(|(n, _)| {
                n.geometry_type == 0
                    && (0..3).any(|a| n.bbox_max[a] - n.bbox_min[a] > 0.01)
                    // exclude LIGHT-GLOW geometry (emissive volumes projecting
                    // into the room — they'd read as walls cutting into grids)
                    && !is_light_glow(&n.name)
            })
            .map(|(n, w)| transform_aabb(w, n.bbox_min, n.bbox_max))
            .collect();
        if !boxes.is_empty() {
            return boxes;
        }
    }
    cgf_model_aabb(assets, path).into_iter().collect()
}

fn is_light_glow(name: &str) -> bool {
    let l = name.to_lowercase();
    l.starts_with("lg_")
        || l.starts_with("light")
        || l.contains("_light")
        || l.contains("glow")
        || l.contains("blinker")
        || l.starts_with("vfx")
}

fn cgf_model_aabb(assets: &AssetSource, path: &str) -> Option<Aabb> {
    let bytes = read_p4k(assets, path).ok()?;
    if bytes.len() >= 4 && &bytes[..4] == b"#ivo" {
        let rd32 = |p: usize| u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap());
        let count = rd32(8) as usize;
        let table = rd32(12) as usize;
        for i in 0..count {
            let base = table + i * 16;
            if base + 16 > bytes.len() {
                break;
            }
            if rd32(base) == 0x92914444 {
                let off =
                    u64::from_le_bytes(bytes[base + 8..base + 16].try_into().unwrap()) as usize;
                let head = &bytes[off..bytes.len().min(off + 64)];
                return aabb_at(head, 24).or_else(|| scan_aabb(head));
            }
        }
        None
    } else if bytes.len() >= 4 && &bytes[..4] == b"CrCh" {
        for (ty, data) in crch_chunks(&bytes) {
            if ty == 0x1000 {
                return aabb_at(data, 108).or_else(|| scan_aabb(&data[..data.len().min(256)]));
            }
        }
        None
    } else {
        None
    }
}

fn aabb_at(data: &[u8], byte_off: usize) -> Option<Aabb> {
    if byte_off + 24 > data.len() {
        return None;
    }
    let f = |i: usize| {
        f32::from_bits(u32::from_le_bytes(
            data[byte_off + i * 4..byte_off + i * 4 + 4]
                .try_into()
                .unwrap(),
        ))
    };
    let v: Vec<f32> = (0..6).map(f).collect();
    let ok = v.iter().all(|x| x.is_finite() && x.abs() < 2.0e3)
        && v[0] < v[3]
        && v[1] < v[4]
        && v[2] < v[5];
    ok.then(|| ([v[0], v[1], v[2]], [v[3], v[4], v[5]]))
}

fn scan_aabb(head: &[u8]) -> Option<Aabb> {
    let n = head.len() / 4;
    let f = |i: usize| {
        f32::from_bits(u32::from_le_bytes(
            head[i * 4..i * 4 + 4].try_into().unwrap(),
        ))
    };
    for w in 0..n.saturating_sub(5) {
        let v: Vec<f32> = (w..w + 6).map(f).collect();
        let ok = v.iter().all(|x| x.is_finite() && x.abs() < 2.0e3)
            && v[0] < v[3]
            && v[1] < v[4]
            && v[2] < v[5]
            && (v[3] - v[0]) > 0.05
            && (v[4] - v[1]) > 0.05;
        if ok {
            return Some(([v[0], v[1], v[2]], [v[3], v[4], v[5]]));
        }
    }
    None
}

// ── db plumbing ──────────────────────────────────────────────────────────────

fn shortest_entity<'a>(
    db: &'a DataCoreDatabase,
    name: &str,
) -> Option<svarog_datacore::Record<'a>> {
    let mut c: Vec<_> = db
        .records_by_type("EntityClassDefinition")
        .filter(|r| {
            let n = r.name().unwrap_or("");
            n.rsplit('.').next().unwrap_or(n).eq_ignore_ascii_case(name)
        })
        .collect();
    c.sort_by_key(|r| r.name().map(|n| n.len()).unwrap_or(usize::MAX));
    c.into_iter().next()
}

fn entity_by_short<'a>(
    db: &'a DataCoreDatabase,
    short: &str,
) -> Option<svarog_datacore::Record<'a>> {
    db.records_by_type("EntityClassDefinition").find(|r| {
        let n = r.name().unwrap_or("");
        n.rsplit('.')
            .next()
            .unwrap_or(n)
            .eq_ignore_ascii_case(short)
    })
}

fn first_cga(db: &DataCoreDatabase, inst: &Instance) -> Option<String> {
    let mut found = None;
    walk(db, inst, 12, &mut 400_000, &mut |ci| {
        if found.is_some() {
            return;
        }
        for p in ci.properties() {
            if let Some(s) = p.value.as_str()
                && s.to_lowercase().ends_with(".cga")
            {
                found = Some(s.to_string());
                return;
            }
        }
    });
    found
}

fn first_geometry_path(db: &DataCoreDatabase, inst: &Instance) -> Option<String> {
    let mut found = None;
    walk(db, inst, 10, &mut 100_000, &mut |ci| {
        if found.is_some() {
            return;
        }
        for p in ci.properties() {
            if let Some(s) = p.value.as_str() {
                let l = s.to_lowercase();
                if l.ends_with(".cgf") || l.ends_with(".cga") {
                    found = Some(s.to_string());
                    return;
                }
            }
        }
    });
    found
}

fn find_component<'a>(
    db: &'a DataCoreDatabase,
    e: &Instance<'a>,
    ty: &str,
) -> Option<Instance<'a>> {
    for p in e.properties() {
        if let Value::Array(_) = p.value
            && let Some(arr) = e.get_array(p.name)
        {
            for elem in arr {
                if let Some(ci) = value_to_instance(db, &elem)
                    && ci.type_name() == Some(ty)
                {
                    return Some(ci);
                }
            }
        }
    }
    None
}

fn vec3(v: Instance) -> [f32; 3] {
    [
        v.get_f32("x").unwrap_or(0.0),
        v.get_f32("y").unwrap_or(0.0),
        v.get_f32("z").unwrap_or(0.0),
    ]
}

fn parse_csv3(s: &str) -> [f32; 3] {
    let mut it = s.split(',').map(|t| t.trim().parse::<f32>().unwrap_or(0.0));
    [
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
    ]
}

fn parse_csv4(s: &str) -> [f32; 4] {
    let mut it = s.split(',').map(|t| t.trim().parse::<f32>().unwrap_or(0.0));
    [
        it.next().unwrap_or(1.0),
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
    ]
}

fn read_p4k(assets: &AssetSource, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if let Some(b) = assets.try_read(path)? {
        return Ok(b);
    }
    assets
        .try_read(&format!("Data\\{path}"))?
        .ok_or_else(|| format!("not in p4k: {path}").into())
}

fn walk<'a>(
    db: &'a DataCoreDatabase,
    root: &Instance<'a>,
    max_depth: u32,
    budget: &mut u32,
    visit: &mut dyn FnMut(&Instance<'a>),
) {
    let mut stack: Vec<(Instance<'a>, u32)> = vec![(*root, 0)];
    while let Some((inst, depth)) = stack.pop() {
        if *budget == 0 {
            return;
        }
        *budget -= 1;
        visit(&inst);
        if depth >= max_depth {
            continue;
        }
        for p in inst.properties() {
            match p.value {
                Value::Array(_) => {
                    if let Some(arr) = inst.get_array(p.name) {
                        for elem in arr {
                            if let Some(ci) = value_to_instance(db, &elem) {
                                stack.push((ci, depth + 1));
                            }
                        }
                    }
                }
                Value::Class { .. }
                | Value::ClassRef(_)
                | Value::StrongPointer(Some(_))
                | Value::WeakPointer(Some(_)) => {
                    if let Some(ci) = value_to_instance(db, &p.value) {
                        stack.push((ci, depth + 1));
                    }
                }
                _ => {}
            }
        }
    }
}

fn value_to_instance<'a>(db: &'a DataCoreDatabase, v: &Value<'a>) -> Option<Instance<'a>> {
    match v {
        Value::Class { struct_index, data } => {
            Some(Instance::from_inline_data(db, *struct_index, data))
        }
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}

// ── NMC reader with parent hierarchy (StarBreaker port, MIT) ─────────────────

mod nmc {
    use super::Mat34;

    const NMC_FULL: u32 = 0x70697FDA;
    const IVO_MAGIC: u32 = 0x6F766923;

    pub struct Node {
        pub name: String,
        pub local: Mat34,
        pub parent: Option<u16>,
        /// Per-node bounding box (entry +152, node-local space).
        pub bbox_min: [f32; 3],
        pub bbox_max: [f32; 3],
        /// Node type at +150 (0 = geometry, 3 = helper, …).
        pub geometry_type: u16,
    }

    pub fn world_transforms(nodes: &[Node]) -> Vec<Mat34> {
        fn resolve(i: usize, nodes: &[Node], world: &mut [Option<Mat34>]) -> Mat34 {
            if let Some(w) = world[i] {
                return w;
            }
            let w = match nodes[i].parent {
                Some(pi) if (pi as usize) < nodes.len() && (pi as usize) != i => {
                    let pw = resolve(pi as usize, nodes, world);
                    super::mat_mul(&pw, &nodes[i].local)
                }
                _ => nodes[i].local,
            };
            world[i] = Some(w);
            w
        }
        let mut world = vec![None; nodes.len()];
        (0..nodes.len())
            .map(|i| resolve(i, nodes, &mut world))
            .collect()
    }

    struct R<'a> {
        d: &'a [u8],
        p: usize,
    }
    impl<'a> R<'a> {
        fn u32(&mut self) -> Option<u32> {
            let b = self.d.get(self.p..self.p + 4)?;
            self.p += 4;
            Some(u32::from_le_bytes(b.try_into().ok()?))
        }
        fn i32(&mut self) -> Option<i32> {
            Some(self.u32()? as i32)
        }
        fn u16(&mut self) -> Option<u16> {
            let b = self.d.get(self.p..self.p + 2)?;
            self.p += 2;
            Some(u16::from_le_bytes(b.try_into().ok()?))
        }
        fn f32(&mut self) -> Option<f32> {
            Some(f32::from_bits(self.u32()?))
        }
        fn adv(&mut self, n: usize) -> Option<()> {
            self.p = self.p.checked_add(n).filter(|&e| e <= self.d.len())?;
            Some(())
        }
        fn bytes(&mut self, n: usize) -> Option<&'a [u8]> {
            let b = self.d.get(self.p..self.p + n)?;
            self.p += n;
            Some(b)
        }
    }

    pub fn parse(file: &[u8]) -> Option<Vec<Node>> {
        let mut h = R { d: file, p: 0 };
        if h.u32()? != IVO_MAGIC {
            return None;
        }
        let _v = h.u32()?;
        let count = h.u32()? as usize;
        let table = h.u32()? as usize;
        let mut off = None;
        for i in 0..count {
            let base = table + i * 16;
            let ty = u32::from_le_bytes(file.get(base..base + 4)?.try_into().ok()?);
            let o = u64::from_le_bytes(file.get(base + 8..base + 16)?.try_into().ok()?);
            if ty == NMC_FULL {
                off = Some(o as usize);
            }
        }
        let mut r = R {
            d: &file[off?..],
            p: 0,
        };
        let _h0 = r.i32()?;
        let n = r.i32()? as usize;
        let _g = r.i32()?;
        let n_unk = r.i32()? as usize;
        let n_mat = r.i32()? as usize;
        let str_sz = r.i32()? as usize;
        let _h6 = r.i32()?;
        let _h7 = r.i32()?;

        let mut locals = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);
        let mut bboxes = Vec::with_capacity(n);
        let mut geom_types = Vec::with_capacity(n);
        for _ in 0..n {
            r.adv(32)?;
            r.adv(48)?; // WorldToBone (unused)
            let mut local = [[0.0f32; 4]; 3];
            for row in &mut local {
                for v in row.iter_mut() {
                    *v = r.f32()?;
                }
            }
            r.adv(12 + 8)?;
            let parent = r.u16()?;
            let gt = r.u16()?;
            let mut bb = [0.0f32; 6];
            for v in bb.iter_mut() {
                *v = r.f32()?;
            }
            r.adv(32)?;
            locals.push(local);
            parents.push(if parent == 0xFFFF { None } else { Some(parent) });
            bboxes.push(bb);
            geom_types.push(gt);
        }
        r.adv(32)?;
        for _ in 0..n_unk {
            r.u16()?;
        }
        for _ in 0..n_mat {
            r.u16()?;
        }
        let sb = r.bytes(str_sz)?;
        let mut names = Vec::with_capacity(n);
        let mut pos = 0;
        for _ in 0..n {
            if pos >= sb.len() {
                names.push(String::new());
                continue;
            }
            let end = sb[pos..]
                .iter()
                .position(|&b| b == 0)
                .map(|q| pos + q)
                .unwrap_or(sb.len());
            names.push(
                std::str::from_utf8(&sb[pos..end])
                    .unwrap_or("")
                    .lines()
                    .next()
                    .unwrap_or("")
                    .to_string(),
            );
            pos = end + 1;
        }
        Some(
            (0..n)
                .map(|i| Node {
                    name: names[i].clone(),
                    local: locals[i],
                    parent: parents[i],
                    bbox_min: [bboxes[i][0], bboxes[i][1], bboxes[i][2]],
                    bbox_max: [bboxes[i][3], bboxes[i][4], bboxes[i][5]],
                    geometry_type: geom_types[i],
                })
                .collect(),
        )
    }
}
