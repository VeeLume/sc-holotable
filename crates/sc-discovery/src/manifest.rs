//! Parse `build_manifest.id` files that ship with each Star Citizen install.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Parsed fields from a `build_manifest.id` file.
///
/// The manifest format used by every Star Citizen build the workspace
/// currently supports is a single JSON object nested under a `Data` key.
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

/// Raw manifest shape as it appears on disk. Private — consumers see the
/// normalized [`BuildManifest`] produced by [`read_build_manifest`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawManifest {
    data: RawManifestData,
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
pub fn read_build_manifest(install_root: &Path) -> Result<BuildManifest> {
    let manifest_path = install_root.join("build_manifest.id");

    let content = std::fs::read_to_string(&manifest_path).map_err(|source| {
        Error::BuildManifestUnreadable {
            path: manifest_path.clone(),
            source,
        }
    })?;

    let raw: RawManifest =
        serde_json::from_str(&content).map_err(|source| Error::BuildManifestMalformed {
            path: manifest_path.clone(),
            source,
        })?;

    Ok(from_raw(raw))
}

fn from_raw(raw: RawManifest) -> BuildManifest {
    let RawManifestData {
        version,
        branch,
        build_id,
        requested_p4_change_num,
    } = raw.data;
    BuildManifest {
        version,
        branch,
        build_id,
        changelist: if requested_p4_change_num.is_empty() {
            None
        } else {
            Some(requested_p4_change_num)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_manifest_full() {
        let json = r#"{"Data":{"Branch":"sc-alpha-4.6.0","BuildId":"None","Version":"4.6.173.39432","RequestedP4ChangeNum":"11592622"}}"#;
        let raw: RawManifest = serde_json::from_str(json).unwrap();
        let manifest = from_raw(raw);
        assert_eq!(manifest.version, "4.6.173.39432");
        assert_eq!(manifest.branch, "sc-alpha-4.6.0");
        assert_eq!(manifest.build_id, "None");
        assert_eq!(manifest.changelist.as_deref(), Some("11592622"));
    }

    #[test]
    fn parse_manifest_without_changelist() {
        let json = r#"{"Data":{"Branch":"sc-alpha-4.6.0","BuildId":"1","Version":"4.6.0.0"}}"#;
        let raw: RawManifest = serde_json::from_str(json).unwrap();
        let manifest = from_raw(raw);
        assert_eq!(manifest.version, "4.6.0.0");
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
