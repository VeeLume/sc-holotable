//! The [`ExtractSnapshot`] on-disk format — raw captured bytes + msgpack + zstd.
//!
//! A snapshot is a byte-level capture of one parse run's *inputs*, not its
//! *outputs*. It bundles the raw `game2.dcb`, the raw `english/global.ini`,
//! and (optionally) other archived files from a live `Data.p4k`, wraps them
//! in a small msgpack envelope with metadata, and zstd-compresses the
//! whole thing.
//!
//! # Why bytes and not cooked types?
//!
//! Earlier revisions of this format serialized [`crate::DatacoreSnapshot`]
//! (the cooked output of [`crate::Datacore::parse`]) directly. That worked
//! functionally but caused a catastrophic compile-time regression in
//! `sc-extract`: `rmp_serde::to_vec_named(&snapshot)` transitively
//! monomorphized `Serialize`/`Deserialize` for every generated type
//! reachable from `DataPools`, pushing this crate's LLVM IR to 4+ million
//! lines and full-feature cold builds to 3+ hours.
//!
//! Storing the raw DCB bytes and re-parsing at load time sidesteps the
//! problem entirely: the type graph of `ExtractSnapshot` becomes
//! `{u32, String, Vec<u8>, BTreeMap}`, a handful of primitive monomorphizations,
//! and nothing in the workspace instantiates serde for generated types.
//! See `docs/benchmarks.md` for the compile-time numbers.
//!
//! It also happens to be the right fit for bulkhead's historical-snapshot
//! comparison feature: the DCB is a stable game format, so an archived
//! 4.7 snapshot parses correctly against any future version of our
//! generated types, with field resolution happening at parse time.
//! Field renames, type changes, and schema evolution all "just work"
//! because the parse-time binding is deferred to load time.
//!
//! # Format
//!
//! 1. [`ExtractSnapshot`] → msgpack via `rmp_serde::to_vec_named` (named
//!    fields so envelope evolution stays forgiving).
//! 2. Compressed with `zstd` at level 3 (good size/speed tradeoff).
//! 3. Written atomically: [`ExtractSnapshot::save`] writes to `<path>.tmp`
//!    and renames over the destination, so a crash mid-write never leaves
//!    a partial file.
//!
//! [`ExtractSnapshot::load`] decompresses, decodes, and verifies
//! [`ExtractSnapshot::SCHEMA_VERSION`]. A mismatch returns
//! [`Error::SnapshotVersionMismatch`]; the consumer should fall back to a
//! fresh parse.
//!
//! # The capture / load / hydrate split
//!
//! Working with a snapshot goes through three phases, which are
//! intentionally decoupled so consumers only pay for what they use:
//!
//! - [`ExtractSnapshot::capture`] — read the raw bytes out of a live
//!   [`AssetSource`] and build the `files` map. Runs at extract time,
//!   owns the choice of what to archive via [`SnapshotCaptureConfig`].
//! - [`ExtractSnapshot::save`] / [`ExtractSnapshot::load`] — zstd +
//!   msgpack I/O of the envelope. Cheap: load just reads the captured
//!   bytes into memory without parsing the DCB. Consumers enumerating
//!   historical snapshots (e.g. bulkhead's patch-diff UI) can inspect
//!   [`SnapshotMeta`] for every snapshot without paying re-parse cost.
//! - [`ExtractSnapshot::hydrate`] — re-parse the captured DCB + locale
//!   bytes into a live [`crate::Datacore`] + [`crate::AssetData`]. This
//!   is where the ~15–25s parse cost lives; consumers call it on the one
//!   snapshot they actually want to query.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::asset_data::{AssetConfig, AssetData};
use crate::assets::AssetSource;
use crate::config::DatacoreConfig;
use crate::datacore::Datacore;
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

/// A captured byte-level snapshot of a parse run's source files.
///
/// `files` is keyed by the in-archive path of each captured file (the same
/// case-insensitive string used by [`crate::AssetSource::find_and_read`]).
/// The minimum useful snapshot contains `game2.dcb`; a standard snapshot
/// also contains `english/global.ini` for locale.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractSnapshot {
    /// When, which install, which schema version.
    pub meta: SnapshotMeta,

    /// Captured raw bytes keyed by in-archive path. At minimum contains
    /// `game2.dcb`; the set is controlled by [`SnapshotCaptureConfig`]
    /// at capture time.
    #[serde(default)]
    pub files: BTreeMap<String, Vec<u8>>,
}

/// Controls which files [`ExtractSnapshot::capture`] copies out of a live
/// [`AssetSource`] into the snapshot's `files` map.
///
/// Adding a new category here is a non-breaking change: the on-disk format
/// is a generic `BTreeMap<String, Vec<u8>>`, so new categories just produce
/// new entries under their in-archive paths. No [`ExtractSnapshot::SCHEMA_VERSION`]
/// bump is required.
#[derive(Debug, Clone)]
pub struct SnapshotCaptureConfig {
    /// Archive `game2.dcb`. **Must be `true`** for any snapshot intended
    /// to be hydrated — [`ExtractSnapshot::hydrate`] returns
    /// [`Error::SnapshotMissingDcb`] otherwise.
    pub archive_dcb: bool,

    /// Archive `english/global.ini` (the localization file).
    pub archive_locale: bool,

    /// Archive every vehicle XML the generator knows about. **Not yet
    /// implemented** — this field is wired as a placeholder so phase 2b
    /// (vehicle XML parsing) can flip it on without a format change. Has
    /// no effect today.
    pub archive_vehicle_xmls: bool,
}

impl SnapshotCaptureConfig {
    /// Default archival set: DCB + locale. This is what most consumers
    /// should use.
    pub fn standard() -> Self {
        Self {
            archive_dcb: true,
            archive_locale: true,
            archive_vehicle_xmls: false,
        }
    }

    /// DCB only — skip locale. Useful if the consumer knows it won't
    /// need display-name resolution.
    pub fn datacore_only() -> Self {
        Self {
            archive_dcb: true,
            archive_locale: false,
            archive_vehicle_xmls: false,
        }
    }

    /// Nothing. Produces an empty `files` map. Legal but produces a
    /// snapshot that cannot be hydrated — useful for meta-only test
    /// fixtures.
    pub fn minimal() -> Self {
        Self {
            archive_dcb: false,
            archive_locale: false,
            archive_vehicle_xmls: false,
        }
    }
}

impl Default for SnapshotCaptureConfig {
    fn default() -> Self {
        Self::standard()
    }
}

impl ExtractSnapshot {
    /// Current snapshot schema version. Bump on incompatible envelope
    /// changes so older snapshots fail [`ExtractSnapshot::load`] cleanly.
    ///
    /// **v5** — byte-bundle rework: `ExtractSnapshot { meta, files }`.
    /// Captured bytes replace the cooked [`crate::DatacoreSnapshot`] +
    /// [`crate::AssetData`] projection. Hydration re-parses on load;
    /// forward/backward compatibility across generator regenerations is
    /// automatic because the DCB format is game-stable. Not backward
    /// compatible with v4 (layout changed).
    ///
    /// **v4** — rework: `ExtractSnapshot { meta, asset_data, datacore }`
    /// replaced the old `ExtractedData` god-struct.
    ///
    /// **v3** — `ReferenceGraph` simplified: edges are `Guid → Vec<Guid>`
    /// (no `field_path`).
    ///
    /// **v2** — flat-pool refactor: `RecordStore` holds `DataPools` +
    /// `RecordIndex`.
    pub const SCHEMA_VERSION: u32 = 5;

    /// Zstd compression level for snapshots. Level 3 is zstd's default and
    /// gives a good size/speed tradeoff for mixed msgpack + DCB payloads.
    const ZSTD_LEVEL: i32 = 3;

    /// Create an empty snapshot with the current schema version set and
    /// no captured files. Useful mostly for tests.
    pub fn new() -> Self {
        Self {
            meta: SnapshotMeta {
                schema_version: Self::SCHEMA_VERSION,
                ..SnapshotMeta::default()
            },
            files: BTreeMap::new(),
        }
    }

    /// Build a snapshot by reading raw bytes out of a live [`AssetSource`].
    ///
    /// For each file category enabled in `config`, this method locates
    /// the file via the same predicate used by
    /// [`crate::Datacore::parse`] / [`crate::AssetData::extract`] and
    /// inserts the bytes into `files` keyed by the returned archive path.
    ///
    /// Fails with [`Error::DcbNotFound`] if `archive_dcb` is true but no
    /// `game2.dcb` entry is present — every practical use of a snapshot
    /// wants the DCB, so silent omission would just produce a landmine.
    /// Missing `global.ini` is not fatal: a warning is logged and capture
    /// continues, matching [`crate::AssetData::extract`]'s forgiving
    /// behavior.
    pub fn capture(
        assets: &AssetSource,
        meta: SnapshotMeta,
        config: &SnapshotCaptureConfig,
    ) -> Result<Self> {
        let start = std::time::Instant::now();
        let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();

        if config.archive_dcb {
            tracing::info!("capturing game2.dcb");
            match assets
                .find_and_read(|name| name.to_ascii_lowercase().ends_with("game2.dcb"))?
            {
                Some((path, bytes)) => {
                    tracing::info!(
                        path = %path,
                        bytes = bytes.len(),
                        "captured game2.dcb"
                    );
                    files.insert(path, bytes);
                }
                None => return Err(Error::DcbNotFound),
            }
        }

        if config.archive_locale {
            tracing::info!("capturing english/global.ini");
            match assets
                .find_and_read(|name| name.to_ascii_lowercase().ends_with("english\\global.ini"))?
            {
                Some((path, bytes)) => {
                    tracing::info!(
                        path = %path,
                        bytes = bytes.len(),
                        "captured english/global.ini"
                    );
                    files.insert(path, bytes);
                }
                None => {
                    tracing::warn!(
                        "no english/global.ini found in archive; snapshot will have no locale"
                    );
                }
            }
        }

        if config.archive_vehicle_xmls {
            // Placeholder — phase 2b will fill this in. Today it's a no-op
            // rather than a hard error so a forward-looking consumer that
            // flips the flag doesn't get a broken build the instant the
            // field exists.
            tracing::warn!(
                "archive_vehicle_xmls is not yet implemented; no vehicle XMLs will be captured"
            );
        }

        let total_bytes: usize = files.values().map(Vec::len).sum();
        tracing::info!(
            entries = files.len(),
            total_bytes = total_bytes,
            elapsed_ms = start.elapsed().as_millis(),
            "snapshot capture complete"
        );

        Ok(Self { meta, files })
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
    /// by a different build of sc-extract. **Does not** parse the DCB —
    /// call [`ExtractSnapshot::hydrate`] for that.
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

        let total_bytes: usize = data.files.values().map(Vec::len).sum();
        tracing::info!(
            path = %path.display(),
            files = data.files.len(),
            total_bytes = total_bytes,
            "snapshot loaded"
        );
        Ok(data)
    }

    /// Re-parse the captured bytes into a live [`Datacore`] + [`AssetData`].
    ///
    /// Constructs an in-memory [`AssetSource`] from `self.files`, then
    /// drives it through the standard extract pipeline
    /// ([`AssetData::extract`] + [`Datacore::parse`]). This is where the
    /// ~15–25s parse cost lives — consumers should call it lazily on the
    /// one snapshot they actually want to query, not eagerly for every
    /// snapshot in a listing.
    ///
    /// Returns [`Error::SnapshotMissingDcb`] if the captured files map
    /// has no entry ending in `game2.dcb`. Missing `global.ini` is not
    /// fatal — [`AssetData::extract`] falls back to an empty locale map
    /// with a warning.
    pub fn hydrate(
        &self,
        asset_config: &AssetConfig,
        dc_config: &DatacoreConfig,
    ) -> Result<(AssetData, Datacore)> {
        let has_dcb = self
            .files
            .keys()
            .any(|k| k.to_ascii_lowercase().ends_with("game2.dcb"));
        if !has_dcb {
            return Err(Error::SnapshotMissingDcb);
        }

        let label = format!(
            "snapshot://game_version={}&build_id={}",
            self.meta.game_version, self.meta.build_id
        );
        let source = AssetSource::from_snapshot(self.files.clone(), label);

        let asset_data = AssetData::extract(&source, asset_config)?;
        let datacore = Datacore::parse(&source, &asset_data, dc_config)?;

        Ok((asset_data, datacore))
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
        assert!(snap.files.is_empty());
    }

    #[test]
    fn round_trips_arbitrary_files_map() {
        // Hand-build a snapshot with a few synthetic file entries, save
        // it, load it, and assert byte-for-byte equality. This exercises
        // the entire save/load path without needing a real DCB.
        let mut snap = ExtractSnapshot::new();
        snap.meta.game_version = "4.7.0".into();
        snap.meta.build_id = "test-build".into();
        snap.meta.extracted_at = "2026-04-15T00:00:00Z".into();
        snap.files
            .insert("data\\game2.dcb".into(), b"fake dcb bytes".to_vec());
        snap.files.insert(
            "data\\localization\\english\\global.ini".into(),
            b"fake locale bytes".to_vec(),
        );
        snap.files
            .insert("data\\meta\\empty.bin".into(), Vec::new());

        let path = tmp_snapshot_path("round_trip.snap");
        snap.save(&path).expect("save");

        let loaded = ExtractSnapshot::load(&path).expect("load");
        assert_eq!(loaded.meta.schema_version, ExtractSnapshot::SCHEMA_VERSION);
        assert_eq!(loaded.meta.game_version, "4.7.0");
        assert_eq!(loaded.meta.build_id, "test-build");
        assert_eq!(loaded.meta.extracted_at, "2026-04-15T00:00:00Z");
        assert_eq!(loaded.files, snap.files);
    }

    #[test]
    fn empty_files_map_round_trips() {
        let snap = ExtractSnapshot::new();
        let path = tmp_snapshot_path("empty.snap");
        snap.save(&path).expect("save");

        let loaded = ExtractSnapshot::load(&path).expect("load");
        assert_eq!(loaded.meta.schema_version, ExtractSnapshot::SCHEMA_VERSION);
        assert!(loaded.files.is_empty());
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
    fn capture_requires_dcb() {
        // Memory-backed AssetSource with no files at all.
        let source = AssetSource::from_snapshot(BTreeMap::new(), "empty");
        let meta = SnapshotMeta::default();

        let result = ExtractSnapshot::capture(&source, meta, &SnapshotCaptureConfig::standard());
        assert!(
            matches!(result, Err(Error::DcbNotFound)),
            "expected DcbNotFound, got {result:?}"
        );
    }

    #[test]
    fn capture_captures_requested_files() {
        // Memory-backed source with a fake DCB and a fake locale file.
        // Note the backslash in the locale key: P4K archive paths are
        // stored with backslashes (CryEngine is Windows-native), and the
        // capture predicate in `ExtractSnapshot::capture` hard-matches
        // `ends_with("english\\global.ini")`. Using forward slashes here
        // would miss the match.
        let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        files.insert("data\\game2.dcb".into(), b"fake dcb".to_vec());
        files.insert(
            "data\\localization\\english\\global.ini".into(),
            b"fake locale".to_vec(),
        );
        let source = AssetSource::from_snapshot(files, "fixture");
        let meta = SnapshotMeta {
            schema_version: ExtractSnapshot::SCHEMA_VERSION,
            game_version: "test".into(),
            build_id: "test".into(),
            extracted_at: "now".into(),
        };

        let snap =
            ExtractSnapshot::capture(&source, meta, &SnapshotCaptureConfig::standard()).unwrap();
        assert_eq!(snap.files.len(), 2);
        assert_eq!(snap.files.get("data\\game2.dcb").unwrap(), b"fake dcb");
        assert_eq!(
            snap.files
                .get("data\\localization\\english\\global.ini")
                .unwrap(),
            b"fake locale"
        );
    }

    #[test]
    fn capture_datacore_only_skips_locale() {
        let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        files.insert("data\\game2.dcb".into(), b"fake dcb".to_vec());
        files.insert(
            "data\\localization\\english\\global.ini".into(),
            b"fake locale".to_vec(),
        );
        let source = AssetSource::from_snapshot(files, "fixture");
        let meta = SnapshotMeta::default();

        let snap = ExtractSnapshot::capture(&source, meta, &SnapshotCaptureConfig::datacore_only())
            .unwrap();
        assert_eq!(snap.files.len(), 1);
        assert!(snap.files.contains_key("data\\game2.dcb"));
        assert!(!snap
            .files
            .contains_key("data\\localization\\english\\global.ini"));
    }

    #[test]
    fn hydrate_rejects_missing_dcb() {
        // Build an ExtractSnapshot with an empty files map and assert
        // hydrate fails fast with SnapshotMissingDcb instead of letting
        // Datacore::parse surface a confusing downstream error.
        let snap = ExtractSnapshot::new();
        let result = snap.hydrate(&AssetConfig::standard(), &DatacoreConfig::standard());
        assert!(
            matches!(result, Err(Error::SnapshotMissingDcb)),
            "expected SnapshotMissingDcb, got {result:?}"
        );
    }
}
