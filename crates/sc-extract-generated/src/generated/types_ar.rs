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

/// DCB type: `ARMarkerPlayerOffsetParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARMarkerPlayerOffsetParams {
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `offsetProportionPerMeter` (Class)
    #[serde(default)]
    pub offset_proportion_per_meter: Option<Handle<Vec3>>,
}

impl Pooled for ARMarkerPlayerOffsetParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.armarker_player_offset_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.armarker_player_offset_params }
}

impl<'a> Extract<'a> for ARMarkerPlayerOffsetParams {
    const TYPE_NAME: &'static str = "ARMarkerPlayerOffsetParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_proportion_per_meter: match inst.get("offsetProportionPerMeter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ARMarkerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARMarkerGlobalParams {
    /// DCB field: `ARMarkerPlayerOffset` (Class)
    #[serde(default)]
    pub armarker_player_offset: Option<Handle<ARMarkerPlayerOffsetParams>>,
}

impl Pooled for ARMarkerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.armarker_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.armarker_global_params }
}

impl<'a> Extract<'a> for ARMarkerGlobalParams {
    const TYPE_NAME: &'static str = "ARMarkerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            armarker_player_offset: match inst.get("ARMarkerPlayerOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ARMarkerPlayerOffsetParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ARMarkerPlayerOffsetParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ArmsLockSingleAbility`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmsLockSingleAbility {
    /// DCB field: `ability` (EnumChoice)
    #[serde(default)]
    pub ability: String,
    /// DCB field: `exclusions` (EnumChoice (array))
    #[serde(default)]
    pub exclusions: Vec<String>,
}

impl Pooled for ArmsLockSingleAbility {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.arms_lock_single_ability }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.arms_lock_single_ability }
}

impl<'a> Extract<'a> for ArmsLockSingleAbility {
    const TYPE_NAME: &'static str = "ArmsLockSingleAbility";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ability: inst.get_str("ability").map(String::from).unwrap_or_default(),
            exclusions: inst.get_array("exclusions")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ArmsLockConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmsLockConfig {
    /// DCB field: `minWeight` (Single)
    #[serde(default)]
    pub min_weight: f32,
    /// DCB field: `maxWeight` (Single)
    #[serde(default)]
    pub max_weight: f32,
    /// DCB field: `baseMinDuration` (Single)
    #[serde(default)]
    pub base_min_duration: f32,
    /// DCB field: `baseMaxDuration` (Single)
    #[serde(default)]
    pub base_max_duration: f32,
    /// DCB field: `baseMinShaking` (Single)
    #[serde(default)]
    pub base_min_shaking: f32,
    /// DCB field: `baseMaxShaking` (Single)
    #[serde(default)]
    pub base_max_shaking: f32,
    /// DCB field: `heavyItemDropDelay` (Single)
    #[serde(default)]
    pub heavy_item_drop_delay: f32,
    /// DCB field: `shakingDecayRate` (Single)
    #[serde(default)]
    pub shaking_decay_rate: f32,
    /// DCB field: `singleUseAbilities` (Class (array))
    #[serde(default)]
    pub single_use_abilities: Vec<Handle<ArmsLockSingleAbility>>,
    /// DCB field: `abilitiesToLock` (EnumChoice (array))
    #[serde(default)]
    pub abilities_to_lock: Vec<String>,
    /// DCB field: `abilitiesThatPreventArmsLowering` (EnumChoice (array))
    #[serde(default)]
    pub abilities_that_prevent_arms_lowering: Vec<String>,
    /// DCB field: `breathTriggerOnDrop` (Reference)
    #[serde(default)]
    pub breath_trigger_on_drop: Option<CigGuid>,
}

impl Pooled for ArmsLockConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.arms_lock_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.arms_lock_config }
}

impl<'a> Extract<'a> for ArmsLockConfig {
    const TYPE_NAME: &'static str = "ArmsLockConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_weight: inst.get_f32("minWeight").unwrap_or_default(),
            max_weight: inst.get_f32("maxWeight").unwrap_or_default(),
            base_min_duration: inst.get_f32("baseMinDuration").unwrap_or_default(),
            base_max_duration: inst.get_f32("baseMaxDuration").unwrap_or_default(),
            base_min_shaking: inst.get_f32("baseMinShaking").unwrap_or_default(),
            base_max_shaking: inst.get_f32("baseMaxShaking").unwrap_or_default(),
            heavy_item_drop_delay: inst.get_f32("heavyItemDropDelay").unwrap_or_default(),
            shaking_decay_rate: inst.get_f32("shakingDecayRate").unwrap_or_default(),
            single_use_abilities: inst.get_array("singleUseAbilities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ArmsLockSingleAbility>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ArmsLockSingleAbility>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            abilities_to_lock: inst.get_array("abilitiesToLock")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            abilities_that_prevent_arms_lowering: inst.get_array("abilitiesThatPreventArmsLowering")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            breath_trigger_on_drop: inst.get("breathTriggerOnDrop").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ArenaCommanderLocationObjectContainersParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaCommanderLocationObjectContainersParams {
    /// DCB field: `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// DCB field: `overrideRootOC` (Boolean)
    #[serde(default)]
    pub override_root_oc: bool,
    /// DCB field: `objectContainers` (String (array))
    #[serde(default)]
    pub object_containers: Vec<String>,
}

impl Pooled for ArenaCommanderLocationObjectContainersParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.arena_commander_location_object_containers_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.arena_commander_location_object_containers_params }
}

impl<'a> Extract<'a> for ArenaCommanderLocationObjectContainersParams {
    const TYPE_NAME: &'static str = "ArenaCommanderLocationObjectContainersParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            override_root_oc: inst.get_bool("overrideRootOC").unwrap_or_default(),
            object_containers: inst.get_array("objectContainers")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ArenaCommanderPlanetOverrideParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaCommanderPlanetOverrideParams {
    /// DCB field: `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// DCB field: `fixedRotation` (Single)
    #[serde(default)]
    pub fixed_rotation: f32,
}

impl Pooled for ArenaCommanderPlanetOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.arena_commander_planet_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.arena_commander_planet_override_params }
}

impl<'a> Extract<'a> for ArenaCommanderPlanetOverrideParams {
    const TYPE_NAME: &'static str = "ArenaCommanderPlanetOverrideParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fixed_rotation: inst.get_f32("fixedRotation").unwrap_or_default(),
        }
    }
}

/// DCB type: `ArenaCommanderScenarioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaCommanderScenarioParams {
    /// DCB field: `locationObjectContainersParams` (StrongPointer)
    #[serde(default)]
    pub location_object_containers_params: Option<Handle<ArenaCommanderLocationObjectContainersParams>>,
    /// DCB field: `planetOverrideParams` (StrongPointer)
    #[serde(default)]
    pub planet_override_params: Option<Handle<ArenaCommanderPlanetOverrideParams>>,
}

impl Pooled for ArenaCommanderScenarioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.arena_commander_scenario_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.arena_commander_scenario_params }
}

impl<'a> Extract<'a> for ArenaCommanderScenarioParams {
    const TYPE_NAME: &'static str = "ArenaCommanderScenarioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_object_containers_params: match inst.get("locationObjectContainersParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ArenaCommanderLocationObjectContainersParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ArenaCommanderLocationObjectContainersParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            planet_override_params: match inst.get("planetOverrideParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ArenaCommanderPlanetOverrideParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ArenaCommanderPlanetOverrideParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AreaServices`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaServices {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `lingeringTimeout` (Single)
    #[serde(default)]
    pub lingering_timeout: f32,
    /// DCB field: `service` (StrongPointer (array))
    #[serde(default)]
    pub service: Vec<Handle<BaseService>>,
}

impl Pooled for AreaServices {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.area_services }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.area_services }
}

impl<'a> Extract<'a> for AreaServices {
    const TYPE_NAME: &'static str = "AreaServices";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            lingering_timeout: inst.get_f32("lingeringTimeout").unwrap_or_default(),
            service: inst.get_array("service")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BaseService>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BaseService>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ARModeSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARModeSettings {
    /// DCB field: `playerLabelOffsetX` (Single)
    #[serde(default)]
    pub player_label_offset_x: f32,
    /// DCB field: `playerLabelOffsetY` (Single)
    #[serde(default)]
    pub player_label_offset_y: f32,
    /// DCB field: `playerLabelOffsetZ` (Single)
    #[serde(default)]
    pub player_label_offset_z: f32,
    /// DCB field: `starMapParams` (Reference)
    #[serde(default)]
    pub star_map_params: Option<CigGuid>,
}

impl Pooled for ARModeSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.armode_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.armode_settings }
}

impl<'a> Extract<'a> for ARModeSettings {
    const TYPE_NAME: &'static str = "ARModeSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            player_label_offset_x: inst.get_f32("playerLabelOffsetX").unwrap_or_default(),
            player_label_offset_y: inst.get_f32("playerLabelOffsetY").unwrap_or_default(),
            player_label_offset_z: inst.get_f32("playerLabelOffsetZ").unwrap_or_default(),
            star_map_params: inst.get("starMapParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ArmorMoveViewRestrictions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorMoveViewRestrictions {
    /// DCB field: `ViewRestriction` (Class)
    #[serde(default)]
    pub view_restriction: Option<Handle<MoveViewRestrictionWeighting>>,
    /// DCB field: `MoveRestriction` (Class)
    #[serde(default)]
    pub move_restriction: Option<Handle<MoveViewRestrictionWeighting>>,
}

impl Pooled for ArmorMoveViewRestrictions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ar.armor_move_view_restrictions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ar.armor_move_view_restrictions }
}

impl<'a> Extract<'a> for ArmorMoveViewRestrictions {
    const TYPE_NAME: &'static str = "ArmorMoveViewRestrictions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            view_restriction: match inst.get("ViewRestriction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            move_restriction: match inst.get("MoveRestriction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

