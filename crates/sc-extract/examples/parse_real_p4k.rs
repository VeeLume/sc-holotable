//! End-to-end smoke test against a real Star Citizen installation.
//!
//! Usage:
//!
//! ```bash
//! # Auto-discover primary installation (standard config, no graph):
//! cargo run -p sc-extract --profile dev-opt --example parse_real_p4k
//!
//! # Explicit path:
//! cargo run -p sc-extract --profile dev-opt --example parse_real_p4k -- "C:/Games/StarCitizen/LIVE/Data.p4k"
//!
//! # Full extraction including reference graph:
//! cargo run -p sc-extract --profile dev-opt --example parse_real_p4k -- --all
//! ```

use std::path::PathBuf;
use std::time::Instant;

use sc_extract::{ExtractConfig, ExtractedData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let use_all = args.iter().any(|a| a == "--all");
    let explicit_path: Option<PathBuf> = args.iter().find(|a| !a.starts_with('-')).map(Into::into);

    let config = if use_all {
        println!("-> using ExtractConfig::all() (includes reference graph)");
        ExtractConfig::all()
    } else {
        println!("-> using ExtractConfig::standard() (graph skipped)");
        ExtractConfig::standard()
    };

    let t0 = Instant::now();

    let (data, assets) = if let Some(p4k_path) = &explicit_path {
        println!("-> parsing {}", p4k_path.display());
        sc_extract::parse_with_config(p4k_path, &config)?
    } else {
        let install = sc_installs::discover_primary()?;
        println!(
            "-> found {} v{} at {}",
            install.channel,
            install.short_version(),
            install.root.display()
        );
        sc_extract::parse_from_install_with(&install, &config)?
    };

    let parse_secs = t0.elapsed().as_secs_f64();

    println!();
    println!("ExtractedData summary");
    println!("---------------------");
    println!("  schema_version : {}", data.schema_version);
    println!("  game_version   : {}", data.game_version);
    println!("  build_id       : {}", data.build_id);
    println!("  extracted_at   : {}", data.extracted_at);
    println!("  records        : {}", data.records.len());
    println!("  locale entries : {}", data.locale.len());
    println!("  manufacturers  : {}", data.manufacturers.len());
    println!("  display names  : {}", data.display_names.len());
    println!("  tag nodes      : {}", data.tag_tree.len());
    println!("  graph edges    : {}", data.graph.edge_count());
    println!("  parse time     : {parse_secs:.2}s");

    // Snapshot round-trip
    println!();
    println!("Snapshot round-trip");
    println!("-------------------");
    let snapshot_path = std::env::temp_dir().join("sc_extract_smoke.snap");
    let t1 = Instant::now();
    data.save_snapshot(&snapshot_path)?;
    let save_secs = t1.elapsed().as_secs_f64();
    let size = std::fs::metadata(&snapshot_path)?.len();
    println!("  snapshot file  : {}", snapshot_path.display());
    println!("  size on disk   : {:.2} MB", size as f64 / 1_000_000.0);
    println!("  save time      : {save_secs:.2}s");

    let t2 = Instant::now();
    let loaded = ExtractedData::load_snapshot(&snapshot_path)?;
    let load_secs = t2.elapsed().as_secs_f64();
    println!("  load time      : {load_secs:.2}s");
    println!("  loaded records : {}", loaded.records.len());
    assert_eq!(loaded.records.len(), data.records.len());

    // Asset access smoke check
    println!();
    println!("Asset access");
    println!("------------");
    match assets.default_profile_xml() {
        Ok(bytes) => println!("  defaultProfile : {} bytes", bytes.len()),
        Err(e) => println!("  defaultProfile : not found ({e})"),
    }

    println!();
    println!("Done.");
    Ok(())
}
