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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SActorForceReactionFilterRangeOverrideDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFilterRangeOverrideDef {
    /// `filterByForceType` (EnumChoice)
    #[serde(default)]
    pub filter_by_force_type: String,
    /// `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// `maxImpulse` (Single)
    #[serde(default)]
    pub max_impulse: f32,
}

impl Pooled for SActorForceReactionFilterRangeOverrideDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_filter_range_override_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_filter_range_override_def }
}

impl<'a> Extract<'a> for SActorForceReactionFilterRangeOverrideDef {
    const TYPE_NAME: &'static str = "SActorForceReactionFilterRangeOverrideDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_by_force_type: inst.get_str("filterByForceType").map(String::from).unwrap_or_default(),
            filter_by_hitter_actor: inst.get_str("filterByHitterActor").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            min_impulse: inst.get_f32("minImpulse").unwrap_or_default(),
            max_impulse: inst.get_f32("maxImpulse").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionFilterItemDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFilterItemDef {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// `maxImpulse` (Single)
    #[serde(default)]
    pub max_impulse: f32,
    /// `rangeOverride` (Class (array))
    #[serde(default)]
    pub range_override: Vec<Handle<SActorForceReactionFilterRangeOverrideDef>>,
}

impl Pooled for SActorForceReactionFilterItemDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_filter_item_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_filter_item_def }
}

impl<'a> Extract<'a> for SActorForceReactionFilterItemDef {
    const TYPE_NAME: &'static str = "SActorForceReactionFilterItemDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            min_impulse: inst.get_f32("minImpulse").unwrap_or_default(),
            max_impulse: inst.get_f32("maxImpulse").unwrap_or_default(),
            range_override: inst.get_array("rangeOverride")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionFilterRangeOverrideDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionFilterRangeOverrideDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionLeanFilterItemDef`
/// Inherits from: `SActorForceReactionFilterItemDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionLeanFilterItemDef {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// `maxImpulse` (Single)
    #[serde(default)]
    pub max_impulse: f32,
    /// `rangeOverride` (Class (array))
    #[serde(default)]
    pub range_override: Vec<Handle<SActorForceReactionFilterRangeOverrideDef>>,
    /// `defaultLeanPose` (EnumChoice)
    #[serde(default)]
    pub default_lean_pose: String,
}

impl Pooled for SActorForceReactionLeanFilterItemDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_lean_filter_item_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_lean_filter_item_def }
}

impl<'a> Extract<'a> for SActorForceReactionLeanFilterItemDef {
    const TYPE_NAME: &'static str = "SActorForceReactionLeanFilterItemDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            min_impulse: inst.get_f32("minImpulse").unwrap_or_default(),
            max_impulse: inst.get_f32("maxImpulse").unwrap_or_default(),
            range_override: inst.get_array("rangeOverride")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionFilterRangeOverrideDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionFilterRangeOverrideDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_lean_pose: inst.get_str("defaultLeanPose").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionLimitDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionLimitDef {
    /// `weaponTwitchMaxOffset` (Single)
    #[serde(default)]
    pub weapon_twitch_max_offset: f32,
    /// `aimPunchMaxAngle` (Single)
    #[serde(default)]
    pub aim_punch_max_angle: f32,
    /// `headRecoilMaxAngleHor` (Single)
    #[serde(default)]
    pub head_recoil_max_angle_hor: f32,
    /// `headRecoilMaxAngleVert` (Single)
    #[serde(default)]
    pub head_recoil_max_angle_vert: f32,
    /// `headRecoilMaxAngleRoll` (Single)
    #[serde(default)]
    pub head_recoil_max_angle_roll: f32,
}

impl Pooled for SActorForceReactionLimitDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_limit_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_limit_def }
}

impl<'a> Extract<'a> for SActorForceReactionLimitDef {
    const TYPE_NAME: &'static str = "SActorForceReactionLimitDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            weapon_twitch_max_offset: inst.get_f32("weaponTwitchMaxOffset").unwrap_or_default(),
            aim_punch_max_angle: inst.get_f32("aimPunchMaxAngle").unwrap_or_default(),
            head_recoil_max_angle_hor: inst.get_f32("headRecoilMaxAngleHor").unwrap_or_default(),
            head_recoil_max_angle_vert: inst.get_f32("headRecoilMaxAngleVert").unwrap_or_default(),
            head_recoil_max_angle_roll: inst.get_f32("headRecoilMaxAngleRoll").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionStateConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionStateConfig {
    /// `delayStaggersWhileInState` (Boolean)
    #[serde(default)]
    pub delay_staggers_while_in_state: bool,
    /// `staggerDelayTimeout` (Single)
    #[serde(default)]
    pub stagger_delay_timeout: f32,
}

impl Pooled for SActorForceReactionStateConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_state_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_state_config }
}

impl<'a> Extract<'a> for SActorForceReactionStateConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionStateConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            delay_staggers_while_in_state: inst.get_bool("delayStaggersWhileInState").unwrap_or_default(),
            stagger_delay_timeout: inst.get_f32("staggerDelayTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionFilterDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFilterDef {
    /// `stateConfig` (Class)
    #[serde(default)]
    pub state_config: Option<Handle<SActorForceReactionStateConfig>>,
    /// `twitches` (Class)
    #[serde(default)]
    pub twitches: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `directStaggers` (Class)
    #[serde(default)]
    pub direct_staggers: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `directKnockdowns` (Class)
    #[serde(default)]
    pub direct_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `flinches` (Class)
    #[serde(default)]
    pub flinches: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `indirectStaggers` (Class)
    #[serde(default)]
    pub indirect_staggers: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `indirectKnockdowns` (Class)
    #[serde(default)]
    pub indirect_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `sustainedDeltaFlinches` (Class)
    #[serde(default)]
    pub sustained_delta_flinches: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `sustainedDeltaStaggers` (Class)
    #[serde(default)]
    pub sustained_delta_staggers: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `sustainedDeltaKnockdowns` (Class)
    #[serde(default)]
    pub sustained_delta_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `sustainedStumble` (Class)
    #[serde(default)]
    pub sustained_stumble: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `sustainedKnockdowns` (Class)
    #[serde(default)]
    pub sustained_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// `lean` (Class)
    #[serde(default)]
    pub lean: Option<Handle<SActorForceReactionLeanFilterItemDef>>,
}

impl Pooled for SActorForceReactionFilterDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_filter_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_filter_def }
}

impl<'a> Extract<'a> for SActorForceReactionFilterDef {
    const TYPE_NAME: &'static str = "SActorForceReactionFilterDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_config: match inst.get("stateConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionStateConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionStateConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            twitches: match inst.get("twitches") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            direct_staggers: match inst.get("directStaggers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            direct_knockdowns: match inst.get("directKnockdowns") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flinches: match inst.get("flinches") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            indirect_staggers: match inst.get("indirectStaggers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            indirect_knockdowns: match inst.get("indirectKnockdowns") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_delta_flinches: match inst.get("sustainedDeltaFlinches") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_delta_staggers: match inst.get("sustainedDeltaStaggers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_delta_knockdowns: match inst.get("sustainedDeltaKnockdowns") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_stumble: match inst.get("sustainedStumble") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_knockdowns: match inst.get("sustainedKnockdowns") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            lean: match inst.get("lean") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionLeanFilterItemDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionLeanFilterItemDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionEnvelope`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionEnvelope {
    /// `attackDuration` (Single)
    #[serde(default)]
    pub attack_duration: f32,
    /// `sustainDuration` (Single)
    #[serde(default)]
    pub sustain_duration: f32,
    /// `releaseDuration` (Single)
    #[serde(default)]
    pub release_duration: f32,
}

impl Pooled for SActorForceReactionEnvelope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_envelope }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_envelope }
}

impl<'a> Extract<'a> for SActorForceReactionEnvelope {
    const TYPE_NAME: &'static str = "SActorForceReactionEnvelope";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attack_duration: inst.get_f32("attackDuration").unwrap_or_default(),
            sustain_duration: inst.get_f32("sustainDuration").unwrap_or_default(),
            release_duration: inst.get_f32("releaseDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionCurve`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionCurve {
    /// `curveName` (String)
    #[serde(default)]
    pub curve_name: String,
    /// `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SActorForceReactionCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_curve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_curve }
}

impl<'a> Extract<'a> for SActorForceReactionCurve {
    const TYPE_NAME: &'static str = "SActorForceReactionCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve_name: inst.get_str("curveName").map(String::from).unwrap_or_default(),
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionCurveConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionCurveConfig {
    /// `attackDuration` (Single)
    #[serde(default)]
    pub attack_duration: f32,
    /// `returnDuration` (Single)
    #[serde(default)]
    pub return_duration: f32,
    /// `returnCurve` (WeakPointer)
    #[serde(default)]
    pub return_curve: Option<Handle<SActorForceReactionCurve>>,
    /// `returnYieldDelay` (Single)
    #[serde(default)]
    pub return_yield_delay: f32,
    /// `returnYieldMagnitude` (Single)
    #[serde(default)]
    pub return_yield_magnitude: f32,
}

impl Pooled for SActorForceReactionCurveConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_curve_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_curve_config }
}

impl<'a> Extract<'a> for SActorForceReactionCurveConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionCurveConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attack_duration: inst.get_f32("attackDuration").unwrap_or_default(),
            return_duration: inst.get_f32("returnDuration").unwrap_or_default(),
            return_curve: match inst.get("returnCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            return_yield_delay: inst.get_f32("returnYieldDelay").unwrap_or_default(),
            return_yield_magnitude: inst.get_f32("returnYieldMagnitude").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionEffectDefaults`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionEffectDefaults {
    /// `yieldMagnitudeMinScale` (Single)
    #[serde(default)]
    pub yield_magnitude_min_scale: f32,
    /// `returnYieldDelay` (Single)
    #[serde(default)]
    pub return_yield_delay: f32,
    /// `returnYieldMagnitude` (Single)
    #[serde(default)]
    pub return_yield_magnitude: f32,
}

impl Pooled for SActorForceReactionEffectDefaults {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_effect_defaults }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_effect_defaults }
}

impl<'a> Extract<'a> for SActorForceReactionEffectDefaults {
    const TYPE_NAME: &'static str = "SActorForceReactionEffectDefaults";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            yield_magnitude_min_scale: inst.get_f32("yieldMagnitudeMinScale").unwrap_or_default(),
            return_yield_delay: inst.get_f32("returnYieldDelay").unwrap_or_default(),
            return_yield_magnitude: inst.get_f32("returnYieldMagnitude").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionGlobalEffectConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionGlobalEffectConfig {
    /// `effectCurves` (Class (array))
    #[serde(default)]
    pub effect_curves: Vec<Handle<SActorForceReactionCurve>>,
    /// `aimPunchDefaults` (Class)
    #[serde(default)]
    pub aim_punch_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// `weaponTwitchDefaults` (Class)
    #[serde(default)]
    pub weapon_twitch_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// `headRecoilPlanarDefaults` (Class)
    #[serde(default)]
    pub head_recoil_planar_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// `headRecoilRollDefaults` (Class)
    #[serde(default)]
    pub head_recoil_roll_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// `aimPunchFrontMaxAngle` (Single)
    #[serde(default)]
    pub aim_punch_front_max_angle: f32,
    /// `aimPunchBackMaxAngle` (Single)
    #[serde(default)]
    pub aim_punch_back_max_angle: f32,
    /// `headRecoilFrontMaxAngle` (Single)
    #[serde(default)]
    pub head_recoil_front_max_angle: f32,
    /// `headRecoilBackMaxAngle` (Single)
    #[serde(default)]
    pub head_recoil_back_max_angle: f32,
    /// `headRecoilRollAxis` (Class)
    #[serde(default)]
    pub head_recoil_roll_axis: Option<Handle<Vec3>>,
}

impl Pooled for SActorForceReactionGlobalEffectConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_global_effect_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_global_effect_config }
}

impl<'a> Extract<'a> for SActorForceReactionGlobalEffectConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionGlobalEffectConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_curves: inst.get_array("effectCurves")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            aim_punch_defaults: match inst.get("aimPunchDefaults") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_twitch_defaults: match inst.get("weaponTwitchDefaults") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil_planar_defaults: match inst.get("headRecoilPlanarDefaults") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil_roll_defaults: match inst.get("headRecoilRollDefaults") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionEffectDefaults>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aim_punch_front_max_angle: inst.get_f32("aimPunchFrontMaxAngle").unwrap_or_default(),
            aim_punch_back_max_angle: inst.get_f32("aimPunchBackMaxAngle").unwrap_or_default(),
            head_recoil_front_max_angle: inst.get_f32("headRecoilFrontMaxAngle").unwrap_or_default(),
            head_recoil_back_max_angle: inst.get_f32("headRecoilBackMaxAngle").unwrap_or_default(),
            head_recoil_roll_axis: match inst.get("headRecoilRollAxis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionAimPunchConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionAimPunchConfig {
    /// `adsZoomScaleFactor` (Single)
    #[serde(default)]
    pub ads_zoom_scale_factor: f32,
    /// `aimPunchDirConeAngle` (Single)
    #[serde(default)]
    pub aim_punch_dir_cone_angle: f32,
    /// `horizontalAimPunchRange` (Class)
    #[serde(default)]
    pub horizontal_aim_punch_range: Option<Handle<Range>>,
    /// `verticalAimPunchRange` (Class)
    #[serde(default)]
    pub vertical_aim_punch_range: Option<Handle<Range>>,
    /// `horizontalRandomAimPunchAtMaxImpulse` (Single)
    #[serde(default)]
    pub horizontal_random_aim_punch_at_max_impulse: f32,
    /// `verticalRandomAimPunchAtMaxImpulse` (Single)
    #[serde(default)]
    pub vertical_random_aim_punch_at_max_impulse: f32,
    /// `curveConfig` (Class)
    #[serde(default)]
    pub curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
}

impl Pooled for SActorForceReactionAimPunchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_aim_punch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_aim_punch_config }
}

impl<'a> Extract<'a> for SActorForceReactionAimPunchConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionAimPunchConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ads_zoom_scale_factor: inst.get_f32("adsZoomScaleFactor").unwrap_or_default(),
            aim_punch_dir_cone_angle: inst.get_f32("aimPunchDirConeAngle").unwrap_or_default(),
            horizontal_aim_punch_range: match inst.get("horizontalAimPunchRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vertical_aim_punch_range: match inst.get("verticalAimPunchRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            horizontal_random_aim_punch_at_max_impulse: inst.get_f32("horizontalRandomAimPunchAtMaxImpulse").unwrap_or_default(),
            vertical_random_aim_punch_at_max_impulse: inst.get_f32("verticalRandomAimPunchAtMaxImpulse").unwrap_or_default(),
            curve_config: match inst.get("curveConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionWeaponTwitchConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionWeaponTwitchConfig {
    /// `adsZoomScaleFactor` (Single)
    #[serde(default)]
    pub ads_zoom_scale_factor: f32,
    /// `offsetAtMinImpulse` (Single)
    #[serde(default)]
    pub offset_at_min_impulse: f32,
    /// `offsetAtMaxImpulse` (Single)
    #[serde(default)]
    pub offset_at_max_impulse: f32,
    /// `offsetRandomAtMaxImpulse` (Single)
    #[serde(default)]
    pub offset_random_at_max_impulse: f32,
    /// `curveConfig` (Class)
    #[serde(default)]
    pub curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
}

impl Pooled for SActorForceReactionWeaponTwitchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_weapon_twitch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_weapon_twitch_config }
}

impl<'a> Extract<'a> for SActorForceReactionWeaponTwitchConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionWeaponTwitchConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ads_zoom_scale_factor: inst.get_f32("adsZoomScaleFactor").unwrap_or_default(),
            offset_at_min_impulse: inst.get_f32("offsetAtMinImpulse").unwrap_or_default(),
            offset_at_max_impulse: inst.get_f32("offsetAtMaxImpulse").unwrap_or_default(),
            offset_random_at_max_impulse: inst.get_f32("offsetRandomAtMaxImpulse").unwrap_or_default(),
            curve_config: match inst.get("curveConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionHeadRecoilConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionHeadRecoilConfig {
    /// `planarADSZoomScaleFactor` (Single)
    #[serde(default)]
    pub planar_adszoom_scale_factor: f32,
    /// `rollADSZoomScaleFactor` (Single)
    #[serde(default)]
    pub roll_adszoom_scale_factor: f32,
    /// `planarDirConeAngle` (Single)
    #[serde(default)]
    pub planar_dir_cone_angle: f32,
    /// `horizontalRecoilRange` (Class)
    #[serde(default)]
    pub horizontal_recoil_range: Option<Handle<Range>>,
    /// `horizontalRandomRecoilAtMaxImpulse` (Single)
    #[serde(default)]
    pub horizontal_random_recoil_at_max_impulse: f32,
    /// `verticalRecoilRange` (Class)
    #[serde(default)]
    pub vertical_recoil_range: Option<Handle<Range>>,
    /// `verticalRandomRecoilAtMaxImpulse` (Single)
    #[serde(default)]
    pub vertical_random_recoil_at_max_impulse: f32,
    /// `rollRecoilRange` (Class)
    #[serde(default)]
    pub roll_recoil_range: Option<Handle<Range>>,
    /// `rollRandomRecoilAtMaxImpulse` (Single)
    #[serde(default)]
    pub roll_random_recoil_at_max_impulse: f32,
    /// `rollAxisInfluencePct` (Single)
    #[serde(default)]
    pub roll_axis_influence_pct: f32,
    /// `planarCurveConfig` (Class)
    #[serde(default)]
    pub planar_curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
    /// `rollCurveConfig` (Class)
    #[serde(default)]
    pub roll_curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
}

impl Pooled for SActorForceReactionHeadRecoilConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_head_recoil_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_head_recoil_config }
}

impl<'a> Extract<'a> for SActorForceReactionHeadRecoilConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionHeadRecoilConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            planar_adszoom_scale_factor: inst.get_f32("planarADSZoomScaleFactor").unwrap_or_default(),
            roll_adszoom_scale_factor: inst.get_f32("rollADSZoomScaleFactor").unwrap_or_default(),
            planar_dir_cone_angle: inst.get_f32("planarDirConeAngle").unwrap_or_default(),
            horizontal_recoil_range: match inst.get("horizontalRecoilRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            horizontal_random_recoil_at_max_impulse: inst.get_f32("horizontalRandomRecoilAtMaxImpulse").unwrap_or_default(),
            vertical_recoil_range: match inst.get("verticalRecoilRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vertical_random_recoil_at_max_impulse: inst.get_f32("verticalRandomRecoilAtMaxImpulse").unwrap_or_default(),
            roll_recoil_range: match inst.get("rollRecoilRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_random_recoil_at_max_impulse: inst.get_f32("rollRandomRecoilAtMaxImpulse").unwrap_or_default(),
            roll_axis_influence_pct: inst.get_f32("rollAxisInfluencePct").unwrap_or_default(),
            planar_curve_config: match inst.get("planarCurveConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_curve_config: match inst.get("rollCurveConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionCurveConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionFOVScaleConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFOVScaleConfig {
    /// `adsZoomScaleFactor` (Single)
    #[serde(default)]
    pub ads_zoom_scale_factor: f32,
    /// `FOVScaleAtMinImpulse` (Single)
    #[serde(default)]
    pub fovscale_at_min_impulse: f32,
    /// `FOVScaleAtMaxImpulse` (Single)
    #[serde(default)]
    pub fovscale_at_max_impulse: f32,
    /// `envelope` (Class)
    #[serde(default)]
    pub envelope: Option<Handle<SActorForceReactionEnvelope>>,
}

impl Pooled for SActorForceReactionFOVScaleConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_fovscale_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_fovscale_config }
}

impl<'a> Extract<'a> for SActorForceReactionFOVScaleConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionFOVScaleConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ads_zoom_scale_factor: inst.get_f32("adsZoomScaleFactor").unwrap_or_default(),
            fovscale_at_min_impulse: inst.get_f32("FOVScaleAtMinImpulse").unwrap_or_default(),
            fovscale_at_max_impulse: inst.get_f32("FOVScaleAtMaxImpulse").unwrap_or_default(),
            envelope: match inst.get("envelope") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionEnvelope>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionEnvelope>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionAnimationTwitchConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionAnimationTwitchConfig {
    /// `blendspaceMin` (Single)
    #[serde(default)]
    pub blendspace_min: f32,
}

impl Pooled for SActorForceReactionAnimationTwitchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_animation_twitch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_animation_twitch_config }
}

impl<'a> Extract<'a> for SActorForceReactionAnimationTwitchConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionAnimationTwitchConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blendspace_min: inst.get_f32("blendspaceMin").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionAnimationFlinchConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionAnimationFlinchConfig {
    /// `blendspaceMin` (Single)
    #[serde(default)]
    pub blendspace_min: f32,
}

impl Pooled for SActorForceReactionAnimationFlinchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_animation_flinch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_animation_flinch_config }
}

impl<'a> Extract<'a> for SActorForceReactionAnimationFlinchConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionAnimationFlinchConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blendspace_min: inst.get_f32("blendspaceMin").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionAnimationStaggerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionAnimationStaggerConfig {
    /// `blendspaceMin` (Single)
    #[serde(default)]
    pub blendspace_min: f32,
    /// `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
}

impl Pooled for SActorForceReactionAnimationStaggerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_animation_stagger_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_animation_stagger_config }
}

impl<'a> Extract<'a> for SActorForceReactionAnimationStaggerConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionAnimationStaggerConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blendspace_min: inst.get_f32("blendspaceMin").unwrap_or_default(),
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionStaggerTagConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionStaggerTagConfig {
    /// `mannequinTag` (String)
    #[serde(default)]
    pub mannequin_tag: String,
    /// `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
}

impl Pooled for SActorForceReactionStaggerTagConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_stagger_tag_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_stagger_tag_config }
}

impl<'a> Extract<'a> for SActorForceReactionStaggerTagConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionStaggerTagConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mannequin_tag: inst.get_str("mannequinTag").map(String::from).unwrap_or_default(),
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionGlobalStaggerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionGlobalStaggerConfig {
    /// `movementReferenceVelocity` (Class)
    #[serde(default)]
    pub movement_reference_velocity: Option<Handle<Range>>,
    /// `staggerDistance` (Class)
    #[serde(default)]
    pub stagger_distance: Option<Handle<Range>>,
    /// `distanceTags` (Class (array))
    #[serde(default)]
    pub distance_tags: Vec<Handle<SActorForceReactionStaggerTagConfig>>,
    /// `viewPitchRangeDeg` (Single)
    #[serde(default)]
    pub view_pitch_range_deg: f32,
    /// `viewYawRangeDeg` (Single)
    #[serde(default)]
    pub view_yaw_range_deg: f32,
}

impl Pooled for SActorForceReactionGlobalStaggerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_global_stagger_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_global_stagger_config }
}

impl<'a> Extract<'a> for SActorForceReactionGlobalStaggerConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionGlobalStaggerConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movement_reference_velocity: match inst.get("movementReferenceVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stagger_distance: match inst.get("staggerDistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_tags: inst.get_array("distanceTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionStaggerTagConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionStaggerTagConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            view_pitch_range_deg: inst.get_f32("viewPitchRangeDeg").unwrap_or_default(),
            view_yaw_range_deg: inst.get_f32("viewYawRangeDeg").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionImpulseAccumulationConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionImpulseAccumulationConfig {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// `accumulationFraction` (Single)
    #[serde(default)]
    pub accumulation_fraction: f32,
    /// `cooldown` (Single)
    #[serde(default)]
    pub cooldown: f32,
}

impl Pooled for SActorForceReactionImpulseAccumulationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_impulse_accumulation_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_impulse_accumulation_config }
}

impl<'a> Extract<'a> for SActorForceReactionImpulseAccumulationConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionImpulseAccumulationConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            decay: inst.get_f32("decay").unwrap_or_default(),
            accumulation_fraction: inst.get_f32("accumulationFraction").unwrap_or_default(),
            cooldown: inst.get_f32("cooldown").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionFlightDurationConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFlightDurationConfig {
    /// `forcePercent` (Single)
    #[serde(default)]
    pub force_percent: f32,
    /// `flightDuration` (Single)
    #[serde(default)]
    pub flight_duration: f32,
}

impl Pooled for SActorForceReactionFlightDurationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_flight_duration_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_flight_duration_config }
}

impl<'a> Extract<'a> for SActorForceReactionFlightDurationConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionFlightDurationConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            force_percent: inst.get_f32("forcePercent").unwrap_or_default(),
            flight_duration: inst.get_f32("flightDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionMovementLaunchConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionMovementLaunchConfig {
    /// `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// `durations` (Class (array))
    #[serde(default)]
    pub durations: Vec<Handle<SActorForceReactionFlightDurationConfig>>,
}

impl Pooled for SActorForceReactionMovementLaunchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_movement_launch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_movement_launch_config }
}

impl<'a> Extract<'a> for SActorForceReactionMovementLaunchConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionMovementLaunchConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            durations: inst.get_array("durations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionFlightDurationConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionFlightDurationConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionBlockADSConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionBlockADSConfig {
    /// `blockADS` (Boolean)
    #[serde(default)]
    pub block_ads: bool,
    /// `triggerADSBlockDuration` (Single)
    #[serde(default)]
    pub trigger_adsblock_duration: f32,
    /// `heldADSBlockDuration` (Single)
    #[serde(default)]
    pub held_adsblock_duration: f32,
}

impl Pooled for SActorForceReactionBlockADSConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_block_adsconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_block_adsconfig }
}

impl<'a> Extract<'a> for SActorForceReactionBlockADSConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionBlockADSConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            block_ads: inst.get_bool("blockADS").unwrap_or_default(),
            trigger_adsblock_duration: inst.get_f32("triggerADSBlockDuration").unwrap_or_default(),
            held_adsblock_duration: inst.get_f32("heldADSBlockDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionBlockConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionBlockConfig {
    /// `directTwitchDisableDuration` (Single)
    #[serde(default)]
    pub direct_twitch_disable_duration: f32,
    /// `directStaggerDisableDuration` (Single)
    #[serde(default)]
    pub direct_stagger_disable_duration: f32,
    /// `directKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub direct_knockdown_disable_duration: f32,
    /// `indirectFlinchDisableDuration` (Single)
    #[serde(default)]
    pub indirect_flinch_disable_duration: f32,
    /// `indirectStaggerDisableDuration` (Single)
    #[serde(default)]
    pub indirect_stagger_disable_duration: f32,
    /// `indirectKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub indirect_knockdown_disable_duration: f32,
    /// `sustainedDeltaFlinchDisableDuration` (Single)
    #[serde(default)]
    pub sustained_delta_flinch_disable_duration: f32,
    /// `sustainedDeltaStaggerDisableDuration` (Single)
    #[serde(default)]
    pub sustained_delta_stagger_disable_duration: f32,
    /// `sustainedDeltaKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub sustained_delta_knockdown_disable_duration: f32,
    /// `sustainedKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub sustained_knockdown_disable_duration: f32,
    /// `useEffortSetWhileDisablingReactions` (Boolean)
    #[serde(default)]
    pub use_effort_set_while_disabling_reactions: bool,
}

impl Pooled for SActorForceReactionBlockConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_block_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_block_config }
}

impl<'a> Extract<'a> for SActorForceReactionBlockConfig {
    const TYPE_NAME: &'static str = "SActorForceReactionBlockConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            direct_twitch_disable_duration: inst.get_f32("directTwitchDisableDuration").unwrap_or_default(),
            direct_stagger_disable_duration: inst.get_f32("directStaggerDisableDuration").unwrap_or_default(),
            direct_knockdown_disable_duration: inst.get_f32("directKnockdownDisableDuration").unwrap_or_default(),
            indirect_flinch_disable_duration: inst.get_f32("indirectFlinchDisableDuration").unwrap_or_default(),
            indirect_stagger_disable_duration: inst.get_f32("indirectStaggerDisableDuration").unwrap_or_default(),
            indirect_knockdown_disable_duration: inst.get_f32("indirectKnockdownDisableDuration").unwrap_or_default(),
            sustained_delta_flinch_disable_duration: inst.get_f32("sustainedDeltaFlinchDisableDuration").unwrap_or_default(),
            sustained_delta_stagger_disable_duration: inst.get_f32("sustainedDeltaStaggerDisableDuration").unwrap_or_default(),
            sustained_delta_knockdown_disable_duration: inst.get_f32("sustainedDeltaKnockdownDisableDuration").unwrap_or_default(),
            sustained_knockdown_disable_duration: inst.get_f32("sustainedKnockdownDisableDuration").unwrap_or_default(),
            use_effort_set_while_disabling_reactions: inst.get_bool("useEffortSetWhileDisablingReactions").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionTwitchRangeDef`
/// Inherits from: `SActorForceReactionEffectRangeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionTwitchRangeDef {
    /// `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// `animationTwitch` (Class)
    #[serde(default)]
    pub animation_twitch: Option<Handle<SActorForceReactionAnimationTwitchConfig>>,
    /// `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionTwitchRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_twitch_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_twitch_range_def }
}

impl<'a> Extract<'a> for SActorForceReactionTwitchRangeDef {
    const TYPE_NAME: &'static str = "SActorForceReactionTwitchRangeDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_range_min: inst.get_f32("effectRangeMin").unwrap_or_default(),
            effect_range_max: inst.get_f32("effectRangeMax").unwrap_or_default(),
            aim_punch: match inst.get("aimPunch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_twitch: match inst.get("weaponTwitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil: match inst.get("headRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fovscale: match inst.get("FOVScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            animation_twitch: match inst.get("animationTwitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionAnimationTwitchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionAnimationTwitchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            block_ads: match inst.get("blockADS") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionTwitchConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionTwitchConfigDef {
    /// `filterByForceType` (EnumChoice)
    #[serde(default)]
    pub filter_by_force_type: String,
    /// `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// `twitchRanges` (Class (array))
    #[serde(default)]
    pub twitch_ranges: Vec<Handle<SActorForceReactionTwitchRangeDef>>,
}

impl Pooled for SActorForceReactionTwitchConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_twitch_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_twitch_config_def }
}

impl<'a> Extract<'a> for SActorForceReactionTwitchConfigDef {
    const TYPE_NAME: &'static str = "SActorForceReactionTwitchConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter_by_force_type: inst.get_str("filterByForceType").map(String::from).unwrap_or_default(),
            filter_by_hitter_actor: inst.get_str("filterByHitterActor").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            twitch_ranges: inst.get_array("twitchRanges")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionTwitchRangeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionTwitchRangeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionFlinchRangeDef`
/// Inherits from: `SActorForceReactionEffectRangeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFlinchRangeDef {
    /// `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// `animationFlinch` (Class)
    #[serde(default)]
    pub animation_flinch: Option<Handle<SActorForceReactionAnimationFlinchConfig>>,
    /// `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionFlinchRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_flinch_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_flinch_range_def }
}

impl<'a> Extract<'a> for SActorForceReactionFlinchRangeDef {
    const TYPE_NAME: &'static str = "SActorForceReactionFlinchRangeDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_range_min: inst.get_f32("effectRangeMin").unwrap_or_default(),
            effect_range_max: inst.get_f32("effectRangeMax").unwrap_or_default(),
            aim_punch: match inst.get("aimPunch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_twitch: match inst.get("weaponTwitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil: match inst.get("headRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fovscale: match inst.get("FOVScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            animation_flinch: match inst.get("animationFlinch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionAnimationFlinchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionAnimationFlinchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            block_ads: match inst.get("blockADS") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionFlinchConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFlinchConfigDef {
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// `flinchRanges` (Class (array))
    #[serde(default)]
    pub flinch_ranges: Vec<Handle<SActorForceReactionFlinchRangeDef>>,
}

impl Pooled for SActorForceReactionFlinchConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_flinch_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_flinch_config_def }
}

impl<'a> Extract<'a> for SActorForceReactionFlinchConfigDef {
    const TYPE_NAME: &'static str = "SActorForceReactionFlinchConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_hitter_actor: inst.get_str("filterByHitterActor").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            flinch_ranges: inst.get_array("flinchRanges")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionFlinchRangeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionFlinchRangeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionStaggerRangeDef`
/// Inherits from: `SActorForceReactionEffectRangeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionStaggerRangeDef {
    /// `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// `animationStagger` (Class)
    #[serde(default)]
    pub animation_stagger: Option<Handle<SActorForceReactionAnimationStaggerConfig>>,
    /// `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionStaggerRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_stagger_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_stagger_range_def }
}

impl<'a> Extract<'a> for SActorForceReactionStaggerRangeDef {
    const TYPE_NAME: &'static str = "SActorForceReactionStaggerRangeDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_range_min: inst.get_f32("effectRangeMin").unwrap_or_default(),
            effect_range_max: inst.get_f32("effectRangeMax").unwrap_or_default(),
            aim_punch: match inst.get("aimPunch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_twitch: match inst.get("weaponTwitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil: match inst.get("headRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fovscale: match inst.get("FOVScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            animation_stagger: match inst.get("animationStagger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionAnimationStaggerConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionAnimationStaggerConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            block_ads: match inst.get("blockADS") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionFilteredStaggerRangeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFilteredStaggerRangeDef {
    /// `filterByForceType` (EnumChoice)
    #[serde(default)]
    pub filter_by_force_type: String,
    /// `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// `staggerRanges` (Class (array))
    #[serde(default)]
    pub stagger_ranges: Vec<Handle<SActorForceReactionStaggerRangeDef>>,
}

impl Pooled for SActorForceReactionFilteredStaggerRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_filtered_stagger_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_filtered_stagger_range_def }
}

impl<'a> Extract<'a> for SActorForceReactionFilteredStaggerRangeDef {
    const TYPE_NAME: &'static str = "SActorForceReactionFilteredStaggerRangeDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter_by_force_type: inst.get_str("filterByForceType").map(String::from).unwrap_or_default(),
            filter_by_hitter_actor: inst.get_str("filterByHitterActor").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            stagger_ranges: inst.get_array("staggerRanges")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionStaggerRangeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionStaggerRangeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionFilteredStaggerConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFilteredStaggerConfigDef {
    /// `reactionBlock` (Class)
    #[serde(default)]
    pub reaction_block: Option<Handle<SActorForceReactionBlockConfig>>,
    /// `staggerFilters` (Class (array))
    #[serde(default)]
    pub stagger_filters: Vec<Handle<SActorForceReactionFilteredStaggerRangeDef>>,
}

impl Pooled for SActorForceReactionFilteredStaggerConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_filtered_stagger_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_filtered_stagger_config_def }
}

impl<'a> Extract<'a> for SActorForceReactionFilteredStaggerConfigDef {
    const TYPE_NAME: &'static str = "SActorForceReactionFilteredStaggerConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reaction_block: match inst.get("reactionBlock") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionBlockConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionBlockConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stagger_filters: inst.get_array("staggerFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionFilteredStaggerRangeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionFilteredStaggerRangeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionUnfilteredStaggerConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionUnfilteredStaggerConfigDef {
    /// `reactionBlock` (Class)
    #[serde(default)]
    pub reaction_block: Option<Handle<SActorForceReactionBlockConfig>>,
    /// `staggerRanges` (Class (array))
    #[serde(default)]
    pub stagger_ranges: Vec<Handle<SActorForceReactionStaggerRangeDef>>,
}

impl Pooled for SActorForceReactionUnfilteredStaggerConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_unfiltered_stagger_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_unfiltered_stagger_config_def }
}

impl<'a> Extract<'a> for SActorForceReactionUnfilteredStaggerConfigDef {
    const TYPE_NAME: &'static str = "SActorForceReactionUnfilteredStaggerConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reaction_block: match inst.get("reactionBlock") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionBlockConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionBlockConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stagger_ranges: inst.get_array("staggerRanges")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionStaggerRangeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionStaggerRangeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionKnockdownRangeDef`
/// Inherits from: `SActorForceReactionEffectRangeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionKnockdownRangeDef {
    /// `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// `launch` (Class)
    #[serde(default)]
    pub launch: Option<Handle<SActorForceReactionMovementLaunchConfig>>,
    /// `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionKnockdownRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_knockdown_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_knockdown_range_def }
}

impl<'a> Extract<'a> for SActorForceReactionKnockdownRangeDef {
    const TYPE_NAME: &'static str = "SActorForceReactionKnockdownRangeDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_range_min: inst.get_f32("effectRangeMin").unwrap_or_default(),
            effect_range_max: inst.get_f32("effectRangeMax").unwrap_or_default(),
            aim_punch: match inst.get("aimPunch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionAimPunchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_twitch: match inst.get("weaponTwitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionWeaponTwitchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil: match inst.get("headRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionHeadRecoilConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fovscale: match inst.get("FOVScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFOVScaleConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            launch: match inst.get("launch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionMovementLaunchConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionMovementLaunchConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            block_ads: match inst.get("blockADS") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionBlockADSConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionKnockdownConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionKnockdownConfigDef {
    /// `reactionBlock` (Class)
    #[serde(default)]
    pub reaction_block: Option<Handle<SActorForceReactionBlockConfig>>,
    /// `knockdownRanges` (Class (array))
    #[serde(default)]
    pub knockdown_ranges: Vec<Handle<SActorForceReactionKnockdownRangeDef>>,
}

impl Pooled for SActorForceReactionKnockdownConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_knockdown_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_knockdown_config_def }
}

impl<'a> Extract<'a> for SActorForceReactionKnockdownConfigDef {
    const TYPE_NAME: &'static str = "SActorForceReactionKnockdownConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reaction_block: match inst.get("reactionBlock") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionBlockConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionBlockConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            knockdown_ranges: inst.get_array("knockdownRanges")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionKnockdownRangeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionKnockdownRangeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionLeanAngleLimitsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionLeanAngleLimitsDef {
    /// `forward` (Single)
    #[serde(default)]
    pub forward: f32,
    /// `backward` (Single)
    #[serde(default)]
    pub backward: f32,
    /// `left` (Single)
    #[serde(default)]
    pub left: f32,
    /// `right` (Single)
    #[serde(default)]
    pub right: f32,
}

impl Pooled for SActorForceReactionLeanAngleLimitsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_lean_angle_limits_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_lean_angle_limits_def }
}

impl<'a> Extract<'a> for SActorForceReactionLeanAngleLimitsDef {
    const TYPE_NAME: &'static str = "SActorForceReactionLeanAngleLimitsDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            forward: inst.get_f32("forward").unwrap_or_default(),
            backward: inst.get_f32("backward").unwrap_or_default(),
            left: inst.get_f32("left").unwrap_or_default(),
            right: inst.get_f32("right").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionLeanFilterDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionLeanFilterDef {
    /// `filter` (Class)
    #[serde(default)]
    pub filter: Option<Handle<ActorMotionStateFilter>>,
    /// `leanAngleLimits` (Class)
    #[serde(default)]
    pub lean_angle_limits: Option<Handle<SActorForceReactionLeanAngleLimitsDef>>,
    /// `hipVOffset` (Single)
    #[serde(default)]
    pub hip_voffset: f32,
}

impl Pooled for SActorForceReactionLeanFilterDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_lean_filter_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_lean_filter_def }
}

impl<'a> Extract<'a> for SActorForceReactionLeanFilterDef {
    const TYPE_NAME: &'static str = "SActorForceReactionLeanFilterDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter: match inst.get("filter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorMotionStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorMotionStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            lean_angle_limits: match inst.get("leanAngleLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionLeanAngleLimitsDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionLeanAngleLimitsDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hip_voffset: inst.get_f32("hipVOffset").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionProceduralLeanPoseList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionProceduralLeanPoseList {
}

impl Pooled for SActorForceReactionProceduralLeanPoseList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_procedural_lean_pose_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_procedural_lean_pose_list }
}

impl<'a> Extract<'a> for SActorForceReactionProceduralLeanPoseList {
    const TYPE_NAME: &'static str = "SActorForceReactionProceduralLeanPoseList";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SActorForceReactionLeanConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionLeanConfigDef {
    /// `forceSmoothDuration` (Single)
    #[serde(default)]
    pub force_smooth_duration: f32,
    /// `minLeanForceForEffort` (Single)
    #[serde(default)]
    pub min_lean_force_for_effort: f32,
    /// `leanFilters` (Class (array))
    #[serde(default)]
    pub lean_filters: Vec<Handle<SActorForceReactionLeanFilterDef>>,
    /// `procLeanPoseDef` (StrongPointer)
    #[serde(default)]
    pub proc_lean_pose_def: Option<Handle<SActorForceReactionProceduralLeanPoseList>>,
}

impl Pooled for SActorForceReactionLeanConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_lean_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_lean_config_def }
}

impl<'a> Extract<'a> for SActorForceReactionLeanConfigDef {
    const TYPE_NAME: &'static str = "SActorForceReactionLeanConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            force_smooth_duration: inst.get_f32("forceSmoothDuration").unwrap_or_default(),
            min_lean_force_for_effort: inst.get_f32("minLeanForceForEffort").unwrap_or_default(),
            lean_filters: inst.get_array("leanFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionLeanFilterDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionLeanFilterDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            proc_lean_pose_def: match inst.get("procLeanPoseDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionProceduralLeanPoseList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionProceduralLeanPoseList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionStumbleConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionStumbleConfigDef {
    /// `minAdditionalSpeed` (Single)
    #[serde(default)]
    pub min_additional_speed: f32,
    /// `maxAdditionalSpeed` (Single)
    #[serde(default)]
    pub max_additional_speed: f32,
}

impl Pooled for SActorForceReactionStumbleConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_stumble_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_stumble_config_def }
}

impl<'a> Extract<'a> for SActorForceReactionStumbleConfigDef {
    const TYPE_NAME: &'static str = "SActorForceReactionStumbleConfigDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_additional_speed: inst.get_f32("minAdditionalSpeed").unwrap_or_default(),
            max_additional_speed: inst.get_f32("maxAdditionalSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionSustainedImpulseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionSustainedImpulseDef {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `sustainedToImpulseRange` (Class)
    #[serde(default)]
    pub sustained_to_impulse_range: Option<Handle<Range>>,
    /// `forceSmoothingDuration` (Single)
    #[serde(default)]
    pub force_smoothing_duration: f32,
    /// `predictionUncertaintyPercent` (Single)
    #[serde(default)]
    pub prediction_uncertainty_percent: f32,
    /// `minForceToTriggerImpulse` (Single)
    #[serde(default)]
    pub min_force_to_trigger_impulse: f32,
}

impl Pooled for SActorForceReactionSustainedImpulseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_sustained_impulse_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_sustained_impulse_def }
}

impl<'a> Extract<'a> for SActorForceReactionSustainedImpulseDef {
    const TYPE_NAME: &'static str = "SActorForceReactionSustainedImpulseDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            sustained_to_impulse_range: match inst.get("sustainedToImpulseRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            force_smoothing_duration: inst.get_f32("forceSmoothingDuration").unwrap_or_default(),
            prediction_uncertainty_percent: inst.get_f32("predictionUncertaintyPercent").unwrap_or_default(),
            min_force_to_trigger_impulse: inst.get_f32("minForceToTriggerImpulse").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionExternalImpulseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionExternalImpulseDef {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `externalCollisionToImpulseRange` (Class)
    #[serde(default)]
    pub external_collision_to_impulse_range: Option<Handle<Range>>,
}

impl Pooled for SActorForceReactionExternalImpulseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_external_impulse_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_external_impulse_def }
}

impl<'a> Extract<'a> for SActorForceReactionExternalImpulseDef {
    const TYPE_NAME: &'static str = "SActorForceReactionExternalImpulseDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            external_collision_to_impulse_range: match inst.get("externalCollisionToImpulseRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionActorBumpDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionActorBumpDef {
    /// `allowPlayerBump` (Boolean)
    #[serde(default)]
    pub allow_player_bump: bool,
    /// `allowPlayerBumpTwitch` (Boolean)
    #[serde(default)]
    pub allow_player_bump_twitch: bool,
    /// `allowPlayerBumpStagger` (Boolean)
    #[serde(default)]
    pub allow_player_bump_stagger: bool,
    /// `allowPlayerBumpKnockdown` (Boolean)
    #[serde(default)]
    pub allow_player_bump_knockdown: bool,
    /// `allowNpcBump` (Boolean)
    #[serde(default)]
    pub allow_npc_bump: bool,
    /// `allowNpcBumpTwitch` (Boolean)
    #[serde(default)]
    pub allow_npc_bump_twitch: bool,
    /// `allowNpcBumpStagger` (Boolean)
    #[serde(default)]
    pub allow_npc_bump_stagger: bool,
    /// `allowNpcBumpKnockdown` (Boolean)
    #[serde(default)]
    pub allow_npc_bump_knockdown: bool,
    /// `playerBumpImpulseScale` (Single)
    #[serde(default)]
    pub player_bump_impulse_scale: f32,
    /// `npcBumpImpulseScale` (Single)
    #[serde(default)]
    pub npc_bump_impulse_scale: f32,
}

impl Pooled for SActorForceReactionActorBumpDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_actor_bump_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_actor_bump_def }
}

impl<'a> Extract<'a> for SActorForceReactionActorBumpDef {
    const TYPE_NAME: &'static str = "SActorForceReactionActorBumpDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow_player_bump: inst.get_bool("allowPlayerBump").unwrap_or_default(),
            allow_player_bump_twitch: inst.get_bool("allowPlayerBumpTwitch").unwrap_or_default(),
            allow_player_bump_stagger: inst.get_bool("allowPlayerBumpStagger").unwrap_or_default(),
            allow_player_bump_knockdown: inst.get_bool("allowPlayerBumpKnockdown").unwrap_or_default(),
            allow_npc_bump: inst.get_bool("allowNpcBump").unwrap_or_default(),
            allow_npc_bump_twitch: inst.get_bool("allowNpcBumpTwitch").unwrap_or_default(),
            allow_npc_bump_stagger: inst.get_bool("allowNpcBumpStagger").unwrap_or_default(),
            allow_npc_bump_knockdown: inst.get_bool("allowNpcBumpKnockdown").unwrap_or_default(),
            player_bump_impulse_scale: inst.get_f32("playerBumpImpulseScale").unwrap_or_default(),
            npc_bump_impulse_scale: inst.get_f32("npcBumpImpulseScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionSustainedForceDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionSustainedForceDef {
    /// `leanForceSmoothing` (Single)
    #[serde(default)]
    pub lean_force_smoothing: f32,
    /// `GForceToLeanForceRange` (Class)
    #[serde(default)]
    pub gforce_to_lean_force_range: Option<Handle<Range>>,
    /// `windToLeanForceRange` (Class)
    #[serde(default)]
    pub wind_to_lean_force_range: Option<Handle<Range>>,
    /// `impulseTriggerDelay` (Single)
    #[serde(default)]
    pub impulse_trigger_delay: f32,
    /// `impulseUpdateDelay` (Single)
    #[serde(default)]
    pub impulse_update_delay: f32,
    /// `GForceImpulseConfig` (Class)
    #[serde(default)]
    pub gforce_impulse_config: Option<Handle<SActorForceReactionSustainedImpulseDef>>,
    /// `windImpulseConfig` (Class)
    #[serde(default)]
    pub wind_impulse_config: Option<Handle<SActorForceReactionSustainedImpulseDef>>,
    /// `sustainedForceSmoothing` (Single)
    #[serde(default)]
    pub sustained_force_smoothing: f32,
    /// `GForceToSustainedForceRange` (Class)
    #[serde(default)]
    pub gforce_to_sustained_force_range: Option<Handle<Range>>,
    /// `windToSustainedForceRange` (Class)
    #[serde(default)]
    pub wind_to_sustained_force_range: Option<Handle<Range>>,
}

impl Pooled for SActorForceReactionSustainedForceDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reaction_sustained_force_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reaction_sustained_force_def }
}

impl<'a> Extract<'a> for SActorForceReactionSustainedForceDef {
    const TYPE_NAME: &'static str = "SActorForceReactionSustainedForceDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lean_force_smoothing: inst.get_f32("leanForceSmoothing").unwrap_or_default(),
            gforce_to_lean_force_range: match inst.get("GForceToLeanForceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wind_to_lean_force_range: match inst.get("windToLeanForceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            impulse_trigger_delay: inst.get_f32("impulseTriggerDelay").unwrap_or_default(),
            impulse_update_delay: inst.get_f32("impulseUpdateDelay").unwrap_or_default(),
            gforce_impulse_config: match inst.get("GForceImpulseConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionSustainedImpulseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionSustainedImpulseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wind_impulse_config: match inst.get("windImpulseConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionSustainedImpulseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionSustainedImpulseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_force_smoothing: inst.get_f32("sustainedForceSmoothing").unwrap_or_default(),
            gforce_to_sustained_force_range: match inst.get("GForceToSustainedForceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wind_to_sustained_force_range: match inst.get("windToSustainedForceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionsStunDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionsStunDef {
    /// `stunToImpulseRate` (Single)
    #[serde(default)]
    pub stun_to_impulse_rate: f32,
    /// `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// `maxImpulseScale` (Single)
    #[serde(default)]
    pub max_impulse_scale: f32,
    /// `affectsProjectiles` (Boolean)
    #[serde(default)]
    pub affects_projectiles: bool,
    /// `affectsMelee` (Boolean)
    #[serde(default)]
    pub affects_melee: bool,
    /// `affectsPhysics` (Boolean)
    #[serde(default)]
    pub affects_physics: bool,
    /// `affectsIndirect` (Boolean)
    #[serde(default)]
    pub affects_indirect: bool,
    /// `affectsSustainedDelta` (Boolean)
    #[serde(default)]
    pub affects_sustained_delta: bool,
}

impl Pooled for SActorForceReactionsStunDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reactions_stun_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reactions_stun_def }
}

impl<'a> Extract<'a> for SActorForceReactionsStunDef {
    const TYPE_NAME: &'static str = "SActorForceReactionsStunDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stun_to_impulse_rate: inst.get_f32("stunToImpulseRate").unwrap_or_default(),
            min_impulse: inst.get_f32("minImpulse").unwrap_or_default(),
            max_impulse_scale: inst.get_f32("maxImpulseScale").unwrap_or_default(),
            affects_projectiles: inst.get_bool("affectsProjectiles").unwrap_or_default(),
            affects_melee: inst.get_bool("affectsMelee").unwrap_or_default(),
            affects_physics: inst.get_bool("affectsPhysics").unwrap_or_default(),
            affects_indirect: inst.get_bool("affectsIndirect").unwrap_or_default(),
            affects_sustained_delta: inst.get_bool("affectsSustainedDelta").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionsVehicleForceDampeningDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionsVehicleForceDampeningDef {
    /// `minMass` (Single)
    #[serde(default)]
    pub min_mass: f32,
    /// `gForceCutoff` (Single)
    #[serde(default)]
    pub g_force_cutoff: f32,
    /// `gForceScale` (Single)
    #[serde(default)]
    pub g_force_scale: f32,
    /// `externalImpulseCutoff` (Single)
    #[serde(default)]
    pub external_impulse_cutoff: f32,
    /// `externalImpulseScale` (Single)
    #[serde(default)]
    pub external_impulse_scale: f32,
}

impl Pooled for SActorForceReactionsVehicleForceDampeningDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reactions_vehicle_force_dampening_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reactions_vehicle_force_dampening_def }
}

impl<'a> Extract<'a> for SActorForceReactionsVehicleForceDampeningDef {
    const TYPE_NAME: &'static str = "SActorForceReactionsVehicleForceDampeningDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_mass: inst.get_f32("minMass").unwrap_or_default(),
            g_force_cutoff: inst.get_f32("gForceCutoff").unwrap_or_default(),
            g_force_scale: inst.get_f32("gForceScale").unwrap_or_default(),
            external_impulse_cutoff: inst.get_f32("externalImpulseCutoff").unwrap_or_default(),
            external_impulse_scale: inst.get_f32("externalImpulseScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionsDef {
    /// `physicsImpulseScale` (Single)
    #[serde(default)]
    pub physics_impulse_scale: f32,
    /// `explosionsImpulseScale` (Single)
    #[serde(default)]
    pub explosions_impulse_scale: f32,
    /// `meleeImpulseScale` (Single)
    #[serde(default)]
    pub melee_impulse_scale: f32,
    /// `bulletImpulseScale` (Single)
    #[serde(default)]
    pub bullet_impulse_scale: f32,
    /// `actorBumpImpulseConfig` (Class)
    #[serde(default)]
    pub actor_bump_impulse_config: Option<Handle<SActorForceReactionActorBumpDef>>,
    /// `externalImpulseConfig` (Class)
    #[serde(default)]
    pub external_impulse_config: Option<Handle<SActorForceReactionExternalImpulseDef>>,
    /// `sustainedForceConfig` (Class)
    #[serde(default)]
    pub sustained_force_config: Option<Handle<SActorForceReactionSustainedForceDef>>,
    /// `vehicleForceDampeningConfig` (Class (array))
    #[serde(default)]
    pub vehicle_force_dampening_config: Vec<Handle<SActorForceReactionsVehicleForceDampeningDef>>,
    /// `stunConfig` (Class)
    #[serde(default)]
    pub stun_config: Option<Handle<SActorForceReactionsStunDef>>,
    /// `reactionLimits` (Class)
    #[serde(default)]
    pub reaction_limits: Option<Handle<SActorForceReactionLimitDef>>,
    /// `staggerGlobalConfig` (Class)
    #[serde(default)]
    pub stagger_global_config: Option<Handle<SActorForceReactionGlobalStaggerConfig>>,
    /// `impulseAccumulationConfig` (Class)
    #[serde(default)]
    pub impulse_accumulation_config: Option<Handle<SActorForceReactionImpulseAccumulationConfig>>,
    /// `effectGlobalConfig` (Class)
    #[serde(default)]
    pub effect_global_config: Option<Handle<SActorForceReactionGlobalEffectConfig>>,
    /// `filters` (Class)
    #[serde(default)]
    pub filters: Option<Handle<SActorForceReactionFilterDef>>,
    /// `twitchConfigs` (Class (array))
    #[serde(default)]
    pub twitch_configs: Vec<Handle<SActorForceReactionTwitchConfigDef>>,
    /// `flinchConfigs` (Class (array))
    #[serde(default)]
    pub flinch_configs: Vec<Handle<SActorForceReactionFlinchConfigDef>>,
    /// `sustainedDeltaFlinchConfigs` (Class (array))
    #[serde(default)]
    pub sustained_delta_flinch_configs: Vec<Handle<SActorForceReactionFlinchConfigDef>>,
    /// `directStaggerConfig` (Class)
    #[serde(default)]
    pub direct_stagger_config: Option<Handle<SActorForceReactionFilteredStaggerConfigDef>>,
    /// `indirectStaggerConfig` (Class)
    #[serde(default)]
    pub indirect_stagger_config: Option<Handle<SActorForceReactionFilteredStaggerConfigDef>>,
    /// `sustainedDeltaStaggerConfig` (Class)
    #[serde(default)]
    pub sustained_delta_stagger_config: Option<Handle<SActorForceReactionUnfilteredStaggerConfigDef>>,
    /// `directKnockdownConfig` (Class)
    #[serde(default)]
    pub direct_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// `indirectKnockdownConfig` (Class)
    #[serde(default)]
    pub indirect_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// `sustainedDeltaKnockdownConfig` (Class)
    #[serde(default)]
    pub sustained_delta_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// `sustainedKnockdownConfig` (Class)
    #[serde(default)]
    pub sustained_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// `sustainedStumbleConfig` (Class)
    #[serde(default)]
    pub sustained_stumble_config: Option<Handle<SActorForceReactionStumbleConfigDef>>,
    /// `leanConfig` (Class)
    #[serde(default)]
    pub lean_config: Option<Handle<SActorForceReactionLeanConfigDef>>,
}

impl Pooled for SActorForceReactionsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_force_reactions_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_force_reactions_def }
}

impl<'a> Extract<'a> for SActorForceReactionsDef {
    const TYPE_NAME: &'static str = "SActorForceReactionsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            physics_impulse_scale: inst.get_f32("physicsImpulseScale").unwrap_or_default(),
            explosions_impulse_scale: inst.get_f32("explosionsImpulseScale").unwrap_or_default(),
            melee_impulse_scale: inst.get_f32("meleeImpulseScale").unwrap_or_default(),
            bullet_impulse_scale: inst.get_f32("bulletImpulseScale").unwrap_or_default(),
            actor_bump_impulse_config: match inst.get("actorBumpImpulseConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionActorBumpDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionActorBumpDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            external_impulse_config: match inst.get("externalImpulseConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionExternalImpulseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionExternalImpulseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_force_config: match inst.get("sustainedForceConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionSustainedForceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionSustainedForceDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_force_dampening_config: inst.get_array("vehicleForceDampeningConfig")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionsVehicleForceDampeningDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionsVehicleForceDampeningDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            stun_config: match inst.get("stunConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionsStunDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionsStunDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reaction_limits: match inst.get("reactionLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionLimitDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionLimitDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stagger_global_config: match inst.get("staggerGlobalConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionGlobalStaggerConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionGlobalStaggerConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            impulse_accumulation_config: match inst.get("impulseAccumulationConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionImpulseAccumulationConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionImpulseAccumulationConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            effect_global_config: match inst.get("effectGlobalConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionGlobalEffectConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionGlobalEffectConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            filters: match inst.get("filters") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilterDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilterDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            twitch_configs: inst.get_array("twitchConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionTwitchConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionTwitchConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            flinch_configs: inst.get_array("flinchConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionFlinchConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionFlinchConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            sustained_delta_flinch_configs: inst.get_array("sustainedDeltaFlinchConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorForceReactionFlinchConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionFlinchConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            direct_stagger_config: match inst.get("directStaggerConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilteredStaggerConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilteredStaggerConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            indirect_stagger_config: match inst.get("indirectStaggerConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionFilteredStaggerConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionFilteredStaggerConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_delta_stagger_config: match inst.get("sustainedDeltaStaggerConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionUnfilteredStaggerConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionUnfilteredStaggerConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            direct_knockdown_config: match inst.get("directKnockdownConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            indirect_knockdown_config: match inst.get("indirectKnockdownConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_delta_knockdown_config: match inst.get("sustainedDeltaKnockdownConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_knockdown_config: match inst.get("sustainedKnockdownConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionKnockdownConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sustained_stumble_config: match inst.get("sustainedStumbleConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionStumbleConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionStumbleConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            lean_config: match inst.get("leanConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorForceReactionLeanConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorForceReactionLeanConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorHitReactionsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorHitReactionsDef {
    /// `impactAccumulationTime` (Single)
    #[serde(default)]
    pub impact_accumulation_time: f32,
    /// `impactReductionFromMass` (Single)
    #[serde(default)]
    pub impact_reduction_from_mass: f32,
    /// `physicsImpactScalePlayer` (Single)
    #[serde(default)]
    pub physics_impact_scale_player: f32,
    /// `physicsImpactScaleAI` (Single)
    #[serde(default)]
    pub physics_impact_scale_ai: f32,
    /// `deathAnimationInterruptionDelay` (Single)
    #[serde(default)]
    pub death_animation_interruption_delay: f32,
    /// `enableHitReactionsLight` (Boolean)
    #[serde(default)]
    pub enable_hit_reactions_light: bool,
    /// `enableHitReactionsMedium` (Boolean)
    #[serde(default)]
    pub enable_hit_reactions_medium: bool,
    /// `enableHitReactionsHeavy` (Boolean)
    #[serde(default)]
    pub enable_hit_reactions_heavy: bool,
    /// `hitThresholdLight` (Single)
    #[serde(default)]
    pub hit_threshold_light: f32,
    /// `hitThresholdMedium` (Single)
    #[serde(default)]
    pub hit_threshold_medium: f32,
    /// `hitThresholdHeavy` (Single)
    #[serde(default)]
    pub hit_threshold_heavy: f32,
}

impl Pooled for SActorHitReactionsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_hit_reactions_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_hit_reactions_def }
}

impl<'a> Extract<'a> for SActorHitReactionsDef {
    const TYPE_NAME: &'static str = "SActorHitReactionsDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            impact_accumulation_time: inst.get_f32("impactAccumulationTime").unwrap_or_default(),
            impact_reduction_from_mass: inst.get_f32("impactReductionFromMass").unwrap_or_default(),
            physics_impact_scale_player: inst.get_f32("physicsImpactScalePlayer").unwrap_or_default(),
            physics_impact_scale_ai: inst.get_f32("physicsImpactScaleAI").unwrap_or_default(),
            death_animation_interruption_delay: inst.get_f32("deathAnimationInterruptionDelay").unwrap_or_default(),
            enable_hit_reactions_light: inst.get_bool("enableHitReactionsLight").unwrap_or_default(),
            enable_hit_reactions_medium: inst.get_bool("enableHitReactionsMedium").unwrap_or_default(),
            enable_hit_reactions_heavy: inst.get_bool("enableHitReactionsHeavy").unwrap_or_default(),
            hit_threshold_light: inst.get_f32("hitThresholdLight").unwrap_or_default(),
            hit_threshold_medium: inst.get_f32("hitThresholdMedium").unwrap_or_default(),
            hit_threshold_heavy: inst.get_f32("hitThresholdHeavy").unwrap_or_default(),
        }
    }
}

/// DCB type: `SPlayerRoleShakeMultipliers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPlayerRoleShakeMultipliers {
    /// `multipliers` (Single)
    #[serde(default)]
    pub multipliers: f32,
}

impl Pooled for SPlayerRoleShakeMultipliers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.splayer_role_shake_multipliers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.splayer_role_shake_multipliers }
}

impl<'a> Extract<'a> for SPlayerRoleShakeMultipliers {
    const TYPE_NAME: &'static str = "SPlayerRoleShakeMultipliers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            multipliers: inst.get_f32("multipliers").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorExternalForceResponseVibrationEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorExternalForceResponseVibrationEntry {
    /// `shakes` (Class (array))
    #[serde(default)]
    pub shakes: Vec<Handle<CameraActorVibrationShakeConfig>>,
}

impl Pooled for SActorExternalForceResponseVibrationEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_external_force_response_vibration_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_external_force_response_vibration_entry }
}

impl<'a> Extract<'a> for SActorExternalForceResponseVibrationEntry {
    const TYPE_NAME: &'static str = "SActorExternalForceResponseVibrationEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            shakes: inst.get_array("shakes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CameraActorVibrationShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CameraActorVibrationShakeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorExternalForceResponseCameraShakeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorExternalForceResponseCameraShakeDef {
    /// `vibrationShakes` (Class)
    #[serde(default)]
    pub vibration_shakes: Option<Handle<SActorExternalForceResponseVibrationEntry>>,
    /// `roleShakeMultipliers` (Class)
    #[serde(default)]
    pub role_shake_multipliers: Option<Handle<SPlayerRoleShakeMultipliers>>,
}

impl Pooled for SActorExternalForceResponseCameraShakeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.sactor_external_force_response_camera_shake_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.sactor_external_force_response_camera_shake_def }
}

impl<'a> Extract<'a> for SActorExternalForceResponseCameraShakeDef {
    const TYPE_NAME: &'static str = "SActorExternalForceResponseCameraShakeDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vibration_shakes: match inst.get("vibrationShakes") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorExternalForceResponseVibrationEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorExternalForceResponseVibrationEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            role_shake_multipliers: match inst.get("roleShakeMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SPlayerRoleShakeMultipliers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SPlayerRoleShakeMultipliers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorForceReactionsProceduralLeanOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionsProceduralLeanOverride {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `maxLeanForward` (Single)
    #[serde(default)]
    pub max_lean_forward: f32,
    /// `maxLeanBackward` (Single)
    #[serde(default)]
    pub max_lean_backward: f32,
    /// `maxLeanLeft` (Single)
    #[serde(default)]
    pub max_lean_left: f32,
    /// `maxLeanRight` (Single)
    #[serde(default)]
    pub max_lean_right: f32,
    /// `moveHips` (Boolean)
    #[serde(default)]
    pub move_hips: bool,
    /// `lockHands` (Boolean)
    #[serde(default)]
    pub lock_hands: bool,
    /// `pose` (EnumChoice)
    #[serde(default)]
    pub pose: String,
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
            pose: inst.get_str("pose").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionsPresetRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionsPresetRecord {
    /// `procLeanOverrides` (Class (array))
    #[serde(default)]
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
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorForceReactionsProceduralLeanOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraActorVibrationShakeConfig`
/// Inherits from: `CameraShakeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraActorVibrationShakeConfig {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `offsetPosition` (Class)
    #[serde(default)]
    pub offset_position: Option<Handle<Vec3>>,
    /// `offsetAngle` (Class)
    #[serde(default)]
    pub offset_angle: Option<Handle<Ang3>>,
    /// `timePeriod` (Single)
    #[serde(default)]
    pub time_period: f32,
    /// `frequencyNoiseFactor` (Single)
    #[serde(default)]
    pub frequency_noise_factor: f32,
    /// `translationNoise` (Single)
    #[serde(default)]
    pub translation_noise: f32,
    /// `rotationNoise` (Single)
    #[serde(default)]
    pub rotation_noise: f32,
    /// `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// `vibrationOutputMapping` (Class)
    #[serde(default)]
    pub vibration_output_mapping: Option<Handle<BezierCurve>>,
    /// `processOnlyOnVibrationIncrease` (Boolean)
    #[serde(default)]
    pub process_only_on_vibration_increase: bool,
    /// `processOnlyOnVibrationIncreaseDuration` (Single)
    #[serde(default)]
    pub process_only_on_vibration_increase_duration: f32,
    /// `processOnlyOnVibrationIncreaseTimeMapping` (Class)
    #[serde(default)]
    pub process_only_on_vibration_increase_time_mapping: Option<Handle<BezierCurve>>,
    /// `scaleFirstPerson` (Single)
    #[serde(default)]
    pub scale_first_person: f32,
    /// `scaleThirdPerson` (Single)
    #[serde(default)]
    pub scale_third_person: f32,
    /// `smoothFactor` (Single)
    #[serde(default)]
    pub smooth_factor: f32,
}

impl Pooled for CameraActorVibrationShakeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_externalforceresponse.camera_actor_vibration_shake_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_externalforceresponse.camera_actor_vibration_shake_config }
}

impl<'a> Extract<'a> for CameraActorVibrationShakeConfig {
    const TYPE_NAME: &'static str = "CameraActorVibrationShakeConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            offset_position: match inst.get("offsetPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_angle: match inst.get("offsetAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time_period: inst.get_f32("timePeriod").unwrap_or_default(),
            frequency_noise_factor: inst.get_f32("frequencyNoiseFactor").unwrap_or_default(),
            translation_noise: inst.get_f32("translationNoise").unwrap_or_default(),
            rotation_noise: inst.get_f32("rotationNoise").unwrap_or_default(),
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            vibration_output_mapping: match inst.get("vibrationOutputMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            process_only_on_vibration_increase: inst.get_bool("processOnlyOnVibrationIncrease").unwrap_or_default(),
            process_only_on_vibration_increase_duration: inst.get_f32("processOnlyOnVibrationIncreaseDuration").unwrap_or_default(),
            process_only_on_vibration_increase_time_mapping: match inst.get("processOnlyOnVibrationIncreaseTimeMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scale_first_person: inst.get_f32("scaleFirstPerson").unwrap_or_default(),
            scale_third_person: inst.get_f32("scaleThirdPerson").unwrap_or_default(),
            smooth_factor: inst.get_f32("smoothFactor").unwrap_or_default(),
        }
    }
}

