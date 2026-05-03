//! Discover Star Citizen installations and resolve their game-file paths.
//!
//! This crate's only job is to locate installed Star Citizen channels
//! (LIVE, Hotfix, PTU, EPTU, Tech Preview) on disk, read their
//! `build_manifest.id` to determine version / branch / build, and hand
//! consumers back enough information to reach `Data.p4k`, `global.ini`,
//! `user.cfg`, and friends.
//!
//! See `docs/sc-installs.md` in the workspace repository for the full
//! design specification and `implementing/sc-installs.md` for
//! implementation notes.
//!
//! # Scope
//!
//! - Reads `%APPDATA%/rsilauncher/launcher store.json` — the launcher's
//!   authoritative encrypted state — by extracting the AES key at runtime
//!   from the launcher's own `app.asar`. Gives a complete inventory of
//!   installed channels independent of any launcher activity.
//! - Falls back to parsing `%APPDATA%/rsilauncher/logs/log.log` if the
//!   store is unavailable. Recognises both `Launcher::launch` events and
//!   `Installer` events, so a channel that has been installed but never
//!   launched is still found.
//! - Reads `build_manifest.id` for version info.
//! - Exposes a [`Channel`] enum with priority ordering and an
//!   [`Installation`] struct with path helpers for the common game files
//!   — [`Installation::data_p4k`], [`Installation::user_cfg`], and the
//!   localization override path
//!   (`<install>/data/Localization/<lang>/global.ini`) that sc-langpatch
//!   writes its patched locale to.
//!
//! # Out of scope
//!
//! This crate does **not** open or parse game files. It has zero
//! dependency on svarog, `sc-extract`, or any domain crate. A consumer
//! that only needs to know "where is LIVE installed?" can depend on
//! `sc-installs` alone and pay nothing for the extraction machinery.
//!
//! # Quick start
//!
//! ```no_run
//! use sc_installs::{discover, Language};
//!
//! let installs = discover()?;
//! for install in &installs {
//!     println!(
//!         "{} v{} at {}",
//!         install.channel,
//!         install.short_version(),
//!         install.root.display()
//!     );
//!
//!     // Path helpers
//!     let p4k = install.data_p4k();
//!     let global_ini = install.localization_override(Language::English);
//!     let _ = (p4k, global_ini);
//! }
//! # Ok::<(), sc_installs::Error>(())
//! ```

mod channel;
mod discovery;
mod error;
mod installation;
mod language;
mod launcher_store;
mod log_parser;
mod manifest;

pub use channel::Channel;
pub use discovery::{
    discover, discover_default, discover_from, discover_last_launched,
    discover_last_launched_from, discover_primary, discover_primary_from,
};
pub use error::{Error, Result};
pub use installation::Installation;
pub use language::Language;
pub use launcher_store::{
    StoreInstall, StoreSnapshot, launcher_store_path, read_launcher_snapshot,
    read_launcher_snapshot_from, read_launcher_store, read_launcher_store_from,
};
pub use log_parser::{
    LogEntry, LogEntryKind, detect_channel_from_process, launcher_log_path,
    parse_launcher_log_entries,
};
pub use manifest::{BuildManifest, read_build_manifest};
