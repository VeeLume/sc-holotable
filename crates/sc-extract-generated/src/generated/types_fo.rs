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

/// DCB type: `FoleyDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyDefinition {
    /// DCB field: `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// DCB field: `items` (Class (array))
    #[serde(default)]
    pub items: Vec<Handle<FoleyItem>>,
    /// DCB field: `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
}

impl Pooled for FoleyDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_definition }
}

impl<'a> Extract<'a> for FoleyDefinition {
    const TYPE_NAME: &'static str = "FoleyDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            priority: inst.get_i32("priority").unwrap_or_default(),
            items: inst.get_array("items")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FoleyItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FoleyItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            user_rtpcs: inst.get_array("userRTPCs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UserRTPC>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UserRTPC>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyBone`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyBone {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for FoleyBone {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_bone }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_bone }
}

impl<'a> Extract<'a> for FoleyBone {
    const TYPE_NAME: &'static str = "FoleyBone";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyAxis`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyAxis {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for FoleyAxis {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_axis }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_axis }
}

impl<'a> Extract<'a> for FoleyAxis {
    const TYPE_NAME: &'static str = "FoleyAxis";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyOneShot`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyOneShot {
    /// DCB field: `trigger` (String)
    #[serde(default)]
    pub trigger: String,
    /// DCB field: `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
    /// DCB field: `actorSpeedThreshold` (Single)
    #[serde(default)]
    pub actor_speed_threshold: f32,
    /// DCB field: `playOnRising` (Boolean)
    #[serde(default)]
    pub play_on_rising: bool,
    /// DCB field: `axis` (Reference)
    #[serde(default)]
    pub axis: Option<CigGuid>,
    /// DCB field: `applyToClient` (Boolean)
    #[serde(default)]
    pub apply_to_client: bool,
    /// DCB field: `applyToNLPC` (Boolean)
    #[serde(default)]
    pub apply_to_nlpc: bool,
    /// DCB field: `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
}

impl Pooled for FoleyOneShot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_one_shot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_one_shot }
}

impl<'a> Extract<'a> for FoleyOneShot {
    const TYPE_NAME: &'static str = "FoleyOneShot";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger: inst.get_str("trigger").map(String::from).unwrap_or_default(),
            threshold: inst.get_f32("threshold").unwrap_or_default(),
            actor_speed_threshold: inst.get_f32("actorSpeedThreshold").unwrap_or_default(),
            play_on_rising: inst.get_bool("playOnRising").unwrap_or_default(),
            axis: inst.get("axis").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            apply_to_client: inst.get_bool("applyToClient").unwrap_or_default(),
            apply_to_nlpc: inst.get_bool("applyToNLPC").unwrap_or_default(),
            user_rtpcs: inst.get_array("userRTPCs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UserRTPC>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UserRTPC>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyCollision`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyCollision {
    /// DCB field: `trigger` (String)
    #[serde(default)]
    pub trigger: String,
    /// DCB field: `minImpactSpeed` (Single)
    #[serde(default)]
    pub min_impact_speed: f32,
    /// DCB field: `retriggerTimeout` (Single)
    #[serde(default)]
    pub retrigger_timeout: f32,
    /// DCB field: `applyToClient` (Boolean)
    #[serde(default)]
    pub apply_to_client: bool,
    /// DCB field: `applyToNLPC` (Boolean)
    #[serde(default)]
    pub apply_to_nlpc: bool,
}

impl Pooled for FoleyCollision {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_collision }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_collision }
}

impl<'a> Extract<'a> for FoleyCollision {
    const TYPE_NAME: &'static str = "FoleyCollision";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger: inst.get_str("trigger").map(String::from).unwrap_or_default(),
            min_impact_speed: inst.get_f32("minImpactSpeed").unwrap_or_default(),
            retrigger_timeout: inst.get_f32("retriggerTimeout").unwrap_or_default(),
            apply_to_client: inst.get_bool("applyToClient").unwrap_or_default(),
            apply_to_nlpc: inst.get_bool("applyToNLPC").unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyLoop`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyLoop {
    /// DCB field: `playTrigger` (String)
    #[serde(default)]
    pub play_trigger: String,
    /// DCB field: `stopTrigger` (String)
    #[serde(default)]
    pub stop_trigger: String,
    /// DCB field: `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
    /// DCB field: `applyToClient` (Boolean)
    #[serde(default)]
    pub apply_to_client: bool,
    /// DCB field: `applyToNLPC` (Boolean)
    #[serde(default)]
    pub apply_to_nlpc: bool,
    /// DCB field: `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
}

impl Pooled for FoleyLoop {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_loop }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_loop }
}

impl<'a> Extract<'a> for FoleyLoop {
    const TYPE_NAME: &'static str = "FoleyLoop";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            play_trigger: inst.get_str("playTrigger").map(String::from).unwrap_or_default(),
            stop_trigger: inst.get_str("stopTrigger").map(String::from).unwrap_or_default(),
            threshold: inst.get_f32("threshold").unwrap_or_default(),
            apply_to_client: inst.get_bool("applyToClient").unwrap_or_default(),
            apply_to_nlpc: inst.get_bool("applyToNLPC").unwrap_or_default(),
            user_rtpcs: inst.get_array("userRTPCs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UserRTPC>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UserRTPC>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyItem {
    /// DCB field: `bone` (Reference)
    #[serde(default)]
    pub bone: Option<CigGuid>,
    /// DCB field: `trackedBoneOverride` (Reference)
    #[serde(default)]
    pub tracked_bone_override: Option<CigGuid>,
    /// DCB field: `referenceBone` (Reference)
    #[serde(default)]
    pub reference_bone: Option<CigGuid>,
    /// DCB field: `velocityRTPCName` (String)
    #[serde(default)]
    pub velocity_rtpcname: String,
    /// DCB field: `velocityRTPCMinimum` (Single)
    #[serde(default)]
    pub velocity_rtpcminimum: f32,
    /// DCB field: `velocityRTPCMaximum` (Single)
    #[serde(default)]
    pub velocity_rtpcmaximum: f32,
    /// DCB field: `collisionImpactRTPCName` (String)
    #[serde(default)]
    pub collision_impact_rtpcname: String,
    /// DCB field: `collisionScuffRTPCName` (String)
    #[serde(default)]
    pub collision_scuff_rtpcname: String,
    /// DCB field: `equipmentMassRTPCName` (String)
    #[serde(default)]
    pub equipment_mass_rtpcname: String,
    /// DCB field: `oneShots` (Class (array))
    #[serde(default)]
    pub one_shots: Vec<Handle<FoleyOneShot>>,
    /// DCB field: `loops` (Class (array))
    #[serde(default)]
    pub loops: Vec<Handle<FoleyLoop>>,
    /// DCB field: `collisions` (Class (array))
    #[serde(default)]
    pub collisions: Vec<Handle<FoleyCollision>>,
}

impl Pooled for FoleyItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_item }
}

impl<'a> Extract<'a> for FoleyItem {
    const TYPE_NAME: &'static str = "FoleyItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bone: inst.get("bone").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            tracked_bone_override: inst.get("trackedBoneOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reference_bone: inst.get("referenceBone").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            velocity_rtpcname: inst.get_str("velocityRTPCName").map(String::from).unwrap_or_default(),
            velocity_rtpcminimum: inst.get_f32("velocityRTPCMinimum").unwrap_or_default(),
            velocity_rtpcmaximum: inst.get_f32("velocityRTPCMaximum").unwrap_or_default(),
            collision_impact_rtpcname: inst.get_str("collisionImpactRTPCName").map(String::from).unwrap_or_default(),
            collision_scuff_rtpcname: inst.get_str("collisionScuffRTPCName").map(String::from).unwrap_or_default(),
            equipment_mass_rtpcname: inst.get_str("equipmentMassRTPCName").map(String::from).unwrap_or_default(),
            one_shots: inst.get_array("oneShots")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FoleyOneShot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FoleyOneShot>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            loops: inst.get_array("loops")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FoleyLoop>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FoleyLoop>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            collisions: inst.get_array("collisions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FoleyCollision>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FoleyCollision>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyFootstepDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyFootstepDefinition {
    /// DCB field: `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// DCB field: `heelLandAudioTrigger` (String)
    #[serde(default)]
    pub heel_land_audio_trigger: String,
    /// DCB field: `toeLandAudioTrigger` (String)
    #[serde(default)]
    pub toe_land_audio_trigger: String,
    /// DCB field: `footLiftAudioTrigger` (String)
    #[serde(default)]
    pub foot_lift_audio_trigger: String,
    /// DCB field: `velocityRTPCName` (String)
    #[serde(default)]
    pub velocity_rtpcname: String,
    /// DCB field: `playerViewRotationRTPCName` (String)
    #[serde(default)]
    pub player_view_rotation_rtpcname: String,
    /// DCB field: `equipmentMassRTPCName` (String)
    #[serde(default)]
    pub equipment_mass_rtpcname: String,
    /// DCB field: `playerSkillModifierRTPCName` (String)
    #[serde(default)]
    pub player_skill_modifier_rtpcname: String,
    /// DCB field: `velocityRTPCMinimum` (Single)
    #[serde(default)]
    pub velocity_rtpcminimum: f32,
    /// DCB field: `velocityRTPCMaximum` (Single)
    #[serde(default)]
    pub velocity_rtpcmaximum: f32,
    /// DCB field: `footMovementThreshold` (Single)
    #[serde(default)]
    pub foot_movement_threshold: f32,
    /// DCB field: `footRotationThreshold` (Single)
    #[serde(default)]
    pub foot_rotation_threshold: f32,
    /// DCB field: `relativeSpeedThreshold` (Single)
    #[serde(default)]
    pub relative_speed_threshold: f32,
    /// DCB field: `heightThreshold` (Single)
    #[serde(default)]
    pub height_threshold: f32,
    /// DCB field: `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
    /// DCB field: `surfaceMap` (Class)
    #[serde(default)]
    pub surface_map: Option<Handle<AudioFootstepSurfacesDefinition>>,
    /// DCB field: `footBones` (Reference (array))
    #[serde(default)]
    pub foot_bones: Vec<CigGuid>,
}

impl Pooled for FoleyFootstepDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.foley_footstep_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.foley_footstep_definition }
}

impl<'a> Extract<'a> for FoleyFootstepDefinition {
    const TYPE_NAME: &'static str = "FoleyFootstepDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            priority: inst.get_i32("priority").unwrap_or_default(),
            heel_land_audio_trigger: inst.get_str("heelLandAudioTrigger").map(String::from).unwrap_or_default(),
            toe_land_audio_trigger: inst.get_str("toeLandAudioTrigger").map(String::from).unwrap_or_default(),
            foot_lift_audio_trigger: inst.get_str("footLiftAudioTrigger").map(String::from).unwrap_or_default(),
            velocity_rtpcname: inst.get_str("velocityRTPCName").map(String::from).unwrap_or_default(),
            player_view_rotation_rtpcname: inst.get_str("playerViewRotationRTPCName").map(String::from).unwrap_or_default(),
            equipment_mass_rtpcname: inst.get_str("equipmentMassRTPCName").map(String::from).unwrap_or_default(),
            player_skill_modifier_rtpcname: inst.get_str("playerSkillModifierRTPCName").map(String::from).unwrap_or_default(),
            velocity_rtpcminimum: inst.get_f32("velocityRTPCMinimum").unwrap_or_default(),
            velocity_rtpcmaximum: inst.get_f32("velocityRTPCMaximum").unwrap_or_default(),
            foot_movement_threshold: inst.get_f32("footMovementThreshold").unwrap_or_default(),
            foot_rotation_threshold: inst.get_f32("footRotationThreshold").unwrap_or_default(),
            relative_speed_threshold: inst.get_f32("relativeSpeedThreshold").unwrap_or_default(),
            height_threshold: inst.get_f32("heightThreshold").unwrap_or_default(),
            user_rtpcs: inst.get_array("userRTPCs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UserRTPC>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UserRTPC>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            surface_map: match inst.get("surfaceMap") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioFootstepSurfacesDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioFootstepSurfacesDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            foot_bones: inst.get_array("footBones")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Formation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formation {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `offsets` (Class (array))
    #[serde(default)]
    pub offsets: Vec<Handle<FormationOffset>>,
    /// DCB field: `playerFormationParams` (Class)
    #[serde(default)]
    pub player_formation_params: Option<Handle<PlayerFormationParams>>,
    /// DCB field: `formationTag` (Reference)
    #[serde(default)]
    pub formation_tag: Option<CigGuid>,
}

impl Pooled for Formation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.formation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.formation }
}

impl<'a> Extract<'a> for Formation {
    const TYPE_NAME: &'static str = "Formation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            offsets: inst.get_array("offsets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FormationOffset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FormationOffset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            player_formation_params: match inst.get("playerFormationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerFormationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerFormationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            formation_tag: inst.get("formationTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `FormationOffset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationOffset {
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `offsetTag` (Reference)
    #[serde(default)]
    pub offset_tag: Option<CigGuid>,
    /// DCB field: `offsetVariation` (Class)
    #[serde(default)]
    pub offset_variation: Option<Handle<Vec3>>,
}

impl Pooled for FormationOffset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.formation_offset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.formation_offset }
}

impl<'a> Extract<'a> for FormationOffset {
    const TYPE_NAME: &'static str = "FormationOffset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_tag: inst.get("offsetTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            offset_variation: match inst.get("offsetVariation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ForceFeedback`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFeedback {
    /// DCB field: `Patterns` (Class (array))
    #[serde(default)]
    pub patterns: Vec<Handle<ForceFeedbackPattern>>,
    /// DCB field: `Envelopes` (Class (array))
    #[serde(default)]
    pub envelopes: Vec<Handle<ForceFeedbackEnvelope>>,
    /// DCB field: `Effects` (Class (array))
    #[serde(default)]
    pub effects: Vec<Handle<ForceFeedbackEffect>>,
}

impl Pooled for ForceFeedback {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.force_feedback }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.force_feedback }
}

impl<'a> Extract<'a> for ForceFeedback {
    const TYPE_NAME: &'static str = "ForceFeedback";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            patterns: inst.get_array("Patterns")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ForceFeedbackPattern>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ForceFeedbackPattern>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            envelopes: inst.get_array("Envelopes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ForceFeedbackEnvelope>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ForceFeedbackEnvelope>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            effects: inst.get_array("Effects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ForceFeedbackEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ForceFeedbackEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ForceFeedbackPattern`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFeedbackPattern {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `samples` (String)
    #[serde(default)]
    pub samples: String,
}

impl Pooled for ForceFeedbackPattern {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.force_feedback_pattern }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.force_feedback_pattern }
}

impl<'a> Extract<'a> for ForceFeedbackPattern {
    const TYPE_NAME: &'static str = "ForceFeedbackPattern";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            samples: inst.get_str("samples").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ForceFeedbackEnvelope`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFeedbackEnvelope {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `samples` (String)
    #[serde(default)]
    pub samples: String,
}

impl Pooled for ForceFeedbackEnvelope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.force_feedback_envelope }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.force_feedback_envelope }
}

impl<'a> Extract<'a> for ForceFeedbackEnvelope {
    const TYPE_NAME: &'static str = "ForceFeedbackEnvelope";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            samples: inst.get_str("samples").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ForceFeedbackEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFeedbackEffect {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `time` (Single)
    #[serde(default)]
    pub time: f32,
    /// DCB field: `MotorAB` (StrongPointer)
    #[serde(default)]
    pub motor_ab: Option<Handle<ForceFeedbackMotor>>,
    /// DCB field: `MotorA` (StrongPointer)
    #[serde(default)]
    pub motor_a: Option<Handle<ForceFeedbackMotor>>,
    /// DCB field: `MotorB` (StrongPointer)
    #[serde(default)]
    pub motor_b: Option<Handle<ForceFeedbackMotor>>,
}

impl Pooled for ForceFeedbackEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.force_feedback_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.force_feedback_effect }
}

impl<'a> Extract<'a> for ForceFeedbackEffect {
    const TYPE_NAME: &'static str = "ForceFeedbackEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            time: inst.get_f32("time").unwrap_or_default(),
            motor_ab: match inst.get("MotorAB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ForceFeedbackMotor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ForceFeedbackMotor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            motor_a: match inst.get("MotorA") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ForceFeedbackMotor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ForceFeedbackMotor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            motor_b: match inst.get("MotorB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ForceFeedbackMotor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ForceFeedbackMotor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ForceFeedbackMotor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFeedbackMotor {
    /// DCB field: `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// DCB field: `pattern` (String)
    #[serde(default)]
    pub pattern: String,
    /// DCB field: `envelope` (String)
    #[serde(default)]
    pub envelope: String,
}

impl Pooled for ForceFeedbackMotor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fo.force_feedback_motor }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fo.force_feedback_motor }
}

impl<'a> Extract<'a> for ForceFeedbackMotor {
    const TYPE_NAME: &'static str = "ForceFeedbackMotor";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            pattern: inst.get_str("pattern").map(String::from).unwrap_or_default(),
            envelope: inst.get_str("envelope").map(String::from).unwrap_or_default(),
        }
    }
}

