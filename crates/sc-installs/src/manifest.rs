//! Parse `build_manifest.id` files that ship with each Star Citizen install.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Parsed fields from a `build_manifest.id` file.
///
/// Supports both the current v2 format (`{"Data": {...}}`) and the legacy
/// flat format that earlier SC builds shipped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BuildManifest {
    /// Full version as reported by the manifest, e.g. `"4.6.173.39432"`.
    pub version: String,
    /// Branch name from the manifest, e.g. `"sc-alpha-4.6.1"`.
    pub branch: String,
    /// Build ID from the manifest, e.g. `"12345"`.
    pub build_id: String,
    /// Changelist number from `Data.RequestedP4ChangeNum`, if present.
    /// Needed for the launcher-visible version string.
    pub changelist: Option<String>,
}

/// Raw v2 manifest shape as it appears on disk. Private — consumers see the
/// normalized [`BuildManifest`] produced by [`read_build_manifest`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawManifest {
    /// v2 format nests everything under a `Data` key.
    #[serde(default)]
    data: Option<RawManifestData>,
    /// Legacy format had the P4K filename at the root level; version is
    /// derived from `Data_<version>.p4k`.
    #[serde(default)]
    requested_p4k_file_name: Option<String>,
    /// Legacy format had branch at the root level.
    #[serde(default)]
    branch: Option<String>,
    /// Legacy format had build id at the root level.
    #[serde(default)]
    build_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawManifestData {
    #[serde(default)]
    version: String,
    #[serde(default)]
    branch: String,
    #[serde(default)]
    build_id: String,
    #[serde(default)]
    requested_p4_change_num: String,
}

/// Read and parse the `build_manifest.id` file in an installation directory.
///
/// Handles both the v2 nested format and the legacy flat format transparently.
pub fn read_build_manifest(install_root: &Path) -> Result<BuildManifest> {
    let manifest_path = install_root.join("build_manifest.id");

    let content =
        std::fs::read_to_string(&manifest_path).map_err(|source| Error::BuildManifestUnreadable {
            path: manifest_path.clone(),
            source,
        })?;

    let raw: RawManifest =
        serde_json::from_str(&content).map_err(|source| Error::BuildManifestMalformed {
            path: manifest_path.clone(),
            source,
        })?;

    Ok(from_raw(raw))
}

/// Normalize a [`RawManifest`] into a [`BuildManifest`], preferring the v2
/// nested shape and falling back to legacy fields.
fn from_raw(raw: RawManifest) -> BuildManifest {
    if let Some(data) = raw.data.as_ref() && !data.version.is_empty() {
        return BuildManifest {
            version: data.version.clone(),
            branch: data.branch.clone(),
            build_id: data.build_id.clone(),
            changelist: if data.requested_p4_change_num.is_empty() {
                None
            } else {
                Some(data.requested_p4_change_num.clone())
            },
        };
    }

    // Legacy: version comes from the P4K filename `Data_<version>.p4k`.
    let version = raw
        .requested_p4k_file_name
        .as_deref()
        .and_then(|s| s.strip_prefix("Data_"))
        .and_then(|s| s.strip_suffix(".p4k"))
        .unwrap_or("")
        .to_string();

    BuildManifest {
        version,
        branch: raw.branch.unwrap_or_default(),
        build_id: raw.build_id.unwrap_or_default(),
        changelist: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_v2_manifest_full() {
        let json = r#"{"Data":{"Branch":"sc-alpha-4.6.0","BuildId":"None","Version":"4.6.173.39432","RequestedP4ChangeNum":"11592622"}}"#;
        let raw: RawManifest = serde_json::from_str(json).unwrap();
        let manifest = from_raw(raw);
        assert_eq!(manifest.version, "4.6.173.39432");
        assert_eq!(manifest.branch, "sc-alpha-4.6.0");
        assert_eq!(manifest.build_id, "None");
        assert_eq!(manifest.changelist.as_deref(), Some("11592622"));
    }

    #[test]
    fn parse_v2_manifest_without_changelist() {
        let json = r#"{"Data":{"Branch":"sc-alpha-4.6.0","BuildId":"1","Version":"4.6.0.0"}}"#;
        let raw: RawManifest = serde_json::from_str(json).unwrap();
        let manifest = from_raw(raw);
        assert_eq!(manifest.version, "4.6.0.0");
        assert_eq!(manifest.changelist, None);
    }

    #[test]
    fn parse_legacy_manifest() {
        let json = r#"{"RequestedP4kFileName":"Data_4.6.1.0.p4k","Branch":"sc-alpha-4.6.1","BuildId":"12345"}"#;
        let raw: RawManifest = serde_json::from_str(json).unwrap();
        let manifest = from_raw(raw);
        assert_eq!(manifest.version, "4.6.1.0");
        assert_eq!(manifest.branch, "sc-alpha-4.6.1");
        assert_eq!(manifest.build_id, "12345");
        assert_eq!(manifest.changelist, None);
    }

    #[test]
    fn read_from_disk() {
        let temp = tempfile::tempdir().unwrap();
        let mut f = std::fs::File::create(temp.path().join("build_manifest.id")).unwrap();
        writeln!(
            f,
            r#"{{"Data":{{"Branch":"sc-alpha-4.6.1","BuildId":"99","Version":"4.6.1.0","RequestedP4ChangeNum":"555"}}}}"#
        )
        .unwrap();
        drop(f);

        let manifest = read_build_manifest(temp.path()).unwrap();
        assert_eq!(manifest.version, "4.6.1.0");
        assert_eq!(manifest.branch, "sc-alpha-4.6.1");
        assert_eq!(manifest.build_id, "99");
        assert_eq!(manifest.changelist.as_deref(), Some("555"));
    }

    #[test]
    fn read_missing_file_returns_unreadable_error() {
        let temp = tempfile::tempdir().unwrap();
        let err = read_build_manifest(temp.path()).unwrap_err();
        assert!(matches!(err, Error::BuildManifestUnreadable { .. }));
    }

    #[test]
    fn read_malformed_json_returns_malformed_error() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join("build_manifest.id"), b"not json at all").unwrap();
        let err = read_build_manifest(temp.path()).unwrap_err();
        assert!(matches!(err, Error::BuildManifestMalformed { .. }));
    }
}
