//! Error types for the `sc-installs` crate.

use std::io;
use std::path::PathBuf;

/// Convenience [`std::result::Result`] alias with [`Error`] as the error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by discovery, manifest parsing, and related operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The RSI Launcher log file was not found at the expected path.
    #[error("RSI Launcher log not found at {0}")]
    LauncherLogNotFound(PathBuf),

    /// The launcher log file exists but could not be read.
    #[error("failed to read launcher log at {path}")]
    LauncherLogUnreadable {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// The launcher log exists but contains no install-related markers
    /// (no `Launcher::launch` events and no `Installer` events). Typically
    /// means the launcher log was rotated or wiped, since even installing
    /// the launcher itself emits no qualifying entries until a channel is
    /// touched.
    #[error("no install-related entries found in launcher log at {0}")]
    NoInstallEntries(PathBuf),

    /// Every entry in the launcher log points at an installation that either
    /// doesn't exist on disk or is missing `Data.p4k`.
    #[error("no valid installations found in launcher log")]
    NoValidInstallations,

    /// The `build_manifest.id` file could not be read.
    #[error("failed to read build manifest at {path}")]
    BuildManifestUnreadable {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// The `build_manifest.id` file could not be parsed as JSON.
    #[error("failed to parse build manifest at {path}")]
    BuildManifestMalformed {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    /// A path component did not parse as a known channel name.
    #[error("could not infer channel from path component: {0}")]
    UnknownChannel(String),

    /// Could not locate the RSI Launcher install directory (needed to
    /// extract the `launcher store.json` encryption key from the launcher's
    /// `app.asar`). Caller should fall back to log parsing.
    #[error("RSI Launcher install directory not found in standard locations")]
    LauncherInstallNotFound,

    /// The launcher's `app.asar` file is present but unreadable.
    #[error("failed to read launcher asar at {path}")]
    LauncherAsarUnreadable {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Streamed through `app.asar` but the encryption-key marker pattern
    /// did not match. Likely cause: the launcher refactored or rotated the
    /// embedded key. Caller should fall back to log parsing.
    #[error("encryption-key marker not found in launcher asar at {path}")]
    EncryptionKeyNotFound { path: PathBuf },

    /// The launcher's persistent store file was not found.
    #[error("RSI Launcher store not found at {0}")]
    LauncherStoreNotFound(PathBuf),

    /// The launcher's persistent store file exists but could not be read.
    #[error("failed to read launcher store at {path}")]
    LauncherStoreUnreadable {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// The launcher store decrypted to something other than the expected
    /// shape, or AES decryption failed (wrong key, truncated file, ...).
    #[error("launcher store could not be decrypted: {0}")]
    LauncherStoreMalformed(String),

    /// Decrypted launcher store is not valid JSON in the expected shape.
    #[error("launcher store JSON could not be parsed")]
    LauncherStoreInvalidJson {
        #[source]
        source: serde_json::Error,
    },
}
