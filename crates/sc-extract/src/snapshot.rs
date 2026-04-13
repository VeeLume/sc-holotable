//! The [`ExtractSnapshot`] on-disk format — msgpack + zstd, atomic write.
//!
//! A snapshot is the serializable projection of one parse run: it pairs
//! a [`DatacoreSnapshot`] (from [`crate::Datacore::into_snapshot`]) with an
//! [`AssetData`] (from [`crate::AssetData::extract`]) and a [`SnapshotMeta`]
//! describing which install it came from and when.
//!
//! Both payload halves are `Option<_>` so the same file format covers all
//! four consumer patterns:
//!
//! - datacore only         — `asset_data == None`
//! - assets only           — `datacore == None`
//! - both                  — the usual case
//! - neither (empty)       — legal but unusual
//!
//! # Format
//!
//! 1. [`ExtractSnapshot`] → msgpack via `rmp_serde::to_vec_named` (named
//!    fields so serde struct-shape evolution is forgiving).
//! 2. Compressed with `zstd` at level 3 (good size/speed tradeoff).
//! 3. Written atomically: `save` writes to `<path>.tmp` and renames over
//!    the destination, so a crash mid-write never leaves a partial file.
//!
//! `load` decompresses, decodes, and verifies [`ExtractSnapshot::SCHEMA_VERSION`].
//! A mismatch returns [`Error::SnapshotVersionMismatch`] and the consumer
//! should fall back to a fresh parse.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::asset_data::AssetData;
use crate::datacore::DatacoreSnapshot;
use crate::error::{Error, Result};

/// Metadata attached to every [`ExtractSnapshot`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnapshotMeta {
    /// Schema version of this snapshot. Bumped on incompatible layout changes.
    pub schema_version: u32,

    /// Game version string (e.g. `"4.7.0-live.11592622"`). Best effort —
    /// may be `"unknown"` if the caller didn't pass an install manifest.
    pub game_version: String,

    /// Build ID from the install's `build_manifest.id`. Best effort.
    pub build_id: String,

    /// Timestamp of when the snapshot was taken (UTC, best-effort ISO-ish).
    pub extracted_at: String,
}

/// Serializable pairing of [`AssetData`] and [`DatacoreSnapshot`] plus
/// descriptive metadata. This is the on-disk snapshot format.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractSnapshot {
    /// When, which install, which schema version.
    pub meta: SnapshotMeta,

    /// Optional asset-sourced data (locale, etc.). `None` if the snapshot
    /// was produced from a datacore-only parse.
    #[serde(default)]
    pub asset_data: Option<AssetData>,

    /// Optional datacore snapshot. `None` if the snapshot was produced
    /// from an assets-only extraction.
    #[serde(default)]
    pub datacore: Option<DatacoreSnapshot>,
}

impl ExtractSnapshot {
    /// Current snapshot schema version. Bump on incompatible layout changes
    /// so older snapshots fail [`ExtractSnapshot::load`] cleanly.
    ///
    /// **v4** — rework: `ExtractSnapshot { meta, asset_data, datacore }`
    /// replaces the old `ExtractedData` god-struct. Not backward compatible
    /// with v3 (schema layout moved; asset data is now optional).
    ///
    /// **v3** — `ReferenceGraph` simplified: edges are `Guid → Vec<Guid>`
    /// (no `field_path`).
    ///
    /// **v2** — flat-pool refactor: `RecordStore` holds `DataPools` +
    /// `RecordIndex`.
    pub const SCHEMA_VERSION: u32 = 4;

    /// Zstd compression level for snapshots. Level 3 is zstd's default and
    /// gives a good size/speed tradeoff for msgpack payloads of this shape.
    const ZSTD_LEVEL: i32 = 3;

    /// Create an empty snapshot with the current schema version set and
    /// no payloads.
    pub fn new() -> Self {
        Self {
            meta: SnapshotMeta {
                schema_version: Self::SCHEMA_VERSION,
                ..SnapshotMeta::default()
            },
            asset_data: None,
            datacore: None,
        }
    }

    /// Serialise to the snapshot format and write atomically to `path`.
    ///
    /// Writes through a sibling `<path>.tmp` first and then renames over
    /// the destination, so a crash mid-write leaves the previous snapshot
    /// (if any) intact.
    pub fn save(&self, path: &Path) -> Result<()> {
        let msgpack = rmp_serde::to_vec_named(self)
            .map_err(|e| Error::SnapshotEncode(e.to_string()))?;

        let compressed = zstd::stream::encode_all(msgpack.as_slice(), Self::ZSTD_LEVEL)
            .map_err(Error::SnapshotCompression)?;

        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|source| Error::SnapshotWrite {
                path: path.to_path_buf(),
                source,
            })?;
        }

        let tmp_path: PathBuf = {
            let mut p = path.as_os_str().to_os_string();
            p.push(".tmp");
            PathBuf::from(p)
        };

        fs::write(&tmp_path, &compressed).map_err(|source| Error::SnapshotWrite {
            path: tmp_path.clone(),
            source,
        })?;

        fs::rename(&tmp_path, path).map_err(|source| Error::SnapshotWrite {
            path: path.to_path_buf(),
            source,
        })?;

        tracing::info!(
            path = %path.display(),
            bytes = compressed.len(),
            "snapshot saved"
        );
        Ok(())
    }

    /// Read a snapshot from disk. Verifies the schema version on load.
    ///
    /// Returns [`Error::SnapshotVersionMismatch`] if the file was written
    /// by a different build of sc-extract.
    pub fn load(path: &Path) -> Result<Self> {
        let compressed = fs::read(path).map_err(|source| Error::SnapshotRead {
            path: path.to_path_buf(),
            source,
        })?;

        let msgpack = zstd::stream::decode_all(compressed.as_slice())
            .map_err(Error::SnapshotCompression)?;

        let data: Self = rmp_serde::from_slice(&msgpack)
            .map_err(|e| Error::SnapshotDecode(e.to_string()))?;

        if data.meta.schema_version != Self::SCHEMA_VERSION {
            return Err(Error::SnapshotVersionMismatch {
                expected: Self::SCHEMA_VERSION,
                found: data.meta.schema_version,
            });
        }

        tracing::info!(
            path = %path.display(),
            records = data.datacore.as_ref().map(|d| d.records.len()).unwrap_or(0),
            has_assets = data.asset_data.is_some(),
            "snapshot loaded"
        );
        Ok(data)
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_snapshot_path(name: &str) -> PathBuf {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join(name);
        std::mem::forget(dir);
        path
    }

    #[test]
    fn new_sets_schema_version() {
        let snap = ExtractSnapshot::new();
        assert_eq!(snap.meta.schema_version, ExtractSnapshot::SCHEMA_VERSION);
        assert!(snap.asset_data.is_none());
        assert!(snap.datacore.is_none());
    }

    #[test]
    fn save_and_load_round_trip() {
        let mut snap = ExtractSnapshot::new();
        snap.meta.game_version = "4.7.0".into();
        snap.meta.build_id = "test-build".into();
        snap.meta.extracted_at = "2026-04-13T00:00:00Z".into();
        snap.asset_data = Some(AssetData::default());
        snap.datacore = Some(DatacoreSnapshot::default());

        let path = tmp_snapshot_path("round_trip.snap");
        snap.save(&path).expect("save");

        let loaded = ExtractSnapshot::load(&path).expect("load");
        assert_eq!(loaded.meta.schema_version, ExtractSnapshot::SCHEMA_VERSION);
        assert_eq!(loaded.meta.game_version, "4.7.0");
        assert_eq!(loaded.meta.build_id, "test-build");
        assert_eq!(loaded.meta.extracted_at, "2026-04-13T00:00:00Z");
        assert!(loaded.asset_data.is_some());
        assert!(loaded.datacore.is_some());
    }

    #[test]
    fn load_rejects_wrong_schema_version() {
        let mut snap = ExtractSnapshot::new();
        snap.meta.schema_version = ExtractSnapshot::SCHEMA_VERSION + 99;

        let path = tmp_snapshot_path("wrong_version.snap");
        let msgpack = rmp_serde::to_vec_named(&snap).unwrap();
        let compressed =
            zstd::stream::encode_all(msgpack.as_slice(), ExtractSnapshot::ZSTD_LEVEL).unwrap();
        std::fs::write(&path, compressed).unwrap();

        match ExtractSnapshot::load(&path) {
            Err(Error::SnapshotVersionMismatch { expected, found }) => {
                assert_eq!(expected, ExtractSnapshot::SCHEMA_VERSION);
                assert_eq!(found, ExtractSnapshot::SCHEMA_VERSION + 99);
            }
            other => panic!("expected SnapshotVersionMismatch, got {other:?}"),
        }
    }

    #[test]
    fn atomic_write_doesnt_leave_tmp_behind() {
        let snap = ExtractSnapshot::new();
        let path = tmp_snapshot_path("atomic.snap");
        snap.save(&path).expect("save");

        let mut tmp = path.as_os_str().to_os_string();
        tmp.push(".tmp");
        let tmp = PathBuf::from(tmp);

        assert!(path.exists(), "target snapshot should exist");
        assert!(!tmp.exists(), "temp file should have been renamed away");
    }

    #[test]
    fn assets_only_snapshot() {
        let mut snap = ExtractSnapshot::new();
        snap.asset_data = Some(AssetData::default());

        let path = tmp_snapshot_path("assets_only.snap");
        snap.save(&path).expect("save");

        let loaded = ExtractSnapshot::load(&path).expect("load");
        assert!(loaded.asset_data.is_some());
        assert!(loaded.datacore.is_none());
    }

    #[test]
    fn datacore_only_snapshot() {
        let mut snap = ExtractSnapshot::new();
        snap.datacore = Some(DatacoreSnapshot::default());

        let path = tmp_snapshot_path("datacore_only.snap");
        snap.save(&path).expect("save");

        let loaded = ExtractSnapshot::load(&path).expect("load");
        assert!(loaded.asset_data.is_none());
        assert!(loaded.datacore.is_some());
    }
}
