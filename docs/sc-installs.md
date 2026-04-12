# `sc-installs` — design specification

> Status: **decisions locked, ready for implementation planning.** Nothing in this document is implemented yet.

## Purpose

`sc-installs` is the shared Star Citizen install discovery crate for the holotable workspace. It locates installed SC channels by parsing the RSI Launcher log, reads their `build_manifest.id` for version info, and hands consumers back typed `Installation` values with path helpers for the common game files.

It is the only crate all three consumers depend on. It has **zero domain dependencies** — no svarog, no `sc-extract`, no DCB awareness.

## Consumers and what they need

| App | What it needs |
|---|---|
| **streamdeck-starcitizen** | All discovered installs + channel cycling (app owns the cycling state) + a short version string for UI display + path to `Data.p4k` (for keybinds extraction) |
| **sc-langpatch** | All discovered installs + validated `Data.p4k` path + **writable** path to `<install>/data/Localization/<lang>/global.ini` + launcher-visible version string for UI |
| **bulkhead** | The primary LIVE install (usually) + path to `Data.p4k` + version info for its damage-model cache key |

## Prior-art audit

The crate consolidates two existing implementations and unblocks a third:

- **streamdeck-starcitizen/src/discovery.rs** (494 lines) — the reference. Typed `Channel` enum, full `build_manifest.id` v2 + legacy parsing, coverage for both launcher log formats (legacy plain-text + JSON-per-line v2.x). **Adopted as the baseline.**
- **sc-langpatch/src-tauri/src/discovery.rs** (133 lines) — simpler regex-based parser. Contributes three things the baseline lacks: (1) strict validation (checks `Data.p4k` exists, not just the directory), (2) the localization-override path helper (`output_dir`), (3) the launcher-visible version format `"4.7.1-live.11592622"`.
- **bulkhead** currently hardcodes `C:\Games\StarCitizen\LIVE\Data.p4k` at `crates/explore/src/main.rs:9`. Nothing to port; it becomes the first greenfield consumer.

### Surprises worth naming

**Two version formats, derived from different manifest fields.**

| Consumer | Format example | Source fields |
|---|---|---|
| streamdeck (`short_version`) | `"4.6"` | First two components of `Data.Version` |
| streamdeck (`version`) | `"4.6.173.39432"` | Raw `Data.Version` |
| sc-langpatch (`game_version`) | `"4.7.1-live.11592622"` | `Data.Branch` (stripped of `sc-alpha-` prefix) + channel + `Data.RequestedP4ChangeNum` |

They are **not** transformations of each other — they read different fields of `build_manifest.id`. Both formats are meaningful in their respective UIs. The lib must expose both without forcing a choice.

**Validation divergence.** streamdeck only checks that the directory exists; sc-langpatch also checks that `Data.p4k` exists inside it. Strict wins — a directory without `Data.p4k` is useless to every consumer.

**ActiveInstallationState is UI state, not discovery state.** streamdeck's `src/state/installations.rs` wraps discovery results in an `ArcSwap` with selected-index, next/previous cycling, and "last launched" tracking. That is Stream Deck plugin behavior, not install-discovery behavior, and it does not belong in `sc-installs`. Each consumer manages its own selection state.

## Proposed API

### Types

```rust
/// Star Citizen release channel.
pub enum Channel {
    Live,
    Hotfix,
    Ptu,
    Eptu,
    TechPreview,
}

impl Channel {
    /// Lower value = higher priority. LIVE is 0, TechPreview is 4.
    pub fn priority(self) -> u8;
    pub fn display_name(self) -> &'static str;       // "LIVE", "PTU", ...
    pub fn lowercase_name(self) -> &'static str;     // "live", "ptu", ...  (for launcher version strings)
    pub fn from_str_loose(s: &str) -> Option<Self>;
    pub const ALL: &'static [Channel] = &[
        Channel::Live, Channel::Hotfix, Channel::Ptu, Channel::Eptu, Channel::TechPreview,
    ];
}

/// Supported Star Citizen localization languages.
pub enum Language {
    English,
    German,
    French,
    Spanish,
    Italian,
    // ...expand as needed
}

impl Language {
    /// The directory name SC uses inside `data/Localization/`.
    pub fn folder_name(self) -> &'static str;        // "english", "german", ...
}

/// A discovered, validated Star Citizen installation.
pub struct Installation {
    pub channel: Channel,
    pub root: PathBuf,
    pub manifest: BuildManifest,
}

/// Parsed fields from `build_manifest.id`.
///
/// Supports both the current v2 format (`{ "Data": { ... } }`) and the
/// legacy flat format (`{ "RequestedP4kFileName": ..., "Branch": ..., ... }`).
pub struct BuildManifest {
    /// Full version as reported by the manifest, e.g. "4.6.173.39432".
    pub version: String,
    /// Branch name from the manifest, e.g. "sc-alpha-4.6.1".
    pub branch: String,
    /// Build ID from the manifest, e.g. "12345".
    pub build_id: String,
    /// Changelist number from `Data.RequestedP4ChangeNum`, if present.
    /// Needed for the launcher-visible version string.
    pub changelist: Option<String>,
}

/// Errors returned by discovery and manifest reading.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RSI Launcher log not found at {0}")]
    LauncherLogNotFound(PathBuf),

    #[error("failed to read launcher log at {path}")]
    LauncherLogUnreadable { path: PathBuf, source: io::Error },

    #[error("no launch entries found in launcher log at {0}")]
    NoLaunchEntries(PathBuf),

    #[error("failed to read build manifest at {path}")]
    BuildManifestUnreadable { path: PathBuf, source: io::Error },

    #[error("failed to parse build manifest at {path}")]
    BuildManifestMalformed { path: PathBuf, source: serde_json::Error },

    #[error("could not infer channel from path component: {0}")]
    UnknownChannel(String),

    #[error("no valid installations found in launcher log")]
    NoValidInstallations,
}
```

### Free functions

```rust
/// Discover every reachable SC installation by parsing the RSI Launcher log.
///
/// Returns installations sorted by channel priority (LIVE first). Installations
/// whose directory is missing or whose `Data.p4k` is missing are filtered out
/// (logged via tracing, not returned as errors).
pub fn discover() -> Result<Vec<Installation>, Error>;

/// Same as `discover()` but reads the launcher log from the given path.
/// Useful for tests and for users whose launcher log lives elsewhere.
pub fn discover_from(launcher_log: &Path) -> Result<Vec<Installation>, Error>;

/// Convenience: the highest-priority valid installation (LIVE first).
///
/// Returns `Err(NoValidInstallations)` if every discovered entry is filtered
/// out by validation. For "the install the user actually plays" semantics,
/// prefer `discover_last_launched()`.
pub fn discover_primary() -> Result<Installation, Error>;

/// Discover the most recently launched install that is still valid.
///
/// Walks backwards through the launcher log and returns the first entry whose
/// install still exists and has a readable `Data.p4k`. This is usually the
/// best "default selection" — `discover_primary()` always returns LIVE first,
/// but if the user primarily plays PTU that is the wrong default.
///
/// Returns `Err(NoValidInstallations)` if no entry in the log corresponds to
/// a still-valid install.
pub fn discover_last_launched() -> Result<Installation, Error>;

/// Same as `discover_last_launched()` but reads the log from a given path.
pub fn discover_last_launched_from(launcher_log: &Path) -> Result<Installation, Error>;

/// Default location of the RSI Launcher log on Windows:
/// `%APPDATA%/rsilauncher/logs/log.log`.
pub fn launcher_log_path() -> PathBuf;
```

### `Installation` methods

```rust
impl Installation {
    // ── Construction ────────────────────────────────────────────────────

    /// Build an installation from an arbitrary path. Reads `build_manifest.id`
    /// and infers the channel from the final path component (LIVE / PTU / ...).
    pub fn from_root(root: impl Into<PathBuf>) -> Result<Self, Error>;

    /// Build an installation from an explicit channel + root. Reads the
    /// manifest but does not try to infer the channel from the directory name.
    pub fn from_parts(channel: Channel, root: impl Into<PathBuf>) -> Result<Self, Error>;

    // ── Paths ───────────────────────────────────────────────────────────

    pub fn data_p4k(&self) -> PathBuf;              // <root>/Data.p4k
    pub fn user_cfg(&self) -> PathBuf;              // <root>/user.cfg
    pub fn build_manifest_path(&self) -> PathBuf;   // <root>/build_manifest.id

    /// `<root>/data/Localization/<lang>/`
    pub fn localization_dir(&self, lang: Language) -> PathBuf;

    /// `<root>/data/Localization/<lang>/global.ini`
    ///
    /// This is the filesystem override path — the file SC reads *in preference
    /// to* the one shipped inside `Data.p4k`. sc-langpatch writes to it.
    pub fn localization_override(&self, lang: Language) -> PathBuf;

    // ── Version helpers ────────────────────────────────────────────────
    //
    // For raw manifest fields access `self.manifest` directly — these helpers
    // only cover the two *derived* formats consumers actually display.

    /// Short version derived from the manifest's full version string,
    /// e.g. "4.6.173.39432" → "4.6".
    pub fn short_version(&self) -> &str;

    /// Launcher-visible version format, e.g. "4.7.1-live.11592622".
    /// Built from manifest `branch` (stripped of the `sc-alpha-` prefix) +
    /// channel (lowercase) + `changelist`. Returns `None` if the manifest
    /// lacks a changelist.
    pub fn launcher_version_string(&self) -> Option<String>;

    // ── Validation ─────────────────────────────────────────────────────

    /// True if the root exists AND `Data.p4k` is present.
    /// This is the same check `discover()` uses to filter invalid installs.
    pub fn is_valid(&self) -> bool;
}
```

### Low-level escape hatches

For consumers that want raw data or need to bypass filtering:

```rust
/// Parse every launch entry from the log without filtering or manifest reading.
pub fn parse_launcher_log_entries(log_path: &Path) -> Result<Vec<(Channel, PathBuf)>, Error>;

/// Read and parse a `build_manifest.id` file directly.
pub fn read_build_manifest(install_root: &Path) -> Result<BuildManifest, Error>;
```

## Design decisions

### Typed `Channel` enum

Adopted from streamdeck. sc-langpatch currently uses `String` because of Specta/Tauri serialization constraints, but that is an app concern — the lib provides `Channel` and sc-langpatch maps to `String` at its own boundary (or adds a Specta impl behind a feature flag; see open questions).

### `BuildManifest` as a struct

streamdeck currently returns `(String, String, String)` from `read_build_manifest`. A named struct is easier to use, has room to grow (the `changelist` field is new), and documents the parser's output.

### Two derived version formats, raw manifest stays accessible

Because the two consumers read different manifest fields to build "the version", both derived formats must be available without privileging one. `Installation` exposes `short_version()` and `launcher_version_string()` as convenience methods.

There is **no** direct `version: String` field on `Installation`. Consumers that want the raw manifest fields (`version`, `branch`, `build_id`, `changelist`) reach them through the public `manifest: BuildManifest` field. Methods exist only for *derived* formats that would otherwise each consumer would have to reimplement identically.

### Strict validation

`discover()` filters out installs where either the root directory or `Data.p4k` is missing. This matches sc-langpatch's stricter check rather than streamdeck's dir-only check. Consumers that want to see raw launcher log entries (including broken ones) can call `parse_launcher_log_entries()` directly.

### `Result<Vec<Installation>, Error>` not `Vec<Installation>`

streamdeck silently returns an empty vec and logs warnings; sc-langpatch `bail!`s with user-facing messages. The lib returns structured errors for "there is literally nothing to discover" cases (launcher log missing, no launch entries), and consumers format their own UX messages on top. Individual broken installs (missing dir, missing Data.p4k) are logged and skipped, not elevated to `Err`.

### App-name channel detection stays in streamdeck

streamdeck's `detect_channel_from_app()` infers a channel from a running process name (e.g. `StarCitizen_PTU.exe` → `Channel::Ptu`). That is a Stream Deck plugin concern — neither bulkhead nor sc-langpatch observe a running process. Leaving it in streamdeck until a second consumer needs it.

### `Language` as a typed enum

Star Citizen's supported languages are a closed set and the folder-name mapping (`English` → `"english"`) is exactly the kind of knowledge a shared lib should own. Consumers get type safety; the cost is one small enum.

### No `ActiveInstallationState` or selection state

Discovered installs come out as a `Vec<Installation>`. Cycling, selection, and last-launched tracking are consumer concerns and stay in each app.

### Error type: `thiserror`

Standard library convention. Consumers using `anyhow` can `?`-propagate transparently. This does mean introducing a `thiserror` dep, which is small and widely adopted.

### Serde always on, Specta behind a feature flag

`Installation`, `Channel`, `Language`, and `BuildManifest` derive `serde::Serialize` + `serde::Deserialize` **unconditionally**. sc-langpatch needs serde at its Tauri boundary, bulkhead may want it for caching, and a `serde` feature flag tends to be a trap: every downstream crate that touches these types has to either re-export the flag or pin it on. Just build it in.

`specta::Type` derivation lives behind a `specta` feature flag, **off by default**. sc-langpatch enables it to get typed Rust↔TS bindings across its Tauri boundary; the other two consumers skip it and pay no compile-time cost.

## What the lib explicitly does *not* provide

These are consumer concerns, listed here so the boundary is unambiguous:

- **Stateful selection / cycling.** Lives in each app.
- **Filesystem writes.** `localization_override()` returns a `PathBuf`; sc-langpatch owns the `std::fs::write` call. This preserves the rule that `sc-installs` has no dep on `sc-extract` and vice versa.
- **Tauri bridge glue.** The lib provides `serde` derives unconditionally and `specta::Type` behind a feature flag, but sc-langpatch still owns its Tauri command wiring, IPC boundary, and any UI-specific fields it wants on top of `Installation`.
- **User-facing error messages.** Errors are structured; message formatting is each consumer's UX.
- **Process / app-name monitoring.** Stays in streamdeck-starcitizen.
- **Loading user configuration overrides** (e.g. "the user pinned a custom install path in a config file"). Consumers can call `Installation::from_root()` with whatever path they load from config.

## Out of scope for this document

- The implementation plan itself — that goes in `implementing/sc-installs.md` once this spec is agreed.
- Non-Windows support. All three consumers are Windows-only today. The lib should compile on other platforms (no Windows-specific APIs in the public surface), but install discovery via `%APPDATA%` is inherently Windows-shaped.
- Integration with a future "user config" layer that lets the user pin custom install paths. `Installation::from_root()` is the entry point when that arrives.
