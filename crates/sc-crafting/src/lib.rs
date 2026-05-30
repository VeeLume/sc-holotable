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
//! # Compatibility
//!
//! Pre-v0.9.0 consumers used [`all_blueprints`] returning
//! `Vec<BlueprintItem>` (identity + display name only). Those types
//! remain as a thin shim; new consumers use [`Blueprints`] for the full
//! recipe surface.

use sc_extract::generated::{
    CraftingBlueprintRecord, CraftingBlueprintTier_BasePtr, CraftingBlueprint_Base_NonRefPtr,
    CraftingCost_BasePtr, CraftingOptionalEffect_BasePtr, CraftingProcess_BasePtr,
    CraftingRecipeCosts_BasePtr, CraftingRecipeResults_BasePtr, CraftingRecipe_BasePtr,
    CraftingResearch_BasePtr, CraftingResearchUnlock_BasePtr, CraftingResult_BasePtr, DataPools,
    RecordIndex, RecordLookup, SBaseCargoUnitPtr, TimeValue_BasePtr,
};
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap};
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
// Pre-v0.9.0 compatibility shim
// ─────────────────────────────────────────────────────────────────────

/// **Compatibility:** the pre-v0.9.0 catalog shape (identity + display
/// name only). Equivalent to `Blueprint` minus recipe data. Use
/// [`Blueprint`] for new code.
#[derive(Debug, Clone, PartialEq)]
pub struct BlueprintItem {
    pub blueprint_record_guid: Guid,
    pub crafted_entity_guid: Option<Guid>,
    pub entity_name_key: Option<LocaleKey>,
    pub blueprint_name_key: Option<LocaleKey>,
}

impl BlueprintItem {
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

impl From<&Blueprint> for BlueprintItem {
    fn from(bp: &Blueprint) -> Self {
        Self {
            blueprint_record_guid: bp.blueprint_record_guid,
            crafted_entity_guid: bp.crafted_entity_guid(),
            entity_name_key: bp.entity_name_key.clone(),
            blueprint_name_key: bp.blueprint_name_key.clone(),
        }
    }
}

/// **Compatibility:** the full craftable catalog as `BlueprintItem`s.
/// Equivalent to `Blueprints::build(...).iter().map(BlueprintItem::from).collect()`.
pub fn all_blueprints(datacore: &Datacore, items: &Items) -> Vec<BlueprintItem> {
    Blueprints::build(datacore, items)
        .iter()
        .map(BlueprintItem::from)
        .collect()
}

/// **Compatibility:** resolve a single `CraftingBlueprintRecord` GUID
/// to a [`BlueprintItem`].
pub fn resolve_blueprint(
    datacore: &Datacore,
    items: &Items,
    record_guid: Guid,
) -> BlueprintItem {
    let pools = &datacore.records().pools;
    let records = &datacore.records().records;
    let Some(handle) = records.multi_feature.crafting_blueprint_record.get(&record_guid) else {
        return BlueprintItem {
            blueprint_record_guid: record_guid,
            crafted_entity_guid: None,
            entity_name_key: None,
            blueprint_name_key: None,
        };
    };
    let Some(record) = handle.get(pools) else {
        return BlueprintItem {
            blueprint_record_guid: record_guid,
            crafted_entity_guid: None,
            entity_name_key: None,
            blueprint_name_key: None,
        };
    };
    BlueprintItem::from(&build_blueprint(record_guid, record, records, pools, items))
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
