//! RSI Launcher log parsing.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::channel::Channel;
use crate::error::{Error, Result};

/// Matches a `Launching Star Citizen <CHANNEL> from (<PATH>)` marker in a
/// launcher log line. Works for both the legacy plain-text format and the
/// JSON-per-line format used by RSI Launcher 2.x — the marker text is
/// identical in both, only the surrounding framing differs.
///
/// - Channel is captured as a run of non-whitespace, non-`(` characters
///   (e.g. `LIVE`, `PTU`, `EPTU`, `HOTFIX`, `TECHPREVIEW`).
/// - Path is captured greedily up to the **last** `)` on the line, so paths
///   that happen to contain `)` still round-trip correctly. (Backslashes
///   inside the path may be `\\`-escaped by the JSON framing; we unescape
///   them after capture.)
static LAUNCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Launching Star Citizen (\S+) from \((.*)\)")
        .expect("launch marker regex is valid")
});

/// Default path to the RSI Launcher log file on Windows:
/// `%APPDATA%/rsilauncher/logs/log.log`.
///
/// Returns a possibly-invalid path if `APPDATA` is unset (e.g. on non-Windows
/// platforms) — the downstream `exists()` check will report the error.
pub fn launcher_log_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata).join("rsilauncher/logs/log.log")
}

/// Parse every `Launching Star Citizen` entry from a launcher log file.
/// Returns the entries in chronological order (as they appear in the log).
///
/// Supports both the legacy plain-text format used by older launchers and
/// the JSON-per-line format used by RSI Launcher 2.x.
///
/// Unlike the high-level discovery functions, this does **not** validate
/// installations or filter missing paths. It's a lower-level helper for
/// consumers that want the raw launcher-log view. If you just want "all
/// valid installs", use [`crate::discover`] instead.
///
/// # Errors
///
/// - [`Error::LauncherLogNotFound`] if the file doesn't exist.
/// - [`Error::LauncherLogUnreadable`] if the file exists but can't be read.
///
/// An empty log (no launch entries) is **not** an error here — it returns an
/// empty vec. Callers that want to treat that as an error should use
/// `discover*` which maps empty results to [`Error::NoLaunchEntries`].
pub fn parse_launcher_log_entries(log_path: &Path) -> Result<Vec<(Channel, PathBuf)>> {
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
        if let Some(entry) = extract_launch_entry(line) {
            entries.push(entry);
        }
    }

    Ok(entries)
}

/// Extract a single `(Channel, PathBuf)` entry from a log line, if the line
/// is a `Launching Star Citizen` entry. Handles both the legacy plain-text
/// format and the JSON v2.x format via a single regex — the marker text is
/// identical in both, the only post-processing needed is unescaping `\\`
/// for the JSON-framed variant.
fn extract_launch_entry(line: &str) -> Option<(Channel, PathBuf)> {
    let caps = LAUNCH_RE.captures(line)?;
    let channel_str = caps.get(1)?.as_str();
    let path_str = caps.get(2)?.as_str();

    // JSON format escapes backslashes as `\\`; normalize them back to single.
    // Legacy format has single backslashes already and is unaffected.
    let path_str = path_str.replace("\\\\", "\\");

    let channel = Channel::from_str_loose(channel_str)?;
    Some((channel, PathBuf::from(path_str)))
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

    #[test]
    fn parse_legacy_format() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "2024-01-01 [Launcher::launch] Launching Star Citizen LIVE from (D:\\StarCitizen\\LIVE)"
        )
        .unwrap();
        writeln!(
            tmp,
            "2024-01-02 [Launcher::launch] Launching Star Citizen PTU from (D:\\StarCitizen\\PTU)"
        )
        .unwrap();
        writeln!(tmp, "some other log line").unwrap();
        writeln!(
            tmp,
            "2024-01-03 [Launcher::launch] Launching Star Citizen LIVE from (E:\\SC\\LIVE)"
        )
        .unwrap();

        let entries = parse_launcher_log_entries(tmp.path()).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].0, Channel::Live);
        assert_eq!(entries[0].1, PathBuf::from("D:\\StarCitizen\\LIVE"));
        assert_eq!(entries[1].0, Channel::Ptu);
        assert_eq!(entries[2].0, Channel::Live);
        assert_eq!(entries[2].1, PathBuf::from("E:\\SC\\LIVE"));
    }

    #[test]
    fn parse_json_v2_format() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            r#"{{ "t":"2025-05-03 17:38:04.649", "[main][info] ": "Launching Star Citizen LIVE from (C:\\Games\\StarCitizen\\LIVE)"  }},"#
        )
        .unwrap();
        writeln!(
            tmp,
            r#"{{ "t":"2025-05-03 18:00:42.832", "[main][info] ": "Launching Star Citizen PTU from (C:\\Games\\StarCitizen\\PTU)"  }},"#
        )
        .unwrap();
        writeln!(
            tmp,
            r#"{{ "t":"...", "[main][info] ": "Checking for update" }},"#
        )
        .unwrap();

        let entries = parse_launcher_log_entries(tmp.path()).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].0, Channel::Live);
        assert_eq!(entries[0].1, PathBuf::from("C:\\Games\\StarCitizen\\LIVE"));
        assert_eq!(entries[1].0, Channel::Ptu);
        assert_eq!(entries[1].1, PathBuf::from("C:\\Games\\StarCitizen\\PTU"));
    }

    #[test]
    fn mixed_formats_in_one_log() {
        // A log that has both plain-text and JSON lines (can happen across
        // launcher version upgrades).
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "[Launcher::launch] Launching Star Citizen LIVE from (C:\\Old\\LIVE)"
        )
        .unwrap();
        writeln!(
            tmp,
            r#"{{ "[main][info] ": "Launching Star Citizen PTU from (C:\\New\\PTU)" }},"#
        )
        .unwrap();

        let entries = parse_launcher_log_entries(tmp.path()).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], (Channel::Live, PathBuf::from("C:\\Old\\LIVE")));
        assert_eq!(entries[1], (Channel::Ptu, PathBuf::from("C:\\New\\PTU")));
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
