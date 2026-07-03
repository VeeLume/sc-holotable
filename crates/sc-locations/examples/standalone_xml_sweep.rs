//! Sweep ALL standalone (non-socpak) .xml p4k entries for the location-CRC
//! preimage. These were NEVER scanned: socpak_guid_sweep only opens .socpak
//! files; Game2.dcb scans only covered the DataForge. This scans the RAW bytes
//! of each standalone XML (CryXmlB string table lives in there as ASCII) for:
//!   - ASCII guid strings (dash form, with or without braces) -> class_crc + crc32c/ieee (bytes+rev)
//!   - literal u32 windows (LE+BE) == target
//! Also DECODES the CryXml and crc's every attribute VALUE + tag string under
//! crc32c + crc32-ieee (+/- lowercase) to catch a name/key preimage.
//!
//! ```bash
//! cargo run -p sc-locations --release --example standalone_xml_sweep -- <targets.txt> [resolved.tsv]
//! ```
use std::collections::{HashMap, HashSet};

use sc_extract::object_container;
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
    let targets: HashSet<u32> = std::fs::read_to_string(&args[0])?
        .lines()
        .filter_map(|l| l.split('\t').next()?.trim().parse().ok())
        .collect();
    let names: HashMap<u32, String> = args
        .get(1)
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
    // standalone .xml entries only (these are NOT inside socpaks)
    let xmls: Vec<String> = assets
        .find(|n| n.to_ascii_lowercase().ends_with(".xml"))
        .map(|e| e.name.to_string())
        .collect();
    eprintln!(
        "standalone xml entries: {}  targets: {}",
        xmls.len(),
        targets.len()
    );

    let mut all_guids: HashMap<[u8; 16], String> = HashMap::new();
    let mut all_strings: HashSet<String> = HashSet::new();
    let mut hits: Vec<String> = Vec::new();
    let mut literal_hits = 0usize;

    for (n, path) in xmls.iter().enumerate() {
        if n % 1000 == 0 {
            eprintln!(
                "  [{n}/{}] guids={} strings={}",
                xmls.len(),
                all_guids.len(),
                all_strings.len()
            );
        }
        let Ok(bytes) = assets.read(path) else {
            continue;
        };
        // raw-byte guid scan (catches CryXmlB string-table ASCII)
        let mut local: HashSet<[u8; 16]> = HashSet::new();
        scan_guids(&bytes, &mut local);
        for g in local {
            all_guids.entry(g).or_insert_with(|| path.clone());
        }
        // literal u32 LE+BE
        for w in bytes.windows(4) {
            let le = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
            let be = u32::from_be_bytes([w[0], w[1], w[2], w[3]]);
            if targets.contains(&le) {
                hits.push(format!("LITERAL LE {le} [{}] in {path}", lbl(le)));
                literal_hits += 1;
            }
            if targets.contains(&be) {
                hits.push(format!("LITERAL BE {be} [{}] in {path}", lbl(be)));
                literal_hits += 1;
            }
        }
        // decode + collect attribute strings (only for first ~6000 to bound cost on strings;
        // guids/literal already cover all)
        if let Ok(Some(node)) = object_container::decode(&bytes) {
            collect_strings(&node, &mut all_strings);
        }
    }
    eprintln!(
        "unique guids: {}  unique strings: {}  literal hits: {literal_hits}",
        all_guids.len(),
        all_strings.len()
    );

    // GUID crc tests
    for (g, src) in &all_guids {
        let guid = Guid::from_bytes(*g);
        let cc = class_crc(&guid);
        if targets.contains(&cc) {
            hits.push(format!("class_crc({guid}) = {cc} [{}] <- {src}", lbl(cc)));
        }
        let mut rev = *g;
        rev.reverse();
        for (form, by) in [("bytes", &g[..]), ("rev", &rev[..])] {
            for (fnname, t) in [("crc32c", crc(by, &c_tbl)), ("ieee", crc(by, &i_tbl))] {
                if targets.contains(&t) {
                    hits.push(format!(
                        "{fnname}({form} {guid}) = {t} [{}] <- {src}",
                        lbl(t)
                    ));
                }
            }
        }
    }
    // string crc tests (name/key convention: crc32-ieee lowercased; also c & raw-case)
    for s in &all_strings {
        let lc = s.to_ascii_lowercase();
        for (label, bytes) in [("raw", s.as_bytes()), ("lc", lc.as_bytes())] {
            let ci = crc(bytes, &i_tbl);
            if targets.contains(&ci) {
                hits.push(format!("ieee({label} \"{s}\") = {ci} [{}]", lbl(ci)));
            }
            let cc = crc(bytes, &c_tbl);
            if targets.contains(&cc) {
                hits.push(format!("crc32c({label} \"{s}\") = {cc} [{}]", lbl(cc)));
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
