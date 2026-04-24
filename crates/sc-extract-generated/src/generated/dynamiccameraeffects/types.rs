// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `dynamiccameraeffects`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `DynamicCameraEffectsList`
pub struct DynamicCameraEffectsList {
    /// `conversation` (Reference)
    pub conversation: Option<CigGuid>,
    /// `interactionModeFocus` (Reference)
    pub interaction_mode_focus: Option<CigGuid>,
    /// `mobiglas` (Reference)
    pub mobiglas: Option<CigGuid>,
    /// `personalInnerThought` (Reference)
    pub personal_inner_thought: Option<CigGuid>,
    /// `externalInventory` (Reference)
    pub external_inventory: Option<CigGuid>,
    /// `personalInventory` (Reference)
    pub personal_inventory: Option<CigGuid>,
    /// `lootInventory` (Reference)
    pub loot_inventory: Option<CigGuid>,
    /// `onFoot` (Reference)
    pub on_foot: Option<CigGuid>,
    /// `vehicleSeat` (Reference)
    pub vehicle_seat: Option<CigGuid>,
    /// `hacking` (Reference)
    pub hacking: Option<CigGuid>,
    /// `inspectMode` (Reference)
    pub inspect_mode: Option<CigGuid>,
    /// `prototypeMobiGlas` (Reference)
    pub prototype_mobi_glas: Option<CigGuid>,
    /// `simpod` (Reference)
    pub simpod: Option<CigGuid>,
    /// `jumpTravel` (Reference)
    pub jump_travel: Option<CigGuid>,
}

impl Pooled for DynamicCameraEffectsList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.dynamiccameraeffects.dynamic_camera_effects_list
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.dynamiccameraeffects.dynamic_camera_effects_list
    }
}

impl<'a> Extract<'a> for DynamicCameraEffectsList {
    const TYPE_NAME: &'static str = "DynamicCameraEffectsList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            conversation: inst
                .get("conversation")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            interaction_mode_focus: inst
                .get("interactionModeFocus")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            mobiglas: inst
                .get("mobiglas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            personal_inner_thought: inst
                .get("personalInnerThought")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            external_inventory: inst
                .get("externalInventory")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            personal_inventory: inst
                .get("personalInventory")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            loot_inventory: inst
                .get("lootInventory")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            on_foot: inst
                .get("onFoot")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            vehicle_seat: inst
                .get("vehicleSeat")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            hacking: inst
                .get("hacking")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            inspect_mode: inst
                .get("inspectMode")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            prototype_mobi_glas: inst
                .get("prototypeMobiGlas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            simpod: inst
                .get("simpod")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            jump_travel: inst
                .get("jumpTravel")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ConstantDOFPosWeights`
pub struct ConstantDOFPosWeights {
    /// `gridDistance` (Int32)
    pub grid_distance: i32,
    /// `weight` (Single)
    pub weight: f32,
}

impl Pooled for ConstantDOFPosWeights {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.dynamiccameraeffects.constant_dofpos_weights
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.dynamiccameraeffects.constant_dofpos_weights
    }
}

impl<'a> Extract<'a> for ConstantDOFPosWeights {
    const TYPE_NAME: &'static str = "ConstantDOFPosWeights";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grid_distance: inst.get_i32("gridDistance").unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFWeights`
pub struct ConstantDOFWeights {
    /// `maxPositionWeight` (Single)
    pub max_position_weight: f32,
    /// `positionWeights` (Class (array))
    pub position_weights: Vec<Handle<ConstantDOFPosWeights>>,
    /// `NPC` (Single)
    pub npc: f32,
    /// `localPlayer` (Single)
    pub local_player: f32,
    /// `entity` (Single)
    pub entity: f32,
    /// `actorLookingAtPlayer` (Single)
    pub actor_looking_at_player: f32,
    /// `door` (Single)
    pub door: f32,
}

impl Pooled for ConstantDOFWeights {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.dynamiccameraeffects.constant_dofweights
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.dynamiccameraeffects.constant_dofweights
    }
}

impl<'a> Extract<'a> for ConstantDOFWeights {
    const TYPE_NAME: &'static str = "ConstantDOFWeights";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_position_weight: inst.get_f32("maxPositionWeight").unwrap_or_default(),
            position_weights: inst
                .get_array("positionWeights")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ConstantDOFPosWeights>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<ConstantDOFPosWeights>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            npc: inst.get_f32("NPC").unwrap_or_default(),
            local_player: inst.get_f32("localPlayer").unwrap_or_default(),
            entity: inst.get_f32("entity").unwrap_or_default(),
            actor_looking_at_player: inst.get_f32("actorLookingAtPlayer").unwrap_or_default(),
            door: inst.get_f32("door").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFGrid`
pub struct ConstantDOFGrid {
    /// `verticalGridSize` (Int32)
    pub vertical_grid_size: i32,
    /// `horizontalGridSize` (Int32)
    pub horizontal_grid_size: i32,
    /// `verticalSpacing` (Single)
    pub vertical_spacing: f32,
    /// `horizontalSpacing` (Single)
    pub horizontal_spacing: f32,
}

impl Pooled for ConstantDOFGrid {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.dynamiccameraeffects.constant_dofgrid
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.dynamiccameraeffects.constant_dofgrid
    }
}

impl<'a> Extract<'a> for ConstantDOFGrid {
    const TYPE_NAME: &'static str = "ConstantDOFGrid";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            vertical_grid_size: inst.get_i32("verticalGridSize").unwrap_or_default(),
            horizontal_grid_size: inst.get_i32("horizontalGridSize").unwrap_or_default(),
            vertical_spacing: inst.get_f32("verticalSpacing").unwrap_or_default(),
            horizontal_spacing: inst.get_f32("horizontalSpacing").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFGlobalData`
pub struct ConstantDOFGlobalData {
    /// `movementThresholdToIgnorePlayer` (Single)
    pub movement_threshold_to_ignore_player: f32,
    /// `distanceToIgnorePlayer` (Single)
    pub distance_to_ignore_player: f32,
    /// `rotationThresholdToDisable` (Single)
    pub rotation_threshold_to_disable: f32,
    /// `pierceability` (Int32)
    pub pierceability: i32,
    /// `nonEntityDistanceScale` (Single)
    pub non_entity_distance_scale: f32,
    /// `LODBiasOnTarget` (Int32)
    pub lodbias_on_target: i32,
    /// `maxRangeScale` (Single)
    pub max_range_scale: f32,
    /// `circleOfConfusion` (Single)
    pub circle_of_confusion: f32,
    /// `gridParams` (Class)
    pub grid_params: Option<Handle<ConstantDOFGrid>>,
    /// `weights` (Class)
    pub weights: Option<Handle<ConstantDOFWeights>>,
}

impl Pooled for ConstantDOFGlobalData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.dynamiccameraeffects.constant_dofglobal_data
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.dynamiccameraeffects.constant_dofglobal_data
    }
}

impl<'a> Extract<'a> for ConstantDOFGlobalData {
    const TYPE_NAME: &'static str = "ConstantDOFGlobalData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movement_threshold_to_ignore_player: inst
                .get_f32("movementThresholdToIgnorePlayer")
                .unwrap_or_default(),
            distance_to_ignore_player: inst.get_f32("distanceToIgnorePlayer").unwrap_or_default(),
            rotation_threshold_to_disable: inst
                .get_f32("rotationThresholdToDisable")
                .unwrap_or_default(),
            pierceability: inst.get_i32("pierceability").unwrap_or_default(),
            non_entity_distance_scale: inst.get_f32("nonEntityDistanceScale").unwrap_or_default(),
            lodbias_on_target: inst.get_i32("LODBiasOnTarget").unwrap_or_default(),
            max_range_scale: inst.get_f32("maxRangeScale").unwrap_or_default(),
            circle_of_confusion: inst.get_f32("circleOfConfusion").unwrap_or_default(),
            grid_params: match inst.get("gridParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ConstantDOFGrid>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            weights: match inst.get("weights") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ConstantDOFWeights>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
