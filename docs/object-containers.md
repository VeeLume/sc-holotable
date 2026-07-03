# object-containers — unifying the socpak placement walk

Status: **spec / not yet implemented.** Consolidates the two (soon three)
parallel socpak scanners in `sc-locations` into one placement-graph walk, and
adds **positions**. Motivated by the sc-cargo-planner routing work (see that
repo's `docs/routing-and-position-data.md` for the investigation that produced
the findings below).

## The problem — parallel walks over the same files

`sc-locations` has grown three overlapping passes over the object-container
socpaks, each re-opening + re-decoding the same members and re-implementing the
GUID/`norm`/entity-iteration helpers:

| Pass | Built for | Scope | Reads | Produces | Positions? |
|---|---|---|---|---|---|
| `containers.rs` `LocationContainers` | resource-gathering (Heart) | `*system.socpak` | `.entxml` | `StarMapObject ↔ socpak` | no |
| `mission_crc.rs` `MissionCrcCatalog` | sc-cargo-planner | all `objectcontainers/*.socpak` | `.soc/.pla/.entxml` | containment chain → `crc32` → name | no |
| *(spike)* positions | route advisor | (both) | + system `.xml` | entity `Pos` | yes |

`containers.rs` is also **broken on 4.8** (it looks for the pre-4.8
`SNavPointObjectMetadataParams`; the component was renamed `SObjectMetadataParams`).

**Insight:** `mission_crc.rs`'s `Ent` graph + `walk` is already the general
object-container **placement graph**. `LocationContainers` (name↔socpak), the CRC
catalog (chain→crc→name), and positions are all **projections of one graph**.

## Measured costs (live 4.8) — one walk is free to unify

9,526 `objectcontainers/*.socpak`, ~838k entities:

| Phase | Cost |
|---|---|
| parse (read + zstd + CryXmlB decode + harvest) | **121.9 s** |
| index (key maps) | 0.58 s |
| chain-walk + `crc32` (984 chains) | **0.007 s** |

Parsing is the entire cost; CRC is free. **One full walk emitting positions *and*
the CRC catalog costs the same as the CRC catalog alone.** This is the
multi-minute cook the `ProcessedSnapshot` already amortizes.

## The data model — placement is primary, identity is layered

The single most important correctness rule (learned the hard way — see the audit
in the cargo-planner doc):

> **Do not gate on `starmapRecord`.** The primary record is the **placement
> entity** (its `Pos` + `EntityCryGUID` graph identity). `starmapRecord`, a
> mission template, and entity `Name` are **optional identity facets**. Real
> objects exist with none of them beyond a position — unnamed outposts,
> mission-objective points (the oxygen dispensers), Benny's Henge.

A resolved *star-map display name* ≈ "the object has a star-map marker." Its
absence does **not** mean the object isn't real.

**Two independent lookup keys into the one graph:**
- **mission DataSet CRC** — `crc32_ieee(",".join(EntityCryGUID chain))`; needs no
  `StarMapObject` (this is how mission-objective leaves resolve).
- **`starmapRecord` GUID** — for star-map / named places (`class_crc` → `Locations`).

The `EntityCryGUID` placement is the node; both are indexes onto it.

### What live 4.8 actually looks like (harvest must be tolerant)

Verified across Stanton + Pyro + Nyx:

- **Entity classes carrying placements:** `OrbitingObjectContainer` (bodies,
  stations), `RastarLocationEntity` (outposts), `LocationObjectContainer`
  (Lagrange rest-stops), `ObjectContainerModifier` (asteroid clusters) — accept
  **any** class.
- **`starmapRecord` lives on varying nodes:** `SObjectMetadataParams` (most),
  `<Elem>` (e.g. MIC-L1 Shallow Frontier Station), `<Child>` (nav-graph, in
  system `.xml` members) — search **any** descendant (`first_attr`-style), never
  just `.next()`.
- **`Pos` scale differs by level:** system-OC placements (planets/moons/stations
  in `*system.socpak`) are **system-global**; sub-body placements
  (`RastarLocationEntity` outposts, rest-stops) are **body-local** — compose with
  the parent body's global `Pos` for a global coordinate. For *ordering* (the
  route advisor), the parent body's position suffices.
- **Orbit:** `EntityComponentOrbit` → `OrbitalRadius` / `OrbitalAngle` /
  `parentGUID` (GUIDs are brace-wrapped, `.NET` order; `EntityCryGUID` is CryGUID
  order — canonicalize both to storage bytes, as `mission_crc.rs` does).
- **`.xml` members** hold the system nav-graph `<Child>` nodes; decode them for
  `*system.socpak` only (decoding every socpak's `.xml` explodes parse time).

### Coverage after generalizing (Stanton, no asteroids)

550/622 positioned — **Planets 5/5, Moons 12/12**, all landing zones + mission
hauling stations. The remainder is legitimately non-static: dev placeholders,
dynamic markers (beacons/objectives), removed stations (INS Jericho, Cry-Astro),
duplicate legacy records (`PYAM-FARSTAT-*` — the live station is `Checkmate`),
and Nyx-not-yet-built. Pyro bodies: 6/6 planets, 6/6 moons.

## Proposed module — `object_containers.rs`

One walk → one graph → thin projections. Keep the existing public types
(`LocationContainers`, `MissionCrcCatalog`) so consumers don't break; add
`ContainerBinding` for positions.

```rust
/// One placed object-container entity — the primary record.
pub struct Placement {
    pub cry: Key,                    // EntityCryGUID (storage bytes) — graph identity
    pub parent: Option<Key>,         // parentGUID (storage bytes)
    pub socpak: String,              // normalized placement socpak
    pub nests: Option<String>,       // objectContainer (socpak nesting edge)
    pub pos: Option<[f64; 3]>,       // entity Pos (system-global or body-local)
    pub orbit: Option<Orbit>,        // OrbitalRadius / OrbitalAngle
    pub name: Option<String>,        // entity Name (raw identity fallback)
    pub starmap: Option<Guid>,       // starmapRecord (optional facet)
    pub mission_leaf: bool,          // has template / MissionLocationParams
}

pub struct ObjectContainers { /* placements + (socpak,cry)/cry/nest indices */ }

impl ObjectContainers {
    pub const COOK_SCHEMA_VERSION: u32 = 1;
    /// One full walk over objectcontainers/*.socpak (+ system .xml).
    pub fn cook(assets: &AssetSource) -> Result<Self>;
    /// Containment chain (root→leaf) of EntityCryGUIDs for a placement.
    pub fn chain(&self, placement: usize) -> Vec<usize>;

    // ── projections ───────────────────────────────────────────────────
    pub fn mission_crc_catalog(&self, locs: &Locations) -> MissionCrcCatalog;
    pub fn location_containers(&self) -> LocationContainers;
    pub fn bindings(&self, locs: &Locations) -> ContainerBindings;
}

/// starmapRecord GUID → position + parent body + orbit. The route advisor's input.
pub struct ContainerBindings { /* guid → ContainerBinding */ }
pub struct ContainerBinding {
    pub pos: Option<[f64; 3]>,       // as-authored (may be body-local)
    pub parent_body: Option<Guid>,   // nearest positioned body (for ordering + global compose)
    pub orbit: Option<Orbit>,
}
```

`MissionCrcCatalog` / `LocationContainers` keep their current `cook(assets, …)`
entry points as thin wrappers that build `ObjectContainers` then project — or
consumers switch to cooking `ObjectContainers` once and projecting N ways.

## Persistence

One cook feeds all products. Options:
- **(preferred)** persist `ObjectContainers` in its own `ProcessedSnapshot`;
  consumers project on load (projection is cheap — the 122 s was the parse).
- or persist each projection separately from one in-memory cook.

sc-cargo-planner's `HolotableSnapshot` bundles `crc_catalog` today; it would
instead persist the `ObjectContainers` graph and `join` it with the DCB-rebuilt
`Locations` into a `Universe` on load — `by_mission_crc` + `global_position`
replace the `crc_catalog` field. `sc-gathering` cooks `ObjectContainers` at
p4k-time and projects `ProviderLocations` (unchanged shape).

**Scope note:** the unified walk is all-socpaks (~122 s), where the old
`LocationContainers` cooked only `*system.socpak`. Since consumers live behind a
cached snapshot, that's amortized; if a consumer's cold-cook time regresses, add a
`Scope::SystemOnly` fast path later — split the socpak enumeration, not the code.

## Regression oracles

- `examples/crc_catalog.rs` — resolves the live gRPC mission CRCs (ground-truth
  spot check: `2273524489 → ARC-L3 Modern Express Station`; MIC-L1 now resolves).
- `examples/system_tree.rs --pos` — ≥ 550/622 Stanton; Pyro bodies 6/6 + 6/6.
- `examples/universe_smoke.rs` — the joined `Universe` end-to-end (by_name /
  by_mission_crc / global_position, verified: Everus Harbor 1149 km from Hurston).

## Phasing

1. ✅ **Extract `object_containers.rs`** — `Placement` model, one `cook`, graph
   indices + `chain` walk, shared GUID/`norm`/`crc32` helpers, tolerant harvest
   (any class, tag-agnostic `starmapRecord`, no gating).
2. ✅ **`Universe`/`Place`** — the join, direct facets + derived helpers,
   `global_position` composition. `MissionCrcCatalog` reprojected onto the graph.
3. ✅ **Retire the old projections.**
   - `LocationContainers` removed; `sc-gathering` migrated to
     `ObjectContainers::realized_socpaks` / `locations_in`.
   - `MissionCrcCatalog` / `MissionLocation` removed; sc-cargo-planner's
     `HolotableSnapshot` now persists `ObjectContainers` + `Locations`, `join`s a
     `Universe` on load, and resolves via `Universe::by_mission_crc`. `COOK_VERSION`
     bumped (one-time re-cook). `ObjectContainers` is now `Serialize`/`Deserialize`.
4. ⏭️ **Route advisor.** `route.rs` (in sc-cargo-planner) orders drop-offs by
   `Place::global_position`. `Scope::SystemOnly` fast-path only if a consumer's
   cold-cook time regresses.

## Caveats

- **Name fallback, not name requirement** (see data model) — objects with a blank
  `StarMapObject` name or none at all are still valid placements; surface entity
  `Name` and let the consumer decide.
- **Duplicate `EntityCryGUID`s** across cloned stations — key placements by
  `(socpak, cry)` and anchor the chain walk per-placement (as `mission_crc.rs`
  already does), never by bare `cry`.
- **Nyx is partial** — some placements exist (QV Breaker Stations), others don't
  (jump points). Not an error; report, don't assume.
