# sc-crafting — design (future-proofing pass, v0.9.0)

Draft, 2026-05-30. Captures the live-DCB findings driving the post-v0.8.0
sc-crafting expansion. Trigger: Hearth needs recipe-level access for the
"what can I craft right now?" query, and crafting is a young CIG feature
whose schema is much wider than what's currently populated. Goal: pick a
shape that fits today's data AND survives the next CIG expansion (more
tiers, populated Refining/Dismantle records, populated results) without
a v1.0 break.

Decision: **sc-crafting will own the entire `libs/foundry/records/crafting/`
subsystem** (minus the legacy salvage recipes, deliberately skipped).
Plus a new tiny `sc-resources` T1 crate is spun up to model
`ResourceType`, because every `Cost_Resource` references one, the quality
quantization records key per-resource, and refining is currently encoded
as a `ResourceType.refined_version` pointer.

## DCB digs (committed in tree)

| Example                                 | Purpose |
| --------------------------------------- | ------- |
| `examples/recipe_census.rs`             | Walks every blueprint → tier → recipe → cost/result/research; tallies process/tier/cost/result variant distributions; raw-counts dormant types via svarog; resource GUID type probe. |
| `examples/crafting_tree.rs`             | Uses `RecordPaths` to map every subdirectory + file under `libs/foundry/records/crafting/`; deep-dumps `craftingglobalparams.xml`. |
| `examples/category_probe.rs`            | Reads the raw DCB schema for `BlueprintCategoryRecord` / `BlueprintCategoryDatabaseRecord` / `CraftingGameplayPropertyDef` to establish what fields exist (vs what the generator emitted). |

All three are read-only digs; safe to re-run.

## Subsystem map under `libs/foundry/records/crafting/`

| Subdir                  | Records | Type(s)                                                            | Notes |
| ----------------------- | ------: | ------------------------------------------------------------------ | ----- |
| `blueprints/`           |    1561 | `CraftingBlueprintRecord`                                          | The catalog (current `all_blueprints` source). |
| `blueprintrewards/`     |     116 | `BlueprintPoolRecord`                                              | Mission reward pools — **already owned by sc-missions**, not in scope here. |
| `blueprintcategories/`  |      21 | `BlueprintCategoryRecord` (×20) + `BlueprintCategoryDatabaseRecord` (×1) | Categories are **marker records** — schema `attribute_count = 0`, struct_size = 0. Semantic content = the record's *name* (`"BlueprintCategoryRecord.RefiningExample1"`, `"…FPSWeapons"`, `"…VehicleWeaponsS1-6"`, `"…Medical"`). Probe confirmed: not a generator bug. The Database record carries a `Vec<Reference>` of all category GUIDs. |
| `craftedproperties/`    |      29 | `CraftingGameplayPropertyDef`                                      | Defines the gameplay properties that crafting effects can modify. Fields: `propertyName` (locale-key), `displayTransformation` (enum), `unitFormat` (locale-key), `nameOverrides: Vec<Class>`. The menu the (currently empty) `CraftingOptionalEntry.effect` slots will draw from. |
| `globalparams/`         |       1 | `CraftingGlobalParams` (singleton)                                 | Knobs: `refiningQualityUnitMultiplier: f32 = 2.0` · `defaultCompositionQuality: i32 = 500` · `dismantleBlacklistResources: Vec<Ref→ResourceType>` (6 entries) · `dismantleBlacklistEntityClasses: Vec<Ref→EntityClassDefinition>` (2 entries) · `defaultBlueprintSelection: StrongPtr→DefaultBlueprintSelection_Whitelist`. **Proves Refining + Dismantle are live mechanics even without per-blueprint records.** |
| `qualitydistribution/`  |      22 | `CraftingQualityDistributionRecord` (×10) + `CraftingQualityLocationOverrideRecord` (×12) | Per-source quality curves — `Creatures` / `FPSMineables` / `GroundMineables` / `Harvestables` / `ShipMineables` × `Common`/`Uncommon`/`Rare`/`Epic`/`Legendary` — with **location overrides** for Pyro / RCD / Torite. |
| `qualityquantization/`  |      38 | `CraftingQualityQuantizationRecord`                                | Per-resource quality→tier discretization (`Quantization_Aluminum`, `Quantization_Gold`, `Quantization_Quantainium`, …). Bridges continuous quality and discrete blueprint tier. |
| `legacy/`               |      39 | `LegacyCraftingRecipeDefRecord` (×34) + `LegacyCraftingRecipeListRecord` (×5) | **Out of scope by decision.** This is the legacy multitool salvage/repair recipe system (`SalvageFillerStation_Small`, `recipe_ammunition_grin_multitool_*`, `recipe_subitem_fuse_*`). Coexists with the new crafting in the DCB; might one day become a `sc-salvage` crate but not now. |

## Live data shape findings (from `recipe_census.rs` over SC 4.8 LIVE)

| Aspect                  | Schema                                          | Live 4.8                                                          |
| ----------------------- | ----------------------------------------------- | ----------------------------------------------------------------- |
| Process variants        | Creation/Refining/Repair/Upgrade/Dismantle      | **Creation only** (1560/1560; 0 records of the others)            |
| Tier count per blueprint| Vec                                             | **always 1** (CIG plans more tiers — confirmed by user)           |
| Recipe `Ref` / `RecordRef` | yes                                          | 0 records — every recipe inlined                                  |
| Mandatory cost types    | Resource / Item / Select                         | **`Select(Select(Resource))`** universally; 0 Item, 0 top-level Resource |
| Optional costs          | `Vec<{cost, effect}>`                            | 0 entries                                                         |
| Results                 | `Vec<Item \| Resource>`                          | 36 inline containers, all with empty `results` vec                |
| Research                | `{ unlock_requirements, research_costs }`        | 883/1560 tiers (57%) carry `CraftingResearch`; both inner Options universally None |
| Resources referenced    | external `CigGuid`                               | 26 unique GUIDs, all `ResourceType`, all in `resourcetypedatabase.xml` |

## Why the future-proofing matters

User confirmed:
> currently only T1 is available, but more are planned. Also refining
> should get some updates.

So the modelled-but-empty schema corners — `tiers: Vec` always 1, `results: Vec` always empty, dormant `CraftingProcess_Refining`/`_Repair`/`_Upgrade`/`_Dismantle` with 0 records, the `Research` slot present-but-empty, `CraftingOptionalEntry.effect` with 0 records — are exactly where CIG is going to populate next. Modelling the schema shape now (Vec stays Vec, Option stays Option, polymorphic enums get all real variants + `Other` fallback) makes the data-population a no-op for consumers; modelling only the populated subset would force a breaking change every time CIG ships content.

## Feature-gating reality (drives the `Other { type_name }` strategy)

Several crafting types live in the `dormant` feature, not `crafting`:

- `CraftingProcess_{Refining, Repair, Upgrade, Dismantle, Base_NonRef}`
- `CraftingResult_Item`, `CraftingResult_Resource`
- `CraftingProcessSpecificRecipeData_Refining`
- `CraftingRecipe_{Ref, RecordRef}`
- `CraftingRecipeCosts_{Ref, RecordRef}`
- `CraftingRecipeResults_{Ref, RecordRef}`
- `CraftingResearchUnlock` (concrete), `CraftingCost_{Ref, RecordRef}`

Under `crafting` alone, the typed enum dispatch for these falls through to
`Unknown { struct_index, instance_index }`. **0 records of any in 4.8
LIVE** so the runtime impact is zero today. When CIG populates them, the
next `regenerate.ps1 -Publish` auto-promotes them out of `dormant`
(classifier is records-populated-driven) — no manual flag flip needed.

**So: we don't enable `dormant`** on sc-crafting's sc-extract dep. Every
polymorphic enum gets an `Other { type_name, struct_index }` fallback to
preserve diagnostic shape until the regen-after-population promotes the
type into the typed enum surface.

## Proposed crate split

```
sc-resources      (NEW T1, ~50 LoC)
    │ Resources: ResourceType pool, refined_version graph
    ▼
sc-crafting       (BIG, ~600-800 LoC)
    │ Blueprints, Categories, GlobalParams, GameplayProperties, Quality{Distribution,LocationOverride,Quantization}
    │ Builds on sc-items (for ItemCost name baking + Creation entity name) and sc-resources
    │ Legacy/* deliberately not modelled
    ▼
sc-missions       (existing — owns BlueprintPoolRecord via blueprint_pools.rs)
```

## sc-resources (new tiny crate)

`ResourceType` is gated under `crafting` (among many) — reachable without
dormant. Tiny: 26 records in use, but the pool may hold a few more
(check at build time).

```rust
pub struct Resources {
    by_guid: HashMap<Guid, Resource>,
}

pub struct Resource {
    pub guid: Guid,
    pub name_key: LocaleKey,                // display_name from DCB
    pub description_key: LocaleKey,
    pub default_thumbnail_path: String,
    pub default_thumbnail_path_svg: String,
    pub density_type: Option<DensityType>,  // see open question
    pub properties: Vec<ResourceProperty>,  // ResourceTypeProperties — open: dig before locking
    pub refined_version: Option<Guid>,      // refining mechanism — raw → refined pointer
    pub rtt_thumbnail_entity_class: Option<Guid>,
    pub default_cargo_containers: Option<Guid>,
}

impl Resources {
    pub fn build(store: &RecordStore) -> Self;
    pub fn get(&self, guid: &Guid) -> Option<&Resource>;
    pub fn iter(&self) -> impl Iterator<Item = &Resource>;
    pub fn refined_version_of(&self, guid: &Guid) -> Option<&Resource>;
}
```

Implements `RecordVisitor` (`Interest::Types(["ResourceType"])`) — joins
the foundational bundled walk in sc-holotable.

## sc-crafting type surface

Rule-6 conformant. Names draft; settle in implementation. All Vecs and
Options match the schema, even when degenerate today.

```rust
// === primary catalog ===

pub struct Blueprints {
    entries: Vec<Blueprint>,
    by_record_guid: HashMap<Guid, usize>,   // index into entries
    by_category: HashMap<Guid, Vec<usize>>, // optional reverse index for UI grouping
}

impl Blueprints {
    pub fn build(datacore: &Datacore, items: &Items) -> Self;
    pub fn iter(&self) -> impl Iterator<Item = &Blueprint>;
    pub fn get(&self, record_guid: Guid) -> Option<&Blueprint>;
    pub fn in_category(&self, category_guid: Guid) -> impl Iterator<Item = &Blueprint>;
}

pub struct Blueprint {
    pub blueprint_record_guid: Guid,
    pub category: Option<Guid>,             // → BlueprintCategoryRecord (marker)
    pub process: Process,
    pub name_key: Option<LocaleKey>,        // blueprintName; may be placeholder
    pub tiers: Vec<Tier>,                   // schema Vec; today always len=1
}

impl Blueprint {
    pub fn crafted_entity_guid(&self) -> Option<Guid>;
    pub fn entity_name_key(&self) -> Option<&LocaleKey>;
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str>;
}

pub enum Process {
    Creation { entity_class: Option<Guid> },
    /// Dormant variants surface here until the next regen-after-population.
    Other { type_name: String, struct_index: u32 },
}

pub struct Tier {
    pub recipe: Option<Recipe>,
    pub research: Option<Research>,
}

pub struct Recipe {
    pub craft_time: Option<TimeValue>,       // open: TimeValue shape probe
    pub costs: Option<RecipeCosts>,
    pub results: Vec<RecipeResult>,          // empty today; schema vec
    pub is_shared: bool,                     // false today (always inline)
}

pub struct RecipeCosts {
    pub mandatory: Option<Cost>,
    pub optional: Vec<OptionalCost>,         // empty today
}

pub struct OptionalCost {
    pub cost: Option<Cost>,
    pub effect_kind: Option<OptionalEffectKind>,
}

pub enum OptionalEffectKind {
    Time,                                    // CraftingOptionEffect_Time
    Other(String),
}

/// The polymorphic ingredient tree. Today every cost shapes as
/// `Select { N, [Select { 1, [Resource] }] }` — pick N groups, each
/// containing one resource alternative.
pub enum Cost {
    Resource(ResourceCost),
    Item(ItemCost),                          // schema; 0 records today
    Select { count: i32, options: Vec<Cost> },
}

pub struct ResourceCost {
    pub resource: Option<Guid>,              // → ResourceType (sc-resources resolves)
    pub quantity: Option<CargoUnit>,         // SBaseCargoUnit — open question
    pub min_quality: i32,
}

pub struct ItemCost {
    pub entity_class: Option<Guid>,
    pub quantity: i32,
    pub min_quality: i32,
}

pub enum RecipeResult {
    Item { entity_class: Option<Guid>, quantity: i32, tier: i32 },
    Resource { resource: Option<Guid>, quantity: Option<CargoUnit> },
    Other { type_name: String, struct_index: u32 },
}

pub struct Research {
    pub unlock: Option<ResearchUnlock>,
    pub costs: Option<RecipeCosts>,
}

pub enum ResearchUnlock {
    Default,                                 // CraftingResearchUnlock_Base
    Other { type_name: String, struct_index: u32 },
}

// === categories ===

pub struct Categories {
    by_guid: HashMap<Guid, Category>,
    database_guid: Option<Guid>,             // BlueprintCategoryDatabaseRecord (×1)
}

pub struct Category {
    pub guid: Guid,
    pub name: String,                        // svarog record name; the only payload
    pub path: String,                        // RecordPath file (rarely useful)
}

impl Categories {
    pub fn build(paths: &RecordPaths) -> Self; // marker records → RecordPaths is enough
    pub fn get(&self, guid: &Guid) -> Option<&Category>;
    pub fn iter(&self) -> impl Iterator<Item = &Category>;
}

// === global params (singleton) ===

pub struct GlobalParams {
    pub refining_quality_unit_multiplier: f32,
    pub default_composition_quality: i32,
    pub dismantle_blacklist_resources: Vec<Guid>,   // → ResourceType
    pub dismantle_blacklist_entity_classes: Vec<Guid>, // → EntityClassDefinition
    pub default_blueprint_selection: BlueprintSelection,
}

pub enum BlueprintSelection {
    Whitelist(/* TBD list shape — DefaultBlueprintSelection_Whitelist */),
    Other { type_name: String, struct_index: u32 },
}

impl GlobalParams {
    pub fn build(datacore: &Datacore) -> Option<Self>; // singleton; None if absent
}

// === gameplay properties (definitions) ===

pub struct GameplayProperties {
    by_guid: HashMap<Guid, GameplayProperty>,
}

pub struct GameplayProperty {
    pub guid: Guid,
    pub property_name_key: LocaleKey,        // "@StatName_..."
    pub unit_format_key: LocaleKey,          // "@LOC_..."
    pub display_transformation: DisplayTransformation,
    pub name_overrides: Vec<NameOverride>,   // Class array; open shape
}

pub enum DisplayTransformation {
    Variant(String),                         // open: probe the enum values
}

// === quality (3 typed surfaces, sharing one bundled walk) ===

pub struct Quality {
    pub distribution: Distribution,
    pub location_overrides: LocationOverrides,
    pub quantization: Quantization,
}

pub struct Distribution {
    by_guid: HashMap<Guid, DistributionEntry>,
}
pub struct DistributionEntry { /* CraftingQualityDistributionRecord — open dig */ }

pub struct LocationOverrides {
    by_guid: HashMap<Guid, LocationOverrideEntry>,
}
pub struct LocationOverrideEntry { /* CraftingQualityLocationOverrideRecord — open dig */ }

pub struct Quantization {
    by_resource: HashMap<Guid, QuantizationEntry>, // keyed by resource GUID
}
pub struct QuantizationEntry { /* CraftingQualityQuantizationRecord — open dig */ }

impl Quality {
    pub fn build(datacore: &Datacore) -> Self;
}
```

## Open questions to resolve in implementation

Quick digs before locking each type:

- **`TimeValue` and `SBaseCargoUnit` shapes** — polymorphic ptrs. Probe
  which variants are populated; decide whether to model a typed
  `Duration` / `CargoQuantity` or pass the discriminator through.
- **`ResourceTypeProperties`** — `Resource.properties: Vec<ResourceProperty>`
  shape; what variants exist?
- **`DensityType`** — `ResourceTypeDensityType` core + variants in
  multi_feature/dormant; minimal model.
- **`DisplayTransformation` enum values** — the gameplay-property
  `displayTransformation` field returned discriminant 17 in a sample;
  enumerate.
- **`DefaultBlueprintSelection_Whitelist`** — what does the whitelist
  list? GUIDs of unlocked blueprints? Verify shape.
- **`NameOverride`** class array on `CraftingGameplayPropertyDef` —
  empty in the sample; probe a non-empty case if one exists.
- **`CraftingQualityDistributionRecord` / `_LocationOverrideRecord` /
  `_QuantizationRecord`** — three separate digs to see their fields and
  any cross-references (probably location overrides reference
  distributions; quantization records likely reference a `ResourceType`).
- **Confirm sc-resources visible record count** — census saw 26 unique
  resources *used by crafting*; the pool itself may hold more (e.g. raw
  ores not yet referenced by any blueprint).

Each is a small RecordPaths-driven dig (5-15 min). Do all upfront before
writing the typed builders so the types match the data.

## Implementation phasing (committable)

1. **`sc-resources`** (~50 LoC + 1 example): `Resources::build`, types,
   minimal test. First because everyone downstream depends on it.
2. **Open-question digs** — extend `crafting_tree.rs` or add small
   one-off probes to resolve each open question. Commit the digs.
3. **sc-crafting core** — `Blueprints`/`Blueprint`/`Process`/`Tier`/
   `Recipe`/`Cost`/`Research`. Keeps the `all_blueprints` shim alive.
   Validate against the census numbers.
4. **sc-crafting categories + global params + gameplay properties** —
   three small types in one pass.
5. **sc-crafting Quality** — `Distribution` + `LocationOverrides` +
   `Quantization`.
6. **Bundled-walk integration** — `ResourcesBuilder` joins the
   foundational walk in sc-holotable. Decide whether
   `BlueprintsBuilder` is worth pooling with the foundations or stays a
   user-call (it depends on `&Items` + `&Resources`, so probably a
   phase-1 dependent).
7. **`sc-bench` + `sc-holotable` umbrella** — add the new fields to
   `Foundations` (if any), expose under the umbrella prelude.
8. **Hearth repoint** — gain recipe-level cost access, switch resource
   lookups to `Resources::get`, surface category-based UI grouping.
9. **Release `sc-holotable/v0.9.0`**.

## Why this scope

- "All process variants" answer (from the user) doesn't apply per-blueprint
  — DCB has 0 records of non-Creation. But it DOES apply to the surrounding
  subsystem: refining is encoded via `ResourceType.refined_version` AND
  `GlobalParams.refining_quality_unit_multiplier` AND a
  `RefiningExample1` category; dismantle is encoded via
  `GlobalParams.dismantleBlacklist*` + a `DismantleExample1` category.
  Owning the whole `crafting/` subsystem captures that.
- Quality is its own real machinery (60 records across 3 types). Folding
  it into sc-crafting matches the DCB folder layout and avoids a
  premature crate split.
- sc-resources is genuinely needed once we touch `Cost_Resource` and
  `Quantization` keyed on resource GUIDs — punting it would force
  consumers to thread a separate lookup.
- Legacy salvage/repair is a distinct gameplay loop with a distinct
  schema; modelling it would double the surface for marginal value to
  Hearth. Note the discovery, skip it.
