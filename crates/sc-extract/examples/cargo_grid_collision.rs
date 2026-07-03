//! Tier C (attempt): per-cell wall-vs-open occupancy for a placed cargo grid.
//!
//! This is the hard tier — "does grid cell (x,y,z) face a wall or open space?"
//! needs the ship's collision/render geometry, not just the DCB. This example is
//! deliberately staged as *reconnaissance first*: it reads the hull `.cga`,
//! enumerates its IVO chunks, and reports which carry mesh / physics geometry and
//! whether their payload is tractably decodable (plain floats vs quantized
//! streams). It then demonstrates the coarsest real occupancy signal that needs
//! **no** triangle decode — the geometry's own bounding box — and documents
//! exactly where the remaining wall of work (full mesh + per-cell raytest) starts.
//!
//! ```bash
//! cargo run -p sc-extract --release --example cargo_grid_collision
//! cargo run -p sc-extract --release --example cargo_grid_collision -- MISC_Freelancer
//! ```
//!
//! See `docs/ship-cargo-grids.md` (§Geometry read — Tier C).

use sc_extract::{AssetConfig, AssetData, AssetSource};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

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

    // ── locate ship + hull geometry path (same as Tier B) ─────────────────────
    let Some(rec) = shortest_entity(db, &ship) else {
        eprintln!("no EntityClassDefinition named '{ship}'");
        return Ok(());
    };
    let inst = rec.as_instance();
    let Some(hull) = first_cga(db, &inst) else {
        eprintln!("no .cga hull geometry");
        return Ok(());
    };
    println!("hull geometry: {hull}");
    let bytes = read_p4k(&assets, &hull)?;
    println!("hull CGA: {} bytes\n", bytes.len());

    // ── 1. chunk reconnaissance ───────────────────────────────────────────────
    let chunks = ivo_chunks(&bytes).ok_or("not an IVO file / bad chunk table")?;
    println!("== IVO chunks ({}) ==", chunks.len());
    for c in &chunks {
        println!(
            "  type=0x{:08X} {:<18} size={:>10}  {}",
            c.ty,
            chunk_name(c.ty),
            c.size,
            geometry_role(c.ty),
        );
    }

    // ── 2. can we get geometry vertices without a full mesh decode? ───────────
    println!("\n== decodability probe ==");
    let mesh_types = [
        (0xB8757777u32, "IvoSkin2"),
        (0x92914444, "MeshIvo320"),
        (0x58DE1772, "StatObjPhysics"),
        (0x90C62222, "PhysicalHierarchy"),
    ];
    let mut any = false;
    let mut hull_aabb: Option<([f32; 3], [f32; 3])> = None;
    for (ty, label) in mesh_types {
        if let Some(c) = chunks.iter().find(|c| c.ty == ty) {
            any = true;
            let data = &bytes[c.off..c.off + c.size.min(bytes.len() - c.off)];
            if let Some(aabb) = report_mesh_chunk(label, data) {
                hull_aabb.get_or_insert(aabb);
            }
        }
    }
    if !any {
        println!("  (no mesh/physics chunk present in this file — hull render mesh may live");
        println!("   in a sibling .cgf, or geometry is split across LODs / sub-objects)");
    }

    // ── 3. coarsest occupancy signal that needs NO triangle decode ────────────
    println!("\n== what a no-triangle occupancy pass yields ==");
    match hull_aabb {
        Some((mn, mx)) => {
            println!(
                "  hull model AABB (m): min({:+.2}, {:+.2}, {:+.2})  max({:+.2}, {:+.2}, {:+.2})",
                mn[0], mn[1], mn[2], mx[0], mx[1], mx[2]
            );
            println!(
                "  extents: {:.1} x {:.1} x {:.1} m",
                mx[0] - mn[0],
                mx[1] - mn[1],
                mx[2] - mn[2]
            );
            println!("  → cheap signal: a placed grid box (Tier B) flush with an AABB face is");
            println!("    'at the hull edge'. Coarse: can't tell an interior bulkhead from open");
            println!("    bay, and can't see the cargo door aperture. Needs frame reconciliation");
            println!(
                "    with the NMC grid transform (both are hull-space, but axis/units differ)."
            );
        }
        None => println!("  (no cheap AABB recoverable from the mesh chunk headers)"),
    }

    // ── 4. the frontier, stated precisely ─────────────────────────────────────
    println!("\n== remaining work for exact per-cell occupancy (the wall) ==");
    println!("  1. Decode the mesh data streams (IvoSkin2 / MeshIvo320): positions are");
    println!("     quantized (16-bit + per-mesh dequant scale/offset from the mesh header),");
    println!("     indices are a separate stream. StarBreaker's ivo/skin.rs + dequant.rs");
    println!("     (MIT) do this — ~a few k LOC to port, the bulk of the 38k-LOC pipeline.");
    println!("  2. Select the COLLISION proxy, not the render mesh (StatObjPhysics /");
    println!("     MeshPhysicsData) — cargo bays have a simplified collision hull.");
    println!("  3. Transform grid cell centers into mesh space (inverse of the Tier B");
    println!("     hardpoint transform) and raytest each cell's 6 face normals against");
    println!("     the collision triangles: hit within <1.25 m = wall, else open.");
    println!("  4. Interior bulkheads/doors live in the interior socpak meshes, not the");
    println!("     hull .cga — a full answer unions hull + interior-socpak collision.");
    println!("\n  Verdict: tractable but weeks of mesh-decode work; the AABB proxy above");
    println!("  is the ceiling of what's cheaply readable. See docs/ship-cargo-grids.md.");

    Ok(())
}

/// Report what we can cheaply learn from a mesh chunk without decoding triangles:
/// leading header words + a heuristic AABB (min/max vec3) if one is present near
/// the header. Returns the AABB if found.
fn report_mesh_chunk(label: &str, data: &[u8]) -> Option<([f32; 3], [f32; 3])> {
    println!("  {label}: {} bytes", data.len());
    let n = 16.min(data.len() / 4);
    let mut floats = Vec::with_capacity(n);
    let mut ints = Vec::with_capacity(n);
    for i in 0..n {
        let b: [u8; 4] = data[i * 4..i * 4 + 4].try_into().unwrap();
        ints.push(u32::from_le_bytes(b));
        floats.push(f32::from_bits(u32::from_le_bytes(b)));
    }
    println!("    head u32: {:?}", &ints[..n.min(8)]);
    let show: Vec<String> = floats
        .iter()
        .map(|f| {
            if f.is_finite() && f.abs() < 1.0e4 {
                format!("{f:.2}")
            } else {
                "·".into()
            }
        })
        .collect();
    println!("    head f32: [{}]", show.join(", "));

    // Scan for the first 6-float window that reads as a valid AABB:
    // three finite mins strictly less than three finite maxes, sane magnitude.
    for w in 0..floats.len().saturating_sub(5) {
        let f = &floats[w..w + 6];
        let ok = f.iter().all(|v| v.is_finite() && v.abs() < 1.0e4)
            && f[0] < f[3]
            && f[1] < f[4]
            && f[2] < f[5]
            && (f[3] - f[0]) > 0.5
            && (f[4] - f[1]) > 0.5;
        if ok {
            let aabb = ([f[0], f[1], f[2]], [f[3], f[4], f[5]]);
            println!(
                "    → candidate model AABB at +{}: min({:.2},{:.2},{:.2}) max({:.2},{:.2},{:.2})",
                w * 4,
                f[0],
                f[1],
                f[2],
                f[3],
                f[4],
                f[5]
            );
            return Some(aabb);
        }
    }
    None
}

// ── IVO chunk table ──────────────────────────────────────────────────────────

struct Chunk {
    ty: u32,
    off: usize,
    size: usize,
}

fn ivo_chunks(file: &[u8]) -> Option<Vec<Chunk>> {
    const IVO_MAGIC: u32 = 0x6F766923;
    let rd = |p: usize| -> Option<u32> {
        file.get(p..p + 4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
    };
    let rd64 = |p: usize| -> Option<u64> {
        file.get(p..p + 8)
            .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
    };
    if rd(0)? != IVO_MAGIC {
        return None;
    }
    let count = rd(8)? as usize;
    let table = rd(12)? as usize;
    // entry: type:u32 version:u32 offset:u64  (16 B). Sort by offset to size them.
    let mut raw: Vec<(u32, usize)> = Vec::with_capacity(count);
    for i in 0..count {
        let base = table + i * 16;
        raw.push((rd(base)?, rd64(base + 8)? as usize));
    }
    let mut by_off: Vec<usize> = (0..count).collect();
    by_off.sort_by_key(|&i| raw[i].1);
    let mut size = vec![0usize; count];
    for (pos, &i) in by_off.iter().enumerate() {
        let next = if pos + 1 < by_off.len() {
            raw[by_off[pos + 1]].1
        } else {
            file.len()
        };
        size[i] = next.saturating_sub(raw[i].1);
    }
    Some(
        (0..count)
            .map(|i| Chunk {
                ty: raw[i].0,
                off: raw[i].1,
                size: size[i],
            })
            .collect(),
    )
}

fn chunk_name(ty: u32) -> &'static str {
    match ty {
        0x70697FDA => "NodeMeshCombos",
        0xB8757777 => "IvoSkin2",
        0x92914444 => "MeshIvo320",
        0x83353333 => "MtlNameIvo320",
        0x58DE1772 => "StatObjPhysics",
        0x90C62222 => "PhysicalHierarchy",
        0xBE5E493E => "ExportFlags",
        0x9351756F => "LODDistance",
        0xB32459D2 => "VisAreas",
        0x2B7ECF9F => "PositionBonemap",
        0x1BBC4103 => "Skeleton",
        0xC201973C => "CompiledBones",
        _ => "?",
    }
}

fn geometry_role(ty: u32) -> &'static str {
    match ty {
        0x70697FDA => "← node scene graph (Tier B placement)",
        0xB8757777 | 0x92914444 => "← render mesh (vertices/indices, quantized)",
        0x58DE1772 | 0x90C62222 => "← COLLISION geometry (Tier C target)",
        _ => "",
    }
}

// ── shared helpers (mirror Tier B) ───────────────────────────────────────────

fn shortest_entity<'a>(
    db: &'a DataCoreDatabase,
    name: &str,
) -> Option<svarog_datacore::Record<'a>> {
    let mut cands: Vec<_> = db
        .records_by_type("EntityClassDefinition")
        .filter(|r| {
            let n = r.name().unwrap_or("");
            n.rsplit('.').next().unwrap_or(n).eq_ignore_ascii_case(name)
        })
        .collect();
    cands.sort_by_key(|r| r.name().map(|n| n.len()).unwrap_or(usize::MAX));
    cands.into_iter().next()
}

fn first_cga(db: &DataCoreDatabase, inst: &Instance) -> Option<String> {
    let mut found: Option<String> = None;
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

fn read_p4k(assets: &AssetSource, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if let Some(b) = assets.try_read(path)? {
        return Ok(b);
    }
    let prefixed = format!("Data\\{path}");
    assets
        .try_read(&prefixed)?
        .ok_or_else(|| format!("not found in p4k: {path}").into())
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
