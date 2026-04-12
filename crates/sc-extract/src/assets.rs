//! Generic non-DCB file access inside a `Data.p4k` archive.
//!
//! [`AssetSource`] is a thin wrapper over [`svarog_p4k::P4kArchive`] that
//! exposes read-only, on-demand access to arbitrary files in the archive.
//! It is deliberately *orthogonal* to [`crate::ExtractedData`]: the envelope
//! holds pure DCB-derived state and is serialisable, while an `AssetSource`
//! holds a live mmap handle and is not.
//!
//! Consumers that only need files (e.g. `defaultBindings.xml`) can construct
//! an `AssetSource` directly without ever parsing the DCB. Consumers that
//! need both call [`crate::parse_from_p4k`] and receive an `ExtractedData`
//! plus an `AssetSource` they can keep alongside it.
//!
//! # Cost model
//!
//! Opening an `AssetSource` is cheap — svarog-p4k memory-maps the archive
//! and parses only the central directory. Reads are eager: each `read` call
//! returns a fully-decompressed `Vec<u8>`. Holding the handle open for many
//! scattered reads is the intended usage pattern.

use std::path::{Path, PathBuf};

use svarog_p4k::{P4kArchive, P4kEntryRef};

use crate::error::{Error, Result};

/// Live handle to a `Data.p4k` archive for generic file access.
///
/// Not serialisable — only `ExtractedData` is. Construct with
/// [`AssetSource::open`] or receive one from [`crate::parse_from_p4k`].
pub struct AssetSource {
    archive: P4kArchive,
    source_path: PathBuf,
}

impl AssetSource {
    /// Open a `Data.p4k` archive for asset access.
    ///
    /// This parses the central directory but does not read any file bytes.
    /// Subsequent reads are on demand.
    pub fn open(p4k_path: &Path) -> Result<Self> {
        let archive = P4kArchive::open(p4k_path).map_err(|source| Error::P4kOpen {
            path: p4k_path.to_path_buf(),
            source,
        })?;
        Ok(Self {
            archive,
            source_path: p4k_path.to_path_buf(),
        })
    }

    /// Filesystem path of the underlying archive. Useful for error messages
    /// and diagnostics; consumers should not need to re-open it.
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    /// Low-level access to the underlying `P4kArchive` for callers that need
    /// to drive iteration or reads themselves.
    pub fn archive(&self) -> &P4kArchive {
        &self.archive
    }

    /// Read a file from the archive by its full in-archive path.
    ///
    /// Matching is case-insensitive and delegates to `P4kArchive::find`.
    /// Returns [`Error::FileNotInP4k`] if no entry matches.
    pub fn read(&self, path: &str) -> Result<Vec<u8>> {
        match self.try_read(path)? {
            Some(bytes) => Ok(bytes),
            None => Err(Error::FileNotInP4k(path.to_string())),
        }
    }

    /// Same as [`read`](Self::read) but returns `Ok(None)` for a miss
    /// instead of [`Error::FileNotInP4k`]. Useful when presence is optional.
    pub fn try_read(&self, path: &str) -> Result<Option<Vec<u8>>> {
        let Some(entry) = self.archive.find(path) else {
            return Ok(None);
        };
        let bytes = self.archive.read(&entry)?;
        Ok(Some(bytes))
    }

    /// Find the first entry matching a predicate and read its bytes. The
    /// first returned `(path, bytes)` pair is from the earliest entry in
    /// archive order whose name satisfies `predicate`.
    ///
    /// Useful when you know a suffix or filename but not the full path —
    /// e.g. "find anything ending in `game2.dcb`".
    pub fn find_and_read<F>(&self, predicate: F) -> Result<Option<(String, Vec<u8>)>>
    where
        F: Fn(&str) -> bool,
    {
        let Some(entry) = self.archive.iter().find(|e| predicate(e.name)) else {
            return Ok(None);
        };
        let name = entry.name.to_string();
        let bytes = self.archive.read(&entry)?;
        Ok(Some((name, bytes)))
    }

    /// Iterate over entries whose path satisfies a predicate.
    ///
    /// Returned references borrow from the archive and are cheap to hold;
    /// read their bytes with [`read_entry`](Self::read_entry).
    pub fn find<'a, F>(&'a self, predicate: F) -> impl Iterator<Item = P4kEntryRef<'a>> + 'a
    where
        F: Fn(&str) -> bool + 'a,
    {
        self.archive.iter().filter(move |e| predicate(e.name))
    }

    /// Read the bytes for an entry reference previously obtained from
    /// [`find`](Self::find) or [`archive`](Self::archive).
    pub fn read_entry(&self, entry: &P4kEntryRef<'_>) -> Result<Vec<u8>> {
        Ok(self.archive.read(entry)?)
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
        match self.find_and_read(|name| {
            name.to_ascii_lowercase().ends_with("defaultprofile.xml")
        })? {
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
        f.debug_struct("AssetSource")
            .field("source_path", &self.source_path)
            .field("entries", &self.archive.entry_count())
            .finish()
    }
}
