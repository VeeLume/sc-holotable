//! sc-bench — comprehensive sc-extract benchmark binary.
//!
//! Exercises the full sc-extract API surface against a real Star Citizen
//! installation: parse, indices, tag tree, manufacturers, display names,
//! locale, reference graph, filter predicates, raw svarog queries, and
//! snapshot round-trip.
//!
//! Output is structured JSON on stdout (for the bench script) or a
//! human-readable summary (with `--human`).
//!
//! Usage:
//!
//! ```bash
//! # JSON output (for bench script consumption):
//! cargo run -p sc-bench --release --features full
//!
//! # Human-readable output (for manual runs):
//! cargo run -p sc-bench --release --features full -- --human
//!
//! # Include reference graph:
//! cargo run -p sc-bench --release --features full -- --all --human
//!
//! # Explicit P4K path:
//! cargo run -p sc-bench --release -- "C:/Games/StarCitizen/LIVE/Data.p4k" --human
//! ```

mod output;
mod runner;

use runner::RunConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let human = args.iter().any(|a| a == "--human");
    let include_graph = args.iter().any(|a| a == "--all");
    let skip_assets = args.iter().any(|a| a == "--no-assets");
    let p4k_path = args.iter().find(|a| !a.starts_with('-')).map(Into::into);

    // In JSON mode, suppress tracing entirely so stdout contains only the
    // JSON object. In human mode, enable tracing at info level (unless
    // RUST_LOG overrides).
    if human {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("off")),
            )
            .init();
    }

    let config = RunConfig {
        p4k_path,
        include_graph,
        skip_assets,
    };

    let result = runner::run(&config)?;

    if human {
        result.print_human();
    } else {
        println!("{}", serde_json::to_string(&result)?);
    }

    Ok(())
}
