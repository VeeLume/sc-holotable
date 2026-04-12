# `sc-installs` — implementation notes

This document records the actual implementation of `sc-installs` — what was built, how it maps to the spec at `docs/sc-installs.md`, and what's deliberately different from the two prior implementations (`streamdeck-starcitizen` and `sc-langpatch`).

## Status

**Complete.** First implementation of the crate. All spec items covered, full unit-test suite passes with `tempfile` fixtures.

## Port source

The implementation is primarily ported from **streamdeck-starcitizen's `src/discovery.rs`** (494 lines, the most complete prior implementation). Two specific additions come from **sc-langpatch's `src-tauri/src/discovery.rs`**:

- **Strict validation** — require `Data.p4k` to exist, not just the install directory.
- **Localization override path** — the `<install>/data/Localization/<lang>/global.ini` write target exposed as `Installation::localization_override(Language)`.

The **launcher-visible version format** (`4.7.1-live.11592622`) is also adopted from sc-langpatch but exposed as a method on `Installation` (`launcher_version_string()`) rather than a free function.

## File layout

```
crates/sc-installs/
├── Cargo.toml
└── src/
    ├── lib.rs            Crate root: crate-level docs, module declarations, public re-exports
    ├── error.rs          Error enum (thiserror, #[non_exhaustive]) + Result alias
    ├── channel.rs        Channel enum + priority + from_str_loose + display/lowercase names
    ├── language.rs       Language enum (English only for now) + folder_name
    ├── manifest.rs       BuildManifest struct + read_build_manifest (handles v2 + legacy)
    ├── installation.rs   Installation struct with path helpers + version helpers + is_valid
    ├── log_parser.rs     launcher_log_path + parse_launcher_log_entries + detect_channel_from_process
    └── discovery.rs      discover / discover_primary / discover_last_launched + _from variants
```

## Dependencies

Added to the workspace `[workspace.dependencies]` for reuse by future crates:

- `thiserror = "2"`
- `serde = { version = "1", features = ["derive"] }`
- `serde_json = "1"`

`tracing` was already in workspace deps from an earlier change.

`sc-installs` itself uses:

- `thiserror` — the error enum
- `serde` + `serde_json` — for `BuildManifest` parsing (v2 + legacy)
- `tracing` — `debug!` logging when filtering out invalid installs
- `specta = "2"` — optional, behind the `specta` feature flag
- `tempfile = "3"` — dev-dep, used by the test suites in every module

The crate has **zero svarog exposure**. `streamdeck-starcitizen` will be able to depend on `sc-installs` alone without pulling in any DCB-parsing machinery.

## Module responsibilities

### `error.rs`

Single `Error` enum with eight variants, `#[derive(thiserror::Error)]` and `#[non_exhaustive]` for forward compatibility:

- `LauncherLogNotFound(PathBuf)` — no log at the expected path
- `LauncherLogUnreadable { path, source }` — file exists but read failed
- `NoLaunchEntries(PathBuf)` — log exists but has zero `Launching Star Citizen` entries
- `NoValidInstallations` — log had entries but every one was filtered out by validation
- `BuildManifestUnreadable { path, source }` — can't read `build_manifest.id`
- `BuildManifestMalformed { path, source }` — file is not valid JSON or doesn't match the expected schema
- `UnknownChannel(String)` — a path component or launcher-log string isn't a recognized channel name

Every variant gives enough structured data for a consumer to format its own UX message. No user-facing strings baked in.

### `channel.rs`

`Channel` enum with five variants: `Live`, `Hotfix`, `Ptu`, `Eptu`, `TechPreview`. Implements:

- `ALL` constant for iteration
- `priority()` — lower is higher priority, LIVE = 0
- `display_name()` — uppercase
- `lowercase_name()` — for launcher version strings
- `from_str_loose()` — case-insensitive, accepts `TECH-PREVIEW`/`TECHPREVIEW`/`TECH`
- `from_path_component()` — crate-internal; wraps `from_str_loose` with an error for unknowns
- `Display` trait forwards to `display_name()`
- Serde derives unconditional; `specta::Type` derive under the feature flag

### `language.rs`

`Language` enum, **English only**, marked `#[non_exhaustive]`. The spec discussed other languages, but Star Citizen uses regional folder variants (`chinese_(simplified)`, `spanish_(latin_american)`) that aren't safe to guess — each language must be added after verification against a real SC install.

Doc comment on the enum explicitly calls this out. Extending is additive and non-breaking.

### `manifest.rs`

`BuildManifest` public struct with four fields: `version`, `branch`, `build_id`, `changelist: Option<String>`.

Private `RawManifest` + `RawManifestData` types for serde deserialization. Handles both formats via a `from_raw` helper:

- **v2 format** (`{"Data": {...}}`): preferred when `data.version` is non-empty. Maps directly.
- **Legacy format** (flat `{"RequestedP4kFileName": ..., "Branch": ..., "BuildId": ...}`): falls back when v2 data is missing. Derives `version` from the `Data_<version>.p4k` filename.

`read_build_manifest(install_root)` reads the file at `<install_root>/build_manifest.id` and returns the normalized struct. Errors on read failure and parse failure are distinct variants so callers can distinguish "file missing" from "file corrupted".

Uses a let-chain (`if let Some(data) = raw.data.as_ref() && !data.version.is_empty()`). Let-chains are stable since 1.88 and the workspace is on 1.94.1.

### `installation.rs`

`Installation` struct with three public fields: `channel`, `root`, `manifest`.

- **Construction**: `from_root(root)` (infers channel from final path component) and `from_parts(channel, root)` (explicit channel).
- **Paths**: `data_p4k`, `user_cfg`, `build_manifest_path`, `localization_dir(lang)`, `localization_override(lang)`.
- **Version helpers**: `short_version()` takes the substring up to the second dot in `manifest.version`; `launcher_version_string()` builds `{branch-stripped}-{channel-lowercase}.{changelist}` and returns `None` if changelist is missing.
- **Validation**: `is_valid()` — directory exists AND `Data.p4k` exists. Same check the discovery functions use internally.

No `version: String` field on `Installation` itself — the spec explicitly called for only method-based version access to prevent consumers from grabbing the raw manifest version unintentionally.

### `log_parser.rs`

Three public functions:

- `launcher_log_path()` — returns `%APPDATA%/rsilauncher/logs/log.log`, handles missing APPDATA gracefully
- `parse_launcher_log_entries(log_path)` — returns raw `Vec<(Channel, PathBuf)>` from the log, no filtering or validation; `Err` only on I/O / "file missing" situations
- `detect_channel_from_process(process_name)` — pure helper for consumers (streamdeck-starcitizen) that observe running processes

The launcher-log parser handles **both** formats (legacy plain-text and JSON v2.x) with a **single substring-based extractor**. Rather than two separate parsers, the function locates `"Launching Star Citizen "` anywhere in the line, then reads up to `" from ("` for the channel and `")"` for the path. The only format-specific detail is unescaping `\\` → `\` for the JSON form, handled uniformly.

This is simpler than sc-langpatch's regex-based parser and handles mixed-format logs transparently (which can happen across launcher upgrades — test case included).

### `discovery.rs`

Six public functions — three high-level discovery entry points plus their `_from` siblings:

- `discover()` / `discover_from(log_path)` — all valid installs sorted by priority
- `discover_primary()` / `discover_primary_from(log_path)` — highest-priority valid install (LIVE first)
- `discover_last_launched()` / `discover_last_launched_from(log_path)` — most recently launched *valid* install, walking the log in reverse chronological order

Internal `validate_install(channel, root) -> Option<Installation>` is the single canonical definition of "is this install valid?". It checks directory existence, `Data.p4k` presence, and successful manifest read, logging `debug!` messages on each skip. All three discovery functions go through this helper.

**Deduplication in `discover_from`**: the launcher log typically contains many historical launches for the same channel. `discover_from` collects into a `HashMap<Channel, PathBuf>` so the most recent entry per channel wins. `discover_last_launched_from` does **not** dedupe — it walks the log in reverse and returns the first valid install regardless of channel, which is what "most recent" semantically means.

## Key design choices (matching the spec)

- **Strict validation** (directory + `Data.p4k` must both exist). Adopted from sc-langpatch. Broken installs are filtered silently with `debug!` logging.
- **Error on empty log** — `NoLaunchEntries` distinguishes "log had no launches" from "all entries were invalid" (`NoValidInstallations`). Callers can message each differently.
- **Two version formats** — `short_version()` for UI display, `launcher_version_string()` for launcher-style display. Neither uses the other as a source; they read different manifest fields.
- **English only in `Language`** — extensible when a real consumer needs more.
- **`_from` variant for every discovery function** — every discovery function has a `_from(&Path)` sibling for testability. Tests never touch the real launcher log.
- **Serde always on, specta behind a feature flag** — per workspace conventions.
- **`thiserror`-based errors** — structured variants, no user-facing strings baked in.

## Test suite

Unit tests live inline in each module via `#[cfg(test)] mod tests`. All tests use `tempfile` for isolated filesystem fixtures — no real SC install or launcher log required.

| Module | Test count | Coverage |
|---|---|---|
| `channel.rs` | 8 | Case-insensitive parsing, TECH variants, unknown-channel error, priority ordering, display/lowercase names, Display trait, `ALL` round-trip, `from_path_component` error |
| `language.rs` | 2 | English folder name, all folder names lowercase |
| `manifest.rs` | 6 | v2 full format, v2 without changelist, legacy format, file read from disk, missing-file error, malformed-JSON error |
| `installation.rs` | 13 | Short version (full, already-short, single component, empty), launcher version string (derived from branch not version, EPTU lowercase, no-changelist None, no-prefix branch), path helpers (`data_p4k`, `localization_override`), `from_root` channel inference, `from_root` unknown-channel error, `is_valid` requires Data.p4k |
| `log_parser.rs` | 9 | Legacy format, JSON v2 format, mixed formats in one log, empty log returns empty vec, missing log error, process-name detection (StarCitizen, PTU, EPTU, Hotfix, TechPreview, non-SC) |
| `discovery.rs` | 14 | `discover_from` with LIVE+PTU fixtures (sort by priority, not log order), filter missing `Data.p4k`, filter missing directory, dedupe by channel (most recent wins), missing log error, empty log error, all-invalid returns empty vec. `discover_primary_from` returns LIVE first, all-invalid errors. `discover_last_launched_from` picks most recent valid, skips invalid most recent, all-invalid errors, missing log error, empty log error. |

**Total: 52 unit tests.**

Notable fixtures:

- `fake_install(parent, channel_name, version, with_data_p4k)` — creates a directory with a valid `build_manifest.id` and optionally a fake `Data.p4k`
- `fake_log(entries)` — writes a plain-text launcher log file with the given entries

## What's deliberately not included

- **Tauri / Specta wiring beyond the feature flag.** The crate exposes `#[cfg_attr(feature = "specta", derive(specta::Type))]` on the public types, but doesn't generate TypeScript bindings itself. Consumers do that at their own boundary.
- **Stateful selection / cycling** (streamdeck-starcitizen's `ActiveInstallationState`). That's Stream Deck plugin UI state, not discovery state. Stays in the consuming app.
- **Actively observing running processes.** `detect_channel_from_process` is a pure string helper — actually watching process lists is the consumer's responsibility.
- **Non-English languages.** Added when a real consumer needs them, after folder-name verification.
- **User-facing error messages.** Errors are structured `thiserror` variants; message formatting for UIs is each consumer's UX layer.

## Switch-over plan

This crate's first real use will come from its three consumers. Validation plan:

1. **streamdeck-starcitizen** switches over first. Its `src/discovery.rs` becomes a thin wrapper that calls `sc_installs::discover` and feeds the result into the existing `ActiveInstallationState`. The existing Stream Deck plugin test suite should pass unchanged. This is the strongest validation because streamdeck is the source of the reference implementation.
2. **sc-langpatch** follows. Its `src-tauri/src/discovery.rs` becomes an import of `sc_installs::Installation` with Specta enabled via the feature flag.
3. **bulkhead** is the first greenfield consumer — no port required, just a direct use of `sc_installs::discover_last_launched`.

## Validation

The implementation was verified end-to-end against the local Rust toolchain:

- `cargo check -p sc-installs` — clean
- `cargo check -p sc-installs --features specta` — clean (locks `specta = "=2.0.0-rc.21"` to match sc-langpatch)
- `cargo test -p sc-installs` — **51 unit tests + 1 doc test pass**
- `cargo test -p sc-installs --features specta` — **51 unit tests + 1 doc test pass**
- `cargo clippy -p sc-installs --all-targets -- -D warnings` — no warnings

Dependency additions to the workspace `[workspace.dependencies]` section: `thiserror = "2"`, `serde` with derive, `serde_json = "1"`.

## Specta version pin

The `specta` dep is pinned with `=2.0.0-rc.21`, not the default `^` operator. Two reasons:

1. Cargo's semver rules for prereleases allow `^2.0.0-rc.21` to match `rc.24`, which is effectively a different version.
2. `rc.24` specifically requires nightly `debug_closure_helpers`, so it won't build on stable. `rc.21` is stable-compatible and matches what `sc-langpatch` uses.

When sc-langpatch eventually upgrades past rc.21, this pin should be bumped in lockstep.

## Known follow-ups

- **Language enum extension.** When sc-langpatch adds support for non-English locales, each language's folder name must be verified against a real SC install before being added here.
- **Process-name detection heuristic.** `detect_channel_from_process` uses simple substring matching. If CIG ever adds a process variant that collides (e.g., `StarCitizen_HotPTU.exe`), the match order will need refinement. Not urgent.
- **`manifest.rs` now uses let-chains.** Originally written as nested `if let` to honor `rust-version = "1.85"`. Converted to `if let Some(data) = raw.data.as_ref() && !data.version.is_empty()` after the workspace was bumped to 1.94.1.
