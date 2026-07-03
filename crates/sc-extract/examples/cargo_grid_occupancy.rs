//! Tier C (approximation): per-face wall-vs-open occupancy for each cargo grid,
//! computed from box-level geometry — no triangle mesh decode.
//!
//! The idea: bring the interior **walls** into the same frame as the **grids**.
//!   - Grids are placed at hull-CGA NMC hardpoint nodes (Tier B).
//!   - Each interior socpak is mounted at an NMC node (`boneName`) plus an
//!     `Offset`, listed in `VehicleComponentParams.objectContainers[]`. So a
//!     section's world transform = `nmc[boneName] ∘ Offset`, and its geometry
//!     (its ObjectContainer AABB, in section-local space) transforms into the
//!     hull frame the grids already live in.
//!
//! With grids and section envelopes in one frame we can answer, per grid face,
//! "is this face flush with the section shell (a hull wall) or facing interior
//! open space?" — a coarse but real occupancy signal. Triangle-exact occupancy
//! (raytest vs the collision mesh) is the next tier; this validates the frame
//! reconciliation that any exact version also needs.
//!
//! ```bash
//! cargo run -p sc-extract --release --example cargo_grid_occupancy
//! cargo run -p sc-extract --release --example cargo_grid_occupancy -- DRAK_Cutlass_Black
//! ```
//!
//! See `docs/ship-cargo-grids.md` (§Geometry read — Tier C).

use std::collections::BTreeMap;

use sc_extract::object_container;
use sc_extract::{AssetConfig, AssetData, AssetSource};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

const CELL_M: f32 = 1.25;

/// A grid placed in the hull frame: axis-aligned box [min,max] (metres).
struct Grid {
    port: String,
    min: [f32; 3],
    max: [f32; 3],
}

/// An interior section's world-space envelope (from its ObjectContainer bounds).
struct Section {
    name: String,
    min: [f32; 3],
    max: [f32; 3],
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

    let Some(rec) = shortest_entity(db, &ship) else {
        eprintln!("no EntityClassDefinition named '{ship}'");
        return Ok(());
    };
    let inst = rec.as_instance();
    println!("entity: {}", rec.name().unwrap_or("?"));

    let Some(hull) = first_cga(db, &inst) else {
        eprintln!("no hull .cga");
        return Ok(());
    };
    let hull_bytes = read_p4k(&assets, &hull)?;
    let nodes = nmc::parse(&hull_bytes).ok_or("NMC parse failed")?;
    let node_by_name: BTreeMap<String, nmc::Node> = nodes
        .into_iter()
        .map(|n| (n.name.to_lowercase(), n))
        .collect();
    println!("hull: {hull}  ({} NMC nodes)", node_by_name.len());

    // ── grids in hull frame (Tier B) ──────────────────────────────────────────
    let grids = collect_grids(db, &inst, &node_by_name);
    println!("grids placed: {}", grids.len());

    // ── interior sections, positioned via NMC boneName ∘ Offset ───────────────
    let refs = object_container_refs(db, &inst);
    println!("object-container refs in DCB: {}", refs.len());
    let mut sections: Vec<Section> = Vec::new();
    let mut bone_hits = 0;
    for r in &refs {
        // Section world transform: bone node (if any) composed with the offset.
        let bone_m = r
            .bone_name
            .as_ref()
            .and_then(|b| node_by_name.get(&b.to_lowercase()))
            .map(|n| {
                bone_hits += 1;
                n.bone_to_world
            })
            .unwrap_or(IDENTITY_3X4);
        let world = compose_offset(bone_m, r.offset);

        // Read the socpak's ObjectContainer bounds (section-local AABB).
        match read_section_bounds(&assets, &r.file_name) {
            Some((mn, mx)) => {
                let (wmn, wmx) = transform_aabb(world, mn, mx);
                println!(
                    "  {:<24} bone={:<22} → world XYZ[{:+.1},{:+.1},{:+.1}]..[{:+.1},{:+.1},{:+.1}]",
                    file_stem(&r.file_name),
                    r.bone_name.as_deref().unwrap_or("-"),
                    wmn[0],
                    wmn[1],
                    wmn[2],
                    wmx[0],
                    wmx[1],
                    wmx[2]
                );
                sections.push(Section {
                    name: file_stem(&r.file_name),
                    min: wmn,
                    max: wmx,
                });
            }
            None => println!(
                "  {:<24} bone={:<22} — no ObjectContainer bounds",
                file_stem(&r.file_name),
                r.bone_name.as_deref().unwrap_or("-")
            ),
        }
    }
    println!(
        "sections with bounds: {} (of {} refs; {bone_hits} bone-anchored)",
        sections.len(),
        refs.len()
    );

    if grids.is_empty() || sections.is_empty() {
        println!("\n(insufficient geometry to compute occupancy for this ship)");
        return Ok(());
    }

    // Prefer INTERIOR sections (the cargo-bay walls) over the exterior shell:
    // drop `*_ext_*` when interior sections exist, so the tightest host is a real
    // interior envelope.
    let has_interior = sections
        .iter()
        .any(|s| s.name.to_lowercase().contains("_int_"));
    if has_interior {
        sections.retain(|s| s.name.to_lowercase().contains("_int_"));
    }

    // ── frame-reconciliation sanity check ─────────────────────────────────────
    // Host each grid in the tightest (smallest-volume) section that contains its
    // centre; fall back to the nearest section centre if none strictly contains
    // it. Containment rate is the load-bearing validation for the whole approach.
    println!("\n== frame check: grid → host interior section ==");
    let mut contained = 0;
    for g in &grids {
        let c = centre(g.min, g.max);
        match host_section(c, &sections) {
            Host::Inside(i) => {
                contained += 1;
                println!(
                    "  {:<34} centre ({:+.1},{:+.1},{:+.1}) ∈ {}",
                    g.port, c[0], c[1], c[2], sections[i].name
                );
            }
            Host::Nearest(i, d) => {
                println!(
                    "  {:<34} centre ({:+.1},{:+.1},{:+.1}) ≈ {} (+{:.1}m outside)",
                    g.port, c[0], c[1], c[2], sections[i].name, d
                );
            }
            Host::None => println!(
                "  {:<34} centre ({:+.1},{:+.1},{:+.1}) — no section",
                g.port, c[0], c[1], c[2]
            ),
        }
    }
    println!(
        "  {contained}/{} grids strictly inside an interior section",
        grids.len()
    );

    // ── occupancy via grid TOPOLOGY (uses only the trusted grid boxes) ────────
    // The section-AABB approach proved too coarse (deck envelope ≫ bay) and the
    // NMC frame reconciliation is unreliable on modular hulls. This method needs
    // neither: it classifies each face from the grids alone.
    //   • down (−Z) is always WALL — grids rest on the deck.
    //   • up   (+Z) is WALL unless a grid is stacked directly above.
    //   • a horizontal face is OPEN if another grid lies across it within a
    //     walkway gap (≤ 2 cells) and overlaps in the perpendicular axes —
    //     i.e. cargo/walkway continues; otherwise WALL (perimeter/bulkhead).
    // Limits: "no neighbour ⇒ WALL" can't tell a hull wall from an open bay you
    // just walk into, and it can't see doors. But it matches the load-bearing
    // question for a planner: which faces can cargo extend/continue across.
    println!("\n== per-face occupancy (grid topology: WALL vs open-to-grid/walkway) ==");
    const FACES: [(&str, usize, i32); 6] = [
        ("-X", 0, -1),
        ("+X", 0, 1),
        ("-Y", 1, -1),
        ("+Y", 1, 1),
        ("-Z", 2, -1),
        ("+Z", 2, 1),
    ];
    const GAP: f32 = 2.0 * CELL_M; // walkway/neighbour reach: 2 SCU
    for (gi, g) in grids.iter().enumerate() {
        let cells = dims_to_cells(g);
        print!(
            "  {:<34} {}x{}x{} cells  ",
            g.port, cells[0], cells[1], cells[2]
        );
        let mut out = Vec::new();
        for (name, axis, dir) in FACES {
            let open = if axis == 2 && dir < 0 {
                false // floor: always wall
            } else {
                grids
                    .iter()
                    .enumerate()
                    .any(|(hj, h)| hj != gi && abuts(g, h, axis, dir, GAP))
            };
            out.push(format!("{name}:{}", if open { "open" } else { "WALL" }));
        }
        println!("{}", out.join("  "));
    }

    println!("\nNOTE: grid-topology heuristic — no wall geometry. 'open' = another grid");
    println!("lies across that face within a walkway gap; 'WALL' = no neighbour (hull,");
    println!("bulkhead, or an unmodelled open bay). Exact walls still need the collision");
    println!("mesh (Tier C-exact). Placement/SCU (Tier B) is exact; this reads only that.");

    Ok(())
}

// ── grids (Tier B, condensed) ────────────────────────────────────────────────

fn collect_grids(
    db: &DataCoreDatabase,
    ship: &Instance,
    node_by_name: &BTreeMap<String, nmc::Node>,
) -> Vec<Grid> {
    let mut grids = Vec::new();
    let mut seen = std::collections::HashSet::new();
    walk(db, ship, 22, &mut 3_000_000, &mut |ci| {
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
        let Some(grec) = ci
            .get(ref_f)
            .and_then(|v| v.as_record_ref())
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
        let Some((dims, off, open)) = grid_box(db, &grec.as_instance()) else {
            return;
        };
        if !open {
            return;
        }
        let port = ci.get_str(port_f).unwrap_or("?").to_string();
        if !seen.insert(port.clone()) {
            return;
        }
        let Some(node) = node_by_name.get(&port.to_lowercase()) else {
            return;
        };
        let wp = node.translation();
        // Grid box centred on the port node, offset by gridPosOffset, half-dims.
        let c = [wp[0] + off[0], wp[1] + off[1], wp[2] + off[2]];
        let h = [dims[0] / 2.0, dims[1] / 2.0, dims[2] / 2.0];
        let _ = cls;
        grids.push(Grid {
            port,
            min: [c[0] - h[0], c[1] - h[1], c[2] - h[2]],
            max: [c[0] + h[0], c[1] + h[1], c[2] + h[2]],
        });
    });
    grids
}

/// Does grid `h` lie across grid `g`'s face (`axis`,`dir`) within `gap`, and
/// overlap it in the other two axes? (i.e. cargo/walkway continues that way.)
fn abuts(g: &Grid, h: &Grid, axis: usize, dir: i32, gap: f32) -> bool {
    let d = if dir > 0 {
        h.min[axis] - g.max[axis]
    } else {
        g.min[axis] - h.max[axis]
    };
    if !(-0.5..=gap).contains(&d) {
        return false;
    }
    for p in 0..3 {
        if p == axis {
            continue;
        }
        let lo = g.min[p].max(h.min[p]);
        let hi = g.max[p].min(h.max[p]);
        if hi - lo < 0.5 {
            return false;
        }
    }
    true
}

fn dims_to_cells(g: &Grid) -> [i64; 3] {
    [
        ((g.max[0] - g.min[0]) / CELL_M).round() as i64,
        ((g.max[1] - g.min[1]) / CELL_M).round() as i64,
        ((g.max[2] - g.min[2]) / CELL_M).round() as i64,
    ]
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

// ── object-container refs from the DCB ────────────────────────────────────────

struct OcRef {
    file_name: String,
    bone_name: Option<String>,
    offset: [f32; 3],
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
        // Offset is a `QuatT` { Position: Vec3, Rotation: Quat }. Read the
        // translation (capitalised `Position` — the actual field). Rotation is
        // treated as identity here (interior sections are axis-aligned; add the
        // quaternion if a rotated section shows up wrong).
        let offset = ci
            .get_instance("Offset")
            .and_then(|o| o.get_instance("Position"))
            .map(vec3)
            .unwrap_or([0.0; 3]);
        out.push(OcRef {
            file_name: file.to_string(),
            bone_name: bone,
            offset,
        });
    });
    out
}

/// Read an interior socpak's ObjectContainer min/max bounds (section-local AABB).
fn read_section_bounds(assets: &AssetSource, socpak_path: &str) -> Option<([f32; 3], [f32; 3])> {
    let bytes = read_p4k(assets, socpak_path).ok()?;
    let mut pak = object_container::Socpak::open(bytes).ok()?;
    // Scan for the ObjectContainer .xml that actually carries min/maxBounds. A
    // socpak also holds VFX sub-containers (`*_vfx_*.xml`) which are
    // ObjectContainers WITHOUT bounds — keep scanning past those.
    let n = pak.len();
    for i in 0..n {
        let Some(name) = pak.name(i) else { continue };
        let lname = name.to_lowercase();
        if !lname.ends_with(".xml") || lname.ends_with("_editor.xml") {
            continue;
        }
        let Ok(data) = pak.read(i) else { continue };
        let Ok(Some(node)) = object_container::decode(&data) else {
            continue;
        };
        if node.tag != "ObjectContainer" {
            continue;
        }
        if let (Some(mn), Some(mx)) = (
            node.attr("minBounds").and_then(parse_vec3),
            node.attr("maxBounds").and_then(parse_vec3),
        ) {
            return Some((mn, mx));
        }
    }
    None
}

// ── small vector / matrix helpers (3x4 row-major, like NMC) ──────────────────

const IDENTITY_3X4: [[f32; 4]; 3] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
];

/// Compose the container's world transform from its bone (NMC helper) transform
/// and its `Offset` position, following StarBreaker's `compose_root_container_transform`:
/// `world = helper ∘ offset`, EXCEPT when the helper already duplicates the offset
/// translation — then use the offset alone (else the translation is applied twice).
/// Offset rotation is treated as identity here.
fn compose_offset(bone: [[f32; 4]; 3], offset: [f32; 3]) -> [[f32; 4]; 3] {
    let t_bone = [bone[0][3], bone[1][3], bone[2][3]];
    // Duplicate-helper: the bone world translation ≈ the offset position → the
    // offset already positions the container; don't re-apply the bone.
    let dup = (0..3).all(|a| (t_bone[a] - offset[a]).abs() < 0.05);
    if dup {
        // offset alone (identity rotation + offset translation)
        [
            [1.0, 0.0, 0.0, offset[0]],
            [0.0, 1.0, 0.0, offset[1]],
            [0.0, 0.0, 1.0, offset[2]],
        ]
    } else {
        // helper ∘ offset: R = R_bone, t = R_bone * offset + t_bone
        let mut m = bone;
        for (r, row) in m.iter_mut().enumerate() {
            row[3] = bone[r][3]
                + bone[r][0] * offset[0]
                + bone[r][1] * offset[1]
                + bone[r][2] * offset[2];
        }
        m
    }
}

fn apply(m: [[f32; 4]; 3], p: [f32; 3]) -> [f32; 3] {
    let mut o = [0.0f32; 3];
    for r in 0..3 {
        o[r] = m[r][0] * p[0] + m[r][1] * p[1] + m[r][2] * p[2] + m[r][3];
    }
    o
}

/// Transform an AABB by a 3x4 matrix and return the enclosing axis-aligned box.
fn transform_aabb(m: [[f32; 4]; 3], mn: [f32; 3], mx: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    let mut nmn = [f32::INFINITY; 3];
    let mut nmx = [f32::NEG_INFINITY; 3];
    for xi in [mn[0], mx[0]] {
        for yi in [mn[1], mx[1]] {
            for zi in [mn[2], mx[2]] {
                let p = apply(m, [xi, yi, zi]);
                for a in 0..3 {
                    nmn[a] = nmn[a].min(p[a]);
                    nmx[a] = nmx[a].max(p[a]);
                }
            }
        }
    }
    (nmn, nmx)
}

fn centre(mn: [f32; 3], mx: [f32; 3]) -> [f32; 3] {
    [
        (mn[0] + mx[0]) / 2.0,
        (mn[1] + mx[1]) / 2.0,
        (mn[2] + mx[2]) / 2.0,
    ]
}

enum Host {
    Inside(usize),
    Nearest(usize, f32),
    None,
}

/// Pick the tightest section containing `c` (smallest volume); if none contains
/// it, the section whose centre is nearest (with the outside distance).
fn host_section(c: [f32; 3], sections: &[Section]) -> Host {
    let vol = |s: &Section| (s.max[0] - s.min[0]) * (s.max[1] - s.min[1]) * (s.max[2] - s.min[2]);
    let mut best_inside: Option<(usize, f32)> = None;
    for (i, s) in sections.iter().enumerate() {
        if point_in(c, s.min, s.max, 0.5) {
            let v = vol(s);
            if best_inside.map(|(_, bv)| v < bv).unwrap_or(true) {
                best_inside = Some((i, v));
            }
        }
    }
    if let Some((i, _)) = best_inside {
        return Host::Inside(i);
    }
    let mut best_near: Option<(usize, f32)> = None;
    for (i, s) in sections.iter().enumerate() {
        let sc = centre(s.min, s.max);
        let d = ((c[0] - sc[0]).powi(2) + (c[1] - sc[1]).powi(2) + (c[2] - sc[2]).powi(2)).sqrt();
        if best_near.map(|(_, bd)| d < bd).unwrap_or(true) {
            best_near = Some((i, d));
        }
    }
    match best_near {
        Some((i, d)) => Host::Nearest(i, d),
        None => Host::None,
    }
}

fn point_in(p: [f32; 3], mn: [f32; 3], mx: [f32; 3], pad: f32) -> bool {
    (0..3).all(|a| p[a] >= mn[a] - pad && p[a] <= mx[a] + pad)
}

fn vec3(v: Instance) -> [f32; 3] {
    [
        v.get_f32("x").unwrap_or(0.0),
        v.get_f32("y").unwrap_or(0.0),
        v.get_f32("z").unwrap_or(0.0),
    ]
}

fn parse_vec3(s: &str) -> Option<[f32; 3]> {
    let mut it = s.split(',').map(|t| t.trim().parse::<f32>().ok());
    Some([it.next()??, it.next()??, it.next()??])
}

fn file_stem(path: &str) -> String {
    path.rsplit(['/', '\\'])
        .next()
        .unwrap_or(path)
        .trim_end_matches(".socpak")
        .to_string()
}

// ── shared db/geometry plumbing (mirrors Tier B) ─────────────────────────────

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

// ── NMC reader (full 3x4 transform) — ported from StarBreaker (MIT) ──────────
mod nmc {
    const NMC_FULL: u32 = 0x70697FDA;
    const IVO_MAGIC: u32 = 0x6F766923;

    #[derive(Clone)]
    pub struct Node {
        pub name: String,
        pub bone_to_world: [[f32; 4]; 3],
    }
    impl Node {
        pub fn translation(&self) -> [f32; 3] {
            [
                self.bone_to_world[0][3],
                self.bone_to_world[1][3],
                self.bone_to_world[2][3],
            ]
        }
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

        let mut mats = Vec::with_capacity(n);
        for _ in 0..n {
            r.adv(32)?;
            let mut _w2b = [[0.0f32; 4]; 3];
            for row in &mut _w2b {
                for v in row.iter_mut() {
                    *v = r.f32()?;
                }
            }
            let mut b2w = [[0.0f32; 4]; 3];
            for row in &mut b2w {
                for v in row.iter_mut() {
                    *v = r.f32()?;
                }
            }
            r.adv(12 + 8)?;
            let _parent = r.u16()?;
            let _gt = r.u16()?;
            r.adv(56)?;
            mats.push(b2w);
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
                    bone_to_world: mats[i],
                })
                .collect(),
        )
    }
}
