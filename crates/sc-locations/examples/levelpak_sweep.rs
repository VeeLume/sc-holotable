//! Scan the MegaMap level.pak (a ZIP) members for the location-CRC preimage.
//! These members (leveldata.xml, levelprototypes.xml, mission_mission0.xml, ...)
//! are NOT standalone p4k entries and are NOT inside any .socpak, so neither the
//! standalone-xml sweep nor the socpak sweep covered them. Lists members, dumps
//! each decoded, harvests GUIDs + strings + literal u32, tests vs targets.
//!
//! ```bash
//! cargo run -p sc-locations --release --example levelpak_sweep -- <targets.txt> [resolved.tsv] [--dump]
//! ```
use std::collections::{HashMap, HashSet};

use sc_extract::object_container::{self, Socpak};
use sc_extract::{AssetSource, Guid, class_crc};

fn table(poly: u32) -> [u32; 256] {
    let mut t = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ poly } else { c >> 1 };
        }
        t[i as usize] = c;
    }
    t
}
fn crc(b: &[u8], t: &[u32; 256]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &x in b {
        c = t[((c ^ x as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}
fn scan_guids(bytes: &[u8], out: &mut HashSet<[u8; 16]>) {
    if bytes.len() < 36 {
        return;
    }
    let hexval = |c: u8| -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    };
    for i in 0..=bytes.len() - 36 {
        if bytes[i + 8] != b'-'
            || bytes[i + 13] != b'-'
            || bytes[i + 18] != b'-'
            || bytes[i + 23] != b'-'
        {
            continue;
        }
        let hexpos = [
            0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 24, 25, 26, 27,
            28, 29, 30, 31, 32, 33, 34, 35,
        ];
        let mut nib = [0u8; 32];
        let mut ok = true;
        for (k, &p) in hexpos.iter().enumerate() {
            match hexval(bytes[i + p]) {
                Some(v) => nib[k] = v,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let mut g = [0u8; 16];
        for b in 0..16 {
            g[b] = (nib[b * 2] << 4) | nib[b * 2 + 1];
        }
        out.insert(g);
    }
}
fn collect_strings(n: &object_container::XmlNode, out: &mut HashSet<String>) {
    out.insert(n.tag.clone());
    for (k, v) in &n.attrs {
        out.insert(k.clone());
        out.insert(v.clone());
    }
    for c in &n.children {
        collect_strings(c, out);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dump = args.iter().any(|a| a == "--dump");
    let targets: HashSet<u32> = std::fs::read_to_string(&args[0])?
        .lines()
        .filter_map(|l| l.split('\t').next()?.trim().parse().ok())
        .collect();
    let names: HashMap<u32, String> = args
        .get(1)
        .filter(|a| !a.starts_with("--"))
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|t| {
            t.lines()
                .filter_map(|l| {
                    let mut it = l.split('\t');
                    let k = it.next()?.trim().parse().ok()?;
                    Some((k, it.next().unwrap_or("").to_string()))
                })
                .collect()
        })
        .unwrap_or_default();
    let lbl = |t: u32| names.get(&t).cloned().unwrap_or_else(|| "?".into());
    let c_tbl = table(0x82F6_3B78);
    let i_tbl = table(0xEDB8_8320);

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;

    // every .pak that is a ZIP of level data
    let paks: Vec<String> = assets
        .find(|n| n.to_ascii_lowercase().ends_with(".pak"))
        .map(|e| e.name.to_string())
        .collect();

    let mut all_guids: HashMap<[u8; 16], String> = HashMap::new();
    let mut all_strings: HashSet<String> = HashSet::new();
    let mut hits: Vec<String> = Vec::new();

    for pakpath in &paks {
        // skip the giant shader caches (not ZIP level data)
        if pakpath.to_ascii_lowercase().contains("shadercache")
            || pakpath.to_ascii_lowercase().contains("terraintexture")
        {
            continue;
        }
        let Ok(bytes) = assets.read(pakpath) else {
            continue;
        };
        let Ok(mut pak) = Socpak::open(bytes) else {
            eprintln!("not a zip: {pakpath}");
            continue;
        };
        eprintln!("== {pakpath}: {} members ==", pak.len());
        for i in 0..pak.len() {
            let nm = pak.name(i).unwrap_or_default();
            let Ok(b) = pak.read(i) else { continue };
            eprintln!("   [{i}] {nm} ({} bytes)", b.len());
            scan_guids(&b, &mut { HashSet::new() }); // warm; real below
            let mut local = HashSet::new();
            scan_guids(&b, &mut local);
            for g in local {
                all_guids
                    .entry(g)
                    .or_insert_with(|| format!("{pakpath}::{nm}"));
            }
            // literal
            for w in b.windows(4) {
                let le = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
                if targets.contains(&le) {
                    hits.push(format!("LITERAL LE {le} [{}] in {pakpath}::{nm}", lbl(le)));
                }
            }
            if let Ok(Some(node)) = object_container::decode(&b) {
                collect_strings(&node, &mut all_strings);
                if dump {
                    println!("---- {nm} ----");
                    fn w(n: &object_container::XmlNode, d: usize, c: &mut usize) {
                        if *c > 200 {
                            return;
                        }
                        let a: Vec<String> =
                            n.attrs.iter().map(|(k, v)| format!("{k}={v}")).collect();
                        println!("{}<{}> {}", "  ".repeat(d.min(8)), n.tag, a.join(" "));
                        *c += 1;
                        for ch in &n.children {
                            w(ch, d + 1, c);
                        }
                    }
                    let mut c = 0;
                    w(&node, 0, &mut c);
                }
            }
        }
    }
    eprintln!("guids: {}  strings: {}", all_guids.len(), all_strings.len());

    for (g, src) in &all_guids {
        let guid = Guid::from_bytes(*g);
        let cc = class_crc(&guid);
        if targets.contains(&cc) {
            hits.push(format!("class_crc({guid})={cc} [{}] <- {src}", lbl(cc)));
        }
        let mut rev = *g;
        rev.reverse();
        for (form, by) in [("bytes", &g[..]), ("rev", &rev[..])] {
            for (fnname, t) in [("crc32c", crc(by, &c_tbl)), ("ieee", crc(by, &i_tbl))] {
                if targets.contains(&t) {
                    hits.push(format!("{fnname}({form} {guid})={t} [{}] <- {src}", lbl(t)));
                }
            }
        }
    }
    for s in &all_strings {
        let lc = s.to_ascii_lowercase();
        for (label, by) in [("raw", s.as_bytes()), ("lc", lc.as_bytes())] {
            let ci = crc(by, &i_tbl);
            if targets.contains(&ci) {
                hits.push(format!("ieee({label} \"{s}\")={ci} [{}]", lbl(ci)));
            }
            let cc = crc(by, &c_tbl);
            if targets.contains(&cc) {
                hits.push(format!("crc32c({label} \"{s}\")={cc} [{}]", lbl(cc)));
            }
        }
    }

    println!("\n==== HITS: {} ====", hits.len());
    let mut seen = HashSet::new();
    for h in &hits {
        if seen.insert(h.clone()) {
            println!("  {h}");
        }
    }
    Ok(())
}
