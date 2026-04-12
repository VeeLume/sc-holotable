// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-skeletonconfigs`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `RagdollRecoveryConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagdollRecoveryConfig {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `physRootAdjust` (Class)
    #[serde(default)]
    pub phys_root_adjust: Option<Handle<AngYPR>>,
    /// `defaultRecoveryAnims` (String (array))
    #[serde(default)]
    pub default_recovery_anims: Vec<String>,
}

impl Pooled for RagdollRecoveryConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_skeletonconfigs.ragdoll_recovery_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_skeletonconfigs.ragdoll_recovery_config }
}

impl<'a> Extract<'a> for RagdollRecoveryConfig {
    const TYPE_NAME: &'static str = "RagdollRecoveryConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            phys_root_adjust: match inst.get("physRootAdjust") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AngYPR>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AngYPR>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_recovery_anims: inst.get_array("defaultRecoveryAnims")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocomotionAnimSyncConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocomotionAnimSyncConfig {
    /// `footJoints` (String (array))
    #[serde(default)]
    pub foot_joints: Vec<String>,
    /// `syncMethod` (EnumChoice)
    #[serde(default)]
    pub sync_method: String,
}

impl Pooled for LocomotionAnimSyncConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_skeletonconfigs.locomotion_anim_sync_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_skeletonconfigs.locomotion_anim_sync_config }
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

/// DCB type: `ActorFootJointPairDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorFootJointPairDef {
    /// `leftFootJoint` (String)
    #[serde(default)]
    pub left_foot_joint: String,
    /// `rightFootJoint` (String)
    #[serde(default)]
    pub right_foot_joint: String,
}

impl Pooled for ActorFootJointPairDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_skeletonconfigs.actor_foot_joint_pair_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_skeletonconfigs.actor_foot_joint_pair_def }
}

impl<'a> Extract<'a> for ActorFootJointPairDef {
    const TYPE_NAME: &'static str = "ActorFootJointPairDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            left_foot_joint: inst.get_str("leftFootJoint").map(String::from).unwrap_or_default(),
            right_foot_joint: inst.get_str("rightFootJoint").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorMeleeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMeleeDef {
    /// `headJoint` (String)
    #[serde(default)]
    pub head_joint: String,
}

impl Pooled for ActorMeleeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_skeletonconfigs.actor_melee_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_skeletonconfigs.actor_melee_def }
}

impl<'a> Extract<'a> for ActorMeleeDef {
    const TYPE_NAME: &'static str = "ActorMeleeDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            head_joint: inst.get_str("headJoint").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorSkeletonConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSkeletonConfig {
    /// `locomotionAnimSyncConfig` (Class)
    #[serde(default)]
    pub locomotion_anim_sync_config: Option<Handle<LocomotionAnimSyncConfig>>,
    /// `ragdollRecoveryConfig` (Class)
    #[serde(default)]
    pub ragdoll_recovery_config: Option<Handle<RagdollRecoveryConfig>>,
    /// `estimatedCyclePhaseFootConfig` (Class)
    #[serde(default)]
    pub estimated_cycle_phase_foot_config: Option<Handle<ActorFootJointPairDef>>,
    /// `preciseCyclePhaseFootConfig` (Class)
    #[serde(default)]
    pub precise_cycle_phase_foot_config: Option<Handle<ActorFootJointPairDef>>,
    /// `meleeConfig` (Class)
    #[serde(default)]
    pub melee_config: Option<Handle<ActorMeleeDef>>,
}

impl Pooled for ActorSkeletonConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_skeletonconfigs.actor_skeleton_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_skeletonconfigs.actor_skeleton_config }
}

impl<'a> Extract<'a> for ActorSkeletonConfig {
    const TYPE_NAME: &'static str = "ActorSkeletonConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            locomotion_anim_sync_config: match inst.get("locomotionAnimSyncConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LocomotionAnimSyncConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LocomotionAnimSyncConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ragdoll_recovery_config: match inst.get("ragdollRecoveryConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RagdollRecoveryConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RagdollRecoveryConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            estimated_cycle_phase_foot_config: match inst.get("estimatedCyclePhaseFootConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFootJointPairDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFootJointPairDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precise_cycle_phase_foot_config: match inst.get("preciseCyclePhaseFootConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFootJointPairDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFootJointPairDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            melee_config: match inst.get("meleeConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorMeleeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorMeleeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AngYPR`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AngYPR {
    /// `yaw` (Single)
    #[serde(default)]
    pub yaw: f32,
    /// `pitch` (Single)
    #[serde(default)]
    pub pitch: f32,
    /// `roll` (Single)
    #[serde(default)]
    pub roll: f32,
}

impl Pooled for AngYPR {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_skeletonconfigs.ang_ypr }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_skeletonconfigs.ang_ypr }
}

impl<'a> Extract<'a> for AngYPR {
    const TYPE_NAME: &'static str = "AngYPR";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            yaw: inst.get_f32("yaw").unwrap_or_default(),
            pitch: inst.get_f32("pitch").unwrap_or_default(),
            roll: inst.get_f32("roll").unwrap_or_default(),
        }
    }
}

