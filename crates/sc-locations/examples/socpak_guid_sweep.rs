//! Comprehensive GUID sweep across EVERY socpak for the location-CRC preimage.
//!
//! `socpak_mine` only covered Stanton top-level body OCs (it skipped nested
//! sub-OCs). This opens ALL `.socpak` in the p4k, byte-scans every text/entity
//! member for ASCII guid-shaped strings (no XML decode — fast), dedupes, then
//! tests each unique GUID under class_crc + crc32c/IEEE (storage bytes &
//! reversed) against the 209 mission location CRCs. Optionally (`--literal`)
//! also literal-scans member bytes for a stored u32 id.
//!
//! ```bash
//! cargo run -p sc-locations --release --example socpak_guid_sweep -- <targets.txt> [resolved.tsv] [--literal]
//! ```
use std::collections::{HashMap, HashSet};

use sc_extract::object_container::Socpak;
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

/// Byte-scan for ASCII guid strings `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`.
/// Cheap: only does the full check when the 4 dashes line up.
fn scan_guids(bytes: &[u8], out: &mut HashSet<[u8; 16]>) {
    if bytes.len() < 36 {
        return;
    }
    let mut hexval = |c: u8| -> Option<u8> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let literal = args.iter().any(|a| a == "--literal");
    let binary = args.iter().any(|a| a == "--binary");
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
                    let nm = it.next().unwrap_or("");
                    let br = it.next().unwrap_or("");
                    Some((k, format!("{nm} / {br}")))
                })
                .collect()
        })
        .unwrap_or_default();
    let lbl = |t: u32| names.get(&t).cloned().unwrap_or_else(|| "?".into());
    eprintln!("targets: {}  literal-scan: {literal}", targets.len());

    let c_tbl = table(0x82F6_3B78); // crc32c
    let i_tbl = table(0xEDB8_8320); // crc32-IEEE

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let socpaks: Vec<String> = assets
        .find(|n| n.to_ascii_lowercase().ends_with(".socpak"))
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("socpaks: {}", socpaks.len());

    let mut guids: HashMap<[u8; 16], String> = HashMap::new(); // guid -> first socpak
    let mut hits: Vec<String> = Vec::new();
    let mut scanned_members = 0usize;

    for (n, sp) in socpaks.iter().enumerate() {
        if n % 500 == 0 {
            eprintln!("  [{n}/{}] {} guids so far", socpaks.len(), guids.len());
        }
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        for i in 0..pak.len() {
            let nm = pak.name(i).unwrap_or_default().to_ascii_lowercase();
            let texty = nm.ends_with(".soc")
                || nm.ends_with(".pla")
                || nm.ends_with(".entxml")
                || nm.ends_with(".xml")
                || nm.ends_with(".eco")
                || nm.ends_with(".rmp");
            if !texty {
                continue;
            }
            let Ok(b) = pak.read(i) else { continue };
            scanned_members += 1;
            let mut local: HashSet<[u8; 16]> = HashSet::new();
            scan_guids(&b, &mut local);
            for g in local {
                guids.entry(g).or_insert_with(|| sp.clone());
            }
            if literal {
                for w in b.windows(4) {
                    let v = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
                    if targets.contains(&v) {
                        hits.push(format!("LITERAL u32 {v} [{}] in {sp}::member{i}", lbl(v)));
                    }
                }
            }
            if binary {
                // 16-byte-window CRC — catches a GUID stored BINARY (not ASCII)
                // inside a non-CryXml chunk of a .soc.
                let mut rev = [0u8; 16];
                for w in b.windows(16) {
                    let fc = crc(w, &c_tbl);
                    if targets.contains(&fc) {
                        hits.push(format!("BIN crc32c {fc} [{}] in {sp}::m{i}", lbl(fc)));
                    }
                    let fi = crc(w, &i_tbl);
                    if targets.contains(&fi) {
                        hits.push(format!("BIN ieee {fi} [{}] in {sp}::m{i}", lbl(fi)));
                    }
                    rev.copy_from_slice(w);
                    rev.reverse();
                    let rc = crc(&rev, &c_tbl);
                    if targets.contains(&rc) {
                        hits.push(format!("BIN crc32c(rev) {rc} [{}] in {sp}::m{i}", lbl(rc)));
                    }
                    let ri = crc(&rev, &i_tbl);
                    if targets.contains(&ri) {
                        hits.push(format!("BIN ieee(rev) {ri} [{}] in {sp}::m{i}", lbl(ri)));
                    }
                }
            }
        }
    }
    eprintln!(
        "scanned {scanned_members} members; unique guids: {}",
        guids.len()
    );

    for (g, src) in &guids {
        let guid = Guid::from_bytes(*g);
        let cc = class_crc(&guid);
        if targets.contains(&cc) {
            hits.push(format!("class_crc({guid}) = {cc}  [{}]  <- {src}", lbl(cc)));
        }
        let mut rev = *g;
        rev.reverse();
        for (form, by) in [("bytes", &g[..]), ("rev", &rev[..])] {
            for (fnname, t) in [("crc32c", crc(by, &c_tbl)), ("ieee", crc(by, &i_tbl))] {
                if targets.contains(&t) {
                    hits.push(format!(
                        "{fnname}({form} {guid}) = {t}  [{}]  <- {src}",
                        lbl(t)
                    ));
                }
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
