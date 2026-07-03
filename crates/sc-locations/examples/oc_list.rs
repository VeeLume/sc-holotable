//! List p4k object-container (.socpak) entries whose path matches any given
//! substring — to test whether the log's `[bracket]` ids are object-container
//! paths (the marker's real home), distinct from StarMapObject records.
//!
//! ```bash
//! cargo run -p sc-locations --release --example oc_list -- hdmsryder sunsetmesa bajinipoint spaceport
//! ```
use sc_extract::AssetSource;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let needles: Vec<String> = std::env::args().skip(1).map(|s| s.to_lowercase()).collect();
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;

    let mut total = 0usize;
    let hits: Vec<String> = assets
        .find(|name| {
            let n = name.to_ascii_lowercase();
            n.ends_with(".socpak") && needles.iter().any(|s| n.contains(s.as_str()))
        })
        .map(|e| e.name.to_string())
        .collect();
    for h in &hits {
        total += 1;
        println!("{h}");
    }
    eprintln!("\nmatched {total} .socpak entries for needles {needles:?}");

    // Also count total socpaks + how many are 'system' OCs for context.
    let all: usize = assets
        .find(|n| n.to_ascii_lowercase().ends_with(".socpak"))
        .count();
    let sys: usize = assets
        .find(|n| n.to_ascii_lowercase().ends_with("system.socpak"))
        .count();
    eprintln!("total .socpak in p4k: {all} (of which *system.socpak: {sys})");
    Ok(())
}
