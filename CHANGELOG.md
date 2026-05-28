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

## [v0.6.0] - 2026-05-28

### Added

- **`sc-installs`: `Installation::platform_id: Option<String>`.** Authoritative
  `'prod'` / `'ptu'` tag from the launcher store, populated during
  `discover()` and `discover_default()`. Lets consumers route to the right
  services endpoint or scope personal state by environment without re-mapping
  from `Channel`. `None` when discovery fell back to log parsing (the log
  doesn't carry this field) or when the install was constructed via
  `Installation::from_root` / `from_parts`. Hearth uses this to keep
  PTU progress out of PU tables; downstream consumers may find it useful
  for any prod-vs-test split. Already exposed on the lower-level
  `StoreInstall`; this just plumbs it to the public `Installation`.

- **`sc-installs`: `read_identity()` / `read_identity_from()`.** Opt-in API
  exposing the currently-logged-in RSI handle from the launcher store's
  `identity` block. New type: `LauncherIdentity { handle: String }`.
  Only the handle is exposed — other identity fields (email, Heap account
  ID, tracking UUIDs) are either PII or already derivable elsewhere, and
  stay internal. New error variant `Error::LauncherIdentityMissing`
  covers the "store decrypted but no nickname" case; callers can fall
  back to log parsing (`Handle[…]` in `<Legacy login response>` lines).
  Consumers should treat the value as a single-point-in-time snapshot:
  switching RSI accounts in the launcher overwrites it, so multi-account
  discovery is fundamentally log-based.

## [v0.5.0] - 2026-05-26

### Changed (breaking)

- **`sc-contracts`: `MissionRewards.blueprint: Option<BlueprintReward>` → `blueprints: Vec<BlueprintReward>`.**
  Multi-pool missions in the DCB store multiple `BlueprintRewards`
  entries side-by-side in their `contractResults`; the old `Option`
  shape combined with an early `return` in `resolve_blueprint_reward`
  silently dropped every pool past the first. The new `Vec` shape
  preserves all entries. Field rename is intentional — every
  consumer's call site breaks at compile time, making the migration
  auditable rather than a silent semantic shift.
- **`sc-contracts`: `BlueprintReward` slimmed to `{ chance, pool_guid }`.**
  Dropped the duplicated `pool_name` and cloned `items: Vec<BlueprintItem>`
  that previously lived on each reward. Consumers resolve the pool's
  name and items via [`BlueprintPoolRegistry::get(pool_guid)`] at
  render time. Single source of truth for pool contents; no more
  silent drift between the registry and the mission's snapshot.
- **`sc-contracts`: `expand_all` no longer takes `&BlueprintPoolRegistry`.**
  Now that `materialise_blueprint` doesn't need to resolve items at
  build time, the param is dropped from the entire `expand_all` /
  `walk_handler` / `emit_*` / `build_expansion` chain. `MissionIndex::build`
  populates the reverse index after expansion via the new
  `link_missions` call.

### Added

- **`sc-contracts`: reverse-index lookups on `BlueprintPoolRegistry`.**
  New methods unlock pool → missions and item → missions queries:
  - `missions_for_pool(pool_guid) -> &[Guid]` — mission GUIDs that
    award the given blueprint pool. Populated by `MissionIndex::build`
    via the new `link_missions(&[Mission])` after `expand_all`.
  - `pools_containing_item(blueprint_record_guid) -> Vec<&BlueprintPool>` —
    every pool that contains the given blueprint record as one of
    its items.
  - `missions_for_item(blueprint_record_guid) -> Vec<Guid>` —
    convenience composition of the above two, dedup'd. Powers
    consumer queries like "missions that drop blueprint X" and
    "missions that drop blueprints I don't own yet" (the cross-
    domain query [`bulkhead`](https://github.com/VeeLume/bulkhead)
    needs once an inventory tracker lands).
- **`sc-extract`: `strip_locale_metadata` helper + universal normalization
  at parse / set time.** CIG ships some entries with a `,P` metadata
  suffix (probably *plural* or *pronoun-aware*) on weapon variant
  names — observed on ~12,873 entries in the SC 4.8 LIVE locale:
  ```
  item_Nameutfl_crossbow_ballistic_01_tint01,P=Novian "Nighthunter" Crossbow
  ```
  DCB references use the bare key, so the suffix silently broke
  lookup for every affected variant. Now stripped at every locale
  construction point: `LocaleMap::parse`, `LocaleMap::parse_utf8_bom`,
  `LocaleMap::set`. New `pub fn strip_locale_metadata(&str) -> &str`
  exposed for downstream consumers (e.g. sc-langpatch's INI parsers)
  so the same normalization can apply at their boundaries too.
- **`sc-contracts`: `blueprint_item_probe` example.** Dumps every
  `BlueprintItem` in a matching mission's blueprint pools with full
  DCB metadata — record names, entity record + struct type,
  `LocalizedItemCache` entries, every component on the crafted entity
  + any `Localization` blocks they carry. Diagnostic tool for
  investigating display-name resolution gaps. Built during the locale
  `,P` investigation and kept around for future digging.

### Fixed

- **`sc-contracts`: `blueprint_pool_consistent` switched to per-set
  equality.** With `MissionRewards.blueprints` now a `Vec`, the
  old "all members agree on one pool guid" check needs to compare
  the *set* of pool guids across members. Members A {X, Y} and B {X, Y}
  are consistent; A {X, Y} and C {X, Z} are not.

### Internal

- `cargo fmt` pass across `crates/sc-contracts/src/axes.rs` and
  ~8 example files. Pure whitespace reflow, no semantic change.

## [v0.4.0] - 2026-05-24

### Added

- **`sc-contracts`: tag-axis classifier.** New `axes` module exposes
  `AxisKind`, `AxisValues`, `AxisDiff`, `SharedTag`. Classifies tags
  by their position in the `TagTree` (`AI/Ship/CombatClass/*`,
  `AI/SkillDefinitions/*`, `Missions/VehicleType/Ship/*`,
  `Ship/Model/*`, `EntityEffectSystem/Tags/*`, etc.) into a small
  set of player-meaningful families. Renderers use this to decide
  which axes to collapse silently (skill, combat-class agreement)
  vs surface as per-alternative variance (hull, ship class, effects
  like Distortion).

- **`Mission::combat_class()` / `Mission::ship_count_range()`.**
  `combat_class` returns the CombatClass tag shared by every ship-spawn
  alternative across the mission (e.g. `"VeryEasy"`), or `None` when
  groups disagree or have no CombatClass tag. `ship_count_range`
  returns the `(min, max)` total ship count summed across all
  ship/entity slot groups — honest min from picking the smallest
  alternative in each group, honest max from the largest.

### Changed (breaking)

- **`sc-contracts`: `EncounterPhase` reshaped to preserve
  `ShipOptions` alternatives.** Old shape `EncounterPhase { name,
  slots: Vec<S> }` flattened the three-level DCB nesting
  (`ShipGroup → ShipOptions → Ship`) and lost the boundary between
  "concurrent slot groups" (all fire) and "weighted alternatives
  inside one group" (engine picks one). New shape:
  `EncounterPhase { name, groups: Vec<SlotGroup<S>> }`, with
  `SlotGroup { options, concurrent_range, weight_uniform, axes,
  shared_tags }`. Each `SlotGroup` corresponds to one
  `SpawnDescription_ShipOptions` and carries pre-computed
  `AxisDiff` + `shared_tags` (with cached `AxisKind`) so consumers
  don't have to re-walk the tag tree.
  - Fixes ~14% of ship-spawn `MissionProperty` displays that were
    over-counting by summing concurrent across alternatives (the
    "Settle a Score shows 6× Scythe instead of 1-3" pattern). See
    the investigation thread in chat history for the full census.
  - **Migration:** read access patterns like `phase.slots.iter()` /
    `phase.slots.len()` become `phase.all_options()` /
    `phase.option_count()` for "flatten everything" consumers, or
    `phase.groups.iter().flat_map(|g| g.options.iter())` for
    consumers that want the explicit grouping. Renderers should
    iterate `phase.groups` directly to respect the alternatives
    boundary.

## [v0.3.0] - 2026-05-03

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
