// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `foley`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `FoleyDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyDefinition {
    /// `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// `items` (Class (array))
    #[serde(default)]
    pub items: Vec<Handle<FoleyItem>>,
    /// `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
}

impl Pooled for FoleyDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_definition }
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
    /// `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for FoleyBone {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_bone }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_bone }
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
    /// `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for FoleyAxis {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_axis }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_axis }
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
    /// `trigger` (String)
    #[serde(default)]
    pub trigger: String,
    /// `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
    /// `actorSpeedThreshold` (Single)
    #[serde(default)]
    pub actor_speed_threshold: f32,
    /// `playOnRising` (Boolean)
    #[serde(default)]
    pub play_on_rising: bool,
    /// `axis` (Reference)
    #[serde(default)]
    pub axis: Option<CigGuid>,
    /// `applyToClient` (Boolean)
    #[serde(default)]
    pub apply_to_client: bool,
    /// `applyToNLPC` (Boolean)
    #[serde(default)]
    pub apply_to_nlpc: bool,
    /// `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
}

impl Pooled for FoleyOneShot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_one_shot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_one_shot }
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
    /// `trigger` (String)
    #[serde(default)]
    pub trigger: String,
    /// `minImpactSpeed` (Single)
    #[serde(default)]
    pub min_impact_speed: f32,
    /// `retriggerTimeout` (Single)
    #[serde(default)]
    pub retrigger_timeout: f32,
    /// `applyToClient` (Boolean)
    #[serde(default)]
    pub apply_to_client: bool,
    /// `applyToNLPC` (Boolean)
    #[serde(default)]
    pub apply_to_nlpc: bool,
}

impl Pooled for FoleyCollision {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_collision }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_collision }
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
    /// `playTrigger` (String)
    #[serde(default)]
    pub play_trigger: String,
    /// `stopTrigger` (String)
    #[serde(default)]
    pub stop_trigger: String,
    /// `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
    /// `applyToClient` (Boolean)
    #[serde(default)]
    pub apply_to_client: bool,
    /// `applyToNLPC` (Boolean)
    #[serde(default)]
    pub apply_to_nlpc: bool,
    /// `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
}

impl Pooled for FoleyLoop {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_loop }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_loop }
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
    /// `bone` (Reference)
    #[serde(default)]
    pub bone: Option<CigGuid>,
    /// `trackedBoneOverride` (Reference)
    #[serde(default)]
    pub tracked_bone_override: Option<CigGuid>,
    /// `referenceBone` (Reference)
    #[serde(default)]
    pub reference_bone: Option<CigGuid>,
    /// `velocityRTPCName` (String)
    #[serde(default)]
    pub velocity_rtpcname: String,
    /// `velocityRTPCMinimum` (Single)
    #[serde(default)]
    pub velocity_rtpcminimum: f32,
    /// `velocityRTPCMaximum` (Single)
    #[serde(default)]
    pub velocity_rtpcmaximum: f32,
    /// `collisionImpactRTPCName` (String)
    #[serde(default)]
    pub collision_impact_rtpcname: String,
    /// `collisionScuffRTPCName` (String)
    #[serde(default)]
    pub collision_scuff_rtpcname: String,
    /// `equipmentMassRTPCName` (String)
    #[serde(default)]
    pub equipment_mass_rtpcname: String,
    /// `oneShots` (Class (array))
    #[serde(default)]
    pub one_shots: Vec<Handle<FoleyOneShot>>,
    /// `loops` (Class (array))
    #[serde(default)]
    pub loops: Vec<Handle<FoleyLoop>>,
    /// `collisions` (Class (array))
    #[serde(default)]
    pub collisions: Vec<Handle<FoleyCollision>>,
}

impl Pooled for FoleyItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_item }
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

/// DCB type: `AudioFootstepSurfacesDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFootstepSurfacesDefinition {
    /// `audioSurfaces` (Class (array))
    #[serde(default)]
    pub audio_surfaces: Vec<Handle<AudioFootstepSurfaceMapping>>,
}

impl Pooled for AudioFootstepSurfacesDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.audio_footstep_surfaces_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.audio_footstep_surfaces_definition }
}

impl<'a> Extract<'a> for AudioFootstepSurfacesDefinition {
    const TYPE_NAME: &'static str = "AudioFootstepSurfacesDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_surfaces: inst.get_array("audioSurfaces")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioFootstepSurfaceMapping>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioFootstepSurfaceMapping>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioFootstepSurfaceMapping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFootstepSurfaceMapping {
    /// `surfaceType` (String)
    #[serde(default)]
    pub surface_type: String,
    /// `heelLandAudioTrigger` (String)
    #[serde(default)]
    pub heel_land_audio_trigger: String,
    /// `toeLandAudioTrigger` (String)
    #[serde(default)]
    pub toe_land_audio_trigger: String,
    /// `footLiftAudioTrigger` (String)
    #[serde(default)]
    pub foot_lift_audio_trigger: String,
    /// `turnPlayAudioTrigger` (String)
    #[serde(default)]
    pub turn_play_audio_trigger: String,
    /// `turnStopAudioTrigger` (String)
    #[serde(default)]
    pub turn_stop_audio_trigger: String,
    /// `fadeSteps` (Int32)
    #[serde(default)]
    pub fade_steps: i32,
}

impl Pooled for AudioFootstepSurfaceMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.audio_footstep_surface_mapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.audio_footstep_surface_mapping }
}

impl<'a> Extract<'a> for AudioFootstepSurfaceMapping {
    const TYPE_NAME: &'static str = "AudioFootstepSurfaceMapping";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            surface_type: inst.get_str("surfaceType").map(String::from).unwrap_or_default(),
            heel_land_audio_trigger: inst.get_str("heelLandAudioTrigger").map(String::from).unwrap_or_default(),
            toe_land_audio_trigger: inst.get_str("toeLandAudioTrigger").map(String::from).unwrap_or_default(),
            foot_lift_audio_trigger: inst.get_str("footLiftAudioTrigger").map(String::from).unwrap_or_default(),
            turn_play_audio_trigger: inst.get_str("turnPlayAudioTrigger").map(String::from).unwrap_or_default(),
            turn_stop_audio_trigger: inst.get_str("turnStopAudioTrigger").map(String::from).unwrap_or_default(),
            fade_steps: inst.get_i32("fadeSteps").unwrap_or_default(),
        }
    }
}

/// DCB type: `UserRTPC`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRTPC {
    /// `userRTPCName` (String)
    #[serde(default)]
    pub user_rtpcname: String,
    /// `userRTPCValue` (Single)
    #[serde(default)]
    pub user_rtpcvalue: f32,
    /// `userRTPCResetValue` (Single)
    #[serde(default)]
    pub user_rtpcreset_value: f32,
}

impl Pooled for UserRTPC {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.user_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.user_rtpc }
}

impl<'a> Extract<'a> for UserRTPC {
    const TYPE_NAME: &'static str = "UserRTPC";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            user_rtpcname: inst.get_str("userRTPCName").map(String::from).unwrap_or_default(),
            user_rtpcvalue: inst.get_f32("userRTPCValue").unwrap_or_default(),
            user_rtpcreset_value: inst.get_f32("userRTPCResetValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `FoleyFootstepDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoleyFootstepDefinition {
    /// `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// `heelLandAudioTrigger` (String)
    #[serde(default)]
    pub heel_land_audio_trigger: String,
    /// `toeLandAudioTrigger` (String)
    #[serde(default)]
    pub toe_land_audio_trigger: String,
    /// `footLiftAudioTrigger` (String)
    #[serde(default)]
    pub foot_lift_audio_trigger: String,
    /// `velocityRTPCName` (String)
    #[serde(default)]
    pub velocity_rtpcname: String,
    /// `playerViewRotationRTPCName` (String)
    #[serde(default)]
    pub player_view_rotation_rtpcname: String,
    /// `equipmentMassRTPCName` (String)
    #[serde(default)]
    pub equipment_mass_rtpcname: String,
    /// `playerSkillModifierRTPCName` (String)
    #[serde(default)]
    pub player_skill_modifier_rtpcname: String,
    /// `velocityRTPCMinimum` (Single)
    #[serde(default)]
    pub velocity_rtpcminimum: f32,
    /// `velocityRTPCMaximum` (Single)
    #[serde(default)]
    pub velocity_rtpcmaximum: f32,
    /// `footMovementThreshold` (Single)
    #[serde(default)]
    pub foot_movement_threshold: f32,
    /// `footRotationThreshold` (Single)
    #[serde(default)]
    pub foot_rotation_threshold: f32,
    /// `relativeSpeedThreshold` (Single)
    #[serde(default)]
    pub relative_speed_threshold: f32,
    /// `heightThreshold` (Single)
    #[serde(default)]
    pub height_threshold: f32,
    /// `userRTPCs` (Class (array))
    #[serde(default)]
    pub user_rtpcs: Vec<Handle<UserRTPC>>,
    /// `surfaceMap` (Class)
    #[serde(default)]
    pub surface_map: Option<Handle<AudioFootstepSurfacesDefinition>>,
    /// `footBones` (Reference (array))
    #[serde(default)]
    pub foot_bones: Vec<CigGuid>,
}

impl Pooled for FoleyFootstepDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.foley.foley_footstep_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.foley.foley_footstep_definition }
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

