# Changelog

Versioning is tracked in two orthogonal axes, both monotonic and immutable:

- **`sc-holotable/vX.Y.Z`** — library API releases, tracked in this file.
  Pre-1.0 convention: `0.X.0` bumps for any public-surface change (additive
  *or* breaking); `0.x.Y` bumps for bugfixes and internal-only changes.
- **`datacore/<sc_version>`** — DataCore regeneration snapshots, cut by
  `tools/regenerate.ps1 -Publish` after a Star Citizen patch. Not tracked
  here.

Consumers pin against whichever axis they care about; tags point at
separate commits and advance independently.

## [Unreleased]

### Added

- **`sc-installs`: launcher-store discovery.** New module `launcher_store`
  reads `%APPDATA%/rsilauncher/launcher store.json` (electron-store /
  AES-256-CBC) for an authoritative list of installed channels — works
  even for channels never launched. Encryption key is extracted at
  runtime from the launcher's own `app.asar` (no embedded secret).
  - New `discover_default()` returns the channel from the launcher's
    `library.defaults[]` — better "default install" UX than
    `discover_primary` (hardcoded LIVE-first).
  - `discover()` and `discover_primary()` now go through the store
    first, fall back to log parsing on any failure.
  - New types: `StoreInstall`, `StoreSnapshot`. New free functions:
    `read_launcher_store`, `read_launcher_snapshot`, plus `_from`
    siblings.
  - New deps: `aes`, `cbc`, `pbkdf2`, `sha2` (pure-rust, no DPAPI).
  - See `docs/launcher-store.md` for the full launcher-store reference.

- **`sc-installs`: broader log parsing.** Recognises `[Installer]` and
  `Deleting <root>\loginData.json` markers in addition to
  `[Launcher::launch]`, so log-fallback discovery finds channels that
  have been installed but never launched. New `LogEntry` /
  `LogEntryKind` types; `parse_launcher_log_entries` returns
  `Vec<LogEntry>`.

- **`Channel::install_dir_name()`** — on-disk directory name for a
  channel (notably `TECH-PREVIEW`, distinct from the display name `TECH`).

### Changed (breaking)

- **`sc-installs`: `Installation::launcher_version_string()` removed.**
  Replaced with two explicit alternatives because the old auto-derivation
  was silently off-by-patch-number once any hotfix shipped on top of an
  X.Y.0 branch (the manifest's `Branch` field doesn't roll forward):
  - `Installation::launcher_version_label: Option<String>` (new public
    field) — authoritative store-provided label, e.g.
    `"4.7.2-live.11715810"`. Set by store-using discovery paths; `None`
    when discovery fell back to log parsing.
  - `Installation::launcher_version_string_derived()` (renamed) —
    locally derived from manifest fields; carries an explicit staleness
    caveat. Consumers that want fallback semantics must opt in
    explicitly.

  `sc-generator` now uses `launcher_version_label` exclusively and
  refuses to fall back to derivation, so a wrong tag like
  `datacore/4.7.0-live.X` for a 4.7.2 build can't slip through. The
  regen script gets this for free.

- **`sc-installs`: `Error::NoLaunchEntries` renamed** to
  `Error::NoInstallEntries` — the error now also covers logs that have
  no `Installer` markers, not just no launch markers.

- **`sc-installs`: `parse_launcher_log_entries` return type changed**
  from `Vec<(Channel, PathBuf)>` to `Vec<LogEntry>`.

- **`sc-installs`: legacy plain-text manifest parsing removed.** Only
  the v2 nested `{"Data": {...}}` shape is parsed now. The legacy
  flat-format support was code path debt — every shipped Star Citizen
  build the workspace targets uses the v2 shape.

## [v0.2.0] - 2026-05-02

### Changed (breaking)

- **`sc-contracts` v2 redesign — Mission-centric API.** Wholesale
  rename and reshape of the public surface; every consumer needs
  updates. Design doc: `docs/sc-contracts-v2.md`; consumer guide:
  `docs/sc-contracts-guide.md`.
  - `Contract` → `Mission`, `ContractIndex` → `MissionIndex`,
    `EncounterWave` → `EncounterPhase` (now generic
    `EncounterPhase<S>`).
  - `MissionOrigin` consolidates the previously-flat handler fields
    (`MissionContractHandler` / `RandomMissionParams` / variation
    metadata) into one enum.
  - `MissionRewards` collapses six previously-flat reward fields
    (UEC, REC, reputation, …) into one struct.
  - `EncounterSlot` carries symmetric `TagBag`s (positive / negative
    / markup / entity) instead of the v1 ad-hoc tag plumbing.
  - `Encounter` enum replaces v1's ship-only encounter type. Variants
    `Ships` / `Npcs` / `Entities` / `Unknown` widen NPC and entity
    coverage; NPC slots expose `mission_allied_marker` (48 slots on
    SC 4.7), `is_critical`, `faction_override`.
  - Implicit BP merge step removed; `Variation`, `title_siblings`,
    and `find_bp_conflicts` deleted. Consumers walking the
    inheritance chain manually should switch to `MissionIndex` /
    `MissionPools`.

### Added

- **`MissionPools`** precomputed on the index, with opt-in
  divergence helpers for consumers that need to reason about per-BP
  pool drift without re-walking the graph themselves.
- **`ShipRegistry::resolve_spawn`** — tag-tree subsumption + spawn-
  state filter. Recovered 80 of 335 previously-empty ship-encounter
  slots on SC 4.7 LIVE (24% drop). Three coupled bugs fixed: ancestor
  tag matching, `AI > Ship > SpawnFlags` / `AI > CargoManifest` state
  tags being misread as identity filters, and the dual-`Ship`-node
  intent gate over-firing. See `status.md` "Last worked on" for the
  data-driven derivation.
- **Narrow-consumer re-exports** on `sc-contracts` and `sc-weapons`
  (`EntityClassDefinition`, locale-key cluster API) so callers don't
  have to depend on `sc-extract` directly for common types.
- **`tools/sc-explorer`** — interactive TUI binary with three tabs
  (Pools / Contracts / Weapons). Per-crate `tui` modules
  (`sc-contracts/src/tui`, `sc-weapons/src/tui`) own their domain
  views behind a `tui` feature.
- **Investigation examples** (committed under
  `crates/sc-contracts/examples/`) — `contract_dump`, `ambush_dig`,
  `damage_dig`, `salvage_pool`, `spawn_dig`, `encounter_analytics`,
  `encounter_kinds`, `role_investigation`, `tier_investigation`.
  Canonical way to run a quick dig against a fresh DCB regen.

### Fixed

- `EncounterSlot` now forwards typed signals on ship-spawn entries
  (previously dropped during extraction).
- TUI encounter-detail view surfaces the forwarded typed signals.

## [v0.1.0] - 2026-04-25

### Added

- `sc_contracts::Contract` and `sc_contracts::Variation` now carry
  `title_key: Option<LocaleKey>` and `description_key: Option<LocaleKey>`
  alongside the resolved `title` / `description` strings. Consumers
  patching `global.ini` (sc-langpatch, translation-extraction tooling)
  no longer need to re-walk the contract inheritance chain to recover
  the raw INI key the displayed text was resolved from.
- `sc_contracts::ResolvedText` gained the same `title_key` /
  `description_key` fields; `resolve_contract_text` fills them during
  the existing inheritance walk at no extra cost.
- `sc_contracts::ExpandedContract` propagates the keys so
  pre-merge consumers see them too.

### Changed

- `ResolvedText` is still constructible via field-init but now requires
  four fields instead of two. Call sites that built it manually should
  use `..Default::default()` to stay forward-compatible.
