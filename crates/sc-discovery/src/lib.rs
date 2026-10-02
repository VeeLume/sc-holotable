//! Discover Star Citizen installations and resolve their game-file paths.
//!
//! This crate's only job is to locate installed Star Citizen channels
//! (LIVE, Hotfix, PTU, EPTU, Tech Preview) on disk, read their
//! `build_manifest.id` to determine version / branch / build, and hand
//! consumers back enough information to reach `Data.p4k`, `global.ini`,
//! `user.cfg`, and friends.
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
//! `sc-discovery` alone and pay nothing for the extraction machinery.
//!
//! Also left to consumers, on purpose: selection state (cycling, "currently
//! selected install" — that is UI state), filesystem writes
//! ([`Installation::localization_override`] returns a path, the caller
//! writes), user-facing error wording (errors are structured), watching
//! running game processes, and loading a user-pinned install path — feed that
//! to [`Installation::from_root`].
//!
//! # Picking one install
//!
//! | Function | Source | Picks |
//! |---|---|---|
//! | [`discover_default`] | store `library.defaults[]`, else falls back to `discover_primary` | what the launcher's big "Launch" button starts — the right default for most UIs |
//! | [`discover_last_launched`] | launcher **log only**, strict | the channel most recently launched; the store has no "last launched" stamp |
//! | [`discover_primary`] | store, then log | highest [`Channel`] priority (LIVE first) — only when LIVE-bias is really what you want |
//!
//! [`discover`] returns every valid install. "Valid" is strict: both the root
//! directory and `Data.p4k` must exist. Individual broken installs are logged
//! and skipped; only "nothing to discover at all" is an `Err`.
//!
//! # Version strings
//!
//! Two formats exist because they come from *different* manifest fields, and
//! neither is a transformation of the other:
//!
//! - [`Installation::short_version`] — `"4.6"`, from `Data.Version`.
//! - the launcher-style label — `"4.7.2-live.11715810"`.
//!   [`Installation::launcher_version_label`] is the store's authoritative
//!   value (`None` on the log-fallback path).
//!   [`Installation::launcher_version_string_derived`] rebuilds one from the
//!   manifest's `Branch` + channel + changelist and **goes stale once a hotfix
//!   ships on an X.Y.0 branch** (it says `4.7.0-…` for a 4.7.2 build, because
//!   `Branch` does not roll forward). There is deliberately no auto-fallback:
//!   opt in with `label.clone().or_else(|| install.launcher_version_string_derived())`.
//!   `sc-generator` refuses to fall back at all, so a wrong `datacore/*` tag
//!   cannot be published.
//!
//! The raw manifest fields stay reachable through [`Installation::manifest`].
//!
//! # Quick start
//!
//! ```no_run
//! use sc_discovery::{discover, Language};
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
//! # Ok::<(), sc_discovery::Error>(())
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
    discover, discover_default, discover_from, discover_last_launched, discover_last_launched_from,
    discover_primary, discover_primary_from,
};
pub use error::{Error, Result};
pub use installation::Installation;
pub use language::Language;
pub use launcher_store::{
    LauncherIdentity, StoreInstall, StoreSnapshot, launcher_store_path, read_identity,
    read_identity_from, read_launcher_snapshot, read_launcher_snapshot_from, read_launcher_store,
    read_launcher_store_from,
};
pub use log_parser::{
    LogEntry, LogEntryKind, detect_channel_from_process, launcher_log_path,
    parse_launcher_log_entries,
};
pub use manifest::{BuildManifest, read_build_manifest};
