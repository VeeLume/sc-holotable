//! Enumerate non-socpak file paths by top-level directory prefix, for chosen
//! extensions. Used to find standalone record/container XML/eco/etc. that
//! could hold location/object-container GUIDs not in Game2.dcb.
//!
//! ```bash
//! cargo run -p sc-locations --release --example xml_paths -- xml eco
//! ```
use sc_extract::AssetSource;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exts: Vec<String> = {
        let a: Vec<String> = std::env::args().skip(1).map(|s| s.to_lowercase()).collect();
        if a.is_empty() { vec!["xml".into()] } else { a }
    };
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let archive = assets.archive().expect("live");
    // prefix = first 3 path segments
    let mut prefixes: BTreeMap<String, (u64, Vec<String>)> = BTreeMap::new();
    for e in archive.iter() {
        let lname = e.name.to_ascii_lowercase();
        let ext = lname.rsplit('.').next().unwrap_or("");
        if !exts.iter().any(|x| x == ext) {
            continue;
        }
        let norm = e.name.replace('\\', "/");
        let segs: Vec<&str> = norm.split('/').collect();
        let depth = segs.len().min(3);
        let prefix = segs[..depth.saturating_sub(1)].join("/");
        let slot = prefixes.entry(prefix).or_insert((0, Vec::new()));
        slot.0 += 1;
        if slot.1.len() < 3 {
            slot.1.push(norm.clone());
        }
    }
    let mut v: Vec<_> = prefixes.into_iter().collect();
    v.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    for (prefix, (c, samples)) in v {
        println!("{c:>7}  {prefix}/");
        for s in samples {
            println!("           e.g. {s}");
        }
    }
    Ok(())
}
