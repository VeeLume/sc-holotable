//! Tier C (full geometry): per-face **distance-to-wall** occupancy for each cargo
//! grid, computed from the ship's real interior wall placements.
//!
//! This is the full-geometry pipeline (StarBreaker-informed, MIT):
//!
//!   1. Hull `.cga` → NMC scene graph with **parent-chain world transforms**
//!      (`world[i] = world[parent] ∘ local[i]` — the chunk's `bone_to_world` is
//!      local-to-parent despite the name; flat reads break on modular hulls
//!      like the Ironclad).
//!   2. Grids: loadout port → NMC world node → world box (Tier B, corrected).
//!   3. Interior sections: `VehicleComponentParams.objectContainers[]`
//!      (`SVehicleObjectContainerParams`: fileName/boneName/Offset) →
//!      `section_world = nmc[boneName] ∘ offset` (duplicate-helper rule).
//!   4. Walls: each section socpak's `.soc` → CrCh `IncludedObjects` chunk
//!      (0x0010) → per-placement (cgf, 3×4 f64 local transform). Wall box =
//!      `section_world ∘ local` applied to the CGF's model AABB (header read,
//!      no triangle decode; per-path cache).
//!   5. Occupancy: for every grid face cell, the distance along the face normal
//!      to the nearest wall box overlapping that cell's footprint. 0 ⇒ wall
//!      flush at the face; ~1.25 ⇒ one-cell walkway then wall; large ⇒ open.
//!
//! Known limit: wall boxes are mesh AABBs — door holes inside a wall piece are
//! not visible (a doorway reads as wall). Everything coarser than a door
//! (margins, walkways, bay perimeter) is real signal.
//!
//! ```bash
//! cargo run -p sc-extract --release --example cargo_grid_walls
//! cargo run -p sc-extract --release --example cargo_grid_walls -- DRAK_Ironclad
//! cargo run -p sc-extract --release --example cargo_grid_walls -- CRUS_Starlifter_C2 --verbose
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};

use sc_extract::object_container::Socpak;
use sc_extract::{AssetConfig, AssetData, AssetSource};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

const CELL_M: f32 = 1.25;
/// Max distance (m) we report before calling a direction simply "open".
const MAX_PROBE_M: f32 = 7.5;

// ─────────────────────────────────────────────────────────────────────────────
// 3×4 row-major transform helpers (rotation R[r][c], translation R[r][3])
// ─────────────────────────────────────────────────────────────────────────────

type Mat34 = [[f32; 4]; 3];
type Aabb = ([f32; 3], [f32; 3]);

const MAT_IDENTITY: Mat34 = [
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

/// Quaternion (w,x,y,z) + position + scale → 3×4 (CryEngine entity `Rotate`).
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

/// CryEngine Euler (degrees, Z·Y·X order) + position → 3×4. Port of
/// StarBreaker's `build_container_transform` (their array is column-major;
/// transposed here into row-major).
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

/// Transform an AABB and return the enclosing axis-aligned box.
fn transform_aabb(m: &Mat34, mn: [f32; 3], mx: [f32; 3]) -> ([f32; 3], [f32; 3]) {
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

// ─────────────────────────────────────────────────────────────────────────────
// main
// ─────────────────────────────────────────────────────────────────────────────

struct Grid {
    port: String,
    min: [f32; 3],
    max: [f32; 3],
}

/// A wall piece in ship-world space (AABB of a placed interior mesh).
struct WallBox {
    min: [f32; 3],
    max: [f32; 3],
    #[allow(dead_code)]
    cgf: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let verbose = args.iter().any(|a| a == "--verbose");
    args.retain(|a| a != "--verbose");
    let ship = args
        .first()
        .cloned()
        .unwrap_or_else(|| "MISC_Freelancer".into());

    let install = sc_discovery::discover_primary()?;
    println!(
        "{} v{}  —  ship '{ship}'",
        install.channel,
        install.short_version()
    );
    let assets = AssetSource::from_install(&install)?;

    // Debug mode: inspect one CGF's chunk table + AABB scan, no DCB parse.
    if let Some(pos) = std::env::args().position(|a| a == "--cgf") {
        if let Some(path) = std::env::args().nth(pos + 1) {
            debug_cgf(&assets, &path)?;
        }
        return Ok(());
    }
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();

    let Some(rec) = shortest_entity(db, &ship) else {
        eprintln!("no EntityClassDefinition named '{ship}'");
        return Ok(());
    };
    let inst = rec.as_instance();
    println!("entity: {}", rec.name().unwrap_or("?"));

    // ── 1. hull NMC with hierarchy-composed world transforms ─────────────────
    let Some(hull) = first_cga(db, &inst) else {
        eprintln!("no hull .cga");
        return Ok(());
    };
    let hull_bytes = read_p4k(&assets, &hull)?;
    let nmc_nodes = nmc::parse(&hull_bytes).ok_or("NMC parse failed")?;
    let world = nmc::world_transforms(&nmc_nodes);
    let node_world: BTreeMap<String, Mat34> = nmc_nodes
        .iter()
        .zip(world.iter())
        .map(|(n, w)| (n.name.to_lowercase(), *w))
        .collect();
    println!(
        "hull: {hull}  ({} NMC nodes, hierarchy-composed)",
        node_world.len()
    );

    // ── 2. grids at corrected world nodes ─────────────────────────────────────
    let mut grids = {
        let mut ctx = GridCtx {
            db,
            assets: &assets,
            nmc_cache: HashMap::new(),
            loadout_cache: HashMap::new(),
        };
        ctx.nmc_cache
            .insert(hull.to_lowercase(), Some(node_world.clone()));
        let mut visited = HashSet::new();
        let mut grids = Vec::new();
        ctx.collect(
            &inst,
            &MAT_IDENTITY,
            std::slice::from_ref(&hull),
            "",
            0,
            &[],
            &mut visited,
            &mut grids,
        );
        grids
    };
    println!("\n== grids (world, hierarchy-corrected) ==");
    for g in &grids {
        let c = [
            (g.min[0] + g.max[0]) / 2.0,
            (g.min[1] + g.max[1]) / 2.0,
            (g.min[2] + g.max[2]) / 2.0,
        ];
        println!(
            "  {:<38} centre ({:+7.2},{:+7.2},{:+7.2})  box {:.1}x{:.1}x{:.1}",
            g.port,
            c[0],
            c[1],
            c[2],
            g.max[0] - g.min[0],
            g.max[1] - g.min[1],
            g.max[2] - g.min[2]
        );
    }

    // ── 3+4. interior sections → wall boxes + triangle soup ───────────────────
    let refs = object_container_refs(db, &inst);
    let mut walls: Vec<WallBox> = Vec::new();
    let mut aabb_cache: HashMap<String, Vec<Aabb>> = HashMap::new();
    let mut n_placements = 0usize;
    let mut no_aabb_tally: BTreeMap<String, usize> = BTreeMap::new();
    // triangle soup (world space) — only for placements near a grid
    let mut soup: Vec<[[f32; 3]; 3]> = Vec::new();
    let mut tri_cache: HashMap<String, Option<std::rc::Rc<TriMesh>>> = HashMap::new();
    // snapshot of grid bounds for the near filter (OC grids found during the
    // loop are rare; their neighborhoods resolve on the next run if needed)
    let grid_bounds: Vec<([f32; 3], [f32; 3])> = grids.iter().map(|g| (g.min, g.max)).collect();
    let near_grid = move |mn: [f32; 3], mx: [f32; 3]| -> bool {
        grid_bounds.iter().any(|(gmin, gmax)| {
            (0..3).all(|a| mn[a] <= gmax[a] + MAX_PROBE_M && mx[a] >= gmin[a] - MAX_PROBE_M)
        })
    };
    println!("\n== interior sections ==");
    for r in &refs {
        let helper = r
            .bone_name
            .as_ref()
            .and_then(|b| node_world.get(&b.to_lowercase()))
            .copied();
        let offset_m = mat_from_pos_euler_deg(r.offset_pos, r.offset_rot_deg);
        let section_world = compose_container(helper, &offset_m);

        let (placements, n_inc, n_ent, oc_grids) =
            read_section_placements(&assets, db, &r.file_name);
        // OC-placed grids (elevator platforms etc.): world = section ∘ entity
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
            println!("  [oc grid] '{}' in {}", og.label, file_stem(&r.file_name));
            grids.push(Grid {
                port: format!("oc:{}", og.label),
                min: wmin,
                max: wmax,
            });
        }
        let n_before = walls.len();
        for pl in &placements {
            n_placements += 1;
            let boxes = aabb_cache
                .entry(pl.cgf.to_lowercase())
                .or_insert_with(|| cgf_sub_boxes(&assets, &pl.cgf))
                .clone();
            if boxes.is_empty() {
                *no_aabb_tally
                    .entry(short_path(&pl.cgf).to_string())
                    .or_default() += 1;
                continue;
            }
            let world_m = mat_mul(&section_world, &pl.local);
            let mut placement_near = false;
            for (mn, mx) in boxes {
                let (wmn, wmx) = transform_aabb(&world_m, mn, mx);
                placement_near |= near_grid(wmn, wmx);
                walls.push(WallBox {
                    min: wmn,
                    max: wmx,
                    cgf: pl.cgf.clone(),
                });
            }
            // triangles for near-grid placements (skips light-glow files? no —
            // whole-mesh triangles include glow surfaces, but rays only care
            // about actual surfaces the ray crosses; glow meshes are thin
            // emissive panels, acceptable)
            if placement_near
                && let Some(tm) = tri_cache
                    .entry(pl.cgf.to_lowercase())
                    .or_insert_with(|| cgf_triangles(&assets, &pl.cgf).map(std::rc::Rc::new))
                    .clone()
            {
                for t in tm.indices.chunks_exact(3) {
                    let a = mat_apply(&world_m, tm.positions[t[0] as usize]);
                    let b = mat_apply(&world_m, tm.positions[t[1] as usize]);
                    let c = mat_apply(&world_m, tm.positions[t[2] as usize]);
                    soup.push([a, b, c]);
                }
            }
        }
        println!(
            "  {:<26} bone={:<24} off=({:+.1},{:+.1},{:+.1})  baked={n_inc} entities={n_ent} walls+={}",
            file_stem(&r.file_name),
            r.bone_name.as_deref().unwrap_or("-"),
            r.offset_pos[0],
            r.offset_pos[1],
            r.offset_pos[2],
            walls.len() - n_before,
        );
        if verbose {
            for pl in placements.iter().take(8) {
                println!(
                    "      {} @ local t=({:+.1},{:+.1},{:+.1})",
                    short_path(&pl.cgf),
                    pl.local[0][3],
                    pl.local[1][3],
                    pl.local[2][3]
                );
            }
        }
    }
    // the ship's own hull geometry: the outer boundary (bay ceilings/floors are
    // often the hull skin itself, present in no interior socpak). Identity
    // transform; keep only near-grid triangles.
    if let Some(tm) = cgf_triangles(&assets, &hull) {
        let mut kept = 0usize;
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
                kept += 1;
            }
        }
        println!("hull triangles near grids: {kept}");
    }

    println!(
        "wall boxes: {} (from {} placements; {} unique CGFs) · {} near-grid triangles",
        walls.len(),
        n_placements,
        aabb_cache.len(),
        soup.len()
    );
    let n_no_aabb: usize = no_aabb_tally.values().sum();
    if n_no_aabb > 0 {
        let mut v: Vec<_> = no_aabb_tally.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        println!("placements without a readable model AABB: {n_no_aabb} — top offenders:");
        for (p, n) in v.iter().take(10) {
            println!("    {n:>4} × {p}");
        }
    }

    // --scan <substr>: decode every interior-section .soc and print entities
    // whose Name/EntityClass matches (finds OC-placed helpers plain grep can't
    // see — CryXmlB chunks inside CrCh .soc files).
    if let Some(pos) = std::env::args().position(|a| a == "--scan")
        && let Some(pat) = std::env::args().nth(pos + 1)
    {
        let pat = pat.to_lowercase();
        println!("\n== OC entities matching '{pat}' ==");
        for r in &refs {
            let Ok(bytes) = read_p4k(&assets, &r.file_name) else {
                continue;
            };
            let Ok(mut pak) = Socpak::open(bytes) else {
                continue;
            };
            for i in 0..pak.len() {
                let Some(name) = pak.name(i) else { continue };
                if !name.to_lowercase().ends_with(".soc") {
                    continue;
                }
                let Ok(soc) = pak.read(i) else { continue };
                let Ok(Some(root)) = sc_extract::object_container::decode(&soc) else {
                    continue;
                };
                for ent in root.find_all("Entity") {
                    let n = ent.attr("Name").unwrap_or("");
                    let c = ent.attr("EntityClass").unwrap_or("");
                    if n.to_lowercase().contains(&pat) || c.to_lowercase().contains(&pat) {
                        println!(
                            "  [{}] Name='{n}' class='{c}' guid={} pos={}",
                            file_stem(&r.file_name),
                            ent.attr("EntityClassGUID").unwrap_or("-"),
                            ent.attr("Pos").unwrap_or("-"),
                        );
                    }
                }
            }
        }
    }

    // --scan-nodes <substr>: search every placed interior mesh's NMC for node
    // names matching — finds TILE item-port helpers (ports whose transform is
    // section_world ∘ placement ∘ node).
    if let Some(pos) = std::env::args().position(|a| a == "--scan-nodes")
        && let Some(pat) = std::env::args().nth(pos + 1)
    {
        let pat = pat.to_lowercase();
        println!("\n== interior-mesh NMC nodes matching '{pat}' ==");
        let mut cache: HashMap<String, Option<Vec<String>>> = HashMap::new();
        for r in &refs {
            let (placements, _, _, _) = read_section_placements(&assets, db, &r.file_name);
            let mut seen_cgf = HashSet::new();
            for pl in &placements {
                if !seen_cgf.insert(pl.cgf.to_lowercase()) {
                    continue;
                }
                let names = cache
                    .entry(pl.cgf.to_lowercase())
                    .or_insert_with(|| {
                        let bytes = read_p4k(&assets, &pl.cgf).ok()?;
                        let nodes = nmc::parse(&bytes)?;
                        Some(nodes.iter().map(|n| n.name.clone()).collect())
                    })
                    .clone();
                let Some(names) = names else { continue };
                for n in names.iter().filter(|n| n.to_lowercase().contains(&pat)) {
                    println!(
                        "  [{}] {} :: {}",
                        file_stem(&r.file_name),
                        short_path(&pl.cgf),
                        n
                    );
                }
            }
        }
    }

    // --who <entity-short-name>: reverse-reference chain — which records point
    // at this entity (2 levels). Finds the mount parent of a grid entity.
    if let Some(pos) = std::env::args().position(|a| a == "--who")
        && let Some(name) = std::env::args().nth(pos + 1)
    {
        let graph = sc_extract::ReferenceGraph::from_database(db);
        // resolve any record type by short name (entity class, container, …)
        let target = entity_by_short(db, &name).or_else(|| {
            db.all_records().find(|r| {
                let n = r.name().unwrap_or("");
                n.rsplit('.')
                    .next()
                    .unwrap_or(n)
                    .eq_ignore_ascii_case(&name)
            })
        });
        if let Some(target) = target {
            println!("\n== who references {} ==", target.name().unwrap_or("?"));
            for g1 in graph.incoming(&target.id()).iter().take(12) {
                let n1 = db
                    .record(g1)
                    .and_then(|r| r.name().map(|s| s.to_string()))
                    .unwrap_or_else(|| format!("{g1}"));
                println!("  ← {n1}");
                for g2 in graph.incoming(g1).iter().take(8) {
                    let n2 = db
                        .record(g2)
                        .and_then(|r| r.name().map(|s| s.to_string()))
                        .unwrap_or_else(|| format!("{g2}"));
                    println!("      ← {n2}");
                }
            }
        } else {
            println!("--who: no record '{name}'");
        }
    }

    if grids.is_empty() || walls.is_empty() {
        println!("\n(nothing to test — missing grids or walls)");
        return Ok(());
    }

    // --near <port-substr>: list wall boxes near that grid + boxes CONTAINING it
    // (containing boxes are the monolithic-shell candidates the distance test
    // currently rejects as "embedding").
    if let Some(pos) = std::env::args().position(|a| a == "--near")
        && let Some(pat) = std::env::args().nth(pos + 1)
    {
        let pat = pat.to_lowercase();
        for g in grids
            .iter()
            .filter(|g| g.port.to_lowercase().contains(&pat))
        {
            println!(
                "\n== boxes near grid {}  [{:+.2},{:+.2},{:+.2}]..[{:+.2},{:+.2},{:+.2}] ==",
                g.port, g.min[0], g.min[1], g.min[2], g.max[0], g.max[1], g.max[2]
            );
            let mut near: Vec<(f32, String)> = Vec::new();
            for w in &walls {
                let contains =
                    (0..3).all(|a| w.min[a] <= g.min[a] + 0.1 && w.max[a] >= g.max[a] - 0.1);
                // gap between boxes
                let mut s = 0.0f32;
                for a in 0..3 {
                    let d = (w.min[a] - g.max[a]).max(g.min[a] - w.max[a]).max(0.0);
                    s += d * d;
                }
                let gap = s.sqrt();
                if contains {
                    println!(
                        "  CONTAINS grid: [{:+7.2},{:+7.2},{:+7.2}]..[{:+7.2},{:+7.2},{:+7.2}]  {}",
                        w.min[0],
                        w.min[1],
                        w.min[2],
                        w.max[0],
                        w.max[1],
                        w.max[2],
                        short_path(&w.cgf)
                    );
                } else if gap < 2.0 {
                    near.push((
                        gap,
                        format!(
                            "gap {gap:4.2}  [{:+7.2},{:+7.2},{:+7.2}]..[{:+7.2},{:+7.2},{:+7.2}]  {}",
                            w.min[0], w.min[1], w.min[2], w.max[0], w.max[1], w.max[2],
                            short_path(&w.cgf)
                        ),
                    ));
                }
            }
            near.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for (_, l) in near.iter().take(30) {
                println!("  {l}");
            }
        }
    }

    // --find <substr>: dump world boxes for matching CGFs
    if let Some(pos) = std::env::args().position(|a| a == "--find")
        && let Some(pat) = std::env::args().nth(pos + 1)
    {
        let pat = pat.to_lowercase();
        println!("\n== wall boxes matching '{pat}' ==");
        for w in walls
            .iter()
            .filter(|w| w.cgf.to_lowercase().contains(&pat))
            .take(24)
        {
            println!(
                "  [{:+7.2},{:+7.2},{:+7.2}]..[{:+7.2},{:+7.2},{:+7.2}]  {}",
                w.min[0],
                w.min[1],
                w.min[2],
                w.max[0],
                w.max[1],
                w.max[2],
                short_path(&w.cgf)
            );
        }
    }

    // floor diagnostic: wall boxes whose XY footprint overlaps a grid and whose
    // Z-range is near the grid's bottom (embedded floors show up here).
    if verbose {
        println!("\n== floor diagnostic (boxes near each grid's bottom) ==");
        for g in &grids {
            println!("  {} bottom z={:+.2}", g.port, g.min[2]);
            let mut hits: Vec<(f32, f32, &str)> = walls
                .iter()
                .filter(|w| {
                    w.max[0] > g.min[0]
                        && w.min[0] < g.max[0]
                        && w.max[1] > g.min[1]
                        && w.min[1] < g.max[1]
                        && w.min[2] < g.min[2] + 1.0
                        && w.max[2] > g.min[2] - 3.0
                })
                .map(|w| (w.min[2], w.max[2], short_path(&w.cgf)))
                .collect();
            hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            for (zmin, zmax, cgf) in hits.iter().take(6) {
                println!("      z[{zmin:+.2},{zmax:+.2}]  {cgf}");
            }
            if hits.is_empty() {
                println!("      (none — floor genuinely missing from wall set)");
            }
        }
    }

    // ── 5. per-face distance-to-wall occupancy ────────────────────────────────
    // For each face cell: distance along the outward normal to the nearest wall
    // box that overlaps the cell's footprint. Then summarize per face.
    println!("\n== per-face distance-to-wall (m; 0=flush wall, {MAX_PROBE_M}+=open) ==");
    const FACES: [(&str, usize, i32); 6] = [
        ("-X", 0, -1),
        ("+X", 0, 1),
        ("-Y", 1, -1),
        ("+Y", 1, 1),
        ("-Z", 2, -1),
        ("+Z", 2, 1),
    ];
    for g in &grids {
        println!("  {} :", g.port);
        for (fname, axis, dir) in FACES {
            let dists = face_cell_distances(g, axis, dir, &walls, &soup);
            if dists.is_empty() {
                continue;
            }
            let mut s = dists.clone();
            s.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let min = s[0];
            let med = s[s.len() / 2];
            let max = *s.last().unwrap();
            let n_flush = dists.iter().filter(|&&d| d < 0.3).count();
            let n_open = dists.iter().filter(|&&d| d >= MAX_PROBE_M).count();
            let verdict = if n_flush * 2 >= dists.len() {
                "WALL (flush)"
            } else if n_open * 2 >= dists.len() {
                "open"
            } else if med < 1.9 {
                "walkway then wall"
            } else {
                "mixed"
            };
            println!(
                "    {fname}: cells={:<3} dist min/med/max = {:>4.2}/{:>4.2}/{:>4.2}   flush={:<3} open={:<3} → {verdict}",
                dists.len(),
                min,
                med,
                max.min(MAX_PROBE_M),
                n_flush,
                n_open
            );
        }
    }

    if soup.is_empty() {
        println!("\nNOTE: box mode (no decodable triangles) — per-node AABBs; door holes");
        println!("invisible, diagonal geometry over-covers. See docs/ship-cargo-grids.md.");
    } else {
        println!("\nNOTE: triangle mode — real mesh raycasts (render geometry incl. hull skin).");
        println!("Door/ramp apertures read open; door ITEMS' own meshes are not in the soup,");
        println!("so a closed door still reads open through its frame. Conservative per-cell");
        println!("minimum over a 5-ray bundle. See docs/ship-cargo-grids.md §Tier C.");
    }

    Ok(())
}

/// Distances from each cell of a grid face to the nearest wall in front of it.
///
/// **Triangle mode** (when a near-grid triangle soup exists): a small ray
/// bundle per cell (center + 4 spread samples) along the outward normal; the
/// cell's clearance is the MINIMUM hit distance (conservative — a box can only
/// extend as far as its most-blocked sample). Rays pass through real door
/// apertures and under sloped panels — the fidelity the AABBs can't give.
/// Triangles are pruned per face to those intersecting the face's probe slab.
///
/// **Box mode** (fallback, no triangles): near-side distance of boxes ahead of
/// the face + far-side upper bound for enclosing shells.
///
/// The bottom face (−Z) is 0 by construction: grids are bottom-anchored on the
/// deck (verified against the Ironclad floor plates).
fn face_cell_distances(
    g: &Grid,
    axis: usize,
    dir: i32,
    walls: &[WallBox],
    soup: &[[[f32; 3]; 3]],
) -> Vec<f32> {
    let face = if dir < 0 { g.min[axis] } else { g.max[axis] };
    // the two lateral axes forming the face plane
    let (u, v) = match axis {
        0 => (1, 2),
        1 => (0, 2),
        _ => (0, 1),
    };
    let nu = (((g.max[u] - g.min[u]) / CELL_M).round() as i32).max(1);
    let nv = (((g.max[v] - g.min[v]) / CELL_M).round() as i32).max(1);
    let mut out = Vec::with_capacity((nu * nv) as usize);
    if axis == 2 && dir < 0 {
        out.resize((nu * nv) as usize, 0.0); // deck by construction
        return out;
    }

    // per-face slab prune of the triangle soup
    let mut slab_min = g.min;
    let mut slab_max = g.max;
    if dir > 0 {
        slab_min[axis] = face - 0.3;
        slab_max[axis] = face + MAX_PROBE_M;
    } else {
        slab_min[axis] = face - MAX_PROBE_M;
        slab_max[axis] = face + 0.3;
    }
    let face_tris: Vec<&[[f32; 3]; 3]> = soup
        .iter()
        .filter(|t| {
            (0..3).all(|a| {
                let lo = t[0][a].min(t[1][a]).min(t[2][a]);
                let hi = t[0][a].max(t[1][a]).max(t[2][a]);
                hi >= slab_min[a] && lo <= slab_max[a]
            })
        })
        .collect();
    let use_tris = !face_tris.is_empty();

    let mut ndir = [0.0f32; 3];
    ndir[axis] = dir as f32;

    for iu in 0..nu {
        for iv in 0..nv {
            let u0 = g.min[u] + iu as f32 * CELL_M;
            let u1 = g.min[u] + (iu + 1) as f32 * CELL_M;
            let v0 = g.min[v] + iv as f32 * CELL_M;
            let v1 = g.min[v] + (iv + 1) as f32 * CELL_M;
            let best = if use_tris {
                // ray bundle: center + 4 spread samples
                let cu = (u0 + u1) / 2.0;
                let cv = (v0 + v1) / 2.0;
                let s = CELL_M * 0.3;
                let mut worst = MAX_PROBE_M;
                for (du, dv) in [(0.0, 0.0), (-s, -s), (s, -s), (-s, s), (s, s)] {
                    let mut orig = [0.0f32; 3];
                    orig[axis] = face - dir as f32 * 0.05; // nudge inside
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
                // box fallback (near-side + enclosing-shell upper bound)
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
            out.push(best);
        }
    }
    out
}

/// `section_world = helper ∘ offset`, unless the helper translation already
/// duplicates the offset (then the offset alone positions the section).
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

// ─────────────────────────────────────────────────────────────────────────────
// DCB reads (ship entity, grids, object-container refs)
// ─────────────────────────────────────────────────────────────────────────────

/// Grid-collection context: DCB + p4k + caches. Grids reach a ship four ways:
///  1. inline manual loadout entries (`SItemPortLoadoutEntryParams`),
///  2. port default items (`SItemPortDefaultItemDef`),
///  3. mounted ITEMS' own loadouts (cargo elevators/bay doors — Perseus,
///     Constellation) behind an `entityClassReference` Reference,
///  4. XML-FILE loadouts (`SItemPortLoadoutXMLParams.loadoutPath` →
///     `<Loadout><Items><Item portName itemName/>` — the Hammerhead's
///     elevator-door grid).
///
/// Port helpers may live in the item's own geometry (any of its .cga/.cgf) or
/// an ancestor's — nearest owner wins.
/// A port's explicit DCB attachment: anchor node name + offset transform
/// (`SItemPortDef.AttachmentImplementation → SItemPortDefHelper`). This is the
/// authoritative port placement — the Idris' 25 cargo-grid ports carry explicit
/// offsets anchored on the hull root; matching an NMC node by PORT NAME (the
/// earlier mechanism) is the degenerate case where the anchor is that node.
#[derive(Clone)]
struct PortAttach {
    helper: String,
    offset: Mat34,
}

struct GridCtx<'a> {
    db: &'a DataCoreDatabase,
    assets: &'a AssetSource,
    nmc_cache: HashMap<String, Option<BTreeMap<String, Mat34>>>,
    loadout_cache: HashMap<String, Vec<(String, String)>>,
}

impl<'a> GridCtx<'a> {
    /// All item mounts of one entity (DCB entries + XML-file loadout items) +
    /// the entity's port-attachment map (port → anchor/offset).
    fn mounts(
        &mut self,
        inst: &Instance<'a>,
    ) -> (
        Vec<(String, svarog_datacore::Record<'a>)>,
        HashMap<String, PortAttach>,
    ) {
        let db = self.db;
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
                    // Name + AttachmentImplementation.Helper.Helper{Name, Offset}
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
                        // only record meaningful attachments (anchor or offset)
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

    /// Parse a `<Loadout>` XML file → (portName, itemName) pairs (cached).
    fn xml_loadout(&mut self, path: &str) -> Vec<(String, String)> {
        if let Some(v) = self.loadout_cache.get(&path.to_lowercase()) {
            return v.clone();
        }
        let v = read_p4k(self.assets, path)
            .ok()
            .and_then(|b| sc_extract::object_container::decode(&b).ok().flatten())
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

    /// Does this entity (or one level of mounted items) carry an open grid?
    fn has_grid(&mut self, inst: &Instance<'a>, depth: u8) -> bool {
        let (mounts, _) = self.mounts(inst);
        for (_, rec) in mounts {
            let gi = rec.as_instance();
            if matches!(grid_box(self.db, &gi), Some((_, _, true))) {
                return true;
            }
            if depth < 1 && self.has_grid(&gi, depth + 1) {
                return true;
            }
        }
        false
    }

    /// All geometry files of an entity (deduped, capped).
    fn geometries(&self, inst: &Instance<'a>) -> Vec<String> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        walk(self.db, inst, 10, &mut 150_000, &mut |ci| {
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

    fn nmc_of(&mut self, cga: &str) -> Option<BTreeMap<String, Mat34>> {
        let assets = self.assets;
        self.nmc_cache
            .entry(cga.to_lowercase())
            .or_insert_with(|| {
                let bytes = read_p4k(assets, cga).ok()?;
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
    fn collect(
        &mut self,
        inst: &Instance<'a>,
        base: &Mat34,
        geoms: &[String],
        chain: &str,
        depth: u8,
        fallbacks: &[(BTreeMap<String, Mat34>, Mat34)],
        visited: &mut HashSet<String>,
        grids: &mut Vec<Grid>,
    ) {
        // merged node maps of this item's geometries (in order)
        let maps: Vec<BTreeMap<String, Mat34>> =
            geoms.iter().filter_map(|g| self.nmc_of(g)).collect();
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
        // full port resolution: explicit DCB attachment (anchor node ∘ offset)
        // → NMC node named after the port → item origin.
        let resolve_port = |port: &str| -> Option<(Mat34, Mat34)> {
            if let Some(pa) = attach.get(&port.to_lowercase()) {
                let (anchor, anchor_base) = if pa.helper.is_empty() {
                    (MAT_IDENTITY, *base)
                } else {
                    lookup(&pa.helper).unwrap_or((MAT_IDENTITY, *base))
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
            if let Some((dims, off, true)) = grid_box(self.db, &gi) {
                // no attachment + no node anywhere → engine default: item origin
                let (node, node_base) = resolve_port(&port).unwrap_or_else(|| {
                    if depth > 0 {
                        println!(
                            "  [~] grid at '{key}': port helper missing — placed at item origin"
                        );
                    }
                    (MAT_IDENTITY, *base)
                });
                let w = mat_mul(&node_base, &node);
                // bottom-anchored, centred X/Y (verified vs Ironclad floor plates)
                let lmin = [off[0] - dims[0] / 2.0, off[1] - dims[1] / 2.0, off[2]];
                let lmax = [
                    off[0] + dims[0] / 2.0,
                    off[1] + dims[1] / 2.0,
                    off[2] + dims[2],
                ];
                let (wmin, wmax) = transform_aabb(&w, lmin, lmax);
                grids.push(Grid {
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
                    println!("  [!] item at '{key}' has grids below but no parent node");
                    continue;
                };
                let child_base = mat_mul(&node_base, &node);
                let child_geoms = self.geometries(&gi);
                println!(
                    "  [child item] '{key}' → {} (geo {:?})",
                    rec.name()
                        .map(|n| n.rsplit('.').next().unwrap_or(n))
                        .unwrap_or("?"),
                    child_geoms
                        .iter()
                        .map(|g| short_path(g))
                        .collect::<Vec<_>>()
                );
                let mut fb = fallbacks.to_vec();
                for m in maps.clone() {
                    fb.push((m, *base));
                }
                self.collect(
                    &gi,
                    &child_base,
                    &child_geoms,
                    &key,
                    depth + 1,
                    &fb,
                    visited,
                    grids,
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
        // Offset: QuatT { Position: Vec3, Rotation: (Ang3 Euler degrees, possibly nested) }
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
                // either an Ang3 {x,y,z} directly, or a wrapper holding one
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

// ─────────────────────────────────────────────────────────────────────────────
// Interior socpak → IncludedObjects placements
// ─────────────────────────────────────────────────────────────────────────────

struct Placement {
    cgf: String,
    local: Mat34,
}

/// A cargo-grid ENTITY placed directly in an interior object container (the
/// third mounting mechanism — e.g. the Hammerhead's elevator grid, which no
/// DCB record references at all).
struct OcGrid {
    label: String,
    local: Mat34,
    dims: [f32; 3],
    off: [f32; 3],
}

/// All static-geometry placements from every `.soc` inside a section socpak:
/// the CrCh `IncludedObjects` chunk (0x0010, baked meshes) plus CryXmlB
/// `Entity` placements (Pos/Rotate/Scale + geometry path, inline or via
/// `EntityClassGUID` → DCB → `SGeometryResourceParams`).
fn read_section_placements(
    assets: &AssetSource,
    db: &DataCoreDatabase,
    socpak_path: &str,
) -> (Vec<Placement>, usize, usize, Vec<OcGrid>) {
    let Ok(bytes) = read_p4k(assets, socpak_path) else {
        return (Vec::new(), 0, 0, Vec::new());
    };
    let Ok(mut pak) = Socpak::open(bytes) else {
        return (Vec::new(), 0, 0, Vec::new());
    };
    let mut out = Vec::new();
    let mut oc_grids = Vec::new();
    let mut n_included = 0usize;
    let mut n_entities = 0usize;
    for i in 0..pak.len() {
        let Some(name) = pak.name(i) else { continue };
        if !name.to_lowercase().ends_with(".soc") {
            continue;
        }
        let Ok(soc) = pak.read(i) else { continue };
        // baked static meshes
        for (ty, data) in crch_chunks(&soc) {
            if ty == 0x0010 {
                let objs = parse_included_objects(data);
                n_included += objs.len();
                out.extend(objs);
            }
        }
        // entity placements (decode() peels every CryXmlB chunk in the .soc)
        if let Ok(Some(root)) = sc_extract::object_container::decode(&soc) {
            for ent in root.find_all("Entity") {
                // an OC-placed cargo grid?
                if let Some(g) = entity_oc_grid(db, ent) {
                    oc_grids.push(g);
                    continue;
                }
                let Some(pl) = entity_placement(db, ent) else {
                    continue;
                };
                n_entities += 1;
                out.push(pl);
            }
        }
    }
    (out, n_included, n_entities, oc_grids)
}

/// Is this OC entity a cargo grid (its class carries an open InventoryContainer)?
fn entity_oc_grid(
    db: &DataCoreDatabase,
    ent: &sc_extract::object_container::XmlNode,
) -> Option<OcGrid> {
    // class link: GUID attr, or class-name attr
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

/// Entity → geometry placement, if it has visual geometry.
fn entity_placement(
    db: &DataCoreDatabase,
    ent: &sc_extract::object_container::XmlNode,
) -> Option<Placement> {
    let pos = parse_csv3(ent.attr("Pos").unwrap_or("0,0,0"));
    let q = parse_csv4(ent.attr("Rotate").unwrap_or("1,0,0,0"));
    let scale = parse_csv3(ent.attr("Scale").unwrap_or("1,1,1"));

    // inline: PropertiesDataCore → EntityGeometryResource → Geometry×3 @path
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
    // fallback: entity class → SGeometryResourceParams geometry
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

/// First `.cgf`/`.cga` geometry path reachable on an entity class (any string field).
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

/// Walk a CrCh chunk table → (type, payload) pairs.
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

/// Parse an `IncludedObjects` chunk (StarBreaker format, MIT): CGF path table +
/// Type1 placement records with a 3×4 f64 row-major transform at +64.
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

    let mut off = 4usize; // 4-byte padding
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
    off += 28; // unknown
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
                // 3×4 f64 row-major at +64: [r00,r01,r02,tx, r10,…,ty, r20,…,tz]
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
                // skip trailing zero padding to the next record
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

// ─────────────────────────────────────────────────────────────────────────────
// CGF model AABB (header scan — no triangle decode)
// ─────────────────────────────────────────────────────────────────────────────

/// Model-space SUB-boxes of a geometry file: one per NMC scene-graph node
/// (per-node bbox at entry +152, node-local space, composed through the node's
/// world transform — verified: the union reproduces the `MeshIvo320` model
/// AABB on the Freelancer bay shell). Falls back to the single model AABB for
/// single-mesh CGFs (empty NMC) and CrCh geometry. This is what decomposes
/// monolithic shells into floor/wall/ceiling panels.
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
                // geometry nodes with a real extent (helpers carry zero boxes)…
                n.geometry_type == 0
                    && (0..3).any(|a| n.bbox_max[a] - n.bbox_min[a] > 0.01)
                    // …excluding LIGHT-GLOW geometry: `LG_*`/`LIGHT_*` nodes are
                    // emissive volumes projecting INTO the room (they end at the
                    // cargo boundary and would read as walls cutting into the
                    // grid). Art-naming heuristic — the NMC metadata carries no
                    // typed marker (checked: only DCC export notes).
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

/// Read a geometry file and recover its model-space AABB from a chunk header.
/// IVO `MeshIvo320` (0x92914444): the AABB sits at a FIXED float offset 6
/// (byte +24) — verified on the Freelancer hull and the DRAK 6×10 floor plate.
/// CrCh MESH (0x1000): try the CryEngine 0x800 layout offset (+108), else scan.
fn cgf_model_aabb(assets: &AssetSource, path: &str) -> Option<([f32; 3], [f32; 3])> {
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
            let ty = rd32(base);
            if ty == 0x92914444 {
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

/// Read a 6-float AABB at a fixed byte offset, validating it.
fn aabb_at(data: &[u8], byte_off: usize) -> Option<([f32; 3], [f32; 3])> {
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

/// Debug helper: dump a CGF's chunk table and the head of its mesh chunk.
fn debug_cgf(assets: &AssetSource, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = read_p4k(assets, path)?;
    println!(
        "{path}: {} bytes, magic {:?}",
        bytes.len(),
        &bytes[..4.min(bytes.len())]
    );
    if &bytes[..4] == b"#ivo" {
        let rd32 = |p: usize| u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap());
        let count = rd32(8) as usize;
        let table = rd32(12) as usize;
        for i in 0..count {
            let base = table + i * 16;
            let ty = rd32(base);
            let off = u64::from_le_bytes(bytes[base + 8..base + 16].try_into().unwrap()) as usize;
            println!("  ivo chunk 0x{ty:08X} @ {off}");
            if ty == 0x92914444 {
                dump_floats(&bytes[off..bytes.len().min(off + 96)]);
            }
        }
    } else if &bytes[..4] == b"CrCh" {
        for (ty, data) in crch_chunks(&bytes) {
            println!("  crch chunk 0x{ty:04X}  {} bytes", data.len());
            if ty == 0x1000 {
                dump_floats(&data[..data.len().min(96)]);
            }
        }
    }
    println!("scan_aabb → {:?}", cgf_model_aabb(assets, path));
    if let Some(nodes) = nmc::parse(&bytes) {
        let worlds = nmc::world_transforms(&nodes);
        println!(
            "NMC nodes ({}):  name | geom_type | RAW bbox | node-world-transformed bbox",
            nodes.len()
        );
        for (n, w) in nodes.iter().zip(worlds.iter()).take(60) {
            let (tmn, tmx) = transform_aabb(w, n.bbox_min, n.bbox_max);
            println!(
                "    {:<40} gt={:<2} xf[{:+6.2},{:+6.2},{:+6.2}]..[{:+6.2},{:+6.2},{:+6.2}]  meta: {}",
                n.name, n.geometry_type, tmn[0], tmn[1], tmn[2], tmx[0], tmx[1], tmx[2], n.meta,
            );
        }
    } else {
        println!("(no NMC — CrCh geometry or parse failure)");
    }
    Ok(())
}

fn dump_floats(head: &[u8]) {
    let n = head.len() / 4;
    let mut fs = Vec::new();
    let mut is = Vec::new();
    for i in 0..n {
        let u = u32::from_le_bytes(head[i * 4..i * 4 + 4].try_into().unwrap());
        is.push(u);
        let f = f32::from_bits(u);
        fs.push(if f.is_finite() && f.abs() < 1e5 {
            format!("{f:.2}")
        } else {
            "·".into()
        });
    }
    println!("    head u32: {:?}", &is[..n.min(12)]);
    println!("    head f32: [{}]", fs.join(", "));
}

// ─────────────────────────────────────────────────────────────────────────────
// Triangle decode (IVO `IvoSkin2` streams — StarBreaker format, MIT)
// ─────────────────────────────────────────────────────────────────────────────

/// A decoded triangle mesh in model space.
struct TriMesh {
    positions: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

/// Decode a geometry file's triangles. The mesh streams live in the companion
/// file (`x.cgf` → `x.cgfm`, `x.cga` → `x.cgam`) — an IVO file whose
/// `IvoSkin2` chunk (0xB8757777) holds:
///   flags:u32 + MeshInfo(76B: counts + model bbox + scaling bbox) + pad(88)
///   + submeshes(48B each: …, first_index, num_indices, first_vertex,
///     page_base, …) + extra_words + tagged streams (tag:u32 elem:u32 data,
///     8-aligned): IVOVERTSUVS elem16 = SNorm u16×3 positions, elem20 = f32×3;
///     IVOINDICES elem2 = u16 (+ per-submesh page_base), elem4 = u32.
///
/// Positions dequantize: (i16/32767) × max(half_extent,1) + center over the
/// SCALING bbox (validated against the DRAK floor plate's model AABB).
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
    let mut skin: Option<(usize, usize)> = None; // (off, end)
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

    // header
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
        // MeshInfo: flags2@4 … model bbox @24..48, scaling bbox @48..72
        scale_min[a] = gf(4 + 44 + a * 4)?;
        scale_max[a] = gf(4 + 56 + a * 4)?;
    }
    let extra_count = g32(4 + 72)? as usize;
    if num_vertices == 0 || num_indices == 0 || num_vertices > 4_000_000 {
        return None;
    }

    // submeshes at 168, 48 B each — need first_index/num_indices/page_base
    let sub_off = 168;
    let mut subs: Vec<(u32, u32, u32)> = Vec::with_capacity(num_submeshes);
    for i in 0..num_submeshes {
        let b = sub_off + i * 48;
        subs.push((g32(b + 4)?, g32(b + 8)?, g32(b + 16)?)); // first_index, num, page_base
    }
    let mut p = sub_off + num_submeshes * 48 + extra_count * 4;

    // streams
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
                    // per-submesh page_base for >64k-vertex meshes
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

/// Möller–Trumbore ray/triangle: distance along `dir` (unit) from `orig`, if hit.
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
    let u = (tv[0] * pv[0] + tv[1] * pv[1] + tv[2] * pv[2]) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let qv = [
        tv[1] * e1[2] - tv[2] * e1[1],
        tv[2] * e1[0] - tv[0] * e1[2],
        tv[0] * e1[1] - tv[1] * e1[0],
    ];
    let v = (dir[0] * qv[0] + dir[1] * qv[1] + dir[2] * qv[2]) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    Some((e2[0] * qv[0] + e2[1] * qv[1] + e2[2] * qv[2]) * inv)
}

/// Scan a header slice for the first 6-float window that reads as a sane AABB.
fn scan_aabb(head: &[u8]) -> Option<([f32; 3], [f32; 3])> {
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

// ─────────────────────────────────────────────────────────────────────────────
// shared db plumbing
// ─────────────────────────────────────────────────────────────────────────────

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

fn vec3(v: Instance) -> [f32; 3] {
    [
        v.get_f32("x").unwrap_or(0.0),
        v.get_f32("y").unwrap_or(0.0),
        v.get_f32("z").unwrap_or(0.0),
    ]
}

fn file_stem(path: &str) -> String {
    path.rsplit(['/', '\\'])
        .next()
        .unwrap_or(path)
        .trim_end_matches(".socpak")
        .to_string()
}

fn short_path(path: &str) -> &str {
    path.rsplit(['/', '\\']).next().unwrap_or(path)
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

// ─────────────────────────────────────────────────────────────────────────────
// NMC reader with parent hierarchy (StarBreaker port, MIT)
// ─────────────────────────────────────────────────────────────────────────────

mod nmc {
    use super::Mat34;

    const NMC_FULL: u32 = 0x70697FDA;
    const IVO_MAGIC: u32 = 0x6F766923;

    pub struct Node {
        pub name: String,
        /// Local-to-parent 3×4 (the chunk calls it `bone_to_world`, but it is
        /// hierarchical — compose up `parent` for the true world transform).
        pub local: Mat34,
        pub parent: Option<u16>,
        /// Per-node bounding box from the entry (+152..+175). Space verified
        /// empirically (see `cgf_sub_boxes`).
        pub bbox_min: [f32; 3],
        pub bbox_max: [f32; 3],
        /// Node type at +150 (0 = geometry, 3 = helper, …).
        pub geometry_type: u16,
        /// Raw `key=value` metadata from the NMC's second string table
        /// (e.g. `class=Light`).
        pub meta: String,
    }

    /// Compose world transforms up the parent chain:
    /// `world[i] = world[parent] ∘ local[i]` (root: local).
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
            r.adv(32)?; // pre-matrix metadata
            r.adv(48)?; // WorldToBone (unused)
            let mut local = [[0.0f32; 4]; 3];
            for row in &mut local {
                for v in row.iter_mut() {
                    *v = r.f32()?;
                }
            }
            r.adv(12 + 8)?; // scale + id/unknown
            let parent = r.u16()?;
            let geom_type = r.u16()?;
            let mut bb = [0.0f32; 6];
            for v in bb.iter_mut() {
                *v = r.f32()?;
            }
            r.adv(32)?; // tail metadata
            locals.push(local);
            parents.push(if parent == 0xFFFF { None } else { Some(parent) });
            bboxes.push(bb);
            geom_types.push(geom_type);
        }
        r.adv(32)?; // footer header
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
        // second string table: per-node key=value metadata (e.g. `class=Light`)
        let meta_len = r.d.len().saturating_sub(r.p);
        let mb = r.bytes(meta_len).unwrap_or(&[]);
        let mut metas = Vec::with_capacity(n);
        let mut mpos = 0;
        for _ in 0..n {
            if mpos >= mb.len() {
                metas.push(String::new());
                continue;
            }
            let end = mb[mpos..]
                .iter()
                .position(|&b| b == 0)
                .map(|q| mpos + q)
                .unwrap_or(mb.len());
            metas.push(String::from_utf8_lossy(&mb[mpos..end]).replace('\n', " "));
            mpos = end + 1;
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
                    meta: metas.get(i).cloned().unwrap_or_default(),
                })
                .collect(),
        )
    }
}
