//! Brute the ENTIRE datacore value surface against the mission location CRCs.
//!
//! The DCB stores every primitive/reference value as flat deduplicated pools
//! (svarog `raw_pool_data`) + the string tables. We test each pool:
//!   - uint32 / int32 / uint64 → LITERAL membership (a stored location id)
//!   - GUID pool → class_crc + crc32c/IEEE over bytes (incl. reversed)
//!   - both string tables → crc32c + crc32-IEEE over each string (±case)
//! against the 209 mission location CRCs.
//!
//! ```bash
//! cargo run -p sc-locations --release --example dcb_value_sweep -- <loc_targets.txt> [loc_resolved.tsv]
//! ```
use std::collections::{HashMap, HashSet};

use sc_extract::svarog_datacore::PoolType;
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, class_crc};

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
                    Some((
                        it.next()?.trim().parse().ok()?,
                        it.next().unwrap_or("").to_string(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    eprintln!("targets: {}", targets.len());

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    // Raw decompressed Game2.dcb bytes — for the complete literal scan.
    let (dcb_name, raw) = assets
        .find_and_read(|n| n.to_ascii_lowercase().ends_with("game2.dcb"))?
        .ok_or("Game2.dcb not found")?;
    eprintln!("raw {dcb_name}: {} bytes", raw.len());
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();
    let c_tbl = table(0x82F6_3B78);
    let i_tbl = table(0xEDB8_8320);
    let _ = PoolType::UInt32; // (pools only hold array-property values; we scan raw bytes instead)

    let mut hits: Vec<String> = Vec::new();
    let lbl = |t: u32| names.get(&t).cloned().unwrap_or_else(|| "?".into());

    // 1) LITERAL scan — every u32 (LE & BE) at every byte offset of the raw DCB.
    //    Catches a stored location id anywhere (instance blobs, pools, anything).
    let mut lit: HashSet<u32> = HashSet::new();
    if raw.len() >= 4 {
        for w in raw.windows(4) {
            lit.insert(u32::from_le_bytes([w[0], w[1], w[2], w[3]]));
            lit.insert(u32::from_be_bytes([w[0], w[1], w[2], w[3]]));
        }
    }
    eprintln!("distinct u32 windows in raw DCB: {}", lit.len());
    for t in &targets {
        if lit.contains(t) {
            hits.push(format!(
                "LITERAL u32 present in DCB bytes: {t}  [{}]",
                lbl(*t)
            ));
        }
    }

    // 2) GUID pool
    let gd = db.raw_pool_data(PoolType::Guid);
    let mut nguid = 0;
    for ch in gd.chunks_exact(16) {
        nguid += 1;
        let mut b = [0u8; 16];
        b.copy_from_slice(ch);
        let g = Guid::from_bytes(b);
        let cc = class_crc(&g);
        if targets.contains(&cc) {
            hits.push(format!("class_crc(GUID {g}) = {cc}  [{}]", lbl(cc)));
        }
        let mut rb = b;
        rb.reverse();
        for bytes in [&b[..], &rb[..]] {
            for (fnname, t) in [("crc32c", crc(bytes, &c_tbl)), ("ieee", crc(bytes, &i_tbl))] {
                if targets.contains(&t) {
                    hits.push(format!("{fnname}(GUID bytes {g}) = {t}  [{}]", lbl(t)));
                }
            }
        }
    }
    eprintln!("GUID pool: {nguid}");

    // 3) string tables
    let mut nstr = 0;
    for tbl_bytes in [db.raw_string_table_1(), db.raw_string_table_2()] {
        for s in tbl_bytes.split(|&b| b == 0) {
            if s.is_empty() {
                continue;
            }
            nstr += 1;
            let lower: Vec<u8> = s.iter().map(|b| b.to_ascii_lowercase()).collect();
            for bytes in [s, &lower[..]] {
                for (fnname, t) in [("crc32c", crc(bytes, &c_tbl)), ("ieee", crc(bytes, &i_tbl))] {
                    if targets.contains(&t) {
                        let txt = String::from_utf8_lossy(s);
                        hits.push(format!("{fnname}(string {txt:?}) = {t}  [{}]", lbl(t)));
                    }
                }
            }
        }
    }
    eprintln!("strings: {nstr}");

    println!("\n==== HITS: {} ====", hits.len());
    let mut seen = HashSet::new();
    for h in &hits {
        if seen.insert(h.clone()) {
            println!("  {h}");
        }
    }
    Ok(())
}
