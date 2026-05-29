//! RSI Launcher log parsing.
//!
//! The launcher writes a single rolling log at
//! `%APPDATA%/rsilauncher/logs/log.log` with one JSON-like line per event.
//! Several distinct event shapes mention an install path; rather than
//! waiting for the user to launch a channel, this module extracts every
//! marker that pins an install root or library directory.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::channel::Channel;
use crate::error::{Error, Result};

/// `[Launcher::launch] Launching Star Citizen <CHANNEL> from (<ROOT>)`.
/// The captured path is always a full channel root.
static LAUNCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Launching Star Citizen (\S+) from \((.+?)\)")
        .expect("launch marker regex is valid")
});

/// `[Installer] Installing Star Citizen <CHAN> <VER> at <LIB> (force DP: ...)`
/// and the matching `Verifying ...` shape. The captured path is the parent
/// **library directory**; the channel subdir still has to be appended.
static INSTALLER_AT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[Installer\] (?:Installing|Verifying) Star Citizen (\S+) \S+ at (.+?) \(force DP")
        .expect("installer-at regex is valid")
});

/// `[Installer] ... (SC <CHAN> <VER>) in <PATH>` (with an occasional stray
/// extra `(` that the launcher emits — `(SC (LIVE 4.7.0-live...) in ...`).
/// Captured path may be either a library dir or a full root; the caller
/// normalises by checking whether the path's last component already matches
/// the channel.
static INSTALLER_IN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\[Installer\][^(]*\(SC \(?(\S+) \S+\)? in (.+?)(?:\\?"|$)"#)
        .expect("installer-in regex is valid")
});

/// `[Launcher::launch] Deleting <ROOT>\loginData.json for <CHAN>`.
/// The captured path is the full channel root.
static DELETE_LOGIN_RE: LazyLock<Regex> = LazyLock::new(|| {
    // `\\+` so we match either a single `\` (plain-text format) or `\\` (the
    // JSON-framed format escapes every backslash). Channel capture uses
    // `[A-Z-]+` rather than `\S+` to avoid swallowing the trailing `"` of
    // the JSON envelope.
    Regex::new(r"Deleting (.+?)\\+loginData\.json for ([A-Z-]+)")
        .expect("delete-login regex is valid")
});

/// What kind of launcher-log marker a [`LogEntry`] came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogEntryKind {
    /// `Launcher::launch Launching Star Citizen ... from (...)`.
    Launch,
    /// Any `[Installer] ...` marker (Installing, Verifying, Delta update
    /// applied, base pack download, ...).
    Install,
    /// `Launcher::launch Deleting <root>\loginData.json for <channel>`,
    /// emitted as part of a launch but observed even on launches that
    /// later abort.
    DeleteLoginData,
}

/// One install-related event extracted from the launcher log.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogEntry {
    pub channel: Channel,
    /// Always normalised to a **full channel root** (the directory that
    /// would contain `Data.p4k`), regardless of whether the underlying
    /// marker mentioned a library directory or a root.
    pub root: PathBuf,
    pub kind: LogEntryKind,
}

/// Default path to the RSI Launcher log file on Windows:
/// `%APPDATA%/rsilauncher/logs/log.log`.
///
/// Returns a possibly-invalid path if `APPDATA` is unset (e.g. on non-Windows
/// platforms) — the downstream `exists()` check will report the error.
pub fn launcher_log_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata).join("rsilauncher/logs/log.log")
}

/// Parse every install-related entry from a launcher log file.
/// Returns the entries in chronological order (as they appear in the log).
///
/// Recognised marker shapes are documented on [`LogEntryKind`]. Paths are
/// normalised to full channel roots via [`Channel::install_dir_name`].
///
/// Unlike the high-level discovery functions, this does **not** validate
/// installations or filter missing paths. It's a lower-level helper for
/// consumers that want the raw launcher-log view.
///
/// # Errors
///
/// - [`Error::LauncherLogNotFound`] if the file doesn't exist.
/// - [`Error::LauncherLogUnreadable`] if the file exists but can't be read.
///
/// An empty log (no install-related entries) is **not** an error here — it
/// returns an empty vec. Callers that want to treat that as an error should
/// use `discover*` which maps empty results to [`Error::NoInstallEntries`].
pub fn parse_launcher_log_entries(log_path: &Path) -> Result<Vec<LogEntry>> {
    if !log_path.exists() {
        return Err(Error::LauncherLogNotFound(log_path.to_path_buf()));
    }

    let content =
        std::fs::read_to_string(log_path).map_err(|source| Error::LauncherLogUnreadable {
            path: log_path.to_path_buf(),
            source,
        })?;

    let mut entries = Vec::new();
    for line in content.lines() {
        if let Some(entry) = extract_entry(line) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

/// Extract a single [`LogEntry`] from a log line if any of the install-related
/// markers match. Returns `None` for unrelated lines.
fn extract_entry(line: &str) -> Option<LogEntry> {
    if let Some(caps) = LAUNCH_RE.captures(line) {
        let channel = Channel::from_str_loose(caps.get(1)?.as_str())?;
        let raw_path = unescape_path(caps.get(2)?.as_str());
        return Some(LogEntry {
            channel,
            root: normalise_root(channel, &raw_path),
            kind: LogEntryKind::Launch,
        });
    }
    if let Some(caps) = INSTALLER_AT_RE.captures(line) {
        let channel = Channel::from_str_loose(caps.get(1)?.as_str())?;
        let raw_path = unescape_path(caps.get(2)?.as_str());
        return Some(LogEntry {
            channel,
            root: normalise_root(channel, &raw_path),
            kind: LogEntryKind::Install,
        });
    }
    if let Some(caps) = INSTALLER_IN_RE.captures(line) {
        let channel = Channel::from_str_loose(caps.get(1)?.as_str())?;
        let raw_path = unescape_path(caps.get(2)?.as_str());
        return Some(LogEntry {
            channel,
            root: normalise_root(channel, &raw_path),
            kind: LogEntryKind::Install,
        });
    }
    if let Some(caps) = DELETE_LOGIN_RE.captures(line) {
        let channel = Channel::from_str_loose(caps.get(2)?.as_str())?;
        let raw_path = unescape_path(caps.get(1)?.as_str());
        return Some(LogEntry {
            channel,
            root: normalise_root(channel, &raw_path),
            kind: LogEntryKind::DeleteLoginData,
        });
    }
    None
}

/// JSON-framed lines escape backslashes as `\\`; flip them back. Plain lines
/// (which don't exist any more in 2.x but the regex still matches) are
/// unaffected because they don't contain `\\`.
fn unescape_path(s: &str) -> PathBuf {
    PathBuf::from(s.replace("\\\\", "\\"))
}

/// Decide whether a captured path is already a channel root or a parent
/// library directory, and return the channel root in either case.
fn normalise_root(channel: Channel, path: &Path) -> PathBuf {
    let last = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if last.eq_ignore_ascii_case(channel.install_dir_name()) {
        path.to_path_buf()
    } else {
        path.join(channel.install_dir_name())
    }
}

/// Detect which Star Citizen channel a running process belongs to, based on
/// its executable name.
///
/// Useful for consumers (like streamdeck-starcitizen) that observe which
/// `StarCitizen*.exe` variant the user launched. Unqualified `StarCitizen.exe`
/// is treated as LIVE.
///
/// Returns `None` if the name doesn't look like an SC executable at all.
pub fn detect_channel_from_process(process_name: &str) -> Option<Channel> {
    let upper = process_name.to_uppercase();
    if upper.contains("EPTU") {
        Some(Channel::Eptu)
    } else if upper.contains("PTU") {
        Some(Channel::Ptu)
    } else if upper.contains("HOTFIX") {
        Some(Channel::Hotfix)
    } else if upper.contains("TECHPREVIEW") || upper.contains("TECH-PREVIEW") {
        Some(Channel::TechPreview)
    } else if upper.contains("STARCITIZEN") {
        // Unqualified StarCitizen.exe is the LIVE build.
        Some(Channel::Live)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn parse(line: &str) -> Option<LogEntry> {
        extract_entry(line)
    }

    #[test]
    fn parse_launch_marker() {
        let line = r#"{ "t":"...", "[main][info] ": "[Launcher::launch] Launching Star Citizen LIVE from (C:\\Games\\StarCitizen\\LIVE)" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::Live);
        assert_eq!(entry.root, PathBuf::from("C:\\Games\\StarCitizen\\LIVE"));
        assert_eq!(entry.kind, LogEntryKind::Launch);
    }

    #[test]
    fn parse_installer_at_appends_channel_dir() {
        // The "at <library>" shape gives a parent directory; channel must
        // be appended to reach the install root.
        let line = r#"{ "t":"...", "[main][info] ": "[Installer] Installing Star Citizen LIVE 4.7.0-live.11576750 at C:\\Games\\StarCitizen (force DP: false)" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::Live);
        assert_eq!(entry.root, PathBuf::from("C:\\Games\\StarCitizen\\LIVE"));
        assert_eq!(entry.kind, LogEntryKind::Install);
    }

    #[test]
    fn parse_installer_at_verifying() {
        let line = r#"{ "t":"...", "[main][info] ": "[Installer] Verifying Star Citizen LIVE 4.7.1-live.11592622 at C:\\Program Files\\Roberts Space Industries\\StarCitizen (force DP: false)" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::Live);
        assert_eq!(
            entry.root,
            PathBuf::from("C:\\Program Files\\Roberts Space Industries\\StarCitizen\\LIVE")
        );
    }

    #[test]
    fn parse_installer_in_root_already_includes_channel() {
        // "in <root>" shape where the path is already a full channel root.
        let line = r#"{ "t":"...", "[main][info] ": "[Installer] Delta update applied (SC LIVE 4.7.1-live.11592622) in C:\\Games\\StarCitizen\\LIVE" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::Live);
        assert_eq!(entry.root, PathBuf::from("C:\\Games\\StarCitizen\\LIVE"));
        assert_eq!(entry.kind, LogEntryKind::Install);
    }

    #[test]
    fn parse_installer_in_with_stray_paren() {
        // The launcher emits a malformed "(SC (LIVE 4.7.0-live.123) in ..."
        // for delta-update-start lines. The regex tolerates the optional `(`.
        let line = r#"{ "t":"...", "[main][info] ": "[Installer] Starting delta update (SC (LIVE 4.7.0-live.11576750) in C:\\Games\\StarCitizen" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::Live);
        assert_eq!(entry.root, PathBuf::from("C:\\Games\\StarCitizen\\LIVE"));
    }

    #[test]
    fn parse_tech_preview_uses_dashed_dirname() {
        // TECH-PREVIEW is the on-disk dir name; display_name() is "TECH".
        let line = r#"{ "t":"...", "[main][info] ": "[Installer] Installing Star Citizen TECH-PREVIEW 4.7-tp.11650317 at C:\\Games\\StarCitizen (force DP: false)" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::TechPreview);
        assert_eq!(
            entry.root,
            PathBuf::from("C:\\Games\\StarCitizen\\TECH-PREVIEW")
        );
    }

    #[test]
    fn parse_tech_preview_root_in_marker() {
        let line = r#"{ "t":"...", "[main][info] ": "[Installer] Initial pack installed (SC TECH-PREVIEW 4.7-tp.11650317) in C:\\Games\\StarCitizen\\TECH-PREVIEW" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::TechPreview);
        assert_eq!(
            entry.root,
            PathBuf::from("C:\\Games\\StarCitizen\\TECH-PREVIEW")
        );
    }

    #[test]
    fn parse_delete_login_data_marker() {
        let line = r#"{ "t":"...", "[main][info] ": "[Launcher::launch] Deleting C:\\Games\\StarCitizen\\LIVE\\loginData.json for LIVE" },"#;
        let entry = parse(line).unwrap();
        assert_eq!(entry.channel, Channel::Live);
        assert_eq!(entry.root, PathBuf::from("C:\\Games\\StarCitizen\\LIVE"));
        assert_eq!(entry.kind, LogEntryKind::DeleteLoginData);
    }

    #[test]
    fn ignores_unrelated_lines() {
        assert!(parse(r#"{ "t":"...", "[main][info] ": "Checking for update" },"#).is_none());
        assert!(parse("some random log line").is_none());
    }

    #[test]
    fn parse_full_log_collects_in_order() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            r#"{{ "t":"...", "[main][info] ": "[Installer] Installing Star Citizen LIVE 4.7.0-live.11576750 at C:\\Games\\StarCitizen (force DP: false)" }},"#
        )
        .unwrap();
        writeln!(tmp, r#"{{ "[main][info] ": "Checking for update" }},"#).unwrap();
        writeln!(
            tmp,
            r#"{{ "t":"...", "[main][info] ": "[Launcher::launch] Launching Star Citizen LIVE from (C:\\Games\\StarCitizen\\LIVE)" }},"#
        )
        .unwrap();
        writeln!(
            tmp,
            r#"{{ "t":"...", "[main][info] ": "[Installer] Initial pack installed (SC TECH-PREVIEW 4.7-tp.11650317) in C:\\Games\\StarCitizen\\TECH-PREVIEW" }},"#
        )
        .unwrap();

        let entries = parse_launcher_log_entries(tmp.path()).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].kind, LogEntryKind::Install);
        assert_eq!(entries[0].channel, Channel::Live);
        assert_eq!(entries[1].kind, LogEntryKind::Launch);
        assert_eq!(entries[2].kind, LogEntryKind::Install);
        assert_eq!(entries[2].channel, Channel::TechPreview);
    }

    #[test]
    fn empty_log_returns_empty_vec_not_error() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let entries = parse_launcher_log_entries(tmp.path()).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn missing_log_returns_not_found_error() {
        let tmp = tempfile::tempdir().unwrap();
        let missing = tmp.path().join("does_not_exist.log");
        let err = parse_launcher_log_entries(&missing).unwrap_err();
        assert!(matches!(err, Error::LauncherLogNotFound(_)));
    }

    #[test]
    fn detect_channel_from_process_starcitizen() {
        assert_eq!(
            detect_channel_from_process("StarCitizen.exe"),
            Some(Channel::Live)
        );
        assert_eq!(
            detect_channel_from_process("starcitizen.exe"),
            Some(Channel::Live)
        );
    }

    #[test]
    fn detect_channel_from_process_variants() {
        assert_eq!(
            detect_channel_from_process("StarCitizen_PTU.exe"),
            Some(Channel::Ptu)
        );
        assert_eq!(
            detect_channel_from_process("StarCitizen_EPTU.exe"),
            Some(Channel::Eptu)
        );
        assert_eq!(
            detect_channel_from_process("StarCitizen_Hotfix.exe"),
            Some(Channel::Hotfix)
        );
        assert_eq!(
            detect_channel_from_process("StarCitizen_TechPreview.exe"),
            Some(Channel::TechPreview)
        );
    }

    #[test]
    fn detect_channel_from_process_non_sc() {
        assert_eq!(detect_channel_from_process("notepad.exe"), None);
        assert_eq!(detect_channel_from_process(""), None);
    }
}
