//! Definitive "location_id = class_crc(a GUID stored in Game2.dcb)" test.
//!
//! `class_crc(guid) == crc32c(guid's 16 storage bytes)` (verified byte-exact).
//! So we slide a 16-byte window over the ENTIRE raw DCB and crc32c (and crc32-IEEE,
//! forward & reversed) each window — catching the record GUID wherever it lives
//! (instance blob, reference, sub-structure), not just the records svarog
//! enumerates. If location_id is a class_crc of any 16 contiguous bytes in the
//! DCB, this finds it (with the byte offset).
//!
//! ```bash
//! cargo run -p sc-locations --release --example dcb_guid_window -- <targets.txt> [resolved.tsv]
//! ```
use std::collections::{HashMap, HashSet};

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
#[inline]
fn crc(b: &[u8], t: &[u32; 256]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &x in b {
        c = t[((c ^ x as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
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
                    Some((
                        k,
                        format!("{} / {}", it.next().unwrap_or(""), it.next().unwrap_or("")),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    let lbl = |t: u32| names.get(&t).cloned().unwrap_or_else(|| "?".into());
    eprintln!("targets: {}", targets.len());

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let (dcb_name, raw) = assets
        .find_and_read(|n| n.to_ascii_lowercase().ends_with("game2.dcb"))?
        .ok_or("Game2.dcb not found")?;
    eprintln!(
        "raw {dcb_name}: {} bytes; windows: {}",
        raw.len(),
        raw.len().saturating_sub(15)
    );

    let c_tbl = table(0x82F6_3B78); // crc32c == class_crc family
    let i_tbl = table(0xEDB8_8320); // crc32-IEEE
    let mut hits: Vec<String> = Vec::new();

    // Sanity: class_crc(Guid) must equal crc32c over its 16 storage bytes.
    {
        let g = Guid::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(
            class_crc(&g),
            crc(
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                &c_tbl
            )
        );
    }

    let mut rev = [0u8; 16];
    for (off, w) in raw.windows(16).enumerate() {
        let fc = crc(w, &c_tbl);
        if targets.contains(&fc) {
            hits.push(format!("crc32c/class_crc @ {off:#x} = {fc}  [{}]", lbl(fc)));
        }
        let fi = crc(w, &i_tbl);
        if targets.contains(&fi) {
            hits.push(format!("crc32-ieee @ {off:#x} = {fi}  [{}]", lbl(fi)));
        }
        rev.copy_from_slice(w);
        rev.reverse();
        let rc = crc(&rev, &c_tbl);
        if targets.contains(&rc) {
            hits.push(format!("crc32c(rev) @ {off:#x} = {rc}  [{}]", lbl(rc)));
        }
        let ri = crc(&rev, &i_tbl);
        if targets.contains(&ri) {
            hits.push(format!("crc32-ieee(rev) @ {off:#x} = {ri}  [{}]", lbl(ri)));
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
