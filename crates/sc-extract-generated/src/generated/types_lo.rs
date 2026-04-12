// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `LocomotionAnimSyncConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomotionAnimSyncConfig {
    /// DCB field: `footJoints` (String (array))
    #[serde(default)]
    pub foot_joints: Vec<String>,
    /// DCB field: `syncMethod` (EnumChoice)
    #[serde(default)]
    pub sync_method: String,
}

impl Pooled for LocomotionAnimSyncConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.locomotion_anim_sync_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.locomotion_anim_sync_config }
}

impl<'a> Extract<'a> for LocomotionAnimSyncConfig {
    const TYPE_NAME: &'static str = "LocomotionAnimSyncConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            foot_joints: inst.get_array("footJoints")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            sync_method: inst.get_str("syncMethod").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `LocalPlayerSpeedThrottleComponent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalPlayerSpeedThrottleComponent {
    /// DCB field: `params` (Class)
    #[serde(default)]
    pub params: Option<Handle<SpeedThrottleConfiguration>>,
}

impl Pooled for LocalPlayerSpeedThrottleComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.local_player_speed_throttle_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.local_player_speed_throttle_component }
}

impl<'a> Extract<'a> for LocalPlayerSpeedThrottleComponent {
    const TYPE_NAME: &'static str = "LocalPlayerSpeedThrottleComponent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SpeedThrottleConfiguration>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SpeedThrottleConfiguration>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LongTermPersistenceSubTypeListOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPersistenceSubTypeListOption {
}

impl Pooled for LongTermPersistenceSubTypeListOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.long_term_persistence_sub_type_list_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.long_term_persistence_sub_type_list_option }
}

impl<'a> Extract<'a> for LongTermPersistenceSubTypeListOption {
    const TYPE_NAME: &'static str = "LongTermPersistenceSubTypeListOption";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LongTermPersistenceWhiteListEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPersistenceWhiteListEntry {
    /// DCB field: `ItemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `SubTypeListOption` (StrongPointer)
    #[serde(default)]
    pub sub_type_list_option: Option<Handle<LongTermPersistenceSubTypeListOption>>,
}

impl Pooled for LongTermPersistenceWhiteListEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.long_term_persistence_white_list_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.long_term_persistence_white_list_entry }
}

impl<'a> Extract<'a> for LongTermPersistenceWhiteListEntry {
    const TYPE_NAME: &'static str = "LongTermPersistenceWhiteListEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("ItemType").map(String::from).unwrap_or_default(),
            sub_type_list_option: match inst.get("SubTypeListOption") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LongTermPersistenceSubTypeListOption>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LongTermPersistenceSubTypeListOption>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LongTermPersistenceGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPersistenceGlobalParams {
    /// DCB field: `LongTermPersistenceWhiteList` (Class (array))
    #[serde(default)]
    pub long_term_persistence_white_list: Vec<Handle<LongTermPersistenceWhiteListEntry>>,
}

impl Pooled for LongTermPersistenceGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.long_term_persistence_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.long_term_persistence_global_params }
}

impl<'a> Extract<'a> for LongTermPersistenceGlobalParams {
    const TYPE_NAME: &'static str = "LongTermPersistenceGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            long_term_persistence_white_list: inst.get_array("LongTermPersistenceWhiteList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LongTermPersistenceWhiteListEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LongTermPersistenceWhiteListEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootArchetypeEntry_Primary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeEntry_Primary {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `additionalTags` (Class)
    #[serde(default)]
    pub additional_tags: Option<Handle<TagsDNFTerm>>,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// DCB field: `optionalData` (StrongPointer (array))
    #[serde(default)]
    pub optional_data: Vec<Handle<EntryOptionalData_Base>>,
}

impl Pooled for LootArchetypeEntry_Primary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype_entry_primary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype_entry_primary }
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
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for LootArchetypeEntry_Secondary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype_entry_secondary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype_entry_secondary }
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
    /// DCB field: `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<LootArchetypeEntry_Primary>>,
}

impl Pooled for LootArchetypeOrGroup_Primary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype_or_group_primary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype_or_group_primary }
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
    /// DCB field: `groupName` (String)
    #[serde(default)]
    pub group_name: String,
    /// DCB field: `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<LootArchetypeEntry_Secondary>>,
}

impl Pooled for LootArchetypeOrGroup_Secondary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype_or_group_secondary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype_or_group_secondary }
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
    /// DCB field: `excludedTags` (Class)
    #[serde(default)]
    pub excluded_tags: Option<Handle<TagList>>,
    /// DCB field: `primaryOrGroup` (Class)
    #[serde(default)]
    pub primary_or_group: Option<Handle<LootArchetypeOrGroup_Primary>>,
    /// DCB field: `secondaryOrGroups` (Class (array))
    #[serde(default)]
    pub secondary_or_groups: Vec<Handle<LootArchetypeOrGroup_Secondary>>,
}

impl Pooled for LootArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype }
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

/// DCB type: `LootTable`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    /// DCB field: `lootArchetypes` (Class (array))
    #[serde(default)]
    pub loot_archetypes: Vec<Handle<WeightedLootArchetype>>,
}

impl Pooled for LootTable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_table }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_table }
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

/// DCB type: `LootConstraints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootConstraints {
    /// DCB field: `poolFilter` (Reference)
    #[serde(default)]
    pub pool_filter: Option<CigGuid>,
    /// DCB field: `secondaryChoices` (StrongPointer)
    #[serde(default)]
    pub secondary_choices: Option<Handle<LootV3SecondaryChoicesRecordRef_Base>>,
    /// DCB field: `fullnessFactorRange` (Class)
    #[serde(default)]
    pub fullness_factor_range: Option<Handle<FloatFactorRange>>,
    /// DCB field: `totalResultsLimit` (Int32)
    #[serde(default)]
    pub total_results_limit: i32,
    /// DCB field: `chanceToGenerate` (Single)
    #[serde(default)]
    pub chance_to_generate: f32,
    /// DCB field: `chanceToGenerateAdditionalAttachedInventories` (Single)
    #[serde(default)]
    pub chance_to_generate_additional_attached_inventories: f32,
    /// DCB field: `advanced` (Class)
    #[serde(default)]
    pub advanced: Option<Handle<AdvancedLootConstraints>>,
}

impl Pooled for LootConstraints {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_constraints }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_constraints }
}

impl<'a> Extract<'a> for LootConstraints {
    const TYPE_NAME: &'static str = "LootConstraints";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pool_filter: inst.get("poolFilter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            secondary_choices: match inst.get("secondaryChoices") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootV3SecondaryChoicesRecordRef_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootV3SecondaryChoicesRecordRef_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fullness_factor_range: match inst.get("fullnessFactorRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FloatFactorRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FloatFactorRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            total_results_limit: inst.get_i32("totalResultsLimit").unwrap_or_default(),
            chance_to_generate: inst.get_f32("chanceToGenerate").unwrap_or_default(),
            chance_to_generate_additional_attached_inventories: inst.get_f32("chanceToGenerateAdditionalAttachedInventories").unwrap_or_default(),
            advanced: match inst.get("advanced") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AdvancedLootConstraints>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AdvancedLootConstraints>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootConfig {
    /// DCB field: `lootConstraints` (Class)
    #[serde(default)]
    pub loot_constraints: Option<Handle<LootConstraints>>,
    /// DCB field: `lootTable` (Reference)
    #[serde(default)]
    pub loot_table: Option<CigGuid>,
    /// DCB field: `lootTableV3` (Reference)
    #[serde(default)]
    pub loot_table_v3: Option<CigGuid>,
}

impl Pooled for LootConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_config }
}

impl<'a> Extract<'a> for LootConfig {
    const TYPE_NAME: &'static str = "LootConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loot_constraints: match inst.get("lootConstraints") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootConstraints>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootConstraints>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loot_table: inst.get("lootTable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            loot_table_v3: inst.get("lootTableV3").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `LootGenerationSpecialEventArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootGenerationSpecialEventArchetype {
    /// DCB field: `eventString` (String)
    #[serde(default)]
    pub event_string: String,
    /// DCB field: `probabilityPerContainer` (Single)
    #[serde(default)]
    pub probability_per_container: f32,
    /// DCB field: `minEntriesPerContainer` (Int32)
    #[serde(default)]
    pub min_entries_per_container: i32,
    /// DCB field: `maxEntriesPerContainer` (Int32)
    #[serde(default)]
    pub max_entries_per_container: i32,
    /// DCB field: `archetype` (Reference)
    #[serde(default)]
    pub archetype: Option<CigGuid>,
    /// DCB field: `archetypeV3` (StrongPointer)
    #[serde(default)]
    pub archetype_v3: Option<Handle<LootArchetypeV3_Base>>,
}

impl Pooled for LootGenerationSpecialEventArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_generation_special_event_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_generation_special_event_archetype }
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

/// DCB type: `LootTableV3_NoRef`
///
/// Inherits from: `LootTableV3_Base` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableV3_NoRef {
}

impl Pooled for LootTableV3_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_table_v3_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_table_v3_no_ref }
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
    /// DCB field: `lootTable` (StrongPointer)
    #[serde(default)]
    pub loot_table: Option<Handle<LootTableV3_NoRef>>,
}

impl Pooled for LootTableV3Record {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_table_v3_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_table_v3_record }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype_v3_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype_v3_base }
}

impl<'a> Extract<'a> for LootArchetypeV3_Base {
    const TYPE_NAME: &'static str = "LootArchetypeV3_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootArchetypeV3_NoRef`
///
/// Inherits from: `LootArchetypeV3_Base` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeV3_NoRef {
}

impl Pooled for LootArchetypeV3_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype_v3_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype_v3_no_ref }
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
    /// DCB field: `lootArchetype` (StrongPointer)
    #[serde(default)]
    pub loot_archetype: Option<Handle<LootArchetypeV3_NoRef>>,
}

impl Pooled for LootArchetypeV3Record {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_archetype_v3_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_archetype_v3_record }
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

/// DCB type: `LootV3SecondaryChoicesRecordRef_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoicesRecordRef_Base {
}

impl Pooled for LootV3SecondaryChoicesRecordRef_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_v3_secondary_choices_record_ref_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_v3_secondary_choices_record_ref_base }
}

impl<'a> Extract<'a> for LootV3SecondaryChoicesRecordRef_Base {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoicesRecordRef_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootV3SecondaryChoicesSingleLayerRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoicesSingleLayerRecord {
    /// DCB field: `choices` (Class (array))
    #[serde(default)]
    pub choices: Vec<Handle<LootV3SecondaryChoiceEntry>>,
}

impl Pooled for LootV3SecondaryChoicesSingleLayerRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_v3_secondary_choices_single_layer_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_v3_secondary_choices_single_layer_record }
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
    /// DCB field: `layers` (Reference (array))
    #[serde(default)]
    pub layers: Vec<CigGuid>,
}

impl Pooled for LootV3SecondaryChoicesMultiLayerRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_v3_secondary_choices_multi_layer_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_v3_secondary_choices_multi_layer_record }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_v3_secondary_choice_entry_selector_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_v3_secondary_choice_entry_selector_base }
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
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// DCB field: `selector` (StrongPointer)
    #[serde(default)]
    pub selector: Option<Handle<LootV3SecondaryChoiceEntrySelector_Base>>,
}

impl Pooled for LootV3SecondaryChoiceEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_v3_secondary_choice_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_v3_secondary_choice_entry }
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
    /// DCB field: `specialEventArchetypes` (Class (array))
    #[serde(default)]
    pub special_event_archetypes: Vec<Handle<LootGenerationSpecialEventArchetype>>,
}

impl Pooled for LootGenerationGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loot_generation_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loot_generation_global_params }
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

/// DCB type: `LocationMissionLimit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMissionLimit {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `autoConsumeIfPlayerAtLocation` (Boolean)
    #[serde(default)]
    pub auto_consume_if_player_at_location: bool,
    /// DCB field: `maxMissionInstances` (Int32)
    #[serde(default)]
    pub max_mission_instances: i32,
    /// DCB field: `dependentParentTags` (Class)
    #[serde(default)]
    pub dependent_parent_tags: Option<Handle<TagList>>,
    /// DCB field: `dependentChildTags` (Class)
    #[serde(default)]
    pub dependent_child_tags: Option<Handle<TagList>>,
}

impl Pooled for LocationMissionLimit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.location_mission_limit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.location_mission_limit }
}

impl<'a> Extract<'a> for LocationMissionLimit {
    const TYPE_NAME: &'static str = "LocationMissionLimit";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            auto_consume_if_player_at_location: inst.get_bool("autoConsumeIfPlayerAtLocation").unwrap_or_default(),
            max_mission_instances: inst.get_i32("maxMissionInstances").unwrap_or_default(),
            dependent_parent_tags: match inst.get("dependentParentTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dependent_child_tags: match inst.get("dependentChildTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LocationResourceSlot`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResourceSlot {
    /// DCB field: `resourceSlotTag` (Reference)
    #[serde(default)]
    pub resource_slot_tag: Option<CigGuid>,
    /// DCB field: `autoConsumeIfPlayerAtLocation` (Boolean)
    #[serde(default)]
    pub auto_consume_if_player_at_location: bool,
}

impl Pooled for LocationResourceSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.location_resource_slot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.location_resource_slot }
}

impl<'a> Extract<'a> for LocationResourceSlot {
    const TYPE_NAME: &'static str = "LocationResourceSlot";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource_slot_tag: inst.get("resourceSlotTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            auto_consume_if_player_at_location: inst.get_bool("autoConsumeIfPlayerAtLocation").unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationEntityType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationEntityType_Base {
}

impl Pooled for LocationEntityType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.location_entity_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.location_entity_type_base }
}

impl<'a> Extract<'a> for LocationEntityType_Base {
    const TYPE_NAME: &'static str = "LocationEntityType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LocationEntityDeclaration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationEntityDeclaration {
    /// DCB field: `staticEntityTags` (Class)
    #[serde(default)]
    pub static_entity_tags: Option<Handle<TagList>>,
    /// DCB field: `entityTags` (Class)
    #[serde(default)]
    pub entity_tags: Option<Handle<TagsDNF>>,
    /// DCB field: `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<LocationEntityType_Base>>,
    /// DCB field: `resourceSlot` (Reference)
    #[serde(default)]
    pub resource_slot: Option<CigGuid>,
}

impl Pooled for LocationEntityDeclaration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.location_entity_declaration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.location_entity_declaration }
}

impl<'a> Extract<'a> for LocationEntityDeclaration {
    const TYPE_NAME: &'static str = "LocationEntityDeclaration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            static_entity_tags: match inst.get("staticEntityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entity_tags: match inst.get("entityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            r#type: match inst.get("type") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LocationEntityType_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LocationEntityType_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            resource_slot: inst.get("resourceSlot").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `LocationMusicConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMusicConfig {
    /// DCB field: `wwiseEventPrefix` (String)
    #[serde(default)]
    pub wwise_event_prefix: String,
    /// DCB field: `musicEventPrefix` (String)
    #[serde(default)]
    pub music_event_prefix: String,
    /// DCB field: `wwiseEventPrefixStarSystem` (String)
    #[serde(default)]
    pub wwise_event_prefix_star_system: String,
    /// DCB field: `musicEventPrefixStarSystem` (String)
    #[serde(default)]
    pub music_event_prefix_star_system: String,
}

impl Pooled for LocationMusicConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.location_music_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.location_music_config }
}

impl<'a> Extract<'a> for LocationMusicConfig {
    const TYPE_NAME: &'static str = "LocationMusicConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wwise_event_prefix: inst.get_str("wwiseEventPrefix").map(String::from).unwrap_or_default(),
            music_event_prefix: inst.get_str("musicEventPrefix").map(String::from).unwrap_or_default(),
            wwise_event_prefix_star_system: inst.get_str("wwiseEventPrefixStarSystem").map(String::from).unwrap_or_default(),
            music_event_prefix_star_system: inst.get_str("musicEventPrefixStarSystem").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `LootingTabParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootingTabParams {
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// DCB field: `itemCategories` (Class (array))
    #[serde(default)]
    pub item_categories: Vec<Handle<ItemCategory>>,
}

impl Pooled for LootingTabParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.looting_tab_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.looting_tab_params }
}

impl<'a> Extract<'a> for LootingTabParams {
    const TYPE_NAME: &'static str = "LootingTabParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            item_categories: inst.get_array("itemCategories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootingInventoryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootingInventoryParams {
    /// DCB field: `defaultTabIconPath` (String)
    #[serde(default)]
    pub default_tab_icon_path: String,
    /// DCB field: `groupSubTabParams` (Class)
    #[serde(default)]
    pub group_sub_tab_params: Option<Handle<LootingTabParams>>,
    /// DCB field: `locationTabParams` (Class)
    #[serde(default)]
    pub location_tab_params: Option<Handle<LootingTabParams>>,
    /// DCB field: `tabParams` (Class (array))
    #[serde(default)]
    pub tab_params: Vec<Handle<LootingTabParams>>,
}

impl Pooled for LootingInventoryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.looting_inventory_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.looting_inventory_params }
}

impl<'a> Extract<'a> for LootingInventoryParams {
    const TYPE_NAME: &'static str = "LootingInventoryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_tab_icon_path: inst.get_str("defaultTabIconPath").map(String::from).unwrap_or_default(),
            group_sub_tab_params: match inst.get("groupSubTabParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootingTabParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootingTabParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_tab_params: match inst.get("locationTabParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootingTabParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootingTabParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tab_params: inst.get_array("tabParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootingTabParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LootingTabParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LootingItemPortSizeClass`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootingItemPortSizeClass {
    /// DCB field: `sizeClass` (Int32)
    #[serde(default)]
    pub size_class: i32,
    /// DCB field: `itemPortTypeSubtypes` (Class (array))
    #[serde(default)]
    pub item_port_type_subtypes: Vec<Handle<TypeSubtypeParams>>,
}

impl Pooled for LootingItemPortSizeClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.looting_item_port_size_class }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.looting_item_port_size_class }
}

impl<'a> Extract<'a> for LootingItemPortSizeClass {
    const TYPE_NAME: &'static str = "LootingItemPortSizeClass";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size_class: inst.get_i32("sizeClass").unwrap_or_default(),
            item_port_type_subtypes: inst.get_array("itemPortTypeSubtypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TypeSubtypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TypeSubtypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutDummyComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutDummyComponentParams {
    /// DCB field: `playerTagPoint` (String)
    #[serde(default)]
    pub player_tag_point: String,
    /// DCB field: `playerIdleAnim` (String)
    #[serde(default)]
    pub player_idle_anim: String,
    /// DCB field: `playerDisplayParams` (Class)
    #[serde(default)]
    pub player_display_params: Option<Handle<UIWorldDisplay3DParams>>,
    /// DCB field: `vehicleTagPoint` (String)
    #[serde(default)]
    pub vehicle_tag_point: String,
    /// DCB field: `vehicleBoundingBox` (Class)
    #[serde(default)]
    pub vehicle_bounding_box: Option<Handle<Vec3>>,
    /// DCB field: `vehicleAngle` (Class)
    #[serde(default)]
    pub vehicle_angle: Option<Handle<Ang3>>,
    /// DCB field: `vehicleDisplayParams` (Class)
    #[serde(default)]
    pub vehicle_display_params: Option<Handle<UIWorldDisplay3DParams>>,
}

impl Pooled for LoadoutDummyComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_dummy_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_dummy_component_params }
}

impl<'a> Extract<'a> for LoadoutDummyComponentParams {
    const TYPE_NAME: &'static str = "LoadoutDummyComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            player_tag_point: inst.get_str("playerTagPoint").map(String::from).unwrap_or_default(),
            player_idle_anim: inst.get_str("playerIdleAnim").map(String::from).unwrap_or_default(),
            player_display_params: match inst.get("playerDisplayParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplay3DParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplay3DParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_tag_point: inst.get_str("vehicleTagPoint").map(String::from).unwrap_or_default(),
            vehicle_bounding_box: match inst.get("vehicleBoundingBox") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_angle: match inst.get("vehicleAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_display_params: match inst.get("vehicleDisplayParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplay3DParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplay3DParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LoadoutDummyTransformParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutDummyTransformParams {
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec3>>,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// DCB field: `lightPosition` (Class)
    #[serde(default)]
    pub light_position: Option<Handle<Vec3>>,
    /// DCB field: `boundToBox` (Boolean)
    #[serde(default)]
    pub bound_to_box: bool,
    /// DCB field: `scale` (Single)
    #[serde(default)]
    pub scale: f32,
    /// DCB field: `boundingBox` (Class)
    #[serde(default)]
    pub bounding_box: Option<Handle<Vec3>>,
    /// DCB field: `pivotAboutCenter` (Boolean)
    #[serde(default)]
    pub pivot_about_center: bool,
}

impl Pooled for LoadoutDummyTransformParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_dummy_transform_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_dummy_transform_params }
}

impl<'a> Extract<'a> for LoadoutDummyTransformParams {
    const TYPE_NAME: &'static str = "LoadoutDummyTransformParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            light_position: match inst.get("lightPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bound_to_box: inst.get_bool("boundToBox").unwrap_or_default(),
            scale: inst.get_f32("scale").unwrap_or_default(),
            bounding_box: match inst.get("boundingBox") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pivot_about_center: inst.get_bool("pivotAboutCenter").unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutItemPreviewTransformParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutItemPreviewTransformParams {
    /// DCB field: `lightPosition` (Class)
    #[serde(default)]
    pub light_position: Option<Handle<Vec3>>,
    /// DCB field: `initialRotation` (Class)
    #[serde(default)]
    pub initial_rotation: Option<Handle<Ang3>>,
    /// DCB field: `rotationChange` (Class)
    #[serde(default)]
    pub rotation_change: Option<Handle<Vec3>>,
    /// DCB field: `offsetPosition` (Class)
    #[serde(default)]
    pub offset_position: Option<Handle<Vec3>>,
    /// DCB field: `offsetScale` (Single)
    #[serde(default)]
    pub offset_scale: f32,
}

impl Pooled for LoadoutItemPreviewTransformParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_item_preview_transform_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_item_preview_transform_params }
}

impl<'a> Extract<'a> for LoadoutItemPreviewTransformParams {
    const TYPE_NAME: &'static str = "LoadoutItemPreviewTransformParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_position: match inst.get("lightPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            initial_rotation: match inst.get("initialRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_change: match inst.get("rotationChange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_position: match inst.get("offsetPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_scale: inst.get_f32("offsetScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutItemPortViewParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutItemPortViewParams {
    /// DCB field: `list` (StrongPointer (array))
    #[serde(default)]
    pub list: Vec<Handle<ItemPortViewInformation>>,
    /// DCB field: `enableSelectingModelItemPort` (Boolean)
    #[serde(default)]
    pub enable_selecting_model_item_port: bool,
}

impl Pooled for LoadoutItemPortViewParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_item_port_view_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_item_port_view_params }
}

impl<'a> Extract<'a> for LoadoutItemPortViewParams {
    const TYPE_NAME: &'static str = "LoadoutItemPortViewParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            list: inst.get_array("list")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPortViewInformation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPortViewInformation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            enable_selecting_model_item_port: inst.get_bool("enableSelectingModelItemPort").unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutItemHighlightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutItemHighlightParams {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `baseColor` (Class)
    #[serde(default)]
    pub base_color: Option<Handle<SRGBA8>>,
    /// DCB field: `highlightColor` (Class)
    #[serde(default)]
    pub highlight_color: Option<Handle<SRGBA8>>,
}

impl Pooled for LoadoutItemHighlightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_item_highlight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_item_highlight_params }
}

impl<'a> Extract<'a> for LoadoutItemHighlightParams {
    const TYPE_NAME: &'static str = "LoadoutItemHighlightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            base_color: match inst.get("baseColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_color: match inst.get("highlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LoadoutRequiredAttachmentsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutRequiredAttachmentsParams {
    /// DCB field: `requiredAttachments` (String (array))
    #[serde(default)]
    pub required_attachments: Vec<String>,
}

impl Pooled for LoadoutRequiredAttachmentsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_required_attachments_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_required_attachments_params }
}

impl<'a> Extract<'a> for LoadoutRequiredAttachmentsParams {
    const TYPE_NAME: &'static str = "LoadoutRequiredAttachmentsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            required_attachments: inst.get_array("requiredAttachments")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutCandidateRootParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutCandidateRootParams {
    /// DCB field: `candidateRootTypes` (EnumChoice (array))
    #[serde(default)]
    pub candidate_root_types: Vec<String>,
}

impl Pooled for LoadoutCandidateRootParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_candidate_root_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_candidate_root_params }
}

impl<'a> Extract<'a> for LoadoutCandidateRootParams {
    const TYPE_NAME: &'static str = "LoadoutCandidateRootParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            candidate_root_types: inst.get_array("candidateRootTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutEditorAdditionalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutEditorAdditionalParams {
}

impl Pooled for LoadoutEditorAdditionalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_editor_additional_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_editor_additional_params }
}

impl<'a> Extract<'a> for LoadoutEditorAdditionalParams {
    const TYPE_NAME: &'static str = "LoadoutEditorAdditionalParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LoadoutEditorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutEditorParams {
    /// DCB field: `dummyTransformParams` (Class)
    #[serde(default)]
    pub dummy_transform_params: Option<Handle<LoadoutDummyTransformParams>>,
    /// DCB field: `itemPreviewTransformParams` (Class)
    #[serde(default)]
    pub item_preview_transform_params: Option<Handle<LoadoutItemPreviewTransformParams>>,
    /// DCB field: `itemPortViewParams` (Class)
    #[serde(default)]
    pub item_port_view_params: Option<Handle<LoadoutItemPortViewParams>>,
    /// DCB field: `highlightParams` (Class)
    #[serde(default)]
    pub highlight_params: Option<Handle<LoadoutItemHighlightParams>>,
    /// DCB field: `requiredAttachmentsParams` (Class)
    #[serde(default)]
    pub required_attachments_params: Option<Handle<LoadoutRequiredAttachmentsParams>>,
    /// DCB field: `candidateRootParams` (Class)
    #[serde(default)]
    pub candidate_root_params: Option<Handle<LoadoutCandidateRootParams>>,
    /// DCB field: `additionalParams` (StrongPointer (array))
    #[serde(default)]
    pub additional_params: Vec<Handle<LoadoutEditorAdditionalParams>>,
    /// DCB field: `dummyEntityClass` (Reference)
    #[serde(default)]
    pub dummy_entity_class: Option<CigGuid>,
    /// DCB field: `loadoutChangeHideDuration` (Single)
    #[serde(default)]
    pub loadout_change_hide_duration: f32,
    /// DCB field: `alphabeticalSort` (Boolean)
    #[serde(default)]
    pub alphabetical_sort: bool,
    /// DCB field: `selfHosting` (Boolean)
    #[serde(default)]
    pub self_hosting: bool,
    /// DCB field: `applyAfterSave` (Boolean)
    #[serde(default)]
    pub apply_after_save: bool,
    /// DCB field: `allowRentalItems` (Boolean)
    #[serde(default)]
    pub allow_rental_items: bool,
    /// DCB field: `allowDuplicateItems` (Boolean)
    #[serde(default)]
    pub allow_duplicate_items: bool,
    /// DCB field: `persistentItemGameModeFlag` (EnumChoice (array))
    #[serde(default)]
    pub persistent_item_game_mode_flag: Vec<String>,
    /// DCB field: `idleAnim` (String)
    #[serde(default)]
    pub idle_anim: String,
    /// DCB field: `previewClip` (String)
    #[serde(default)]
    pub preview_clip: String,
    /// DCB field: `vehicleSelectDisplay` (Boolean)
    #[serde(default)]
    pub vehicle_select_display: bool,
    /// DCB field: `showRECWallet` (Boolean)
    #[serde(default)]
    pub show_recwallet: bool,
}

impl Pooled for LoadoutEditorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_editor_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_editor_params }
}

impl<'a> Extract<'a> for LoadoutEditorParams {
    const TYPE_NAME: &'static str = "LoadoutEditorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            dummy_transform_params: match inst.get("dummyTransformParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LoadoutDummyTransformParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LoadoutDummyTransformParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_preview_transform_params: match inst.get("itemPreviewTransformParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LoadoutItemPreviewTransformParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LoadoutItemPreviewTransformParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_port_view_params: match inst.get("itemPortViewParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LoadoutItemPortViewParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LoadoutItemPortViewParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LoadoutItemHighlightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LoadoutItemHighlightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            required_attachments_params: match inst.get("requiredAttachmentsParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LoadoutRequiredAttachmentsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LoadoutRequiredAttachmentsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            candidate_root_params: match inst.get("candidateRootParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LoadoutCandidateRootParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LoadoutCandidateRootParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            additional_params: inst.get_array("additionalParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LoadoutEditorAdditionalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LoadoutEditorAdditionalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            dummy_entity_class: inst.get("dummyEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            loadout_change_hide_duration: inst.get_f32("loadoutChangeHideDuration").unwrap_or_default(),
            alphabetical_sort: inst.get_bool("alphabeticalSort").unwrap_or_default(),
            self_hosting: inst.get_bool("selfHosting").unwrap_or_default(),
            apply_after_save: inst.get_bool("applyAfterSave").unwrap_or_default(),
            allow_rental_items: inst.get_bool("allowRentalItems").unwrap_or_default(),
            allow_duplicate_items: inst.get_bool("allowDuplicateItems").unwrap_or_default(),
            persistent_item_game_mode_flag: inst.get_array("persistentItemGameModeFlag")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            idle_anim: inst.get_str("idleAnim").map(String::from).unwrap_or_default(),
            preview_clip: inst.get_str("previewClip").map(String::from).unwrap_or_default(),
            vehicle_select_display: inst.get_bool("vehicleSelectDisplay").unwrap_or_default(),
            show_recwallet: inst.get_bool("showRECWallet").unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutEditorComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutEditorComponentParams {
    /// DCB field: `displayParams` (Class)
    #[serde(default)]
    pub display_params: Option<Handle<UIWorldDisplay3DParams>>,
    /// DCB field: `editorParams` (Class)
    #[serde(default)]
    pub editor_params: Option<Handle<LoadoutEditorParams>>,
    /// DCB field: `sunlightParams` (Class)
    #[serde(default)]
    pub sunlight_params: Option<Handle<RTTSunlightParams>>,
}

impl Pooled for LoadoutEditorComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_editor_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_editor_component_params }
}

impl<'a> Extract<'a> for LoadoutEditorComponentParams {
    const TYPE_NAME: &'static str = "LoadoutEditorComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_params: match inst.get("displayParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplay3DParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplay3DParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            editor_params: match inst.get("editorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LoadoutEditorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LoadoutEditorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sunlight_params: match inst.get("sunlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RTTSunlightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RTTSunlightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LoadoutKit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutKit {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for LoadoutKit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_lo.loadout_kit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_lo.loadout_kit }
}

impl<'a> Extract<'a> for LoadoutKit {
    const TYPE_NAME: &'static str = "LoadoutKit";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            loadout: match inst.get("loadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

