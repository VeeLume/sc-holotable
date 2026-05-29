# Workspace structure

How the sc-holotable workspace is organized, what belongs where, and the conventions every crate follows. Source-of-truth for crate boundaries and the public API contract.

> **Status:** design doc, captured 2026-05-26 from a planning session that examined the live DCB record shapes. The crate set is partly aspirational — shipped today: `sc-discovery`, `sc-extract`, `sc-extract-generated`, `sc-items`, `sc-tags`, `sc-manufacturers`, `sc-crafting`, `sc-weapons`, `sc-missions`, and the `sc-holotable` umbrella. The rest land as their data needs become concrete. The framing, conventions, and naming rules apply now.

## Two tiers

The workspace organizes crates by what they *serve*, not by what DCB folder they cover.

**T1 — data-shaped (DCB-faithful).** One crate per cluster of related record types. Optimized for "give me the typed shape of records of type X." Boundaries roughly mirror CIG's folder structure so the mental model stays simple.

**T2 — query-shaped (consumer-faithful).** One crate per consumer-facing question-family. Joins across T1 freely. Hides cross-domain resolution behind a single API. Does **not** mirror CIG folders.

The split exists because the data is heavily cross-referenced and consumers asking realistic questions (*"where does Agricium spawn and what quality can I expect"*) cross 4-5 record types in 3-4 different folders. A consumer that depends on every T1 to answer one question reinvents the join every time; a T2 crate owns it once.

T0 is reserved for crates with no DCB / svarog dependency at all (`sc-discovery`, future `sc-log`).

## Crate catalogue

### T0 — no domain dependencies

| Crate | Owns | Status |
|---|---|---|
| `sc-discovery` | Install discovery (channels, paths, launcher log, `build_manifest.id`) | shipped |
| `sc-log` | Game.log event parsing (mission lifecycle, blueprint receipts, equip events) | future — driven by bulkhead Inventory panel |

### T1 — data layer

| Crate | Owns (DCB folders / record types) | Status |
|---|---|---|
| `sc-extract` | Three-stage extraction API, `LocaleMap`, `ReferenceGraph`, `RecordPaths`, the bundled-walk API (`RecordVisitor`/`BundledWalk`), `ExtractSnapshot` + generic `ProcessedSnapshot`. Re-exports svarog as escape hatch. | shipped |
| `sc-extract-generated` | Generated `Extract`/`Pooled` types from `sc-generator`. **Workspace-internal.** | shipped |
| `sc-items` | The universal item envelope: `Item` + `Items` index (the `EntityClassDefinition` + `SAttachableComponentParams.AttachDef` shape: locale keys + typed `Type`/`SubType`, `is_inventory_item`). Future: `ItemPort`, `DensityClass`, `BaseRecord + Variants` resolver. **Workspace-internal.** | shipped (thin) |
| `sc-tags` | `Tags` — hierarchical tag tree from the DCB `TagDatabase` (ancestors/descendants/path). | shipped |
| `sc-manufacturers` | `Manufacturers` — registry over `SCItemManufacturer` (GUID + code lookup). | shipped |
| `sc-weapons` | Combat-math primitives: ship + FPS + missile + melee. `ShipWeapon`, `FpsWeapon`, `Missile`. Owns damage / fire_action / sustain / ammo / classify. | shipped — partial (melee deferred) |
| `sc-vehicles` | Hulls: `entities/spaceships/` (~150 base hulls × ~6 variants) + `entities/groundvehicles/` (~40). | future |
| `sc-shipcomponents` | Non-weapon ship-bolted items: shields, thrusters, powerplants, coolers, QT/jump drives, scanners, radars, fuel, capacitors, countermeasures, missile racks, turret/weapon-mount hardpoints, ship armor. 15-30 sub-modules. | future |
| `sc-equipment` | FPS non-weapons: armor (`scitem/characters/.../pu_armor/`), gear, tools, consumables, weapon modifiers (barrel comps, scopes, mags). | future |
| `sc-crafting` | Entire `crafting/` folder: `Blueprint`, `BlueprintCategory`, `BlueprintReward`, `CraftedProperties`, `CraftingQualityDistributionRecord` (with extraction-method variants: ship / FPS / ground / harvestable / creature), `QualityQuantization`, global params. Plus `refiningprocess/` (9 records — same machinery family). | shipped (thin: `all_blueprints` + `BlueprintItem`); full `crafting/` modelling future |
| `sc-resources` | World primitives: `ResourceType`, `MineableElement`, `harvestables/`, `commoditytypedatabase` / `commodityconfiguration`, `cargomanifest/`, mining global params, `rockcompositionpresets`. *What stuff exists in the world.* Knows nothing about crafting. | future |
| `sc-locations` | `ssolarsystem`, `jumppoints`, `starmap`, `servicebeacon`, `megamap`. Extract from current `sc-missions`. | future |
| `sc-factions` | `factions/` (59 records) + `reputation/` (8 subdirs) + `lawsystem/` (4 subdirs). Tight coupling via `Faction.factionReputationRef`, `factionType=LawEnforcement`, `ableToArrest` justifies one crate. | future |
| `sc-actors` | NPC archetypes (`actor/actors/npc_archetypes/`). Only stand up when a consumer (sc-missions for missiongiver detail, bulkhead inspector for NPC profiles) needs it. | deferred |

### T2 — query layer

| Crate | Joins | Replaces |
|---|---|---|
| `sc-missions` | Missions × crafting (reward blueprints) × locations × factions × vehicles (encounter spawns). Owns mission↔X reverse indices (`missions_for_pool`, `pools_containing_item`, `missions_for_item` — already a precedent in current `BlueprintPools`). Also absorbs `contracts/`, `missiontype/`, `missiongiver/`, `missiondata/`, `missionbroker/`, `missionscenarios/`, `missionfailureconditions/`. | Renamed from current `sc-missions`. |
| `sc-loadouts` | Vehicles × shipcomponents × weapons × `loadoutkits/` × `sloadoutassortment/`. Answers "what fits where" and "default loadouts per hull / role." | New. |
| `sc-economy` | Resources × crafting × locations × factions. Answers "where does resource X spawn, with what quality, what's the refined yield, who polices that area." | New. |

### Aggregator

| Crate | Role |
|---|---|
| `sc-holotable` | Umbrella prelude. Re-exports typed surfaces from every T1 + T2 behind feature flags. **The recommended public dependency** for downstream consumers. See [Umbrella crate](#umbrella-crate) below. |

## Naming

- **T1**: `sc-<domain>` — names a thing (`sc-resources`, `sc-vehicles`, `sc-crafting`).
- **T2**: `sc-<use-case>` — names a question family (`sc-missions`, `sc-loadouts`, `sc-economy`).
- Not rigid (`sc-missions` reads like a T1; it's T2 because it joins five domains), but: T1 names a *thing*, T2 names a *question*.

## Per-crate conventions

Most are extracted from how `sc-weapons` is already written. New T1 crates follow them so the workspace API stays consistent. **Rule 6 (domain-noun cooked index) is the one that the older, quickly-evolved crates — `sc-weapons` and `sc-missions` — are migrating *toward*, not yet examples of; see the migration map in the project note.**

### 1. Foreign refs as `Guid`, never flattened strings

```rust
pub struct ShipWeapon {
    pub manufacturer_guid: Option<Guid>,   // ✓ — visible reference
    // pub manufacturer_name: String,      // ✗ — pretends to be complete
}
```

Doc-comment points at the registry / lookup function. A consumer sees the `Guid` and knows there's something to resolve. The struct doesn't pretend to own data it doesn't own.

Canonical example: [`sc-weapons/src/ship.rs:29`](../crates/sc-weapons/src/ship.rs).

### 2. `LocaleKey` + call-site resolver — never embed strings

```rust
pub struct ShipWeapon {
    pub name_key: Option<LocaleKey>,
    pub desc_key: Option<LocaleKey>,
}
impl ShipWeapon {
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> { ... }
    pub fn description<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> { ... }
}
```

Raw `@` preserved. Resolution against a `LocaleMap` is the consumer's job. See `docs/localization.md` for the workspace-wide rule.

### 3. Raw entity handle as escape hatch on every typed record

```rust
pub struct ShipWeapon {
    // ... typed fields ...
    pub entity_handle: Handle<EntityClassDefinition>,
}
```

Doc: *"escape hatch for consumers that want to reach through to unmodelled fields via `&DataPools`."* Every typed record carries this. The typed wrapper is visibly partial — a consumer holding it has one field's distance to the raw underlying record.

### 4. `raw::` module re-export

Each crate exposes a `raw::` module re-exporting the lower-layer types needed to use the entity handle:

```rust
pub mod raw {
    pub use sc_extract::svarog_datacore;
    pub use sc_extract::{DataCoreDatabase, Instance, Value};
}
```

Doc: *"reach for these only as a last resort."*

### 5. Narrow-consumer re-exports

A consumer should be able to depend on just one T1 crate and construct iterator args without adding `sc-extract` as a direct dep:

```rust
// In sc-weapons/src/lib.rs
pub use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore,
    ExtractSnapshot, Guid, LocaleKey, LocaleMap, ...
};
```

Type identity is preserved because every crate in the workspace pins the same `sc-extract` rev.

### 6. Cooked index: one domain-noun type, built uniformly

Each data crate exposes its materialized data as **one primary type named for
its domain** — a plural noun, **no structural suffix**:

| Crate            | Type            | not                              |
|------------------|-----------------|----------------------------------|
| sc-items         | `Items`         | ~~Items~~                    |
| sc-tags          | `Tags`          | ~~Tags~~                      |
| sc-manufacturers | `Manufacturers` | ~~Manufacturers~~         |
| sc-missions      | `Missions`      | ~~Missions~~                 |
| sc-weapons       | `Weapons`       | ~~`iter_*` + `WeaponPools` tuple~~ |

The data *shape* — flat lookup, hierarchy, grouping — is expressed through
**methods, not the name**: `Tags` still has `.ancestors()`/`.children()`,
`Weapons` still exposes its collision pools. A consumer never has to guess
whether a crate calls its entry point a `Cache`, `Registry`, `Index`, `Tree`,
or `Catalog` — it is the domain noun. Singular record types stay singular
(`Item`, `Mission`, `ShipWeapon`), so `Missions` (the index) vs `Mission`
(the record) reads naturally. The one sanctioned non-noun is
`sc_extract::RecordPaths` — a foundational index named for *what* it indexes
rather than a domain.

**Construction is always `Type::build`:**

```rust
let items = Items::build(store);             // &RecordStore — reads typed pools only
let paths = RecordPaths::build(&datacore);   // &Datacore — needs raw db()
let ships = Ships::build(&datacore, &tags);  // dependency indices appended by ref
```

- **Input is the narrowest sufficient** — `&RecordStore` when the build only
  reads the typed pools; `&Datacore` only when it needs `db()` (raw strings /
  file paths) or asset data. Dependency indices follow the source, by reference.
- **Returns owned `Self`** — no `&DataPools` borrow survives into the value
  (see rule 7).

**Every index also has a `{Type}Builder`** for bundled walks —
`#[derive(Default)]` + `impl RecordVisitor<Output = {Type}>`. `Type::build` may
delegate to it or stand alone; either way a consumer building several indices
fuses them into one `all_records` pass via `BundledWalk`
([`crates/sc-extract/src/bundled_walk.rs`](../crates/sc-extract/src/bundled_walk.rs)).

### 7. Owned data; grouping & streaming as members, not new top-level types

Typed records own all their data after construction — no `&DataPools` borrow
lives in the struct:

```rust
let weapon = ShipWeapon::try_new(handle, guid, pools, ...)?;
weapon.burst_dps()?;                 // plain field reads, no &DataPools
weapon.display_name(&locale)?;
```

Two uniform building blocks the domain index (rule 6) is assembled from:

- **`try_new -> Option<Self>`** for a single record that may not fit the typed
  contract (returns `None` instead of a half-built value).
- **`iter_*(&Datacore, &deps) -> impl Iterator`** for streaming a large record
  family without materializing an index — the lazy counterpart to `build`.

Secondary structures — collision pools (`LocaleKey -> Vec<Guid>`), by-axis
groupings — are **fields or methods on the domain index, never standalone
top-level types** a consumer must discover separately:

```rust
pub struct Weapons {
    pub ships: Vec<ShipWeapon>,
    pub fps: Vec<FpsWeapon>,
    pub missiles: Vec<Missile>,
    /// Collision index over all families (the former standalone `WeaponPools`,
    /// now a field). More axes (by manufacturer, size, tag) land as
    /// non-breaking sibling fields.
    pub pools: WeaponPools,
}
```

The grouping type stays an open struct with extensible axes (a new grouping is
a non-breaking sibling field) — it simply isn't an entry point of its own.

## Cross-crate conventions

### 8. T1 README leads with a T2 pointer where one exists

Each T1 crate's `README.md` opens with:

> Most consumers want **[`sc-<t2>`](...)** — this crate is the underlying data layer. Depend on it directly only if you want flat record iteration without cross-domain context.

This addresses the "I have a `ShipWeapon`, looks complete, but I don't realize there's blueprint / mission / loadout context the typed struct silently omits" footgun.

The per-record `entity_handle` (rule 3) covers *within-domain* incompleteness. The T2 pointer covers *cross-domain* incompleteness.

### 9. T2 crates re-export T1 records, never duplicate them

A T2 join crate that needs `ShipWeapon` re-exports it via `pub use sc_weapons::ShipWeapon`. It does not define its own `WeaponView` or `LoadoutWeapon`. Single type identity across the workspace.

### 10. `sc-items` is workspace-internal

Consumers reach typed wrappers (`ShipWeapon`, `Shield`, `Vehicle`) in their domain crates, not the raw `Item` envelope. The envelope is implementation detail shared between domain crates.

## Umbrella crate

`sc-holotable` is the recommended public dependency. It exists to solve four pain points that the current "depend on individual crates" pattern creates (visible in `sc-langpatch/src-tauri/Cargo.toml` today):

1. **Multiple tag pins to keep in sync.** Today: `sc-discovery` + `sc-extract` + `sc-weapons` + `sc-missions` each carry `tag = "sc-holotable/v0.5.0"`. Easy to leave one behind on bump.
2. **`sc-extract` feature flags that have to be remembered.** Without `features = ["contracts", "servicebeacon", "entities"]`, the registries silently come back empty.
3. **Svarog rev has to match holotable's pin** for type identity (the narrow-re-export pattern, rule 5, breaks otherwise).
4. **Profile overrides have to be mirrored** from holotable's `Cargo.toml`.

### Consumer-facing shape

```toml
sc-holotable = {
    git = "https://github.com/VeeLume/sc-holotable.git",
    tag = "v1.0.0",
    features = ["missions", "weapons"]
}
```

One dep. One tag. Features describe what you want, not what crates you need.

### Feature map

```toml
[features]
default = []

# T1 (data) — pulls one crate each, with required sc-extract DCB features
installs        = ["dep:sc-discovery"]
extract         = ["dep:sc-extract"]
weapons         = ["dep:sc-weapons", "extract", "sc-extract/entities-scitem"]
vehicles        = ["dep:sc-vehicles", "extract",
                   "sc-extract/spaceships", "sc-extract/groundvehicles"]
shipcomponents  = ["dep:sc-shipcomponents", "extract", "sc-extract/entities-scitem"]
equipment       = ["dep:sc-equipment", "extract"]
crafting        = ["dep:sc-crafting", "extract", "sc-extract/crafting"]
resources       = ["dep:sc-resources", "extract", "sc-extract/mining"]
locations       = ["dep:sc-locations", "extract", "sc-extract/starmap"]
factions        = ["dep:sc-factions", "extract", "sc-extract/factions"]

# T2 (queries) — pull their full T1 transitive set
missions = ["dep:sc-missions", "crafting", "locations", "factions", "vehicles"]
loadouts = ["dep:sc-loadouts", "vehicles", "shipcomponents", "weapons", "equipment"]
economy  = ["dep:sc-economy",  "resources", "crafting", "locations", "factions"]

# Convenience aggregators
all-t1 = ["weapons", "vehicles", "shipcomponents", "equipment",
          "crafting", "resources", "locations", "factions"]
all-t2 = ["missions", "loadouts", "economy"]
full   = ["installs", "all-t1", "all-t2"]
```

### Module shape

```rust
// sc_holotable::weapons::{ShipWeapon, FpsWeapon, Missile, WeaponPools}
// sc_holotable::missions::{Mission, Missions, ...}
// sc_holotable::asset::{AssetSource, AssetData, Datacore, ExtractSnapshot}
// sc_holotable::install::Install
// sc_holotable::prelude::*    — the common types: Mission, ShipWeapon, Vehicle,
//                               LocaleMap, LocaleKey, Guid, ...
// sc_holotable::raw::*         — the per-crate escape hatch surfaces
```

Both modules (for explicit naming) and a `prelude::*` (for "give me the common types").

### What langpatch's Cargo.toml becomes

Before (today):
```toml
svarog-p4k       = { git, rev = "7f06225" }
svarog-datacore  = { git, rev = "7f06225" }
sc-discovery      = { git, tag = "sc-holotable/v0.5.0" }
sc-extract       = { git, tag = "sc-holotable/v0.5.0",
                     features = ["contracts", "servicebeacon", "entities"] }
sc-weapons       = { git, tag = "sc-holotable/v0.5.0" }
sc-missions     = { git, tag = "sc-holotable/v0.5.0" }
```

After:
```toml
sc-holotable = { git, tag = "v1.0.0", features = ["installs", "missions", "weapons"] }
```

Six git deps → one. Feature flags described declaratively. Svarog never named.

### Versioning policy

- Adding a feature is non-breaking. Removing a feature is breaking.
- T1 record-type internals can change without major bumps as long as their public field set stays stable (the `entity_handle` escape hatch, rule 3, gives flexibility for downstream consumers who reach unmodelled fields).
- Single unified workspace tag (`v1.0.0`, `v1.1.0`, ...). No per-crate tags.

### Future option: T1 → workspace-internal

Once consumers have migrated to the umbrella, the individual T1 crates can be marked workspace-internal (`publish = false`). The public API contract becomes the umbrella's surface. This is a one-line change later — not committed now.

## Open questions

These need investigation before the corresponding crate locks its API.

### Variant resolution heuristic

`entities/spaceships/` has 920 records but ~150 base hulls — the rest are variants:

```
aegs_avenger_stalker.xml                            base
aegs_avenger_stalker_pu_ai_advocacy.xml             AI variant
aegs_avenger_stalker_pu_ai_civ_lowfuel.xml          variant of variant
```

Same with FPS weapons (color variants `_black02`, `_mat01`), ship armor (`_invincible`, `_invulnerable`), AI-module turrets.

The naming looks regular (`_pu_ai_*`, `_<color><digits>`, `_lowfuel`, `_invulnerable`), but a heuristic needs validation against 20+ samples before going into `sc-items::BaseRecord + Variants`. Without it, every consumer's "show me all Avenger Stalkers" produces 9 adjacent rows of the same ship.

### Ship armor vs FPS armor cross-tree

Both `scitem/ships/armor/` and `scitem/characters/human/armor/pu_armor/` use `Type=Armor` in `SItemDefinition`. Disambiguation is presumably by `SubType` or tags. Worth a quick FPS armor record peek before deciding whether `sc-shipcomponents::armor` and `sc-equipment::armor` share a base via `sc-items::ArmorBase` or stay parallel.

### Mineable spawn placement

"Where does Agricium spawn" requires data we haven't located yet. `MineableElement` records describe extraction physics (instability, optimal window, cluster factor) but not *where* nodes appear. Probably in `objectcontainers/`, `level/`, or `starmap/` planet definitions. Without this, `sc-economy::where_does_resource_spawn` returns "I know what Agricium is but not where to find it."

### Turret child-port primitive

`turret` records expose child `ItemPort`s for the mounted weapons. The "items that themselves expose ItemPorts" capability is generic — `missile_racks`, `weapon_mounts`, `subitem` likely behave the same way. Verify the ItemPort primitive in `sc-items` handles child ports cleanly before locking the API.

### Log scraping crate timing

`sc-log` is planned as a T0 sibling to `sc-discovery` (parsing `Game.log` for mission lifecycle, blueprint receipts, equip events — see the bulkhead status note for the SCMDB-derived design). Justified once bulkhead's Inventory or Mission-history panels start landing. Not before — empty crates rot.

### `sc-actors` deferral threshold

NPC archetypes (`actor/actors/npc_archetypes/`) are referenced by `MissionGiver.entityClass`. For displaying a missiongiver's localized name + cooldowns, `sc-missions` can stay self-contained. For richer NPC profiles (the SCMDB-style mission inspector), `sc-actors` becomes a hard dependency. Stand it up when the second consumer asks.
