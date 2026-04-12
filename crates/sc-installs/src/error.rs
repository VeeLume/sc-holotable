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

    /// The launcher log exists but has no `Launching Star Citizen` entries.
    /// Typically means the user has installed the launcher but never launched
    /// the game.
    #[error("no launch entries found in launcher log at {0}")]
    NoLaunchEntries(PathBuf),

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
}
