# `sc-extract` — design specification

> Status: **implemented and in use as of 2026-04-13.** Phases 2a + 2c + 2d are landed (hand-written half). Phase 2b (vehicle XML) is deliberately deferred — the design is still captured in the "Vehicle XML" section below as forward-looking spec.
>
> The API was reworked on 2026-04-13 to split runtime state from serializable data. See the "Entry points" and "Result types" sections for the current shape. Older implementation notes in `implementing/sc-extract-2a.md` and `implementing/sc-extract-2c.md` describe the pre-rework state and are kept for historical context.

## Purpose

`sc-extract` is the foundational data-access crate for the holotable workspace. It owns the complete generated DataCore type catalog, wraps svarog with ergonomic re-exports, builds the reference graph, parses non-DCB game files (vehicle XML, `global.ini`), and exposes every cross-cutting service (tags, manufacturers, localization, display names, filters, vehicle data) that domain crates depend on.

It is the only non-`sc-installs` crate in the workspace that has svarog as a dependency. Every other crate (`sc-ammo`, `sc-weapons`, `sc-contracts`, future component crates, future `sc-ships`) depends on `sc-extract` and gets its svarog access through re-exports.

**Two data sources, one crate.** Star Citizen game data lives in two fundamentally different formats inside `Data.p4k`: the DataCore (`Game2.dcb`, ~500 MB binary object database, ~200k records) for most game data, and **CryXmlB vehicle XML files** (`data/scripts/entities/vehicles/implementations/xml/*.xml`) for ship structure, hardpoint definitions, and flight performance. sc-extract handles both — the layering is on data source, not on format.

## Consumers and what they need

| App or crate | What it needs |
|---|---|
| `sc-ammo` | Generated `AmmoParams` + related types via `DatacoreSnapshot::records`; svarog re-exports for custom extraction. |
| `sc-weapons` | Generated `EntityClassDefinition`, `SCItemWeaponComponentParams`, fire-action enum via `DatacoreSnapshot::records`; access to ammo records through the same store. |
| `sc-contracts` | Generated contract generator types; **reference graph** (for reverse lookups from tags/blueprints back to contracts); tag tree; localization. |
| `bulkhead` (future consumer) | Everything above plus playable-content filters, display-name cache, snapshot round-trip. |
| `sc-langpatch` (future consumer) | Everything above plus `LocaleMap` round-trip (parse + serialize patched `global.ini`) and `sc-installs` path helpers for the write target. |

## Scope

**What `sc-extract` owns:**

- svarog re-exports and ergonomic aliases
- Generated DataCore type catalog (produced by `tools/sc-generator`, see `docs/codegen.md`)
- The `Extract` + `Pooled` traits and the `Builder` (flat-pool materialiser) emitted by the generator
- `ReferenceGraph` (built once during parse, rebuilt on snapshot hydrate)
- Tag tree navigation (hand-written on top of generated `Tag` records)
- Manufacturer registry (hand-written lookup on top of generated `SCItemManufacturer`)
- `LocaleMap` — `global.ini` parse + serialize (UTF-16 LE with BOM)
- **Vehicle XML** (CryXmlB) parsing — *deferred; spec only.* The non-DCB ship data source; see the Vehicle XML section below.
- Entity display-name resolver (pre-computed during parse, cached in snapshot)
- Playable-content filters (`is_playable_weapon`, `is_playable_ship`)
- Staged result types: [`AssetSource`] (live P4K handle or memory-backed byte map), [`AssetData`] + [`Datacore`] + [`DatacoreSnapshot`] (runtime session owning `DataCoreDatabase`), [`ExtractSnapshot`] (on-disk byte bundle wrapping raw captured DCB + asset bytes)
- Three staged entry points: [`AssetSource::from_install`], [`AssetData::extract`], [`Datacore::parse`], plus the snapshot quartet [`ExtractSnapshot::capture`] / `save` / `load` / `hydrate`

**What `sc-extract` does *not* own:**

- Domain-specific wrappers (`Weapon`, `Ammo`, `Ship`, etc.). Those live in domain crates.
- Weapon-intrinsic calculations (DPS, sustained DPS, heat curves). Those live in `sc-weapons`.
- Damage-pipeline calculations (shield → armor → hull). Those live in `bulkhead` for now, eventually `sc-ships`.
- Filesystem writes to the SC install directory. `sc-langpatch` owns its own `std::fs::write` calls using path helpers from `sc-installs`.
- Stateful selection / UI logic.
- Tauri or Specta bindings. Serde derives are always-on, Specta is not offered (domain crates can add it at their own boundary).

## svarog re-exports

```rust
// sc-extract/src/lib.rs

// Full-namespace escape hatches for the three svarog crates.
pub use svarog_common;
pub use svarog_datacore;
pub use svarog_p4k;

// Ergonomic aliases at the crate root:
pub use svarog_datacore::{
    ArrayElementType, ArrayRef, DataCoreDatabase, DataType, Instance, InstanceRef, Query, Record,
    RecordRef, Value,
};
pub use svarog_common::CigGuid as Guid;
```

Consumers write `use sc_extract::{Guid, Value, Instance};` for the 90% case and drop to `sc_extract::svarog_datacore::...` when they need something unusual. sc-extract does **not** redefine any of these types — they all come from svarog.

Distinct reference semantics preserved from svarog:

| svarog type | Meaning | Where it appears |
|---|---|---|
| `Value::Class { struct_index, data }` | Inline class, bytes embedded in parent's byte stream | Most nested struct fields |
| `Value::ClassRef(InstanceRef)` | Array element of Class type, stored as a separate instance | Arrays of class structs |
| `Value::StrongPointer(Option<InstanceRef>)` | Owning pointer into an instance pool | Some component references |
| `Value::WeakPointer(Option<InstanceRef>)` | Non-owning pointer into an instance pool | Cross-component refs |
| `Value::Reference(Option<RecordRef>)` | Cross-record reference by GUID | Ammo ← weapon, manufacturer ← item, tag ← any |

All four are semantically distinct and the generated code + reference graph preserve the distinction.

## Generated types

The `sc_extract::generated::*` module contains every DataCore struct and enum as Rust types, produced once per game patch by `tools/sc-generator`. See `docs/codegen.md` for the generator design.

Key facts for consumers:

- All types derive `Serialize`, `Deserialize`, `Debug`, `Clone`. `Default` is *not* derived (reachability pruning + flat-pool handles don't need it, and skipping the derive cuts compile time). **Note (v5 snapshot rework):** nothing in the workspace currently instantiates serde against these generated types — the `ExtractSnapshot` persistence path archives raw DCB bytes and re-parses on load, so the derives are latent trait impls with zero codegen cost. They're kept on the struct definitions as an optional future tool (e.g. JSON debug dumps), not because anything persists through them.
- Abstract base types (types in the DCB with multiple concrete subclasses) become polymorphic enums: `SWeaponActionParamsVariant { SWeaponActionFireRapidParams(...), SWeaponActionFireSingleParams(...), SWeaponActionSequenceParams(...), Unrecognized(String) }`.
- Inheritance is **flattened** — parent-struct fields are inlined into the child struct rather than being kept as a `_parent: ParentType` field. See the rationale in `docs/codegen.md`.
- Field names are `snake_case` versions of DataCore field names. Rust keyword collisions use raw identifiers (`r#type`, `r#ref`) or trailing-underscore (`Self_`).
- Every generated struct implements the `Extract` + `Pooled` traits.

### The `Extract` / `Pooled` traits and the `Builder`

The earlier sketch of a single recursive `FromInstance::from_instance` was replaced by a **flat-pool** materialiser. The generator emits one `impl Extract<'a>` + one `impl Pooled` per reachable struct, and the hand-written [`Builder`] in `sc-extract-generated` walks every DCB record iteratively through a heap-allocated worklist, writing slot handles (`Handle<T>`) into a per-type `DataPools`.

```rust
pub trait Pooled: Sized {
    /// Returns a mutable reference to this type's pool inside DataPools,
    /// growing it if necessary.
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>>;
    fn pool<'p>(pools: &'p DataPools) -> &'p Vec<Option<Self>>;
}

pub trait Extract<'a>: Sized + Pooled {
    /// Materialize this type from a svarog `Instance`, allocating handles
    /// into `builder.pools` for any nested class / pointer fields.
    fn extract(inst: &Instance<'a>, builder: &mut Builder<'a>) -> Option<Self>;
}
```

The `Builder` owns a `&'a DataCoreDatabase`, a `DataPools`, a `RecordIndex`, and a worklist. Its `consume_database().finish()` pipeline seeds the worklist from `db.all_records()`, drains it iteratively, and returns a [`RecordStore`] ready for typed access. See `implementing/sc-generator.md` for the worklist details and why the old recursive `FromInstance` hit a stack overflow.

Consumers almost never see `Extract` or `Builder` directly — they use the resulting [`RecordStore`] via [`Datacore`].

## The reference graph

The `ReferenceGraph` captures every cross-record reference found in the DCB during parsing. Built once per session, queried at runtime for both forward and reverse lookups. This is the crate's answer to sc-langpatch's "find all contracts that reference this tag" patterns — it replaces linear scans with O(1) lookups. (Prior to the v5 snapshot rework the graph was also serialized into snapshot files; it's now rebuilt at hydrate time alongside the rest of the cooked state, since hydrate re-runs the full `Datacore::parse` pipeline.)

```rust
pub struct ReferenceGraph {
    /// For each record: its outgoing reference edges.
    out_edges: HashMap<Guid, Vec<Edge>>,
    /// For each target GUID: the records that reference it (inverse of out_edges).
    in_edges: HashMap<Guid, Vec<Guid>>,
}

pub struct Edge {
    pub from: Guid,
    pub to: Guid,
    /// Dotted path into the source record, e.g. "contractResults[2].blueprintPool".
    /// Stored as an interned string to keep memory bounded.
    pub field_path: InternedStr,
    pub kind: EdgeKind,
}

#[derive(Debug, Clone, Copy)]
pub enum EdgeKind {
    Reference,     // Value::Reference — GUID-based cross-record ref
    StrongPtr,     // Value::StrongPointer
    WeakPtr,       // Value::WeakPointer
}

impl ReferenceGraph {
    // ── Forward lookup (who does this record reference?) ────────────────

    pub fn outgoing(&self, from: &Guid) -> &[Edge];
    pub fn outgoing_of_kind(&self, from: &Guid, kind: EdgeKind) -> impl Iterator<Item = &Edge> + '_;

    // ── Reverse lookup (who references this record?) ───────────────────

    pub fn incoming(&self, target: &Guid) -> &[Guid];

    /// Reverse lookup scoped to a specific record type.
    /// Replaces sc-langpatch's current linear-scan + filter pattern.
    pub fn incoming_of_type<'a>(
        &'a self,
        target: &Guid,
        type_name: &str,
        records: &'a RecordStore,
    ) -> impl Iterator<Item = &'a Guid> + 'a;

    // ── Full iteration ─────────────────────────────────────────────────

    pub fn all_edges(&self) -> impl Iterator<Item = &Edge>;
}
```

**Memory budget:** 200k records × ~5 outgoing edges each ≈ 1M edges. Each edge: `16B (from) + 16B (to) + 4B (field_path interned id) + 1B (kind) = ~40B`. Total ≈ 40MB in memory. Acceptable for a desktop app. The `field_path` strings are interned to avoid duplicating common paths like `"Components"` across thousands of edges.

**The graph is built exactly once per session**, during [`Datacore::parse`] when [`DatacoreConfig::build_graph`] is true. Apps that load a snapshot get a fresh graph via [`ExtractSnapshot::hydrate`] — hydrate re-runs the full parse pipeline against the captured DCB bytes, so every cooked index (graph, tags, manufacturers, display names) is rebuilt deterministically.

> Note: `ReferenceGraph` was simplified as part of the flat-pool refactor — edges are now `Guid → Vec<Guid>` instead of carrying a `field_path` or `kind`. The `Edge { from, to, field_path, kind }` shape shown in the next block is the earlier design, preserved here for historical context. The live code only exposes `outgoing(guid) -> &[Guid]` and `incoming(guid) -> &[Guid]`.

## Record store

The snapshot holds every extracted record as a fully-materialized typed value, indexed for typed and untyped access. **All records are parsed eagerly during [`Datacore::parse`]; the store never holds lazy views, raw bytes, or references back into the DCB.** See the "Parsing model" section below for the full rationale.

> The shape below is the original per-type-HashMap sketch. The live implementation uses a **flat-pool** `RecordStore` built on top of the generic `Handle<T>(u32)` type — see `implementing/sc-generator.md` for the actual layout. The ergonomic accessor story (typed `entity_class()` / `manufacturer()` helpers) is not yet exposed; consumers walk the pools directly today.

The exact internal shape is a generated detail (see `docs/codegen.md`) — consumers see the following API:

```rust
pub struct RecordStore { /* generated per-type HashMaps, one per top-level record type */ }

impl RecordStore {
    /// Total number of records across all types.
    pub fn len(&self) -> usize;

    /// Untyped lookup by GUID. Returns a `RecordView` that the caller
    /// pattern-matches on to get the typed value.
    pub fn get(&self, guid: &Guid) -> Option<RecordView<'_>>;

    /// Iterate records of a specific type by name.
    /// Replaces sc-langpatch's current `records_by_type_containing` + filter pattern.
    pub fn records_of_type(&self, type_name: &str) -> impl Iterator<Item = RecordView<'_>> + '_;

    /// Generated convenience methods for each top-level type:
    pub fn entity_class(&self, guid: &Guid) -> Option<&EntityClassDefinition>;
    pub fn tag_record(&self, guid: &Guid) -> Option<&Tag>;
    pub fn manufacturer(&self, guid: &Guid) -> Option<&SCItemManufacturer>;
    pub fn all_entity_classes(&self) -> impl Iterator<Item = (&Guid, &EntityClassDefinition)> + '_;
    // ... one pair per top-level type, emitted by the generator
}

/// Untyped view over any stored record.
pub enum RecordView<'a> {
    EntityClassDefinition(&'a EntityClassDefinition),
    Tag(&'a Tag),
    SCItemManufacturer(&'a SCItemManufacturer),
    // ... one variant per top-level type, emitted by the generator
}
```

The `records_of_type` and `get` methods handle cross-type iteration without the caller having to know about the internal per-type storage. The generated convenience methods give zero-cost typed access to specific types without pattern matching.

## Tag tree

Hand-written navigation helpers on top of the generated `Tag` records. The DCB has ~18k tag records arranged in a strict parent-child tree.

```rust
pub struct TagTree {
    by_guid: HashMap<Guid, TagNode>,
    by_name: HashMap<String, Vec<Guid>>,  // name collisions happen; return all matches
    roots: Vec<Guid>,                     // top-level tags (no parent)
}

pub struct TagNode {
    pub guid: Guid,
    pub name: String,
    pub parent: Option<Guid>,
    pub children: Vec<Guid>,
    /// Legacy numeric id on some older tags. None for newer ones.
    pub legacy_guid: Option<u32>,
}

impl TagTree {
    pub fn get(&self, guid: &Guid) -> Option<&TagNode>;

    /// Returns all tags with this name. Collisions exist in the DCB.
    pub fn by_name(&self, name: &str) -> &[Guid];

    /// Walk upward to the root.
    pub fn ancestors(&self, guid: &Guid) -> impl Iterator<Item = &TagNode> + '_;

    /// Walk downward (depth-first).
    pub fn descendants(&self, guid: &Guid) -> impl Iterator<Item = &TagNode> + '_;

    /// True if `guid` is a (transitive) descendant of `ancestor`.
    pub fn is_descendant_of(&self, guid: &Guid, ancestor: &Guid) -> bool;

    /// Dotted path from root, e.g. `["Global", "Manufacturer", "Aegis"]`.
    pub fn path(&self, guid: &Guid) -> Vec<&str>;
}
```

The tree is built during parse by walking `TagDatabase` records (which contain nested `Tag` records via `children` arrays) and indexed by both GUID and name. Rebuilt on snapshot hydrate.

## Manufacturer registry

Flat ~100 records. Hand-written lookup on top of the generated `SCItemManufacturer` struct.

```rust
pub struct ManufacturerRegistry {
    by_guid: HashMap<Guid, SCItemManufacturer>,
    by_code: HashMap<String, Guid>,  // "GATS" → Guid, "AEGS" → Guid
}

impl ManufacturerRegistry {
    pub fn get(&self, guid: &Guid) -> Option<&SCItemManufacturer>;
    pub fn by_code(&self, code: &str) -> Option<&SCItemManufacturer>;
    pub fn all(&self) -> impl Iterator<Item = &SCItemManufacturer> + '_;
}
```

Built during parse by scanning all records of type `SCItemManufacturer`. Rebuilt on snapshot hydrate.

## LocaleMap

Parses and round-trips `global.ini` — the UTF-16 LE (with BOM) localization file shipped inside `Data.p4k` at `Data/Localization/<lang>/global.ini`.

```rust
pub struct LocaleMap {
    entries: HashMap<String, String>,
    language: Language,
}

impl LocaleMap {
    /// Parse from UTF-16 LE bytes (handles BOM).
    pub fn parse(bytes: &[u8], language: Language) -> Result<Self>;

    /// Serialize back to UTF-16 LE bytes with BOM.
    /// Used by `sc-langpatch` to produce its patched override file.
    pub fn serialize(&self) -> Vec<u8>;

    pub fn get(&self, key: &str) -> Option<&str>;

    /// Resolve a localization key that may have an `@` prefix.
    /// `"@item_NameGATS_Ballistic_S4"` and `"item_NameGATS_Ballistic_S4"`
    /// both look up the same entry.
    pub fn resolve(&self, loc_key: &str) -> Option<&str>;

    pub fn set(&mut self, key: String, value: String);
    pub fn remove(&mut self, key: &str) -> Option<String>;

    pub fn language(&self) -> Language;
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_;
}

pub enum Language {
    English,
    German,
    French,
    Spanish,
    Italian,
    ChineseSimplified,
    Japanese,
    Korean,
    Polish,
    Russian,
    // expand as SC adds more
}

impl Language {
    pub fn folder_name(self) -> &'static str;  // "english", "german", ...
}

/// Convenience extractor that pulls a locale from `Data.p4k` in one call.
/// Chains svarog P4K reading + LocaleMap::parse. For consumers that already
/// have bytes from another source (e.g. a patched override file), use
/// `LocaleMap::parse` directly.
pub fn extract_locale(p4k_path: &Path, lang: Language) -> Result<LocaleMap>;
```

**Scope boundary:** `sc-extract` deals in bytes and types. It does **not** write patched `global.ini` files to the install's override path — that is `sc-langpatch`'s concern, using a path helper from `sc-installs` and the serializer from `LocaleMap`. This preserves the rule that `sc-extract` has no dependency on `sc-installs`.

## Vehicle XML (CryXmlB files)

> **Status: not implemented (phase 2b deliberately deferred).** This section is forward-looking spec. When vehicle XML parsing lands, it plugs in naturally alongside the DCB/locale split at two layers: (a) runtime — a hand-written parser produces cooked typed structs, held in a `VehicleXmlStore` inside [`AssetData`] or adjacent; (b) archival — the raw vehicle XML bytes get captured into the existing `ExtractSnapshot::files` map via a flipped `SnapshotCaptureConfig::archive_vehicle_xmls` flag, and re-parsed on hydrate. No format version bump is needed because the byte-bundle shape generalizes trivially. A real working implementation exists in the sibling `sc-damage-calculator` crate at `src/extract/hull.rs` (~490 lines) and will be ported when the first consumer needs it.

The DCB doesn't contain everything. Ships specifically have two fundamentally different data sources: the DCB holds the ship **entity** (`VehicleComponentParams`, default loadout references), while the **vehicle XML** holds the hull structure, hardpoint definitions with gimbal limits, pipe topology, and flight performance. A complete ship model requires merging both sources.

Vehicle XMLs are **CryXmlB** (binary CryEngine XML) files stored in the P4K at paths like `data/scripts/entities/vehicles/implementations/xml/aegs_gladius.xml`. The DCB's `VehicleComponentParams.vehicleDefinition` field points at the XML path. When implemented, sc-extract will parse these during extraction and store the results alongside DCB records.

**Parsing is hand-written, not generated.** The vehicle XML schema is ad-hoc, inconsistent, and accumulates historical oddities — codegen isn't a useful fit. The hand-written types live in `sc-extract/src/vehicle_xml/`, separate from `src/generated/`.

### Types

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleXml {
    pub name: String,              // "AEGS_Gladius"
    pub display_name: String,      // "AEGS Gladius" (raw, not yet localized)
    pub sub_type: String,          // "Vehicle_Spaceship"
    pub size: u32,

    /// Recursive tree of hull parts. Only parts with `damage_max > 0` or
    /// children with damage are included; purely structural wrapper parts
    /// are flattened away.
    pub parts: Vec<VehiclePart>,

    /// Flattened list of every item port found anywhere in the part tree.
    /// Also reachable via walking `parts`.
    pub item_ports: Vec<VehicleItemPort>,

    /// Named pipes (Power, Heat, Fuel, Shield, WeaponRegen, WeaponAmmoLoad, ...).
    pub pipes: Vec<VehiclePipe>,

    pub damage_multipliers: VehicleDamageMultipliers,
    pub movement_params: Option<VehicleMovementParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiclePart {
    pub name: String,
    /// "Animated", "AnimatedJoint", "ItemPort", ...
    pub class: String,
    pub damage_max: f32,
    pub detach_ratio: Option<f32>,
    pub children: Vec<VehiclePart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleItemPort {
    pub port_name: String,         // "hardpoint_gun_left_wing"
    pub display_name: String,
    pub min_size: u32,
    pub max_size: u32,
    pub types: Vec<VehicleItemPortType>,
    pub flags: String,             // "invisible uneditable", "left wing", etc.
    pub weapon_group: String,
    pub pitch: Option<GimbalRange>,
    pub yaw: Option<GimbalRange>,
    pub pipe_connections: Vec<VehiclePipeConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleItemPortType {
    pub kind: String,              // "WeaponGun", "SeatAccess", "LandingSystem"
    pub sub_types: Vec<String>,    // e.g. ["Gun", "GunTurret"]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GimbalRange {
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiclePipe {
    pub name: String,              // "MainPower", "WeaponRegenCrew"
    pub class: String,             // "Power", "Heat", "WeaponRegen"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiclePipeConnection {
    pub pipe_class: String,        // "Power", "WeaponRegen"
    pub pipe: String,              // "MainPower", "WeaponRegenCrew"
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleDamageMultipliers {
    pub bullet: Option<f32>,
    pub bullet_energy: Option<f32>,
    pub bullet_distortion: Option<f32>,
    pub explosion: Option<f32>,
    pub explosion_splash: Option<f32>,
    pub collision: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleMovementParams {
    pub max_engine_thrust: Option<f32>,
    pub max_retro_thrust: Option<f32>,
    pub max_directional_thrust: Option<f32>,
    pub max_angular_acceleration: Option<(f32, f32, f32)>,
    pub rotation_damping: Option<f32>,
}
```

### Parsing API

```rust
/// Parse a single vehicle XML from raw CryXmlB bytes.
pub fn parse_vehicle_xml(bytes: &[u8]) -> Result<VehicleXml>;

/// Extract every vehicle XML referenced by any ship ECD in the P4K.
/// Will be called by `Datacore::parse` during extraction when implemented.
///
/// Takes a map of ship record GUID → `vehicleDefinition` path string.
/// Deduplicates by path (multiple ship variants often share a vehicle XML)
/// and returns a map of ship record GUID → parsed `VehicleXml`.
///
/// Uses a filename-suffix pre-filter to avoid scanning all ~700k P4K entries
/// during lookup. Pattern ported from sc-damage-calculator's `hull.rs`.
pub(crate) fn extract_all_vehicle_xmls(
    archive: &P4kArchive,
    vehicle_defs: &HashMap<Guid, String>,
) -> HashMap<Guid, VehicleXml>;
```

### Iteration helpers

```rust
impl VehicleXml {
    /// Walk the part tree depth-first, yielding every part.
    pub fn all_parts(&self) -> impl Iterator<Item = &VehiclePart> + '_;

    /// Walk the part tree depth-first, yielding only parts with `damage_max > 0`.
    pub fn hull_parts(&self) -> impl Iterator<Item = &VehiclePart> + '_;

    /// Total hull HP — sum of `damage_max` across every part in the tree.
    pub fn total_hull_hp(&self) -> f32;

    /// Body HP heuristic — sum of `damage_max` at depth 1 (direct children
    /// of the root part). See bulkhead's `docs/damage-system.md` for the
    /// rationale behind this particular depth-1 rule.
    pub fn body_hp(&self) -> f32;

    /// Find an item port by exact name.
    pub fn find_port(&self, name: &str) -> Option<&VehicleItemPort>;
}
```

### Scope — what's covered and what isn't

**Covered:**

- Hull part tree with `damage_max` (primary source of ship HP)
- Item ports with types, sizes, gimbal limits, pipe connections
- Pipe topology (Power, Heat, Fuel, Shield, WeaponRegen, WeaponAmmoLoad)
- Vehicle-level damage multipliers (bullet, energy, explosion, collision)
- Movement / flight performance parameters

**Deliberately not covered yet:**

- `<Modifications>` (ship variants like `PirateSalvage`, `S42_ImportantCharacter`, `SpaceMines`) — these override fields of a base ship definition. No current consumer needs variant resolution. Revisit if `sc-ships` is added and needs to materialize variants.
- `<Damages>` behaviors (camera shake, burn effects, destroy effects) — cinematics, not gameplay.
- `<Physics>` buoyancy / water resistance — not relevant to space combat.
- `<VisorElement>`, HUD palette, landing offset, misc. UI metadata.

The rule: include anything bulkhead's damage calc or sc-langpatch will want; exclude cosmetic / UI / modification content until there's a concrete consumer need.

### Prior art

This module is ported from sc-damage-calculator's `src/extract/hull.rs` (~490 lines, working implementation — the real thing, not a stub like bulkhead's placeholder `vehicle_xml.rs`). Key differences from sc-damage-calculator's version:

- **Typed structs, not JSON strings.** sc-damage-calculator serialized `types` and `pipe_connections` as JSON strings for SQLite storage. sc-extract uses proper Rust structs since the snapshot format is msgpack, not SQL.
- **Pipes list added.** sc-damage-calculator didn't extract `<Pipes>`. sc-extract adds it for component-connection analysis (understanding which pipes a given hardpoint draws from).
- **Movement params added.** sc-damage-calculator didn't extract `<MovementParams>`. sc-extract adds it because bulkhead's damage sim will eventually need flight performance for target modeling.
- **`GimbalRange` as a typed struct** instead of four separate `Option<f32>` fields.

The extraction-time path-matching optimization (filename-suffix pre-filter before full path comparison, to avoid lowercasing all 700k+ P4K entries) transfers over as-is. It's load-bearing at parse time — without it, vehicle XML lookup is seconds slower per ship.

## Entity display-name resolver

Computing a display name for an entity requires walking:

```
EntityClassDefinition.Components
  → SAttachableComponentParams
    → AttachDef
      → Localization
        → Name  (e.g. "@item_NameGATS_Ballistic_S4")
          → strip '@' → lookup in LocaleMap
            → resolved string (e.g. "CF-337 Panther")
```

This is the single most common operation across consumers (bulkhead UI, sc-langpatch contract enrichment, sc-weapons display). Every consumer currently walks it independently. `sc-extract` computes it once during parse and caches the result in the snapshot:

```rust
/// Pre-computed display names for every EntityClassDefinition that has one.
/// Held inside the live `DatacoreSnapshot`, computed during `Datacore::parse`
/// when `DatacoreConfig::build_display_names` is true and the accompanying
/// `AssetData` had a non-empty locale.
pub struct DisplayNameCache {
    by_record: HashMap<Guid, String>,
}

impl DisplayNameCache {
    pub fn get(&self, record: &Guid) -> Option<&str>;
}

/// Low-level resolver used by the parse path. Consumers normally read from
/// the pre-computed cache instead of calling this directly.
pub fn resolve_entity_display_name(
    ecd: &EntityClassDefinition,
    locale: &LocaleMap,
) -> Option<String>;
```

## Playable-content filters

Not every record is user-visible content. bulkhead's `CLAUDE.md` documents the rules (weapons without localization are placeholders, AI ship variants are hidden, debris/wreck ships are hidden, etc.). `sc-extract` exposes these rules as predicates — consumers decide whether to filter, the lib does not filter for them.

```rust
pub fn is_playable_weapon(ecd: &EntityClassDefinition, display_name: Option<&str>) -> bool;
pub fn is_playable_ship(ecd: &EntityClassDefinition, display_name: Option<&str>) -> bool;
pub fn is_playable_shield(ecd: &EntityClassDefinition, display_name: Option<&str>) -> bool;
// expand as sibling component crates land
```

The exact rules live in bulkhead's `docs/damage-system.md` and migrate here as those crates are written. The rules live centrally in `sc-extract` rather than in each domain crate — centralizing avoids drift when multiple consumers disagree about what counts as "playable".

## Result types

The crate splits data into three orthogonal layers along the **data-source** axis (assets vs. datacore) and the **lifetime** axis (live handles vs. owned runtime data vs. on-disk byte bundles):

| Type | Layer | Kind | Source |
|---|---|---|---|
| [`AssetSource`] | runtime | live P4K handle *or* memory-backed byte map | file / snapshot |
| [`AssetData`] | runtime | locale + future asset caches | asset files (e.g. `global.ini`) |
| [`Datacore`] | runtime | owned `DataCoreDatabase` + inner [`DatacoreSnapshot`] | DCB |
| [`DatacoreSnapshot`] | runtime | records, graph, tags, manufacturers, display names | DCB |
| [`ExtractSnapshot`] | serializable (on-disk) | `{ meta, files: BTreeMap<String, Vec<u8>> }` byte bundle | captured from `AssetSource` |
| [`SnapshotMeta`] | serializable | schema version + game version + build id + timestamp | install manifest |

```rust
/// Read-only byte feed, backed by either a live `.p4k` mmap or a
/// captured byte map from a snapshot. The same reading API dispatches
/// over either — `Datacore::parse` and `AssetData::extract` don't care
/// which source produced the bytes they consume.
pub struct AssetSource { /* Live(P4kArchive) | Memory(BTreeMap<String, Vec<u8>>) */ }

/// Owned runtime bundle of asset-sourced values. Currently just the locale;
/// designed to grow. Not serialized.
#[derive(Debug, Clone, Default)]
pub struct AssetData {
    pub locale: LocaleMap,
}

/// Cooked runtime bundle of every DCB-derived value from one parse pass.
/// Not serialized — persistence happens at the ExtractSnapshot layer by
/// archiving raw DCB bytes and re-parsing on load.
#[derive(Debug, Clone, Default)]
pub struct DatacoreSnapshot {
    pub records: RecordStore,
    pub graph: ReferenceGraph,
    pub tag_tree: TagTree,
    pub manufacturers: ManufacturerRegistry,
    pub display_names: DisplayNameCache,
}

/// Live datacore session: owns the parsed `DataCoreDatabase` (so raw svarog
/// queries stay available) plus the cooked `DatacoreSnapshot`.
pub struct Datacore { /* db + snapshot */ }

impl Datacore {
    pub fn db(&self) -> &DataCoreDatabase;          // raw escape hatch
    pub fn snapshot(&self) -> &DatacoreSnapshot;
    pub fn into_snapshot(self) -> DatacoreSnapshot;
    pub fn records(&self) -> &RecordStore;
}

/// On-disk single-file snapshot format (v5). Archives raw captured bytes,
/// not cooked typed pools — see "Snapshot format" below for the rationale.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractSnapshot {
    pub meta: SnapshotMeta,
    pub files: BTreeMap<String, Vec<u8>>,  // keyed by in-archive path
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnapshotMeta {
    pub schema_version: u32,
    pub game_version: String,
    pub build_id: String,
    pub extracted_at: String,     // RFC 3339 UTC
}
```

The runtime types (`AssetData`, `DatacoreSnapshot`, `Datacore`) hold cooked data for the duration of a session; they are not directly persisted. The only serde-bound type on the hot persistence path is `ExtractSnapshot` itself, which holds primitive bytes — this is a deliberate choice to avoid cascading `Serialize`/`Deserialize` monomorphization across the ~6,000 generated types in `sc-extract-generated`. See "Snapshot format" and `docs/benchmarks.md` for the compile-time story.

## Parsing model: eager by design

[`Datacore::parse`] performs a **single, complete, eager pass** over the DataCore. Every record is seeded into the [`Builder`]'s worklist, every `Extract::extract` is called, every field is materialized into a slot handle inside `DataPools`. By the time `Datacore::parse` returns, the caller has a fully-typed [`DatacoreSnapshot`] — and [`Datacore`] *also keeps the `DataCoreDatabase` alive* so raw svarog queries remain available through [`Datacore::db`].

The [`DatacoreSnapshot`] itself holds no references into the DCB bytes and survives the database being dropped. Consumers query against it for the rest of the session and drop it when done.

### Why eager

Two reasons, in order of importance:

1. **The cross-cutting services have to walk everything anyway.** `ReferenceGraph` needs a full pass to discover every edge. `TagTree` needs a full pass to find every tag. `DisplayNameCache` needs a full pass to resolve every entity's display name. `ManufacturerRegistry` needs a full pass. If you're walking the DCB to build these indices, you might as well materialize the records while you're there — the marginal cost per record is small.

2. **Memory is affordable.** A full eager `DatacoreSnapshot` fits comfortably in the memory budget of a modern desktop app. The one workspace consumer with tight memory constraints (streamdeck-starcitizen) doesn't use `sc-extract` at all — it only depends on `sc-installs`.

(An earlier revision of this section listed "the snapshot model requires it" as the primary reason — that rationale evaporated in the v5 format change. Snapshots now archive raw DCB bytes and re-parse on load, so the cooked `DatacoreSnapshot` no longer needs to survive serialization. Eager parsing is still the right call, but the justification is now about query-time ergonomics, not serde compatibility.)

### Memory budget (estimate)

These are rough projections. Real numbers will come from the first implementation pass. If any category turns out substantially larger, we reconsider — but none is on a trajectory to blow up meaningfully.

| Component | Approximate size |
|---|---|
| `RecordStore` (all records materialized) | 60–120 MB |
| `ReferenceGraph` | ~40 MB |
| `vehicle_xmls` | ~5 MB (few hundred ships, typed) |
| `DisplayNameCache` | ~5 MB |
| `LocaleMap` | ~5 MB |
| `TagTree` | ~2 MB |
| `ManufacturerRegistry` | < 1 MB |
| **Total** | **~120–180 MB** |

For comparison: the raw DCB (`Game2.dcb`) inside `Data.p4k` is ~500 MB. Our extracted representation is *smaller* than the DCB because typed values drop per-record metadata overhead (property tables, string pool references, value array indirection) and keep only the data itself.

### Lazy was considered and rejected

An earlier sketch considered materializing records on demand via `FromInstance` against a borrowed `&'db DataCoreDatabase`. Three reasons it was rejected:

- **Indices are eager anyway.** The graph, tag tree, display names, and manufacturers all need a full pass to build. Laziness only saves on the `RecordStore`, which is ~50% of total memory. Partial optimization for significant complexity.
- **Lifetime complexity propagates.** `Datacore<'db>` would poison every consumer signature down the stack. Every function that takes `Datacore` would need a lifetime parameter. Unworkable at the scale of three consumer apps.
- **Query-time cost.** Lazy materialization reparses bytes on every access. Death by a thousand cuts for consumers that touch many records — which all of them do.

The parse-time cost of eager loading (15–25 s on a cold run) is paid once per session — consumers that want to skip it across sessions use a snapshot, which captures the DCB bytes and re-drives eager parse on load.

> **Note:** The *owned* `DataCoreDatabase` inside [`Datacore`] is a different beast from a *borrowed* `&'db DataCoreDatabase` lifetime parameter. The owned version keeps the db alive as an escape hatch for raw svarog queries without propagating lifetimes into every consumer signature.

### `ExtractSnapshot::load` vs. `hydrate`

[`ExtractSnapshot::load`] is the *cheap* step: read the file, zstd-decompress it, and `rmp_deserialize` the envelope into an `ExtractSnapshot`. The result is just metadata plus the captured byte map; no DCB parse has happened yet. This matters for consumers that enumerate many historical snapshots (e.g. bulkhead's patch-diff UI) — they can inspect `meta.game_version` on every file without paying per-snapshot parse cost.

[`ExtractSnapshot::hydrate`] is the *expensive* step: build an in-memory [`AssetSource`] from the captured bytes, run [`AssetData::extract`] and [`Datacore::parse`] against it, and return the live runtime types. This is where the 15–25 s parse cost lives; consumers call it lazily on the one snapshot they actually want to query.

### Runtime filters via `DatacoreConfig`

Expensive indices can be disabled at parse time through [`DatacoreConfig`]:

```rust
pub struct DatacoreConfig {
    pub build_graph: bool,          // ~7s, +15 MB
    pub build_tag_tree: bool,
    pub build_manufacturers: bool,
    pub build_display_names: bool,  // needs a non-empty AssetData.locale
}

impl DatacoreConfig {
    pub fn all() -> Self;           // everything including graph
    pub fn standard() -> Self;      // everything except graph — recommended default
    pub fn minimal() -> Self;       // just records
    pub fn builder() -> DatacoreConfigBuilder;
}
```

[`AssetConfig`] is the asset-side equivalent — currently just `build_locale: bool`, designed to grow. These runtime toggles are orthogonal to the compile-time **feature flags** that gate which generated struct types are even present in the build; see `docs/feature-gating.md` for the latter.

A possible future optimization is "profile-aware" parsing — tighter filters that drop records before materialization, not just skipping index-building. Not planned; add when a consumer actually needs it.

## Entry points

Three staged building blocks, plus the snapshot round-trip pair. Each step maps to one of the four consumer patterns.

```rust
impl AssetSource {
    /// Open a `Data.p4k` by path. Parses the central directory; file
    /// reads are on-demand.
    pub fn open(p4k_path: &Path) -> Result<Self>;

    /// Convenience: open the `Data.p4k` from a discovered installation.
    pub fn from_install(install: &sc_installs::Installation) -> Result<Self>;
}

impl AssetData {
    /// Read every asset-sourced file enabled by `config` (currently just
    /// `global.ini`). Returns an owned, serde-friendly bundle.
    pub fn extract(assets: &AssetSource, config: &AssetConfig) -> Result<Self>;
}

impl Datacore {
    /// Parse the DCB from an open AssetSource and build every index
    /// enabled by `config`. `asset_data` provides the locale map used to
    /// resolve display names — pass `AssetData::default()` if unneeded.
    ///
    /// Tens of seconds on a cold parse; the returned Datacore owns the
    /// live DataCoreDatabase for raw queries via `db()`.
    pub fn parse(
        assets: &AssetSource,
        asset_data: &AssetData,
        config: &DatacoreConfig,
    ) -> Result<Self>;
}

impl ExtractSnapshot {
    /// Read the raw bytes of well-known files out of a live `AssetSource`
    /// and bundle them as a snapshot. `config` controls which categories
    /// of files get archived (DCB, locale, future vehicle XMLs).
    pub fn capture(
        assets: &AssetSource,
        meta: SnapshotMeta,
        config: &SnapshotCaptureConfig,
    ) -> Result<Self>;

    /// Serialize to the on-disk format (msgpack envelope + zstd). Atomic
    /// write via `<path>.tmp` and rename.
    pub fn save(&self, path: &Path) -> Result<()>;

    /// Read from disk, zstd-decompress, msgpack-decode the envelope.
    /// Cheap: does NOT re-parse the DCB. Returns
    /// `Err(Error::SnapshotVersionMismatch)` if the schema_version
    /// doesn't match this build.
    pub fn load(path: &Path) -> Result<Self>;

    /// Re-parse the captured DCB + locale bytes into a live
    /// `Datacore` + `AssetData`. ~15–25s of work; consumers should call
    /// this lazily on the one snapshot they want to query, not eagerly
    /// for every historical snapshot in a listing.
    pub fn hydrate(
        &self,
        asset_config: &AssetConfig,
        dc_config: &DatacoreConfig,
    ) -> Result<(AssetData, Datacore)>;
}
```

### The four consumer patterns

```rust
// 1. install only — use sc-installs, don't touch sc-extract
let install = sc_installs::discover_primary()?;

// 2. install + assets — streamdeck-starcitizen pattern
let assets = AssetSource::from_install(&install)?;
let bytes = assets.read("data/libs/foo.xml")?;

// 3. install + assets + datacore — bulkhead / sc-langpatch pattern
let assets = AssetSource::from_install(&install)?;
let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

// High-level access via the cooked snapshot:
let snap = datacore.snapshot();
for record in snap.records.iter() { /* ... */ }

// Raw escape hatch while the session is alive:
let db = datacore.db();
let entities = db.records_by_type("EntityClassDefinition");

// Persist for next cold-start: capture the raw bytes from the live
// archive and write the envelope to disk.
let on_disk = ExtractSnapshot::capture(
    &assets,
    sc_extract::snapshot_meta_from_install(&install),
    &SnapshotCaptureConfig::standard(),
)?;
on_disk.save(path)?;

// 4. install + datacore only (rare — locale lives in AssetData)
let assets = AssetSource::from_install(&install)?;
let datacore = Datacore::parse(&assets, &AssetData::default(), &DatacoreConfig::standard())?;
// display_names is empty because no locale was built.

// Load back from disk with no live P4K handle. `load` only deserializes
// the envelope — it does NOT re-parse the DCB.
let loaded = ExtractSnapshot::load(path)?;
println!("archived snapshot from {}", loaded.meta.game_version);

// When you actually want to query the data, call `hydrate`. This is
// where the 15–25s parse cost lives — reserve it for the one snapshot
// you're actually diffing or querying, not every file in a listing.
let (loaded_assets, loaded_datacore) = loaded.hydrate(
    &AssetConfig::standard(),
    &DatacoreConfig::standard(),
)?;
for record in loaded_datacore.snapshot().records.iter() { /* ... */ }
```

**Snapshots are self-contained.** An archived `.snap` file carries the raw `game2.dcb` and `global.ini` bytes inside its envelope, so `load` + `hydrate` produces fully-populated runtime types against the current generated schema, regardless of which game version the snapshot came from. Historical snapshots across many SC patches can all be loaded into the same consumer binary because the parse-time schema binding is deferred to hydrate time.

## Snapshot format

On-disk: a single file — conventionally `<name>.snap`, but sc-extract doesn't enforce a suffix.

- **Envelope** (`msgpack` via `rmp-serde::to_vec_named`) holds `ExtractSnapshot { meta, files: BTreeMap<String, Vec<u8>> }`. The envelope's type graph is entirely primitive (`u32`, `String`, `Vec<u8>`, `BTreeMap`), so its serde monomorphization cost is trivial — unlike earlier revisions, which serialized the cooked generated types and hit catastrophic compile-time regressions. See `docs/benchmarks.md`.
- **Zstd** (level 3) wraps the whole envelope. Applied at the file boundary: write = `rmp_serialize → zstd_encode → atomic write via .tmp`; read = `std::fs::read → zstd_decode → rmp_deserialize`.
- **Payload**: the captured `files` map contains the raw bytes of well-known files copied out of the source archive at capture time. A standard snapshot holds `game2.dcb` (~300 MB uncompressed, ~60 MB zstd'd) plus `english/global.ini` (~2 MB uncompressed, ~500 KB zstd'd). Expected on-disk size: ~60–80 MB per SC patch.

### The capture / load / hydrate split

Three discrete phases, intentionally decoupled so consumers only pay for what they need:

| Phase | What it does | Cost |
|---|---|---|
| `capture` | Reads raw bytes from a live `AssetSource` and builds the `files` map. Runs at extract time; the caller chooses which categories to include via `SnapshotCaptureConfig`. | ~1 s |
| `save` | msgpack encode + zstd compress + atomic write. | ~2 s |
| `load` | Read file + zstd decode + msgpack decode. **Does not parse the DCB.** Consumers can inspect `meta` on every snapshot in a listing without paying per-snapshot parse cost. | <1 s |
| `hydrate` | Build an in-memory `AssetSource` from the captured bytes, run `AssetData::extract` + `Datacore::parse`. This is where the actual parse cost lives. | ~15–25 s |

### Why bytes and not cooked types?

Earlier revisions serialized [`DatacoreSnapshot`] directly via `rmp_serde::to_vec_named(&snapshot)`. Functionally correct, but the call site transitively monomorphized `<T as Serialize>::serialize::<RmpSerializer>` for every generated type reachable from `DataPools` — about 1,756 instantiations for a narrow feature like `ammoparams`, ~6,000 for `--features full`. sc-extract's LLVM IR hit 4+ million lines and full cold builds hit **3h 26m** with a 25 GB RAM peak.

Switching to a byte-bundle shape collapses the type graph of `ExtractSnapshot` to `{u32, String, Vec<u8>, BTreeMap}`. Total LLVM IR in sc-extract drops to ~400 k lines, full cold builds to ~25–40 m, peak RAM to ~6 GB. Neither the `Serialize` nor `Deserialize` derives on `RecordStore` / `DataPools` / `RecordIndex` are ever instantiated anywhere in the workspace — they become latent trait impls with zero codegen cost.

This isn't just a compile-time win. It also **fixes historical-snapshot compatibility**: because the parse-time schema binding is deferred to hydrate time, an archived 4.7 snapshot parses correctly against any future set of generated types. Field renames, struct reshuffles, and type additions across game patches all resolve naturally via svarog's field-by-name resolution at parse time, with no migration code. This is what enables bulkhead's "compare weapon stats across archived SC patches" use case.

### Capture config

```rust
pub struct SnapshotCaptureConfig {
    pub archive_dcb: bool,           // default true, required for hydrate
    pub archive_locale: bool,        // default true
    pub archive_vehicle_xmls: bool,  // default false, placeholder for phase 2b
}
```

Adding a new file category is a non-breaking change: the format is a generic `BTreeMap<String, Vec<u8>>`, so new categories just add entries under new in-archive paths. No `SCHEMA_VERSION` bump required.

### Version history

`ExtractSnapshot::SCHEMA_VERSION` is currently **5**. History:

- **v5** (2026-04-15) — byte-bundle rework: `ExtractSnapshot { meta, files: BTreeMap<String, Vec<u8>> }`. Captured raw bytes replace the cooked `DatacoreSnapshot` + `AssetData` projection; hydration re-parses on load. Primary motivation: break the rmp_serde monomorphization cliff in sc-extract's build. Secondary benefit: historical-snapshot compatibility becomes automatic. Not backward compatible with v4.
- **v4** (2026-04-13) — rework: `ExtractSnapshot { meta, asset_data, datacore }` replaces the old `ExtractedData` god-struct. `SnapshotMeta` holds provenance.
- **v3** — `ReferenceGraph` simplified to `Guid → Vec<Guid>`; the `Edge { from, to, field_path, kind }` shape was dropped.
- **v2** — flat-pool refactor: `RecordStore` holds `DataPools` + `RecordIndex` instead of nested `Option<Box<T>>` trees.

### Schema evolution rules

With the v5 format, schema evolution operates at two levels:

1. **Envelope evolution** — adding new fields to `ExtractSnapshot` or `SnapshotMeta`. Use `#[serde(default)]` on new optional fields; no `SCHEMA_VERSION` bump. Adding new categories to `SnapshotCaptureConfig` is similarly non-breaking — the `files` map just gains entries under new keys.
2. **Payload evolution** — adding new fields to generated DCB record types. Handled entirely at hydrate time by svarog's field-by-name resolution. Old snapshots loaded against new generated types populate only the fields that exist in both versions; new fields default to their `Default` impl. No migration code, no version bump. This is what enables cross-patch historical comparison.

Incompatible envelope changes (e.g. renaming the `files` field) still require a `SCHEMA_VERSION` bump; old files fail fast with `Error::SnapshotVersionMismatch`.

## Logging

Follows the workspace `tracing` convention. Usage:

- `info!` — phase transitions in `Datacore::parse` / `AssetData::extract` (opening p4k, parsing DCB, building graph, resolving display names, serializing).
- `debug!` — per-record details, skipped records, resolution failures that are expected.
- `warn!` — recoverable issues (missing fields that should be present, unresolved references to non-existent GUIDs, etc.).
- `error!` — only for failures that terminate the operation.

No `println!`, no `eprintln!`.

## Design decisions

### svarog re-exports, not redefinitions

svarog already exposes concrete Rust types (`Value`, `InstanceRef`, `RecordRef`, `ArrayRef`, `CigGuid`, `DataCoreDatabase`, `Record`, `Instance`, `Query`, `DataType`). Defining parallel types in `sc-extract` would just cause churn when svarog changes. We re-export and rename where it adds value (`CigGuid` → `Guid` for brevity) and leave everything else alone.

### All generated types live in `sc-extract`

Every DataCore struct and enum is generated into `sc-extract/src/generated/`, not split across domain crates. The alternative — hand-maintained target config telling the generator which types go where — is manual toil that breaks on every game patch. The correct factoring is "sc-extract owns the complete DCB schema; domain crates import what they need and build hand-written wrappers around them".

### `Extract` + `Pooled` traits in `sc-extract-generated`

The generator produces `impl Extract<'a>` + `impl Pooled` for every reachable struct. The traits live in `sc-extract-generated` (the workspace-internal crate) and are re-exported from `sc-extract` for consumers who need to implement them on hand-written types. Domain crates don't normally interact with these directly — they work with already-materialized values via the typed pools in `DatacoreSnapshot::records`.

### Graph built during parse, rebuilt on hydrate

The reference graph is built exactly once per live session. Cross-session persistence happens at the snapshot layer: `capture` archives the raw DCB bytes, and `hydrate` re-runs `Datacore::parse` against them, reconstructing the graph alongside every other cooked index. An earlier revision serialized the cooked graph directly into snapshot files — the v5 byte-bundle format dropped that in favor of re-parsing, which is ~15–25 s but sidesteps the rmp_serde monomorphization cost on ~6 k generated types.

### Display names computed during parse, rebuilt on hydrate

Same reasoning. Walking `Components → SAttachableComponentParams → AttachDef → Localization → Name → locale lookup` for 50,000 entities is expensive. Compute it once per live session; snapshots re-drive the computation through hydrate.

### Tag tree structure preserved, not flattened

The DCB tag system is genuinely hierarchical (e.g. `Global.Manufacturer.Aegis`). Flattening it to a list of `(guid, name)` tuples throws away useful structure — sc-langpatch and future consumers will want `ancestors()`, `descendants()`, and `is_descendant_of()` for ship-tier filtering and category matching. The tree is small enough (~18k nodes) that preserving the structure costs nothing.

### Parse and load are first-class equal paths

Both are public and equally supported. Parsing produces a runtime [`Datacore`] (with live `DataCoreDatabase`); loading a snapshot produces a plain [`ExtractSnapshot`] with an `Option<DatacoreSnapshot>` inside. Consumers that only need the cooked data work against `DatacoreSnapshot` regardless of which side produced it — the shape is identical.

### Filter rules are predicates, not mandatory transformations

Each consumer decides whether to hide AI ship variants, placeholder weapons, or debris. The lib provides `is_playable_*` functions; it does not silently drop records during extraction.

### No user-facing error messages

Errors are structured (`thiserror` enum). Message formatting for UIs is each consumer's UX layer.

## What `sc-extract` does *not* provide

Explicit list, so the layering stays sharp:

- **Domain wrappers.** `ShipWeapon`, `FpsWeapon`, `Ammo`, `Ship`, etc. live in their domain crates.
- **Calculations.** Anything that reads weapon/ship fields and computes a derived value (DPS, sustained DPS, time-to-overheat, damage pipeline) lives in the domain or consumer crate. `sc-extract` provides raw access only.
- **Filesystem writes to the install.** `LocaleMap::serialize()` produces bytes; `sc-langpatch` writes them.
- **Stateful selection or UI logic.**
- **Tauri / Specta glue.**
- **User-facing diagnostic strings.**
- **Install discovery.** That's `sc-installs`. `sc-extract` takes `&Path` values from the caller.

## Resolved decisions

Captured here for audit trail; the rationale is baked into the relevant sections above.

- **`RecordStore` uses flat pools with generic `Handle<T>`**, not per-type HashMaps. The earlier sketch (per-type `HashMap<Guid, T>` with generated `RecordStore::entity_class()` accessors) hit rustc / linker scaling problems at ~20k generated items; moving to a single generic `Handle<T>(u32)` + blanket `impl<T: Pooled> Index<Handle<T>> for DataPools` kept compile cost linear. See `implementing/sc-generator.md` for the full story.
- **Parsing is eager.** [`Datacore::parse`] materializes every record in a single iterative pass. Lazy loading was considered and rejected — see the "Parsing model" section. The `DataCoreDatabase` is kept alive inside the returned `Datacore` (owned, not borrowed) as a raw-query escape hatch; call [`Datacore::into_snapshot`] to drop it.
- **Playable-content filters are centralized** in `sc-extract`, not in each domain crate. Avoids drift between consumers that disagree about what counts as "playable".
- **`ReferenceGraph::incoming_of_type` takes `&RecordStore` as a parameter** rather than denormalizing `type_name` into the graph. Keeps the graph compact at the cost of a per-query hashmap lookup.
- **`Error` is a `thiserror` enum** with variants covering the parse / load / snapshot / IO failure modes. Detailed variant list will be finalized when the first real error-raising code lands.
- **Vehicle XML / CryXmlB parsing lives in `sc-extract`** (when implemented). Hand-written, not generated. Currently deferred — see the Vehicle XML section.

## Out of scope for this document

- The generator itself — see `docs/codegen.md`.
- `RawAmmo` / `Ammo` wrappers — see `docs/sc-ammo.md`.
- `ShipWeapon` / `FpsWeapon` wrappers — see `docs/sc-weapons.md`.
- Non-Windows support. All consumers are Windows-only today. The lib should compile on other platforms but install discovery is inherently Windows-shaped.
