# `sc-extract` — design specification

> Status: **proposal, awaiting review.** Nothing in this document is implemented yet.

## Purpose

`sc-extract` is the foundational data-access crate for the holotable workspace. It owns the complete generated DataCore type catalog, wraps svarog with ergonomic re-exports, builds the reference graph, parses non-DCB game files (vehicle XML, `global.ini`), and exposes every cross-cutting service (tags, manufacturers, localization, display names, filters, vehicle data) that domain crates depend on.

It is the only non-`sc-installs` crate in the workspace that has svarog as a dependency. Every other crate (`sc-ammo`, `sc-weapons`, `sc-contracts`, future component crates, future `sc-ships`) depends on `sc-extract` and gets its svarog access through re-exports.

**Two data sources, one crate.** Star Citizen game data lives in two fundamentally different formats inside `Data.p4k`: the DataCore (`Game2.dcb`, ~500 MB binary object database, ~200k records) for most game data, and **CryXmlB vehicle XML files** (`data/scripts/entities/vehicles/implementations/xml/*.xml`) for ship structure, hardpoint definitions, and flight performance. sc-extract handles both — the layering is on data source, not on format.

## Consumers and what they need

| App or crate | What it needs |
|---|---|
| `sc-ammo` | Generated `AmmoParams` + related types; `FromInstance` trait; svarog re-exports for custom extraction. |
| `sc-weapons` | Generated `EntityClassDefinition`, `SCItemWeaponComponentParams`, fire-action enum; access to ammo records through the record store; `FromInstance`. |
| `sc-contracts` | Generated contract generator types; **reference graph** (for reverse lookups from tags/blueprints back to contracts); tag tree; localization. |
| `bulkhead` (future consumer) | Everything above plus playable-content filters, display-name cache, save/load snapshots. |
| `sc-langpatch` (future consumer) | Everything above plus `LocaleMap` round-trip (parse + serialize patched `global.ini`) and `sc-installs` path helpers for the write target. |

## Scope

**What `sc-extract` owns:**

- svarog re-exports and ergonomic aliases
- Generated DataCore type catalog (produced by `tools/sc-generator`, see `docs/codegen.md`)
- The `FromInstance` trait and its generated impls
- `ReferenceGraph` (built once during parse, serialized in snapshot)
- Tag tree navigation (hand-written on top of generated `Tag` records)
- Manufacturer registry (hand-written lookup on top of generated `SCItemManufacturer`)
- `LocaleMap` — `global.ini` parse + serialize (UTF-16 LE with BOM)
- **Vehicle XML** (CryXmlB) parsing — hull parts, item ports, damage multipliers, pipes, movement params. The non-DCB ship data source.
- Entity display-name resolver (pre-computed during parse, cached in snapshot)
- Playable-content filters (`is_playable_weapon`, `is_playable_ship`, etc.)
- `ExtractedData` envelope — the unified in-memory representation
- Two public entry points: `parse_from_p4k` and `load_snapshot`, plus `save_snapshot`

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

pub use svarog;   // escape hatch — full svarog namespace available as `sc_extract::svarog`

// Ergonomic aliases at the crate root:
pub use svarog::datacore::{
    Value, InstanceRef, RecordRef, ArrayRef, ArrayElementType,
    DataCoreDatabase, Record, Instance, Query, DataType,
};
pub use svarog_common::CigGuid as Guid;
```

Consumers write `use sc_extract::{Guid, Value, Instance};` for the 90% case and drop to `sc_extract::svarog::...` when they need something unusual. sc-extract does **not** redefine any of these types — they all come from svarog.

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

- All types derive `Serialize`, `Deserialize`, `Debug`, `Clone`, and `Default` where possible.
- Optional fields use `#[serde(default)]` so older snapshots still deserialize cleanly when new fields are added.
- Abstract base types (types in the DCB with multiple concrete subclasses) become polymorphic enums: `SWeaponActionParamsVariant { SWeaponActionFireRapidParams(...), SWeaponActionFireSingleParams(...), SWeaponActionSequenceParams(...), Unknown }`.
- Inheritance is **flattened** — parent-struct fields are inlined into the child struct rather than being kept as a `_parent: ParentType` field. See the rationale in `docs/codegen.md`.
- Field names are `snake_case` versions of DataCore field names. Rust keyword collisions use raw identifiers (`r#type`, `r#ref`).
- Every generated struct implements the `FromInstance` trait.

### The `FromInstance` trait

```rust
pub trait FromInstance: Sized {
    /// Construct a typed value from a svarog `Instance`.
    ///
    /// Returns `None` if the instance is missing fields the type requires.
    /// Missing optional fields produce their serde defaults.
    fn from_instance(inst: &Instance<'_>) -> Option<Self>;
}
```

Defined in `sc-extract`. Implemented by the generator for every generated struct and enum. This is the bridge between raw svarog access (zero-copy views into DCB bytes) and owned serializable values.

```rust
// Example of generated impl (not hand-written):
impl FromInstance for SCItemWeaponComponentParams {
    fn from_instance(inst: &Instance<'_>) -> Option<Self> {
        Some(Self {
            connection_params: inst.get_instance("connectionParams")
                .and_then(|i| SWeaponConnectionParams::from_instance(&i))?,
            fire_actions: inst.get_array("fireActions")
                .map(|arr| arr.filter_map(|v| /* dispatch on polymorphic variant */).collect())
                .unwrap_or_default(),
            // ... one line per field, all generated
        })
    }
}
```

Consumers rarely call `from_instance` directly — they get already-materialized values from the record store on `ExtractedData`. `FromInstance` is primarily an implementation detail of the parse path.

## The reference graph

The `ReferenceGraph` captures every cross-record reference found in the DCB during parsing. Built once, stored in the snapshot, queried at runtime for both forward and reverse lookups. This is the crate's answer to sc-langpatch's "find all contracts that reference this tag" patterns — it replaces linear scans with O(1) lookups.

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

**The graph is built exactly once**, during `parse_from_p4k`. It is serialized as part of the snapshot. Apps that `load_snapshot` get the fully-built graph; they never rebuild it.

## Record store

The snapshot holds every extracted record as a fully-materialized typed value, indexed for typed and untyped access. **All records are parsed eagerly during `parse_from_p4k`; the store never holds lazy views, raw bytes, or references back into the DCB.** See the "Parsing model" section below for the full rationale.

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

The tree is built during parse by walking `TagDatabase` records (which contain nested `Tag` records via `children` arrays) and indexed by both GUID and name. Serialized in the snapshot.

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

Built during parse by scanning all records of type `SCItemManufacturer`. Stored in the snapshot.

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

The DCB doesn't contain everything. Ships specifically have two fundamentally different data sources: the DCB holds the ship **entity** (`VehicleComponentParams`, default loadout references), while the **vehicle XML** holds the hull structure, hardpoint definitions with gimbal limits, pipe topology, and flight performance. A complete ship model requires merging both sources.

Vehicle XMLs are **CryXmlB** (binary CryEngine XML) files stored in the P4K at paths like `data/scripts/entities/vehicles/implementations/xml/aegs_gladius.xml`. The DCB's `VehicleComponentParams.vehicleDefinition` field points at the XML path. sc-extract parses these during `parse_from_p4k`, stores the results in the snapshot, and exposes them alongside DCB records.

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
/// Called by `parse_from_p4k` during extraction.
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
/// Stored as part of `ExtractedData`, computed during `parse_from_p4k`.
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

## `ExtractedData` envelope

The unified in-memory representation. Produced identically by both `parse_from_p4k` and `load_snapshot`, because the snapshot *is* a serialized `ExtractedData`.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedData {
    // ── Metadata ─────────────────────────────────────────────────────────
    /// Bumped on incompatible type or structure changes. Readers reject
    /// snapshots with unknown schema versions.
    pub schema_version: u32,
    /// Bumped on codegen logic changes that produce different output
    /// from the same DCB input.
    pub generator_version: String,
    /// Game build this snapshot was extracted from, e.g. "4.6.173.39432".
    pub game_version: String,
    /// Branch string from build_manifest.id, e.g. "sc-alpha-4.6.1".
    pub game_branch: String,
    /// Build id from build_manifest.id.
    pub build_id: String,
    /// SHA-256 of the Data.p4k file that was parsed. Used to detect
    /// whether a cached snapshot is still fresh for the current install.
    pub p4k_sha256: String,
    /// UTC ISO-8601 timestamp.
    pub extracted_at: String,

    // ── Raw record store ───────────────────────────────────────────────
    pub records: RecordStore,

    // ── Indices and registries (all built during parse) ────────────────
    pub graph: ReferenceGraph,
    pub tags: TagTree,
    pub manufacturers: ManufacturerRegistry,
    pub display_names: DisplayNameCache,

    // ── Localization ──────────────────────────────────────────────────
    pub locale: LocaleMap,

    // ── Vehicle XML data (ship structure) ─────────────────────────────
    /// Parsed vehicle XMLs keyed by ship record GUID. Only ships that
    /// have a resolvable `vehicleDefinition` path appear here.
    pub vehicle_xmls: HashMap<Guid, VehicleXml>,
}

impl ExtractedData {
    /// Total count across all record types.
    pub fn record_count(&self) -> usize;

    /// True if this snapshot's `p4k_sha256` matches the given file.
    /// Used by apps to check whether a cached snapshot is still valid.
    pub fn matches_p4k(&self, p4k_path: &Path) -> Result<bool>;
}
```

## Parsing model: eager by design

`parse_from_p4k` performs a **single, complete, eager pass** over the DataCore. Every record is read, every `FromInstance::from_instance` is called, every field is materialized into an owned Rust value. By the time `parse_from_p4k` returns, the `DataCoreDatabase` is dropped and `ExtractedData` holds every record in memory as a fully-typed, owned value.

There is no lazy loading. There are no deferred reads. No part of `ExtractedData` holds a reference to the original DCB bytes. This design choice is load-bearing and was deliberately chosen over a lazy alternative — see "Lazy was considered and rejected" below.

### Why eager

Three reasons, in order of importance:

1. **The snapshot model requires it.** `load_snapshot` reads a serialized `ExtractedData` from disk. If parsing were lazy, the deferred references would have nothing to defer *to* at load time — the DCB isn't around. Snapshots would have to embed the full DCB bytes to make lazy loading survive a reload, which defeats the entire point of having a compact cached snapshot.

2. **The cross-cutting services have to walk everything anyway.** `ReferenceGraph` needs a full pass to discover every edge. `TagTree` needs a full pass to find every tag. `DisplayNameCache` needs a full pass to resolve every entity's display name. `ManufacturerRegistry` needs a full pass. If you're walking the DCB to build these indices, you might as well materialize the records while you're there — the marginal cost per record is small.

3. **Memory is affordable.** A full eager `ExtractedData` fits comfortably in the memory budget of a modern desktop app. The one workspace consumer with tight memory constraints (streamdeck-starcitizen) doesn't use `sc-extract` at all — it only depends on `sc-installs`.

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

An earlier sketch considered keeping `DataCoreDatabase` alive inside `ExtractedData` and materializing records on demand via `FromInstance`. Four reasons it was rejected:

- **Snapshots break.** There's no serializable form of "a lazy reference into a DCB that no longer exists". Snapshots would need to embed the full 500 MB DCB to survive a reload. The msgpack + zstd pipeline assumes self-contained data.
- **Indices are eager anyway.** The graph, tag tree, display names, and manufacturers all need a full pass to build. Laziness only saves on the `RecordStore`, which is ~50% of total memory. Partial optimization for significant complexity.
- **Lifetime complexity propagates.** `ExtractedData<'db>` would poison every consumer signature down the stack. Every function that takes `ExtractedData` would need a lifetime parameter. Unworkable at the scale of three consumer apps.
- **Query-time cost.** Lazy materialization reparses bytes on every access. Death by a thousand cuts for consumers that touch many records — which all of them do.

The parse-time cost of eager loading (tens of seconds, once per game patch, paid by the `sc-generator` tool or by bulkhead's first-run flow) amortizes across every subsequent `load_snapshot` (sub-second). That's the right trade.

### `load_snapshot` is strictly deserialization

`load_snapshot` reads the file, zstd-decompresses it, and `rmp_deserialize`s the bytes into `ExtractedData`. It never touches a `Data.p4k`, never opens a DCB, never calls `FromInstance`. The path from snapshot bytes to a working `ExtractedData` has zero DCB involvement.

This is why `parse_from_p4k` and `load_snapshot` can return the same type: `ExtractedData` is fully self-contained once populated, regardless of how it was populated.

### Partial or selective parsing — explicitly out of scope

A possible future optimization is "profile-aware" parsing — e.g., bulkhead could tell the generator "I only need weapons, skip everything else" and get a smaller snapshot. This is **not** planned for the initial version and does not constrain the current design.

If it's added later, it takes the form of an optional filter applied during `parse_from_p4k` that drops records before materialization — not a lazy loading mechanism. The eager model stays.

## Entry points

Two clearly-named public functions, plus a save helper:

```rust
/// Parse Star Citizen game data directly from a `Data.p4k` archive.
///
/// This is the slow path — reads the full DCB, builds the reference graph,
/// resolves cross-references, computes display names, runs `FromInstance`
/// over every record. Typically takes tens of seconds and consumes
/// significant memory during extraction.
///
/// Use this when:
/// - No snapshot exists yet
/// - The user's installed game version has changed
/// - A one-off development / debugging tool doesn't want to manage snapshots
pub fn parse_from_p4k(p4k_path: &Path) -> Result<ExtractedData>;

/// Load pre-parsed game data from a snapshot file.
///
/// This is the fast path — reads `<path>.msgpack.zst`, zstd-decodes,
/// msgpack-deserializes. Typically takes under a second.
///
/// Returns `Err(Error::SchemaVersionMismatch)` if the snapshot's
/// `schema_version` is unknown to this build.
pub fn load_snapshot(snapshot_path: &Path) -> Result<ExtractedData>;

/// Serialize an `ExtractedData` to disk as a zstd-compressed msgpack file.
/// Writes atomically (via temp-file + rename).
pub fn save_snapshot(data: &ExtractedData, path: &Path) -> Result<()>;
```

**The snapshot IS a serialized `ExtractedData`.** There is no separate "snapshot type". `parse_from_p4k` and `load_snapshot` return the exact same shape.

## Snapshot format

On-disk: `<name>.msgpack.zst`

- **msgpack** (`rmp-serde`) for the data layer. Serde-native, compact, schema-evolvable via `#[serde(default)]`.
- **zstd** for compression. Applied at the file boundary — read = `std::fs::read → zstd_decode → rmp_deserialize`; write = `rmp_serialize → zstd_encode → atomic write`.
- Expected size: 5–30 MB depending on game patch. Fits comfortably in git if committed as a fixture for testing.

Schema evolution:

1. When types change in a compatible way (new optional field, new enum variant), `#[serde(default)]` handles it. No version bump.
2. When types change incompatibly, bump `SCHEMA_VERSION` in `sc-extract`. Readers reject unknown versions.
3. `generator_version` is bumped on codegen logic changes even when types haven't changed — for debugging, not for gating reads.

## Logging

Follows the workspace `tracing` convention. Usage:

- `info!` — phase transitions in `parse_from_p4k` (opening p4k, parsing DCB, building graph, resolving display names, serializing).
- `debug!` — per-record details, skipped records, resolution failures that are expected.
- `warn!` — recoverable issues (missing fields that should be present, unresolved references to non-existent GUIDs, etc.).
- `error!` — only for failures that terminate the operation.

No `println!`, no `eprintln!`.

## Design decisions

### svarog re-exports, not redefinitions

svarog already exposes concrete Rust types (`Value`, `InstanceRef`, `RecordRef`, `ArrayRef`, `CigGuid`, `DataCoreDatabase`, `Record`, `Instance`, `Query`, `DataType`). Defining parallel types in `sc-extract` would just cause churn when svarog changes. We re-export and rename where it adds value (`CigGuid` → `Guid` for brevity) and leave everything else alone.

### All generated types live in `sc-extract`

Every DataCore struct and enum is generated into `sc-extract/src/generated/`, not split across domain crates. The alternative — hand-maintained target config telling the generator which types go where — is manual toil that breaks on every game patch. The correct factoring is "sc-extract owns the complete DCB schema; domain crates import what they need and build hand-written wrappers around them".

### Single `FromInstance` trait, defined in `sc-extract`

The generator produces `impl FromInstance for Foo` for every generated struct. The trait itself lives in `sc-extract` because it depends on `svarog::Instance`. Domain crates don't interact with `FromInstance` directly — they work with already-materialized values from `RecordStore`.

### Graph built during parse, stored in snapshot

Apps never rebuild the reference graph on load. Cold-start cost matters, and a 40 MB graph de/serializes in well under a second via msgpack + zstd. Forcing every app to rebuild the graph from raw records would add 5–10 seconds to every startup.

### Display names computed during parse, cached in snapshot

Same reasoning. Walking `Components → SAttachableComponentParams → AttachDef → Localization → Name → locale lookup` for 50,000 entities is expensive. Compute it once, store the results.

### Tag tree structure preserved, not flattened

The DCB tag system is genuinely hierarchical (e.g. `Global.Manufacturer.Aegis`). Flattening it to a list of `(guid, name)` tuples throws away useful structure — sc-langpatch and future consumers will want `ancestors()`, `descendants()`, and `is_descendant_of()` for ship-tier filtering and category matching. The tree is small enough (~18k nodes) that preserving the structure costs nothing.

### Parse and load are first-class equal paths

Both are public, both return `ExtractedData`, both are equally supported. The generator CLI (`tools/sc-generator`) is a thin wrapper over `parse_from_p4k + save_snapshot`. Apps choose based on their workflow — bulkhead caches snapshots by game version, a one-off CLI tool might skip snapshots entirely and parse fresh every run.

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

- **`RecordStore` is per-type HashMaps**, not a single type-erased map. Each top-level DataCore record type gets its own `HashMap<Guid, T>` field, emitted by the generator. Gives zero-cost typed access via generated accessors like `RecordStore::entity_class(&guid)` and `RecordStore::tag_record(&guid)`. The `RecordView` enum covers the untyped `get(&guid)` case by pattern-matching across every generated variant.
- **Parsing is eager.** `parse_from_p4k` materializes every record in a single pass and drops the `DataCoreDatabase` before returning. Lazy loading was considered and rejected — see the "Parsing model" section.
- **Playable-content filters are centralized** in `sc-extract`, not in each domain crate. Avoids drift between consumers that disagree about what counts as "playable".
- **`ReferenceGraph::incoming_of_type` takes `&RecordStore` as a parameter** rather than denormalizing `type_name` into the graph. Keeps the graph compact at the cost of a per-query hashmap lookup.
- **`Error` is a `thiserror` enum** with variants covering the parse / load / snapshot / IO failure modes. Detailed variant list will be finalized when the first real error-raising code lands.
- **Vehicle XML / CryXmlB parsing lives in `sc-extract`.** Hand-written module at `sc-extract/src/vehicle_xml/`, not generated. Full section above.

## Out of scope for this document

- The generator itself — see `docs/codegen.md`.
- `RawAmmo` / `Ammo` wrappers — see `docs/sc-ammo.md`.
- `ShipWeapon` / `FpsWeapon` wrappers — see `docs/sc-weapons.md`.
- Non-Windows support. All consumers are Windows-only today. The lib should compile on other platforms but install discovery is inherently Windows-shaped.
