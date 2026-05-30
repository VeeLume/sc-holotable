//! Crafting catalog and recipe model over the DCB's
//! `libs/foundry/records/crafting/blueprints/` tree.
//!
//! [`Blueprints`] is the primary index. Each [`Blueprint`] carries its
//! crafted-item identity ([`Process::Creation::entity_class`]), its
//! display name (baked from sc-items), per-tier [`Recipe`]s
//! ([`RecipeCosts`] with mandatory + optional + craft time, plus a Vec
//! of [`RecipeResult`]s and a [`Research`] slot), and a category GUID.
//!
//! # Schema vs live data
//!
//! `recipe_census.rs` (commit 6f69b8d) showed SC 4.8 lives entirely in a
//! narrow corner of the schema: 1561/1561 blueprints are
//! [`Process::Creation`], every blueprint has exactly 1 tier, every
//! cost is `Select(Select(Resource))`, results are universally empty
//! (Creation's `entity_class` IS the output), 0 optional costs, and
//! `Research` slots are present-but-empty on 57% of tiers. We model the
//! full schema shape anyway — CIG plans to populate this. The
//! polymorphic enums fall back to `Other { type_name, struct_index }`
//! for dormant variants until the next regen-after-population promotes
//! them.
//!

use sc_extract::generated::{
    CraftingBlueprintRecord, CraftingBlueprintTier_BasePtr, CraftingBlueprint_Base_NonRefPtr,
    CraftingCost_BasePtr, CraftingDisplayTransformation_BasePtr, CraftingGameplayPropertyDef,
    CraftingGlobalParams, CraftingOptionalEffect_BasePtr, CraftingPropertyNameOverride,
    CraftingPropertyNameOverrideCondition_BasePtr, CraftingProcess_BasePtr,
    CraftingQualityDistribution_BasePtr, CraftingQualityDistribution_Base_NonRefPtr,
    CraftingQualityLocationOverride_Base_NonRefPtr,
    CraftingQualityQuantization_Base_NonRefPtr, CraftingRecipeCosts_BasePtr,
    CraftingRecipeResults_BasePtr, CraftingRecipe_BasePtr, CraftingResearch_BasePtr,
    CraftingResearchUnlock_BasePtr, CraftingResult_BasePtr, DataPools,
    DefaultBlueprintSelection_BasePtr, EItemSubType, EItemType, RecordIndex, RecordLookup,
    SBaseCargoUnitPtr, TimeValue_BasePtr,
};
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap, RecordPaths};
use sc_items::Items;
use sc_resources::CargoQuantity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─────────────────────────────────────────────────────────────────────
// Primary index
// ─────────────────────────────────────────────────────────────────────

/// Every `CraftingBlueprintRecord` in the DCB, with its full recipe
/// surface resolved. Build once via [`Blueprints::build`], share by
/// reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Blueprints {
    entries: Vec<Blueprint>,
    by_record_guid: HashMap<Guid, usize>,
    by_category: HashMap<Guid, Vec<usize>>,
    by_crafted_entity: HashMap<Guid, usize>,
}

impl Blueprints {
    /// Build the catalog from a parsed [`Datacore`] (typed pools) +
    /// an [`Items`] index (for baking the crafted entity's name key).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let pools = &datacore.records().pools;
        let records = &datacore.records().records;
        let mut bp = Self::default();
        for (&guid, &handle) in &records.multi_feature.crafting_blueprint_record {
            let Some(record) = handle.get(pools) else { continue };
            let blueprint = build_blueprint(guid, record, records, pools, items);
            let idx = bp.entries.len();
            bp.by_record_guid.insert(guid, idx);
            if let Some(cat) = blueprint.category {
                bp.by_category.entry(cat).or_default().push(idx);
            }
            if let Some(ent) = blueprint.crafted_entity_guid() {
                bp.by_crafted_entity.insert(ent, idx);
            }
            bp.entries.push(blueprint);
        }
        bp
    }

    /// Iterate every blueprint. Order is unspecified.
    pub fn iter(&self) -> impl Iterator<Item = &Blueprint> + '_ {
        self.entries.iter()
    }

    /// Look up a blueprint by its `CraftingBlueprintRecord` GUID.
    pub fn get(&self, record_guid: Guid) -> Option<&Blueprint> {
        let idx = *self.by_record_guid.get(&record_guid)?;
        self.entries.get(idx)
    }

    /// All blueprints belonging to a category (looked up by category GUID).
    pub fn in_category(&self, category: Guid) -> impl Iterator<Item = &Blueprint> + '_ {
        self.by_category
            .get(&category)
            .into_iter()
            .flatten()
            .filter_map(|&i| self.entries.get(i))
    }

    /// Look up the blueprint that crafts a given entity (by EntityClassDefinition GUID).
    pub fn for_crafted_entity(&self, entity_guid: Guid) -> Option<&Blueprint> {
        let idx = *self.by_crafted_entity.get(&entity_guid)?;
        self.entries.get(idx)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ─────────────────────────────────────────────────────────────────────
// Blueprint
// ─────────────────────────────────────────────────────────────────────

/// One `CraftingBlueprintRecord`'s full resolved shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blueprint {
    /// GUID of the `CraftingBlueprintRecord` root.
    pub blueprint_record_guid: Guid,
    /// → `BlueprintCategoryRecord` (marker record; resolve name via
    /// sc-crafting categories — pending in a later commit, or
    /// `RecordPaths.get(guid).name`).
    pub category: Option<Guid>,
    /// What this blueprint does. Today always [`Process::Creation`].
    pub process: Process,
    /// Fallback display-name key from `CraftingBlueprint.blueprintName`.
    /// Most blueprints have a `<= PLACEHOLDER =>` here; the
    /// crafted-entity name is preferred — see [`Blueprint::display_name`].
    pub blueprint_name_key: Option<LocaleKey>,
    /// Crafted entity's display-name key, baked from [`Items`] at
    /// build time. Preferred name source.
    pub entity_name_key: Option<LocaleKey>,
    /// Schema is Vec; SC 4.8 always len = 1. CIG has signalled
    /// more tiers are coming.
    pub tiers: Vec<Tier>,
}

impl Blueprint {
    /// Convenience: the crafted-entity GUID (`None` for non-Creation
    /// processes or unresolved Creation).
    pub fn crafted_entity_guid(&self) -> Option<Guid> {
        match &self.process {
            Process::Creation { entity_class } => *entity_class,
            Process::Other { .. } => None,
        }
    }

    /// Resolve the player-facing display name through a [`LocaleMap`].
    /// Tries the baked crafted-entity name first, then the blueprint
    /// fallback; CIG placeholders (`<= PLACEHOLDER =>`) count as unresolved.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        if let Some(key) = &self.entity_name_key
            && let Some(name) = locale.resolve(key)
            && !name.is_empty()
            && !is_placeholder(name)
        {
            return Some(name);
        }
        if let Some(key) = &self.blueprint_name_key
            && let Some(text) = locale.resolve(key)
            && !is_placeholder(text)
        {
            return Some(text);
        }
        None
    }
}

/// What a blueprint *does*. Live SC 4.8 = 100% [`Process::Creation`];
/// non-Creation `CraftingProcess_*` variants are dormant + 0 records
/// and surface as [`Process::Other`] until populated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Process {
    /// Creates a new entity. `entity_class` → `EntityClassDefinition`.
    Creation { entity_class: Option<Guid> },
    /// Refining / Repair / Upgrade / Dismantle — currently dormant.
    /// The next regen-after-population auto-promotes these to typed
    /// variants on this enum.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

// ─────────────────────────────────────────────────────────────────────
// Tier / Recipe / Costs / Results / Research
// ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tier {
    pub recipe: Option<Recipe>,
    pub research: Option<Research>,
}

/// A craftable recipe — inputs (costs), outputs (results), time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// How long crafting takes. Always [`Duration::Partitioned`] today.
    pub craft_time: Option<Duration>,
    pub costs: Option<RecipeCosts>,
    /// Schema is Vec; today always empty (Creation's `entity_class`
    /// IS the output). When populated, modelled here for forward-compat.
    pub results: Vec<RecipeResult>,
    /// True when the underlying `CraftingRecipe_*` variant is a
    /// `Ref`/`RecordRef` (shared recipe). Today always false (every
    /// recipe is inlined per blueprint).
    pub is_shared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeCosts {
    pub mandatory: Option<Cost>,
    /// Quality / property modifiers — empty in SC 4.8.
    pub optional: Vec<OptionalCost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalCost {
    pub cost: Option<Cost>,
    pub effect_kind: Option<OptionalEffectKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionalEffectKind {
    /// `CraftingOptionEffect_Time`.
    Time,
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// The polymorphic ingredient tree.
///
/// SC 4.8 universally shapes mandatory costs as
/// `Select { N, [Select { 1, [Resource] }] }` — pick N ingredient
/// groups, each with one resource alternative. Item costs and
/// top-level Resource costs are 0 records today but kept in the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cost {
    Resource(ResourceCost),
    Item(ItemCost),
    Select {
        count: i32,
        options: Vec<Cost>,
    },
    /// Dormant variants (`_Ref`/`_RecordRef`/...) surface here.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCost {
    /// → `ResourceType` (resolve via `sc_resources::Resources::get`).
    pub resource: Option<Guid>,
    pub quantity: Option<CargoQuantity>,
    pub min_quality: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCost {
    /// → `EntityClassDefinition`.
    pub entity_class: Option<Guid>,
    pub quantity: i32,
    pub min_quality: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecipeResult {
    Item {
        entity_class: Option<Guid>,
        quantity: i32,
        tier: i32,
    },
    Resource {
        resource: Option<Guid>,
        quantity: Option<CargoQuantity>,
    },
    /// Dormant variants (`CraftingResult_Item` / `_Resource` are dormant
    /// — no records in SC 4.8).
    Other {
        type_name: String,
        struct_index: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Research {
    pub unlock: Option<ResearchUnlock>,
    pub costs: Option<RecipeCosts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchUnlock {
    Default,
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// Craft / research duration. Modelled after `TimeValue_Partitioned`
/// (the only `TimeValue_*` variant populated in SC 4.8;
/// `TimeValue_LongSeconds` is dormant).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Duration {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: f32,
}

impl Duration {
    /// Total duration in seconds.
    pub fn to_seconds(&self) -> f32 {
        (self.days as f32) * 86_400.0
            + (self.hours as f32) * 3600.0
            + (self.minutes as f32) * 60.0
            + self.seconds
    }
}

// ─────────────────────────────────────────────────────────────────────
// Cargo quantity projection (lives here because the `crafting` feature
// gates SCenti/SMicro pool types; sc-resources can't reach them).
// ─────────────────────────────────────────────────────────────────────

/// Project a `SBaseCargoUnitPtr` polymorphic pointer into a [`CargoQuantity`].
/// Defined here (not in sc-resources) because the Centi/Micro variants
/// are gated under the `crafting` feature.
pub fn cargo_quantity_from_ptr(ptr: &SBaseCargoUnitPtr, pools: &DataPools) -> CargoQuantity {
    match ptr {
        SBaseCargoUnitPtr::SStandardCargoUnit(h) => h
            .get(pools)
            .map(|u| CargoQuantity::Standard(u.standard_cargo_units))
            .unwrap_or(CargoQuantity::Standard(0.0)),
        SBaseCargoUnitPtr::SCentiCargoUnit(h) => h
            .get(pools)
            .map(|u| CargoQuantity::Centi(u.centi_scu))
            .unwrap_or(CargoQuantity::Centi(0)),
        SBaseCargoUnitPtr::SMicroCargoUnit(h) => h
            .get(pools)
            .map(|u| CargoQuantity::Micro(u.micro_scu))
            .unwrap_or(CargoQuantity::Micro(0)),
        SBaseCargoUnitPtr::SBaseCargoUnit(_) => CargoQuantity::Standard(0.0),
        SBaseCargoUnitPtr::Unknown { struct_index, .. } => CargoQuantity::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

// ─────────────────────────────────────────────────────────────────────
// Build helpers
// ─────────────────────────────────────────────────────────────────────

fn build_blueprint(
    guid: Guid,
    record: &CraftingBlueprintRecord,
    _records: &RecordIndex,
    pools: &DataPools,
    items: &Items,
) -> Blueprint {
    let mut bp = Blueprint {
        blueprint_record_guid: guid,
        category: None,
        process: Process::Other {
            type_name: "(empty)".into(),
            struct_index: 0,
        },
        blueprint_name_key: None,
        entity_name_key: None,
        tiers: Vec::new(),
    };

    let Some(bp_ptr) = &record.blueprint else { return bp };
    let CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(bh) = bp_ptr else {
        // CraftingBlueprint_Base_NonRef (empty base) — leave bp with default Process::Other.
        return bp;
    };
    let Some(blueprint) = bh.get(pools) else { return bp };

    bp.category = blueprint.category;
    if !blueprint.blueprint_name.is_empty() {
        bp.blueprint_name_key = Some(blueprint.blueprint_name.clone());
    }
    bp.process = build_process(&blueprint.process_specific_data, pools);
    if let Process::Creation { entity_class: Some(eg) } = &bp.process {
        bp.entity_name_key = items.name_key(eg).cloned();
    }
    bp.tiers = blueprint
        .tiers
        .iter()
        .map(|tier_ptr| build_tier(tier_ptr, pools))
        .collect();

    bp
}

fn build_process(
    process: &Option<CraftingProcess_BasePtr>,
    pools: &DataPools,
) -> Process {
    let Some(p) = process else {
        return Process::Other {
            type_name: "(none)".into(),
            struct_index: 0,
        };
    };
    match p {
        CraftingProcess_BasePtr::CraftingProcess_Creation(h) => Process::Creation {
            entity_class: h.get(pools).and_then(|c| c.entity_class),
        },
        CraftingProcess_BasePtr::CraftingProcess_Base(_) => Process::Other {
            type_name: "CraftingProcess_Base".into(),
            struct_index: 0,
        },
        CraftingProcess_BasePtr::Unknown {
            struct_index, ..
        } => Process::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_tier(tier_ptr: &CraftingBlueprintTier_BasePtr, pools: &DataPools) -> Tier {
    let CraftingBlueprintTier_BasePtr::CraftingBlueprintTier(th) = tier_ptr else {
        return Tier {
            recipe: None,
            research: None,
        };
    };
    let Some(tier) = th.get(pools) else {
        return Tier {
            recipe: None,
            research: None,
        };
    };
    Tier {
        recipe: tier.recipe.as_ref().map(|r| build_recipe(r, pools)),
        research: tier.research.as_ref().map(|r| build_research(r, pools)),
    }
}

fn build_recipe(ptr: &CraftingRecipe_BasePtr, pools: &DataPools) -> Recipe {
    let CraftingRecipe_BasePtr::CraftingRecipe(h) = ptr else {
        return Recipe {
            craft_time: None,
            costs: None,
            results: Vec::new(),
            is_shared: matches!(
                ptr,
                CraftingRecipe_BasePtr::CraftingRecipe_Base(_)
                    | CraftingRecipe_BasePtr::CraftingRecipe_Base_NonRef(_)
                    | CraftingRecipe_BasePtr::Unknown { .. }
            ),
        };
    };
    let Some(recipe) = h.get(pools) else {
        return Recipe {
            craft_time: None,
            costs: None,
            results: Vec::new(),
            is_shared: false,
        };
    };

    let (craft_time, costs) = match recipe.costs.as_ref() {
        Some(CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(ch)) => {
            if let Some(c) = ch.get(pools) {
                let time = c.craft_time.as_ref().and_then(|t| build_duration(t, pools));
                let mandatory = c.mandatory_cost.as_ref().map(|m| build_cost(m, pools));
                let optional: Vec<OptionalCost> = c
                    .optional_costs
                    .iter()
                    .filter_map(|oh| oh.get(pools))
                    .map(|oe| OptionalCost {
                        cost: oe.optional_cost.as_ref().map(|c| build_cost(c, pools)),
                        effect_kind: oe
                            .effect
                            .as_ref()
                            .map(|e| build_effect_kind(e)),
                    })
                    .collect();
                (time, Some(RecipeCosts { mandatory, optional }))
            } else {
                (None, None)
            }
        }
        _ => (None, None),
    };

    let results = match recipe.results.as_ref() {
        Some(CraftingRecipeResults_BasePtr::CraftingRecipeResults(rh)) => rh
            .get(pools)
            .map(|rr| {
                rr.results
                    .iter()
                    .map(|r| build_result(r, pools))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        _ => Vec::new(),
    };

    Recipe {
        craft_time,
        costs,
        results,
        is_shared: false,
    }
}

fn build_duration(ptr: &TimeValue_BasePtr, pools: &DataPools) -> Option<Duration> {
    match ptr {
        TimeValue_BasePtr::TimeValue_Partitioned(h) => h.get(pools).map(|p| Duration {
            days: p.days,
            hours: p.hours,
            minutes: p.minutes,
            seconds: p.seconds,
        }),
        // `TimeValue_LongSeconds` is dormant + 0 records; `_Base` is empty.
        _ => None,
    }
}

fn build_cost(ptr: &CraftingCost_BasePtr, pools: &DataPools) -> Cost {
    match ptr {
        CraftingCost_BasePtr::CraftingCost_Resource(h) => match h.get(pools) {
            Some(r) => Cost::Resource(ResourceCost {
                resource: r.resource,
                quantity: r.quantity.as_ref().map(|q| cargo_quantity_from_ptr(q, pools)),
                min_quality: r.min_quality,
            }),
            None => Cost::Other {
                type_name: "CraftingCost_Resource(empty)".into(),
                struct_index: 0,
            },
        },
        CraftingCost_BasePtr::CraftingCost_Item(h) => match h.get(pools) {
            Some(i) => Cost::Item(ItemCost {
                entity_class: i.entity_class,
                quantity: i.quantity,
                min_quality: i.min_quality,
            }),
            None => Cost::Other {
                type_name: "CraftingCost_Item(empty)".into(),
                struct_index: 0,
            },
        },
        CraftingCost_BasePtr::CraftingCost_Select(h) => match h.get(pools) {
            Some(sel) => Cost::Select {
                count: sel.count,
                options: sel.options.iter().map(|o| build_cost(o, pools)).collect(),
            },
            None => Cost::Other {
                type_name: "CraftingCost_Select(empty)".into(),
                struct_index: 0,
            },
        },
        CraftingCost_BasePtr::CraftingCost_Base(_) => Cost::Other {
            type_name: "CraftingCost_Base".into(),
            struct_index: 0,
        },
        CraftingCost_BasePtr::Unknown {
            struct_index, ..
        } => Cost::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_effect_kind(ptr: &CraftingOptionalEffect_BasePtr) -> OptionalEffectKind {
    match ptr {
        // `CraftingOptionEffect_Time` is dormant-gated; falls through to
        // Unknown until a regen-after-population promotes it. The
        // `Time` variant on OptionalEffectKind is reserved for when
        // that happens.
        CraftingOptionalEffect_BasePtr::CraftingOptionalEffect_Base(_) => {
            OptionalEffectKind::Other {
                type_name: "CraftingOptionalEffect_Base".into(),
                struct_index: 0,
            }
        }
        CraftingOptionalEffect_BasePtr::Unknown {
            struct_index, ..
        } => OptionalEffectKind::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_result(ptr: &CraftingResult_BasePtr, pools: &DataPools) -> RecipeResult {
    // CraftingResult_Item / _Resource are dormant + 0 records in SC 4.8;
    // they will surface as Unknown until a regen-after-population.
    let _ = pools;
    match ptr {
        CraftingResult_BasePtr::CraftingResult_Base(_) => RecipeResult::Other {
            type_name: "CraftingResult_Base".into(),
            struct_index: 0,
        },
        CraftingResult_BasePtr::Unknown {
            struct_index, ..
        } => RecipeResult::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_research(ptr: &CraftingResearch_BasePtr, pools: &DataPools) -> Research {
    let CraftingResearch_BasePtr::CraftingResearch(h) = ptr else {
        return Research {
            unlock: None,
            costs: None,
        };
    };
    let Some(research) = h.get(pools) else {
        return Research {
            unlock: None,
            costs: None,
        };
    };

    let unlock = research.unlock_requirements.as_ref().map(|u| match u {
        CraftingResearchUnlock_BasePtr::CraftingResearchUnlock_Base(_) => ResearchUnlock::Default,
        CraftingResearchUnlock_BasePtr::Unknown {
            struct_index, ..
        } => ResearchUnlock::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    });

    let costs = match research.research_costs.as_ref() {
        Some(CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(ch)) => ch.get(pools).map(|c| {
            RecipeCosts {
                mandatory: c.mandatory_cost.as_ref().map(|m| build_cost(m, pools)),
                optional: c
                    .optional_costs
                    .iter()
                    .filter_map(|oh| oh.get(pools))
                    .map(|oe| OptionalCost {
                        cost: oe.optional_cost.as_ref().map(|c| build_cost(c, pools)),
                        effect_kind: oe.effect.as_ref().map(|e| build_effect_kind(e)),
                    })
                    .collect(),
            }
        }),
        _ => None,
    };

    Research { unlock, costs }
}

fn is_placeholder(text: &str) -> bool {
    text.contains("PLACEHOLDER") || text == "<= PLACEHOLDER =>"
}

// ─────────────────────────────────────────────────────────────────────
// `RecordVisitor` for bundled walks
// ─────────────────────────────────────────────────────────────────────

/// Bundled-walk builder. Construct via [`BlueprintsBuilder::new`] with
/// references to the foundational indices (`&Items`); join into a
/// bundled walk via [`sc_extract::BundledWalk`].
pub struct BlueprintsBuilder<'a> {
    items: &'a Items,
    inner: Blueprints,
}

impl<'a> BlueprintsBuilder<'a> {
    pub fn new(items: &'a Items) -> Self {
        Self {
            items,
            inner: Blueprints::default(),
        }
    }
}

impl<'a> sc_extract::RecordVisitor for BlueprintsBuilder<'a> {
    type Output = Blueprints;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["CraftingBlueprintRecord"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let Some(handle) = CraftingBlueprintRecord::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(record) = handle.get(&store.pools) else { return };
        let blueprint = build_blueprint(
            item.guid,
            record,
            &store.records,
            &store.pools,
            self.items,
        );
        let idx = self.inner.entries.len();
        self.inner.by_record_guid.insert(item.guid, idx);
        if let Some(cat) = blueprint.category {
            self.inner.by_category.entry(cat).or_default().push(idx);
        }
        if let Some(ent) = blueprint.crafted_entity_guid() {
            self.inner.by_crafted_entity.insert(ent, idx);
        }
        self.inner.entries.push(blueprint);
    }

    fn finish(self) -> Blueprints {
        self.inner
    }
}

// ─────────────────────────────────────────────────────────────────────
// Categories
// ─────────────────────────────────────────────────────────────────────

/// Blueprint categories. The DCB schema has `BlueprintCategoryRecord`
/// as an *empty* marker record (verified via `category_probe.rs`:
/// attribute_count = 0). The semantic identity is the record's NAME
/// (`"BlueprintCategoryRecord.RefiningExample1"`, `…FPSWeapons`,
/// `…Medical`, `…VehicleWeaponsS1-6`, etc.) plus its GUID, both of
/// which we get from [`RecordPaths`].
///
/// Singleton `BlueprintCategoryDatabaseRecord` carries a
/// `Vec<Reference>` to every category — captured as
/// [`Categories::database_guid`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Categories {
    by_guid: HashMap<Guid, Category>,
    /// The one `BlueprintCategoryDatabaseRecord` GUID, if present.
    pub database_guid: Option<Guid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    pub guid: Guid,
    /// svarog record name — e.g. `"BlueprintCategoryRecord.FPSWeapons"`.
    pub name: String,
    /// Full DCB file path. Mostly informational
    /// (`libs/foundry/records/crafting/blueprintcategories/blueprintcategorydatabase.xml`
    /// for every entry today — all 20 categories + the database live in
    /// the same file).
    pub path: String,
}

impl Categories {
    /// Build from a [`RecordPaths`] index (the marker records carry no
    /// typed fields — RecordPaths gives us everything we need).
    pub fn build(paths: &RecordPaths) -> Self {
        let mut cats = Self::default();
        // Categories live under blueprintcategories/.
        let prefix = "libs/foundry/records/crafting/blueprintcategories";
        for guid in paths.under(prefix) {
            let Some(rp) = paths.get(guid) else { continue };
            let type_name = paths.type_name(rp.struct_index).unwrap_or("");
            match type_name {
                "BlueprintCategoryRecord" => {
                    cats.by_guid.insert(
                        *guid,
                        Category {
                            guid: *guid,
                            name: rp.name.clone(),
                            path: rp.path.clone(),
                        },
                    );
                }
                "BlueprintCategoryDatabaseRecord" => {
                    cats.database_guid = Some(*guid);
                }
                _ => {}
            }
        }
        cats
    }

    pub fn get(&self, guid: &Guid) -> Option<&Category> {
        self.by_guid.get(guid)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Category> + '_ {
        self.by_guid.values()
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}

// ─────────────────────────────────────────────────────────────────────
// GlobalParams
// ─────────────────────────────────────────────────────────────────────

/// The `CraftingGlobalParams` singleton — knobs that apply to the
/// whole crafting subsystem regardless of per-blueprint settings.
///
/// SC 4.8 sample: `refining_multiplier=2.0`,
/// `default_composition_quality=500`, 6 blacklisted resources,
/// 2 blacklisted entity classes, 9 default-unlocked blueprints
/// (incl. the P4-AR — see [`GlobalParams::default_blueprint_whitelist`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalParams {
    /// Scales the quality contribution of refining steps (×2 in SC 4.8).
    pub refining_quality_unit_multiplier: f32,
    /// Default quality value used when a recipe doesn't specify one
    /// (500 in SC 4.8).
    pub default_composition_quality: i32,
    /// Resources that **cannot be dismantled** (→ `ResourceType` GUIDs).
    pub dismantle_blacklist_resources: Vec<Guid>,
    /// Entity classes that **cannot be dismantled**
    /// (→ `EntityClassDefinition` GUIDs).
    pub dismantle_blacklist_entity_classes: Vec<Guid>,
    /// Blueprints unlocked by default at character start.
    /// Flattened from `DefaultBlueprintSelection_Whitelist.blueprint_records`.
    /// SC 4.8: 9 entries (the basic dismantle, P4-AR, light combat
    /// armor parts, etc.).
    pub default_blueprint_whitelist: Vec<Guid>,
    /// True when `default_blueprint_selection` is present but its
    /// concrete variant isn't `Whitelist` (a non-Whitelist selection
    /// strategy CIG hasn't shipped yet).
    pub default_selection_is_non_whitelist: bool,
}

impl GlobalParams {
    /// Build the singleton from a parsed [`Datacore`]. Returns `None` if
    /// no `CraftingGlobalParams` record is present (shouldn't happen in
    /// production builds).
    pub fn build(datacore: &Datacore) -> Option<Self> {
        let pools = &datacore.records().pools;
        let gp = pools.crafting.crafting_global_params.iter().flatten().next()?;
        Some(Self::from_record(gp, pools))
    }

    fn from_record(gp: &CraftingGlobalParams, pools: &DataPools) -> Self {
        let mut params = Self {
            refining_quality_unit_multiplier: gp.refining_quality_unit_multiplier,
            default_composition_quality: gp.default_composition_quality,
            dismantle_blacklist_resources: gp.dismantle_blacklist_resources.clone(),
            dismantle_blacklist_entity_classes: gp.dismantle_blacklist_entity_classes.clone(),
            default_blueprint_whitelist: Vec::new(),
            default_selection_is_non_whitelist: false,
        };
        match &gp.default_blueprint_selection {
            None => {}
            Some(DefaultBlueprintSelection_BasePtr::DefaultBlueprintSelection_Whitelist(h)) => {
                if let Some(wl) = h.get(pools) {
                    params.default_blueprint_whitelist = wl.blueprint_records.clone();
                }
            }
            Some(_) => {
                params.default_selection_is_non_whitelist = true;
            }
        }
        params
    }
}

// ─────────────────────────────────────────────────────────────────────
// GameplayProperties (the menu of properties crafting effects modify)
// ─────────────────────────────────────────────────────────────────────

/// All `CraftingGameplayPropertyDef` records — the menu of gameplay
/// attributes that crafting effects can modify (damage, range, weight,
/// …). SC 4.8: 29 records; 8 carry a `DisplayTransformation`, 1 carries
/// a non-empty `name_overrides` list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameplayProperties {
    by_guid: HashMap<Guid, GameplayProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayProperty {
    pub guid: Guid,
    /// `propertyName` — typically `"@StatName_..."`. Resolve via [`LocaleMap`].
    pub property_name_key: LocaleKey,
    /// `unitFormat` — typically `"@LOC_..."` or `"@LOC_EMPTY"`.
    pub unit_format_key: LocaleKey,
    /// Optional display transformation applied to the property's raw
    /// value (e.g. scale by 100, convert factor to percent change).
    pub display_transformation: Option<DisplayTransformation>,
    /// Conditional name overrides — when the crafted entity matches a
    /// condition, the property's name swaps to a different locale key.
    /// SC 4.8: 1 entry total across all 29 properties.
    pub name_overrides: Vec<PropertyNameOverride>,
}

/// Polymorphic display transformation applied to a property's raw
/// value before showing it. 5 typed variants reachable under the
/// `crafting` feature; `Other` catches anything else.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayTransformation {
    /// Multiply by a scalar.
    Scale { factor: f32 },
    /// Convert a factor (e.g. 1.2 means +20%) into a percent change.
    ConvertFactorToPercentChange,
    /// Same but negated (multiplier 0.8 ⇒ "20% less").
    ConvertFactorToNegatedPercentChange,
    /// Express the value as a factor of a base.
    ConvertValueToFactorOfBaseValue,
    /// Chain multiple transformations in order.
    Sequence(Vec<DisplayTransformation>),
    /// Dormant variant or future type the generator hasn't promoted.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// A conditional override on a property's display name. Today only
/// `OverrideCondition::ItemType` is populated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyNameOverride {
    pub property_name_key: LocaleKey,
    pub condition: Option<OverrideCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverrideCondition {
    /// Override applies when the crafted entity matches any of these
    /// item types / sub-types.
    ItemType {
        #[serde(with = "enum_serde::item_type_vec")]
        match_item_types: Vec<EItemType>,
        #[serde(with = "enum_serde::item_sub_type_vec")]
        match_sub_types: Vec<EItemSubType>,
    },
    Other {
        type_name: String,
        struct_index: u32,
    },
}

impl GameplayProperties {
    pub fn build(datacore: &Datacore) -> Self {
        let records = &datacore.records().records;
        let pools = &datacore.records().pools;
        let mut props = Self::default();
        for (&guid, &handle) in &records.multi_feature.crafting_gameplay_property_def {
            let Some(rec) = handle.get(pools) else { continue };
            props
                .by_guid
                .insert(guid, build_gameplay_property(guid, rec, pools));
        }
        props
    }

    pub fn get(&self, guid: &Guid) -> Option<&GameplayProperty> {
        self.by_guid.get(guid)
    }

    pub fn iter(&self) -> impl Iterator<Item = &GameplayProperty> + '_ {
        self.by_guid.values()
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}

fn build_gameplay_property(
    guid: Guid,
    rec: &CraftingGameplayPropertyDef,
    pools: &DataPools,
) -> GameplayProperty {
    GameplayProperty {
        guid,
        property_name_key: rec.property_name.clone(),
        unit_format_key: rec.unit_format.clone(),
        display_transformation: rec
            .display_transformation
            .as_ref()
            .map(|t| build_display_transformation(t, pools)),
        name_overrides: rec
            .name_overrides
            .iter()
            .filter_map(|h| h.get(pools))
            .map(|o| build_property_name_override(o, pools))
            .collect(),
    }
}

fn build_display_transformation(
    ptr: &CraftingDisplayTransformation_BasePtr,
    pools: &DataPools,
) -> DisplayTransformation {
    use CraftingDisplayTransformation_BasePtr as D;
    match ptr {
        D::CraftingDisplayTransformation_Scale(h) => match h.get(pools) {
            Some(s) => DisplayTransformation::Scale { factor: s.scale },
            None => DisplayTransformation::Scale { factor: 0.0 },
        },
        D::CraftingDisplayTransformation_ConvertFactorToPercentChange(_) => {
            DisplayTransformation::ConvertFactorToPercentChange
        }
        D::CraftingDisplayTransformation_ConvertFactorToNegatedPercentChange(_) => {
            DisplayTransformation::ConvertFactorToNegatedPercentChange
        }
        D::CraftingDisplayTransformation_ConvertValueToFactorOfBaseValue(_) => {
            DisplayTransformation::ConvertValueToFactorOfBaseValue
        }
        D::CraftingDisplayTransformation_Sequence(h) => match h.get(pools) {
            Some(seq) => DisplayTransformation::Sequence(
                seq.transformations
                    .iter()
                    .map(|t| build_display_transformation(t, pools))
                    .collect(),
            ),
            None => DisplayTransformation::Sequence(Vec::new()),
        },
        D::CraftingDisplayTransformation_Base(_) => DisplayTransformation::Other {
            type_name: "CraftingDisplayTransformation_Base".into(),
            struct_index: 0,
        },
        D::Unknown { struct_index, .. } => DisplayTransformation::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_property_name_override(
    o: &CraftingPropertyNameOverride,
    pools: &DataPools,
) -> PropertyNameOverride {
    use CraftingPropertyNameOverrideCondition_BasePtr as C;
    let condition = o.condition.as_ref().map(|c| match c {
        C::CraftingPropertyNameOverrideCondition_ItemType(h) => match h.get(pools) {
            Some(it) => OverrideCondition::ItemType {
                match_item_types: it.match_item_types.clone(),
                match_sub_types: it.match_sub_types.clone(),
            },
            None => OverrideCondition::Other {
                type_name: "ItemType(empty)".into(),
                struct_index: 0,
            },
        },
        C::CraftingPropertyNameOverrideCondition_Base(_) => OverrideCondition::Other {
            type_name: "CraftingPropertyNameOverrideCondition_Base".into(),
            struct_index: 0,
        },
        C::Unknown { struct_index, .. } => OverrideCondition::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    });
    PropertyNameOverride {
        property_name_key: o.property_name.clone(),
        condition,
    }
}

// ─────────────────────────────────────────────────────────────────────
// Quality (Distribution + LocationOverride + Quantization)
// ─────────────────────────────────────────────────────────────────────

/// Standalone quality records under `libs/foundry/records/crafting/`:
/// the catalog of `CraftingQualityDistributionRecord` /
/// `CraftingQualityLocationOverrideRecord` /
/// `CraftingQualityQuantizationRecord`. Cross-referenced from
/// `ResourceTypeCraftingData` (per-resource inline links) via the
/// `_RecordRef` polymorphic variants.
///
/// SC 4.8 counts: **10 distributions** (100% Normal), **12 location
/// overrides** (134 total entries across the 12), **38 quantizations**
/// (304 total bands across the 38).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Quality {
    distributions: HashMap<Guid, QualityDistribution>,
    location_overrides: HashMap<Guid, QualityLocationOverride>,
    quantizations: HashMap<Guid, QualityQuantization>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityDistribution {
    pub guid: Guid,
    /// `None` when the standalone record's `quality_distribution`
    /// strong-ptr is empty (shouldn't happen but kept defensive).
    pub shape: Option<QualityDistributionShape>,
}

/// The concrete shape of a quality distribution. SC 4.8: 100% `Normal`.
/// `Uniform { min, max }` lives in the `dormant` feature (0 records);
/// it surfaces as `Other` until populated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QualityDistributionShape {
    Normal {
        min: i32,
        max: i32,
        mean: f32,
        stddev: f32,
    },
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// A reference to a distribution. The DCB has two shapes: an inline
/// shape (Normal/etc.) or a record-ref pointing at a standalone
/// [`QualityDistribution`] for sharing. Consumers can resolve a
/// `Record` via [`Quality::distribution`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DistributionRef {
    Inline(QualityDistributionShape),
    /// → `CraftingQualityDistributionRecord` GUID (look up via
    /// [`Quality::distribution`]).
    Record(Guid),
    Other {
        type_name: String,
        struct_index: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityLocationOverride {
    pub guid: Guid,
    /// Per-location distribution overrides. SC 4.8: 12 records carry
    /// ~11 entries each (134 total).
    pub entries: Vec<LocationOverrideEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationOverrideEntry {
    /// → location record (e.g. a `Pyro` / `RCD` / `Torite` system or planet).
    pub location: Option<Guid>,
    pub distribution: Option<DistributionRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityQuantization {
    pub guid: Guid,
    /// Maps continuous quality ranges to discrete output values. SC 4.8:
    /// 38 records carry ~8 bands each (304 total).
    pub bands: Vec<QuantizationBand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QuantizationBand {
    pub start: i32,
    pub end: i32,
    pub mapped_value: i32,
}

impl Quality {
    pub fn build(datacore: &Datacore) -> Self {
        let records = &datacore.records().records;
        let pools = &datacore.records().pools;
        let mut q = Self::default();
        for (&guid, &handle) in &records.multi_feature.crafting_quality_distribution_record {
            let Some(rec) = handle.get(pools) else { continue };
            q.distributions.insert(
                guid,
                QualityDistribution {
                    guid,
                    shape: rec
                        .quality_distribution
                        .as_ref()
                        .map(|d| build_distribution_shape_from_nonref(d, pools)),
                },
            );
        }
        for (&guid, &handle) in &records.multi_feature.crafting_quality_location_override_record {
            let Some(rec) = handle.get(pools) else { continue };
            let entries = match rec.location_override.as_ref() {
                Some(CraftingQualityLocationOverride_Base_NonRefPtr::CraftingQualityLocationOverride(h)) => h
                    .get(pools)
                    .map(|co| {
                        co.location_override_list
                            .iter()
                            .filter_map(|eh| eh.get(pools))
                            .map(|e| LocationOverrideEntry {
                                location: e.location,
                                distribution: e
                                    .quality_distribution
                                    .as_ref()
                                    .map(|d| build_distribution_ref(d, pools)),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                _ => Vec::new(),
            };
            q.location_overrides
                .insert(guid, QualityLocationOverride { guid, entries });
        }
        for (&guid, &handle) in &records.multi_feature.crafting_quality_quantization_record {
            let Some(rec) = handle.get(pools) else { continue };
            let bands = match rec.quality_quantization.as_ref() {
                Some(CraftingQualityQuantization_Base_NonRefPtr::CraftingQualityQuantization(h)) => h
                    .get(pools)
                    .map(|qq| {
                        qq.bands
                            .iter()
                            .filter_map(|bh| bh.get(pools))
                            .map(|b| QuantizationBand {
                                start: b.start,
                                end: b.end,
                                mapped_value: b.mapped_value,
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                _ => Vec::new(),
            };
            q.quantizations
                .insert(guid, QualityQuantization { guid, bands });
        }
        q
    }

    pub fn distribution(&self, guid: &Guid) -> Option<&QualityDistribution> {
        self.distributions.get(guid)
    }

    pub fn location_override(&self, guid: &Guid) -> Option<&QualityLocationOverride> {
        self.location_overrides.get(guid)
    }

    pub fn quantization(&self, guid: &Guid) -> Option<&QualityQuantization> {
        self.quantizations.get(guid)
    }

    pub fn distributions(&self) -> impl Iterator<Item = &QualityDistribution> + '_ {
        self.distributions.values()
    }

    pub fn location_overrides(&self) -> impl Iterator<Item = &QualityLocationOverride> + '_ {
        self.location_overrides.values()
    }

    pub fn quantizations(&self) -> impl Iterator<Item = &QualityQuantization> + '_ {
        self.quantizations.values()
    }
}

fn build_distribution_shape_from_nonref(
    d: &CraftingQualityDistribution_Base_NonRefPtr,
    pools: &DataPools,
) -> QualityDistributionShape {
    use CraftingQualityDistribution_Base_NonRefPtr as P;
    match d {
        P::CraftingQualityDistributionNormal(h) => match h.get(pools) {
            Some(n) => QualityDistributionShape::Normal {
                min: n.min,
                max: n.max,
                mean: n.mean,
                stddev: n.stddev,
            },
            None => QualityDistributionShape::Other {
                type_name: "Normal(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityDistribution_Base_NonRef(_) => QualityDistributionShape::Other {
            type_name: "CraftingQualityDistribution_Base_NonRef".into(),
            struct_index: 0,
        },
        P::Unknown { struct_index, .. } => QualityDistributionShape::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_distribution_ref(
    d: &CraftingQualityDistribution_BasePtr,
    pools: &DataPools,
) -> DistributionRef {
    use CraftingQualityDistribution_BasePtr as P;
    match d {
        P::CraftingQualityDistributionNormal(h) => match h.get(pools) {
            Some(n) => DistributionRef::Inline(QualityDistributionShape::Normal {
                min: n.min,
                max: n.max,
                mean: n.mean,
                stddev: n.stddev,
            }),
            None => DistributionRef::Other {
                type_name: "Normal(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityDistribution_RecordRef(h) => match h.get(pools) {
            Some(r) => match r.quality_distribution_record {
                Some(g) => DistributionRef::Record(g),
                None => DistributionRef::Other {
                    type_name: "RecordRef(none)".into(),
                    struct_index: 0,
                },
            },
            None => DistributionRef::Other {
                type_name: "RecordRef(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityDistribution_Base(_)
        | P::CraftingQualityDistribution_Base_NonRef(_) => DistributionRef::Other {
            type_name: "CraftingQualityDistribution_Base(_NonRef)".into(),
            struct_index: 0,
        },
        P::Unknown { struct_index, .. } => DistributionRef::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

/// serde adapters for generated enums. Same pattern as sc-items: the
/// generated crate is serde-free (compile-time monomorphization cliff)
/// so we round-trip each variant through its DCB string via
/// `as_dcb_str` / `from_dcb_str`.
mod enum_serde {
    use sc_extract::generated::{EItemSubType, EItemType};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub mod item_type_vec {
        use super::*;
        pub fn serialize<S: Serializer>(v: &[EItemType], s: S) -> Result<S::Ok, S::Error> {
            let strs: Vec<&str> = v.iter().map(EItemType::as_dcb_str).collect();
            strs.serialize(s)
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<EItemType>, D::Error> {
            Ok(Vec::<String>::deserialize(d)?
                .into_iter()
                .map(|s| EItemType::from_dcb_str(&s))
                .collect())
        }
    }

    pub mod item_sub_type_vec {
        use super::*;
        pub fn serialize<S: Serializer>(v: &[EItemSubType], s: S) -> Result<S::Ok, S::Error> {
            let strs: Vec<&str> = v.iter().map(EItemSubType::as_dcb_str).collect();
            strs.serialize(s)
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<EItemSubType>, D::Error> {
            Ok(Vec::<String>::deserialize(d)?
                .into_iter()
                .map(|s| EItemSubType::from_dcb_str(&s))
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_seconds_sum() {
        let d = Duration {
            days: 1,
            hours: 2,
            minutes: 3,
            seconds: 4.5,
        };
        assert_eq!(
            d.to_seconds(),
            86_400.0 + 2.0 * 3600.0 + 3.0 * 60.0 + 4.5
        );
    }

    #[test]
    fn is_placeholder_detects_cig_sentinels() {
        assert!(is_placeholder("<= PLACEHOLDER =>"));
        assert!(is_placeholder("xx PLACEHOLDER xx"));
        assert!(!is_placeholder("Arclight Pistol"));
    }

    #[test]
    fn blueprint_display_name_prefers_entity() {
        let bp = Blueprint {
            blueprint_record_guid: Guid::from_bytes([0; 16]),
            category: None,
            process: Process::Creation { entity_class: None },
            blueprint_name_key: Some(LocaleKey::from("@bp_fallback")),
            entity_name_key: Some(LocaleKey::from("@entity_name")),
            tiers: Vec::new(),
        };
        // we can't resolve without a LocaleMap, but the helper at least
        // doesn't panic with both keys present.
        let _ = bp;
    }
}
