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

## [v0.10.0] - 2026-05-31

### Added

- **`sc-items`: `catalog` module + two-tier `ItemCatalog` index.** A
  gear catalog that recovers the design ↔ model ↔ colorway relationship
  CIG doesn't model cleanly, in two tiers so a UI can render either a
  whole matching design or a single item with its colorways:

  - **`Collection`** — the models that read as the **same design**:
    "Geist Armor" helmet + arms + core + legs; a gun with its magazine.
    Carries a human `name` ("Geist Armor") and its member model ids.
  - **`Model`** — one design + one slot: "Geist Armor Helmet" and all
    its colorways; "LH86 Pistol" and its paints; "LH86 Magazine" on its
    own. Carries `base: Guid` + `members: Vec<Guid>` (base first — the
    plain "design + slot" item, else a "…Base" default, else CIG's
    canonical first colorway `…_01_01_01`, else shortest name),
    `item_type`/`item_sub_type`, and an optional `collection` link.

  Classification is driven by the **display name** — the signal users
  actually read — not by CIG's `Armor / FPS / Set / …` tag, which proved
  unreliable (one set tag bundles visually-unrelated models: the
  `FieldRecon` tag mixes Geist, Field Recon, and FBL-8a helmets). The
  design name is the base display name's words up to the slot noun
  (Helmet/Arms/Core/…, known from `item_type`): "Geist Armor Helmet" →
  "Geist Armor". Colorways trail *after* the slot noun, so they fold
  into one model regardless of how the colorway or underlying record is
  named. A `Model` is `(design, item_type, item_sub_type, size, grade)`;
  a `Collection` is `(category, design)`. The `size`/`grade` split (from
  `SItemDefinition`, now surfaced on `Item`) keeps distinct ship-weapon
  size classes apart — "Deadbolt I Cannon" (S1) and "Deadbolt V Cannon"
  (S5) are separate models under one "Deadbolt" collection — robustly,
  where parsing the display numeral would fail ("Omnisky IX" is S3).
  Weapons/clothing have no slot noun, so their design is the leading word
  (grouping a gun with its magazine, or a cannon's whole size ladder).

  Scope: a **gear** catalog (armor, clothing, weapons), built over
  [`Items`] and restricted to real inventory items
  (`is_inventory_item()`). Excluded: non-inventory attachables (NPC
  archetypes, seat-access, doors, tattoos); non-gear items (ship
  components, world props); and dev-template items with placeholder
  names (`<= PLACEHOLDER =>`).

  API:
  ```rust
  let catalog = ItemCatalog::build(&items, &paths, &locale);
  catalog.model_of(guid)              // -> Option<&Model>      (item + colorways)
  catalog.collection_of(guid)         // -> Option<&Collection> (the design)
  catalog.models_in(collection)       // iterate a design's models
  catalog.members_of_collection(col)  // iterate every member of a design
  catalog.base_of(guid)               // -> Option<Guid>
  model.variants() / model.is_solo()
  ```

  Verified against live SC DCB (SC 1.0): the polluted `FieldRecon` tag
  dissolves into the "Geist Armor", "Field Recon Suit", and "FBL-8a"
  designs a user would actually name; mixed-type models and
  non-inventory models are both 0. Classification needs only `&Items` +
  `&LocaleMap` — no tag tree, so sc-items no longer depends on `sc-tags`
  (`&RecordPaths` is used only to pick a model's base/header). serde-clean
  — round-trips through `ProcessedSnapshot`.

## [v0.9.0] - 2026-05-31

### Added

- **New crate `sc-resources`** — typed catalog over the DCB's
  `ResourceType` records (`libs/foundry/records/resourcetypedatabase/`).
  Surfaces `Resources::build(&RecordStore)` + GUID lookup +
  `refined_version_of()` graph walk + `RecordVisitor` impl for bundled
  walks. `Resource` carries display name + description, thumbnail
  paths, RTT entity ref, `refined_version` (the raw→refined edge),
  `validate_default_cargo_box`, `density: Option<Density>`, and
  `volatility: Option<Volatility>`. SC 4.8: 206 records, 30 refining
  edges, 205 resolve names, 206/206 carry density with physically
  grounded values (Tungsten Ore = 19,300 kg/m³ ≈ real Tungsten).
  Plus the shared `CargoQuantity` primitive (Standard/Centi/Micro
  variants with `.to_scu()` normalization) and `DensityUnit`
  (GramsPerCm3 with `.to_kg_per_m3()`).

- **`sc-crafting` future-proofing pass — full recipe surface.** Owns
  the whole `libs/foundry/records/crafting/` subsystem (minus the
  legacy salvage/repair recipes). 8 typed surfaces, every polymorphic
  enum carries an `Other { type_name, struct_index }` fallback for
  dormant variants so future regen-after-population promotes them
  cleanly. Live-validated end-to-end against SC 4.8 LIVE.
  - **`Blueprints`** index + `Blueprint { category, process, tiers }`
    with by_record_guid / by_category / by_crafted_entity reverse
    lookups. `Process { Creation { entity_class } | Other }`. Tier
    carries `recipe: Option<Recipe>` + `research: Option<Research>`.
    Sample: P4-AR Rifle resolves to 150s craft time + Aluminum 0.04
    SCU + Hephaestanite 0.02 SCU + Iron 0.02 SCU.
  - **`Duration { days, hours, minutes, seconds }`** projected from
    `TimeValue_Partitioned` (the only populated `TimeValue_*` in 4.8).
  - **`Recipe { craft_time, costs, results, is_shared }`** with
    `RecipeCosts { mandatory, optional }` and the recursive
    `Cost { Resource(ResourceCost) | Item(ItemCost) | Select { count,
    options } | Other }` tree. SC 4.8 universally shapes mandatory
    as `Select(Select(Resource))`.
  - **`ResourceCost { resource: Guid, quantity: Option<CargoQuantity>,
    min_quality }`** and `ItemCost { entity_class, quantity, min_quality }`.
  - **`RecipeResult { Item | Resource | Other }`** — Vec preserved
    even though SC 4.8 results are universally empty (Creation's
    `entity_class` IS the output).
  - **`Research { unlock, costs }`** + `ResearchUnlock { Default |
    Other }`. Present-but-empty on 57% of tiers.
  - **`Categories`** — 20 marker-record categories (FPSWeapons /
    FPSArmours / VehicleWeaponsS1-6 / Medical / Refining/Dismantle
    Examples / …) + database_guid, built from `RecordPaths`.
  - **`GlobalParams`** singleton — refining_quality_unit_multiplier,
    default_composition_quality, dismantle blacklists, and
    `default_blueprint_whitelist` (the 9 default-unlocked blueprints
    at character start: basic dismantle, P4-AR, behr_pistol, light
    combat armor parts, ammo magazines).
  - **`GameplayProperties`** (29 records) — `GameplayProperty {
    property_name_key, unit_format_key, display_transformation,
    name_overrides }` with `DisplayTransformation { Scale { factor }
    | ConvertFactorToPercentChange | ConvertFactorToNegatedPercentChange
    | ConvertValueToFactorOfBaseValue | Sequence(Vec<Self>) | Other }`
    (recursive). `PropertyNameOverride` with `OverrideCondition::ItemType
    { match_item_types: Vec<EItemType>, match_sub_types: Vec<EItemSubType> }`
    via the enum_serde DCB-string adapter pattern.
  - **`Quality`** — standalone records under `qualitydistribution/`,
    `qualitylocationoverride/`, `qualityquantization/`. 10 / 12 / 38
    in SC 4.8. `QualityDistributionShape::Normal { min, max, mean,
    stddev }`, `LocationOverrideEntry { location, distribution:
    Option<DistributionRef> }` where `DistributionRef { Inline |
    Record(Guid) | Other }` captures both inline and shared-by-RecordRef
    forms, and `QuantizationBand { start, end, mapped_value }` maps
    quality ranges to discrete outputs. Per-resource quality data
    (also reachable via `ResourceType.properties` → `ResourceTypeCraftingData`)
    has been confirmed independent of the standalone records (134/134
    location-override entries inline, 0 RecordRef cross-links).
  - **`cargo_quantity_from_ptr`** projection helper. Lives here, not
    sc-resources, because the Centi/Micro pool types are gated under
    the `crafting` feature.
  - **`BlueprintsBuilder`** implementing `RecordVisitor` for bundled
    walks (interest = `["CraftingBlueprintRecord"]`).

- **Umbrella `sc-holotable` integration.** New `resources` feature,
  `resources` module re-exporting `sc_resources::*`. `crafting` now
  depends on `resources` transitively. Prelude grows the full
  sc-resources + sc-crafting type surface.

### Changed

- **`Foundations` grows a `resources: Resources` field.**
  `build_foundations` is now a 5-builder bundled `all_records` pass
  (was 4). `RecordPaths` still declares `AllRecords` so the walk shape
  doesn't change — Resources rides along for free.

- **`HolotableSnapshot` grows `resources: Option<Resources>`** and
  bumps `HOLOTABLE_COOK_VERSION` from 1 to 2. Old snapshots fall back
  per the `ProcessedSnapshot` version-guard machinery
  (rebuild from raw / live).

### Removed (breaking)

- **`sc_crafting::BlueprintItem`** and the free functions
  **`sc_crafting::all_blueprints` / `sc_crafting::resolve_blueprint`**.
  Use [`Blueprints::build`] + [`Blueprints::get`] + [`Blueprints::iter`]
  instead — the new types carry everything the old shims did, plus the
  full recipe surface. `sc-missions` re-exports of the same names also
  dropped; `sc_missions::BlueprintPoolEntry.blueprint` is now a
  `sc_crafting::Blueprint`.

### Fixed

- `holotable_snapshot_round_trip` test built `HolotableSnapshot`
  without its `items` field — was masked because workspace
  `cargo check --all-targets` runs with the umbrella's empty default
  features, so the test target never compiled.

## [v0.8.0] - 2026-05-29

### Changed (breaking)

- **Uniform API shape: domain-noun cooked indexes** (new rules 6–7 in
  `docs/workspace-structure.md`). Each data crate exposes one primary type
  named for its domain, built with `Type::build`, shape via methods, grouping
  folded in as a field. Renames: `ItemCache`→`Items`, `TagTree`→`Tags`,
  `ManufacturerRegistry`→`Manufacturers`, `MissionIndex`→`Missions`, and the
  sc-missions sub-types `ShipRegistry`/`LocationRegistry`/`LocalityRegistry`/
  `RewardCurrencyCatalog`/`BlueprintPoolRegistry` →
  `Ships`/`Locations`/`Localities`/`RewardCurrencies`/`BlueprintPools`. Each
  index also exposes a `{Type}Builder` for bundled walks.
- **Foundational builders take `&RecordStore`** (narrowest sufficient input):
  `Items`/`Tags`/`Manufacturers::build(&RecordStore)`. Builders needing the raw
  db (`RecordPaths`) or asset data keep `&Datacore`; dependency indices are
  appended by reference.
- **`sc-weapons`: `build_weapon_pools` (tuple) → `Weapons`** —
  `Weapons { ships, fps, missiles, pools }` via `Weapons::build(&Datacore,
  &Items)`. `WeaponPools` is now the `pools` field; `iter_*` remain for
  streaming.
- **Crate rename `sc-contracts` → `sc-missions`.**
- **Crate rename `sc-installs` → `sc-discovery`** — the `install` substring
  tripped Windows installer-detection (UAC), blocking the test binary from
  launching. The crate discovers/locates the install; it doesn't install.

### Removed

- **`DatacoreConfig`** — dissolved; cooked indices are explicit `X::build`
  calls in their owning crates.
- **`DatacoreSnapshot`, `Datacore::snapshot()`, `Datacore::into_snapshot()`** —
  `Datacore` owns a `RecordStore` directly; use `Datacore::records()`.

### Added

- **`RecordPaths`** (sc-extract) — every record's file path/name/type by GUID
  plus a `/`-segment trie (`get`/`at`/`under`/`children`/`roots`). The DCB path
  is a classification axis (e.g. manufacturer kind).
- **Bundled-walk API** (sc-extract) — `RecordVisitor`/`Interest`/`VisitItem`/
  `BundledWalk` + tuple `VisitorSet`. Build several indices in one
  `all_records` pass; `X::build` stays as the single-index path.
- **Generic `ProcessedSnapshot<T>`** (sc-extract) — serialize a cooked index
  (zstd + msgpack, envelope + cook-schema version guard) for sub-second load
  vs the raw snapshot's full re-parse. All four cooked indices are
  serde-capable (`LocaleKey` gains serde; the generated `EItemType`/
  `EItemSubType` round-trip as DCB strings via a new generator-emitted
  `as_dcb_str`).
- **`sc-holotable` umbrella crate** — feature-gated re-exports + `prelude`,
  `build_foundations` (one bundled pass over all foundational indices), and
  `HolotableSnapshot` (a serializable bundle of the cooked indices). The
  recommended public dependency.

## [v0.7.0] - 2026-05-28

### Added

- **`sc-extract`: item classification on `LocalizedItem` / `LocalizedItemCache`.**
  The per-entity cache now also captures the raw `AttachDef.Type` and
  `AttachDef.SubType` strings (e.g. `"Char_Armor_Helmet"`, `"WeaponPersonal"`)
  alongside the existing locale keys — they come from the same
  `SAttachableComponentParams.AttachDef` (`SItemDefinition`) walk, so no extra
  pass. New `LocalizedItem.item_type` / `.item_sub_type` fields and
  `LocalizedItemCache::item_type(guid)` / `::item_sub_type(guid)` accessors.
  Values are the DCB enum-value names verbatim, leaving category mapping to
  consumers. Hearth uses this to group its blueprint catalog by item type.
- The localization walk now tolerates a missing `Localization` block: an
  entity with a `Type` but no localized name is still cached (previously it
  was skipped entirely). Existing name-resolution behaviour is unchanged for
  entities that do have `Localization`.

## [v0.6.0] - 2026-05-28

### Added

- **`sc-discovery`: `Installation::platform_id: Option<String>`.** Authoritative
  `'prod'` / `'ptu'` tag from the launcher store, populated during
  `discover()` and `discover_default()`. Lets consumers route to the right
  services endpoint or scope personal state by environment without re-mapping
  from `Channel`. `None` when discovery fell back to log parsing (the log
  doesn't carry this field) or when the install was constructed via
  `Installation::from_root` / `from_parts`. Hearth uses this to keep
  PTU progress out of PU tables; downstream consumers may find it useful
  for any prod-vs-test split. Already exposed on the lower-level
  `StoreInstall`; this just plumbs it to the public `Installation`.

- **`sc-discovery`: `read_identity()` / `read_identity_from()`.** Opt-in API
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

- **`sc-missions`: `MissionRewards.blueprint: Option<BlueprintReward>` → `blueprints: Vec<BlueprintReward>`.**
  Multi-pool missions in the DCB store multiple `BlueprintRewards`
  entries side-by-side in their `contractResults`; the old `Option`
  shape combined with an early `return` in `resolve_blueprint_reward`
  silently dropped every pool past the first. The new `Vec` shape
  preserves all entries. Field rename is intentional — every
  consumer's call site breaks at compile time, making the migration
  auditable rather than a silent semantic shift.
- **`sc-missions`: `BlueprintReward` slimmed to `{ chance, pool_guid }`.**
  Dropped the duplicated `pool_name` and cloned `items: Vec<BlueprintItem>`
  that previously lived on each reward. Consumers resolve the pool's
  name and items via [`BlueprintPools::get(pool_guid)`] at
  render time. Single source of truth for pool contents; no more
  silent drift between the registry and the mission's snapshot.
- **`sc-missions`: `expand_all` no longer takes `&BlueprintPools`.**
  Now that `materialise_blueprint` doesn't need to resolve items at
  build time, the param is dropped from the entire `expand_all` /
  `walk_handler` / `emit_*` / `build_expansion` chain. `Missions::build`
  populates the reverse index after expansion via the new
  `link_missions` call.

### Added

- **`sc-missions`: reverse-index lookups on `BlueprintPools`.**
  New methods unlock pool → missions and item → missions queries:
  - `missions_for_pool(pool_guid) -> &[Guid]` — mission GUIDs that
    award the given blueprint pool. Populated by `Missions::build`
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
- **`sc-missions`: `blueprint_item_probe` example.** Dumps every
  `BlueprintItem` in a matching mission's blueprint pools with full
  DCB metadata — record names, entity record + struct type,
  `LocalizedItemCache` entries, every component on the crafted entity
  + any `Localization` blocks they carry. Diagnostic tool for
  investigating display-name resolution gaps. Built during the locale
  `,P` investigation and kept around for future digging.

### Fixed

- **`sc-missions`: `blueprint_pool_consistent` switched to per-set
  equality.** With `MissionRewards.blueprints` now a `Vec`, the
  old "all members agree on one pool guid" check needs to compare
  the *set* of pool guids across members. Members A {X, Y} and B {X, Y}
  are consistent; A {X, Y} and C {X, Z} are not.

### Internal

- `cargo fmt` pass across `crates/sc-missions/src/axes.rs` and
  ~8 example files. Pure whitespace reflow, no semantic change.

## [v0.4.0] - 2026-05-24

### Added

- **`sc-missions`: tag-axis classifier.** New `axes` module exposes
  `AxisKind`, `AxisValues`, `AxisDiff`, `SharedTag`. Classifies tags
  by their position in the `Tags` (`AI/Ship/CombatClass/*`,
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

- **`sc-missions`: `EncounterPhase` reshaped to preserve
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

- **`sc-discovery`: launcher-store discovery.** New module `launcher_store`
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

- **`sc-discovery`: broader log parsing.** Recognises `[Installer]` and
  `Deleting <root>\loginData.json` markers in addition to
  `[Launcher::launch]`, so log-fallback discovery finds channels that
  have been installed but never launched. New `LogEntry` /
  `LogEntryKind` types; `parse_launcher_log_entries` returns
  `Vec<LogEntry>`.

- **`Channel::install_dir_name()`** — on-disk directory name for a
  channel (notably `TECH-PREVIEW`, distinct from the display name `TECH`).

### Changed (breaking)

- **`sc-discovery`: `Installation::launcher_version_string()` removed.**
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

- **`sc-discovery`: `Error::NoLaunchEntries` renamed** to
  `Error::NoInstallEntries` — the error now also covers logs that have
  no `Installer` markers, not just no launch markers.

- **`sc-discovery`: `parse_launcher_log_entries` return type changed**
  from `Vec<(Channel, PathBuf)>` to `Vec<LogEntry>`.

- **`sc-discovery`: legacy plain-text manifest parsing removed.** Only
  the v2 nested `{"Data": {...}}` shape is parsed now. The legacy
  flat-format support was code path debt — every shipped Star Citizen
  build the workspace targets uses the v2 shape.

## [v0.2.0] - 2026-05-02

### Changed (breaking)

- **`sc-missions` v2 redesign — Mission-centric API.** Wholesale
  rename and reshape of the public surface; every consumer needs
  updates. Design doc: `docs/sc-missions-v2.md`; consumer guide:
  `docs/sc-missions-guide.md`.
  - `Contract` → `Mission`, `ContractIndex` → `Missions`,
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
    inheritance chain manually should switch to `Missions` /
    `MissionPools`.

### Added

- **`MissionPools`** precomputed on the index, with opt-in
  divergence helpers for consumers that need to reason about per-BP
  pool drift without re-walking the graph themselves.
- **`Ships::resolve_spawn`** — tag-tree subsumption + spawn-
  state filter. Recovered 80 of 335 previously-empty ship-encounter
  slots on SC 4.7 LIVE (24% drop). Three coupled bugs fixed: ancestor
  tag matching, `AI > Ship > SpawnFlags` / `AI > CargoManifest` state
  tags being misread as identity filters, and the dual-`Ship`-node
  intent gate over-firing. See `status.md` "Last worked on" for the
  data-driven derivation.
- **Narrow-consumer re-exports** on `sc-missions` and `sc-weapons`
  (`EntityClassDefinition`, locale-key cluster API) so callers don't
  have to depend on `sc-extract` directly for common types.
- **`tools/sc-explorer`** — interactive TUI binary with three tabs
  (Pools / Contracts / Weapons). Per-crate `tui` modules
  (`sc-missions/src/tui`, `sc-weapons/src/tui`) own their domain
  views behind a `tui` feature.
- **Investigation examples** (committed under
  `crates/sc-missions/examples/`) — `contract_dump`, `ambush_dig`,
  `damage_dig`, `salvage_pool`, `spawn_dig`, `encounter_analytics`,
  `encounter_kinds`, `role_investigation`, `tier_investigation`.
  Canonical way to run a quick dig against a fresh DCB regen.

### Fixed

- `EncounterSlot` now forwards typed signals on ship-spawn entries
  (previously dropped during extraction).
- TUI encounter-detail view surfaces the forwarded typed signals.

## [v0.1.0] - 2026-04-25

### Added

- `sc_missions::Contract` and `sc_missions::Variation` now carry
  `title_key: Option<LocaleKey>` and `description_key: Option<LocaleKey>`
  alongside the resolved `title` / `description` strings. Consumers
  patching `global.ini` (sc-langpatch, translation-extraction tooling)
  no longer need to re-walk the contract inheritance chain to recover
  the raw INI key the displayed text was resolved from.
- `sc_missions::ResolvedText` gained the same `title_key` /
  `description_key` fields; `resolve_contract_text` fills them during
  the existing inheritance walk at no extra cost.
- `sc_missions::ExpandedContract` propagates the keys so
  pre-merge consumers see them too.

### Changed

- `ResolvedText` is still constructible via field-init but now requires
  four fields instead of two. Call sites that built it manually should
  use `..Default::default()` to stay forward-compatible.
