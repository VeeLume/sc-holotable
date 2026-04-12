// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `lootgeneration`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `EntryOptionalData_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryOptionalData_Base {
}

impl Pooled for EntryOptionalData_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.entry_optional_data_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.entry_optional_data_base }
}

impl<'a> Extract<'a> for EntryOptionalData_Base {
    const TYPE_NAME: &'static str = "EntryOptionalData_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootArchetypeEntry_Primary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeEntry_Primary {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `additionalTags` (Class)
    #[serde(default)]
    pub additional_tags: Option<Handle<TagsDNFTerm>>,
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// `optionalData` (StrongPointer (array))
    #[serde(default)]
    pub optional_data: Vec<Handle<EntryOptionalData_Base>>,
}

impl Pooled for LootArchetypeEntry_Primary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype_entry_primary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype_entry_primary }
}

impl<'a> Extract<'a> for LootArchetypeEntry_Primary {
    const TYPE_NAME: &'static str = "LootArchetypeEntry_Primary";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            additional_tags: match inst.get("additionalTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNFTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weight: inst.get_f32("weight").unwrap_or_default(),
            optional_data: inst.get_array("optionalData")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntryOptionalData_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EntryOptionalData_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootArchetypeEntry_Secondary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeEntry_Secondary {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for LootArchetypeEntry_Secondary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype_entry_secondary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype_entry_secondary }
}

impl<'a> Extract<'a> for LootArchetypeEntry_Secondary {
    const TYPE_NAME: &'static str = "LootArchetypeEntry_Secondary";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `LootArchetypeOrGroup_Primary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeOrGroup_Primary {
    /// `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<LootArchetypeEntry_Primary>>,
}

impl Pooled for LootArchetypeOrGroup_Primary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype_or_group_primary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype_or_group_primary }
}

impl<'a> Extract<'a> for LootArchetypeOrGroup_Primary {
    const TYPE_NAME: &'static str = "LootArchetypeOrGroup_Primary";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootArchetypeEntry_Primary>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LootArchetypeEntry_Primary>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootArchetypeOrGroup_Secondary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeOrGroup_Secondary {
    /// `groupName` (String)
    #[serde(default)]
    pub group_name: String,
    /// `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<LootArchetypeEntry_Secondary>>,
}

impl Pooled for LootArchetypeOrGroup_Secondary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype_or_group_secondary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype_or_group_secondary }
}

impl<'a> Extract<'a> for LootArchetypeOrGroup_Secondary {
    const TYPE_NAME: &'static str = "LootArchetypeOrGroup_Secondary";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            group_name: inst.get_str("groupName").map(String::from).unwrap_or_default(),
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootArchetypeEntry_Secondary>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LootArchetypeEntry_Secondary>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetype {
    /// `excludedTags` (Class)
    #[serde(default)]
    pub excluded_tags: Option<Handle<TagList>>,
    /// `primaryOrGroup` (Class)
    #[serde(default)]
    pub primary_or_group: Option<Handle<LootArchetypeOrGroup_Primary>>,
    /// `secondaryOrGroups` (Class (array))
    #[serde(default)]
    pub secondary_or_groups: Vec<Handle<LootArchetypeOrGroup_Secondary>>,
}

impl Pooled for LootArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype }
}

impl<'a> Extract<'a> for LootArchetype {
    const TYPE_NAME: &'static str = "LootArchetype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            excluded_tags: match inst.get("excludedTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            primary_or_group: match inst.get("primaryOrGroup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootArchetypeOrGroup_Primary>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootArchetypeOrGroup_Primary>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            secondary_or_groups: inst.get_array("secondaryOrGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootArchetypeOrGroup_Secondary>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LootArchetypeOrGroup_Secondary>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `NumResultsConstraints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumResultsConstraints {
    /// `minResults` (Int32)
    #[serde(default)]
    pub min_results: i32,
    /// `maxResults` (Int32)
    #[serde(default)]
    pub max_results: i32,
}

impl Pooled for NumResultsConstraints {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.num_results_constraints }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.num_results_constraints }
}

impl<'a> Extract<'a> for NumResultsConstraints {
    const TYPE_NAME: &'static str = "NumResultsConstraints";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_results: inst.get_i32("minResults").unwrap_or_default(),
            max_results: inst.get_i32("maxResults").unwrap_or_default(),
        }
    }
}

/// DCB type: `WeightedLootArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedLootArchetype {
    /// `archetype` (Reference)
    #[serde(default)]
    pub archetype: Option<CigGuid>,
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// `numberOfResultsConstraints` (Class)
    #[serde(default)]
    pub number_of_results_constraints: Option<Handle<NumResultsConstraints>>,
}

impl Pooled for WeightedLootArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.weighted_loot_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.weighted_loot_archetype }
}

impl<'a> Extract<'a> for WeightedLootArchetype {
    const TYPE_NAME: &'static str = "WeightedLootArchetype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            archetype: inst.get("archetype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weight: inst.get_f32("weight").unwrap_or_default(),
            number_of_results_constraints: match inst.get("numberOfResultsConstraints") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<NumResultsConstraints>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<NumResultsConstraints>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootTable`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    /// `lootArchetypes` (Class (array))
    #[serde(default)]
    pub loot_archetypes: Vec<Handle<WeightedLootArchetype>>,
}

impl Pooled for LootTable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_table }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_table }
}

impl<'a> Extract<'a> for LootTable {
    const TYPE_NAME: &'static str = "LootTable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loot_archetypes: inst.get_array("lootArchetypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WeightedLootArchetype>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WeightedLootArchetype>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootGenerationSpecialEventArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootGenerationSpecialEventArchetype {
    /// `eventString` (String)
    #[serde(default)]
    pub event_string: String,
    /// `probabilityPerContainer` (Single)
    #[serde(default)]
    pub probability_per_container: f32,
    /// `minEntriesPerContainer` (Int32)
    #[serde(default)]
    pub min_entries_per_container: i32,
    /// `maxEntriesPerContainer` (Int32)
    #[serde(default)]
    pub max_entries_per_container: i32,
    /// `archetype` (Reference)
    #[serde(default)]
    pub archetype: Option<CigGuid>,
    /// `archetypeV3` (StrongPointer)
    #[serde(default)]
    pub archetype_v3: Option<Handle<LootArchetypeV3_Base>>,
}

impl Pooled for LootGenerationSpecialEventArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_generation_special_event_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_generation_special_event_archetype }
}

impl<'a> Extract<'a> for LootGenerationSpecialEventArchetype {
    const TYPE_NAME: &'static str = "LootGenerationSpecialEventArchetype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            event_string: inst.get_str("eventString").map(String::from).unwrap_or_default(),
            probability_per_container: inst.get_f32("probabilityPerContainer").unwrap_or_default(),
            min_entries_per_container: inst.get_i32("minEntriesPerContainer").unwrap_or_default(),
            max_entries_per_container: inst.get_i32("maxEntriesPerContainer").unwrap_or_default(),
            archetype: inst.get("archetype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            archetype_v3: match inst.get("archetypeV3") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootArchetypeV3_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootArchetypeV3_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PoolFilter_NoRef`
/// Inherits from: `PoolFilter_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolFilter_NoRef {
}

impl Pooled for PoolFilter_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.pool_filter_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.pool_filter_no_ref }
}

impl<'a> Extract<'a> for PoolFilter_NoRef {
    const TYPE_NAME: &'static str = "PoolFilter_NoRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PoolFilterRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolFilterRecord {
    /// `filter` (StrongPointer)
    #[serde(default)]
    pub filter: Option<Handle<PoolFilter_NoRef>>,
}

impl Pooled for PoolFilterRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.pool_filter_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.pool_filter_record }
}

impl<'a> Extract<'a> for PoolFilterRecord {
    const TYPE_NAME: &'static str = "PoolFilterRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter: match inst.get("filter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PoolFilter_NoRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PoolFilter_NoRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootTableV3_NoRef`
/// Inherits from: `LootTableV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableV3_NoRef {
}

impl Pooled for LootTableV3_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_table_v3_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_table_v3_no_ref }
}

impl<'a> Extract<'a> for LootTableV3_NoRef {
    const TYPE_NAME: &'static str = "LootTableV3_NoRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootTableV3Record`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableV3Record {
    /// `lootTable` (StrongPointer)
    #[serde(default)]
    pub loot_table: Option<Handle<LootTableV3_NoRef>>,
}

impl Pooled for LootTableV3Record {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_table_v3_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_table_v3_record }
}

impl<'a> Extract<'a> for LootTableV3Record {
    const TYPE_NAME: &'static str = "LootTableV3Record";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loot_table: match inst.get("lootTable") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootTableV3_NoRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootTableV3_NoRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootArchetypeV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeV3_Base {
}

impl Pooled for LootArchetypeV3_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype_v3_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype_v3_base }
}

impl<'a> Extract<'a> for LootArchetypeV3_Base {
    const TYPE_NAME: &'static str = "LootArchetypeV3_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootArchetypeV3_NoRef`
/// Inherits from: `LootArchetypeV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeV3_NoRef {
}

impl Pooled for LootArchetypeV3_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype_v3_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype_v3_no_ref }
}

impl<'a> Extract<'a> for LootArchetypeV3_NoRef {
    const TYPE_NAME: &'static str = "LootArchetypeV3_NoRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootArchetypeV3Record`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeV3Record {
    /// `lootArchetype` (StrongPointer)
    #[serde(default)]
    pub loot_archetype: Option<Handle<LootArchetypeV3_NoRef>>,
}

impl Pooled for LootArchetypeV3Record {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_archetype_v3_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_archetype_v3_record }
}

impl<'a> Extract<'a> for LootArchetypeV3Record {
    const TYPE_NAME: &'static str = "LootArchetypeV3Record";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loot_archetype: match inst.get("lootArchetype") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootArchetypeV3_NoRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootArchetypeV3_NoRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootV3SecondaryChoicesSingleLayerRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoicesSingleLayerRecord {
    /// `choices` (Class (array))
    #[serde(default)]
    pub choices: Vec<Handle<LootV3SecondaryChoiceEntry>>,
}

impl Pooled for LootV3SecondaryChoicesSingleLayerRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_v3_secondary_choices_single_layer_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_v3_secondary_choices_single_layer_record }
}

impl<'a> Extract<'a> for LootV3SecondaryChoicesSingleLayerRecord {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoicesSingleLayerRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            choices: inst.get_array("choices")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootV3SecondaryChoiceEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LootV3SecondaryChoiceEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootV3SecondaryChoicesMultiLayerRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoicesMultiLayerRecord {
    /// `layers` (Reference (array))
    #[serde(default)]
    pub layers: Vec<CigGuid>,
}

impl Pooled for LootV3SecondaryChoicesMultiLayerRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_v3_secondary_choices_multi_layer_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_v3_secondary_choices_multi_layer_record }
}

impl<'a> Extract<'a> for LootV3SecondaryChoicesMultiLayerRecord {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoicesMultiLayerRecord";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            layers: inst.get_array("layers")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootV3SecondaryChoiceEntrySelector_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoiceEntrySelector_Base {
}

impl Pooled for LootV3SecondaryChoiceEntrySelector_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_v3_secondary_choice_entry_selector_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_v3_secondary_choice_entry_selector_base }
}

impl<'a> Extract<'a> for LootV3SecondaryChoiceEntrySelector_Base {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoiceEntrySelector_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootV3SecondaryChoiceEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoiceEntry {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// `selector` (StrongPointer)
    #[serde(default)]
    pub selector: Option<Handle<LootV3SecondaryChoiceEntrySelector_Base>>,
}

impl Pooled for LootV3SecondaryChoiceEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_v3_secondary_choice_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_v3_secondary_choice_entry }
}

impl<'a> Extract<'a> for LootV3SecondaryChoiceEntry {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoiceEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
            selector: match inst.get("selector") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootV3SecondaryChoiceEntrySelector_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootV3SecondaryChoiceEntrySelector_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootGenerationGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootGenerationGlobalParams {
    /// `specialEventArchetypes` (Class (array))
    #[serde(default)]
    pub special_event_archetypes: Vec<Handle<LootGenerationSpecialEventArchetype>>,
}

impl Pooled for LootGenerationGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_generation_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_generation_global_params }
}

impl<'a> Extract<'a> for LootGenerationGlobalParams {
    const TYPE_NAME: &'static str = "LootGenerationGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            special_event_archetypes: inst.get_array("specialEventArchetypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootGenerationSpecialEventArchetype>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LootGenerationSpecialEventArchetype>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

