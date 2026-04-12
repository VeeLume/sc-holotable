//! `sc-snapshot` — produce a compressed `ExtractedData` snapshot from a
//! Star Citizen install.
//!
//! Separate from `sc-generator`: `sc-generator` regenerates Rust bindings
//! from the DCB schema at development time, while `sc-snapshot` is a
//! runtime-ish tool that walks real game data and writes a portable
//! snapshot (msgpack + zstd) for consumers to load later.
//!
//! Depending on `sc-installs` here — which `sc-extract` deliberately does
//! not — lets the tool auto-discover the install, read `build_manifest.id`,
//! and stamp the snapshot with real `game_version` / `build_id` values
//! instead of the `"unknown"` placeholders `parse_from_p4k` produces on its
//! own.
//!
//! ```text
//! # Auto-discover primary install, write to snapshot.bin
//! cargo run -p sc-snapshot -- -o snapshot.bin
//!
//! # Explicit install root
//! cargo run -p sc-snapshot -- --install "C:/Games/StarCitizen/LIVE" -o live.snap
//!
//! # Explicit p4k path (no sc-installs metadata — version/build_id stay "unknown")
//! cargo run -p sc-snapshot -- --p4k "C:/Games/StarCitizen/LIVE/Data.p4k" -o live.snap
//!
//! # Parse and report but don't write
//! cargo run -p sc-snapshot -- --check
//! ```

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use sc_extract::{parse_from_p4k, ExtractedData};
use sc_installs::Installation;

#[derive(Debug)]
enum Source {
    /// Auto-discover the primary install via `sc_installs::discover_primary`.
    Auto,
    /// An install root that sc-installs can read metadata from.
    Install(PathBuf),
    /// A raw P4K path — no metadata stamping.
    P4k(PathBuf),
}

#[derive(Debug)]
struct Options {
    source: Source,
    out: Option<PathBuf>,
    check_only: bool,
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let options = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("sc-snapshot: {e}");
            eprintln!();
            print_usage();
            return ExitCode::from(2);
        }
    };

    match run(&options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            tracing::error!("snapshot failed: {e}");
            ExitCode::FAILURE
        }
    }
}

// ── CLI parsing ────────────────────────────────────────────────────────────

fn parse_args() -> Result<Options, String> {
    let mut args = std::env::args().skip(1);
    let mut install: Option<PathBuf> = None;
    let mut p4k: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut check_only = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--install" => {
                let v = args.next().ok_or("--install requires a value")?;
                install = Some(PathBuf::from(v));
            }
            "--p4k" => {
                let v = args.next().ok_or("--p4k requires a value")?;
                p4k = Some(PathBuf::from(v));
            }
            "-o" | "--out" => {
                let v = args.next().ok_or("--out requires a value")?;
                out = Some(PathBuf::from(v));
            }
            "--check" => check_only = true,
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if install.is_some() && p4k.is_some() {
        return Err("--install and --p4k are mutually exclusive".into());
    }

    let source = match (install, p4k) {
        (Some(i), None) => Source::Install(i),
        (None, Some(p)) => Source::P4k(p),
        (None, None) => Source::Auto,
        (Some(_), Some(_)) => unreachable!("guarded above"),
    };

    if out.is_none() && !check_only {
        return Err("-o/--out is required unless --check is passed".into());
    }

    Ok(Options {
        source,
        out,
        check_only,
    })
}

fn print_usage() {
    eprintln!("Usage: sc-snapshot [--install <path> | --p4k <path>] -o <snapshot> [--check]");
    eprintln!();
    eprintln!("Sources (pick at most one — auto-discover is the default):");
    eprintln!("  --install <path>   Install root (e.g. C:/Games/StarCitizen/LIVE).");
    eprintln!("                     Reads build_manifest.id to stamp the snapshot with");
    eprintln!("                     real game_version / build_id.");
    eprintln!("  --p4k <path>       Raw Data.p4k path. Skips install metadata — the");
    eprintln!("                     snapshot's game_version / build_id stay \"unknown\".");
    eprintln!("  (none)             Auto-discover the primary install via sc-installs.");
    eprintln!();
    eprintln!("Output:");
    eprintln!("  -o, --out <path>   Snapshot output path (required unless --check).");
    eprintln!("  --check            Parse and report but don't write a snapshot.");
    eprintln!();
    eprintln!("  -h, --help         Show this message.");
}

// ── Orchestration ──────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("install discovery failed")]
    Discovery(#[source] sc_installs::Error),

    #[error("install at {path} is not usable")]
    InvalidInstall {
        path: PathBuf,
        #[source]
        source: sc_installs::Error,
    },

    #[error("sc-extract failed")]
    Extract(#[from] sc_extract::Error),
}

fn run(options: &Options) -> Result<(), Error> {
    let start = Instant::now();

    // Resolve the p4k path and (optionally) the Installation so we can
    // stamp the snapshot with real metadata.
    let (p4k_path, install) = resolve_source(&options.source)?;

    tracing::info!(path = %p4k_path.display(), "running sc-extract::parse_from_p4k");
    let (mut data, _assets) = parse_from_p4k(&p4k_path)?;

    if let Some(inst) = install.as_ref() {
        data.game_version = inst.manifest.version.clone();
        data.build_id = inst.manifest.build_id.clone();
        tracing::info!(
            game_version = %data.game_version,
            build_id = %data.build_id,
            "stamped snapshot with install metadata"
        );
    }

    print_summary(&data, start.elapsed().as_secs_f64());

    if options.check_only {
        tracing::info!("--check mode: skipping snapshot write");
        return Ok(());
    }

    let out = options
        .out
        .as_ref()
        .expect("validated in parse_args when not --check");
    let save_start = Instant::now();
    data.save_snapshot(out)?;
    let save_secs = save_start.elapsed().as_secs_f64();

    let size_mb = std::fs::metadata(out)
        .map(|m| m.len() as f64 / 1_000_000.0)
        .unwrap_or(0.0);

    println!();
    println!("Snapshot written");
    println!("  path      : {}", out.display());
    println!("  size      : {size_mb:.2} MB");
    println!("  save time : {save_secs:.2}s");

    Ok(())
}

/// Turn a `--install` / `--p4k` / auto source into a concrete p4k path and,
/// when possible, an `Installation` for metadata. Auto mode and `--install`
/// both yield an `Installation`; `--p4k` yields just the raw path.
fn resolve_source(source: &Source) -> Result<(PathBuf, Option<Installation>), Error> {
    match source {
        Source::Auto => {
            let install = sc_installs::discover_primary().map_err(Error::Discovery)?;
            tracing::info!(
                channel = ?install.channel,
                root = %install.root.display(),
                "auto-discovered install"
            );
            let p4k = install.data_p4k();
            Ok((p4k, Some(install)))
        }
        Source::Install(root) => {
            let install =
                Installation::from_root(root.clone()).map_err(|source| Error::InvalidInstall {
                    path: root.clone(),
                    source,
                })?;
            tracing::info!(
                channel = ?install.channel,
                root = %install.root.display(),
                "using explicit install"
            );
            let p4k = install.data_p4k();
            Ok((p4k, Some(install)))
        }
        Source::P4k(path) => {
            tracing::info!(
                path = %path.display(),
                "using raw p4k path (no install metadata)"
            );
            Ok((path.clone(), None))
        }
    }
}

fn print_summary(data: &ExtractedData, parse_secs: f64) {
    println!();
    println!("ExtractedData");
    println!("  schema version : {}", data.schema_version);
    println!("  game version   : {}", data.game_version);
    println!("  build id       : {}", data.build_id);
    println!("  extracted at   : {}", data.extracted_at);
    println!("  records        : {}", data.records.len());
    println!("  locale entries : {}", data.locale.len());
    println!("  manufacturers  : {}", data.manufacturers.len());
    println!("  display names  : {}", data.display_names.len());
    println!("  tag nodes      : {}", data.tag_tree.len());
    println!("  graph edges    : {}", data.graph.edge_count());
    println!("  parse time     : {parse_secs:.2}s");
}
