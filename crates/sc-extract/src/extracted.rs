//! The `ExtractedData` envelope — a fully-owned, serialisable snapshot of
//! every DCB-derived value sc-extract produces in a single pass.
//!
//! `ExtractedData` is the "pure data" half of the Phase 2d layering:
//!
//! - It owns the [`RecordStore`], [`ReferenceGraph`], [`TagTree`],
//!   [`ManufacturerRegistry`], [`DisplayNameCache`], and [`LocaleMap`].
//! - It does **not** hold a live `P4kArchive`. Consumers that want
//!   on-demand asset access hold a separate [`AssetSource`] alongside it.
//!
//! This split keeps snapshots small and portable while still letting
//! consumers that need raw P4K files reach them via [`AssetSource`].
//!
//! # Snapshot format
//!
//! Snapshots are encoded as **msgpack + zstd**:
//!
//! 1. `ExtractedData` → msgpack bytes via `rmp_serde::to_vec_named`
//!    (named fields so the format survives field-order reshuffles).
//! 2. Compressed with `zstd` at a default level.
//! 3. Written atomically: `save_snapshot` writes to `<path>.tmp` and then
//!    renames over the destination so a crash mid-write never leaves a
//!    partial file in place.
//!
//! `load_snapshot` decompresses, decodes, and verifies the
//! [`ExtractedData::SCHEMA_VERSION`]. A mismatch returns
//! [`Error::SnapshotVersionMismatch`] so consumers can fall back to a fresh
//! `parse_from_p4k` run.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::display_names::DisplayNameCache;
use crate::error::{Error, Result};
use crate::generated::RecordStore;
use crate::graph::ReferenceGraph;
use crate::locale::LocaleMap;
use crate::manufacturers::ManufacturerRegistry;
use crate::tags::TagTree;

/// Bundled DCB-derived state: every record plus every index / registry /
/// cache built from the same parse pass.
///
/// Produced by [`crate::parse_from_p4k`] / [`crate::parse_snapshot_only`],
/// serialised by [`ExtractedData::save_snapshot`], and restored by
/// [`ExtractedData::load_snapshot`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExtractedData {
    /// Schema version of this snapshot. Bumped on incompatible changes.
    pub schema_version: u32,

    /// Game version string (e.g. `"4.7"`). Best effort — may be `"unknown"`
    /// if the launcher metadata next to `Data.p4k` is missing.
    pub game_version: String,

    /// Build ID from the install's `build_manifest.id`. Best effort.
    pub build_id: String,

    /// ISO-8601 timestamp (UTC) of when this data was extracted.
    pub extracted_at: String,

    /// Every top-level DCB record, split by concrete Rust type.
    #[serde(default)]
    pub records: RecordStore,

    /// Cross-record reference graph (forward + reverse edges).
    #[serde(default)]
    pub graph: ReferenceGraph,

    /// Hierarchical tag tree.
    #[serde(default)]
    pub tag_tree: TagTree,

    /// Manufacturer lookup by GUID and ticker code.
    #[serde(default)]
    pub manufacturers: ManufacturerRegistry,

    /// Pre-computed localized entity display names.
    #[serde(default)]
    pub display_names: DisplayNameCache,

    /// Parsed `global.ini` localization table.
    #[serde(default)]
    pub locale: LocaleMap,
}

impl ExtractedData {
    /// Current snapshot schema version. Bump on incompatible layout changes
    /// so older snapshots fail [`ExtractedData::load_snapshot`] cleanly.
    ///
    /// **v3** — `ReferenceGraph` simplified: edges are now `Guid → Vec<Guid>`
    /// (no `field_path` strings). v2 snapshots stored `Edge { from, to,
    /// field_path, kind }` and are not loadable.
    ///
    /// **v2** — flat-pool refactor: `RecordStore` now holds `DataPools`
    /// (per-type `Vec<Option<T>>`) plus `RecordIndex` (per-type
    /// `HashMap<CigGuid, TId>`). Old v1 snapshots stored nested
    /// `Option<Box<T>>` trees and are not loadable.
    pub const SCHEMA_VERSION: u32 = 3;

    /// Zstd compression level for snapshots. Level 3 is zstd's default and
    /// gives a good size/speed tradeoff for msgpack payloads of this shape.
    const ZSTD_LEVEL: i32 = 3;

    /// Create an empty envelope with the current schema version set.
    pub fn new() -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            ..Self::default()
        }
    }

    /// Serialise to the snapshot format and write atomically to `path`.
    ///
    /// Writes through a sibling `<path>.tmp` first and then renames over
    /// the destination, so a crash mid-write leaves the previous snapshot
    /// (if any) intact.
    pub fn save_snapshot(&self, path: &Path) -> Result<()> {
        let msgpack = rmp_serde::to_vec_named(self)
            .map_err(|e| Error::SnapshotEncode(e.to_string()))?;

        let compressed =
            zstd::stream::encode_all(msgpack.as_slice(), Self::ZSTD_LEVEL)
                .map_err(Error::SnapshotCompression)?;

        // Ensure the parent exists so `tempfile`-style writes don't race.
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
    pub fn load_snapshot(path: &Path) -> Result<Self> {
        let compressed = fs::read(path).map_err(|source| Error::SnapshotRead {
            path: path.to_path_buf(),
            source,
        })?;

        let msgpack = zstd::stream::decode_all(compressed.as_slice())
            .map_err(Error::SnapshotCompression)?;

        let data: Self = rmp_serde::from_slice(&msgpack)
            .map_err(|e| Error::SnapshotDecode(e.to_string()))?;

        if data.schema_version != Self::SCHEMA_VERSION {
            return Err(Error::SnapshotVersionMismatch {
                expected: Self::SCHEMA_VERSION,
                found: data.schema_version,
            });
        }

        tracing::info!(
            path = %path.display(),
            records = data.records.len(),
            "snapshot loaded"
        );
        Ok(data)
    }

    /// Total number of DCB records held across all top-level types.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_snapshot_path(name: &str) -> PathBuf {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join(name);
        // Leak the tempdir — save/load are fast enough that we don't care
        // about cleanup in unit tests.
        std::mem::forget(dir);
        path
    }

    #[test]
    fn new_sets_schema_version() {
        let data = ExtractedData::new();
        assert_eq!(data.schema_version, ExtractedData::SCHEMA_VERSION);
        assert!(data.records.is_empty());
        assert_eq!(data.record_count(), 0);
    }

    #[test]
    fn save_and_load_round_trip() {
        let mut data = ExtractedData::new();
        data.game_version = "4.7.0".into();
        data.build_id = "test-build".into();
        data.extracted_at = "2026-04-11T00:00:00Z".into();

        let path = tmp_snapshot_path("round_trip.snap");
        data.save_snapshot(&path).expect("save");

        let loaded = ExtractedData::load_snapshot(&path).expect("load");
        assert_eq!(loaded.schema_version, ExtractedData::SCHEMA_VERSION);
        assert_eq!(loaded.game_version, "4.7.0");
        assert_eq!(loaded.build_id, "test-build");
        assert_eq!(loaded.extracted_at, "2026-04-11T00:00:00Z");
    }

    #[test]
    fn load_rejects_wrong_schema_version() {
        // Craft a snapshot with a bumped schema version and verify load rejects it.
        let mut data = ExtractedData::new();
        data.schema_version = ExtractedData::SCHEMA_VERSION + 99;

        let path = tmp_snapshot_path("wrong_version.snap");
        let msgpack = rmp_serde::to_vec_named(&data).unwrap();
        let compressed =
            zstd::stream::encode_all(msgpack.as_slice(), ExtractedData::ZSTD_LEVEL).unwrap();
        std::fs::write(&path, compressed).unwrap();

        match ExtractedData::load_snapshot(&path) {
            Err(Error::SnapshotVersionMismatch { expected, found }) => {
                assert_eq!(expected, ExtractedData::SCHEMA_VERSION);
                assert_eq!(found, ExtractedData::SCHEMA_VERSION + 99);
            }
            other => panic!("expected SnapshotVersionMismatch, got {other:?}"),
        }
    }

    #[test]
    fn atomic_write_doesnt_leave_tmp_behind() {
        let data = ExtractedData::new();
        let path = tmp_snapshot_path("atomic.snap");
        data.save_snapshot(&path).expect("save");

        let mut tmp = path.as_os_str().to_os_string();
        tmp.push(".tmp");
        let tmp = PathBuf::from(tmp);

        assert!(path.exists(), "target snapshot should exist");
        assert!(!tmp.exists(), "temp file should have been renamed away");
    }
}
