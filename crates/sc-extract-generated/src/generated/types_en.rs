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

/// DCB type: `EnvironmentTemperatureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentTemperatureParams {
    /// DCB field: `bodyDefaultMinTemperatureThreshold` (Single)
    #[serde(default)]
    pub body_default_min_temperature_threshold: f32,
    /// DCB field: `bodyDefaultMaxTemperatureThreshold` (Single)
    #[serde(default)]
    pub body_default_max_temperature_threshold: f32,
    /// DCB field: `suitTempLossRateModifier` (Single)
    #[serde(default)]
    pub suit_temp_loss_rate_modifier: f32,
    /// DCB field: `bodyTempIncreaseRateModifier` (Single)
    #[serde(default)]
    pub body_temp_increase_rate_modifier: f32,
    /// DCB field: `bodyTempDecreaseRateModifier` (Single)
    #[serde(default)]
    pub body_temp_decrease_rate_modifier: f32,
    /// DCB field: `suitTempRegenRateModifier` (Single)
    #[serde(default)]
    pub suit_temp_regen_rate_modifier: f32,
    /// DCB field: `bodyTempRegenRateModifier` (Single)
    #[serde(default)]
    pub body_temp_regen_rate_modifier: f32,
    /// DCB field: `minRegenRate` (Single)
    #[serde(default)]
    pub min_regen_rate: f32,
    /// DCB field: `temperatureUIParams` (Class)
    #[serde(default)]
    pub temperature_uiparams: Option<Handle<TemperatureUIParams>>,
}

impl Pooled for EnvironmentTemperatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.environment_temperature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.environment_temperature_params }
}

impl<'a> Extract<'a> for EnvironmentTemperatureParams {
    const TYPE_NAME: &'static str = "EnvironmentTemperatureParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            body_default_min_temperature_threshold: inst.get_f32("bodyDefaultMinTemperatureThreshold").unwrap_or_default(),
            body_default_max_temperature_threshold: inst.get_f32("bodyDefaultMaxTemperatureThreshold").unwrap_or_default(),
            suit_temp_loss_rate_modifier: inst.get_f32("suitTempLossRateModifier").unwrap_or_default(),
            body_temp_increase_rate_modifier: inst.get_f32("bodyTempIncreaseRateModifier").unwrap_or_default(),
            body_temp_decrease_rate_modifier: inst.get_f32("bodyTempDecreaseRateModifier").unwrap_or_default(),
            suit_temp_regen_rate_modifier: inst.get_f32("suitTempRegenRateModifier").unwrap_or_default(),
            body_temp_regen_rate_modifier: inst.get_f32("bodyTempRegenRateModifier").unwrap_or_default(),
            min_regen_rate: inst.get_f32("minRegenRate").unwrap_or_default(),
            temperature_uiparams: match inst.get("temperatureUIParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TemperatureUIParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TemperatureUIParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityAudioControllerTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAudioControllerTypeParams {
    /// DCB field: `audioControllerParams` (StrongPointer)
    #[serde(default)]
    pub audio_controller_params: Option<Handle<SEntityAudioControllerParams>>,
}

impl Pooled for EntityAudioControllerTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_audio_controller_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_audio_controller_type_params }
}

impl<'a> Extract<'a> for EntityAudioControllerTypeParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerTypeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_params: match inst.get("audioControllerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityAudioControllerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityAudioControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityAudioControllerTypeManagementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAudioControllerTypeManagementParams {
    /// DCB field: `audioControllerEntityType` (EnumChoice)
    #[serde(default)]
    pub audio_controller_entity_type: String,
    /// DCB field: `maxFullLODs` (Int32)
    #[serde(default)]
    pub max_full_lods: i32,
    /// DCB field: `maxLowLODs` (Int32)
    #[serde(default)]
    pub max_low_lods: i32,
}

impl Pooled for EntityAudioControllerTypeManagementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_audio_controller_type_management_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_audio_controller_type_management_params }
}

impl<'a> Extract<'a> for EntityAudioControllerTypeManagementParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerTypeManagementParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_entity_type: inst.get_str("audioControllerEntityType").map(String::from).unwrap_or_default(),
            max_full_lods: inst.get_i32("maxFullLODs").unwrap_or_default(),
            max_low_lods: inst.get_i32("maxLowLODs").unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityAudioControllerManagerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAudioControllerManagerParams {
    /// DCB field: `params` (Class (array))
    #[serde(default)]
    pub params: Vec<Handle<EntityAudioControllerTypeManagementParams>>,
}

impl Pooled for EntityAudioControllerManagerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_audio_controller_manager_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_audio_controller_manager_params }
}

impl<'a> Extract<'a> for EntityAudioControllerManagerParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerManagerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: inst.get_array("params")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityAudioControllerTypeManagementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EntityAudioControllerTypeManagementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityAudioControllerRtpcSubscriberListDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAudioControllerRtpcSubscriberListDef {
    /// DCB field: `rtpcs` (Class (array))
    #[serde(default)]
    pub rtpcs: Vec<Handle<AudioRtpc>>,
}

impl Pooled for EntityAudioControllerRtpcSubscriberListDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_audio_controller_rtpc_subscriber_list_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_audio_controller_rtpc_subscriber_list_def }
}

impl<'a> Extract<'a> for EntityAudioControllerRtpcSubscriberListDef {
    const TYPE_NAME: &'static str = "EntityAudioControllerRtpcSubscriberListDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rtpcs: inst.get_array("rtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntitlementItemType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementItemType {
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for EntitlementItemType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entitlement_item_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entitlement_item_type }
}

impl<'a> Extract<'a> for EntitlementItemType {
    const TYPE_NAME: &'static str = "EntitlementItemType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `EntitlementAccountItemGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementAccountItemGlobalParams {
    /// DCB field: `accountItemTypes` (Class (array))
    #[serde(default)]
    pub account_item_types: Vec<Handle<EntitlementItemType>>,
}

impl Pooled for EntitlementAccountItemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entitlement_account_item_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entitlement_account_item_global_params }
}

impl<'a> Extract<'a> for EntitlementAccountItemGlobalParams {
    const TYPE_NAME: &'static str = "EntitlementAccountItemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            account_item_types: inst.get_array("accountItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntitlementItemType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EntitlementItemType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntitlementNonInventoryStorableItemGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementNonInventoryStorableItemGlobalParams {
    /// DCB field: `itemTypes` (Class (array))
    #[serde(default)]
    pub item_types: Vec<Handle<EntitlementItemType>>,
}

impl Pooled for EntitlementNonInventoryStorableItemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entitlement_non_inventory_storable_item_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entitlement_non_inventory_storable_item_global_params }
}

impl<'a> Extract<'a> for EntitlementNonInventoryStorableItemGlobalParams {
    const TYPE_NAME: &'static str = "EntitlementNonInventoryStorableItemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntitlementItemType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EntitlementItemType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityClassDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClassDefinition {
    /// DCB field: `Category` (String)
    #[serde(default)]
    pub category: String,
    /// DCB field: `Icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `Invisible` (Boolean)
    #[serde(default)]
    pub invisible: bool,
    /// DCB field: `BBoxSelection` (Boolean)
    #[serde(default)]
    pub bbox_selection: bool,
    /// DCB field: `defaultEditorColor` (Class)
    #[serde(default)]
    pub default_editor_color: Option<Handle<RGB>>,
    /// DCB field: `entityDensityClass` (Reference)
    #[serde(default)]
    pub entity_density_class: Option<CigGuid>,
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// DCB field: `StaticEntityClassData` (StrongPointer (array))
    #[serde(default)]
    pub static_entity_class_data: Vec<Handle<EntityClassStaticDataParams>>,
    /// DCB field: `Components` (StrongPointer (array))
    #[serde(default)]
    pub components: Vec<Handle<DataForgeComponentParams>>,
}

impl Pooled for EntityClassDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_class_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_class_definition }
}

impl<'a> Extract<'a> for EntityClassDefinition {
    const TYPE_NAME: &'static str = "EntityClassDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            category: inst.get_str("Category").map(String::from).unwrap_or_default(),
            icon: inst.get_str("Icon").map(String::from).unwrap_or_default(),
            invisible: inst.get_bool("Invisible").unwrap_or_default(),
            bbox_selection: inst.get_bool("BBoxSelection").unwrap_or_default(),
            default_editor_color: match inst.get("defaultEditorColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entity_density_class: inst.get("entityDensityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            static_entity_class_data: inst.get_array("StaticEntityClassData")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityClassStaticDataParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EntityClassStaticDataParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            components: inst.get_array("Components")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DataForgeComponentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DataForgeComponentParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityClassStaticDataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClassStaticDataParams {
}

impl Pooled for EntityClassStaticDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_class_static_data_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_class_static_data_params }
}

impl<'a> Extract<'a> for EntityClassStaticDataParams {
    const TYPE_NAME: &'static str = "EntityClassStaticDataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityDefaultLoadoutParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefaultLoadoutParams {
    /// DCB field: `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for EntityDefaultLoadoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_default_loadout_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_default_loadout_params }
}

impl<'a> Extract<'a> for EntityDefaultLoadoutParams {
    const TYPE_NAME: &'static str = "EntityDefaultLoadoutParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
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

/// DCB type: `EngineeringStateMessages`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringStateMessages {
    /// DCB field: `message` (Locale)
    #[serde(default)]
    pub message: String,
}

impl Pooled for EngineeringStateMessages {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.engineering_state_messages }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.engineering_state_messages }
}

impl<'a> Extract<'a> for EngineeringStateMessages {
    const TYPE_NAME: &'static str = "EngineeringStateMessages";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `EntryOptionalData_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryOptionalData_Base {
}

impl Pooled for EntryOptionalData_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entry_optional_data_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entry_optional_data_base }
}

impl<'a> Extract<'a> for EntryOptionalData_Base {
    const TYPE_NAME: &'static str = "EntryOptionalData_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityClusterId`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClusterId {
}

impl Pooled for EntityClusterId {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_cluster_id }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_cluster_id }
}

impl<'a> Extract<'a> for EntityClusterId {
    const TYPE_NAME: &'static str = "EntityClusterId";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityClusterMember`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClusterMember {
    /// DCB field: `variables` (StrongPointer (array))
    #[serde(default)]
    pub variables: Vec<Handle<MissionVariableBase>>,
}

impl Pooled for EntityClusterMember {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.entity_cluster_member }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.entity_cluster_member }
}

impl<'a> Extract<'a> for EntityClusterMember {
    const TYPE_NAME: &'static str = "EntityClusterMember";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variables: inst.get_array("variables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionVariableBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionVariableBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EnemyAwarenessConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyAwarenessConfig {
    /// DCB field: `numAwareEnemiesParameter` (Reference)
    #[serde(default)]
    pub num_aware_enemies_parameter: Option<CigGuid>,
    /// DCB field: `numInCombatEnemiesParameter` (Reference)
    #[serde(default)]
    pub num_in_combat_enemies_parameter: Option<CigGuid>,
}

impl Pooled for EnemyAwarenessConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_en.enemy_awareness_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_en.enemy_awareness_config }
}

impl<'a> Extract<'a> for EnemyAwarenessConfig {
    const TYPE_NAME: &'static str = "EnemyAwarenessConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            num_aware_enemies_parameter: inst.get("numAwareEnemiesParameter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            num_in_combat_enemies_parameter: inst.get("numInCombatEnemiesParameter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

