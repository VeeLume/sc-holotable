//! Typed representation of a discovered Star Citizen installation.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::channel::Channel;
use crate::error::{Error, Result};
use crate::language::Language;
use crate::manifest::{BuildManifest, read_build_manifest};

/// A discovered, validated Star Citizen installation.
///
/// Constructed either by the discovery functions ([`crate::discover`] and
/// friends) or explicitly via [`Installation::from_root`] and
/// [`Installation::from_parts`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Installation {
    /// Release channel (LIVE, Hotfix, PTU, EPTU, TechPreview).
    pub channel: Channel,
    /// Root directory of the installation on disk,
    /// e.g. `C:\Games\StarCitizen\LIVE`.
    pub root: PathBuf,
    /// Parsed `build_manifest.id` contents.
    pub manifest: BuildManifest,
}

impl Installation {
    /// Build an installation from an arbitrary path. Reads
    /// `build_manifest.id` and infers the channel from the final path
    /// component (e.g. `LIVE`, `PTU`, `EPTU`).
    ///
    /// Returns an error if the manifest is missing or malformed, or if the
    /// final path component isn't a recognisable channel name.
    pub fn from_root(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        let last = root
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::UnknownChannel(root.display().to_string()))?;
        let channel = Channel::from_path_component(last)?;
        let manifest = read_build_manifest(&root)?;
        Ok(Self {
            channel,
            root,
            manifest,
        })
    }

    /// Build an installation from an explicit channel + root. Reads the
    /// manifest but does not try to infer the channel from the directory
    /// name. Use this when you already know the channel (e.g. from the
    /// launcher log).
    pub fn from_parts(channel: Channel, root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        let manifest = read_build_manifest(&root)?;
        Ok(Self {
            channel,
            root,
            manifest,
        })
    }

    // ── Paths ────────────────────────────────────────────────────────────

    /// Path to `Data.p4k` at `<root>/Data.p4k`.
    pub fn data_p4k(&self) -> PathBuf {
        self.root.join("Data.p4k")
    }

    /// Path to `user.cfg` at `<root>/user.cfg`.
    pub fn user_cfg(&self) -> PathBuf {
        self.root.join("user.cfg")
    }

    /// Path to `build_manifest.id` at `<root>/build_manifest.id`.
    pub fn build_manifest_path(&self) -> PathBuf {
        self.root.join("build_manifest.id")
    }

    /// Path to the localization directory for a given language,
    /// `<root>/data/Localization/<lang>/`.
    pub fn localization_dir(&self, lang: Language) -> PathBuf {
        self.root
            .join("data")
            .join("Localization")
            .join(lang.folder_name())
    }

    /// Path to the localization override `global.ini` for a given language,
    /// `<root>/data/Localization/<lang>/global.ini`.
    ///
    /// This is the *write target* for sc-langpatch. Star Citizen reads
    /// override files from the filesystem in preference to the versions
    /// shipped inside `Data.p4k`.
    pub fn localization_override(&self, lang: Language) -> PathBuf {
        self.localization_dir(lang).join("global.ini")
    }

    // ── Version helpers ──────────────────────────────────────────────────

    /// Short version derived from the manifest's full version string,
    /// e.g. `"4.6.173.39432"` → `"4.6"`. Takes the substring up to the
    /// second dot.
    pub fn short_version(&self) -> &str {
        let version = &self.manifest.version;
        let mut dots = 0;
        for (i, c) in version.char_indices() {
            if c == '.' {
                dots += 1;
                if dots == 2 {
                    return &version[..i];
                }
            }
        }
        version
    }

    /// Launcher-visible version format, e.g. `"4.7.1-live.11592622"`.
    ///
    /// Built from manifest `branch` (stripped of the `sc-alpha-` prefix) +
    /// lowercase channel name + changelist. Returns `None` if the manifest
    /// lacks a changelist (legacy manifests don't carry one).
    pub fn launcher_version_string(&self) -> Option<String> {
        let changelist = self.manifest.changelist.as_deref()?;
        let branch = self
            .manifest
            .branch
            .strip_prefix("sc-alpha-")
            .unwrap_or(&self.manifest.branch);
        Some(format!(
            "{branch}-{channel}.{changelist}",
            channel = self.channel.lowercase_name()
        ))
    }

    // ── Validation ──────────────────────────────────────────────────────

    /// True if the root directory exists AND `Data.p4k` is present.
    /// This is the same check the discovery functions use to filter
    /// invalid installs.
    pub fn is_valid(&self) -> bool {
        self.root.is_dir() && self.data_p4k().is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn make_manifest(version: &str, changelist: Option<&str>) -> BuildManifest {
        BuildManifest {
            version: version.to_string(),
            branch: "sc-alpha-4.6.1".to_string(),
            build_id: "12345".to_string(),
            changelist: changelist.map(str::to_string),
        }
    }

    fn make_install(channel: Channel, version: &str, changelist: Option<&str>) -> Installation {
        Installation {
            channel,
            root: PathBuf::from("C:\\SC\\LIVE"),
            manifest: make_manifest(version, changelist),
        }
    }

    #[test]
    fn short_version_full() {
        let inst = make_install(Channel::Live, "4.6.173.39432", None);
        assert_eq!(inst.short_version(), "4.6");
    }

    #[test]
    fn short_version_already_short() {
        let inst = make_install(Channel::Live, "4.6", None);
        assert_eq!(inst.short_version(), "4.6");
    }

    #[test]
    fn short_version_single_component() {
        let inst = make_install(Channel::Live, "4", None);
        assert_eq!(inst.short_version(), "4");
    }

    #[test]
    fn short_version_empty() {
        let inst = make_install(Channel::Live, "", None);
        assert_eq!(inst.short_version(), "");
    }

    #[test]
    fn launcher_version_string_is_derived_from_branch_not_version() {
        // Branch is "sc-alpha-4.6.1" → stripped to "4.6.1".
        // The manifest.version field is deliberately different to prove
        // that launcher_version_string ignores it.
        let inst = make_install(Channel::Live, "4.7.0.99", Some("42"));
        assert_eq!(
            inst.launcher_version_string().as_deref(),
            Some("4.6.1-live.42")
        );
    }

    #[test]
    fn launcher_version_string_eptu_uses_lowercase() {
        let inst = make_install(Channel::Eptu, "4.7.0.0", Some("77"));
        assert_eq!(
            inst.launcher_version_string().as_deref(),
            Some("4.6.1-eptu.77")
        );
    }

    #[test]
    fn launcher_version_string_without_changelist_is_none() {
        let inst = make_install(Channel::Live, "4.6.1.0", None);
        assert!(inst.launcher_version_string().is_none());
    }

    #[test]
    fn launcher_version_string_without_alpha_prefix() {
        // If branch doesn't start with "sc-alpha-", use it as-is.
        let inst = Installation {
            channel: Channel::Live,
            root: PathBuf::from("C:\\SC\\LIVE"),
            manifest: BuildManifest {
                version: "4.6.1.0".to_string(),
                branch: "4.6.1-release".to_string(),
                build_id: "1".to_string(),
                changelist: Some("99".to_string()),
            },
        };
        assert_eq!(
            inst.launcher_version_string().as_deref(),
            Some("4.6.1-release-live.99")
        );
    }

    #[test]
    fn data_p4k_path() {
        let inst = make_install(Channel::Live, "4.6.1.0", None);
        assert_eq!(inst.data_p4k(), PathBuf::from("C:\\SC\\LIVE\\Data.p4k"));
    }

    #[test]
    fn localization_override_english() {
        let inst = make_install(Channel::Live, "4.6.1.0", None);
        let expected: PathBuf = [
            "C:\\SC\\LIVE",
            "data",
            "Localization",
            "english",
            "global.ini",
        ]
        .iter()
        .collect();
        assert_eq!(inst.localization_override(Language::English), expected);
    }

    #[test]
    fn from_root_infers_channel_from_last_component() {
        let temp = tempfile::tempdir().unwrap();
        let live_dir = temp.path().join("LIVE");
        std::fs::create_dir_all(&live_dir).unwrap();
        let mut f = std::fs::File::create(live_dir.join("build_manifest.id")).unwrap();
        writeln!(
            f,
            r#"{{"Data":{{"Branch":"sc-alpha-4.6.1","BuildId":"1","Version":"4.6.1.0"}}}}"#
        )
        .unwrap();
        drop(f);

        let inst = Installation::from_root(&live_dir).unwrap();
        assert_eq!(inst.channel, Channel::Live);
        assert_eq!(inst.manifest.version, "4.6.1.0");
    }

    #[test]
    fn from_root_unknown_channel_errors() {
        let temp = tempfile::tempdir().unwrap();
        let weird_dir = temp.path().join("wut");
        std::fs::create_dir_all(&weird_dir).unwrap();
        let mut f = std::fs::File::create(weird_dir.join("build_manifest.id")).unwrap();
        writeln!(
            f,
            r#"{{"Data":{{"Branch":"sc-alpha-4.6.1","BuildId":"1","Version":"4.6.1.0"}}}}"#
        )
        .unwrap();
        drop(f);

        let err = Installation::from_root(&weird_dir).unwrap_err();
        assert!(matches!(err, Error::UnknownChannel(_)));
    }

    #[test]
    fn is_valid_requires_data_p4k_file() {
        let temp = tempfile::tempdir().unwrap();
        let install = temp.path().to_path_buf();
        let mut f = std::fs::File::create(install.join("build_manifest.id")).unwrap();
        writeln!(
            f,
            r#"{{"Data":{{"Branch":"sc-alpha-4.6.1","BuildId":"1","Version":"4.6.1.0"}}}}"#
        )
        .unwrap();
        drop(f);

        let inst = Installation::from_parts(Channel::Live, &install).unwrap();
        assert!(
            !inst.is_valid(),
            "is_valid should be false without Data.p4k"
        );

        std::fs::write(install.join("Data.p4k"), b"fake p4k bytes").unwrap();
        assert!(
            inst.is_valid(),
            "is_valid should be true once Data.p4k exists"
        );
    }
}
