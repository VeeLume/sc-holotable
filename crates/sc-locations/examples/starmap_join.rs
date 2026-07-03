//! Can we join starmap.space POIs to our socpak placements by EntityCryGUID?
//!
//! The community POI GUIDs are NOT DataForge records (see `position_audit`: 0/1824
//! resolve). Hypothesis: they are CryEngine `EntityCryGUID`s of the placed
//! entities in the *body* socpaks. This walks EVERY `objectcontainers/*.socpak`,
//! builds `storage-key → (Name, Pos, socpak)` for every entity (both GUID byte
//! orders), then joins the starmap POIs and — where they hit — compares our
//! entity `Pos` (m) against their body-local XYZ (km) to see if the frames agree.
//!
//! ```bash
//! cargo run -p sc-locations --release --example starmap_join -- <starmap_pois.json>
//! ```

use std::collections::HashMap;

use sc_extract::AssetSource;
use sc_extract::object_container::{Socpak, decode};

type Key = [u8; 16];
const MAP: [usize; 16] = [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8];

fn hex16(s: &str) -> Option<[u8; 16]> {
    let h: Vec<u8> = s.bytes().filter(u8::is_ascii_hexdigit).collect();
    if h.len() != 32 {
        return None;
    }
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[i] = ((h[i * 2] as char).to_digit(16)? * 16 + (h[i * 2 + 1] as char).to_digit(16)?) as u8;
    }
    Some(b)
}
fn cry_key(s: &str) -> Option<Key> {
    let db = hex16(s)?;
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[MAP[i]] = db[i];
    }
    Some(b)
}
fn std_key(s: &str) -> Option<Key> {
    let d = hex16(s)?;
    Some([
        d[3], d[2], d[1], d[0], d[5], d[4], d[7], d[6], d[8], d[9], d[10], d[11], d[12], d[13],
        d[14], d[15],
    ])
}
fn vec3(raw: &str) -> Option<[f64; 3]> {
    let p: Vec<f64> = raw
        .split([',', ' '])
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    (p.len() >= 3).then(|| [p[0], p[1], p[2]])
}

struct Ent {
    name: String,
    pos: Option<[f64; 3]>,
    socpak: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("warn").init();
    let json_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "starmap_pois.json".to_string());

    let install = sc_discovery::discover_primary()?;
    println!("-> {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;

    // Build storage-key → entity over EVERY objectcontainers socpak (both byte
    // orders keyed in — cry entities register under cry_key; we also register a
    // std_key alias so a POI GUID in either rendering finds it).
    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak") && l.contains("objectcontainers")
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("-> walking {} socpaks…", socpaks.len());

    let mut by_key: HashMap<Key, Ent> = HashMap::new();
    let mut ents = 0usize;
    for (n, sp) in socpaks.iter().enumerate() {
        if n % 500 == 0 {
            eprintln!("   {n}/{}", socpaks.len());
        }
        let base = sp.rsplit(['\\', '/']).next().unwrap_or(sp).to_string();
        let allow_xml = sp
            .to_ascii_lowercase()
            .replace('\\', "/")
            .contains("/system/");
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        for m in 0..pak.len() {
            let mem = pak.name(m).unwrap_or_default().to_ascii_lowercase();
            let ok = mem.ends_with(".soc")
                || mem.ends_with(".pla")
                || mem.ends_with(".entxml")
                || (allow_xml && mem.ends_with(".xml"));
            if !ok {
                continue;
            }
            let Ok(b) = pak.read(m) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            for e in root.find_all("Entity") {
                let Some(disp) = e.attr("EntityCryGUID") else {
                    continue;
                };
                let Some(k) = cry_key(disp) else { continue };
                ents += 1;
                let ent = Ent {
                    name: e.attr("Name").unwrap_or("").to_string(),
                    pos: e.attr("Pos").and_then(vec3),
                    socpak: base.clone(),
                };
                // prefer an entry that has a Pos if colliding
                by_key
                    .entry(k)
                    .and_modify(|old| {
                        if old.pos.is_none() && ent.pos.is_some() {
                            old.pos = ent.pos;
                        }
                    })
                    .or_insert(ent);
            }
        }
    }
    eprintln!("-> {} entities, {} distinct cry keys\n", ents, by_key.len());

    // Join starmap POIs.
    let bytes = std::fs::read(&json_path)?;
    let pois: serde_json::Value = serde_json::from_slice(&bytes)?;
    let arr = pois.as_array().cloned().unwrap_or_default();

    let mut hit_cry = 0usize;
    let mut hit_std = 0usize;
    let mut hit_pos = 0usize;
    let mut samples: Vec<String> = Vec::new();
    for p in &arr {
        let g = p.get("GUID").and_then(|v| v.as_str()).unwrap_or("");
        if g.is_empty() {
            continue;
        }
        let name = p.get("PoiName").and_then(|v| v.as_str()).unwrap_or("?");
        let sx = p.get("XCoord").and_then(|v| v.as_f64());
        let sy = p.get("YCoord").and_then(|v| v.as_f64());
        let sz = p.get("ZCoord").and_then(|v| v.as_f64());

        let ck = cry_key(g);
        let sk = std_key(g);
        let hit = ck
            .and_then(|k| by_key.get(&k))
            .map(|e| (e, "cry"))
            .or_else(|| sk.and_then(|k| by_key.get(&k)).map(|e| (e, "std")));
        let Some((e, order)) = hit else { continue };
        if order == "cry" {
            hit_cry += 1;
        } else {
            hit_std += 1;
        }
        if let (Some(op), Some(sx), Some(sy), Some(sz)) = (e.pos, sx, sy, sz) {
            hit_pos += 1;
            // their km → m; compare magnitudes (frame-agnostic) + components.
            let s_m = [sx * 1000.0, sy * 1000.0, sz * 1000.0];
            let onm = (op[0] * op[0] + op[1] * op[1] + op[2] * op[2]).sqrt();
            let snm = (s_m[0] * s_m[0] + s_m[1] * s_m[1] + s_m[2] * s_m[2]).sqrt();
            if samples.len() < 14 {
                samples.push(format!(
                    "  {name:<26} [{order}] ‖ours‖={onm:>11.0} ‖theirs‖={snm:>11.0} Δ={:>10.0}  ours={:?} theirs(m)={:?} ({})",
                    (onm - snm).abs(),
                    op.map(|v| v as i64),
                    s_m.map(|v| v as i64),
                    e.socpak
                ));
            }
        }
    }

    println!("═══ starmap ↔ socpak EntityCryGUID join ═══");
    println!("  POIs                       : {}", arr.len());
    println!("  matched (cry order)        : {hit_cry}");
    println!("  matched (std order)        : {hit_std}");
    println!("  matched AND both have Pos  : {hit_pos}");
    println!("  — samples (ours = entity Pos m; theirs = starmap XYZ km×1000) —");
    for s in &samples {
        println!("{s}");
    }
    Ok(())
}
