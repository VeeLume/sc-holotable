//! High-level install discovery — the "give me all valid installs" entry points.
//!
//! Every public function here has a `_from` sibling that takes an explicit
//! log path, so callers can substitute a fixture or a custom log location.
//! The non-`_from` variants use [`launcher_log_path`] under the hood.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use tracing::debug;

use crate::channel::Channel;
use crate::error::{Error, Result};
use crate::installation::Installation;
use crate::log_parser::{launcher_log_path, parse_launcher_log_entries};

/// Discover every reachable Star Citizen installation by parsing the
/// RSI Launcher log at its default path.
///
/// Installations are returned sorted by channel priority (LIVE first).
/// Entries whose directory is missing or whose `Data.p4k` is missing are
/// filtered out and logged at `debug!` level — they do not fail the call.
///
/// # Errors
///
/// Returns `Err` only for "nothing at all" situations:
///
/// - [`Error::LauncherLogNotFound`] — the launcher log doesn't exist
/// - [`Error::LauncherLogUnreadable`] — exists but couldn't be read
/// - [`Error::NoLaunchEntries`] — log has no `Launching Star Citizen` lines
///
/// Note that `Ok(vec![])` is possible: it means the log had entries but
/// every one of them was filtered out by validation. The [`discover_primary`]
/// convenience promotes this case to [`Error::NoValidInstallations`] if
/// that's what the caller wants.
pub fn discover() -> Result<Vec<Installation>> {
    discover_from(&launcher_log_path())
}

/// Like [`discover`] but reads the launcher log from the given path.
/// Useful for tests and for users whose launcher log lives somewhere other
/// than `%APPDATA%/rsilauncher/logs/log.log`.
pub fn discover_from(log_path: &Path) -> Result<Vec<Installation>> {
    let entries = parse_launcher_log_entries(log_path)?;
    if entries.is_empty() {
        return Err(Error::NoLaunchEntries(log_path.to_path_buf()));
    }

    // Deduplicate by channel: the most recent entry for each channel wins.
    // This matches streamdeck-starcitizen's behavior — the launcher log can
    // contain many historical launches for the same channel, and we only
    // care about the most recent path.
    let mut by_channel: HashMap<Channel, PathBuf> = HashMap::new();
    for (channel, path) in entries {
        by_channel.insert(channel, path);
    }

    let mut installations = Vec::new();
    for (channel, path) in by_channel {
        if let Some(install) = validate_install(channel, &path) {
            installations.push(install);
        }
    }

    installations.sort_by_key(|i| i.channel.priority());
    Ok(installations)
}

/// Convenience: the highest-priority valid installation (LIVE first).
///
/// Returns [`Error::NoValidInstallations`] if every discovered entry is
/// filtered out by validation.
///
/// For "the install the user actually plays" semantics, prefer
/// [`discover_last_launched`] — `discover_primary` always returns LIVE first
/// even if the user primarily plays PTU.
pub fn discover_primary() -> Result<Installation> {
    discover_primary_from(&launcher_log_path())
}

/// Like [`discover_primary`] but reads the log from the given path.
pub fn discover_primary_from(log_path: &Path) -> Result<Installation> {
    let mut all = discover_from(log_path)?;
    if all.is_empty() {
        return Err(Error::NoValidInstallations);
    }
    Ok(all.remove(0))
}

/// Discover the most recently launched installation that is still valid.
///
/// Walks the launcher log in reverse chronological order and returns the
/// first entry whose install still exists on disk and has a readable
/// `Data.p4k`. This is usually the best "default selection" — unlike
/// [`discover_primary`] (which always returns LIVE first), this returns
/// the channel the user most recently played.
///
/// # Errors
///
/// Returns [`Error::NoValidInstallations`] if no entry in the log
/// corresponds to a still-valid install. Returns the same errors as
/// [`discover`] for "no log" / "empty log" cases.
pub fn discover_last_launched() -> Result<Installation> {
    discover_last_launched_from(&launcher_log_path())
}

/// Like [`discover_last_launched`] but reads the log from the given path.
pub fn discover_last_launched_from(log_path: &Path) -> Result<Installation> {
    let entries = parse_launcher_log_entries(log_path)?;
    if entries.is_empty() {
        return Err(Error::NoLaunchEntries(log_path.to_path_buf()));
    }

    // Walk in reverse chronological order (most recent entry first).
    for (channel, path) in entries.into_iter().rev() {
        if let Some(install) = validate_install(channel, &path) {
            return Ok(install);
        }
    }

    Err(Error::NoValidInstallations)
}

/// Try to build a valid [`Installation`] from a channel + root.
///
/// Returns `None` and logs a `debug!` message if:
///
/// - The root directory doesn't exist
/// - `Data.p4k` isn't present at the root
/// - `build_manifest.id` is missing or malformed
///
/// This is the single canonical definition of "is this install valid?" used
/// by all the discovery functions.
fn validate_install(channel: Channel, root: &Path) -> Option<Installation> {
    if !root.is_dir() {
        debug!(
            channel = %channel,
            path = %root.display(),
            "skipping install: directory does not exist"
        );
        return None;
    }
    if !root.join("Data.p4k").is_file() {
        debug!(
            channel = %channel,
            path = %root.display(),
            "skipping install: Data.p4k missing"
        );
        return None;
    }
    match Installation::from_parts(channel, root) {
        Ok(install) => Some(install),
        Err(e) => {
            debug!(
                channel = %channel,
                path = %root.display(),
                error = %e,
                "skipping install: failed to read build manifest"
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Create a fake install directory with a valid `build_manifest.id` and
    /// (optionally) a `Data.p4k` file. Returns the path to the install root.
    fn fake_install(
        parent: &Path,
        channel_name: &str,
        version: &str,
        with_data_p4k: bool,
    ) -> PathBuf {
        let root = parent.join(channel_name);
        std::fs::create_dir_all(&root).unwrap();
        let mut f = std::fs::File::create(root.join("build_manifest.id")).unwrap();
        writeln!(
            f,
            r#"{{"Data":{{"Branch":"sc-alpha-4.6.1","BuildId":"1","Version":"{version}"}}}}"#
        )
        .unwrap();
        drop(f);
        if with_data_p4k {
            std::fs::write(root.join("Data.p4k"), b"fake p4k").unwrap();
        }
        root
    }

    /// Write a fake launcher log file with the given entries (legacy format).
    fn fake_log(entries: &[(Channel, &Path)]) -> tempfile::NamedTempFile {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        for (channel, path) in entries {
            writeln!(
                tmp,
                "[Launcher::launch] Launching Star Citizen {} from ({})",
                channel.display_name(),
                path.display()
            )
            .unwrap();
        }
        tmp
    }

    #[test]
    fn discover_from_returns_all_valid_installs_sorted_by_priority() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", true);
        let ptu_dir = fake_install(temp.path(), "PTU", "4.7.0.0", true);

        // Write PTU first so we can verify sorting is by priority, not log order.
        let log = fake_log(&[(Channel::Ptu, &ptu_dir), (Channel::Live, &live_dir)]);

        let installs = discover_from(log.path()).unwrap();
        assert_eq!(installs.len(), 2);
        assert_eq!(installs[0].channel, Channel::Live);
        assert_eq!(installs[1].channel, Channel::Ptu);
    }

    #[test]
    fn discover_from_filters_missing_data_p4k() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", true);
        let ptu_dir = fake_install(temp.path(), "PTU", "4.7.0.0", false); // no Data.p4k

        let log = fake_log(&[(Channel::Live, &live_dir), (Channel::Ptu, &ptu_dir)]);

        let installs = discover_from(log.path()).unwrap();
        assert_eq!(installs.len(), 1);
        assert_eq!(installs[0].channel, Channel::Live);
    }

    #[test]
    fn discover_from_filters_missing_directory() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", true);
        let ghost_dir = temp.path().join("GHOST_PTU"); // never created

        let log = fake_log(&[(Channel::Live, &live_dir), (Channel::Ptu, &ghost_dir)]);

        let installs = discover_from(log.path()).unwrap();
        assert_eq!(installs.len(), 1);
        assert_eq!(installs[0].channel, Channel::Live);
    }

    #[test]
    fn discover_from_dedupes_by_channel_keeping_most_recent() {
        let temp = tempfile::tempdir().unwrap();
        let old_dir = fake_install(temp.path(), "LIVE_old", "4.6.0.0", true);
        let new_dir = fake_install(temp.path(), "LIVE_new", "4.7.0.0", true);

        // Two LIVE entries — the later one should win.
        let log = fake_log(&[(Channel::Live, &old_dir), (Channel::Live, &new_dir)]);

        let installs = discover_from(log.path()).unwrap();
        assert_eq!(installs.len(), 1);
        assert_eq!(installs[0].root, new_dir);
        assert_eq!(installs[0].manifest.version, "4.7.0.0");
    }

    #[test]
    fn discover_from_missing_log_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let missing = tmp.path().join("no-such.log");
        let err = discover_from(&missing).unwrap_err();
        assert!(matches!(err, Error::LauncherLogNotFound(_)));
    }

    #[test]
    fn discover_from_empty_log_errors() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let err = discover_from(tmp.path()).unwrap_err();
        assert!(matches!(err, Error::NoLaunchEntries(_)));
    }

    #[test]
    fn discover_from_all_invalid_returns_empty_vec() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", false); // no Data.p4k

        let log = fake_log(&[(Channel::Live, &live_dir)]);

        let installs = discover_from(log.path()).unwrap();
        assert!(installs.is_empty());
    }

    #[test]
    fn discover_primary_from_returns_live_first() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", true);
        let ptu_dir = fake_install(temp.path(), "PTU", "4.7.0.0", true);

        let log = fake_log(&[(Channel::Ptu, &ptu_dir), (Channel::Live, &live_dir)]);

        let install = discover_primary_from(log.path()).unwrap();
        assert_eq!(install.channel, Channel::Live);
    }

    #[test]
    fn discover_primary_from_all_invalid_errors() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", false);

        let log = fake_log(&[(Channel::Live, &live_dir)]);

        let err = discover_primary_from(log.path()).unwrap_err();
        assert!(matches!(err, Error::NoValidInstallations));
    }

    #[test]
    fn discover_last_launched_from_picks_most_recent_valid() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", true);
        let ptu_dir = fake_install(temp.path(), "PTU", "4.7.0.0", true);

        // Log order: LIVE first, then PTU as the most recent.
        let log = fake_log(&[(Channel::Live, &live_dir), (Channel::Ptu, &ptu_dir)]);

        let install = discover_last_launched_from(log.path()).unwrap();
        assert_eq!(install.channel, Channel::Ptu);
    }

    #[test]
    fn discover_last_launched_from_skips_invalid_most_recent() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", true);
        let ptu_dir = fake_install(temp.path(), "PTU", "4.7.0.0", false); // invalid

        // PTU is most recent but invalid; LIVE is older but valid.
        let log = fake_log(&[(Channel::Live, &live_dir), (Channel::Ptu, &ptu_dir)]);

        let install = discover_last_launched_from(log.path()).unwrap();
        assert_eq!(install.channel, Channel::Live);
    }

    #[test]
    fn discover_last_launched_from_all_invalid_errors() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = fake_install(temp.path(), "LIVE", "4.6.1.0", false);

        let log = fake_log(&[(Channel::Live, &live_dir)]);

        let err = discover_last_launched_from(log.path()).unwrap_err();
        assert!(matches!(err, Error::NoValidInstallations));
    }

    #[test]
    fn discover_last_launched_from_missing_log_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let missing = tmp.path().join("no-such.log");
        let err = discover_last_launched_from(&missing).unwrap_err();
        assert!(matches!(err, Error::LauncherLogNotFound(_)));
    }

    #[test]
    fn discover_last_launched_from_empty_log_errors() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let err = discover_last_launched_from(tmp.path()).unwrap_err();
        assert!(matches!(err, Error::NoLaunchEntries(_)));
    }
}
