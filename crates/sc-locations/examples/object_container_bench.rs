//! Measurement bench for the `object_containers.rs` unification decision.
//!
//! Answers three questions before we commit to a single full socpak walk:
//!   1. **Cost of parsing the full tree** — enumerate every
//!      `objectcontainers/*.socpak`, decode every `.soc/.pla/.entxml` member,
//!      harvest placements. Timed in isolation.
//!   2. **Cost of CRC-ing the full tree** — build the key indices, walk each
//!      placement's containment chain, `crc32` it. Timed in isolation.
//!   3. **What `<Child>` harvesting reveals** — how many named/positioned places
//!      the entity-level pass misses that live on nested `<Child>` nodes, and
//!      whether the missing bodies/stations (Crusader, ArcCorp, Everus Harbor,
//!      Baijini Point, Seraphim, …) show up once we harvest them.
//!
//! Usage: cargo run -p sc-locations --example object_container_bench --release

use std::collections::HashMap;
use std::time::Instant;

use sc_extract::object_container::{Socpak, decode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, class_crc};
use sc_locations::Locations;

// ── GUID / CRC helpers (mirrors mission_crc.rs — will be shared post-unify) ──
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
fn crc32_ieee(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = (c >> 1) ^ (0xEDB8_8320 & (c & 1).wrapping_neg());
        }
    }
    !c
}
fn norm(raw: &str) -> String {
    let s = raw.replace('\\', "/").to_ascii_lowercase();
    s.strip_prefix("data/").unwrap_or(&s).to_string()
}
fn vec3(raw: &str) -> Option<[f64; 3]> {
    let p: Vec<f64> = raw
        .split([',', ' '])
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    (p.len() >= 3).then(|| [p[0], p[1], p[2]])
}
fn name_of<'a>(starmap: &str, locs: &Locations, ad: &'a AssetData) -> Option<&'a str> {
    let g = starmap
        .trim()
        .trim_matches(['{', '}'])
        .parse::<Guid>()
        .ok()?;
    locs.by_crc(class_crc(&g))?.display_name(&ad.locale)
}

/// A harvested placement — Entity or Child node.
struct P {
    cry: Option<Key>,
    parent: Option<Key>,
    socpak: String,
    nests: Option<String>,
    starmap: Option<String>,
    pos: Option<[f64; 3]>,
    from_child: bool,
    is_place: bool, // has starmap or mission template → a chain leaf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let install = sc_discovery::discover_primary()?;
    println!("-> {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let ad = AssetData::extract(&assets, &AssetConfig::standard())?;
    let t = Instant::now();
    let datacore = sc_extract::Datacore::parse(&assets, &ad)?;
    let locs = Locations::build(datacore.records());
    println!("-> datacore+locations: {:.2}s\n", t.elapsed().as_secs_f64());

    // ── 1. enumerate ────────────────────────────────────────────────────
    let t = Instant::now();
    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak") && l.contains("objectcontainers")
        })
        .map(|e| e.name.to_string())
        .collect();
    println!("═══ 1. ENUMERATE ═══");
    println!("  objectcontainers/*.socpak : {}", socpaks.len());
    println!(
        "  enumerate time            : {:.3}s\n",
        t.elapsed().as_secs_f64()
    );

    // ── 2. PARSE (read + zstd + CryXmlB decode + harvest) ───────────────
    let t = Instant::now();
    let mut ents: Vec<P> = Vec::new();
    let (mut members, mut decoded, mut e_pos, mut e_star) = (0u64, 0u64, 0u64, 0u64);
    let (mut c_nodes, mut c_pos, mut c_star) = (0u64, 0u64, 0u64);
    let mut child_dumps = 0;

    for sp in &socpaks {
        let sp_norm = norm(sp);
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        for m in 0..pak.len() {
            let mm = pak.name(m).unwrap_or_default().to_ascii_lowercase();
            if !(mm.ends_with(".soc") || mm.ends_with(".pla") || mm.ends_with(".entxml")) {
                continue;
            }
            members += 1;
            let Ok(b) = pak.read(m) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            decoded += 1;

            for e in root.find_all("Entity") {
                let cry = e.attr("EntityCryGUID").and_then(cry_key);
                let parent = e
                    .descendants()
                    .find_map(|n| n.attr("parentGUID"))
                    .and_then(std_key);
                let nests = e
                    .find_all("EntityComponentObjectContainer")
                    .next()
                    .and_then(|c| c.attr("objectContainer"))
                    .map(norm);
                let pos = e.attr("Pos").and_then(vec3);
                // Entity's OWN nav id: starmapRecord on SObjectMetadataParams,
                // *not* descending into <Child> (measured separately below).
                let e_starmap = e
                    .find_all("SObjectMetadataParams")
                    .next()
                    .and_then(|p| p.attr("starmapRecord"))
                    .filter(|s| !s.starts_with("00000000-"))
                    .map(str::to_string);
                if pos.is_some() {
                    e_pos += 1;
                }
                if e_starmap.is_some() {
                    e_star += 1;
                }
                let has_mission = e.find_all("MissionLocationParams").next().is_some();
                ents.push(P {
                    cry,
                    parent,
                    socpak: sp_norm.clone(),
                    nests,
                    is_place: e_starmap.is_some() || has_mission,
                    starmap: e_starmap,
                    pos,
                    from_child: false,
                });

                // <Child> sub-placements.
                for ch in e.find_all("Child") {
                    c_nodes += 1;
                    let cpos = ch.attr("pos").or_else(|| ch.attr("Pos")).and_then(vec3);
                    let cstar = ch
                        .attr("starmapRecord")
                        .filter(|s| !s.starts_with("00000000-"))
                        .map(str::to_string);
                    if cpos.is_some() {
                        c_pos += 1;
                    }
                    if cstar.is_none() {
                        continue;
                    }
                    c_star += 1;
                    if child_dumps < 3 {
                        println!("  raw <Child>: attrs = {:?}", ch.attrs);
                        child_dumps += 1;
                    }
                    ents.push(P {
                        cry: ch.attr("EntityCryGUID").and_then(cry_key),
                        parent: ch.attr("parentGUID").and_then(std_key).or(cry.map(|k| k)),
                        socpak: sp_norm.clone(),
                        nests: None,
                        is_place: true,
                        starmap: cstar,
                        pos: cpos,
                        from_child: true,
                    });
                }
            }
        }
    }
    let t_parse = t.elapsed().as_secs_f64();
    println!("\n═══ 2. PARSE ═══");
    println!("  members decoded           : {decoded}/{members}");
    println!(
        "  entities                  : {}",
        ents.iter().filter(|p| !p.from_child).count()
    );
    println!("    with Pos                : {e_pos}");
    println!("    with own starmapRecord  : {e_star}");
    println!("  <Child> nodes             : {c_nodes}");
    println!("    with pos                : {c_pos}");
    println!("    with starmapRecord      : {c_star}");
    println!("  PARSE TIME                : {t_parse:.2}s\n");

    // ── 3. INDEX + CRC ──────────────────────────────────────────────────
    let t = Instant::now();
    let mut by_sp_key: HashMap<(String, Key), usize> = HashMap::new();
    let mut by_key: HashMap<Key, Vec<usize>> = HashMap::new();
    let mut nest_owner: HashMap<String, usize> = HashMap::new();
    for (i, e) in ents.iter().enumerate() {
        if let Some(k) = e.cry {
            by_sp_key.entry((e.socpak.clone(), k)).or_insert(i);
            by_key.entry(k).or_default().push(i);
        }
        if let Some(oc) = &e.nests {
            nest_owner.entry(oc.clone()).or_insert(i);
        }
    }
    let t_index = t.elapsed().as_secs_f64();

    let t = Instant::now();
    let mut crcs = 0u64;
    for i in 0..ents.len() {
        if !ents[i].is_place || ents[i].cry.is_none() {
            continue;
        }
        // Chain walk (parentGUID edges, then socpak-nesting), root→leaf.
        let mut chain = vec![i];
        let mut cur = i;
        for _ in 0..24 {
            let e = &ents[cur];
            let next = e
                .parent
                .and_then(|pk| {
                    by_sp_key
                        .get(&(e.socpak.clone(), pk))
                        .copied()
                        .or_else(|| by_key.get(&pk).and_then(|v| v.first().copied()))
                })
                .or_else(|| nest_owner.get(&e.socpak).copied());
            match next {
                Some(n) if !chain.contains(&n) => {
                    chain.push(n);
                    cur = n;
                }
                _ => break,
            }
        }
        if chain.len() < 2 {
            continue;
        }
        // crc over the EntityCryGUID chain (display strings) — measure the cost.
        let joined: Vec<String> = chain
            .iter()
            .rev()
            .filter_map(|&j| ents[j].cry.map(|k| format!("{k:02x?}")))
            .collect();
        let _ = crc32_ieee(joined.join(",").as_bytes());
        crcs += 1;
    }
    let t_crc = t.elapsed().as_secs_f64();
    println!("═══ 3. INDEX + CRC ═══");
    println!("  index time                : {t_index:.3}s");
    println!("  crc chains computed       : {crcs}");
    println!("  CRC TIME                  : {t_crc:.3}s\n");

    // ── 4. COVERAGE — what child harvesting reveals ─────────────────────
    let mut entity_names: HashMap<String, bool> = HashMap::new(); // name → has_pos
    let mut child_names: HashMap<String, bool> = HashMap::new();
    for e in &ents {
        let Some(sm) = &e.starmap else { continue };
        let Some(n) = name_of(sm, &locs, &ad) else {
            continue;
        };
        let map = if e.from_child {
            &mut child_names
        } else {
            &mut entity_names
        };
        let slot = map.entry(n.to_string()).or_insert(false);
        *slot |= e.pos.is_some();
    }
    let only_child: Vec<&String> = child_names
        .keys()
        .filter(|n| !entity_names.contains_key(*n))
        .collect();
    println!("═══ 4. COVERAGE ═══");
    println!("  named via entity          : {}", entity_names.len());
    println!("  named via <Child>         : {}", child_names.len());
    println!("  names ONLY via <Child>    : {}", only_child.len());
    println!();

    let targets = [
        "Crusader",
        "ArcCorp",
        "Daymar",
        "Yela",
        "Cellin",
        "Hurston",
        "microTech",
        "Everus Harbor",
        "Baijini Point",
        "Port Tressler",
        "Seraphim Station",
        "Grim HEX",
        "Area18",
        "Lorville",
    ];
    println!("  target coverage (E=entity-pos, C=child-pos, -=named-no-pos, x=absent):");
    for t in targets {
        let e = entity_names.get(t);
        let c = child_names.get(t);
        let mark = match (e, c) {
            (Some(true), _) => "E",
            (_, Some(true)) => "C",
            (Some(false), _) | (_, Some(false)) => "-",
            (None, None) => "x",
        };
        println!("    [{mark}] {t}");
    }
    Ok(())
}
