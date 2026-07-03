//! **Phase 1 / A1** — reconstruct each mission-location chain from static socpaks
//! and reproduce the oracle CRC end-to-end (seeded only by the leaf GUID, NOT the
//! oracle's chain). Validates the top-down walk + the `[drop:]` start rule.
//!
//! Walk up from a leaf `(socpak, cry)`: parent = its `parentGUID` (resolved by
//! storage key, preferring the same socpak), else the OOC whose `objectContainer`
//! nests this socpak. Render every node as a CryGUID display string, then for
//! `drop ∈ 0..3` check `crc32_ieee(",".join(chain[drop:])) == oracle_crc`.
//!
//! ```bash
//! cargo run -p sc-locations --release --example crc_build -- <loc_triples.tsv>
//! ```

use std::collections::{HashMap, HashSet};

use sc_extract::AssetSource;
use sc_extract::object_container::{Socpak, XmlNode, decode};

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
    let sd = hex16(s)?;
    Some([
        sd[3], sd[2], sd[1], sd[0], sd[5], sd[4], sd[7], sd[6], sd[8], sd[9], sd[10], sd[11],
        sd[12], sd[13], sd[14], sd[15],
    ])
}
fn render_cry(k: &Key) -> String {
    let d: Vec<u8> = MAP.iter().map(|&i| k[i]).collect();
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        d[0],
        d[1],
        d[2],
        d[3],
        d[4],
        d[5],
        d[6],
        d[7],
        d[8],
        d[9],
        d[10],
        d[11],
        d[12],
        d[13],
        d[14],
        d[15]
    )
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
fn first_attr<'a>(e: &'a XmlNode, key: &str) -> Option<&'a str> {
    e.descendants()
        .find_map(|n| n.attr(key))
        .filter(|s| !s.is_empty())
}
fn norm(raw: &str) -> String {
    let s = raw.replace('\\', "/").to_ascii_lowercase();
    s.strip_prefix("data/").unwrap_or(&s).to_string()
}

#[derive(Clone)]
struct Ent {
    key: Key,
    disp: String,
    socpak: String,
    parent: Option<Key>,
    #[allow(dead_code)] // kept for debugging dumps
    oc: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_path = std::env::args().nth(1).expect("loc_triples.tsv arg");

    // oracle: name -> (crc, leaf_key)
    let text = std::fs::read_to_string(&oracle_path)?;
    let mut oracle: HashMap<String, (u32, Key)> = HashMap::new();
    for line in text.lines() {
        let mut it = line.split('\t');
        let (Some(h), Some(chain), nm) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let Ok(crc) = h.parse::<u32>() else { continue };
        let name = nm
            .unwrap_or("")
            .split('[')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        let Some(leaf) = chain.rsplit('.').next().and_then(cry_key) else {
            continue;
        };
        oracle.entry(name).or_insert((crc, leaf));
    }
    eprintln!("oracle: {} chains", oracle.len());

    // scan + index
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak") && l.contains("objectcontainers")
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("scanning {} socpaks ...", socpaks.len());

    let mut ents: Vec<Ent> = Vec::new();
    let mut by_sp_key: HashMap<(String, Key), usize> = HashMap::new();
    let mut by_key: HashMap<Key, Vec<usize>> = HashMap::new();
    let mut nest_owner: HashMap<String, usize> = HashMap::new(); // child socpak -> owner ent
    for (i, sp) in socpaks.iter().enumerate() {
        if i % 3000 == 0 {
            eprintln!("  [{i}/{}] {}", socpaks.len(), ents.len());
        }
        let sp_norm = norm(sp);
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        for m in 0..pak.len() {
            let nm = pak.name(m).unwrap_or_default().to_ascii_lowercase();
            if !(nm.ends_with(".soc") || nm.ends_with(".pla") || nm.ends_with(".entxml")) {
                continue;
            }
            let Ok(b) = pak.read(m) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            for e in root.find_all("Entity") {
                if !e
                    .attr("EntityClass")
                    .unwrap_or("")
                    .contains("ObjectContainer")
                {
                    continue;
                }
                let Some(disp) = e.attr("EntityCryGUID") else {
                    continue;
                };
                let disp = disp.to_ascii_lowercase();
                let Some(key) = cry_key(&disp) else { continue };
                let parent = first_attr(e, "parentGUID").and_then(std_key);
                let oc = first_attr(e, "objectContainer").map(norm);
                let idx = ents.len();
                if let Some(ref ocp) = oc {
                    nest_owner.entry(ocp.clone()).or_insert(idx);
                }
                by_sp_key.entry((sp_norm.clone(), key)).or_insert(idx);
                by_key.entry(key).or_default().push(idx);
                ents.push(Ent {
                    key,
                    disp,
                    socpak: sp_norm.clone(),
                    parent,
                    oc,
                });
            }
        }
    }
    eprintln!(
        "indexed {} placements ({} distinct keys); {} nesting edges\n",
        ents.len(),
        by_key.len(),
        nest_owner.len()
    );

    // walk up from an ent index -> reconstructed chain (root..leaf) of display strings
    let walk = |start: usize| -> Vec<String> {
        let mut chain = vec![ents[start].disp.clone()];
        let mut cur = start;
        let mut seen: HashSet<Key> = HashSet::new();
        seen.insert(ents[start].key);
        for _ in 0..24 {
            let e = &ents[cur];
            if let Some(pk) = e.parent {
                if seen.contains(&pk) {
                    break;
                }
                // resolve the parent placement: prefer same socpak, else any
                let pe = by_sp_key
                    .get(&(e.socpak.clone(), pk))
                    .copied()
                    .or_else(|| by_key.get(&pk).and_then(|v| v.first().copied()));
                match pe {
                    Some(pe) => {
                        seen.insert(pk);
                        chain.push(ents[pe].disp.clone());
                        cur = pe;
                    }
                    None => {
                        chain.push(render_cry(&pk));
                        break;
                    } // parent not an entity (universe root)
                }
            } else if let Some(oe) = nest_owner.get(&e.socpak).copied() {
                if seen.contains(&ents[oe].key) {
                    break;
                }
                seen.insert(ents[oe].key);
                chain.push(ents[oe].disp.clone());
                cur = oe;
            } else {
                break;
            }
        }
        chain.reverse();
        chain
    };

    // validate: for each oracle leaf, try every placement, find a drop that matches the crc
    let mut by_drop: HashMap<usize, usize> = HashMap::new();
    let (mut matched, mut leaf_found, mut total) = (0, 0, 0);
    let mut shown = 0;
    for (name, (crc, leaf)) in &oracle {
        total += 1;
        let Some(places) = by_key.get(leaf) else {
            continue;
        };
        leaf_found += 1;
        let mut best: Option<(usize, Vec<String>)> = None;
        'outer: for &p in places {
            let chain = walk(p);
            for drop in 0..chain.len().min(4) {
                if crc32_ieee(chain[drop..].join(",").as_bytes()) == *crc {
                    best = Some((drop, chain.clone()));
                    break 'outer;
                }
            }
        }
        if let Some((drop, chain)) = best {
            matched += 1;
            *by_drop.entry(drop).or_default() += 1;
            if shown < 6 {
                shown += 1;
                eprintln!(
                    "✓ {name}: drop={drop} chain={}",
                    chain
                        .iter()
                        .map(|g| g[..8].to_string())
                        .collect::<Vec<_>>()
                        .join("→")
                );
            }
        } else if shown < 6 {
            shown += 1;
            let ex = walk(places[0]);
            eprintln!(
                "✗ {name}: no drop matched crc={crc}; walked={}",
                ex.iter()
                    .map(|g| g[..8].to_string())
                    .collect::<Vec<_>>()
                    .join("→")
            );
        }
    }
    eprintln!("\n==== A1: offline chain → CRC ====");
    eprintln!("oracle chains:        {total}");
    eprintln!("leaf found:           {leaf_found}");
    eprintln!("CRC reproduced:       {matched}");
    let mut d: Vec<_> = by_drop.into_iter().collect();
    d.sort();
    eprintln!("by drop-count:        {d:?}   (drop=2 ⇒ matches the [2:] rule directly)");
    Ok(())
}
