# Ship cargo grids — data-source map (for `sc-cargo-planner`)

> Status: **investigated; Q1 + Q2 proven end-to-end, Q3 + Q4 characterized with
> the frontier identified.** This is the data-source spec for the "ship cargo
> grid" domain — where cargo can be placed, how much, where the grids sit inside
> the hull, and what bounds/opens them. Grounded on **LIVE (SC 4.8)**: the live
> `Game2.dcb` parsed via `sc-extract`, plus the MISC Freelancer's expanded
> interior object containers. The SCU arithmetic reproduces every Freelancer
> variant's published capacity **exactly** (66 / 120 / 36 SCU).
>
> Reproduce with `crates/sc-extract/examples/cargo_grid_probe.rs` (see
> [Reproduce](#reproduce)).

## TL;DR

A cargo grid is **not** one record — it is a small chain that straddles the two
data layers this workspace already knows (`docs/dcb-exploration-guide.md`):

- **The grid itself (size, cells, capacity, what fits) is fully in the DCB**, as
  a seeded **`InventoryContainer`** record whose `interiorDimensions` (metres) ÷
  the 1.25 m cell = the SCU cell grid. This is a clean, typed, GUID-resolvable
  read.
- **Which grids a ship has, and at which hardpoints, is in the DCB** — the ship
  entity's manual loadout (`SItemPortLoadoutEntryParams`: `itemPortName` →
  `entityClassName`).
- **Where each grid sits in 3-D is *not* in the DCB.** The loadout only names an
  item port (`hardpoint_cargogrid_rear`); the port's transform lives in the
  ship's geometry (a CGA helper named after the port). Same for room walls.
- **Room topology and openings are in the object-container (`.socpak`) layer** —
  room entities + a precomputed room-connectivity graph (`.rmxml`) + door/portal
  entities whose *connector* component carries an aperture bounds rectangle.

`SCItemCargoGridParams` — the type whose name looks like the answer — has **zero
live instances** (correctly classified `dormant`). It is a legacy/mining-crate
component, not how ship cargo grids are modelled. Don't build on it.

## The chain

```
EntityClassDefinition (ship, e.g. MISC_Freelancer)
  └─ SEntityComponentDefaultLoadoutParams → SItemPortLoadoutManualParams
       └─ entries[]  : SItemPortLoadoutEntryParams
            itemPortName   = "hardpoint_cargogrid_rear"   ── names a geometry helper (transform lives in the CGA)
            entityClassName = "MISC_Freelancer_CargoGrid_Rear"
                              │
                              ▼
       EntityClassDefinition (the cargo-grid ITEM, its own entity class)
         └─ SCItemInventoryContainerComponentParams
              containerParams = <Reference GUID> ──────────────┐
                                                                ▼
              InventoryContainer  (seeded record — Datacore::resolve::<InventoryContainer>)
                interiorDimensions : Vec3   (metres)   ► ÷1.25 per axis = SCU cell grid
                inventoryType      : InventoryOpenContainerType   (open = cargo grid)
                    gridCellSize        = 1 cm quantum
                    minPermittedItemSize= (1.25,1.25,1.25)   (1 SCU footprint)
                    maxPermittedItemSize= (2.5,10,2.5)       (biggest item that fits)
                    gridPosOffset       = Vec3               (local nudge within the port)
```

**Mode is data, not strings:** a cargo grid is an `InventoryContainer` whose
`inventoryType` polymorphic subclass is **`InventoryOpenContainerType`** (vs
`InventoryClosedContainerType` for sealed personal inventory). No name matching.

## The four questions

### 1. How is a cargo grid defined? — `InventoryContainer` (DCB, proven)

The grid item's `SCItemInventoryContainerComponentParams.containerParams`
([multi_feature/types.rs:624197](../crates/sc-extract-generated/src/generated/multi_feature/types.rs#L624197))
is a `Reference` to a seeded **`InventoryContainer`** record
([multi_feature/types.rs:461402](../crates/sc-extract-generated/src/generated/multi_feature/types.rs#L461402)):

| Field | Type | Meaning |
|---|---|---|
| `interiorDimensions` | `Vec3` (m) | the grid box; **÷ 1.25 m per axis = SCU cell counts** |
| `inventoryType` | poly base | `InventoryOpenContainerType` ⇒ cargo grid |
| `inventoryType.gridCellSize` | cm | placement quantum (1 cm) |
| `inventoryType.minPermittedItemSize` | `Vec3` | smallest slot = (1.25,1.25,1.25) = 1 SCU |
| `inventoryType.maxPermittedItemSize` | `Vec3` | largest item the grid accepts |
| `inventoryType.gridPosOffset` | `Vec3` | grid origin offset within the port |

`InventoryOpenContainerType` is at
[multi_feature/types.rs:460683](../crates/sc-extract-generated/src/generated/multi_feature/types.rs#L460683).

**SCU capacity is derived, not stored.** 1 SCU occupies a 1.25 m cube, so
`capacity = ∏ round(dim_axis / 1.25)`. Verified against live records:

| Grid (`InventoryContainer.*`) | `interiorDimensions` (m) | cells (÷1.25) | SCU |
|---|---|---|--:|
| `MISC_Freelancer_CargoGrid_Rear` | 2.5 × 11.25 × 3.75 | 2 × 9 × 3 | 54 |
| `MISC_Freelancer_CargoGrid_Mid` | 2.5 × 1.25 × 3.75 | 2 × 1 × 3 | 6 |
| `MISC_Freelancer_CargoGrid_MAX_Rear` | 5.0 × 11.25 × 3.75 | 4 × 9 × 3 | 108 |
| `MISC_Freelancer_CargoGrid_DUR_Rear` | 2.5 × 5.0 × 3.75 | 2 × 4 × 3 | 24 |

### 2. Placement inside the ship — split: *which* is DCB, *where* is geometry

The ship's **manual loadout** lists each mounted grid and its item port
(`SItemPortLoadoutEntryParams`,
[multi_feature/types.rs:330771](../crates/sc-extract-generated/src/generated/multi_feature/types.rs#L330771)).
Live, for `MISC_Freelancer`:

```
port 'hardpoint_cargogrid_rear'      -> MISC_Freelancer_CargoGrid_Rear   (54)
port 'hardpoint_cargogrid_mid_left'  -> MISC_Freelancer_CargoGrid_Mid    ( 6)
port 'hardpoint_cargogrid_mid_right' -> MISC_Freelancer_CargoGrid_Mid    ( 6)
                                                                    total  66 SCU ✓
```

Cross-variant (same run) — the loadout is the *only* thing that changes, and each
sum matches the published number exactly:

| Variant | rear grid | + 2× mid | total | published |
|---|---|--:|--:|--:|
| `MISC_Freelancer` | Rear (54) | 12 | **66** | 66 |
| `MISC_Freelancer_MAX` | Rear_MAX (108) | 12 | **120** | 120 |
| `MISC_Freelancer_DUR` | Rear_DUR (24) | 12 | **36** | 36 |
| `MISC_Freelancer_MIS` | Rear_DUR (24) | 12 | **36** | 36 |

**The 3-D transform of a grid is not in the DCB.** `SItemPortDef`
([multi_feature/types.rs:470577](../crates/sc-extract-generated/src/generated/multi_feature/types.rs#L470577))
carries `Name`, `defaultItem`, `MinSize`/`MaxSize`, `gridBehavior`,
pitch/yaw/roll limits — but **no position**. The port is bound to a **named
helper in the ship's geometry** (`hardpoint_cargogrid_rear`), and the helper's
transform lives in the ship's `.cga`. So absolute grid placement =
`helper_transform ∘ gridPosOffset`, and `helper_transform` is a **geometry-layer
read** (CGA hardpoint helpers), *not* DCB. This is the same helper-name → CGA
mechanism every item port (weapons, components) uses.

### 3. Wall (blocked) vs free walkable space — room system; volumes are geometry

The interior object containers carry a **room system**, but with an important
split:

- **Room topology is data.** Each interior `.socpak` ships a `.rmxml`
  **`RoomMapping`** — a precomputed room-to-room graph: for every room
  (`SuperGUID`) it lists reachable rooms with a `Distance` and a
  `NextRoomSuperGUID` next-hop; `Distance="3.4028235e+38"` (FLT_MAX) = not
  reachable. This tells you which rooms connect, and through which connector.
- **Rooms carry no bounding volume in the DCB.** `SEntityComponentRoomParams`
  ([multi_feature](../crates/sc-extract-generated/src/generated/multi_feature/types.rs))
  has `roomType`, `roomName`, `isPhysical`, `roomExtensions` — but **no box /
  extent**. A room's spatial volume (its walls) is the **collision/room geometry**
  placed in the `.soc` (e.g. `misc_freelancer_cargo.cgf`,
  `misc_hull_wall_*_components_door_*.cga`) at the room entity's transform.

**Consequence for the planner:** "does grid cell (x,y,z) face a wall or open
space?" is **not a stored boolean.** What *is* derivable without touching mesh
geometry: the grid box (Q1) placed at its port (Q2), and which **room** it lives
in. Deciding wall-vs-open per grid face needs the room's collision hull — a
geometry-layer computation (parse the room/hull `.cga` collision, or the baked
room-volume in the `.soc`). That is the frontier for Q3; the DCB + `.rmxml` alone
give topology, not per-face occupancy.

### 4. Openings (ramps / doors / hatches) — door + connector entities (derivable)

Openings are **entities in the interior `.socpak`**, and unlike room walls, the
opening's rectangle *is* carried in data:

- **Doors** are `SCItemDoor` entities (`SCItemDoorParams`,
  `SCItemDoorSingleProceduralParams`, `SCItemDoorAnimationParams`) placed at
  named ports — e.g. `Port9_Door_RN_NoRoomConnector`, and the cargo-bay door
  (`probe_freelancer_cargo_door_fade`, mesh `misc_door_a.cga`).
- **Portals** connect rooms: `portal_rear`, `portal_rear_top` entities; these are
  the graph edges the `.rmxml` references.
- **The aperture bounds are stored.** `SEntityComponentRoomConnectorParams`
  ([multi_feature](../crates/sc-extract-generated/src/generated/multi_feature/types.rs))
  carries **`defaultAreaBounds`** + `boundsOffset` (the opening rectangle),
  plus `orientationMode`, `apertureAnimateTime` (door animation), and
  `audioSoundProofing`. So the opening's position/size and the two rooms it joins
  are recoverable from the OC entity tree, without mesh work.

Ramps and the big cargo-bay doors are just door/animated entities with the same
shape — identify them by their room-connector + door components and the cargo
mesh they animate (`misc_freelancer_cargo.cgf`). Distinguishing "external cargo
opening" from "internal doorway" is a connector-type / room-pair question
(does it connect an interior room to the exterior room?), not a name match.

## DCB vs other files — where each answer lives

| Question | Layer | Concrete source |
|---|---|---|
| Grid size / cells / capacity / what fits | **DCB** | `InventoryContainer` record (via grid item's `containerParams`) |
| Which grids a ship has + their ports | **DCB** | ship `SItemPortLoadoutEntryParams` (`itemPortName`, `entityClassName`) |
| Grid 3-D transform | **Geometry** | CGA helper named after the item port (`hardpoint_cargogrid_*`) |
| Room topology (which rooms connect) | **OC (`.socpak`)** | `<name>.rmxml` `RoomMapping` graph |
| Room membership of a grid | **OC (`.socpak`)** | `SEntityComponentRoomParams` entity + transform in `.soc` |
| Room wall volume (per-face blocked/open) | **Geometry** | room/hull collision `.cga` / baked room volume in `.soc` — *frontier* |
| Doors / ramps / hatches + aperture bounds | **OC (`.socpak`)** | `SCItemDoor*` + `SEntityComponentRoomConnectorParams.defaultAreaBounds` |

Ship interior OCs live at `Data\ObjectContainers\Ships\<MFR>\<Ship>\*.socpak`
(e.g. `.../MISC/Freelancer/base_int_back_main.socpak`), each expanding to a plain
`ObjectContainer` `.xml` (bounds + static-entity tags), a binary `.soc` (CrChF →
CryXmlB entity tree — decode with `sc_extract::object_container`), and the
`.rmxml` room graph.

## Record / component reference

Generated bindings, all under `crates/sc-extract-generated/src/generated/`:

| Type | Location | Role |
|---|---|---|
| `InventoryContainer` (seeded record) | `multi_feature/types.rs:461402` | **the grid definition** — `interiorDimensions`, `inventoryType` |
| `InventoryOpenContainerType` | `multi_feature/types.rs:460683` | cargo-grid variant — cell size, min/max item, `gridPosOffset` |
| `SCItemInventoryContainerComponentParams` | `multi_feature/types.rs:624197` | grid item's component → `containerParams` Reference |
| `SItemPortLoadoutEntryParams` | `multi_feature/types.rs:330771` | loadout entry: `itemPortName` → `entityClassName` |
| `SItemPortDef` | `multi_feature/types.rs:470577` | port def (name, defaultItem, `gridBehavior`) — **no transform** |
| `CargoControllerParams` | `multi_feature/types.rs:236546` | cargo *behaviour* (docking, batch load/unload) — no geometry |
| `SEntityComponentRoomParams` | `multi_feature/types.rs` | room entity — name/type, **no volume** |
| `SEntityComponentRoomConnectorParams` | `multi_feature/types.rs` | portal/door aperture — **`defaultAreaBounds`** |
| `SCItemDoorParams` | `entities_scitem_doors/` | door entity |
| `SCItemCargoGridParams` | `dormant/types.rs:32460` | **dead end** — 0 live instances; legacy/mining |

### Live counts (SC 4.8)

| Type (as a component across entity classes) | # entities |
|---|--:|
| `SCItemInventoryContainerComponentParams` grid items | (per-ship; Freelancer = 4 variants) |
| `CargoControllerParams` | 963 |
| `CargoGridOccupantProperties` (per cargo *box* stacking) | 1,399 |
| `SCItemCargoGridParams` | **0** |

`CargoGridOccupantProperties`/`CargoGridOccupantFace`
([multi_feature/types.rs:496540](../crates/sc-extract-generated/src/generated/multi_feature/types.rs#L496540))
are per-**cargo-box** stacking rules (which of the 6 faces may point up, and its
`CargoFaceStackingSupport` = `StackAll`/`StackSelf`/`StackNone`), not a property
of the ship grid — relevant later for *stacking* logic, not grid layout.

## What is NOT in the data (frontier / runtime)

- **Per-face wall vs open for a grid cell** — not a stored field. **Approximated**
  (Tier C-approx, `cargo_grid_occupancy.rs`): box-level per-face wall/open by
  reconciling interior-section AABBs into the grid frame. Triangle-exact occupancy
  (collision-mesh raytest) remains frontier.
- **Grid 3-D transform** — CGA helper transform (geometry layer), keyed by the
  item-port name. **Now demonstrated** (Tier B, `cargo_grid_placement.rs`): read
  the hull CGA's NMC scene graph and match the port node. Remaining gap is DCB-side
  loadout enumeration for modular/inherited grids, not the geometry read.
- **External-vs-internal opening classification** — inferable from the room pair
  a connector joins (interior room ↔ exterior), but not a labelled flag.
- **Runtime cargo contents / current occupancy** — server-side, never in the p4k.

## Geometry read — feasibility (Tiers B & C)

Q2's grid transform and Q3's per-cell wall test both live in the ship's geometry.
The format is well understood and there is a complete **MIT-licensed** reference
implementation in `E:\repros\StarBreaker` (`starbreaker-chunks` + `starbreaker-3d`)
that already assembles whole ships to glTF — so this is a *porting* exercise, not
format reverse-engineering.

### The format

CryEngine geometry (`.cga` animated / `.cgf` static) is a chunk file in one of
two shapes, auto-detected by magic:

- **IVO** (`#ivo`, magic `0x6F766923`, version `0x900`) — the **modern** format;
  current SC ships are IVO (verified: `MISC_Freelancer.cga`, the cargo door
  `.cga`). Header = magic/version/chunk_count/table_offset; chunk table entry =
  `chunk_type:u32 version:u32 offset:u64` (16 B).
- **CrCh** (`CrCh`, magic `0x68437243`, version `0x746`) — legacy CryEngine, the
  **same chunk family** as the `.soc`/`.socpak` files we already decode. Entry =
  `type:u16 version:u16 id:i32 size:u32 offset:u32`.

Chunk *types* of interest: IVO `NodeMeshCombos` (`0x70697FDA`, "NMC") holds the
node scene graph; CrCh `Node` (`0x100B`) / `Helper` (`0x1001`) are the legacy
equivalents; mesh/collision live in `IvoSkin2` (`0xB8757777`), `MeshIvo320`
(`0x92914444`), `StatObjPhysics` (`0x58DE1772`), `PhysicalHierarchy` (`0x90C62222`).

### Tier B — placement (grid transforms): moderate, small port

The item-port transform is **not** in any XML. The vehicle implementation XML
(`Data\Scripts\Entities\Vehicles\Implementations\Xml\MISC_Freelancer.xml`, plain
UTF-8) lists `<Part name="hardpoint_cargogrid_rear" class="ItemPort">` with only
size/flags/type — **104 item ports, only 4 positioned `<Helper>`s**. The real
transform is a named node in the hull `.cga`'s **NMC** chunk, a *metadata* scene
graph of node name + 3×4 `bone_to_world` (already the world transform — no
hierarchy walk needed) + per-node `properties` (`class=ItemPort`, …).

```
ship EntityClassDefinition
  ├─ SGeometryResourceParams → hull geometry path (e.g.
  │    "Objects\Spaceships\Ships\MISC\Freelancer_v2\MISC_Freelancer.cga")   ── AssetSource::read
  │      └─ IVO chunk table → NMC_Full (0x70697FDA) → nodes[name, bone_to_world]
  │           node.name == itemPortName ("hardpoint_cargogrid_rear")
  │             └─ bone_to_world[*][3] = grid world position  ∘ InventoryContainer.gridPosOffset
  └─ loadout (SItemPortLoadoutEntryParams) → which grid at which port (§2)
```

**No mesh/vertex decoding.** The port surface is ~700 MIT LOC:
`starbreaker-chunks` (418) + `nmc.rs` (196) + the NMC world-transform lookup
(~100 from `pipeline/loadout.rs`). This gives *every* hardpoint's position, not
just cargo. See [`cargo_grid_placement.rs`](../crates/sc-extract/examples/cargo_grid_placement.rs)
for the worked port (~120 LOC of NMC reader inline).

**Validated (SC 4.8).** The example resolves each grid's box (DCB) *and* world
position (geometry NMC), and its SCU totals match the published values exactly for
hull-mounted grids:

| Ship | grids (port → SCU) | total | placed |
|---|---|--:|---|
| `MISC_Freelancer` | rear 54, mid_left 6, mid_right 6 | **66** | 3/3 |
| `MISC_Freelancer_MAX` | rear 108, 2× mid 6 | **120** | 3/3 |
| `DRAK_Cutlass_Black` | main 40, rear 6 | **46** | 2/2 |

The mid grids resolve to symmetric `±1.40 m` X with identical Y/Z — an
independent correctness check on the transform read.

**Grid mount mechanisms — there are FOUR** (all handled by `cargo_grid_walls` /
the viewer pipeline; detection is name-independent — only *open*
`InventoryContainer`s count, excluding closed personal inventory and the
`35³ = 21952 SCU` `*_CargoGrid_Template` placeholders):

1. **Inline manual loadout entries** (`SItemPortLoadoutEntryParams`) — most ships
   (Freelancer, Cutlass, Ironclad).
2. **Port default items** (`SItemPortDefaultItemDef`).
3. **Mounted items' own loadouts** — cargo-bay doors / lift items carry the grid
   (Perseus: `hardpoint_cargo_lift` → lift item → grid at `hardpoint_cargo_grid`
   in the lift's own CGA). Reached only via `entityClassReference` — a
   `Reference`, invisible to plain tree walks; the collector resolves and
   descends (cycle-guarded, only where a grid exists below).
4. **XML-file loadouts** — `SItemPortLoadoutXMLParams.loadoutPath` →
   `scripts/loadouts/…/*.xml` with `<Item portName itemName/>` (Hammerhead: the
   elevator door's grid; the grid entity is referenced *nowhere* in the DCB by
   GUID — a `ReferenceGraph` reverse lookup finds nothing).

**Port transforms — the authoritative source is the port def itself:**
`SItemPortDef.AttachmentImplementation → SItemPortDefHelperNode →
SItemPortDefHelper { Name: <anchor node>, Offset: QuatT }`. Discovered via the
Idris-P, whose 25 cargo-grid ports carry explicit offsets anchored on the hull
root — the port names exist *nowhere else* in the game data (not NMC nodes, not
bones, not OC entities, not the vehicle XML). Resolution order:

1. **explicit attachment**: anchor node (item/ancestor geometries; identity if
   absent — e.g. the anchor is the geometry root) ∘ `Offset`;
2. **NMC node named after the port** — the degenerate case that carried the
   simple ships;
3. **item origin** (engine default; nothing anywhere).

Validated: Idris-P 25 grids distributed correctly (hangar rails at X ±17),
Taurus 168+6 = **174 SCU** exact, Hammerhead **40**, Andromeda **96**; the
Ironclad secure holds shifted +1.26 m under rule 1 and still read fully
enclosed. A sixth mechanism (grid entity placed directly in an interior `.soc`)
is implemented but not yet observed on any ship.
The `InventoryContainer` census (`cargo_grid_probe --containers <ship>`) remains
the mount-path-independent cross-check.

### Tier C — per-face wall occupancy

"Does a grid face a wall or open space?" needs geometry beyond the grid box.
**The full-geometry pipeline is built and validated against in-game ground truth**
(`cargo_grid_walls.rs`) — see C-full below. The earlier attempts (C-recon,
C-approx §history) are kept for the record of what *doesn't* work.

**C-full — BUILT & VALIDATED: per-face distance-to-wall from real wall placements**
([`cargo_grid_walls.rs`](../crates/sc-extract/examples/cargo_grid_walls.rs)). The
pipeline: hull NMC with **hierarchy-composed world transforms** → grids at their
port nodes → interior sections (`VehicleComponentParams.objectContainers[]`,
`nmc[boneName] ∘ Offset`) → per-section wall placements (CrCh `IncludedObjects`
chunk 0x0010 + CryXmlB `Entity` placements) → each placement's CGF model AABB
(header read, no triangle decode) → per grid-face-cell **distance to nearest wall
box**. Validated against player-known layouts (SC 4.8):

| Ship | ground truth | pipeline reads |
|---|---|---|
| Ironclad secure holds (5×8 SCU) | enclosed rooms, 1 SCU door | all 6 faces WALL flush; the door is a 1.0 m hatch (`…compartment_door_100`) |
| Ironclad main holds (4 grids) | walls around, central front-back walkway, side ladder strips | floor flush (3–5 cm), centerline faces open, outboard 0.75–1.4 m then wall, bulkheads flush |
| C2 Hercules | walkable ~1 SCU margin all round | floor flush, ceiling 1.2–1.9 m, rear wall 0.75 m, side aisles ≥1.78 m at tightest |
| Freelancer | rear loading door | side aisles 0.6 m then wall; rear face open = the cargo door ✓ |

**Load-bearing format discoveries** (all empirically verified — these are the
things that broke every shortcut before):

1. **NMC `bone_to_world` is local-to-parent, not world.** True world transforms
   compose up the `parent_index` chain (`world[i] = world[parent] ∘ local[i]`).
   A flat read silently works on simple hulls (Freelancer/Cutlass) and breaks on
   modular ones (Ironclad grids landed 14 m too low).
2. **The grid box is bottom-anchored at the port node** (centred X/Y, extending
   up in Z) — verified to 5 cm against the Ironclad's `drak_mod_cargogrid_6x10`
   floor plates. Centring it vertically sinks the grid half-depth into the deck.
3. **`SVehicleObjectContainerParams.Offset` is a `QuatT`** — read `Position` /
   `Rotation` (capitalised); the rotation wraps an `Ang3` (Euler degrees,
   CryEngine Z·Y·X). Lowercase reads silently return zero.
4. **`MeshIvo320` stores the model AABB at fixed byte +24.** A heuristic float
   scan is off-by-one hazardous (a stray leading float forms a "valid" box and
   scrambles the axes — floors read as 12 m-tall slabs).
5. Interior walls come from **two sources per `.soc`**: the `IncludedObjects`
   chunk (baked placements, 3×4 f64 row-major transform at +64 of each Type1
   record) *and* CryXmlB `Entity` nodes (Pos/Rotate quat/Scale + geometry inline
   at `PropertiesDataCore → EntityGeometryResource → Geometry×3 @path` or via
   `EntityClassGUID` → DCB).

**Distance rules** (refined against Freelancer visual validation): per face cell,
the distance is the min over (a) boxes **ahead** of the face → exact distance to
their near side; (b) **enclosing shells** the face is embedded in (monolithic
interior meshes like `misc_freelancer_cargo.cgf` / `…_smallcargo.cgf`) → distance
to the shell's far AABB side, an *upper bound* on open space — without this,
shell-built ships read falsely open on whole face sections. The **bottom face is
0 by construction** (grids are bottom-anchored on the deck). With these, the
Freelancer reads correctly: mids blocked on all sides except inward to the
walkway; rear-grid sides walled (0.6 m walkable strip) with the formerly-false
"open" front sections now bounded by the bay shell; rear face the most-open
(the ramp).

**Wall resolution — per-node sub-boxes.** Wall boxes are not per-placement mesh
AABBs but **per-NMC-node boxes**: every geometry node in a placed CGF/CGA
carries its own bounding box in the NMC entry (+152, node-local space, composed
through the node's world transform — verified: the union reproduces the
`MeshIvo320` model AABB). This decomposes monolithic shells into their actual
floor/wall/ceiling panels: the Freelancer's rear-grid side reading tightened
from a 3.2 m shell bound to real panels at 0.61–0.76 m, and the C2's falsely
"open" side aisles became bounded. Single-mesh CGFs (empty NMC) fall back to
the model AABB.

**Light-glow exclusion.** `LG_*` / `LIGHT_*` / `*glow*` NMC nodes are emissive
volumes that project INTO the room (they end at the cargo boundary) — as
sub-boxes they read as walls cutting into the grids. They are excluded by an
art-naming heuristic; the NMC metadata table was checked and carries no typed
marker (only DCC export notes), so naming is the only semantics that layer has.

**Triangle mode — C-exact, BUILT.** The occupancy now raycasts against **real
decoded triangles** whenever they're available (which is essentially always for
IVO-era ships). Format (StarBreaker-informed, MIT): the mesh streams live in
the geometry's companion file (`x.cgf` → `x.cgfm`, `x.cga` → `x.cgam`) — an IVO
file whose `IvoSkin2` chunk (0xB8757777) holds `flags:u32` + `MeshInfo` (76 B:
counts, model bbox, **scaling bbox**) + 88 B pad + submeshes (48 B each, incl.
`page_base` for u16 index paging) + tagged streams (`tag:u32 elem:u32 data`,
8-byte aligned). Positions: `IVOVERTSUVS` elem 16 = SNorm u16×3, dequantized
`(i16/32767) × max(half_extent, 1) + center` over the *scaling* bbox; elem 20 =
plain f32×3. Indices: `IVOINDICES` elem 2 = u16 + per-submesh `page_base`,
elem 4 = u32.

The soup is pruned to near-grid placements (± probe range) **plus the ship's
own hull skin** (bay ceilings/outer boundary are often the hull mesh, present
in no interior socpak). Per face cell: a 5-ray bundle along the outward normal,
clearance = the conservative minimum hit. Wins over boxes, all validated:
sloped walls measured per-cell (C2), single-mesh interiors finally read (the
Cutlass: aisles 0.62 m, ceiling flush, rear ramp open), door/ramp **apertures
read open** (the Ironclad secure-hold 1 m hatch shows as an open cell; C2/
Cutlass/Freelancer rear ramps read as the loading direction), and box
over-covers from racks/gantries disappear. Ships without decodable triangles
fall back to the per-node box mode.

**Fidelity limits (honest, triangle mode):** (a) the soup is *render* geometry
— door ITEMS' own meshes aren't included, so a doorway reads open even when
the door is closed (for a planner this is the loading aperture — arguably the
right reading); (b) rays sample at 5 points per 1.25 m cell — sub-cell gaps or
grates can slip between samples; (c) glow meshes are in the soup (thin panels;
negligible in practice). Runtime: the biggest ship (Ironclad, 10.6 M near-grid
triangles) computes in ~10 s on top of the DCB parse.

#### History — the shortcuts that did NOT survive validation

**C-recon — what's in the geometry** ([`cargo_grid_collision.rs`](../crates/sc-extract/examples/cargo_grid_collision.rs)):

```
MISC_Freelancer.cga (12.3 MB) — IVO chunks:
  StatObjPhysics    11 709 592 B   ← the collision mesh (95% of the file)
  PositionBonemap      524 328 B
  NodeMeshCombos        54 016 B   ← Tier B placement (done)
  CompiledBones/MeshIvo320/… small
```

The collision data **is present** (11.7 MB `StatObjPhysics` for the Freelancer,
10.4 MB for the Cutlass), and the `MeshIvo320` header yields a cheap model AABB
(Freelancer `22.6 × 36.7 × 7.8 m`, matches the real hull) — but an AABB alone is a
crude proxy.

- **Section-envelope AABB** ([`cargo_grid_occupancy.rs`](../crates/sc-extract/examples/cargo_grid_occupancy.rs),
  superseded): test grid faces against each interior section's `ObjectContainer`
  bounds. Looked right on the Freelancer (tight-fit coincidence), collapsed on the
  C2/Ironclad — a deck envelope is far bigger than the bay, so everything reads
  open. Also carried the flat-NMC bug. Kept only as a cautionary example.
- **Grid-topology heuristic** (same example): faces open where another grid abuts
  within a walkway gap. Threshold-dependent (missed the Ironclad's 4-SCU-wide
  walkway) and cannot distinguish an adjacent walled room from open cargo space
  (false-connected the secure holds).

**C-exact — remaining frontier.** Sub-AABB fidelity: decode the quantized mesh
streams (StarBreaker `ivo/skin.rs` + `dequant.rs`, MIT) or the `StatObjPhysics`
collision chunk (StarBreaker names but does **not** decode it — genuine RE), then
raytest grid-cell faces against triangles. Buys door apertures and shell-interior
walls; weeks of work.

### Scope tiers

| Tier | Delivers | Status |
|---|---|---|
| **A** | grid dims + SCU + which grids per ship | ✅ `cargo_grid_probe` |
| **B** | + 3-D placement of each grid | ✅ `cargo_grid_placement` (hierarchy + explicit port attachments) |
| **C-full** | + per-face distance-to-wall (per-node boxes) | ✅ `cargo_grid_walls` — box fallback mode |
| **C-exact** | + triangle raycasts: slopes, apertures, single-mesh interiors | ✅ `cargo_grid_walls` + viewer — render-mesh streams decoded (`StatObjPhysics` collision proxy remains undecoded; render mesh serves instead) |

## Reproduce

DCB side — `crates/sc-extract/examples/cargo_grid_probe.rs`:

```bash
# component census + which entities carry SCItemCargoGridParams (proves the 0):
cargo run -p sc-extract --release --example cargo_grid_probe

# a ship's cargo grids: top-level components, loadout grid mounts, InventoryContainer refs:
cargo run -p sc-extract --release --example cargo_grid_probe -- --entity MISC_Freelancer

# resolve a grid's InventoryContainer to read interiorDimensions / cell size:
cargo run -p sc-extract --release --example cargo_grid_probe -- --guid <containerParams-GUID>

# every grid box a ship defines (regardless of mount path), with SCU:
cargo run -p sc-extract --release --example cargo_grid_probe -- --containers Freelancer
```

Tier B — `crates/sc-extract/examples/cargo_grid_placement.rs` (reads the hull CGA
geometry, resolves each grid's 3-D placement):

```bash
cargo run -p sc-extract --release --example cargo_grid_placement                 # MISC_Freelancer
cargo run -p sc-extract --release --example cargo_grid_placement -- DRAK_Cutlass_Black
```

Tier C — `crates/sc-extract/examples/cargo_grid_walls.rs` (the validated
full-geometry pipeline: per-face distance-to-wall):

```bash
cargo run -p sc-extract --release --example cargo_grid_walls -- DRAK_Ironclad
cargo run -p sc-extract --release --example cargo_grid_walls -- CRUS_Starlifter_C2
# debug helpers: --verbose (placements + floor diagnostic), --find <cgf-substr>,
#                --cgf <path> (chunk table + AABB of one geometry file)
```

Historical: `cargo_grid_collision.rs` (chunk recon + model AABB) and
`cargo_grid_occupancy.rs` (superseded box-level attempts — kept as the record of
what failed validation).

**Visual validation — `tools/sc-cargo-viewer`** (egui/eframe): interactive 3D
viewer over the same pipeline. Ship list = the typed vehicle census (every
`EntityClassDefinition` with a top-level `VehicleComponentParams`, ~970 on 4.8);
renders grid wireframes, wall boxes (distance-filtered), and each grid face's
cells **colored by measured distance-to-wall** (red = flush wall, yellow =
walkway gap, green = open). Drag = orbit, right-drag = pan, scroll = zoom;
top/front/side presets.

```bash
cargo run -p sc-cargo-viewer --release
```

Object-container side — expand a ship's interior and read the room graph + entity
tree (svarog at the pinned rev; `E:\repros\Svarog\target\release\svarog.exe`):

```bash
svarog.exe p4k-extract -p "C:\Games\StarCitizen\LIVE\Data.p4k" \
  -o <out> --filter '*Freelancer*base_int_back_main.socpak' --expand-socpak
# → <out>/.../base_int_back_main/base_int_back_main.rmxml   (RoomMapping graph)
#   .../base_int_back_main.soc    (CrChF → CryXmlB entity tree; decode via
#                                   sc_extract::object_container::decode)
#   .../base_int_back_main.xml    (ObjectContainer bounds + static-entity tags)
```

The `.soc` entity tree exposes `SEntityComponentRoomParams` (rooms),
`portal_*` (portals), `SCItemDoor*` (doors incl. the cargo bay door), and the
wall/cargo CGA meshes — decode it with the existing
`crates/sc-extract/src/object_container.rs` reader (same CrCh→CryXmlB path the
`soc_harvestable_probe` uses).

## Toward a crate

A future `sc-cargo` (or a module in a ship crate) would join, per ship variant:
`loadout entries → grid items → InventoryContainer` to produce a typed
**`CargoGrid { port, dims_m, cells, scu, min/max_item, offset, world_pos }`** and a
ship **total SCU**. Status of the pieces:

- **Grid box + SCU (Tier A)** — DCB-only, typed, reproducible today
  (`cargo_grid_probe`). ✅
- **3-D placement (Tier B)** — demonstrated (`cargo_grid_placement`): port the IVO
  chunk table + NMC reader (~700 MIT LOC) and match the port node. The productizing
  work is the **DCB loadout enumeration**, not the geometry — a robust walk must
  cover manual loadouts, port `defaultItem`s, base-class inheritance, and removable
  cargo modules (the Constellation case). ✅ core / ⚠️ enumeration.
- **Openings (Q4)** — an OC-entity read reusing the `object_container` decoder
  (`SCItemDoor*` + room-connector `defaultAreaBounds`). Not yet built as an example.
- **Per-cell wall occupancy (Tier C)** — needs a collision-mesh decoder
  (`StatObjPhysics`); scoped in `cargo_grid_collision`, weeks of work, likely
  approximated (grid box = usable) in a first product.
