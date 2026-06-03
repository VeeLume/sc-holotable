# `sc-locations` — design specification

> Status: **implemented.** Core + identity surface, typed `LocationKind`,
> class-CRC resolution, hierarchy. 5 lib tests; wired into `Foundations` /
> `HolotableSnapshot` (cook v3). Grounded in a data exploration against LIVE
> (`StarMapObject` / `StarMapObjectType` records under
> `libs/foundry/records/starmap/pu/…`): 2,054 locations, 21 distinct kinds, 0
> untyped. The CRC bridge is verified byte-exact: `StarMapObject.Nyx_Levski`
> (`468d4102-a210-47b5-8bc3-084f791a173c`) → `class_crc` `3723364946`, the
> location `subject_id` seen on the EntityGraph gRPC wire.

## Purpose

`sc-locations` is a hand-written domain wrapper over `StarMapObject` records —
the universe's stable *places*: solar systems, planets, moons, landing zones,
rest stops, stations, outposts, mining claims. It materializes them into
ergonomic owned [`Location`] structs (localized name, typed category, hierarchy)
that consumers use without threading `&DataPools` through every call, and
indexes them by GUID **and by class-CRC** so an EntityGraph wire `subject_id`
resolves straight to a typed location.

It is the canonical example of the "record type with no typed domain crate"
bucket getting one: `StarMapObject` records are not items, so they don't resolve
through `sc-items`'s catalog — they need their own surface. The generic
`sc_extract::CrcIndex` already resolves *any* record CRC to a GUID + record
name; `sc-locations` is the typed upgrade for the location domain specifically
(localized name, category enum, hierarchy), the same way `sc-items` is the typed
upgrade for items.

## Consumers

| Crate / app | What it uses |
|---|---|
| `sc-dossier` | Resolve location `subject_id` CRCs → name + category. **Drives correctness.** |
| (future) mission/contract tooling | Location display names + hierarchy for "where" enrichment. Reads a subset. |

## Scope

**What `sc-locations` v1 owns:**

- [`Location`] — materialized struct for one `StarMapObject` (2,004 records in
  4.7). Fields are the **core + identity** set (see below).
- [`LocationKind`] — typed category enum resolved from the location's
  `StarMapObjectType.name`, with an `Unrecognized(String)` fallback for
  forward-compat. Mirrors `EItemType`.
- [`Locations`] — index of every location by GUID, by class-CRC, and a
  parent→children adjacency for hierarchy traversal.
- Class-CRC resolution: `Locations::by_crc(crc) -> Option<&Location>` /
  `guid_by_crc`. The typed counterpart to `CrcIndex` (returns `&Location`).
- Hierarchy: `parent_of` / `children_of` / `ancestors` (up to the system root).

**What `sc-locations` v1 does NOT own (deferred — the expansion point):**

- `amenities` (the `Vec<Guid>` into `StarMapAmenityTypeEntry`), quantum-travel
  data, radar/signature params, location images, audio triggers.
- `StarMapMissionObject` / `StarMapPartyMemberObject` — dynamic runtime markers,
  not stable places.
- Localized **category** label resolution beyond the raw `classification`
  LocaleKey (consumers resolve it through `LocaleMap` themselves if needed).
- Any geometry/render fields (`starMapGeomPath`, `navIcon` rendering, …) beyond
  the `nav_icon` enum carried for identification.

## Data model (verified against 4.7 LIVE)

`StarMapObject` is a seeded record type in `sc-extract-generated`'s
`multi_feature` module, gated by the **`starmap`** feature. Its typed fields
(already decoded by the generator):

| `StarMapObject` field | Type | Use |
|---|---|---|
| `name` | `LocaleKey` | `@ui_pregame_port_Levski_name` → "Levski" |
| `description` | `LocaleKey` | long-form |
| `callout1` / `callout2` / `callout3` | `LocaleKey` | short callout lines |
| `r#type` | `Option<CigGuid>` | → `StarMapObjectType` (category) |
| `parent` | `Option<CigGuid>` | → parent `StarMapObject` (hierarchy) |
| `nav_icon` | `NavPointIconEnum` | typed icon class (LandingZone, …) |
| `respawn_location_type` | `ERespawnLocationType` | respawn classification |
| `jurisdiction` | `Option<CigGuid>` | → law-system jurisdiction record |

`StarMapObjectType` is **also** a seeded record type under the same `starmap`
gate. Its `name` is a plain `String` category identifier ("SolarSystem",
"LandingZone", "RestStop", …) and `classification` is a `LocaleKey` localized
label. The full type set lives only in the DCB (the `StarMapObjectType` records
are sub-records, not present in the extracted XML), so it must be enumerated by
a one-time dump at implementation time (phase 0).

Hierarchy is a `parent` reference chain: `Nyx_Levski → NyxStar`, walking up to
the `SolarSystem` root. `parent` points at another `StarMapObject`, so the
adjacency is closed within the location set.

## Public surface

```rust
/// One universe location, materialized from a StarMapObject record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub guid: Guid,
    /// `name` — primary display-name key (`@…`, raw; resolve at call site).
    pub name_key: Option<LocaleKey>,
    /// `description`.
    pub desc_key: Option<LocaleKey>,
    /// `callout1..3`, empties dropped.
    pub callouts: Vec<LocaleKey>,
    /// Typed category, resolved from `type → StarMapObjectType.name`.
    pub kind: LocationKind,
    /// `navIcon` — typed icon class (identification aid). The generated enum,
    /// stored directly; serde via a dcb-str adapter (like sc-items' EItemType).
    pub nav_icon: NavPointIconEnum,
    /// `respawnLocationType` — the generated enum, stored directly.
    pub respawn: ERespawnLocationType,
    /// `parent` — hierarchy parent (another Location), if any.
    pub parent: Option<Guid>,
    /// `jurisdiction` — law-system record GUID (bare reference, not followed).
    pub jurisdiction: Option<Guid>,
}

impl Location {
    /// Resolve the display name through a LocaleMap (None if empty/missing).
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str>;
}

/// Typed location category, from StarMapObjectType.name. The 21 known variants
/// are the type records observed in live DCB data; unknown (a future patch's
/// type) → Unrecognized.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationKind {
    Anomaly, Asteroid, AsteroidValidQt, CardinalPoint, JumpPoint, LandingZone,
    Manmade, ManmadeJumpPoint, ManmadeVisibleOnInteraction, Moon, NavPoint,
    Outpost, OutpostInvalidQt, Planet, PointOfInterest, QuantumTracePoint,
    S42Moon, S42Planet, SolarSystem, Star, YouAreHere,
    Unrecognized(String),
}
impl LocationKind {
    pub fn from_dcb_str(s: &str) -> Self;   // exact StarMapObjectType.name → variant
    pub fn as_dcb_str(&self) -> &str;
}
```

The live kind distribution (LIVE): Outpost 1006, Asteroid 680,
Asteroid_ValidQT 131, Manmade 99, Manmade_VisibleOnInteraction 42, Anomaly 19,
Moon 19, Planet 14, PointOfInterest 14, LandingZone 7, Outpost_InvalidQT 7,
NavPoint 4, SolarSystem 4, Star 3, JumpPoint 2, CardinalPoint 1,
QuantumTracePoint 1, YouAreHere 1. Note the *category* is the StarMapObjectType,
**not** the nav icon: Levski's kind is `Manmade` (a Delamar-asteroid station)
while its nav icon is `LandingZone`; planetary cities (Area18, Lorville, Orison)
are kind `LandingZone`.

```rust

/// Every location, indexed by GUID + class-CRC, with hierarchy adjacency.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Locations { /* by_guid; derived: by_crc, children */ }

impl Locations {
    /// Build by walking the typed StarMapObject pool. `locale` is not needed at
    /// build (keys stay raw); category resolution reads StarMapObjectType from
    /// the same store.
    pub fn build(store: &RecordStore) -> Self;

    pub fn get(&self, guid: &Guid) -> Option<&Location>;

    // ── class-CRC resolution (the sc-dossier entry point) ──
    pub fn by_crc(&self, crc: u32) -> Option<&Location>;
    pub fn guid_by_crc(&self, crc: u32) -> Option<Guid>;

    // ── hierarchy ──
    pub fn parent_of(&self, guid: &Guid) -> Option<&Location>;
    pub fn children_of(&self, guid: &Guid) -> impl Iterator<Item = &Location> + '_;
    pub fn ancestors(&self, guid: &Guid) -> impl Iterator<Item = &Location> + '_;

    pub fn iter(&self) -> impl Iterator<Item = (&Guid, &Location)> + '_;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

/// RecordVisitor variant for fusing the location walk into a bundled pass
/// (mirrors sc-items' ItemsBuilder).
pub struct LocationsBuilder { /* … */ }
```

## Mechanics

- **Class-CRC**: reuse `sc_extract::class_crc`. Build a scoped
  `by_crc: HashMap<u32, Guid>` over `StarMapObject` GUIDs only (same pattern as
  `Items::by_crc`, collision-logged via `warn!`). `by_crc` returns `&Location`,
  which the generic `CrcIndex` cannot — that's why the typed crate carries its
  own index rather than delegating.
- **Category resolution**: follow the `type` reference with
  `Datacore::resolve::<StarMapObjectType>(&guid)` (or the store's typed lookup),
  read `.name`, map through `LocationKind::from_dcb_str`. No string-matching on
  paths or display names — the category is typed data (design principle 5).
- **Enums as DCB strings in serde**: `LocationKind`, `NavPointIcon`,
  `RespawnLocationType` serialize via the same `enum_serde` adapter shape
  `sc-items` uses (generated enums are serde-free).
- **Derived indices not serialized**: `by_crc` and the `children` adjacency are
  rebuilt from `by_guid` on deserialize via a `Repr` shadow
  (`#[serde(from/into)]`), exactly like `Items::by_crc`. Zero snapshot bloat.
- **Feature closure**: `sc-extract = { features = ["starmap"] }` — pulls both
  `StarMapObject` and `StarMapObjectType` (and their closures) into the build.

## Crate layout (mirrors `sc-items`)

```
crates/sc-locations/
  Cargo.toml                  # sc-extract[starmap], serde, tracing; dev: sc-discovery, tracing-subscriber
  src/lib.rs                  # Location, LocationKind, NavPointIcon, RespawnLocationType, Locations, LocationsBuilder
  examples/location_dump.rs   # smoke test: resolve a CRC, print name + kind + ancestor chain
docs/sc-locations.md          # this spec
```

## Integration

- `Foundations` (`sc-holotable`): add a `locations: Locations` field, built in
  the bundled walk alongside `items` / `tags` / `paths`.
- `HolotableSnapshot`: add `locations: Option<Locations>`; bump
  `HOLOTABLE_COOK_VERSION` 2 → 3.
- `sc-holotable` prelude + `crates`-glob: re-export `Location`, `LocationKind`,
  `Locations`.
- `CHANGELOG.md` `[Unreleased]`: note the new crate (public-surface change).

## Phasing (all complete)

0. ✅ **Dumped the `StarMapObjectType` catalog** — 21 distinct names, 0 untyped
   across 2,054 records. Populated `LocationKind`'s variant list.
1. ✅ **Core crate.** `Location` (core + identity fields), `Locations` with GUID
   + CRC indices, serde round-trip via `Repr` shadow. `by_crc(3_723_364_946)` →
   "Levski", kind `Manmade` (the live `StarMapObjectType`; its *nav icon* is
   `LandingZone` — the crate surfaces the real typed category, not the icon).
2. ✅ **Category + hierarchy.** `LocationKind` resolved from `StarMapObjectType`;
   `parent_of` / `children_of` / `ancestors` (cycle-guarded). `ancestors(Area18)`
   → ArcCorp → Stanton.
3. ✅ **Wiring.** `Foundations` + `HolotableSnapshot` (cook v3) + prelude
   re-exports + CHANGELOG entry.

## Caveats (carried forward)

- **`ancestors` cycle guard**: `parent` is a strict tree in live data, but the
  walk caps at the first repeat defensively (a malformed patch can't hang a
  consumer).
- **Record name vs. localized name**: `Location` carries the localized
  `name_key` (→ "Levski"). The *internal* record name ("Nyx_Levski") is not a
  `StarMapObject` field — it's DCB record metadata, reachable via `RecordPaths`
  if a consumer wants the stable internal id. v1 does not duplicate it into
  `Location`; add a `record_name` field later if a consumer needs it.
- **Deferred fields** (the expansion point): `amenities`, quantum-travel data,
  radar/signature params, location images/audio, and resolved category labels.
