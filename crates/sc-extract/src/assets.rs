//! Generic non-DCB file access, backed by either a live P4K archive or
//! a hand-assembled byte map captured from a snapshot.
//!
//! [`AssetSource`] exposes read-only, on-demand access to arbitrary
//! files keyed by their in-archive path. It is deliberately *orthogonal*
//! to the runtime [`crate::Datacore`] / [`crate::AssetData`] types: those
//! hold cooked runtime state, while an `AssetSource` is the raw byte feed
//! that the parse pipeline consumes.
//!
//! # Backing stores
//!
//! An `AssetSource` is either **live** (wrapping a `svarog_p4k::P4kArchive`
//! mmap'd off disk) or **memory** (wrapping a `BTreeMap<String, Vec<u8>>`
//! captured from a snapshot file). The public API dispatches on the inner
//! variant transparently: `find_and_read`, `read`, and `try_read` all
//! behave identically from the caller's perspective.
//!
//! This is what lets [`crate::Datacore::parse`] and
//! [`crate::AssetData::extract`] consume either a live install or a
//! captured snapshot through the exact same code path — the substitution
//! is entirely internal to `AssetSource`.
//!
//! # Cost model
//!
//! Opening a **live** source is cheap — svarog-p4k memory-maps the archive
//! and parses only the central directory. Reads are eager: each `read`
//! call returns a fully-decompressed `Vec<u8>`. Holding the handle open
//! for many scattered reads is the intended usage pattern.
//!
//! Opening a **memory** source is a move of the pre-decompressed byte map.
//! Reads just clone the relevant `Vec<u8>` out of the map (which already
//! holds its contents in plain bytes). No decompression cost per read.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use svarog_p4k::{P4kArchive, P4kEntryRef};

use crate::error::{Error, Result};

/// Read-only byte feed, backed by either a live `.p4k` archive or a
/// snapshot's captured file map.
///
/// Construct with [`AssetSource::open`] / [`AssetSource::from_install`]
/// for the live case, or [`AssetSource::from_snapshot`] for a memory-
/// backed source rehydrated from an [`crate::ExtractSnapshot`].
pub struct AssetSource {
    inner: AssetSourceInner,
}

/// Internal backing store for an [`AssetSource`].
///
/// Kept private so we can rearrange the representation (e.g. add a
/// lazy-zstd variant) without touching callers.
enum AssetSourceInner {
    /// Live `.p4k` mmap handle.
    Live {
        archive: P4kArchive,
        source_path: PathBuf,
    },
    /// Pre-captured, fully-decompressed byte map keyed by in-archive path.
    /// Used by the snapshot-hydration path.
    Memory {
        files: BTreeMap<String, Vec<u8>>,
        /// Diagnostic label (no semantic meaning). Typically something
        /// like `"snapshot://game_version=4.7"`.
        label: String,
    },
}

impl AssetSource {
    /// Open a `Data.p4k` archive for asset access.
    ///
    /// Parses the central directory but does not read any file bytes —
    /// subsequent reads are on demand.
    pub fn open(p4k_path: &Path) -> Result<Self> {
        let archive = P4kArchive::open(p4k_path).map_err(|source| Error::P4kOpen {
            path: p4k_path.to_path_buf(),
            source,
        })?;
        Ok(Self {
            inner: AssetSourceInner::Live {
                archive,
                source_path: p4k_path.to_path_buf(),
            },
        })
    }

    /// Convenience: open the `Data.p4k` from a discovered installation.
    ///
    /// ```no_run
    /// let install = sc_installs::discover_primary()?;
    /// let assets = sc_extract::AssetSource::from_install(&install)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_install(install: &sc_installs::Installation) -> Result<Self> {
        Self::open(&install.data_p4k())
    }

    /// Rehydrate an asset source from a captured byte map.
    ///
    /// This is the load-side counterpart to
    /// [`crate::ExtractSnapshot::capture`]: consumers who loaded a snapshot
    /// feed its `files` map into this constructor to build an `AssetSource`
    /// that serves the same bytes the original live archive would have.
    ///
    /// The memory backing transparently supports all of the reading methods
    /// ([`read`](Self::read), [`try_read`](Self::try_read),
    /// [`find_and_read`](Self::find_and_read)), with the contract that the
    /// set of retrievable paths is exactly what the snapshot captured —
    /// anything not present in the map returns [`Error::FileNotInP4k`]
    /// / `Ok(None)`.
    ///
    /// The `label` is a diagnostic-only string (shown in `Debug` output
    /// and tracing). Pass something identifying, e.g. the snapshot's
    /// game version or file path.
    pub fn from_snapshot(files: BTreeMap<String, Vec<u8>>, label: impl Into<String>) -> Self {
        Self {
            inner: AssetSourceInner::Memory {
                files,
                label: label.into(),
            },
        }
    }

    /// Filesystem path of the underlying archive, or `None` for a
    /// snapshot-backed source (which has no on-disk P4K).
    pub fn source_path(&self) -> Option<&Path> {
        match &self.inner {
            AssetSourceInner::Live { source_path, .. } => Some(source_path),
            AssetSourceInner::Memory { .. } => None,
        }
    }

    /// Low-level access to the underlying `P4kArchive`, or `None` for a
    /// snapshot-backed source.
    ///
    /// Returns `None` for memory-backed sources because there is no real
    /// archive to drive iteration against. Callers that need to enumerate
    /// entries should use [`find_and_read`](Self::find_and_read) or
    /// [`try_read`](Self::try_read), which dispatch correctly over both
    /// backing stores.
    pub fn archive(&self) -> Option<&P4kArchive> {
        match &self.inner {
            AssetSourceInner::Live { archive, .. } => Some(archive),
            AssetSourceInner::Memory { .. } => None,
        }
    }

    /// Read a file from the archive by its full in-archive path.
    ///
    /// Matching is case-insensitive. Returns [`Error::FileNotInP4k`] if
    /// no entry matches in the live case, or if the path is not present
    /// in the captured byte map for a snapshot-backed source.
    pub fn read(&self, path: &str) -> Result<Vec<u8>> {
        match self.try_read(path)? {
            Some(bytes) => Ok(bytes),
            None => Err(Error::FileNotInP4k(path.to_string())),
        }
    }

    /// Same as [`read`](Self::read) but returns `Ok(None)` for a miss
    /// instead of [`Error::FileNotInP4k`]. Useful when presence is optional.
    pub fn try_read(&self, path: &str) -> Result<Option<Vec<u8>>> {
        match &self.inner {
            AssetSourceInner::Live { archive, .. } => {
                let Some(entry) = archive.find(path) else {
                    return Ok(None);
                };
                let bytes = archive.read(&entry)?;
                Ok(Some(bytes))
            }
            AssetSourceInner::Memory { files, .. } => {
                // Match `P4kArchive::find`'s case-insensitive semantics so
                // callers that hard-code a path like `"Data/Game2.dcb"` get
                // the same behavior against either backing store.
                let needle = path.to_ascii_lowercase();
                for (k, v) in files.iter() {
                    if k.to_ascii_lowercase() == needle {
                        return Ok(Some(v.clone()));
                    }
                }
                Ok(None)
            }
        }
    }

    /// Find the first entry matching a predicate and read its bytes.
    ///
    /// For a live source, the first returned `(path, bytes)` pair comes
    /// from the earliest entry in archive order whose name satisfies
    /// `predicate`. For a snapshot-backed source, iteration is in
    /// `BTreeMap` key order (sorted by in-archive path); this is
    /// deterministic but may differ from original archive order. In
    /// practice this only matters when multiple entries could match the
    /// same predicate (e.g. duplicated `game2.dcb`), which doesn't occur
    /// for the well-known paths we capture.
    ///
    /// Useful when you know a suffix or filename but not the full path —
    /// e.g. "find anything ending in `game2.dcb`".
    pub fn find_and_read<F>(&self, predicate: F) -> Result<Option<(String, Vec<u8>)>>
    where
        F: Fn(&str) -> bool,
    {
        match &self.inner {
            AssetSourceInner::Live { archive, .. } => {
                let Some(entry) = archive.iter().find(|e| predicate(e.name)) else {
                    return Ok(None);
                };
                let name = entry.name.to_string();
                let bytes = archive.read(&entry)?;
                Ok(Some((name, bytes)))
            }
            AssetSourceInner::Memory { files, .. } => {
                for (path, bytes) in files.iter() {
                    if predicate(path.as_str()) {
                        return Ok(Some((path.clone(), bytes.clone())));
                    }
                }
                Ok(None)
            }
        }
    }

    /// Iterate over entries whose path satisfies a predicate.
    ///
    /// Live-only: returns an empty iterator for a snapshot-backed source,
    /// because `P4kEntryRef` is borrowed from a live `P4kArchive`. For
    /// snapshot-backed sources, use [`find_and_read`](Self::find_and_read)
    /// or [`try_read`](Self::try_read) instead.
    pub fn find<'a, F>(&'a self, predicate: F) -> Box<dyn Iterator<Item = P4kEntryRef<'a>> + 'a>
    where
        F: Fn(&str) -> bool + 'a,
    {
        match &self.inner {
            AssetSourceInner::Live { archive, .. } => {
                Box::new(archive.iter().filter(move |e| predicate(e.name)))
            }
            AssetSourceInner::Memory { .. } => Box::new(std::iter::empty()),
        }
    }

    /// Read the bytes for an entry reference previously obtained from
    /// [`find`](Self::find).
    ///
    /// Only works on live sources (the only place `P4kEntryRef` can come
    /// from). Returns [`Error::FileNotInP4k`] with a diagnostic name if
    /// called against a memory-backed source — this is effectively
    /// unreachable unless a caller constructs a `P4kEntryRef` by hand.
    pub fn read_entry(&self, entry: &P4kEntryRef<'_>) -> Result<Vec<u8>> {
        match &self.inner {
            AssetSourceInner::Live { archive, .. } => Ok(archive.read(entry)?),
            AssetSourceInner::Memory { .. } => Err(Error::FileNotInP4k(
                "<read_entry on snapshot-backed source>".into(),
            )),
        }
    }

    // ── Typed helpers for well-known files ───────────────────────────────
    //
    // These are thin conveniences over `read` / `find_and_read`. They exist
    // for discoverability — a consumer that just wants bindings or a vehicle
    // XML shouldn't have to know the exact in-archive path. New helpers can
    // be added here as consumers need them.

    /// Read `defaultProfile.xml` (the canonical keybinding profile).
    ///
    /// Looks for the file by suffix match so layout shuffles inside the
    /// archive don't break callers.
    pub fn default_profile_xml(&self) -> Result<Vec<u8>> {
        match self
            .find_and_read(|name| name.to_ascii_lowercase().ends_with("defaultprofile.xml"))?
        {
            Some((_path, bytes)) => Ok(bytes),
            None => Err(Error::FileNotInP4k("defaultProfile.xml".into())),
        }
    }

    /// Read a vehicle XML by its in-archive relative path.
    ///
    /// This is currently a thin alias over [`read`](Self::read) — it exists
    /// as a named entry point so future callers can find it even before the
    /// hand-written vehicle XML parser lands.
    pub fn vehicle_xml(&self, relative_path: &str) -> Result<Vec<u8>> {
        self.read(relative_path)
    }
}

impl std::fmt::Debug for AssetSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            AssetSourceInner::Live {
                archive,
                source_path,
            } => f
                .debug_struct("AssetSource")
                .field("backing", &"live")
                .field("source_path", source_path)
                .field("entries", &archive.entry_count())
                .finish(),
            AssetSourceInner::Memory { files, label } => f
                .debug_struct("AssetSource")
                .field("backing", &"memory")
                .field("label", label)
                .field("entries", &files.len())
                .finish(),
        }
    }
}
