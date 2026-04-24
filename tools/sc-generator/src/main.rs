//! `sc-generator` — DataCore schema → Rust codegen tool.
//!
//! This is the **offline** generator that walks a Star Citizen
//! `Data.p4k`, discovers every DataCore struct and enum, and emits the
//! matching Rust types into `crates/sc-extract-generated/src/generated/`.
//!
//! Developer workflow: run once when a new game patch lands, commit the
//! regenerated output, review the diff, fix any call sites that broke.
//!
//! ```text
//! cargo run -p sc-generator --release
//! cargo run -p sc-generator --release -- --p4k "C:\Games\StarCitizen\LIVE\Data.p4k"
//! ```
//!
//! Without `--p4k`, the generator auto-discovers the primary LIVE install
//! via `sc-installs` and uses its `Data.p4k`.
//!
//! Optional flags:
//!
//! - `--p4k <path>`       — override auto-discovery with an explicit P4K path
//! - `--out-dir <path>`   — override the default `crates/sc-extract-generated/src/generated/`
//! - `--check`            — parse everything but don't write files; for CI
//!
//! On start, the generator prints a machine-parseable line:
//!
//! ```text
//! sc-generator: sc_version=<full version> channel=<LIVE/PTU/...> p4k=<path>
//! ```
//!
//! Scripts can parse this to record version metadata (e.g. for git tags).
//!
//! Analysis/diagnostic subcommands previously available (`--dump-paths`,
//! `--dump-features`, `--check-polymorphism`, etc.) have been removed.
//!
//! See `docs/codegen.md` for the full design.

use std::path::PathBuf;
use std::process::ExitCode;

mod closure;
mod emit;
mod features;
mod naming;
mod pipeline;

use pipeline::{RunOptions, run, run_dump_refs};

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
            eprintln!("sc-generator: {e}");
            eprintln!();
            print_usage();
            return ExitCode::from(2);
        }
    };

    if options.dump_refs {
        return match run_dump_refs(&options) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                tracing::error!("dump-refs failed: {e}");
                ExitCode::FAILURE
            }
        };
    }

    match run(&options) {
        Ok(summary) => {
            tracing::info!(
                struct_count = summary.struct_count,
                enum_count = summary.enum_count,
                output = %summary.out_dir.display(),
                "generation complete"
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            tracing::error!("generation failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args() -> Result<RunOptions, String> {
    let mut args = std::env::args().skip(1);
    let mut p4k: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut check_only = false;
    let mut dump_refs = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--p4k" => {
                let v = args.next().ok_or("--p4k requires a value")?;
                p4k = Some(PathBuf::from(v));
            }
            "--out-dir" => {
                let v = args.next().ok_or("--out-dir requires a value")?;
                out_dir = Some(PathBuf::from(v));
            }
            "--check" => check_only = true,
            "--dump-refs" => dump_refs = true,
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let p4k = match p4k {
        Some(path) => path,
        None => autodiscover_p4k()?,
    };
    let out_dir = out_dir.unwrap_or_else(default_out_dir);

    Ok(RunOptions {
        p4k,
        out_dir,
        check_only,
        dump_refs,
    })
}

/// Locate the primary (LIVE) Star Citizen install via `sc-installs` and
/// return its `Data.p4k` path. Prints a machine-parseable identity line to
/// stdout so scripts can capture the version/channel/path without having
/// to re-run discovery themselves.
fn autodiscover_p4k() -> Result<PathBuf, String> {
    let install = sc_installs::discover_primary()
        .map_err(|e| format!("auto-discovery failed: {e}. Pass --p4k <path> explicitly."))?;
    let p4k = install.data_p4k();
    // Machine-parseable: one line, stable key=value, no tracing envelope.
    println!(
        "sc-generator: sc_version={} channel={} p4k={}",
        install.manifest.version,
        install.channel,
        p4k.display(),
    );
    Ok(p4k)
}

/// Default output directory relative to the current working directory.
fn default_out_dir() -> PathBuf {
    PathBuf::from("crates/sc-extract-generated/src/generated")
}

fn print_usage() {
    eprintln!("Usage: sc-generator [--p4k <path>] [--out-dir <path>] [--check]");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  --p4k <path>       Path to Data.p4k to generate from.");
    eprintln!("                     When omitted, auto-discovered via sc-installs");
    eprintln!("                     (primary / LIVE channel).");
    eprintln!("  --out-dir <path>   Where to write generated files");
    eprintln!("                     (default: crates/sc-extract-generated/src/generated)");
    eprintln!("  --check            Parse and generate in-memory but don't write files");
    eprintln!("  -h, --help         Show this message");
}
