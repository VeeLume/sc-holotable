# Resource gathering — data-source map & crate plan

> Status: **investigated; the location join is proven, crate not yet built.**
> This is the data-source spec for the "gatherable resources" domain —
> ship/ROC/FPS mining, plants (harvestables), and salvage/debris — covering the
> four questions: **where** a resource is found, **how rare** it is, its
> **quality distribution**, and its **scan signal**. The one engineering unknown
> (the body→provider `.soc` join) is now closed end-to-end by a working probe
> example (see [The object-container join](#the-object-container-join--solved-validated)).
> No domain crate is scaffolded yet; the closing sections propose one.
>
> Grounded in a deep exploration against **LIVE (SC 1.0 / 4.8, changelist on
> the current `datacore/*` tag)**: the live `Game2.dcb` parsed via `sc-extract`,
> the full DataCore exported to a 60,309-file XML corpus, and every Stanton +
> Nyx object container expanded. Every checkable number in the reference tool
> (SCMDB / Rocks-Syndicate-style mining UIs) was reproduced **exactly** from
> this data. A 12-agent verification workflow (6 trace + 5 adversarial verify,
> all `holds=true` at high confidence) confirmed the chains below.

## TL;DR — the one thing that was wrong before

An earlier pass concluded *"per-location resource distribution is server-side,
not in the p4k (telemetry only)."* **That is false.** It came from stale web
sources (describing a pre-4.x state) and from agents pattern-matching
"harvestable = flora." The truth:

- **The per-location distribution IS in the game data**, in the DataCore plus
  the object-container (`.soc`) entity instances.
- **The harvestable-provider system is the *generic world resource spawner*** —
  it covers ship mineables, ROC (ground-vehicle) mineables, FPS mineables,
  plants, *and* salvage/debris, not just plants. The name is misleading.
- One record per celestial body (`HarvestableProviderPreset`, 49 of them) holds
  the entire per-location rarity table; the screenshots' percentages are simple
  normalizations of two probability fields.

## The unified chain

Every gathering type flows through one structure, anchored on a per-body
`HarvestableProviderPreset`:

```
Object container (body socpak)  ──HarvestableProviderComponent preset=<GUID>──┐
  (pivot.entxml / .soc, CrChF binary)                                         │
                                                                              ▼
HarvestableProviderPreset  (49 records, one per body)
  e.g. HPP_Stanton4b (Clio), HPP_Nyx_GlaciemRing, HPP_AaronHalo, HPP_Lagrange_A…G,
       HPP_Pyro1…6, HPP_ShipGraveyard_001, HPP_SpaceDerelict_General
│
├─ harvestableGroups[]  : HarvestableElementGroup
│    groupName            (SpaceShip_Mineables / GroundVehicle_Mineables /
│                          FPS_Mineables / Harvestables / Salvage_*)
│    groupProbability     ──────────────────────────────► MODE %  (group / Σgroups)
│    └─ harvestables[]  : HarvestableElement
│         relativeProbability ─────────────────────────► per-resource %  (elem / Σelems in group)
│         ├─ clustering  ► HarvestableClusterPreset ────► "100% · 4–6"
│         │     probabilityOfClustering + weighted {minSize,maxSize} bands
│         └─ harvestable ► HarvestablePreset
│               entityClass ► EntityClassDefinition  (mineable rock / plant / wreck)
│                 ├─ MineableParams
│                 │     globalParams  ► MiningGlobalParams  (ship | fps | groundvehicle ← MODE)
│                 │     composition   ► MineableComposition ► within-rock element %, qualityScale
│                 └─ SSCSignatureSystemParams
│                       radarProperties.baseSignatureParams.signatures[4]  ► scan signal ×1000
│                       contactType ► RadarContactTypeEntry "MineableMinearlDeposits"
│
└─ (element) ► MineableElement ► resourceType ► ResourceType ► displayName (locale)
                                                  └─ properties[ResourceTypeCraftingData]
                                                       qualityDistribution  ► quality bell curve
                                                       qualityQuantization  ► B1…B8 bands
                                                       qualityLocationOverride ► per-place shift
```

**Mode is distinguished by data, not strings:** which `MiningGlobalParams` the
rock's `MineableParams.globalParams` points at (`miningglobalparamsship` /
`…fps` / `…groundvehicle`) *is* the ship/ROC/FPS classification. No name
matching.

## The four questions

### 1. Where — two edges, both anchored on the object container

"Where" is **two separate links**, and neither is a DCB-internal edge from a
resource record to a location:

1. **Resource → provider → object container** (presence). Each body/field's root
   entity carries a `HarvestableProviderComponent` whose `preset` is the
   `RecordId` of a `HarvestableProviderPreset`. Verified verbatim for Clio:
   - `oc/…/stanton/stanton4b/stanton4b/pivot.entxml`:
     `<HarvestableProviderComponent __type="HarvestableProviderParams" preset="703a18ca-7f7c-4489-a64a-cd0cd359b8fe"/>`
   - → `dcbraw/…/harvestable/providerpresets/system/stanton/hpp_stanton4b.xml`.
   Reading it is a solved, structured parse — see
   [The object-container join](#the-object-container-join--solved-validated).
2. **Object container ↔ StarMapObject** (the actual place). The body socpak is
   tied to a `StarMapObject` (the navigable location) by a GUID pair in the
   **system** object container — see
   [Location binding](#location-binding--the-system-oc-bridge). This is the hop
   that turns `hpp_stanton4b` into "Clio."

**No DCB record references the provider GUIDs**; the body→preset binding exists
*only* in the object-container instances. `HarvestableProviderPreset` is a seeded
record type in `sc-extract` (gated by the `harvestable` feature).

There is also a **third, independent edge — quality by location** — a direct
typed DCB reference from a resource's quality data to a `StarMapObject`; see
[Location binding](#location-binding--the-system-oc-bridge).

### 2. Rarity — two normalized probability fields

| Tool figure | Source field | Computation |
|---|---|---|
| Mode % (Ship/ROC/FPS share) | `HarvestableElementGroup.groupProbability` | `group / Σ groups` |
| Per-resource % within a mode | `HarvestableElement.relativeProbability` | `elem / Σ elems in group` |
| Cluster "100%" column | `HarvestableClusterPreset.probabilityOfClustering` | direct |
| Cluster "4–6" size range | weighted `HarvestableClusterParams{minSize,maxSize}` | envelope of the bands |

Verified exact for Clio, Glaciem Ring, and Aaron Halo (worked examples below).

> **Correction from verification:** cluster presets are *weighted discrete size
> distributions* (each band has `minSize==maxSize` with a `relativeProbability`
> weight), not uniform ranges. `commonshipmineable_cluster` = `{6@0.1, 5@0.3,
> 4@0.6}` → displayed "4–6". A faithful model must keep the per-size weights,
> not just the min/max.

### 3. Quality — `ResourceType` → `ResourceTypeCraftingData` → crafting-quality records

The B1…B8 "quality distribution" modal is the **crafting-quality system**,
already modeled in `sc-crafting`. The bridge is
`ResourceType.properties[ResourceTypeCraftingData]`
([multi_feature/types.rs:287134](crates/sc-extract-generated/src/generated/multi_feature/types.rs:287134)):

```
ResourceTypeCraftingData {
    qualityDistribution     → CraftingQualityDistribution_Base  (Normal{min,max,mean,stddev})
    qualityLocationOverride → CraftingQualityLocationOverride_Base
    qualityQuantization     → CraftingQualityQuantization_Base  (8 × Band{start,end,mappedValue})
}
```

Verified for **Hadanite**: distribution `Normal{min=201, max=1000, mean=201,
stddev=298}` (exactly the modal's mean 201 / σ 298 / range 201–1000); the 8
quantization bands' `mappedValue`s are exactly `274 / 526 / 665 / 762 / 867 /
916 / 959 / 1000` (band ranges `0–399, 400–599, …, 999–1000`). The displayed
band **percentages** (50/33/9/5/3/1/<1/<0.1) are *not* stored — they are the
integral of the truncated normal over each band (recomputed independently to
within 0.2 pp).

> **Per-location quality overrides are real and populated.** `Pyro` shifts every
> domain (e.g. Hadanite Pyro mean 209 / σ 308); two records carry genuine
> 62-entry per-location lists (`LegendaryShipMineable_QualityOverride_RCD` →
> `Ore_Savrilium`, `UncommonShipMineable_QualityOverride_Torite` →
> `Ore_Torite`) referencing the Nyx Glaciem rock-cracker cluster StarMapObjects
> with a raised quality floor (min 651 vs default 501). So quality *can* vary by
> place even though it shares no mechanism with resource presence.

Quality machinery already lives in `sc-crafting` as `Quality` /
`QualityDistribution` / `QualityQuantization` / `QualityLocationOverride`. The
gathering crate joins to it, it does not re-model it.

### 4. Signal — per-rock `signatures[4]` (the `Resource` channel)

Each mineable-rock `EntityClassDefinition` carries
`SSCSignatureSystemParams → radarProperties → baseSignatureParams →
signatures[8]`, an array indexed by `ESignatureType`. Index **4 = `Resource`**
([enums.rs:32762](crates/sc-extract-generated/src/generated/enums.rs:32762):
`Infrared, Electromagnetic, CrossSection, Decibel, Resource, Identity, …`). It
is the only non-zero channel on these rocks, and its value is the tool's "sig"
× 1000. Verified for **all 25 ship-mineable ores** plus the uniform modes:

| Resource | stored | displayed | | Resource | stored | displayed |
|---|--:|--:|---|---|--:|--:|
| Quantainium | 3170 | 3.170 | | Tungsten | 3870 | 3.870 |
| Stileron | 3185 | 3.185 | | Agricium | 3885 | 3.885 |
| Savrilium | 3200 | 3.200 | | Torite | 3900 | 3.900 |
| Ouratite | 3370 | 3.370 | | Hephaestanite | 4180 | 4.180 |
| Riccite | 3385 | 3.385 | | Tin | 4195 | 4.195 |
| Lindinium | 3400 | 3.400 | | Quartz | 4210 | 4.210 |
| Beryl | 3540 | 3.540 | | Corundum | 4225 | 4.225 |
| Taranite | 3555 | 3.555 | | Copper | 4240 | 4.240 |
| Borase | 3570 | 3.570 | | Silicon | 4255 | 4.255 |
| Gold | 3585 | 3.585 | | Iron | 4270 | 4.270 |
| Bexalite | 3600 | 3.600 | | Aluminum | 4285 | 4.285 |
| Laranite | 3825 | 3.825 | | Ice | 4300 | 4.300 |
| Aslarite | 3840 | 3.840 | | | | |
| Titanium | 3855 | 3.855 | | **FPS gems** | 3000 | 3.000 |
| | | | | **ROC gems** | 4000 | 4.000 |
| | | | | **Salvage scrap** | 2000 | 2.000 |

The values encode the resource: a `+15` step per resource within each rarity
tier (tier sets the base: legendary 3170+, epic 3370+, rare 3540+, uncommon
3825+, common 4180+). The in-game cluster readout = `N × base` for an N-rock
cluster — the **summation is engine runtime behavior, not stored** (the per-rock
base is data; the aggregation rule is inferred).

> **Corrections from verification:**
> - Salvage signature is **not** uniformly 2000 — only scrap is; some debris is
>   1700/1850, and in current data C2 = 2400, 890 = 3000. The tool's "2.000"
>   reflects the scrap/group entry or predates a rebalance.
> - Legacy *archetype* rocks (`asteroid{c,e,m,p,q,s}type…`, felsic/granite/etc.)
>   instead carry a per-archetype value (4700–4900) that does **not** identify
>   the contained resource. Only the modern per-resource rock family encodes the
>   resource in the signature.

## Worked example — Clio (`HPP_Stanton4b`, `stanton4b`)

Three groups; every screenshot number falls out:

| Group | `groupProbability` | Mode % (`/43.5`) |
|---|--:|--:|
| `SpaceShip_Mineables` | 6 | 13.8 % |
| `GroundVehicle_Mineables` | 12.5 | 28.7 % |
| `FPS_Mineables` | 25 | 57.5 % |

| Element | `relativeProbability` | per-resource % | sig | cluster |
|---|--:|--:|--:|---|
| Raw Ice | 40 | 40.0 | 4.300 | 100% · 4–6 |
| Copper | 40 | 40.0 | 4.240 | 100% · 4–6 |
| Taranite | 18 | 18.0 | 3.555 | 100% · 2–4 |
| Quantainium | 2 | 2.0 | 3.170 | 25% · 2 |
| Aphorite (FPS) | 61.1 | 61.1 | 3.000 | 100% · 10–21 |
| Dolivine (FPS) | 36.9 | 36.9 | 3.000 | 100% · 10–21 |
| Janalite (FPS) | 2 | 2.0 | 3.000 | 100% · 10–25 |
| Glacosite (ROC) | 8.5 | 68.0 | 4.000 | 100% · 10–21 |
| Beradom (ROC) | 2.5 | 20.0 | 4.000 | 100% · 10–21 |
| Feynmaline (ROC) | 1.5 | 12.0 | 4.000 | 100% · 10–25 |

Ship & FPS groups sum to 100 (values are literal); the ROC group sums to 12.5,
so it must be normalized (`8.5/12.5 = 68%`). `areas` is `Count=0` — Clio has no
per-area overrides. Drill into one element: Raw Ice → `mining_common_ice.xml`
(`HarvestablePreset`) → entity `mineablerock_surfacecommon_ice.xml` →
`MineableParams{globalParams: miningglobalparamsship, composition:
surfaceshipmining/commonshipmineables_ice}` and `signatures[4]=4300`. The
composition's parts give the within-rock content (Ice: 9.7–15.7% @ qualityScale
1.0, 34.3–84.3% @ qualityScale 0.49) — distinct from the occurrence % above.

## Worked example — Glaciem Ring (`HPP_Nyx_GlaciemRing`, Nyx belt)

Five groups (`groupProbability` 0.1 / 0.04 / 0.01 / 0.0057 / 0.00033, Σ =
0.15603); `0.1/0.15603 = 64.1%` "Ship Mining". The Ship group's 7 elements'
`relativeProbability` sum to exactly 100 and *are* the tool's percentages:
Torite 28.5, Bexalite 18, Ice 13.9, Aluminum 13.8, Iron 13.8, Lindinium 10,
Savrilium 2. Tier → cluster: common 100%·4–6, uncommon 100%·3–5, rare 100%·2–4,
epic 40%·2–3, legendary 25%·2. The other four groups are salvage (next section).

## Salvage & debris

Same provider system. `HPP_Nyx_GlaciemRing`'s salvage groups:

| Group (data) | Tool label | `groupProbability` | % |
|---|---|--:|--:|
| `Salvage_FreshDerelicts` | Derelict Salvage | 0.04 | 25.6 |
| `Salvage_BrokenShips_Poor` | Debris (Small) | 0.01 | 6.4 |
| `Salvage_BrokenShips_Normal` | Debris (Medium) | 0.0057 | 3.7 |
| `Salvage_BrokenShips_Elite` | Debris (Large) | 0.00033 | 0.2 |

A wreck enters via `HarvestableElement.harvestable → HarvestablePreset →
entityClass` pointing at a wreck `EntityClassDefinition`, e.g.
`salvageabledebris_c2.xml` (`@vehicle_NameCRUS_Starlifter_C2`),
`salvageabledebris_890.xml` (`@vehicle_NameORIG_890Jump`), and the scrap cluster
`salvagescrap_reclaimermultigeo`. Inside `Salvage_FreshDerelicts` the weights
900 / 5 / 2 normalize to 99.2% / 0.6% / 0.2% (the tool's "Legacy Salvage
Cluster" / C2 / 890). The "S/M/L Debris – N parts" rows are the count of
*distinct* `salvageablerepairable_shipdebris_{s,m,xl}_*` entity classes in each
group (5 / 9 / 5).

- **The labels "Legacy Salvage Cluster", "Derelict Salvage", "Debris (S/M/L)"
  are tool-invented** — they are not in `global.ini`; the data names are
  `Salvage_*`.
- **Surface crash sites** (`stanton…/derelict/drlct_sfce_*`) are a *separate,
  hand-placed* mechanism — static `Brush` hull pieces + `TagPoint` loot, no
  `HarvestableProviderComponent`. Not part of this chain.
- The visual `AsteroidFieldComposition` records (`environments/…`) are
  mesh/fog-only — zero resource linkage.

## Plants / harvestables

Same provider system, the `Harvestables` group (`groupProbability` ~35). Chain:
`plant_X` (base entity) → `SubHarvestableSlot` → `fruit_X` (carryable entity) →
`ResourceContainer.defaultComposition` → `ResourceType.X` → display name.

- 14/14 listed plants + 4/8 FPS minerals are computable from provider presets.
- **Caveats:** Carinite / Jaclium / Saldynium come only from *cave* slot-presets
  (`SubHarvestableMultiConfigRecord`); the DCB→cave-OC join wasn't located.
  Sadaryx has no harvestable preset at all in current data. **Clio
  (`HPP_Stanton4b`) currently has no plant group** — Stanton plants live on
  Hurston / microTech; community lists putting plants on Clio are stale.
- Some carryables (Amiant/Flareweed/Wuotan) have an empty
  `ResourceContainer.defaultComposition` — identity is only the AttachDef locale
  key, and quality is a runtime `generateRandomQuality`, not `ResourceType`.

## The object-container join — solved & validated

**Decision (2026-06-17): structured `.soc` parse** — the clean, GUID-exact path,
no name-matching (honors design principle #5). The body→provider binding is the
`HarvestableProviderComponent.preset` GUID on the body's `ProceduralEntity`,
inside the body's object-container `.socpak`. **As of 2026-06-17 this is proven
end-to-end** by `crates/sc-extract/examples/soc_harvestable_probe.rs`.

**The join lives in two file shapes** — both now decodable with **svarog +
~40 lines**:

1. **Planet/moon bodies** → `pivot.entxml` inside the body socpak. This is
   **plain UTF-8 XML** (Clio) or raw CryXmlB — `svarog_cryxml::CryXml` reads the
   CryXmlB case; plain XML needs no decode at all. Zero new format work.
2. **Asteroid fields / gas clouds / lagrange childclouds** → the field
   generator's binary `.soc`. Despite the `CrChF` magic, a `.soc` is a **CrCh
   chunk file** whose chunk table includes a **`CRYXMLB` chunk (type `0x0004`)**
   carrying the entity tree — the same CryXmlB svarog reads.

```
body socpak (ZIP)  →  .soc (CrCh chunk file)  →  CRYXMLB chunk (0x0004)
  →  svarog_cryxml::CryXml::parse → to_xml_string → <Entity> component children
  →  HarvestableProviderComponent.preset   (the HarvestableProviderPreset GUID)
```

**Key finding — no PropertiesDataCore binary parser is needed.** The component
property tables the earlier pass byte-carved from a *"PropertiesDataCore property
table"* live **inside** the CryXmlB chunk (verified: the `PropertiesDataCore`
string sits at byte `0x32B63`, within the `0x0004` chunk range). svarog's CryXmlB
decoder surfaces them as ordinary XML elements. The byte-scanning was only a
workaround for never having peeled the chunk.

**Verified CrCh header layout** (little-endian; empirically confirmed — chunk
offsets + sizes land exactly on EOF, and the `0x0004` chunk begins with
`CryXmlB\0`):

```
0x00  magic "CrCh"        [4]   (a 5th byte 'F' follows in the version field)
0x04  version             u32
0x08  chunk_count         u32
0x0C  chunk_table_offset  u32
table entry (16 bytes):  type:u16  version:u16  id:u32  size:u32  offset:u32
```

The probe detects CryXmlB chunks by **payload magic** (`CryXmlB\0`), not by
trusting `type == 0x0004`, so it can't silently misparse a version variant.

**Probe results (against `target/probe-resources/`, 2026-06-17):** 6,380
`.soc`/`.pla`/`.entxml` files scanned → **56 provider components, 40 with a real
GUID, 16 null, 0 decode failures.** Real GUIDs came from **both** routes:

- CrCh `.soc`: `childcloud_s4_l*.soc` → `71dad029…` / `4debe820…` / `8b8f001f…`.
- Plain-XML/`.pla`: Clio `stanton4b` → `703a18ca-7f7c-4489-a64a-cd0cd359b8fe`
  (= `HPP_Stanton4b`, the **validation gate — passes**), Daymar `stanton2b` →
  `bbf69b47…`, microTech `stanton4_planet_only.pla` → `2bdb874e…`.

**Caveats confirmed by the probe:**
- *Null presets are normal* — segment/child containers carry `00000000-…`; one
  entity per body carries the real GUID. Scan all, ignore nulls.
- *Include-only CrCh containers* (`nyx1/2/3.soc`: only a `0x0002` + `0x0010`
  `INCLUDED_OBJECTS` chunk, no `0x0004`) have **no entity tree** and thus no
  provider — a clean skip, not an error.

**Tooling:** `svarog-cryxml` is pinned in the workspace (git rev `7f06225`) and
added to `sc-extract`'s **dev-dependencies** (example-only — no library bloat).
The CrCh chunk-table peel (`cryxml_chunks` in the example) is the **single new
piece** and is what graduates into a real `sc-extract` SOC source. StarBreaker
(`E:\repros\StarBreaker`: `starbreaker-chunks`, `starbreaker-cryxml`,
`starbreaker-3d/src/socpak.rs`) remains the format cross-reference, but **no port
was needed** — svarog's CryXmlB decoder does the whole payload.

## Location binding — the system-OC bridge

> **Correction (2026-06-18).** An earlier draft claimed there was *no* GUID link
> between an object container and its `StarMapObject`, so location had to be
> recovered by a **body-token name match** (OC folder `stanton4b` ⇔ record
> `StarMapObject.Stanton4b`). **That was wrong.** A typed GUID link exists; it
> just lives in the **system** container, not the body's own. The name
> correspondence is real but *incidental* — use it only as a sanity cross-check,
> not the join.

### The bridge: `OrbitingObjectContainer` entities in the system OC

Each system has a top-level object container (`stantonsystem.socpak`,
`pyrosystem`, `nyxsystem`). Inside it, **every body, moon, lagrange point, jump
point, asteroid base, comm array, … is placed by an `OrbitingObjectContainer`
entity** that carries both halves of the location link on one entity:

```
<Entity Name="OOC_Stanton_4b_Clio" EntityClass="OrbitingObjectContainer" …>
  <EntityComponentObjectContainer  objectContainer="…/stanton/stanton4b.socpak"/>   ← the body socpak
  <EntityComponentObjectMetadata>
    <SNavPointObjectMetadataParams  starmapRecord="2a21d86f-…"/>                     ← the StarMapObject GUID
  …
```

So the full presence chain is **GUID/path at every hop — no name matching**:

```
StarMapObject  ◄──starmapRecord──  OrbitingObjectContainer (system OC)  ──objectContainer──►  body socpak
                                                                                                   │
                                                  HarvestableProviderComponent.preset ◄───────────┘
                                                                                                   ▼
                                                  HarvestableProviderPreset → groups/elements → ResourceType
```

### Granularity falls out of the pairs
- **Bodies / moons / lagrange / jump points** → one `OrbitingObjectContainer` →
  one socpak → one `StarMapObject` (`stanton4b.socpak`,
  `lagrangepoints/stanton4_l4.socpak`, `jumppoints/jumppoint_stanton_terra.socpak`).
  Clean 1:1.
- **Asteroid bases** → **many distinct `starmapRecord` GUIDs point at the *same*
  reusable socpak** (`asteroidbase/ab_mine_stanton_cloud_med_001.socpak` appears
  20+ times). One provider (in that one socpak) applies to *all* those placements;
  find them by enumerating every `OrbitingObjectContainer` whose `objectContainer`
  is that socpak. The 1:many is a join, not a guess.

### The StarMapObject hierarchy (for keying / display)
- Rooted at the **`Star`**, *not* the `SolarSystem` (the `SolarSystem` record is a
  parentless, childless sibling node). 1962/2054 locations carry a `parent`;
  `sc-locations` exposes `parent_of` / `children_of` / `ancestors`.
- 21 `LocationKind`s; there is **no dedicated Lagrange/AsteroidCluster/AsteroidBase
  kind** — lagrange points and asteroid clusters are `Asteroid`/`AsteroidValidQt`,
  jump points are `Anomaly`, stations are `Manmade`.
- Pyro shape (851 records): 6 planets, 6 moons, ~381 asteroid-type, 101 outposts;
  `PrivateMiningPoint_*` hang off lagrange asteroid clusters; encounter clusters
  hang off the Star. Aaron Halo has **no** single `StarMapObject` — it is placed
  *as* the cloud of `ab_mine_*` asteroid-base nodes (its `aaronhalo.socpak`
  placement has an empty `starmapRecord`).

### Quality-by-location — the one direct DCB→location edge
Separate from presence, and the **only** place the DCB itself references a
location: `ResourceType → ResourceTypeCraftingData → qualityLocationOverride →
CraftingQualityLocationOverrideEntry.location` is a **GUID Reference to a
`StarMapObject`**. It is hierarchy-aware (the `location` may be a whole system,
e.g. `pyrosolarsystem`, or a single cluster, e.g. the 61
`asteroidclusterbase_nyx_rockcracker_*`) and **sparse** (only resources with
per-place quality variance have it — the trace found ~2: Savrilium-RCD, Torite).
Quality can therefore be **more granular than presence** (per-cluster overrides
on a per-field provider).

### Caveats
- **`objectContainer` paths vary** in case/prefix (`objectcontainers/…`,
  `ObjectContainers/PU/…`, `Data/ObjectContainers/…`) — normalize before matching
  p4k entries.
- **Not every placement is a location** — scattered derelict sets and the skybox
  place with an empty `starmapRecord`.
- **`dcbraw` XML is lossy for references** — the exporter empties `parent`/`type`
  etc., so the hierarchy is only correct off the **binary** `Game2.dcb`. Use the
  typed parse (`sc-locations`), reproduced by
  `crates/sc-locations/examples/dump_location_tree.rs`.

### Cook implication
The location cook must parse the **system** object containers (not just body
socpaks): collect `OrbitingObjectContainer` `(starmapRecord, objectContainer)`
pairs → the reusable **`StarMapObject` ↔ socpak** map → per socpak read the
`HarvestableProviderComponent.preset` → compose `StarMapObject → provider →
ResourceType`.

## What is NOT in the data (runtime / absent)

- **Absolute spawn density** (rocks per km²) — runtime; only normalized shares
  are data-derived.
- **Cluster signal summation** (`reading = N × base`) — runtime aggregation.
- **Per-cell terrain placement** — procedural at runtime; the data side ends at
  the provider + geometry tag + `HarvestablePreset.transformParams`
  (slope/elevation/scale gates).
- **Cave harvestable → location join** (Carinite/Jaclium/Saldynium) — not
  located in the corpus; likely a `SubHarvestableComponent` on cave OC entities.
- **Sadaryx** — absent from the harvestable system entirely.
- **Band percentages** and **the /1000 signal display scaling** — UI-side
  computations, not stored fields.

## Existing crate landscape

| Crate | Owns | Feature |
|---|---|---|
| `sc-resources` | `ResourceType` catalog (206 — dimension table), refining graph, density | `resourcetypedatabase` |
| `sc-crafting` | recipes + **quality** subsystem (`Quality`/`QualityDistribution`/`QualityQuantization`/`QualityLocationOverride`) | `crafting` |
| `sc-locations` | `StarMapObject` places (the "where" targets) | `starmap` |

The gathering layer is the missing piece; it *joins* these, it does not
duplicate them.

## Proposed crate

A new domain crate (working name `sc-mining` or `sc-gathering`) over
`sc-extract`, depending on `sc-resources` (catalog) and joining to `sc-crafting`
(quality) and `sc-locations` (place). Sketch:

- **`Provider`** — wraps `HarvestableProviderPreset`: groups → elements with
  normalized rarity, resolved to `ResourceType` + mode + cluster + signal.
- **`Deposit`** — wraps `MineableComposition` + parts (within-rock content,
  `qualityScale`).
- **`Mineable`** — `MineableElement` joined to `ResourceType` + mining mechanics
  (instability/resistance/optimal-window/cluster-factor) + resolved signal.
- **`GatheringPools` / per-location index** — keyed by `StarMapObject` GUID via
  the [system-OC bridge](#location-binding--the-system-oc-bridge), classifying
  ship/ROC/FPS/plant/salvage by `MiningGlobalParams` family (not group-name
  strings — see [the mode note](#the-unified-chain)).
- Salvage and plants as modules in the same crate (they share the provider
  spine) or as a thin follow-on.

Open decisions before scaffolding: **(a)** crate shape (one `sc-mining` with
modules vs `sc-gathering` umbrella vs per-method crates); **(c)** initial scope
(mining-only end-to-end first, vs all three at once). **(b) the location join is
resolved** — it is the system-OC `(starmapRecord, objectContainer)` bridge
([Location binding](#location-binding--the-system-oc-bridge)), **not** the
name-token match an earlier draft assumed. Remaining build work: (1) graduate the
example's `cryxml_chunks` peel into an `sc-extract` SOC source; (2) the location
cook parses **system** OCs for the bridge pairs, then body socpaks for the
provider component.

## Record-type reference

Generated bindings (all in `crates/sc-extract-generated/src/generated/`):

| Record / struct | Location | Key fields |
|---|---|---|
| `ResourceType` | `multi_feature/types.rs:601212` | `displayName`, `properties[]`, `densityType`, `refinedVersion` |
| `ResourceTypeCraftingData` | `multi_feature/types.rs:287134` | `qualityDistribution`, `qualityLocationOverride`, `qualityQuantization` |
| `MineableElement` | `multi_feature/types.rs:258742` | `resourceType`, `elementInstability/Resistance`, `elementOptimalWindowMidpoint`, `elementClusterFactor` |
| `MineableComposition` | `multi_feature/types.rs:258948` | `depositName`, `minimumDistinctElements`, `compositionArray[]` |
| `MineableCompositionPart` | `multi_feature/types.rs:258852` | `mineableElement`, `min/maxPercentage`, `probability`, `curveExponent`, `qualityScale` |
| `MineableParams` (component) | `multi_feature/types.rs:259053` | `globalParams`, `composition`, `filledFactor` |
| `MiningGlobalParams` | `multi_feature/types.rs:258284` | `defaultMass`, `cSCUPerVolume`, instability/breaking curves |
| `HarvestableProviderPreset` | `harvestable/types.rs:548` | `harvestableGroups[]`, `areas[]` |
| `HarvestableElementGroup` | `harvestable/types.rs:354` | `groupName`, `groupProbability`, `harvestables[]` |
| `HarvestableElement` | `harvestable/types.rs:285` | `harvestable`, `relativeProbability`, `clustering` |
| `HarvestableClusterPreset` | `harvestable/types.rs:240` | `probabilityOfClustering`, `clusterParamsArray[]` |
| `HarvestableClusterParams` | `harvestable/types.rs:204` | `relativeProbability`, `minSize`, `maxSize`, `min/maxProximity` |
| `CraftingQualityDistributionNormal` | `multi_feature/types.rs:285220` | `min`, `max`, `mean`, `stddev` |
| `CraftingQualityQuantizationBand` | `multi_feature/types.rs:286437` | `start`, `end`, `mappedValue` |
| `CraftingQualityLocationOverrideEntry` | `multi_feature/types.rs:285734` | `location`, `qualityDistribution` |
| `SSCSignatureSystemParams` (component) | `multi_feature/types.rs` (~710562) | `radarProperties` → `baseSignatureParams.signatures[]` |
| `ESignatureType` (signal channel index) | `enums.rs:32762` | index 4 = `Resource` |

Seeded record types (queryable via `Datacore::resolve` / `records_by_type`):
`ResourceType`, `ResourceTypeGroup`, `ResourceTypeDatabase`, `MineableElement`,
`MineableComposition`, `MiningGlobalParams`, `HarvestablePreset`,
`HarvestableProviderPreset`, `HarvestableClusterPreset`, `HarvestableSetup`,
`VehicleSalvageGlobalParams`.

### Live counts (SC 1.0 / 4.8)

| Record type | Count |
|---|--:|
| `ResourceType` | 206 |
| `MineableElement` | 46 |
| `MineableComposition` | 249 |
| `MiningGlobalParams` | 7 |
| `HarvestablePreset` | 571 |
| `HarvestableProviderPreset` | 49 (one per body) |
| `*Debris*` record types | 0 (debris are entity classes, not their own record type) |

## Verification & reproduction

The data corpora live under the gitignored `target/probe-resources/`:

- `p4k_listing.txt` — full 1,347,571-entry p4k index (`path<TAB>size`).
- `dcbraw/` — the DataCore exported to 60,309 record XMLs. **Caveat:** the
  exporter collapses records sharing a DCB file path, so
  `resourcetypedatabase/resourcetypedatabase.xml` retains only one `ResourceType`
  — dump per-record via `svarog dcb-extract -f resourcetypedatabase` when you
  need them individually.
- `oc/` — every Stanton + Nyx object container, `--expand-socpak`.

Tooling: rebuild `E:\repros\Svarog\target\release\svarog.exe` (the pinned rev
supports DCB v8). `p4k-extract --filter "*Game2.dcb"` exports all records to
XML; `--expand-socpak` unpacks object containers; the `svarog` CLI's
`cryxml-convert` reads CryXmlB but **not** CrChF `.soc` (it only matches a
direct `CryXmlB\0` magic at offset 0). The body→provider join from code is the
`soc_harvestable_probe` example:

```bash
# scan the probe dir; assert Clio's provider GUID is present (exit 1 if not):
cargo run -p sc-extract --example soc_harvestable_probe -- \
  --expect 703a18ca-7f7c-4489-a64a-cd0cd359b8fe
```

It peels the CrCh chunk table itself and feeds the `CRYXMLB` chunk to
`svarog_cryxml` — so the full path (DCB provider records via `sc-extract` +
body join via the `.soc` scan) is reproducible from code. To extend coverage,
expand more bodies' socpaks into `target/probe-resources/` and re-run.

The `StarMapObject` hierarchy (and the kind tally per system) is reproduced by
`crates/sc-locations/examples/dump_location_tree.rs`, which hydrates the loose
`dcbfile/Data/Game2.dcb` via an in-memory `AssetSource::from_snapshot` and walks
the typed `Locations`. The system-OC `(starmapRecord, objectContainer)` bridge
was read directly from the expanded **Stanton** system OC
(`oc/…/stanton/stantonsystem/stantonsystem/entdata/*.entxml`, placement entities
with `EntityClass="OrbitingObjectContainer"`). The **Pyro** system OC is not in
the corpus (Pyro was not expanded), so the Pyro bridge is reasoned, not yet
verified — expand `pyrosystem.socpak` to confirm.
