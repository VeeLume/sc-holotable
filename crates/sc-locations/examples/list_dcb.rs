//! List candidate DataForge / data files in the p4k (anything not a socpak/asset
//! that could hold object-container records): .dcb, .dcb-like, .xml manifests at
//! root, etc. Used to find a *second* DataForge beyond Game2.dcb.
//!
//! ```bash
//! cargo run -p sc-locations --release --example list_dcb -- [ext1 ext2 ...]
//! ```
use sc_extract::AssetSource;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exts: Vec<String> = {
        let a: Vec<String> = std::env::args().skip(1).collect();
        if a.is_empty() {
            vec![".dcb".into(), ".dpk".into(), ".dco".into()]
        } else {
            a.into_iter().map(|s| s.to_lowercase()).collect()
        }
    };
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    for ext in &exts {
        let hits: Vec<String> = assets
            .find(|n| n.to_ascii_lowercase().ends_with(ext.as_str()))
            .map(|e| format!("{}  ({} bytes)", e.name, e.uncompressed_size))
            .collect();
        println!("==== {ext}: {} ====", hits.len());
        for h in hits.iter().take(60) {
            println!("  {h}");
        }
    }
    Ok(())
}
