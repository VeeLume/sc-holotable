//! Probe: every launcher run in the real launcher log, and the patch state of
//! each discovered install.
//!
//! Re-proves the claims in the `patch_state` module docs: every run that
//! wrote files either completed or is listed here as not completed, and runs
//! closed as `Interrupted` are the ones a launcher restart or a newer run cut
//! off.
//!
//! ```text
//! cargo run -p sc-discovery --example patch_state
//! ```

use std::collections::BTreeMap;

use sc_discovery::{discover, launcher_log_path, launcher_runs};

fn main() -> Result<(), sc_discovery::Error> {
    let runs = launcher_runs(&launcher_log_path())?;
    let mut tally: BTreeMap<String, usize> = BTreeMap::new();
    for run in &runs {
        let key = format!(
            "{:?} / {} / wrote_files={}",
            run.kind,
            run.outcome.map_or("OPEN".to_string(), |o| format!("{o:?}")),
            run.wrote_files
        );
        *tally.entry(key).or_default() += 1;
    }
    println!("{} runs in {}", runs.len(), launcher_log_path().display());
    for (key, n) in &tally {
        println!("  {n:>3}  {key}");
    }

    println!("\nruns that did not complete:");
    for run in runs.iter().filter(|r| {
        r.outcome
            .is_none_or(|o| o != sc_discovery::RunOutcome::Completed)
    }) {
        println!(
            "  {}  {:<5} {:?} {} wrote_files={} paused={} -> {:?}",
            run.started_at,
            run.channel,
            run.kind,
            run.version_label,
            run.wrote_files,
            run.paused,
            run.outcome
        );
    }

    println!("\npatch state per install:");
    for install in discover()? {
        println!(
            "  {:<5} {}  manifest changelist {:?}, store label {:?}\n        {:?}",
            install.channel,
            install.root.display(),
            install.manifest.changelist,
            install.launcher_version_label,
            install.patch_state()?
        );
    }
    Ok(())
}
