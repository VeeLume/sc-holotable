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
