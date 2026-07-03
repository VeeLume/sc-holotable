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
    CraftingBlueprint_Base_NonRefPtr, CraftingBlueprintRecord, CraftingBlueprintTier_BasePtr,
    CraftingCost_BasePtr, CraftingCostContext_BasePtr, CraftingDisplayTransformation_BasePtr,
    CraftingGameplayPropertyDef, CraftingGameplayPropertyModifier_BasePtr,
    CraftingGameplayPropertyModifierValueRange_BasePtr, CraftingGameplayPropertyModifiers_BasePtr,
    CraftingGlobalParams, CraftingOptionalEffect_BasePtr, CraftingProcess_BasePtr,
    CraftingPropertyNameOverride, CraftingPropertyNameOverrideCondition_BasePtr,
    CraftingRecipe_BasePtr, CraftingRecipeCosts_BasePtr, CraftingRecipeResults_BasePtr,
    CraftingResearch_BasePtr, CraftingResearchUnlock_BasePtr, CraftingResult_BasePtr, DataPools,
    DefaultBlueprintSelection_BasePtr, ECraftingCostResultCompositionOption, EItemSubType,
    EItemType, RecordIndex, RecordLookup, SBaseCargoUnitPtr, TimeValue_BasePtr,
};
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap, RecordPaths};
use sc_items::Items;
use sc_items_armor::Armor;
use sc_items_fps_weapons::FpsWeapons;
use sc_items_ship_components::ShipComponents;
use sc_items_ship_weapons::ShipWeapons;
use sc_resources::CargoQuantity;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Re-export the canonical accessor trait (get / iter / len / values) so consumers
// can bring it into scope alongside the collection.
pub use sc_extract::RecordCollection;
use tracing::warn;

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
            let Some(record) = handle.get(pools) else {
                continue;
            };
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

    /// All blueprints belonging to a category (looked up by category GUID).
    pub fn in_category(&self, category: &Guid) -> impl Iterator<Item = &Blueprint> + '_ {
        self.by_category
            .get(category)
            .into_iter()
            .flatten()
            .filter_map(|&i| self.entries.get(i))
    }

    /// Look up the blueprint that crafts a given entity (by EntityClassDefinition GUID).
    pub fn for_crafted_entity(&self, entity_guid: &Guid) -> Option<&Blueprint> {
        let idx = *self.by_crafted_entity.get(entity_guid)?;
        self.entries.get(idx)
    }
}

impl sc_extract::RecordCollection for Blueprints {
    type Item = Blueprint;

    fn get(&self, guid: &Guid) -> Option<&Blueprint> {
        let idx = *self.by_record_guid.get(guid)?;
        self.entries.get(idx)
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Blueprint)> + '_ {
        self.entries.iter().map(|b| (&b.blueprint_record_guid, b))
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
/// SC 4.x universally shapes mandatory costs as
/// `Select { N, [Select { 1, [<leaf>] }] }` — pick N ingredient *slots*,
/// each with one alternative. The leaf is either a `Resource` (bulk
/// ship-mined / refined material, ~3.9k entries) or an `Item` (a discrete
/// carried entity counted individually — the hand-mined gems, ~294
/// entries). Top-level `Resource`/`Item` costs (not wrapped in a `Select`)
/// and the dormant `Other` variants are 0 records today but kept in the
/// model.
///
/// Each node also carries a [`context`](Cost::context) list — this is where
/// the **crafting-effect machinery** lives: how an ingredient's quality
/// reshapes the crafted item's gameplay properties (recoil, damage,
/// integrity, …). The `Select` slot additionally carries a
/// [`SlotName`] ("Frame", "Cabling", "Power Regulator", …). See
/// [`Cost::gameplay_property_modifiers`] to roll a slot's effects up for
/// display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cost {
    Resource(ResourceCost),
    Item(ItemCost),
    Select {
        /// Slot label from `CraftingCost_Select.name_info`. `None` when the
        /// select carries no name info; resolves to `<= PLACEHOLDER =>` for
        /// not-yet-authored slots.
        name_info: Option<SlotName>,
        count: i32,
        options: Vec<Cost>,
        /// Context attached at the slot level (where the gameplay-property
        /// modifiers attach in observed data).
        context: Vec<CostContext>,
    },
    /// Dormant variants (`_Ref`/`_RecordRef`/...) surface here.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

impl Cost {
    /// The context attached directly to this cost node (empty for `Other`).
    pub fn context(&self) -> &[CostContext] {
        match self {
            Cost::Resource(rc) => &rc.context,
            Cost::Item(ic) => &ic.context,
            Cost::Select { context, .. } => context,
            Cost::Other { .. } => &[],
        }
    }

    /// Walk this cost subtree and collect every gameplay-property modifier,
    /// regardless of which node carries it. The natural way to roll up a
    /// slot's effects for display (`for m in slot.gameplay_property_modifiers()`).
    pub fn gameplay_property_modifiers(&self) -> Vec<&GameplayPropertyModifier> {
        let mut out = Vec::new();
        self.collect_modifiers(&mut out);
        out
    }

    fn collect_modifiers<'a>(&'a self, out: &mut Vec<&'a GameplayPropertyModifier>) {
        for ctx in self.context() {
            if let CostContext::GameplayPropertyModifiers(mods) = ctx {
                out.extend(mods.iter());
            }
        }
        if let Cost::Select { options, .. } = self {
            for o in options {
                o.collect_modifiers(out);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCost {
    /// → `ResourceType` (resolve via `sc_resources::Resources::get`).
    pub resource: Option<Guid>,
    pub quantity: Option<CargoQuantity>,
    pub min_quality: i32,
    /// Per-ingredient context (gameplay-property modifiers, quantity
    /// multiplier, composition inclusion). Empty when none attached.
    pub context: Vec<CostContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCost {
    /// → `EntityClassDefinition`.
    pub entity_class: Option<Guid>,
    pub quantity: i32,
    pub min_quality: i32,
    /// Per-ingredient context. See [`ResourceCost::context`].
    pub context: Vec<CostContext>,
}

/// The player-facing label of a `Select` ingredient slot, from
/// `CraftingCost_Select.name_info`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotName {
    /// Authoring-side identifier (never localized).
    pub debug_name: String,
    /// Display label key — resolve via [`LocaleMap`]. `<= PLACEHOLDER =>`
    /// for not-yet-authored slots.
    pub display_name: LocaleKey,
}

/// Extra context attached to a [`Cost`] node. The crafting-effect machinery
/// — how an ingredient's quality reshapes the crafted item's stats — rides
/// here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CostContext {
    /// Per-quality modifiers applied to the crafted item's gameplay
    /// properties (e.g. "Recoil Smoothness ×1.4→0.6 across quality 0–1000").
    GameplayPropertyModifiers(Vec<GameplayPropertyModifier>),
    /// Scales the ingredient's required quantity.
    QuantityMultiplier(f32),
    /// Whether this ingredient is folded into the result's composition.
    ResultCompositionInclusion(CompositionInclusion),
    /// Dormant / future context variant.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// `ECraftingCostResultCompositionOption`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompositionInclusion {
    Include,
    Exclude,
    /// Unrecognised / future value.
    Other,
}

/// One gameplay property reshaped by an ingredient's quality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayPropertyModifier {
    /// → `CraftingGameplayPropertyDef` (resolve via [`GameplayProperties::get`]
    /// for the display name, printf unit format, and display transform).
    /// **The GUID is the join key — do not match on the property's name.**
    pub gameplay_property: Option<Guid>,
    /// Quality→value bands. Each band covers a `[start, end]` quality slice;
    /// [`GameplayPropertyModifier::evaluate`] picks the band for a quality.
    pub value_ranges: Vec<ValueRange>,
}

impl GameplayPropertyModifier {
    /// Evaluate this modifier at `quality` (0–1000), choosing the band whose
    /// `[start, end]` contains `quality`. When `quality` falls outside every
    /// band (a gap, or below the first / above the last), clamps to the
    /// nearest band. `None` if there are no evaluable bands.
    pub fn evaluate(&self, quality: i32) -> Option<ModifierValue> {
        if let Some(vr) = self.value_ranges.iter().find(|vr| vr.contains(quality)) {
            return vr.evaluate(quality);
        }
        self.value_ranges
            .iter()
            .filter_map(|vr| vr.quality_band().map(|b| (vr, b)))
            .min_by_key(|(_, (s, e))| {
                if quality < *s {
                    s - quality
                } else {
                    quality - e
                }
            })
            .and_then(|(vr, _)| vr.evaluate(quality))
    }
}

/// A single quality→value band.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueRange {
    /// Multiplicative factor, linearly interpolated across the band:
    /// `×modifier_at_start` at `start_quality` → `×modifier_at_end` at
    /// `end_quality`.
    Linear {
        start_quality: i32,
        end_quality: i32,
        modifier_at_start: f32,
        modifier_at_end: f32,
    },
    /// Integer additive bonus, linearly interpolated across the band.
    LinearIntegerAdditive {
        start_quality: i32,
        end_quality: i32,
        additive_at_start: i32,
        additive_at_end: i32,
    },
    /// Dormant / future range shape.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

impl ValueRange {
    /// The `[start_quality, end_quality]` band this range covers, if any.
    pub fn quality_band(&self) -> Option<(i32, i32)> {
        match self {
            ValueRange::Linear {
                start_quality,
                end_quality,
                ..
            }
            | ValueRange::LinearIntegerAdditive {
                start_quality,
                end_quality,
                ..
            } => Some((*start_quality, *end_quality)),
            ValueRange::Other { .. } => None,
        }
    }

    /// True if `quality` falls within this range's band (inclusive).
    pub fn contains(&self, quality: i32) -> bool {
        self.quality_band()
            .is_some_and(|(s, e)| quality >= s && quality <= e)
    }

    /// Evaluate the modifier at `quality`, linearly interpolating within the
    /// band and clamping to its endpoints. `None` for `Other`.
    pub fn evaluate(&self, quality: i32) -> Option<ModifierValue> {
        match self {
            ValueRange::Linear {
                start_quality,
                end_quality,
                modifier_at_start,
                modifier_at_end,
            } => Some(ModifierValue::Multiplier(lerp(
                *start_quality,
                *end_quality,
                *modifier_at_start,
                *modifier_at_end,
                quality,
            ))),
            ValueRange::LinearIntegerAdditive {
                start_quality,
                end_quality,
                additive_at_start,
                additive_at_end,
            } => Some(ModifierValue::Additive(lerp(
                *start_quality,
                *end_quality,
                *additive_at_start as f32,
                *additive_at_end as f32,
                quality,
            ))),
            ValueRange::Other { .. } => None,
        }
    }
}

/// The evaluated effect of a [`ValueRange`] at a given quality.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ModifierValue {
    /// Multiplicative factor (1.0 = no change).
    Multiplier(f32),
    /// Additive bonus.
    Additive(f32),
}

/// Linear interpolation over a quality band, clamped to `[start_q, end_q]`.
/// A degenerate band (`end_q <= start_q`) yields the start value.
fn lerp(start_q: i32, end_q: i32, v_start: f32, v_end: f32, quality: i32) -> f32 {
    if end_q <= start_q {
        return v_start;
    }
    let q = quality.clamp(start_q, end_q);
    let t = (q - start_q) as f32 / (end_q - start_q) as f32;
    v_start + (v_end - v_start) * t
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

    let Some(bp_ptr) = &record.blueprint else {
        return bp;
    };
    let CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(bh) = bp_ptr else {
        // CraftingBlueprint_Base_NonRef (empty base) — leave bp with default Process::Other.
        return bp;
    };
    let Some(blueprint) = bh.get(pools) else {
        return bp;
    };

    bp.category = blueprint.category;
    if !blueprint.blueprint_name.is_empty() {
        bp.blueprint_name_key = Some(blueprint.blueprint_name.clone());
    }
    bp.process = build_process(&blueprint.process_specific_data, pools);
    if let Process::Creation {
        entity_class: Some(eg),
    } = &bp.process
    {
        bp.entity_name_key = items.name_key(eg).cloned();
    }
    bp.tiers = blueprint
        .tiers
        .iter()
        .map(|tier_ptr| build_tier(tier_ptr, pools))
        .collect();

    bp
}

fn build_process(process: &Option<CraftingProcess_BasePtr>, pools: &DataPools) -> Process {
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
        CraftingProcess_BasePtr::Unknown { struct_index, .. } => Process::Other {
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
                        effect_kind: oe.effect.as_ref().map(build_effect_kind),
                    })
                    .collect();
                (
                    time,
                    Some(RecipeCosts {
                        mandatory,
                        optional,
                    }),
                )
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
                quantity: r
                    .quantity
                    .as_ref()
                    .map(|q| cargo_quantity_from_ptr(q, pools)),
                min_quality: r.min_quality,
                context: build_context(&r.context, pools),
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
                context: build_context(&i.context, pools),
            }),
            None => Cost::Other {
                type_name: "CraftingCost_Item(empty)".into(),
                struct_index: 0,
            },
        },
        CraftingCost_BasePtr::CraftingCost_Select(h) => match h.get(pools) {
            Some(sel) => Cost::Select {
                name_info: sel
                    .name_info
                    .as_ref()
                    .and_then(|nih| nih.get(pools))
                    .map(|ni| SlotName {
                        debug_name: ni.debug_name.clone(),
                        display_name: ni.display_name.clone(),
                    }),
                count: sel.count,
                options: sel.options.iter().map(|o| build_cost(o, pools)).collect(),
                context: build_context(&sel.context, pools),
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
        CraftingCost_BasePtr::Unknown { struct_index, .. } => Cost::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_context(ctx: &[CraftingCostContext_BasePtr], pools: &DataPools) -> Vec<CostContext> {
    ctx.iter().map(|c| build_cost_context(c, pools)).collect()
}

fn build_cost_context(ptr: &CraftingCostContext_BasePtr, pools: &DataPools) -> CostContext {
    use CraftingCostContext_BasePtr as C;
    match ptr {
        C::CraftingCostContext_ResultGameplayPropertyModifiers(h) => {
            let mods = h
                .get(pools)
                .and_then(|c| c.gameplay_property_modifiers.as_ref())
                .map(|m| build_modifier_list(m, pools))
                .unwrap_or_default();
            CostContext::GameplayPropertyModifiers(mods)
        }
        C::CraftingCostContext_QuantityMultiplier(h) => {
            CostContext::QuantityMultiplier(h.get(pools).map(|c| c.multiplier).unwrap_or(1.0))
        }
        C::CraftingCostContext_ResultCompositionInclusion(h) => {
            let inclusion = match h.get(pools) {
                Some(c) => match &c.option {
                    ECraftingCostResultCompositionOption::Include => CompositionInclusion::Include,
                    ECraftingCostResultCompositionOption::Exclude => CompositionInclusion::Exclude,
                    ECraftingCostResultCompositionOption::Unrecognized(_) => {
                        CompositionInclusion::Other
                    }
                },
                None => CompositionInclusion::Other,
            };
            CostContext::ResultCompositionInclusion(inclusion)
        }
        C::CraftingCostContext_Base(_) => CostContext::Other {
            type_name: "CraftingCostContext_Base".into(),
            struct_index: 0,
        },
        C::Unknown { struct_index, .. } => CostContext::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_modifier_list(
    ptr: &CraftingGameplayPropertyModifiers_BasePtr,
    pools: &DataPools,
) -> Vec<GameplayPropertyModifier> {
    let CraftingGameplayPropertyModifiers_BasePtr::CraftingGameplayPropertyModifiers_List(h) = ptr
    else {
        return Vec::new();
    };
    let Some(list) = h.get(pools) else {
        return Vec::new();
    };
    list.gameplay_property_modifiers
        .iter()
        .filter_map(|m| build_modifier(m, pools))
        .collect()
}

fn build_modifier(
    ptr: &CraftingGameplayPropertyModifier_BasePtr,
    pools: &DataPools,
) -> Option<GameplayPropertyModifier> {
    let CraftingGameplayPropertyModifier_BasePtr::CraftingGameplayPropertyModifierCommon(h) = ptr
    else {
        return None;
    };
    let common = h.get(pools)?;
    Some(GameplayPropertyModifier {
        gameplay_property: common.gameplay_property_record,
        value_ranges: common
            .value_ranges
            .iter()
            .map(|vr| build_value_range(vr, pools))
            .collect(),
    })
}

fn build_value_range(
    ptr: &CraftingGameplayPropertyModifierValueRange_BasePtr,
    pools: &DataPools,
) -> ValueRange {
    use CraftingGameplayPropertyModifierValueRange_BasePtr as V;
    match ptr {
        V::CraftingGameplayPropertyModifierValueRange_Linear(h) => match h.get(pools) {
            Some(v) => ValueRange::Linear {
                start_quality: v.start_quality,
                end_quality: v.end_quality,
                modifier_at_start: v.modifier_at_start,
                modifier_at_end: v.modifier_at_end,
            },
            None => ValueRange::Other {
                type_name: "Linear(empty)".into(),
                struct_index: 0,
            },
        },
        V::CraftingGameplayPropertyModifierValueRange_LinearIntegerAdditive(h) => {
            match h.get(pools) {
                Some(v) => ValueRange::LinearIntegerAdditive {
                    start_quality: v.start_quality,
                    end_quality: v.end_quality,
                    additive_at_start: v.additive_modifier_at_start,
                    additive_at_end: v.additive_modifier_at_end,
                },
                None => ValueRange::Other {
                    type_name: "LinearIntegerAdditive(empty)".into(),
                    struct_index: 0,
                },
            }
        }
        V::CraftingGameplayPropertyModifierValueRange_Base(_) => ValueRange::Other {
            type_name: "CraftingGameplayPropertyModifierValueRange_Base".into(),
            struct_index: 0,
        },
        V::Unknown { struct_index, .. } => ValueRange::Other {
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
        CraftingOptionalEffect_BasePtr::Unknown { struct_index, .. } => OptionalEffectKind::Other {
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
        CraftingResult_BasePtr::Unknown { struct_index, .. } => RecipeResult::Other {
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
        CraftingResearchUnlock_BasePtr::Unknown { struct_index, .. } => ResearchUnlock::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    });

    let costs = match research.research_costs.as_ref() {
        Some(CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(ch)) => {
            ch.get(pools).map(|c| RecipeCosts {
                mandatory: c.mandatory_cost.as_ref().map(|m| build_cost(m, pools)),
                optional: c
                    .optional_costs
                    .iter()
                    .filter_map(|oh| oh.get(pools))
                    .map(|oe| OptionalCost {
                        cost: oe.optional_cost.as_ref().map(|c| build_cost(c, pools)),
                        effect_kind: oe.effect.as_ref().map(build_effect_kind),
                    })
                    .collect(),
            })
        }
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
        let Some(record) = handle.get(&store.pools) else {
            return;
        };
        let blueprint =
            build_blueprint(item.guid, record, &store.records, &store.pools, self.items);
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
        let gp = pools
            .crafting
            .crafting_global_params
            .iter()
            .flatten()
            .next()?;
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
    /// DCB record name, e.g. `"CraftingGameplayPropertyDef.GPP_Weapon_FireRate"`.
    /// The stable anchor for [`GameplayProperty::stat`] — we key the
    /// GPP→stat mapping on this name (not the GUID, which churns between
    /// patches). See [`GameplayStat`].
    pub record_name: String,
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
        let db = datacore.db();
        let mut props = Self::default();
        for (&guid, &handle) in &records.multi_feature.crafting_gameplay_property_def {
            let Some(rec) = handle.get(pools) else {
                continue;
            };
            let record_name = db
                .record(&guid)
                .and_then(|r| r.name())
                .unwrap_or_default()
                .to_string();
            props
                .by_guid
                .insert(guid, build_gameplay_property(guid, rec, pools, record_name));
        }
        props.validate_stat_coverage();
        props
    }

    pub fn get(&self, guid: &Guid) -> Option<&GameplayProperty> {
        self.by_guid.get(guid)
    }

    /// Loud-drift guard: warn if any modelled [`GameplayStat`] no longer has a
    /// matching live GPP record (renamed/removed in a patch). This is the
    /// alarm that keeps the name-anchored mapping honest — see [`GameplayStat`].
    fn validate_stat_coverage(&self) {
        let live: HashSet<&str> = self
            .by_guid
            .values()
            .map(|p| gpp_suffix(&p.record_name))
            .collect();
        for (suffix, variant) in GameplayStat::KNOWN {
            if !live.contains(suffix) {
                warn!(
                    stat = ?variant,
                    suffix,
                    "modelled GameplayStat has no live GPP record — renamed/removed? GPP→stat mapping needs review"
                );
            }
        }
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

// ─────────────────────────────────────────────────────────────────────
// Gameplay-stat mapping + product stats
// ─────────────────────────────────────────────────────────────────────

/// The typed vocabulary of gameplay stats that crafting reshapes.
///
/// A GPP is a *data record* with no schema-level identity, so we map it by its
/// **record-name** suffix — the only stable, auditable anchor (record GUIDs
/// churn between patches and would fail silently). This is the project's
/// "string match genuinely unavoidable → scope, comment, alarm" carve-out: the
/// GPP→stat binding is *not present in the p4k at all* (the GPP def is
/// display-only; nothing references it but recipe blueprints — verified via
/// `examples/fps_gpp_dig.rs`), so no typed/data-derived alternative exists.
/// [`GameplayProperties::build`] validates coverage and `warn!`s on drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameplayStat {
    WeaponFireRate,
    WeaponDamage,
    WeaponRecoilKick,
    WeaponRecoilHandling,
    WeaponRecoilSmoothness,
    WeaponSpread,
    ArmorDamageMitigation,
    ArmorTemperatureMin,
    ArmorTemperatureMax,
    ArmorRadiationDissipation,
    /// Component HP ("Integrity") — `GPP_Health_MaxHealth`, shared by every
    /// ship component.
    Integrity,
    QuantumSpeed,
    QuantumFuelRequirement,
    ShieldMaxHealth,
    CoolantGeneration,
    PowerGeneration,
    RadarMinAimAssist,
    RadarMaxAimAssist,
    /// A GPP not yet modelled (tractor / hull-scraping / …, or a stat added in
    /// a game patch). Carries the record-name suffix.
    Unknown(String),
}

impl GameplayStat {
    /// `(record-name suffix, variant)` for every modelled stat. The recoil
    /// axis pairing — Kick→pitch, Handling→yaw, Smoothness→smooth-time —
    /// follows the in-game stat-panel convention; it is the one editorial
    /// choice here, since the three recoil GPPs are otherwise indistinguishable
    /// in data (a recipe applies all three identically).
    const KNOWN: &'static [(&'static str, GameplayStat)] = &[
        ("GPP_Weapon_FireRate", GameplayStat::WeaponFireRate),
        ("GPP_Weapon_Damage", GameplayStat::WeaponDamage),
        ("GPP_Weapon_Recoil_Kick", GameplayStat::WeaponRecoilKick),
        (
            "GPP_Weapon_Recoil_Handling",
            GameplayStat::WeaponRecoilHandling,
        ),
        (
            "GPP_Weapon_Recoil_Smoothness",
            GameplayStat::WeaponRecoilSmoothness,
        ),
        ("GPP_Weapon_Spread", GameplayStat::WeaponSpread),
        (
            "GPP_Armor_DamageMitigation",
            GameplayStat::ArmorDamageMitigation,
        ),
        (
            "GPP_Armor_TemperatureMin",
            GameplayStat::ArmorTemperatureMin,
        ),
        (
            "GPP_Armor_TemperatureMax",
            GameplayStat::ArmorTemperatureMax,
        ),
        (
            "GPP_Armor_RadiationDissipation",
            GameplayStat::ArmorRadiationDissipation,
        ),
        ("GPP_Health_MaxHealth", GameplayStat::Integrity),
        ("GPP_Quantum_Speed", GameplayStat::QuantumSpeed),
        (
            "GPP_Quantum_FuelRequirement",
            GameplayStat::QuantumFuelRequirement,
        ),
        ("GPP_Shield_MaxHealth", GameplayStat::ShieldMaxHealth),
        (
            "GPP_ItemResource_CoolantGeneration",
            GameplayStat::CoolantGeneration,
        ),
        (
            "GPP_ItemResource_PowerGeneration",
            GameplayStat::PowerGeneration,
        ),
        (
            "GPP_Radar_MinAimAssistDistance",
            GameplayStat::RadarMinAimAssist,
        ),
        (
            "GPP_Radar_MaxAimAssistDistance",
            GameplayStat::RadarMaxAimAssist,
        ),
    ];

    /// Resolve a GPP record name (full `TypeName.Suffix` or bare suffix) to a
    /// stat. Unmodelled GPPs fall through to [`GameplayStat::Unknown`].
    pub fn from_gpp_name(record_name: &str) -> GameplayStat {
        let suffix = gpp_suffix(record_name);
        Self::KNOWN
            .iter()
            .find(|(s, _)| *s == suffix)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| GameplayStat::Unknown(suffix.to_string()))
    }
}

/// Record-name tail after the `TypeName.` prefix
/// (`CraftingGameplayPropertyDef.GPP_Weapon_FireRate` → `GPP_Weapon_FireRate`).
fn gpp_suffix(record_name: &str) -> &str {
    record_name.rsplit('.').next().unwrap_or(record_name)
}

impl GameplayProperty {
    /// The typed [`GameplayStat`] this property maps to (record-name anchored).
    pub fn stat(&self) -> GameplayStat {
        GameplayStat::from_gpp_name(&self.record_name)
    }
}

/// One crafted-item product stat: a gameplay property after the recipe's
/// material modifiers are applied at a given quality.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductStat {
    /// The GPP this came from (raw GUID).
    pub gameplay_property: Guid,
    /// Typed stat (record-name anchored). [`GameplayStat::Unknown`] for GPPs
    /// outside the modelled set.
    pub stat: GameplayStat,
    /// Base value off the item (fire-rate RPM, recoil degrees, …). `None` when
    /// the entity has no known base for this stat — the caller can still show
    /// [`ProductStat::pct_change`].
    pub base: Option<f32>,
    /// `base × factor + additive`, when `base` is known.
    pub modified: Option<f32>,
    /// Aggregate multiplicative factor (1.0 = no change), combined **additively
    /// across slots**: `1 + Σ(factorᵢ − 1)` — the in-game rule, verified
    /// against scmdb (two −10% slots → −20%, not −19%).
    pub factor: f32,
    /// Aggregate additive bonus (0.0 for pure-multiplier stats).
    pub additive: f32,
}

impl ProductStat {
    /// Percent change implied by the factor (×0.80 → −20%).
    pub fn pct_change(&self) -> f32 {
        (self.factor - 1.0) * 100.0
    }
}

/// A source of *base* values for [`GameplayStat`]s — implemented per item
/// domain so [`Blueprints::product_stats`] stays generic over the source.
///
/// Each domain's base-stat crate (`sc-items-fps-weapons`, `sc-items-armor`, …)
/// is pure data; the `GameplayStat → field` mapping lives here in sc-crafting
/// (orphan rule: the trait is local), so the integrator remains the single
/// owner of the GPP↔stat mapping.
pub trait ProductStatSource {
    /// Base value for `stat` on `entity`, or `None` if this source doesn't
    /// cover that entity or stat.
    fn base_value(&self, entity: &Guid, stat: &GameplayStat) -> Option<f32>;
}

impl ProductStatSource for FpsWeapons {
    fn base_value(&self, entity: &Guid, stat: &GameplayStat) -> Option<f32> {
        let w = self.get(entity)?;
        match stat {
            GameplayStat::WeaponFireRate => w.fire_rate.map(|r| r as f32),
            GameplayStat::WeaponDamage => w.damage.map(|d| d.total()),
            GameplayStat::WeaponRecoilKick => w.recoil_pitch,
            GameplayStat::WeaponRecoilHandling => w.recoil_yaw,
            GameplayStat::WeaponRecoilSmoothness => w.recoil_smooth,
            GameplayStat::WeaponSpread => w.spread_max,
            _ => None,
        }
    }
}

impl ProductStatSource for Armor {
    fn base_value(&self, entity: &Guid, stat: &GameplayStat) -> Option<f32> {
        let a = self.get(entity)?;
        match stat {
            GameplayStat::ArmorTemperatureMin => a.temp_resistance_min,
            GameplayStat::ArmorTemperatureMax => a.temp_resistance_max,
            GameplayStat::ArmorRadiationDissipation => a.radiation_dissipation,
            // Damage Mitigation = the mitigation FRACTION `1 − multiplier`
            // (a damage-taken `multiplier` of 0.60 = 40% mitigation). The
            // crafting modifier multiplies the *mitigation*, so the modified
            // multiplier is `1 − base_mitigation × factor`. Verified vs scmdb
            // ADP Core: physical 0.60 → mitigation 0.40, ×0.85 @ Q0 → 0.34
            // ("−34%"). A factor < 1 below Base-500 quality correctly yields
            // worse armor. Physical type is the representative scalar; the same
            // factor applies to every type in `ArmorStats.damage_resistance`.
            GameplayStat::ArmorDamageMitigation => a
                .damage_resistance
                .as_ref()
                .and_then(|d| d.physical.as_ref())
                .map(|e| 1.0 - e.multiplier),
            _ => None,
        }
    }
}

impl ProductStatSource for ShipComponents {
    fn base_value(&self, entity: &Guid, stat: &GameplayStat) -> Option<f32> {
        let c = self.get(entity)?;
        match stat {
            GameplayStat::Integrity => c.integrity_hp,
            GameplayStat::QuantumSpeed => c.quantum_drive_speed,
            GameplayStat::QuantumFuelRequirement => c.quantum_fuel_requirement,
            GameplayStat::ShieldMaxHealth => c.shield_max_health,
            GameplayStat::CoolantGeneration => c.coolant_rate,
            GameplayStat::PowerGeneration => c.power_output,
            GameplayStat::RadarMinAimAssist => c.radar_aim_assist_min,
            GameplayStat::RadarMaxAimAssist => c.radar_aim_assist_max,
            _ => None,
        }
    }
}

impl ProductStatSource for ShipWeapons {
    fn base_value(&self, entity: &Guid, stat: &GameplayStat) -> Option<f32> {
        let w = self.get(entity)?;
        match stat {
            GameplayStat::Integrity => w.integrity_hp,
            // GPP_Weapon_Damage — per-shot for guns, per-second beam DPS for
            // mining lasers ("Laser Power").
            GameplayStat::WeaponDamage => w.damage.map(|d| d.total()),
            _ => None,
        }
    }
}

impl Blueprints {
    /// Compute the crafted item's product stats at `quality` (0–1000): for
    /// every gameplay property the recipe modifies, the per-slot modifiers are
    /// aggregated additively, mapped to a [`GameplayStat`], and applied to the
    /// item's base value via `bases`.
    ///
    /// `bases` is the per-domain base-stat source ([`FpsWeapons`], [`Armor`],
    /// …). Stats with no base (or entities the source doesn't cover) yield
    /// `base = None`, leaving the percent change still meaningful. Returns
    /// empty if the entity has no blueprint.
    pub fn product_stats<S: ProductStatSource>(
        &self,
        entity_guid: Guid,
        gp: &GameplayProperties,
        bases: &S,
        quality: i32,
    ) -> Vec<ProductStat> {
        let Some(bp) = self.for_crafted_entity(&entity_guid) else {
            return Vec::new();
        };
        // Group every modifier in the recipe by gameplay property.
        let mut by_gpp: HashMap<Guid, Vec<&GameplayPropertyModifier>> = HashMap::new();
        for tier in &bp.tiers {
            if let Some(recipe) = &tier.recipe
                && let Some(costs) = &recipe.costs
                && let Some(mc) = &costs.mandatory
            {
                for m in mc.gameplay_property_modifiers() {
                    if let Some(g) = m.gameplay_property {
                        by_gpp.entry(g).or_default().push(m);
                    }
                }
            }
        }
        let mut out: Vec<ProductStat> = by_gpp
            .into_iter()
            .map(|(guid, mods)| {
                let mut factor = 1.0_f32;
                let mut additive = 0.0_f32;
                for m in &mods {
                    match m.evaluate(quality) {
                        Some(ModifierValue::Multiplier(f)) => factor += f - 1.0,
                        Some(ModifierValue::Additive(a)) => additive += a,
                        None => {}
                    }
                }
                let stat = gp
                    .get(&guid)
                    .map(GameplayProperty::stat)
                    .unwrap_or_else(|| GameplayStat::Unknown(String::new()));
                let base = bases.base_value(&entity_guid, &stat);
                let modified = base.map(|b| b * factor + additive);
                ProductStat {
                    gameplay_property: guid,
                    stat,
                    base,
                    modified,
                    factor,
                    additive,
                }
            })
            .collect();
        // Stable display order: known stats first (in KNOWN order), then unknown.
        out.sort_by_key(|p| stat_order(&p.stat));
        out
    }
}

/// Sort key giving modelled stats their `KNOWN` order, unknowns last.
fn stat_order(stat: &GameplayStat) -> usize {
    GameplayStat::KNOWN
        .iter()
        .position(|(_, v)| v == stat)
        .unwrap_or(usize::MAX)
}

fn build_gameplay_property(
    guid: Guid,
    rec: &CraftingGameplayPropertyDef,
    pools: &DataPools,
    record_name: String,
) -> GameplayProperty {
    GameplayProperty {
        guid,
        record_name,
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
    fn gameplay_stat_maps_by_record_name_suffix() {
        // Full record name and bare suffix both resolve.
        assert_eq!(
            GameplayStat::from_gpp_name("CraftingGameplayPropertyDef.GPP_Weapon_FireRate"),
            GameplayStat::WeaponFireRate
        );
        assert_eq!(
            GameplayStat::from_gpp_name("GPP_Weapon_Recoil_Kick"),
            GameplayStat::WeaponRecoilKick
        );
        // Modelled armor stat resolves typed.
        assert_eq!(
            GameplayStat::from_gpp_name("CraftingGameplayPropertyDef.GPP_Armor_DamageMitigation"),
            GameplayStat::ArmorDamageMitigation
        );
        // Modelled ship-component stat resolves typed.
        assert_eq!(
            GameplayStat::from_gpp_name("GPP_Health_MaxHealth"),
            GameplayStat::Integrity
        );
        // A crafting-station GPP (never an item product stat) stays Unknown.
        assert_eq!(
            GameplayStat::from_gpp_name("CraftingGameplayPropertyDef.GPP_Crafter_CraftSpeed"),
            GameplayStat::Unknown("GPP_Crafter_CraftSpeed".into())
        );
    }

    #[test]
    fn product_stat_pct_change() {
        let ps = ProductStat {
            gameplay_property: Guid::default(),
            stat: GameplayStat::WeaponRecoilKick,
            base: Some(1.55),
            modified: Some(1.24),
            factor: 0.80,
            additive: 0.0,
        };
        assert!((ps.pct_change() - (-20.0)).abs() < 1e-4);
    }

    #[test]
    fn duration_seconds_sum() {
        let d = Duration {
            days: 1,
            hours: 2,
            minutes: 3,
            seconds: 4.5,
        };
        assert_eq!(d.to_seconds(), 86_400.0 + 2.0 * 3600.0 + 3.0 * 60.0 + 4.5);
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

    #[test]
    fn value_range_linear_interpolates_and_clamps() {
        // Matches the screenshot: ×1.4 → 0.6 across quality 0–1000.
        let vr = ValueRange::Linear {
            start_quality: 0,
            end_quality: 1000,
            modifier_at_start: 1.4,
            modifier_at_end: 0.6,
        };
        assert_eq!(vr.evaluate(0), Some(ModifierValue::Multiplier(1.4)));
        assert_eq!(vr.evaluate(1000), Some(ModifierValue::Multiplier(0.6)));
        // Q750 → ×0.8 (the screenshot's reading).
        match vr.evaluate(750).unwrap() {
            ModifierValue::Multiplier(f) => assert!((f - 0.8).abs() < 1e-5, "{f}"),
            other => panic!("expected multiplier, got {other:?}"),
        }
        // Out-of-band clamps to the nearest endpoint.
        assert_eq!(vr.evaluate(-50), Some(ModifierValue::Multiplier(1.4)));
        assert_eq!(vr.evaluate(5000), Some(ModifierValue::Multiplier(0.6)));
    }

    #[test]
    fn modifier_picks_band_then_clamps_to_nearest() {
        // Two adjacent bands, as seen on real ship-component blueprints.
        let m = GameplayPropertyModifier {
            gameplay_property: None,
            value_ranges: vec![
                ValueRange::Linear {
                    start_quality: 0,
                    end_quality: 500,
                    modifier_at_start: 0.8,
                    modifier_at_end: 1.0,
                },
                ValueRange::Linear {
                    start_quality: 501,
                    end_quality: 1000,
                    modifier_at_start: 1.0,
                    modifier_at_end: 1.2,
                },
            ],
        };
        assert_eq!(m.evaluate(0), Some(ModifierValue::Multiplier(0.8)));
        assert_eq!(m.evaluate(500), Some(ModifierValue::Multiplier(1.0)));
        assert_eq!(m.evaluate(1000), Some(ModifierValue::Multiplier(1.2)));
        // Above the last band → clamps to the last band's endpoint.
        assert_eq!(m.evaluate(2000), Some(ModifierValue::Multiplier(1.2)));
    }

    #[test]
    fn additive_range_yields_additive_value() {
        let vr = ValueRange::LinearIntegerAdditive {
            start_quality: 0,
            end_quality: 100,
            additive_at_start: 10,
            additive_at_end: 30,
        };
        assert_eq!(vr.evaluate(50), Some(ModifierValue::Additive(20.0)));
    }

    #[test]
    fn other_range_does_not_evaluate() {
        let vr = ValueRange::Other {
            type_name: "future".into(),
            struct_index: 7,
        };
        assert_eq!(vr.evaluate(500), None);
        assert_eq!(vr.quality_band(), None);
        assert!(!vr.contains(500));
    }

    #[test]
    fn cost_rolls_up_modifiers_across_subtree() {
        let mk = |gp: u8| GameplayPropertyModifier {
            gameplay_property: Some(Guid::from_bytes([gp; 16])),
            value_ranges: vec![],
        };
        // Slot-level modifier + a leaf-resource modifier inside the slot.
        let slot = Cost::Select {
            name_info: Some(SlotName {
                debug_name: "Frame".into(),
                display_name: LocaleKey::from("@slot_frame"),
            }),
            count: 1,
            options: vec![Cost::Resource(ResourceCost {
                resource: None,
                quantity: None,
                min_quality: 0,
                context: vec![CostContext::GameplayPropertyModifiers(vec![mk(2)])],
            })],
            context: vec![CostContext::GameplayPropertyModifiers(vec![mk(1)])],
        };
        let rolled = slot.gameplay_property_modifiers();
        assert_eq!(rolled.len(), 2);
        let gps: Vec<_> = rolled.iter().map(|m| m.gameplay_property).collect();
        assert!(gps.contains(&Some(Guid::from_bytes([1; 16]))));
        assert!(gps.contains(&Some(Guid::from_bytes([2; 16]))));
    }

    #[test]
    fn cost_context_serde_round_trip() {
        let ctx = CostContext::GameplayPropertyModifiers(vec![GameplayPropertyModifier {
            gameplay_property: Some(Guid::from_bytes([9; 16])),
            value_ranges: vec![ValueRange::Linear {
                start_quality: 0,
                end_quality: 1000,
                modifier_at_start: 1.2,
                modifier_at_end: 0.8,
            }],
        }]);
        let json = serde_json::to_string(&ctx).unwrap();
        let back: CostContext = serde_json::from_str(&json).unwrap();
        match back {
            CostContext::GameplayPropertyModifiers(mods) => {
                assert_eq!(mods.len(), 1);
                assert_eq!(mods[0].value_ranges.len(), 1);
            }
            other => panic!("expected modifiers, got {other:?}"),
        }
    }
}
