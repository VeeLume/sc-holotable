// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-externalforceresponse`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SActorForceReactionProceduralXianLeanPose`
/// Inherits from: `SActorForceReactionProceduralLeanPose`
pub struct SActorForceReactionProceduralXianLeanPose {
    /// `hipOffsetScaleFB` (Single)
    pub hip_offset_scale_fb: f32,
    /// `hipOffsetScaleLR` (Single)
    pub hip_offset_scale_lr: f32,
    /// `footOffsetScale` (Single)
    pub foot_offset_scale: f32,
    /// `firstPersonLeanPitchScale` (Single)
    pub first_person_lean_pitch_scale: f32,
    /// `firstPersonLeanRollScale` (Single)
    pub first_person_lean_roll_scale: f32,
    /// `spineBones` (Class)
    pub spine_bones: Option<Handle<SActorForceReactionLeanBoneDef>>,
}

impl Pooled for SActorForceReactionProceduralXianLeanPose {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_procedural_xian_lean_pose }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_procedural_xian_lean_pose }
}

impl<'a> Extract<'a> for SActorForceReactionProceduralXianLeanPose {
    const TYPE_NAME: &'static str = "SActorForceReactionProceduralXianLeanPose";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hip_offset_scale_fb: inst.get_f32("hipOffsetScaleFB").unwrap_or_default(),
            hip_offset_scale_lr: inst.get_f32("hipOffsetScaleLR").unwrap_or_default(),
            foot_offset_scale: inst.get_f32("footOffsetScale").unwrap_or_default(),
            first_person_lean_pitch_scale: inst.get_f32("firstPersonLeanPitchScale").unwrap_or_default(),
            first_person_lean_roll_scale: inst.get_f32("firstPersonLeanRollScale").unwrap_or_default(),
            spine_bones: match inst.get("spineBones") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionLeanBoneDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionProceduralXianLeanPoseList`
/// Inherits from: `SActorForceReactionProceduralLeanPoseList`
pub struct SActorForceReactionProceduralXianLeanPoseList {
    /// `poseTypes` (Class)
    pub pose_types: Option<Handle<SActorForceReactionProceduralXianLeanPose>>,
}

impl Pooled for SActorForceReactionProceduralXianLeanPoseList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_procedural_xian_lean_pose_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_procedural_xian_lean_pose_list }
}

impl<'a> Extract<'a> for SActorForceReactionProceduralXianLeanPoseList {
    const TYPE_NAME: &'static str = "SActorForceReactionProceduralXianLeanPoseList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pose_types: match inst.get("poseTypes") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionProceduralXianLeanPose>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionsProceduralLeanOverride`
pub struct SActorForceReactionsProceduralLeanOverride {
    /// `name` (String)
    pub name: String,
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `maxLeanForward` (Single)
    pub max_lean_forward: f32,
    /// `maxLeanBackward` (Single)
    pub max_lean_backward: f32,
    /// `maxLeanLeft` (Single)
    pub max_lean_left: f32,
    /// `maxLeanRight` (Single)
    pub max_lean_right: f32,
    /// `moveHips` (Boolean)
    pub move_hips: bool,
    /// `lockHands` (Boolean)
    pub lock_hands: bool,
    /// `pose` (EnumChoice)
    pub pose: EProcLeanPoseType,
}

impl Pooled for SActorForceReactionsProceduralLeanOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reactions_procedural_lean_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reactions_procedural_lean_override }
}

impl<'a> Extract<'a> for SActorForceReactionsProceduralLeanOverride {
    const TYPE_NAME: &'static str = "SActorForceReactionsProceduralLeanOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            max_lean_forward: inst.get_f32("maxLeanForward").unwrap_or_default(),
            max_lean_backward: inst.get_f32("maxLeanBackward").unwrap_or_default(),
            max_lean_left: inst.get_f32("maxLeanLeft").unwrap_or_default(),
            max_lean_right: inst.get_f32("maxLeanRight").unwrap_or_default(),
            move_hips: inst.get_bool("moveHips").unwrap_or_default(),
            lock_hands: inst.get_bool("lockHands").unwrap_or_default(),
            pose: EProcLeanPoseType::from_dcb_str(inst.get_str("pose").unwrap_or("")),
        }
    }
}

/// DCB type: `SActorForceReactionsPresetRecord`
pub struct SActorForceReactionsPresetRecord {
    /// `procLeanOverrides` (Class (array))
    pub proc_lean_overrides: Vec<Handle<SActorForceReactionsProceduralLeanOverride>>,
}

impl Pooled for SActorForceReactionsPresetRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reactions_preset_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reactions_preset_record }
}

impl<'a> Extract<'a> for SActorForceReactionsPresetRecord {
    const TYPE_NAME: &'static str = "SActorForceReactionsPresetRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            proc_lean_overrides: inst.get_array("procLeanOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionsProceduralLeanOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SActorForceReactionsProceduralLeanOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

