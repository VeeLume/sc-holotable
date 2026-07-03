//! Tier B: resolve each cargo grid's **3-D placement inside the ship** by
//! reading the hull geometry.
//!
//! The DCB gives the grid box (`InventoryContainer.interiorDimensions`) and which
//! grid mounts at which item port (the ship loadout), but *not* the port's 3-D
//! transform — that lives in the ship's `.cga` as a named node in the
//! `NodeMeshCombos` (NMC, `0x70697FDA`) scene-graph chunk. This example ports the
//! minimal IVO chunk-table + NMC reader (from the MIT-licensed StarBreaker
//! reference), matches each `hardpoint_cargogrid_*` port to its NMC node, and
//! prints the grid's world position + box.
//!
//! ```bash
//! cargo run -p sc-extract --release --example cargo_grid_placement
//! cargo run -p sc-extract --release --example cargo_grid_placement -- MISC_Freelancer_MAX
//! ```
//!
//! See `docs/ship-cargo-grids.md` (§Geometry read — Tier B).

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

/// 1 SCU cell = 1.25 m per axis.
const CELL_M: f32 = 1.25;

struct GridMount {
    port: String,
    entity_class: String,
    dims_m: [f32; 3],
    offset: [f32; 3],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ship = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "MISC_Freelancer".into());

    let install = sc_discovery::discover_primary()?;
    println!(
        "{} v{}  —  ship '{ship}'",
        install.channel,
        install.short_version()
    );
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();

    // ── 1. locate the ship entity (shortest exact-ish name match) ─────────────
    let mut cands: Vec<_> = db
        .records_by_type("EntityClassDefinition")
        .filter(|r| {
            let n = r.name().unwrap_or("");
            let short = n.rsplit('.').next().unwrap_or(n);
            short.eq_ignore_ascii_case(&ship)
        })
        .collect();
    cands.sort_by_key(|r| r.name().map(|n| n.len()).unwrap_or(usize::MAX));
    let Some(rec) = cands.into_iter().next() else {
        eprintln!("no EntityClassDefinition named '{ship}'");
        return Ok(());
    };
    let inst = rec.as_instance();
    println!("entity: {}  ({})", rec.name().unwrap_or("?"), rec.id());

    // ── 2. hull geometry path (first .cga on SGeometryResourceParams) ─────────
    let mut hull_cga: Option<String> = None;
    walk(db, &inst, 12, &mut 400_000, &mut |ci| {
        if hull_cga.is_some() {
            return;
        }
        if ci.type_name() == Some("SGeometryResourceParams") {
            let mut sub = 20_000;
            walk(db, ci, 8, &mut sub, &mut |gi| {
                if hull_cga.is_some() {
                    return;
                }
                for p in gi.properties() {
                    if let Some(s) = p.value.as_str()
                        && s.to_lowercase().ends_with(".cga")
                    {
                        hull_cga = Some(s.to_string());
                        return;
                    }
                }
            });
        }
    });
    let Some(hull_cga) = hull_cga else {
        eprintln!("no .cga hull geometry found on the ship entity");
        return Ok(());
    };
    println!("hull geometry: {hull_cga}");

    // ── 3. enumerate every mounted item, keep the ones that ARE cargo grids ───
    //
    // Name-independent (design rule #5): a cargo grid is any mounted entity whose
    // InventoryContainer is an *open* type (`InventoryOpenContainerType` /
    // `InventoryOpenAlwaysContainerType`) — closed = personal inventory, and the
    // 35^3 `*_CargoGrid_Template` placeholders are closed/oversized and excluded.
    let mut mounts: Vec<GridMount> = Vec::new();
    let mut seen_ports: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut n_mount_sites = 0usize;
    walk(db, &inst, 22, &mut 3_000_000, &mut |ci| {
        // Grids reach a ship two ways: a manual loadout entry
        // (`SItemPortLoadoutEntryParams`) or a port's default item
        // (`SItemPortDefaultItemDef`). Both carry a port name + an entity link.
        let (port_field, cls_field, ref_field) = match ci.type_name() {
            Some("SItemPortLoadoutEntryParams") => {
                ("itemPortName", "entityClassName", "entityClassReference")
            }
            Some("SItemPortDefaultItemDef") => ("itemPort", "", "entityClass"),
            _ => return,
        };
        n_mount_sites += 1;
        let cls = if cls_field.is_empty() {
            ""
        } else {
            ci.get_str(cls_field).unwrap_or("")
        };
        let grid_rec = ci
            .get(ref_field)
            .and_then(|v| v.as_record_ref())
            .and_then(|r| db.record(&r.guid))
            .or_else(|| {
                if cls.is_empty() {
                    None
                } else {
                    find_entity_by_short_name(db, cls)
                }
            });
        let Some(grid_rec) = grid_rec else { return };
        let gi = grid_rec.as_instance();
        let Some((dims_m, offset, open)) = resolve_grid_box(db, &gi) else {
            return;
        };
        if !open {
            return;
        }
        let port = ci.get_str(port_field).unwrap_or("?").to_string();
        if !seen_ports.insert(port.clone()) {
            return; // a loadout entry overrides a port default — keep one per port
        }
        let label = if cls.is_empty() {
            grid_rec.name().map(short).unwrap_or_else(|| "?".into())
        } else {
            cls.to_string()
        };
        mounts.push(GridMount {
            port,
            entity_class: label,
            dims_m,
            offset,
        });
    });
    println!(
        "grid mount-sites seen: {n_mount_sites}; open-container grids: {}",
        mounts.len()
    );
    if mounts.is_empty() {
        println!("(no cargo grids resolved from the loadout on this variant)");
        return Ok(());
    }

    // ── 4. read the hull CGA from the p4k and parse its NMC scene graph ───────
    let bytes = read_p4k(&assets, &hull_cga)?;
    println!("hull CGA: {} bytes", bytes.len());
    let nodes = nmc::parse_nmc_full(&bytes).ok_or("failed to parse NMC_Full from hull CGA")?;
    println!("NMC nodes: {}", nodes.len());

    // node name (lowercased) → world translation
    let mut node_pos: BTreeMap<String, [f32; 3]> = BTreeMap::new();
    for n in &nodes {
        node_pos.insert(n.name.to_lowercase(), n.world_translation());
    }

    // ── 5. join and report ────────────────────────────────────────────────────
    println!(
        "\n{:<34} {:<32} {:>10} {:>5}  {:<24} offset",
        "port", "grid entity", "dims(m)", "SCU", "world pos (m)"
    );
    println!("{}", "-".repeat(130));
    let mut total_scu = 0i64;
    let mut placed = 0;
    for m in &mounts {
        let cells = [
            (m.dims_m[0] / CELL_M).round() as i64,
            (m.dims_m[1] / CELL_M).round() as i64,
            (m.dims_m[2] / CELL_M).round() as i64,
        ];
        let scu = cells[0].max(0) * cells[1].max(0) * cells[2].max(0);
        total_scu += scu;
        let dims = format!("{:.2}x{:.2}x{:.2}", m.dims_m[0], m.dims_m[1], m.dims_m[2]);
        let pos = node_pos.get(&m.port.to_lowercase());
        let pos_s = match pos {
            Some(p) => {
                placed += 1;
                format!("({:+.2}, {:+.2}, {:+.2})", p[0], p[1], p[2])
            }
            None => "<node not found>".into(),
        };
        let off = format!(
            "({:+.2},{:+.2},{:+.2})",
            m.offset[0], m.offset[1], m.offset[2]
        );
        println!(
            "{:<34} {:<32} {:>10} {:>5}  {:<24} {}",
            m.port,
            short(&m.entity_class),
            dims,
            scu,
            pos_s,
            off
        );
    }
    println!("{}", "-".repeat(130));
    println!(
        "total: {total_scu} SCU across {} grids; {placed}/{} placed from geometry",
        mounts.len(),
        mounts.len()
    );

    // Honesty check: cargo-grid helper nodes present in the GEOMETRY but with no
    // DCB mount traversed. These show the geometry side is complete even where the
    // DCB loadout enumeration (variant inheritance / removable cargo modules /
    // nested childItems) doesn't reach — the split this example is about.
    let unmounted: Vec<_> = node_pos
        .keys()
        .filter(|n| {
            n.contains("cargogrid") && !seen_ports.iter().any(|p| p.eq_ignore_ascii_case(n))
        })
        .collect();
    if !unmounted.is_empty() {
        println!(
            "\ncargo-grid ports in geometry with no DCB mount traversed ({}):",
            unmounted.len()
        );
        for n in unmounted {
            let p = node_pos[n];
            println!("  {n:<34} ({:+.2}, {:+.2}, {:+.2})", p[0], p[1], p[2]);
        }
        println!("  (grid box for these is DCB-resolvable via the InventoryContainer census; the");
        println!(
            "   loadout link is the frontier — see docs/ship-cargo-grids.md §Tier B coverage)"
        );
    }

    Ok(())
}

/// Find an `EntityClassDefinition` by its short name (segment after the last `.`).
fn find_entity_by_short_name<'a>(
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

/// grid entity instance → (interiorDimensions, gridPosOffset, is_open_grid) via its
/// `SCItemInventoryContainerComponentParams.containerParams` → `InventoryContainer`.
/// `is_open_grid` is true when `inventoryType` is an *open* container (a real
/// cargo grid) rather than a closed personal-inventory container.
fn resolve_grid_box(
    db: &DataCoreDatabase,
    grid_inst: &Instance,
) -> Option<([f32; 3], [f32; 3], bool)> {
    let icc = find_component(db, grid_inst, "SCItemInventoryContainerComponentParams")?;
    let container_guid = icc
        .get("containerParams")
        .and_then(|v| v.as_record_ref())
        .map(|r| r.guid)?;
    let container = db.record(&container_guid)?.as_instance();
    let dims = container
        .get_instance("interiorDimensions")
        .map(vec3)
        .unwrap_or([0.0; 3]);
    let inv_type = container.get_instance("inventoryType");
    let open = inv_type
        .as_ref()
        .and_then(|it| it.type_name())
        .map(|t| t.starts_with("InventoryOpen"))
        .unwrap_or(false);
    let offset = inv_type
        .and_then(|it| it.get_instance("gridPosOffset"))
        .map(vec3)
        .unwrap_or([0.0; 3]);
    Some((dims, offset, open))
}

fn vec3(v: Instance) -> [f32; 3] {
    [
        v.get_f32("x").unwrap_or(0.0),
        v.get_f32("y").unwrap_or(0.0),
        v.get_f32("z").unwrap_or(0.0),
    ]
}

fn short(name: &str) -> String {
    name.rsplit('.').next().unwrap_or(name).to_string()
}

/// Read a geometry file from the p4k, trying the DCB path both bare and with the
/// `Data\` prefix (DCB geometry paths omit it; p4k entries carry it).
fn read_p4k(assets: &AssetSource, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if let Some(b) = assets.try_read(path)? {
        return Ok(b);
    }
    let prefixed = format!("Data\\{path}");
    if let Some(b) = assets.try_read(&prefixed)? {
        return Ok(b);
    }
    Err(format!("not found in p4k: {path} (also tried {prefixed})").into())
}

// ── generic component / tree helpers (raw db layer) ──────────────────────────

fn find_component<'a>(
    db: &'a DataCoreDatabase,
    entity: &Instance<'a>,
    type_name: &str,
) -> Option<Instance<'a>> {
    for p in entity.properties() {
        if let Value::Array(_) = p.value
            && let Some(arr) = entity.get_array(p.name)
        {
            for elem in arr {
                if let Some(ci) = value_to_instance(db, &elem)
                    && ci.type_name() == Some(type_name)
                {
                    return Some(ci);
                }
            }
        }
    }
    None
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

// ── NMC (NodeMeshCombos) reader — ported from StarBreaker (MIT) ───────────────
//
// Reads the IVO chunk table, finds the NMC_Full chunk (0x70697FDA), and decodes
// the per-node scene graph. IMPORTANT: the chunk's `bone_to_world` matrix is
// local-to-parent despite the name — the true world transform composes up the
// `parent` chain (`world[i] = world[parent] ∘ local[i]`). A flat read happens
// to work on simple hulls (Freelancer, Cutlass) and silently breaks on modular
// ones (Ironclad). `parse_nmc_full` returns nodes with composed world matrices.
mod nmc {
    pub const NMC_FULL: u32 = 0x70697FDA;
    const IVO_MAGIC: u32 = 0x6F766923; // "#ivo"

    pub struct NmcNode {
        pub name: String,
        /// Hierarchy-composed world transform (3x4 row-major).
        pub bone_to_world: [[f32; 4]; 3],
    }

    impl NmcNode {
        /// World-space translation (metres) = 4th column of the 3x4 matrix.
        pub fn world_translation(&self) -> [f32; 3] {
            [
                self.bone_to_world[0][3],
                self.bone_to_world[1][3],
                self.bone_to_world[2][3],
            ]
        }
    }

    fn mat_mul(a: &[[f32; 4]; 3], b: &[[f32; 4]; 3]) -> [[f32; 4]; 3] {
        let mut o = [[0.0f32; 4]; 3];
        for r in 0..3 {
            for c in 0..3 {
                o[r][c] = a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c];
            }
            o[r][3] = a[r][0] * b[0][3] + a[r][1] * b[1][3] + a[r][2] * b[2][3] + a[r][3];
        }
        o
    }

    fn compose_worlds(locals: &[[[f32; 4]; 3]], parents: &[Option<u16>]) -> Vec<[[f32; 4]; 3]> {
        fn resolve(
            i: usize,
            locals: &[[[f32; 4]; 3]],
            parents: &[Option<u16>],
            world: &mut [Option<[[f32; 4]; 3]>],
        ) -> [[f32; 4]; 3] {
            if let Some(w) = world[i] {
                return w;
            }
            let w = match parents[i] {
                Some(pi) if (pi as usize) < locals.len() && (pi as usize) != i => {
                    let pw = resolve(pi as usize, locals, parents, world);
                    mat_mul(&pw, &locals[i])
                }
                _ => locals[i],
            };
            world[i] = Some(w);
            w
        }
        let mut world = vec![None; locals.len()];
        (0..locals.len())
            .map(|i| resolve(i, locals, parents, &mut world))
            .collect()
    }

    struct Rdr<'a> {
        d: &'a [u8],
        p: usize,
    }
    impl<'a> Rdr<'a> {
        fn new(d: &'a [u8]) -> Self {
            Self { d, p: 0 }
        }
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
        fn u64(&mut self) -> Option<u64> {
            let b = self.d.get(self.p..self.p + 8)?;
            self.p += 8;
            Some(u64::from_le_bytes(b.try_into().ok()?))
        }
        fn f32(&mut self) -> Option<f32> {
            Some(f32::from_bits(self.u32()?))
        }
        fn advance(&mut self, n: usize) -> Option<()> {
            self.p = self.p.checked_add(n).filter(|&e| e <= self.d.len())?;
            Some(())
        }
        fn bytes(&mut self, n: usize) -> Option<&'a [u8]> {
            let b = self.d.get(self.p..self.p + n)?;
            self.p += n;
            Some(b)
        }
        fn remaining(&self) -> usize {
            self.d.len().saturating_sub(self.p)
        }
    }

    /// Find the NMC_Full chunk in an IVO file and decode its node list.
    pub fn parse_nmc_full(file: &[u8]) -> Option<Vec<NmcNode>> {
        let mut h = Rdr::new(file);
        if h.u32()? != IVO_MAGIC {
            return None; // legacy CrCh geometry uses Node/Helper chunks instead
        }
        let _version = h.u32()?;
        let chunk_count = h.u32()? as usize;
        let _table_off = h.u32()?;
        // chunk table entry: type:u32 version:u32 offset:u64
        let mut nmc_off = None;
        for _ in 0..chunk_count {
            let ty = h.u32()?;
            let _v = h.u32()?;
            let off = h.u64()?;
            if ty == NMC_FULL {
                nmc_off = Some(off as usize);
            }
        }
        let nmc = &file[nmc_off?..];
        let mut r = Rdr::new(nmc);

        // header: 8 x i32; h[1] = total node count
        let _h0 = r.i32()?;
        let num_nodes = r.i32()? as usize;
        let _num_geom = r.i32()?;
        let num_unk = r.i32()? as usize;
        let num_mat = r.i32()? as usize;
        let str_table_size = r.i32()? as usize;
        let _h6 = r.i32()?;
        let _h7 = r.i32()?;

        // per-node entry: 208 bytes
        let mut mats = Vec::with_capacity(num_nodes);
        let mut parents = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            r.advance(32)?; // pre-matrix metadata
            r.advance(48)?; // WorldToBone (unused)
            let mut b2w = [[0.0f32; 4]; 3];
            for row in &mut b2w {
                for v in row.iter_mut() {
                    *v = r.f32()?;
                }
            }
            r.advance(12)?; // scale
            r.advance(8)?; // id + unknown
            let parent = r.u16()?;
            let _geom_type = r.u16()?;
            r.advance(56)?; // bbox(24) + remaining(32)
            mats.push(b2w);
            parents.push(if parent == 0xFFFF { None } else { Some(parent) });
        }

        // footer: 32-byte header, then unk + mat indices
        r.advance(32)?;
        for _ in 0..num_unk {
            r.u16()?;
        }
        for _ in 0..num_mat {
            r.u16()?;
        }

        // string table: num_nodes null-separated names (first line = name)
        let str_bytes = r.bytes(str_table_size)?;
        let mut names = Vec::with_capacity(num_nodes);
        let mut pos = 0;
        for _ in 0..num_nodes {
            if pos >= str_bytes.len() {
                names.push(String::new());
                continue;
            }
            let end = str_bytes[pos..]
                .iter()
                .position(|&b| b == 0)
                .map(|q| pos + q)
                .unwrap_or(str_bytes.len());
            let full = std::str::from_utf8(&str_bytes[pos..end]).unwrap_or("");
            names.push(full.lines().next().unwrap_or("").to_string());
            pos = end + 1;
        }
        let _ = r.remaining(); // second (metadata) string table — unused here

        let worlds = compose_worlds(&mats, &parents);
        Some(
            (0..num_nodes)
                .map(|i| NmcNode {
                    name: names.get(i).cloned().unwrap_or_default(),
                    bone_to_world: worlds[i],
                })
                .collect(),
        )
    }
}
