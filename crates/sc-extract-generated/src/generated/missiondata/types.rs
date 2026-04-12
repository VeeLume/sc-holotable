// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missiondata`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SpawnSettingsInventoryItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnSettingsInventoryItem {
    /// `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
    /// `quantity` (Int32)
    #[serde(default)]
    pub quantity: i32,
    /// `amountInventoriesToFill` (Int32)
    #[serde(default)]
    pub amount_inventories_to_fill: i32,
    /// `markAsMissionItem` (Boolean)
    #[serde(default)]
    pub mark_as_mission_item: bool,
}

impl Pooled for SpawnSettingsInventoryItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.spawn_settings_inventory_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.spawn_settings_inventory_item }
}

impl<'a> Extract<'a> for SpawnSettingsInventoryItem {
    const TYPE_NAME: &'static str = "SpawnSettingsInventoryItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            quantity: inst.get_i32("quantity").unwrap_or_default(),
            amount_inventories_to_fill: inst.get_i32("amountInventoriesToFill").unwrap_or_default(),
            mark_as_mission_item: inst.get_bool("markAsMissionItem").unwrap_or_default(),
        }
    }
}

/// DCB type: `AutoSpawnSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSpawnSettings {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `initialActivity` (String)
    #[serde(default)]
    pub initial_activity: String,
    /// `positiveCharacterTags` (Class)
    #[serde(default)]
    pub positive_character_tags: Option<Handle<TagList>>,
    /// `negativeCharacterTags` (Class)
    #[serde(default)]
    pub negative_character_tags: Option<Handle<TagList>>,
    /// `excludeShipCrew` (Boolean)
    #[serde(default)]
    pub exclude_ship_crew: bool,
    /// `excludeSpawnGender` (EnumChoice)
    #[serde(default)]
    pub exclude_spawn_gender: String,
    /// `minGroupSize` (Int32)
    #[serde(default)]
    pub min_group_size: i32,
    /// `maxGroupSize` (Int32)
    #[serde(default)]
    pub max_group_size: i32,
    /// `maxConcurrentSpawns` (Int32)
    #[serde(default)]
    pub max_concurrent_spawns: i32,
    /// `maxSpawns` (Int32)
    #[serde(default)]
    pub max_spawns: i32,
    /// `minSpawnDelay` (Single)
    #[serde(default)]
    pub min_spawn_delay: f32,
    /// `maxSpawnDelay` (Single)
    #[serde(default)]
    pub max_spawn_delay: f32,
    /// `inventoryItems` (Class (array))
    #[serde(default)]
    pub inventory_items: Vec<Handle<SpawnSettingsInventoryItem>>,
    /// `closetPositiveTags` (Class)
    #[serde(default)]
    pub closet_positive_tags: Option<Handle<TagList>>,
    /// `closetNegativeTags` (Class)
    #[serde(default)]
    pub closet_negative_tags: Option<Handle<TagList>>,
    /// `roomPositiveTags` (Class)
    #[serde(default)]
    pub room_positive_tags: Option<Handle<TagList>>,
    /// `roomNegativeTags` (Class)
    #[serde(default)]
    pub room_negative_tags: Option<Handle<TagList>>,
    /// `defendAreaPositiveTags` (Class)
    #[serde(default)]
    pub defend_area_positive_tags: Option<Handle<TagList>>,
    /// `defendAreaNegativeTags` (Class)
    #[serde(default)]
    pub defend_area_negative_tags: Option<Handle<TagList>>,
    /// `scheduleAreaPositiveTags` (Class)
    #[serde(default)]
    pub schedule_area_positive_tags: Option<Handle<TagList>>,
    /// `scheduleAreaNegativeTags` (Class)
    #[serde(default)]
    pub schedule_area_negative_tags: Option<Handle<TagList>>,
    /// `entityTags` (Class)
    #[serde(default)]
    pub entity_tags: Option<Handle<TagList>>,
    /// `factionOverride` (Reference)
    #[serde(default)]
    pub faction_override: Option<CigGuid>,
    /// `missionAlliedMarker` (Boolean)
    #[serde(default)]
    pub mission_allied_marker: bool,
    /// `isCritical` (Boolean)
    #[serde(default)]
    pub is_critical: bool,
}

impl Pooled for AutoSpawnSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.auto_spawn_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.auto_spawn_settings }
}

impl<'a> Extract<'a> for AutoSpawnSettings {
    const TYPE_NAME: &'static str = "AutoSpawnSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            initial_activity: inst.get_str("initialActivity").map(String::from).unwrap_or_default(),
            positive_character_tags: match inst.get("positiveCharacterTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            negative_character_tags: match inst.get("negativeCharacterTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exclude_ship_crew: inst.get_bool("excludeShipCrew").unwrap_or_default(),
            exclude_spawn_gender: inst.get_str("excludeSpawnGender").map(String::from).unwrap_or_default(),
            min_group_size: inst.get_i32("minGroupSize").unwrap_or_default(),
            max_group_size: inst.get_i32("maxGroupSize").unwrap_or_default(),
            max_concurrent_spawns: inst.get_i32("maxConcurrentSpawns").unwrap_or_default(),
            max_spawns: inst.get_i32("maxSpawns").unwrap_or_default(),
            min_spawn_delay: inst.get_f32("minSpawnDelay").unwrap_or_default(),
            max_spawn_delay: inst.get_f32("maxSpawnDelay").unwrap_or_default(),
            inventory_items: inst.get_array("inventoryItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SpawnSettingsInventoryItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SpawnSettingsInventoryItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            closet_positive_tags: match inst.get("closetPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            closet_negative_tags: match inst.get("closetNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_positive_tags: match inst.get("roomPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_negative_tags: match inst.get("roomNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            defend_area_positive_tags: match inst.get("defendAreaPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            defend_area_negative_tags: match inst.get("defendAreaNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            schedule_area_positive_tags: match inst.get("scheduleAreaPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            schedule_area_negative_tags: match inst.get("scheduleAreaNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entity_tags: match inst.get("entityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            faction_override: inst.get("factionOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_allied_marker: inst.get_bool("missionAlliedMarker").unwrap_or_default(),
            is_critical: inst.get_bool("isCritical").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionLocationTags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationTags {
    /// `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for MissionLocationTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_location_tags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_location_tags }
}

impl<'a> Extract<'a> for MissionLocationTags {
    const TYPE_NAME: &'static str = "MissionLocationTags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionStringVariant`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionStringVariant {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `string` (Locale)
    #[serde(default)]
    pub string: String,
}

impl Pooled for MissionStringVariant {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_string_variant }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_string_variant }
}

impl<'a> Extract<'a> for MissionStringVariant {
    const TYPE_NAME: &'static str = "MissionStringVariant";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            string: inst.get_str("string").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionStringVariants`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionStringVariants {
    /// `variants` (Class (array))
    #[serde(default)]
    pub variants: Vec<Handle<MissionStringVariant>>,
}

impl Pooled for MissionStringVariants {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_string_variants }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_string_variants }
}

impl<'a> Extract<'a> for MissionStringVariants {
    const TYPE_NAME: &'static str = "MissionStringVariants";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variants: inst.get_array("variants")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionStringVariant>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionStringVariant>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationMissionLimit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMissionLimit {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `autoConsumeIfPlayerAtLocation` (Boolean)
    #[serde(default)]
    pub auto_consume_if_player_at_location: bool,
    /// `maxMissionInstances` (Int32)
    #[serde(default)]
    pub max_mission_instances: i32,
    /// `dependentParentTags` (Class)
    #[serde(default)]
    pub dependent_parent_tags: Option<Handle<TagList>>,
    /// `dependentChildTags` (Class)
    #[serde(default)]
    pub dependent_child_tags: Option<Handle<TagList>>,
}

impl Pooled for LocationMissionLimit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.location_mission_limit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.location_mission_limit }
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

/// DCB type: `SMissionLocationModule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMissionLocationModule {
    /// `isPersistent` (Boolean)
    #[serde(default)]
    pub is_persistent: bool,
    /// `missionModule` (String)
    #[serde(default)]
    pub mission_module: String,
    /// `missionInputs` (StrongPointer (array))
    #[serde(default)]
    pub mission_inputs: Vec<Handle<AbstractMissionInitParam>>,
    /// `properties` (Class (array))
    #[serde(default)]
    pub properties: Vec<Handle<MissionProperty>>,
}

impl Pooled for SMissionLocationModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.smission_location_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.smission_location_module }
}

impl<'a> Extract<'a> for SMissionLocationModule {
    const TYPE_NAME: &'static str = "SMissionLocationModule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            is_persistent: inst.get_bool("isPersistent").unwrap_or_default(),
            mission_module: inst.get_str("missionModule").map(String::from).unwrap_or_default(),
            mission_inputs: inst.get_array("missionInputs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AbstractMissionInitParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AbstractMissionInitParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionLocationData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationData {
    /// `generalTags` (Class)
    #[serde(default)]
    pub general_tags: Option<Handle<MissionLocationTags>>,
    /// `producesTags` (Class)
    #[serde(default)]
    pub produces_tags: Option<Handle<TagSearchTerm>>,
    /// `consumesTags` (Class)
    #[serde(default)]
    pub consumes_tags: Option<Handle<TagSearchTerm>>,
    /// `aiSpawnTags` (Class)
    #[serde(default)]
    pub ai_spawn_tags: Option<Handle<TagList>>,
    /// `stringVariants` (Class)
    #[serde(default)]
    pub string_variants: Option<Handle<MissionStringVariants>>,
    /// `missionModules` (Class (array))
    #[serde(default)]
    pub mission_modules: Vec<Handle<SMissionLocationModule>>,
    /// `missionLimits` (Class (array))
    #[serde(default)]
    pub mission_limits: Vec<Handle<LocationMissionLimit>>,
    /// `autoSpawnSettings` (Class (array))
    #[serde(default)]
    pub auto_spawn_settings: Vec<Handle<AutoSpawnSettings>>,
    /// `isSecurityNetworkHost` (Boolean)
    #[serde(default)]
    pub is_security_network_host: bool,
    /// `defaultSecurityNetworkManifest` (Reference)
    #[serde(default)]
    pub default_security_network_manifest: Option<CigGuid>,
    /// `disabled` (Boolean)
    #[serde(default)]
    pub disabled: bool,
    /// `entityClusterMember` (Reference)
    #[serde(default)]
    pub entity_cluster_member: Option<CigGuid>,
}

impl Pooled for MissionLocationData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_location_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_location_data }
}

impl<'a> Extract<'a> for MissionLocationData {
    const TYPE_NAME: &'static str = "MissionLocationData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            general_tags: match inst.get("generalTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionLocationTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionLocationTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            produces_tags: match inst.get("producesTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagSearchTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagSearchTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            consumes_tags: match inst.get("consumesTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagSearchTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagSearchTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ai_spawn_tags: match inst.get("aiSpawnTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            string_variants: match inst.get("stringVariants") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionStringVariants>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionStringVariants>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mission_modules: inst.get_array("missionModules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMissionLocationModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SMissionLocationModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            mission_limits: inst.get_array("missionLimits")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LocationMissionLimit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LocationMissionLimit>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            auto_spawn_settings: inst.get_array("autoSpawnSettings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AutoSpawnSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AutoSpawnSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            is_security_network_host: inst.get_bool("isSecurityNetworkHost").unwrap_or_default(),
            default_security_network_manifest: inst.get("defaultSecurityNetworkManifest").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            disabled: inst.get_bool("disabled").unwrap_or_default(),
            entity_cluster_member: inst.get("entityClusterMember").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionLocationTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationTemplate {
    /// `locationData` (Class)
    #[serde(default)]
    pub location_data: Option<Handle<MissionLocationData>>,
}

impl Pooled for MissionLocationTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_location_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_location_template }
}

impl<'a> Extract<'a> for MissionLocationTemplate {
    const TYPE_NAME: &'static str = "MissionLocationTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_data: match inst.get("locationData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionLocationData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionLocationData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionItem {
    /// `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// `stringVariants` (Class)
    #[serde(default)]
    pub string_variants: Option<Handle<MissionStringVariants>>,
    /// `weighting` (Single)
    #[serde(default)]
    pub weighting: f32,
    /// `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
}

impl Pooled for MissionItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_item }
}

impl<'a> Extract<'a> for MissionItem {
    const TYPE_NAME: &'static str = "MissionItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            string_variants: match inst.get("stringVariants") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionStringVariants>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionStringVariants>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weighting: inst.get_f32("weighting").unwrap_or_default(),
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionOrganization`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionOrganization {
    /// `organizationTags` (Class)
    #[serde(default)]
    pub organization_tags: Option<Handle<MissionLocationTags>>,
    /// `stringVariants` (Class)
    #[serde(default)]
    pub string_variants: Option<Handle<MissionStringVariants>>,
    /// `weighting` (Single)
    #[serde(default)]
    pub weighting: f32,
    /// `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
}

impl Pooled for MissionOrganization {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_organization }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_organization }
}

impl<'a> Extract<'a> for MissionOrganization {
    const TYPE_NAME: &'static str = "MissionOrganization";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            organization_tags: match inst.get("organizationTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionLocationTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionLocationTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            string_variants: match inst.get("stringVariants") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionStringVariants>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionStringVariants>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weighting: inst.get_f32("weighting").unwrap_or_default(),
            faction_reputation: inst.get("factionReputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `EntityClusterId`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClusterId {
}

impl Pooled for EntityClusterId {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.entity_cluster_id }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.entity_cluster_id }
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
    /// `variables` (StrongPointer (array))
    #[serde(default)]
    pub variables: Vec<Handle<MissionVariableBase>>,
}

impl Pooled for EntityClusterMember {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.entity_cluster_member }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.entity_cluster_member }
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

/// DCB type: `LocationResourceSlot`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResourceSlot {
    /// `resourceSlotTag` (Reference)
    #[serde(default)]
    pub resource_slot_tag: Option<CigGuid>,
    /// `autoConsumeIfPlayerAtLocation` (Boolean)
    #[serde(default)]
    pub auto_consume_if_player_at_location: bool,
}

impl Pooled for LocationResourceSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.location_resource_slot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.location_resource_slot }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.location_entity_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.location_entity_type_base }
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
    /// `staticEntityTags` (Class)
    #[serde(default)]
    pub static_entity_tags: Option<Handle<TagList>>,
    /// `entityTags` (Class)
    #[serde(default)]
    pub entity_tags: Option<Handle<TagsDNF>>,
    /// `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<LocationEntityType_Base>>,
    /// `resourceSlot` (Reference)
    #[serde(default)]
    pub resource_slot: Option<CigGuid>,
}

impl Pooled for LocationEntityDeclaration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.location_entity_declaration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.location_entity_declaration }
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

/// DCB type: `ModuleDeclarationType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclarationType_Base {
}

impl Pooled for ModuleDeclarationType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.module_declaration_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.module_declaration_type_base }
}

impl<'a> Extract<'a> for ModuleDeclarationType_Base {
    const TYPE_NAME: &'static str = "ModuleDeclarationType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ModuleDeclaration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclaration {
    /// `module` (String)
    #[serde(default)]
    pub module: String,
    /// `moduleType` (StrongPointer)
    #[serde(default)]
    pub module_type: Option<Handle<ModuleDeclarationType_Base>>,
    /// `properties` (Class (array))
    #[serde(default)]
    pub properties: Vec<Handle<MissionProperty>>,
}

impl Pooled for ModuleDeclaration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.module_declaration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.module_declaration }
}

impl<'a> Extract<'a> for ModuleDeclaration {
    const TYPE_NAME: &'static str = "ModuleDeclaration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            module: inst.get_str("module").map(String::from).unwrap_or_default(),
            module_type: match inst.get("moduleType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ModuleDeclarationType_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ModuleDeclarationType_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionLocality`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocality {
    /// `availableLocations` (Reference (array))
    #[serde(default)]
    pub available_locations: Vec<CigGuid>,
}

impl Pooled for MissionLocality {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.mission_locality }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.mission_locality }
}

impl<'a> Extract<'a> for MissionLocality {
    const TYPE_NAME: &'static str = "MissionLocality";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            available_locations: inst.get_array("availableLocations")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SpawnDescriptionEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnDescriptionEntry {
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `spawnGroup` (StrongPointer)
    #[serde(default)]
    pub spawn_group: Option<Handle<BaseMissionPropertyValue>>,
}

impl Pooled for SpawnDescriptionEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.spawn_description_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.spawn_description_entry }
}

impl<'a> Extract<'a> for SpawnDescriptionEntry {
    const TYPE_NAME: &'static str = "SpawnDescriptionEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            spawn_group: match inst.get("spawnGroup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseMissionPropertyValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseMissionPropertyValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SpawnDescriptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnDescriptions {
    /// `spawnDescriptions` (Class (array))
    #[serde(default)]
    pub spawn_descriptions: Vec<Handle<SpawnDescriptionEntry>>,
}

impl Pooled for SpawnDescriptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiondata.spawn_descriptions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiondata.spawn_descriptions }
}

impl<'a> Extract<'a> for SpawnDescriptions {
    const TYPE_NAME: &'static str = "SpawnDescriptions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spawn_descriptions: inst.get_array("spawnDescriptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SpawnDescriptionEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SpawnDescriptionEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

