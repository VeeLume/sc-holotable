//! **Phase 0** — verify the mission-location chain is reconstructable OFFLINE.
//!
//! CryGUIDs are reused across cloned stations (§2.1) AND the two attrs that link
//! the hierarchy use DIFFERENT string renderings of the same 16 storage bytes:
//! `EntityCryGUID` is CryGUID-order (`4a269932-183e-362d-…`), `parentGUID` is
//! standard .NET order (`{183E362D-9932-4A26-…}`). So we canonicalize every GUID
//! to its storage bytes and test **edge-existence** for each oracle chain
//! (`loc_triples.tsv`, tail = `chain[2:]` = the hashed part):
//!   edge `gi → gi+1` exists if (a) some `gi+1` has `parentGUID == gi`, or
//!   (b) some `gi` nests a socpak `S` (`objectContainer`) containing `gi+1`.
//! All edges present ⇒ chain is offline-reconstructable.
//!
//! ```bash
//! cargo run -p sc-locations --release --example chain_verify -- <loc_triples.tsv>
//! ```

use std::collections::{HashMap, HashSet};

use sc_extract::AssetSource;
use sc_extract::object_container::{Socpak, XmlNode, decode};

type Key = [u8; 16]; // canonical storage bytes

fn hex16(s: &str) -> Option<[u8; 16]> {
    let h: Vec<u8> = s.bytes().filter(u8::is_ascii_hexdigit).collect();
    if h.len() != 32 {
        return None;
    }
    let mut b = [0u8; 16];
    for i in 0..16 {
        let hi = (h[i * 2] as char).to_digit(16)?;
        let lo = (h[i * 2 + 1] as char).to_digit(16)?;
        b[i] = (hi * 16 + lo) as u8;
    }
    Some(b)
}
/// CryGUID display order `[b7 b6 b5 b4]-[b3 b2]-[b1 b0]-[b15 b14]-[b13..b8]` → storage.
fn cry_key(s: &str) -> Option<Key> {
    let db = hex16(s)?;
    let map = [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8];
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[map[i]] = db[i];
    }
    Some(b)
}
/// Standard .NET GUID string (Data1/2/3 LE, Data4 BE) → storage.
fn std_key(s: &str) -> Option<Key> {
    let sd = hex16(s)?;
    Some([
        sd[3], sd[2], sd[1], sd[0], sd[5], sd[4], sd[7], sd[6], sd[8], sd[9], sd[10], sd[11],
        sd[12], sd[13], sd[14], sd[15],
    ])
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle_path = std::env::args().nth(1).expect("loc_triples.tsv arg");

    // oracle: name -> chain of canonical keys (one representative per name)
    let text = std::fs::read_to_string(&oracle_path)?;
    let mut oracle: HashMap<String, Vec<Key>> = HashMap::new();
    for line in text.lines() {
        let mut it = line.split('\t');
        let (Some(_), Some(chain), nm) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let name = nm
            .unwrap_or("")
            .split('[')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        let keys: Option<Vec<Key>> = chain.split('.').map(cry_key).collect();
        if let Some(keys) = keys {
            if keys.len() >= 3 {
                oracle.entry(name).or_insert(keys);
            }
        }
    }
    eprintln!("oracle: {} distinct named chains", oracle.len());

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

    let mut nests: HashMap<Key, HashSet<String>> = HashMap::new(); // cry -> child socpaks
    let mut socpak_crys: HashMap<String, HashSet<Key>> = HashMap::new(); // socpak -> crys
    let mut parents: HashMap<Key, HashSet<Key>> = HashMap::new(); // cry -> parent crys
    let mut seen = 0usize;
    for (i, sp) in socpaks.iter().enumerate() {
        if i % 2000 == 0 {
            eprintln!("  [{i}/{}] {seen}", socpaks.len());
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
                let Some(k) = e.attr("EntityCryGUID").and_then(cry_key) else {
                    continue;
                };
                seen += 1;
                socpak_crys.entry(sp_norm.clone()).or_default().insert(k);
                if let Some(oc) = first_attr(e, "objectContainer") {
                    nests.entry(k).or_default().insert(norm(oc));
                }
                if let Some(pk) = first_attr(e, "parentGUID").and_then(std_key) {
                    parents.entry(k).or_default().insert(pk);
                }
            }
        }
    }
    let all: HashSet<Key> = socpak_crys.values().flatten().copied().collect();
    eprintln!(
        "indexed {} crys ({seen} placements); {} nesting, {} parent edges\n",
        all.len(),
        nests.len(),
        parents.len()
    );

    let edge = |gi: &Key, gj: &Key| -> bool {
        if parents.get(gj).map_or(false, |ps| ps.contains(gi)) {
            return true;
        }
        if let Some(socpaks) = nests.get(gi) {
            for s in socpaks {
                if socpak_crys.get(s).map_or(false, |c| c.contains(gj)) {
                    return true;
                }
            }
        }
        false
    };

    let (mut full_ok, mut leaf_present, mut total) = (0, 0, 0);
    let mut break_at: HashMap<usize, usize> = HashMap::new();
    for (_name, chain) in &oracle {
        total += 1;
        let tail = &chain[2..];
        if all.contains(tail.last().unwrap()) {
            leaf_present += 1;
        }
        let mut ok = true;
        for k in 0..tail.len() - 1 {
            if !edge(&tail[k], &tail[k + 1]) {
                ok = false;
                *break_at.entry(k).or_default() += 1;
                break;
            }
        }
        if ok {
            full_ok += 1;
        }
    }
    eprintln!("==== Phase 0: offline edge-existence ====");
    eprintln!("oracle chains:            {total}");
    eprintln!("leaf present in socpaks:  {leaf_present}");
    eprintln!("ALL tail edges present:   {full_ok}  <- offline-reconstructable");
    let mut bk: Vec<_> = break_at.into_iter().collect();
    bk.sort();
    eprintln!("first-broken-edge by tail position (0=top): {bk:?}");
    Ok(())
}
