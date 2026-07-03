//! Resolve mission location CRCs against a captured EntityGraph container graph.
//!
//! TEST B: is a mission DataSet `location_id` == `class_crc(ContainerNode.guid)`?
//! TEST A: is `ContainerNode.socpak_file_path_hash` a crc32 of the socpak path?
//! If both hold: location CRC -> container node -> socpak path -> (name).
//!
//! ```bash
//! cargo run -p sc-locations --release --example container_resolve -- \
//!   <cnodes.tsv> <loc_targets.txt> [loc_resolved.tsv]
//! ```
//! cnodes.tsv: guid\tgeid\tsocpak_content_hash\tsocpak_file_path_hash (one node/line)
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let nodes_path = &args[0];
    let targets_path = &args[1];

    // class_crc(node.guid) -> guid ; plus the socpak-hash sets and a fph->guid map.
    let mut by_crc: HashMap<u32, String> = HashMap::new();
    let mut fph_set: HashSet<u32> = HashSet::new();
    let mut content_set: HashSet<u32> = HashSet::new();
    let mut fph_to_guid: HashMap<u32, String> = HashMap::new();
    for line in std::fs::read_to_string(nodes_path)?.lines() {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 4 {
            continue;
        }
        let Ok(g) = Guid::from_str(c[0]) else {
            continue;
        };
        let ch: u32 = c[2].parse().unwrap_or(0);
        let fph: u32 = c[3].parse().unwrap_or(0);
        by_crc
            .entry(class_crc(&g))
            .or_insert_with(|| c[0].to_string());
        if fph != 0 {
            fph_set.insert(fph);
            fph_to_guid.entry(fph).or_insert_with(|| c[0].to_string());
        }
        if ch != 0 {
            content_set.insert(ch);
        }
    }
    eprintln!(
        "nodes: {} distinct class_crc, {} distinct fph, {} distinct content_hash",
        by_crc.len(),
        fph_set.len(),
        content_set.len()
    );

    // targets + optional names
    let targets: Vec<u32> = std::fs::read_to_string(targets_path)?
        .lines()
        .filter_map(|l| l.split('\t').next()?.trim().parse().ok())
        .collect();
    let names: HashMap<u32, String> = args
        .get(2)
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|t| {
            t.lines()
                .filter_map(|l| {
                    let mut it = l.split('\t');
                    let crc = it.next()?.trim().parse().ok()?;
                    Some((crc, it.next().unwrap_or("").to_string()))
                })
                .collect()
        })
        .unwrap_or_default();

    // socpak path -> crc32c(lower, '\'->'/', 'data/' stripped) — the proven fph fn.
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let c_tbl = table(0x82F6_3B78); // crc32c (Castagnoli)
    let norm = |p: &str| {
        let l = p.replace('\\', "/").to_lowercase();
        l.strip_prefix("data/").unwrap_or(&l).to_string()
    };
    let mut socpak_by_fph: HashMap<u32, String> = HashMap::new();
    let mut npaths = 0;
    for e in assets.find(|n| n.to_ascii_lowercase().ends_with(".socpak")) {
        npaths += 1;
        let h = crc(norm(&e.name).as_bytes(), &c_tbl);
        socpak_by_fph.entry(h).or_insert_with(|| e.name.to_string());
    }
    eprintln!(
        "socpak paths: {npaths} (crc32c index {})",
        socpak_by_fph.len()
    );

    // ── Classify each location CRC against the candidate namespaces ──────────
    println!("\n== location CRC classification ==");
    let mut by_class = 0;
    let mut by_fph = 0;
    let mut by_content = 0;
    let mut by_socpak_path = 0;
    for t in &targets {
        let nm = names.get(t).map(|s| s.as_str()).unwrap_or("?");
        let mut how: Vec<String> = Vec::new();
        if by_crc.contains_key(t) {
            by_class += 1;
            how.push("class_crc(node.guid)".into());
        }
        if fph_set.contains(t) {
            by_fph += 1;
            how.push("node.socpak_file_path_hash".into());
        }
        if content_set.contains(t) {
            by_content += 1;
            how.push("node.socpak_content_hash".into());
        }
        if let Some(sp) = socpak_by_fph.get(t) {
            by_socpak_path += 1;
            how.push(format!("crc32c(socpak path) = {sp}"));
        }
        if !how.is_empty() {
            println!("  {t:<11} [{nm}] <- {}", how.join("  |  "));
        }
    }
    println!(
        "\nsummary over {} targets: class_crc(guid)={by_class}  fph={by_fph}  content_hash={by_content}  socpak-path-crc32c={by_socpak_path}",
        targets.len()
    );
    Ok(())
}
