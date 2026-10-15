//! Whether the RSI Launcher is changing an install right now, or left one
//! half-changed.
//!
//! ## What the launcher does to an install
//!
//! Read from RSI Launcher 2.18.1's own source (`app.asar`) and checked
//! against every run in a real launcher log (April to October 2026).
//!
//! An install, update or verify is one **run** of the launcher's install
//! pipeline. Its last phase, the patcher, never skips and steps through
//! `open-p4k` → file lists → `update-loose-files` → `update-p4k-structure` →
//! `update-files-inside-p4k` (→ verification steps on a verify).
//!
//! - `build_manifest.id` is a loose file. It names the **new** build about two
//!   minutes into a run, while `Data.p4k` is still rewritten **in place** for
//!   another ten to twenty minutes. Nothing before `update-loose-files`
//!   writes to the install.
//! - The launcher store's `library.installed[].channels[].version` and
//!   `versionLabel` move only when a run succeeds; the store has no
//!   in-progress state at all.
//! - A run that fails, is cancelled or dies after `update-loose-files`
//!   leaves files from two builds behind. Pausing the patcher has ended in
//!   `BaseP4kCorruptedError` twice; only a Verify repaired the install.
//!
//! [`Installation::patch_state`] combines both sources: the log tells whether
//! a run is open and how the last one ended, the store whether the build on
//! disk is the one the launcher last finished. The store check also covers a
//! wiped or rotated log.
//!
//! Only the log format of launcher 2.14 and later (`[Pipeline]` /
//! `[PatcherPhase]` lines) yields runs; older `[Installer]` lines are ignored.
//! A launcher that dies mid-run leaves its run open until the launcher starts
//! again, so the run reads as [`PatchState::Patching`] until then.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::channel::Channel;
use crate::error::{Error, Result};
use crate::installation::Installation;
use crate::launcher_store::{StoreSnapshot, read_launcher_snapshot};
use crate::log_parser::{launcher_log_path, normalise_root, unescape_path};
use crate::manifest::read_build_manifest;

/// `[Pipeline] Installing Star Citizen LIVE 4.10.2-live.12913470 at
/// C:\\Games\\StarCitizen (type: update, forceDP: false)`. The path is the
/// library directory; the channel directory still has to be appended.
static RUN_START_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\[Pipeline\] (?:Installing|Verifying) Star Citizen (\S+) (\S+) at (.+?) \(type: (\w+), forceDP",
    )
    .expect("run-start regex is valid")
});

/// The `"t":"2026-10-13 17:47:20.295"` stamp every log entry opens with.
static TIMESTAMP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\{ "t":"([^"]+)""#).expect("timestamp regex is valid"));

/// What a run was asked to do, from the `type:` of its start line.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum RunKind {
    /// A first install of the channel.
    Install,
    /// A delta update to a new build.
    Update,
    /// A verify (and repair) of the installed build.
    Verify,
    /// A `type:` this crate does not know yet, verbatim.
    Unrecognized(String),
}

impl RunKind {
    fn from_log(s: &str) -> Self {
        match s {
            "install" => Self::Install,
            "update" => Self::Update,
            "verify" => Self::Verify,
            other => Self::Unrecognized(other.to_string()),
        }
    }
}

/// How a run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum RunOutcome {
    /// The patcher finished; the launcher records the build as installed.
    Completed,
    /// The pipeline logged an error.
    Failed,
    /// The user or the launcher cancelled the run.
    Cancelled,
    /// The launcher started again, or began another run, while this one was
    /// open.
    Interrupted,
}

/// One install, update or verify run, read from the launcher log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LauncherRun {
    pub channel: Channel,
    pub kind: RunKind,
    /// The build the run installs, e.g. `"4.10.2-live.12913470"`.
    pub version_label: String,
    /// Install root the run works on, e.g. `C:\Games\StarCitizen\LIVE`.
    pub root: PathBuf,
    /// When the run started, as the log stamps it (launcher-local time,
    /// `2026-10-13 17:47:20.295`).
    pub started_at: String,
    /// The patcher reached its first write step (`update-loose-files`).
    /// A run that ends before that has left the install untouched.
    pub wrote_files: bool,
    /// The run is paused. Only meaningful while it is open.
    pub paused: bool,
    /// `None` while the run is open.
    pub outcome: Option<RunOutcome>,
}

impl LauncherRun {
    /// The run has not ended.
    pub fn is_open(&self) -> bool {
        self.outcome.is_none()
    }
}

/// Every install, update and verify run in a launcher log, oldest first.
///
/// The launcher runs one pipeline at a time, so lines between a start line
/// and the next one belong to that run.
///
/// # Errors
///
/// [`Error::LauncherLogNotFound`] / [`Error::LauncherLogUnreadable`].
pub fn launcher_runs(log_path: &Path) -> Result<Vec<LauncherRun>> {
    let content = std::fs::read_to_string(log_path).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            Error::LauncherLogNotFound(log_path.to_path_buf())
        } else {
            Error::LauncherLogUnreadable {
                path: log_path.to_path_buf(),
                source,
            }
        }
    })?;
    Ok(parse_runs(&content))
}

fn parse_runs(content: &str) -> Vec<LauncherRun> {
    let mut runs: Vec<LauncherRun> = Vec::new();
    for line in content.lines() {
        if let Some(caps) = RUN_START_RE.captures(line) {
            close_open(&mut runs, RunOutcome::Interrupted);
            let Some(channel) = Channel::from_str_loose(&caps[1]) else {
                continue;
            };
            let started_at = TIMESTAMP_RE
                .captures(line)
                .map(|t| t[1].to_string())
                .unwrap_or_default();
            runs.push(LauncherRun {
                channel,
                kind: RunKind::from_log(&caps[4]),
                version_label: caps[2].to_string(),
                root: normalise_root(channel, &unescape_path(&caps[3])),
                started_at,
                wrote_files: false,
                paused: false,
                outcome: None,
            });
            continue;
        }
        // A new launcher process starts with a fresh installer; whatever
        // run the previous one had open is over.
        if line.contains("[ConnectivityService] Registered IPC listeners") {
            close_open(&mut runs, RunOutcome::Interrupted);
            continue;
        }
        let Some(run) = runs.last_mut().filter(|r| r.is_open()) else {
            continue;
        };
        if line.contains("installer@update-loose-files-start") {
            run.wrote_files = true;
        } else if line.contains("[PatcherPhase] Delta update completed (") {
            run.outcome = Some(RunOutcome::Completed);
        } else if line.contains("[Pipeline] Error: ") {
            run.outcome = Some(RunOutcome::Failed);
        } else if line.contains("[PatcherPhase] Delta update aborted (")
            || line.contains(" aborted due to cancellation.")
            || line.contains(" aborted for unknown reason")
        {
            run.outcome = Some(RunOutcome::Cancelled);
        } else if line.contains("[FlowControl] Pausing phase:")
            || line.contains(" aborted due to pause")
        {
            run.paused = true;
        } else if line.contains("[FlowControl] Resuming phase:") {
            run.paused = false;
        }
    }
    runs
}

fn close_open(runs: &mut [LauncherRun], outcome: RunOutcome) {
    if let Some(run) = runs.last_mut().filter(|r| r.is_open()) {
        run.outcome = Some(outcome);
    }
}

/// Whether an install is safe to read from or write to, as far as the
/// launcher is concerned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum PatchState {
    /// No run is open on the install, and nothing says the files on disk are
    /// anything but the build the launcher last finished.
    Settled,
    /// A run on the install is open (possibly paused): `Data.p4k` and the
    /// loose files can change at any moment.
    Patching(LauncherRun),
    /// No run is open, but the files on disk are not a build the launcher
    /// finished. The launcher shows the channel as needing an update or a
    /// verify.
    Unsettled(Unsettled),
}

impl PatchState {
    pub fn is_settled(&self) -> bool {
        matches!(self, Self::Settled)
    }
}

/// Why an install is [`PatchState::Unsettled`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum Unsettled {
    /// The last run that wrote to the install did not complete. Its files
    /// are a mix of two builds and `Data.p4k` may be corrupt.
    RunDidNotComplete(LauncherRun),
    /// The launcher store records another build for this channel than
    /// `build_manifest.id` names. Either a run stopped partway (the manifest
    /// moves early, the store only on success), or the folder came from
    /// another channel and the launcher has not patched it yet.
    NotTheRecordedBuild { recorded: u64, on_disk: u64 },
}

impl Installation {
    /// Ask the launcher log, the launcher store and `build_manifest.id`, all
    /// read afresh, whether the launcher is patching this install or left it
    /// half-patched.
    ///
    /// Check before reading `Data.p4k` or writing into the install, and
    /// again after a long read: a run can start at any time. Without a
    /// readable store the verdict rests on the log alone.
    ///
    /// ```no_run
    /// use sc_discovery::{PatchState, discover_default};
    ///
    /// let install = discover_default()?;
    /// match install.patch_state()? {
    ///     PatchState::Settled => { /* read Data.p4k */ }
    ///     PatchState::Patching(run) => println!("launcher is installing {}", run.version_label),
    ///     other => println!("install not usable until the launcher repairs it: {other:?}"),
    /// }
    /// # Ok::<(), sc_discovery::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`Error::LauncherLogNotFound`] / [`Error::LauncherLogUnreadable`]:
    /// without the log an open run cannot be ruled out.
    pub fn patch_state(&self) -> Result<PatchState> {
        let store = read_launcher_snapshot()
            .inspect_err(|e| debug!(error = %e, "launcher store unavailable for the patch state"))
            .ok();
        self.patch_state_from(&launcher_log_path(), store.as_ref())
    }

    /// Like [`Self::patch_state`] with an explicit log path and an
    /// already-read store snapshot (`None`: judge by the log alone).
    pub fn patch_state_from(
        &self,
        log_path: &Path,
        store: Option<&StoreSnapshot>,
    ) -> Result<PatchState> {
        let runs = launcher_runs(log_path)?;
        let recorded = store
            .and_then(|s| {
                s.installs
                    .iter()
                    .find(|i| i.channel == self.channel && same_dir(&i.root, &self.root))
            })
            .and_then(|i| i.changelist);
        let on_disk = read_build_manifest(&self.root)
            .ok()
            .and_then(|m| m.changelist?.parse::<u64>().ok());
        Ok(classify(self.channel, &self.root, &runs, recorded, on_disk))
    }
}

fn classify(
    channel: Channel,
    root: &Path,
    runs: &[LauncherRun],
    recorded: Option<u64>,
    on_disk: Option<u64>,
) -> PatchState {
    let mut mine = runs
        .iter()
        .rev()
        .filter(|r| r.channel == channel && same_dir(&r.root, root));
    if let Some(latest) = mine.clone().next()
        && latest.is_open()
    {
        return PatchState::Patching(latest.clone());
    }
    // Runs that ended before writing changed nothing; look past them to the
    // last one that did.
    if let Some(last_writer) = mine.find(|r| r.wrote_files)
        && last_writer.outcome != Some(RunOutcome::Completed)
    {
        return PatchState::Unsettled(Unsettled::RunDidNotComplete(last_writer.clone()));
    }
    if let (Some(recorded), Some(on_disk)) = (recorded, on_disk)
        && recorded != on_disk
    {
        return PatchState::Unsettled(Unsettled::NotTheRecordedBuild { recorded, on_disk });
    }
    PatchState::Settled
}

/// Path equality as Windows sees it: case-insensitive, either separator,
/// trailing separators ignored.
fn same_dir(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| {
        p.to_string_lossy()
            .replace('/', "\\")
            .trim_end_matches('\\')
            .to_ascii_lowercase()
    };
    norm(a) == norm(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lines copied from a real launcher 2.18.1 log, trimmed to the markers.
    const COMPLETED_UPDATE: &str = r#"{ "t":"2026-10-13 17:47:20.295", "[main][info] ": "[Pipeline] Installing Star Citizen LIVE 4.10.2-live.12913470 at C:\\Games\\StarCitizen (type: update, forceDP: false)"  },
{ "t":"2026-10-13 17:48:26.455", "[main][info] ": "[PatcherPhase] Starting delta update (SC LIVE 4.10.2-live.12913470) in C:\\Games\\StarCitizen (statistics enabled: false)"  },
{ "t":"2026-10-13 17:49:24.908", "[main][info] ": "[CigDataPatcher] Calling progress callback: installer@update-loose-files-start"  },
{ "t":"2026-10-13 17:50:17.269", "[main][info] ": "[CigDataPatcher] Calling progress callback: installer@update-files-inside-p4k-start"  },
{ "t":"2026-10-13 18:03:39.782", "[main][info] ": "[CigDataPatcher] Calling progress callback: installer@update-files-inside-p4k-end"  },
{ "t":"2026-10-13 18:03:39.849", "[main][info] ": "[PatcherPhase] Delta update completed (SC LIVE 4.10.2-live.12913470) in C:\\Games\\StarCitizen\\LIVE"  },
"#;

    const PAUSED_THEN_CORRUPT: &str = r#"{ "t":"2026-07-16 12:01:47.251", "[main][info] ": "[Pipeline] Installing Star Citizen LIVE 4.9.0-live.12232306 at C:\\Games\\StarCitizen (type: update, forceDP: false)"  },
{ "t":"2026-07-16 12:02:50.000", "[main][info] ": "[CigDataPatcher] Calling progress callback: installer@update-loose-files-start"  },
{ "t":"2026-07-16 13:59:32.112", "[main][info] ": "[FlowControl] Pausing phase: patcher"  },
{ "t":"2026-07-16 15:13:56.965", "[main][info] ": "[FlowControl] Resuming phase: patcher"  },
{ "t":"2026-07-16 15:39:29.903", "[main][error]": "[Pipeline] Error: BaseP4kCorruptedError: UNKNOWN ERROR: CopyToChunk failed"  },
"#;

    const LIVE: &str = r"C:\Games\StarCitizen\LIVE";

    fn runs(log: &str) -> Vec<LauncherRun> {
        parse_runs(log)
    }

    fn state(log: &str, recorded: Option<u64>, on_disk: Option<u64>) -> PatchState {
        classify(
            Channel::Live,
            Path::new(LIVE),
            &runs(log),
            recorded,
            on_disk,
        )
    }

    #[test]
    fn completed_update_is_one_closed_run() {
        let runs = runs(COMPLETED_UPDATE);
        assert_eq!(runs.len(), 1);
        let run = &runs[0];
        assert_eq!(run.channel, Channel::Live);
        assert_eq!(run.kind, RunKind::Update);
        assert_eq!(run.version_label, "4.10.2-live.12913470");
        assert_eq!(run.root, PathBuf::from(LIVE));
        assert_eq!(run.started_at, "2026-10-13 17:47:20.295");
        assert!(run.wrote_files);
        assert_eq!(run.outcome, Some(RunOutcome::Completed));
    }

    #[test]
    fn completed_update_with_matching_store_is_settled() {
        assert_eq!(
            state(COMPLETED_UPDATE, Some(12913470), Some(12913470)),
            PatchState::Settled
        );
    }

    #[test]
    fn run_cut_mid_p4k_is_patching() {
        // The log as it stood at 18:00 that day: manifest already moved,
        // store not yet.
        let cut: String = COMPLETED_UPDATE
            .lines()
            .take(4)
            .collect::<Vec<_>>()
            .join("\n");
        let PatchState::Patching(run) = state(&cut, Some(12881860), Some(12913470)) else {
            panic!("an open run must read as patching");
        };
        assert!(run.wrote_files);
        assert!(!run.paused);
    }

    #[test]
    fn pause_resume_then_error_is_unsettled() {
        let PatchState::Unsettled(Unsettled::RunDidNotComplete(run)) =
            state(PAUSED_THEN_CORRUPT, Some(12189054), Some(12232306))
        else {
            panic!("a failed writing run must read as unsettled");
        };
        assert_eq!(run.outcome, Some(RunOutcome::Failed));
        assert!(!run.paused);
    }

    #[test]
    fn verify_after_failure_settles() {
        let log = format!(
            "{PAUSED_THEN_CORRUPT}{}",
            r#"{ "t":"2026-07-16 15:44:22.159", "[main][info] ": "[Pipeline] Verifying Star Citizen LIVE 4.9.0-live.12232306 at C:\\Games\\StarCitizen (type: verify, forceDP: false)"  },
{ "t":"2026-07-16 15:45:00.000", "[main][info] ": "[CigDataPatcher] Calling progress callback: installer@update-loose-files-start"  },
{ "t":"2026-07-16 16:00:04.761", "[main][info] ": "[PatcherPhase] Delta update completed (SC LIVE 4.9.0-live.12232306) in C:\\Games\\StarCitizen\\LIVE"  },
"#
        );
        let runs = runs(&log);
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[1].kind, RunKind::Verify);
        assert_eq!(
            state(&log, Some(12232306), Some(12232306)),
            PatchState::Settled
        );
    }

    #[test]
    fn failure_before_writing_does_not_hide_an_earlier_broken_run() {
        let log = format!(
            "{PAUSED_THEN_CORRUPT}{}",
            r#"{ "t":"2026-07-16 15:43:39.880", "[main][info] ": "[Pipeline] Installing Star Citizen LIVE 4.9.0-live.12232306 at C:\\Games\\StarCitizen (type: update, forceDP: false)"  },
{ "t":"2026-07-16 15:43:50.000", "[main][error]": "[Pipeline] Error: RetrieveManifestError: NETWORK FATAL ERROR"  },
"#
        );
        let PatchState::Unsettled(Unsettled::RunDidNotComplete(run)) = state(&log, None, None)
        else {
            panic!("the earlier writing run decides");
        };
        assert_eq!(run.started_at, "2026-07-16 12:01:47.251");
    }

    #[test]
    fn failure_before_writing_on_a_settled_install_stays_settled() {
        let log = format!(
            "{COMPLETED_UPDATE}{}",
            r#"{ "t":"2026-10-20 18:00:00.000", "[main][info] ": "[Pipeline] Installing Star Citizen LIVE 4.10.3-live.13000000 at C:\\Games\\StarCitizen (type: update, forceDP: false)"  },
{ "t":"2026-10-20 18:00:10.000", "[main][error]": "[Pipeline] Error: RetrieveManifestError: NETWORK FATAL ERROR"  },
"#
        );
        assert_eq!(
            state(&log, Some(12913470), Some(12913470)),
            PatchState::Settled
        );
    }

    #[test]
    fn launcher_restart_closes_an_open_run() {
        let cut: String = COMPLETED_UPDATE
            .lines()
            .take(4)
            .collect::<Vec<_>>()
            .join("\n");
        let log = format!(
            "{cut}\n{}",
            r#"{ "t":"2026-10-13 19:00:00.000", "[main][info] ": "[ConnectivityService] Registered IPC listeners"  },"#
        );
        let PatchState::Unsettled(Unsettled::RunDidNotComplete(run)) =
            state(&log, Some(12881860), Some(12913470))
        else {
            panic!("a run the launcher died in must read as unsettled");
        };
        assert_eq!(run.outcome, Some(RunOutcome::Interrupted));
    }

    #[test]
    fn store_mismatch_without_runs_is_unsettled() {
        assert_eq!(
            state("", Some(12881860), Some(12913470)),
            PatchState::Unsettled(Unsettled::NotTheRecordedBuild {
                recorded: 12881860,
                on_disk: 12913470
            })
        );
    }

    #[test]
    fn runs_on_other_channels_and_roots_are_ignored() {
        let ptu = COMPLETED_UPDATE
            .lines()
            .take(4)
            .collect::<Vec<_>>()
            .join("\n")
            .replace("LIVE", "PTU");
        assert_eq!(state(&ptu, None, None), PatchState::Settled);
        let elsewhere = COMPLETED_UPDATE
            .lines()
            .take(4)
            .collect::<Vec<_>>()
            .join("\n")
            .replace(r"C:\\Games", r"D:\\Games");
        assert_eq!(state(&elsewhere, None, None), PatchState::Settled);
    }

    #[test]
    fn root_comparison_ignores_case_and_trailing_separator() {
        assert!(same_dir(
            Path::new(r"C:\Games\StarCitizen\LIVE\"),
            Path::new("c:/games/starcitizen/live")
        ));
    }

    #[test]
    fn paused_run_is_patching_and_paused() {
        let paused: String = PAUSED_THEN_CORRUPT
            .lines()
            .take(3)
            .collect::<Vec<_>>()
            .join("\n");
        let PatchState::Patching(run) = state(&paused, None, None) else {
            panic!("a paused run is still open");
        };
        assert!(run.paused);
    }
}
