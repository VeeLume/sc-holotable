//! Processed snapshot — a single **cooked index**, serialized for fast load.
//!
//! Where [`crate::ExtractSnapshot`] archives the raw DCB *inputs* (and must
//! re-parse + re-materialize ~34s on `hydrate`, per the Step-0 measurement), a
//! [`ProcessedSnapshot`] stores a cooked index's *output* directly — so
//! loading it skips both parse and build entirely (sub-second).
//!
//! It is **generic** and names no domain type: any `T: Serialize +
//! DeserializeOwned` can ride in it. The hand-written cooked indices
//! ([`crate::RecordPaths`], `sc-tags`' `Tags`, `sc-manufacturers`'
//! `Manufacturers`) are serde-clean (primitives only) and qualify
//! today. Anything reaching a generated type (e.g. `sc-items`' `Items`,
//! which embeds the generated `EItemType` / `LocaleKey`) does **not** — those
//! types carry no serde and giving them serde would be a generated-crate
//! change, the exact compile-time cliff the byte-bundle `ExtractSnapshot`
//! exists to avoid. See the note's finding 1.
//!
//! # Two version numbers
//!
//! - [`ProcessedSnapshot::ENVELOPE_VERSION`] — this wrapper's own format.
//!   Bumped if the envelope layout changes.
//! - `cook_schema_version` — the version of the cooked index *inside*,
//!   owned by the caller (each index type bumps its own when its layout
//!   changes). [`ProcessedSnapshot::load`] takes the expected value and
//!   rejects a stale file with [`Error::ProcessedSnapshotStale`], so the
//!   caller falls back to a raw hydrate or fresh build.
//!
//! [`SnapshotMeta`] rides along purely for provenance (build_id /
//! game_version / extracted_at): "which parse run was this cooked from?"
//!
//! # Format
//!
//! Identical machinery to [`crate::ExtractSnapshot`]: `rmp_serde` named
//! msgpack → zstd level 3 → atomic `<path>.tmp`-then-rename write.

use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::snapshot::SnapshotMeta;

/// A serialized cooked index plus provenance and versioning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedSnapshot<T> {
    /// Provenance of the parse run this index was cooked from. Its
    /// `schema_version` field is the raw [`crate::ExtractSnapshot`] version of
    /// that run (traceability only) — *not* the cook format version.
    pub meta: SnapshotMeta,
    /// Version of this [`ProcessedSnapshot`] envelope format.
    pub envelope_version: u32,
    /// Version of the cooked index format inside. Caller-owned, per index type.
    pub cook_schema_version: u32,
    /// The cooked index itself.
    pub index: T,
}

impl<T> ProcessedSnapshot<T>
where
    T: Serialize + DeserializeOwned,
{
    /// Current envelope format version.
    pub const ENVELOPE_VERSION: u32 = 1;

    /// Zstd level — matches [`crate::ExtractSnapshot`].
    const ZSTD_LEVEL: i32 = 3;

    /// Wrap a cooked index with its provenance and cook-format version.
    pub fn new(meta: SnapshotMeta, cook_schema_version: u32, index: T) -> Self {
        Self {
            meta,
            envelope_version: Self::ENVELOPE_VERSION,
            cook_schema_version,
            index,
        }
    }

    /// Serialize and write atomically to `path` (`<path>.tmp` then rename).
    pub fn save(&self, path: &Path) -> Result<()> {
        let msgpack =
            rmp_serde::to_vec_named(self).map_err(|e| Error::SnapshotEncode(e.to_string()))?;
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
        Ok(())
    }

    /// Read a processed snapshot, verifying both the envelope version and the
    /// cooked-index version.
    ///
    /// Returns [`Error::SnapshotVersionMismatch`] if the envelope format
    /// differs, or [`Error::ProcessedSnapshotStale`] if the cooked index
    /// format differs from `expected_cook_version`. Either way the caller
    /// should fall back to a raw hydrate or fresh parse + rebuild.
    pub fn load(path: &Path, expected_cook_version: u32) -> Result<Self> {
        let compressed = fs::read(path).map_err(|source| Error::SnapshotRead {
            path: path.to_path_buf(),
            source,
        })?;
        let msgpack =
            zstd::stream::decode_all(compressed.as_slice()).map_err(Error::SnapshotCompression)?;
        let data: Self =
            rmp_serde::from_slice(&msgpack).map_err(|e| Error::SnapshotDecode(e.to_string()))?;

        if data.envelope_version != Self::ENVELOPE_VERSION {
            return Err(Error::SnapshotVersionMismatch {
                expected: Self::ENVELOPE_VERSION,
                found: data.envelope_version,
            });
        }
        if data.cook_schema_version != expected_cook_version {
            return Err(Error::ProcessedSnapshotStale {
                expected: expected_cook_version,
                found: data.cook_schema_version,
            });
        }
        Ok(data)
    }

    /// Consume the snapshot and return just the cooked index.
    pub fn into_index(self) -> T {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RecordPath, RecordPaths};

    fn tmp(name: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join(name);
        (dir, path)
    }

    #[test]
    fn round_trip_and_version_guard() {
        let (_dir, path) = tmp("vec.cook");
        let snap = ProcessedSnapshot::new(SnapshotMeta::default(), 7, vec![1u32, 2, 3]);
        snap.save(&path).unwrap();

        // Matching cook version loads.
        let loaded: ProcessedSnapshot<Vec<u32>> = ProcessedSnapshot::load(&path, 7).unwrap();
        assert_eq!(loaded.index, vec![1, 2, 3]);
        assert_eq!(loaded.cook_schema_version, 7);

        // Stale cook version is rejected with the distinct error.
        let err = ProcessedSnapshot::<Vec<u32>>::load(&path, 8);
        assert!(matches!(
            err,
            Err(Error::ProcessedSnapshotStale {
                expected: 8,
                found: 7
            })
        ));
    }

    #[test]
    fn round_trips_a_real_index() {
        let (_dir, path) = tmp("paths.cook");
        let mut paths = RecordPaths::new();
        paths.insert(RecordPath {
            guid: crate::Guid::from_bytes([1; 16]),
            name: "ARMA".into(),
            struct_index: 0,
            is_main: true,
            path: "libs/foundry/records/scitemmanufacturer/arma.xml".into(),
        });

        ProcessedSnapshot::new(SnapshotMeta::default(), 1, paths)
            .save(&path)
            .unwrap();

        let loaded: ProcessedSnapshot<RecordPaths> = ProcessedSnapshot::load(&path, 1).unwrap();
        assert_eq!(loaded.index.len(), 1);
        assert_eq!(
            loaded
                .index
                .at("libs/foundry/records/scitemmanufacturer/arma.xml")
                .len(),
            1
        );
    }
}
