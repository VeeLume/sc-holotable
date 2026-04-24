//! Error type for the `sc-extract` crate.
//!
//! One unified [`Error`] enum covers every failure path in sc-extract.
//! Variants are added as new modules land; the enum is `#[non_exhaustive]`
//! so additions are non-breaking.

use std::io;
use std::path::PathBuf;

/// Convenience [`std::result::Result`] alias with [`Error`] as the error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by sc-extract operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    // ── Localization ─────────────────────────────────────────────────────
    /// A `global.ini` file could not be read from disk.
    #[error("failed to read locale file at {path}")]
    LocaleIoError {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// A `global.ini` byte buffer could not be decoded as UTF-16 LE.
    /// This happens when the file is corrupted or not actually a Star
    /// Citizen localization file (e.g. the caller passed a `.p4k` by mistake).
    #[error("locale bytes are not valid UTF-16 LE")]
    LocaleDecodeFailed,

    /// A line in `global.ini` was malformed (no `=` separator).
    #[error("malformed locale line at {line_number}: {line}")]
    LocaleMalformedLine { line_number: usize, line: String },

    // ── Asset / P4K access ───────────────────────────────────────────────
    /// Failed to open a `Data.p4k` archive.
    #[error("failed to open P4K archive at {path}")]
    P4kOpen {
        path: PathBuf,
        #[source]
        source: svarog_p4k::Error,
    },

    /// A lookup or read against an already-open P4K failed.
    #[error("P4K operation failed")]
    P4k(#[from] svarog_p4k::Error),

    /// The caller asked for a file path that doesn't exist in the P4K.
    #[error("file not found in P4K: {0}")]
    FileNotInP4k(String),

    // ── DCB parse ────────────────────────────────────────────────────────
    /// `Game2.dcb` could not be located inside the P4K archive.
    #[error("Game2.dcb not found inside P4K archive")]
    DcbNotFound,

    /// `Game2.dcb` bytes failed to parse as a DataCore database.
    #[error("failed to parse Game2.dcb")]
    DcbParse(#[source] svarog_datacore::Error),

    // ── Snapshot save / load ─────────────────────────────────────────────
    /// Reading a snapshot file from disk failed.
    #[error("failed to read snapshot at {path}")]
    SnapshotRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Writing a snapshot file to disk failed.
    #[error("failed to write snapshot at {path}")]
    SnapshotWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Decoding snapshot bytes (msgpack) failed.
    #[error("failed to decode snapshot: {0}")]
    SnapshotDecode(String),

    /// Encoding a snapshot as msgpack failed.
    #[error("failed to encode snapshot: {0}")]
    SnapshotEncode(String),

    /// Zstd compression/decompression of a snapshot failed.
    #[error("snapshot compression error")]
    SnapshotCompression(#[source] io::Error),

    /// A loaded snapshot was written by an incompatible schema version.
    #[error("snapshot schema version mismatch: file has {found}, this build expects {expected}")]
    SnapshotVersionMismatch { expected: u32, found: u32 },

    /// A snapshot's captured-files map did not contain `game2.dcb` bytes,
    /// so [`crate::ExtractSnapshot::hydrate`] cannot rebuild the datacore.
    /// This happens when the snapshot was captured with
    /// `SnapshotCaptureConfig::archive_dcb = false` or was hand-constructed
    /// with the DCB omitted.
    #[error("snapshot does not contain game2.dcb bytes; cannot hydrate datacore")]
    SnapshotMissingDcb,
}
