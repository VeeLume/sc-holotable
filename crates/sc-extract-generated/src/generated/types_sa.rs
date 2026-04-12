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

/// DCB type: `SAIPerceptionDisturbanceTypeSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAIPerceptionDisturbanceTypeSettings {
    /// DCB field: `disturbanceScoreThreshold` (Single)
    #[serde(default)]
    pub disturbance_score_threshold: f32,
    /// DCB field: `disturbanceScore` (UInt32)
    #[serde(default)]
    pub disturbance_score: u32,
}

impl Pooled for SAIPerceptionDisturbanceTypeSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saiperception_disturbance_type_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saiperception_disturbance_type_settings }
}

impl<'a> Extract<'a> for SAIPerceptionDisturbanceTypeSettings {
    const TYPE_NAME: &'static str = "SAIPerceptionDisturbanceTypeSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            disturbance_score_threshold: inst.get_f32("disturbanceScoreThreshold").unwrap_or_default(),
            disturbance_score: inst.get_u32("disturbanceScore").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAIPerceptionMeterSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAIPerceptionMeterSettings {
    /// DCB field: `curiousPerceptionBehaviourThresholds` (Single)
    #[serde(default)]
    pub curious_perception_behaviour_thresholds: f32,
    /// DCB field: `threatenedPerceptionBehaviourThresholds` (Single)
    #[serde(default)]
    pub threatened_perception_behaviour_thresholds: f32,
    /// DCB field: `threatenedDisturbanceThreshold` (UInt32)
    #[serde(default)]
    pub threatened_disturbance_threshold: u32,
    /// DCB field: `meterDecayRate` (Single)
    #[serde(default)]
    pub meter_decay_rate: f32,
    /// DCB field: `disturbanceTypeSettings` (Class)
    #[serde(default)]
    pub disturbance_type_settings: Option<Handle<SAIPerceptionDisturbanceTypeSettings>>,
}

impl Pooled for SAIPerceptionMeterSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saiperception_meter_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saiperception_meter_settings }
}

impl<'a> Extract<'a> for SAIPerceptionMeterSettings {
    const TYPE_NAME: &'static str = "SAIPerceptionMeterSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curious_perception_behaviour_thresholds: inst.get_f32("curiousPerceptionBehaviourThresholds").unwrap_or_default(),
            threatened_perception_behaviour_thresholds: inst.get_f32("threatenedPerceptionBehaviourThresholds").unwrap_or_default(),
            threatened_disturbance_threshold: inst.get_u32("threatenedDisturbanceThreshold").unwrap_or_default(),
            meter_decay_rate: inst.get_f32("meterDecayRate").unwrap_or_default(),
            disturbance_type_settings: match inst.get("disturbanceTypeSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionDisturbanceTypeSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionDisturbanceTypeSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAIPerceptionAudioTypeSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAIPerceptionAudioTypeSettings {
    /// DCB field: `audioIntensityCurve` (Class)
    #[serde(default)]
    pub audio_intensity_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `perceptionMeterCap` (Single)
    #[serde(default)]
    pub perception_meter_cap: f32,
    /// DCB field: `isCombatAudioEvent` (Boolean)
    #[serde(default)]
    pub is_combat_audio_event: bool,
    /// DCB field: `isDistractionAudioEvent` (Boolean)
    #[serde(default)]
    pub is_distraction_audio_event: bool,
}

impl Pooled for SAIPerceptionAudioTypeSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saiperception_audio_type_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saiperception_audio_type_settings }
}

impl<'a> Extract<'a> for SAIPerceptionAudioTypeSettings {
    const TYPE_NAME: &'static str = "SAIPerceptionAudioTypeSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_intensity_curve: match inst.get("audioIntensityCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            perception_meter_cap: inst.get_f32("perceptionMeterCap").unwrap_or_default(),
            is_combat_audio_event: inst.get_bool("isCombatAudioEvent").unwrap_or_default(),
            is_distraction_audio_event: inst.get_bool("isDistractionAudioEvent").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAIPerceptionAudioSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAIPerceptionAudioSettings {
    /// DCB field: `audioTypeSettings` (Class)
    #[serde(default)]
    pub audio_type_settings: Option<Handle<SAIPerceptionAudioTypeSettings>>,
}

impl Pooled for SAIPerceptionAudioSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saiperception_audio_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saiperception_audio_settings }
}

impl<'a> Extract<'a> for SAIPerceptionAudioSettings {
    const TYPE_NAME: &'static str = "SAIPerceptionAudioSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_type_settings: match inst.get("audioTypeSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionAudioTypeSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionAudioTypeSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAIPerceptionVisualSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAIPerceptionVisualSettings {
    /// DCB field: `firstSeenPerceptionMeterDistanceCurve` (Class)
    #[serde(default)]
    pub first_seen_perception_meter_distance_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `visualIntensityDistanceCurve` (Class)
    #[serde(default)]
    pub visual_intensity_distance_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `disguisedVisualIntensityScaleDistanceCurve` (Class)
    #[serde(default)]
    pub disguised_visual_intensity_scale_distance_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `lightIntensityVisualIntensityScaleCurve` (Class)
    #[serde(default)]
    pub light_intensity_visual_intensity_scale_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `lowStanceVisualIntensityScale` (Single)
    #[serde(default)]
    pub low_stance_visual_intensity_scale: f32,
    /// DCB field: `peripheralFOVVisualIntensityScale` (Single)
    #[serde(default)]
    pub peripheral_fovvisual_intensity_scale: f32,
    /// DCB field: `sixthSenseFOVVisualIntensityScale` (Single)
    #[serde(default)]
    pub sixth_sense_fovvisual_intensity_scale: f32,
}

impl Pooled for SAIPerceptionVisualSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saiperception_visual_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saiperception_visual_settings }
}

impl<'a> Extract<'a> for SAIPerceptionVisualSettings {
    const TYPE_NAME: &'static str = "SAIPerceptionVisualSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            first_seen_perception_meter_distance_curve: match inst.get("firstSeenPerceptionMeterDistanceCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            visual_intensity_distance_curve: match inst.get("visualIntensityDistanceCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disguised_visual_intensity_scale_distance_curve: match inst.get("disguisedVisualIntensityScaleDistanceCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            light_intensity_visual_intensity_scale_curve: match inst.get("lightIntensityVisualIntensityScaleCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            low_stance_visual_intensity_scale: inst.get_f32("lowStanceVisualIntensityScale").unwrap_or_default(),
            peripheral_fovvisual_intensity_scale: inst.get_f32("peripheralFOVVisualIntensityScale").unwrap_or_default(),
            sixth_sense_fovvisual_intensity_scale: inst.get_f32("sixthSenseFOVVisualIntensityScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorForceReactionFilterRangeOverrideDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFilterRangeOverrideDef {
    /// DCB field: `filterByForceType` (EnumChoice)
    #[serde(default)]
    pub filter_by_force_type: String,
    /// DCB field: `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// DCB field: `maxImpulse` (Single)
    #[serde(default)]
    pub max_impulse: f32,
}

impl Pooled for SActorForceReactionFilterRangeOverrideDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_filter_range_override_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_filter_range_override_def }
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
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// DCB field: `maxImpulse` (Single)
    #[serde(default)]
    pub max_impulse: f32,
    /// DCB field: `rangeOverride` (Class (array))
    #[serde(default)]
    pub range_override: Vec<Handle<SActorForceReactionFilterRangeOverrideDef>>,
}

impl Pooled for SActorForceReactionFilterItemDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_filter_item_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_filter_item_def }
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
///
/// Inherits from: `SActorForceReactionFilterItemDef` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionLeanFilterItemDef {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// DCB field: `maxImpulse` (Single)
    #[serde(default)]
    pub max_impulse: f32,
    /// DCB field: `rangeOverride` (Class (array))
    #[serde(default)]
    pub range_override: Vec<Handle<SActorForceReactionFilterRangeOverrideDef>>,
    /// DCB field: `defaultLeanPose` (EnumChoice)
    #[serde(default)]
    pub default_lean_pose: String,
}

impl Pooled for SActorForceReactionLeanFilterItemDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_lean_filter_item_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_lean_filter_item_def }
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
    /// DCB field: `weaponTwitchMaxOffset` (Single)
    #[serde(default)]
    pub weapon_twitch_max_offset: f32,
    /// DCB field: `aimPunchMaxAngle` (Single)
    #[serde(default)]
    pub aim_punch_max_angle: f32,
    /// DCB field: `headRecoilMaxAngleHor` (Single)
    #[serde(default)]
    pub head_recoil_max_angle_hor: f32,
    /// DCB field: `headRecoilMaxAngleVert` (Single)
    #[serde(default)]
    pub head_recoil_max_angle_vert: f32,
    /// DCB field: `headRecoilMaxAngleRoll` (Single)
    #[serde(default)]
    pub head_recoil_max_angle_roll: f32,
}

impl Pooled for SActorForceReactionLimitDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_limit_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_limit_def }
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
    /// DCB field: `delayStaggersWhileInState` (Boolean)
    #[serde(default)]
    pub delay_staggers_while_in_state: bool,
    /// DCB field: `staggerDelayTimeout` (Single)
    #[serde(default)]
    pub stagger_delay_timeout: f32,
}

impl Pooled for SActorForceReactionStateConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_state_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_state_config }
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
    /// DCB field: `stateConfig` (Class)
    #[serde(default)]
    pub state_config: Option<Handle<SActorForceReactionStateConfig>>,
    /// DCB field: `twitches` (Class)
    #[serde(default)]
    pub twitches: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `directStaggers` (Class)
    #[serde(default)]
    pub direct_staggers: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `directKnockdowns` (Class)
    #[serde(default)]
    pub direct_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `flinches` (Class)
    #[serde(default)]
    pub flinches: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `indirectStaggers` (Class)
    #[serde(default)]
    pub indirect_staggers: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `indirectKnockdowns` (Class)
    #[serde(default)]
    pub indirect_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `sustainedDeltaFlinches` (Class)
    #[serde(default)]
    pub sustained_delta_flinches: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `sustainedDeltaStaggers` (Class)
    #[serde(default)]
    pub sustained_delta_staggers: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `sustainedDeltaKnockdowns` (Class)
    #[serde(default)]
    pub sustained_delta_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `sustainedStumble` (Class)
    #[serde(default)]
    pub sustained_stumble: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `sustainedKnockdowns` (Class)
    #[serde(default)]
    pub sustained_knockdowns: Option<Handle<SActorForceReactionFilterItemDef>>,
    /// DCB field: `lean` (Class)
    #[serde(default)]
    pub lean: Option<Handle<SActorForceReactionLeanFilterItemDef>>,
}

impl Pooled for SActorForceReactionFilterDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_filter_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_filter_def }
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
    /// DCB field: `attackDuration` (Single)
    #[serde(default)]
    pub attack_duration: f32,
    /// DCB field: `sustainDuration` (Single)
    #[serde(default)]
    pub sustain_duration: f32,
    /// DCB field: `releaseDuration` (Single)
    #[serde(default)]
    pub release_duration: f32,
}

impl Pooled for SActorForceReactionEnvelope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_envelope }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_envelope }
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
    /// DCB field: `curveName` (String)
    #[serde(default)]
    pub curve_name: String,
    /// DCB field: `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SActorForceReactionCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_curve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_curve }
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
    /// DCB field: `attackDuration` (Single)
    #[serde(default)]
    pub attack_duration: f32,
    /// DCB field: `returnDuration` (Single)
    #[serde(default)]
    pub return_duration: f32,
    /// DCB field: `returnCurve` (WeakPointer)
    #[serde(default)]
    pub return_curve: Option<Handle<SActorForceReactionCurve>>,
    /// DCB field: `returnYieldDelay` (Single)
    #[serde(default)]
    pub return_yield_delay: f32,
    /// DCB field: `returnYieldMagnitude` (Single)
    #[serde(default)]
    pub return_yield_magnitude: f32,
}

impl Pooled for SActorForceReactionCurveConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_curve_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_curve_config }
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
    /// DCB field: `yieldMagnitudeMinScale` (Single)
    #[serde(default)]
    pub yield_magnitude_min_scale: f32,
    /// DCB field: `returnYieldDelay` (Single)
    #[serde(default)]
    pub return_yield_delay: f32,
    /// DCB field: `returnYieldMagnitude` (Single)
    #[serde(default)]
    pub return_yield_magnitude: f32,
}

impl Pooled for SActorForceReactionEffectDefaults {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_effect_defaults }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_effect_defaults }
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
    /// DCB field: `effectCurves` (Class (array))
    #[serde(default)]
    pub effect_curves: Vec<Handle<SActorForceReactionCurve>>,
    /// DCB field: `aimPunchDefaults` (Class)
    #[serde(default)]
    pub aim_punch_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// DCB field: `weaponTwitchDefaults` (Class)
    #[serde(default)]
    pub weapon_twitch_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// DCB field: `headRecoilPlanarDefaults` (Class)
    #[serde(default)]
    pub head_recoil_planar_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// DCB field: `headRecoilRollDefaults` (Class)
    #[serde(default)]
    pub head_recoil_roll_defaults: Option<Handle<SActorForceReactionEffectDefaults>>,
    /// DCB field: `aimPunchFrontMaxAngle` (Single)
    #[serde(default)]
    pub aim_punch_front_max_angle: f32,
    /// DCB field: `aimPunchBackMaxAngle` (Single)
    #[serde(default)]
    pub aim_punch_back_max_angle: f32,
    /// DCB field: `headRecoilFrontMaxAngle` (Single)
    #[serde(default)]
    pub head_recoil_front_max_angle: f32,
    /// DCB field: `headRecoilBackMaxAngle` (Single)
    #[serde(default)]
    pub head_recoil_back_max_angle: f32,
    /// DCB field: `headRecoilRollAxis` (Class)
    #[serde(default)]
    pub head_recoil_roll_axis: Option<Handle<Vec3>>,
}

impl Pooled for SActorForceReactionGlobalEffectConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_global_effect_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_global_effect_config }
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
    /// DCB field: `adsZoomScaleFactor` (Single)
    #[serde(default)]
    pub ads_zoom_scale_factor: f32,
    /// DCB field: `aimPunchDirConeAngle` (Single)
    #[serde(default)]
    pub aim_punch_dir_cone_angle: f32,
    /// DCB field: `horizontalAimPunchRange` (Class)
    #[serde(default)]
    pub horizontal_aim_punch_range: Option<Handle<Range>>,
    /// DCB field: `verticalAimPunchRange` (Class)
    #[serde(default)]
    pub vertical_aim_punch_range: Option<Handle<Range>>,
    /// DCB field: `horizontalRandomAimPunchAtMaxImpulse` (Single)
    #[serde(default)]
    pub horizontal_random_aim_punch_at_max_impulse: f32,
    /// DCB field: `verticalRandomAimPunchAtMaxImpulse` (Single)
    #[serde(default)]
    pub vertical_random_aim_punch_at_max_impulse: f32,
    /// DCB field: `curveConfig` (Class)
    #[serde(default)]
    pub curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
}

impl Pooled for SActorForceReactionAimPunchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_aim_punch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_aim_punch_config }
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
    /// DCB field: `adsZoomScaleFactor` (Single)
    #[serde(default)]
    pub ads_zoom_scale_factor: f32,
    /// DCB field: `offsetAtMinImpulse` (Single)
    #[serde(default)]
    pub offset_at_min_impulse: f32,
    /// DCB field: `offsetAtMaxImpulse` (Single)
    #[serde(default)]
    pub offset_at_max_impulse: f32,
    /// DCB field: `offsetRandomAtMaxImpulse` (Single)
    #[serde(default)]
    pub offset_random_at_max_impulse: f32,
    /// DCB field: `curveConfig` (Class)
    #[serde(default)]
    pub curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
}

impl Pooled for SActorForceReactionWeaponTwitchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_weapon_twitch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_weapon_twitch_config }
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
    /// DCB field: `planarADSZoomScaleFactor` (Single)
    #[serde(default)]
    pub planar_adszoom_scale_factor: f32,
    /// DCB field: `rollADSZoomScaleFactor` (Single)
    #[serde(default)]
    pub roll_adszoom_scale_factor: f32,
    /// DCB field: `planarDirConeAngle` (Single)
    #[serde(default)]
    pub planar_dir_cone_angle: f32,
    /// DCB field: `horizontalRecoilRange` (Class)
    #[serde(default)]
    pub horizontal_recoil_range: Option<Handle<Range>>,
    /// DCB field: `horizontalRandomRecoilAtMaxImpulse` (Single)
    #[serde(default)]
    pub horizontal_random_recoil_at_max_impulse: f32,
    /// DCB field: `verticalRecoilRange` (Class)
    #[serde(default)]
    pub vertical_recoil_range: Option<Handle<Range>>,
    /// DCB field: `verticalRandomRecoilAtMaxImpulse` (Single)
    #[serde(default)]
    pub vertical_random_recoil_at_max_impulse: f32,
    /// DCB field: `rollRecoilRange` (Class)
    #[serde(default)]
    pub roll_recoil_range: Option<Handle<Range>>,
    /// DCB field: `rollRandomRecoilAtMaxImpulse` (Single)
    #[serde(default)]
    pub roll_random_recoil_at_max_impulse: f32,
    /// DCB field: `rollAxisInfluencePct` (Single)
    #[serde(default)]
    pub roll_axis_influence_pct: f32,
    /// DCB field: `planarCurveConfig` (Class)
    #[serde(default)]
    pub planar_curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
    /// DCB field: `rollCurveConfig` (Class)
    #[serde(default)]
    pub roll_curve_config: Option<Handle<SActorForceReactionCurveConfig>>,
}

impl Pooled for SActorForceReactionHeadRecoilConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_head_recoil_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_head_recoil_config }
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
    /// DCB field: `adsZoomScaleFactor` (Single)
    #[serde(default)]
    pub ads_zoom_scale_factor: f32,
    /// DCB field: `FOVScaleAtMinImpulse` (Single)
    #[serde(default)]
    pub fovscale_at_min_impulse: f32,
    /// DCB field: `FOVScaleAtMaxImpulse` (Single)
    #[serde(default)]
    pub fovscale_at_max_impulse: f32,
    /// DCB field: `envelope` (Class)
    #[serde(default)]
    pub envelope: Option<Handle<SActorForceReactionEnvelope>>,
}

impl Pooled for SActorForceReactionFOVScaleConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_fovscale_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_fovscale_config }
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
    /// DCB field: `blendspaceMin` (Single)
    #[serde(default)]
    pub blendspace_min: f32,
}

impl Pooled for SActorForceReactionAnimationTwitchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_animation_twitch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_animation_twitch_config }
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
    /// DCB field: `blendspaceMin` (Single)
    #[serde(default)]
    pub blendspace_min: f32,
}

impl Pooled for SActorForceReactionAnimationFlinchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_animation_flinch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_animation_flinch_config }
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
    /// DCB field: `blendspaceMin` (Single)
    #[serde(default)]
    pub blendspace_min: f32,
    /// DCB field: `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
}

impl Pooled for SActorForceReactionAnimationStaggerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_animation_stagger_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_animation_stagger_config }
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
    /// DCB field: `mannequinTag` (String)
    #[serde(default)]
    pub mannequin_tag: String,
    /// DCB field: `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
}

impl Pooled for SActorForceReactionStaggerTagConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_stagger_tag_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_stagger_tag_config }
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
    /// DCB field: `movementReferenceVelocity` (Class)
    #[serde(default)]
    pub movement_reference_velocity: Option<Handle<Range>>,
    /// DCB field: `staggerDistance` (Class)
    #[serde(default)]
    pub stagger_distance: Option<Handle<Range>>,
    /// DCB field: `distanceTags` (Class (array))
    #[serde(default)]
    pub distance_tags: Vec<Handle<SActorForceReactionStaggerTagConfig>>,
    /// DCB field: `viewPitchRangeDeg` (Single)
    #[serde(default)]
    pub view_pitch_range_deg: f32,
    /// DCB field: `viewYawRangeDeg` (Single)
    #[serde(default)]
    pub view_yaw_range_deg: f32,
}

impl Pooled for SActorForceReactionGlobalStaggerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_global_stagger_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_global_stagger_config }
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
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// DCB field: `accumulationFraction` (Single)
    #[serde(default)]
    pub accumulation_fraction: f32,
    /// DCB field: `cooldown` (Single)
    #[serde(default)]
    pub cooldown: f32,
}

impl Pooled for SActorForceReactionImpulseAccumulationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_impulse_accumulation_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_impulse_accumulation_config }
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
    /// DCB field: `forcePercent` (Single)
    #[serde(default)]
    pub force_percent: f32,
    /// DCB field: `flightDuration` (Single)
    #[serde(default)]
    pub flight_duration: f32,
}

impl Pooled for SActorForceReactionFlightDurationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_flight_duration_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_flight_duration_config }
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
    /// DCB field: `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// DCB field: `durations` (Class (array))
    #[serde(default)]
    pub durations: Vec<Handle<SActorForceReactionFlightDurationConfig>>,
}

impl Pooled for SActorForceReactionMovementLaunchConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_movement_launch_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_movement_launch_config }
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
    /// DCB field: `blockADS` (Boolean)
    #[serde(default)]
    pub block_ads: bool,
    /// DCB field: `triggerADSBlockDuration` (Single)
    #[serde(default)]
    pub trigger_adsblock_duration: f32,
    /// DCB field: `heldADSBlockDuration` (Single)
    #[serde(default)]
    pub held_adsblock_duration: f32,
}

impl Pooled for SActorForceReactionBlockADSConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_block_adsconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_block_adsconfig }
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
    /// DCB field: `directTwitchDisableDuration` (Single)
    #[serde(default)]
    pub direct_twitch_disable_duration: f32,
    /// DCB field: `directStaggerDisableDuration` (Single)
    #[serde(default)]
    pub direct_stagger_disable_duration: f32,
    /// DCB field: `directKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub direct_knockdown_disable_duration: f32,
    /// DCB field: `indirectFlinchDisableDuration` (Single)
    #[serde(default)]
    pub indirect_flinch_disable_duration: f32,
    /// DCB field: `indirectStaggerDisableDuration` (Single)
    #[serde(default)]
    pub indirect_stagger_disable_duration: f32,
    /// DCB field: `indirectKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub indirect_knockdown_disable_duration: f32,
    /// DCB field: `sustainedDeltaFlinchDisableDuration` (Single)
    #[serde(default)]
    pub sustained_delta_flinch_disable_duration: f32,
    /// DCB field: `sustainedDeltaStaggerDisableDuration` (Single)
    #[serde(default)]
    pub sustained_delta_stagger_disable_duration: f32,
    /// DCB field: `sustainedDeltaKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub sustained_delta_knockdown_disable_duration: f32,
    /// DCB field: `sustainedKnockdownDisableDuration` (Single)
    #[serde(default)]
    pub sustained_knockdown_disable_duration: f32,
    /// DCB field: `useEffortSetWhileDisablingReactions` (Boolean)
    #[serde(default)]
    pub use_effort_set_while_disabling_reactions: bool,
}

impl Pooled for SActorForceReactionBlockConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_block_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_block_config }
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
///
/// Inherits from: `SActorForceReactionEffectRangeDef` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionTwitchRangeDef {
    /// DCB field: `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// DCB field: `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// DCB field: `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// DCB field: `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// DCB field: `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// DCB field: `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// DCB field: `animationTwitch` (Class)
    #[serde(default)]
    pub animation_twitch: Option<Handle<SActorForceReactionAnimationTwitchConfig>>,
    /// DCB field: `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionTwitchRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_twitch_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_twitch_range_def }
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
    /// DCB field: `filterByForceType` (EnumChoice)
    #[serde(default)]
    pub filter_by_force_type: String,
    /// DCB field: `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `twitchRanges` (Class (array))
    #[serde(default)]
    pub twitch_ranges: Vec<Handle<SActorForceReactionTwitchRangeDef>>,
}

impl Pooled for SActorForceReactionTwitchConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_twitch_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_twitch_config_def }
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
///
/// Inherits from: `SActorForceReactionEffectRangeDef` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionFlinchRangeDef {
    /// DCB field: `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// DCB field: `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// DCB field: `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// DCB field: `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// DCB field: `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// DCB field: `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// DCB field: `animationFlinch` (Class)
    #[serde(default)]
    pub animation_flinch: Option<Handle<SActorForceReactionAnimationFlinchConfig>>,
    /// DCB field: `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionFlinchRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_flinch_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_flinch_range_def }
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
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `flinchRanges` (Class (array))
    #[serde(default)]
    pub flinch_ranges: Vec<Handle<SActorForceReactionFlinchRangeDef>>,
}

impl Pooled for SActorForceReactionFlinchConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_flinch_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_flinch_config_def }
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
///
/// Inherits from: `SActorForceReactionEffectRangeDef` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionStaggerRangeDef {
    /// DCB field: `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// DCB field: `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// DCB field: `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// DCB field: `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// DCB field: `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// DCB field: `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// DCB field: `animationStagger` (Class)
    #[serde(default)]
    pub animation_stagger: Option<Handle<SActorForceReactionAnimationStaggerConfig>>,
    /// DCB field: `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionStaggerRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_stagger_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_stagger_range_def }
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
    /// DCB field: `filterByForceType` (EnumChoice)
    #[serde(default)]
    pub filter_by_force_type: String,
    /// DCB field: `filterByHitterActor` (EnumChoice)
    #[serde(default)]
    pub filter_by_hitter_actor: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `staggerRanges` (Class (array))
    #[serde(default)]
    pub stagger_ranges: Vec<Handle<SActorForceReactionStaggerRangeDef>>,
}

impl Pooled for SActorForceReactionFilteredStaggerRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_filtered_stagger_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_filtered_stagger_range_def }
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
    /// DCB field: `reactionBlock` (Class)
    #[serde(default)]
    pub reaction_block: Option<Handle<SActorForceReactionBlockConfig>>,
    /// DCB field: `staggerFilters` (Class (array))
    #[serde(default)]
    pub stagger_filters: Vec<Handle<SActorForceReactionFilteredStaggerRangeDef>>,
}

impl Pooled for SActorForceReactionFilteredStaggerConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_filtered_stagger_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_filtered_stagger_config_def }
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
    /// DCB field: `reactionBlock` (Class)
    #[serde(default)]
    pub reaction_block: Option<Handle<SActorForceReactionBlockConfig>>,
    /// DCB field: `staggerRanges` (Class (array))
    #[serde(default)]
    pub stagger_ranges: Vec<Handle<SActorForceReactionStaggerRangeDef>>,
}

impl Pooled for SActorForceReactionUnfilteredStaggerConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_unfiltered_stagger_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_unfiltered_stagger_config_def }
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
///
/// Inherits from: `SActorForceReactionEffectRangeDef` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionKnockdownRangeDef {
    /// DCB field: `effectRangeMin` (Single)
    #[serde(default)]
    pub effect_range_min: f32,
    /// DCB field: `effectRangeMax` (Single)
    #[serde(default)]
    pub effect_range_max: f32,
    /// DCB field: `aimPunch` (Class)
    #[serde(default)]
    pub aim_punch: Option<Handle<SActorForceReactionAimPunchConfig>>,
    /// DCB field: `weaponTwitch` (Class)
    #[serde(default)]
    pub weapon_twitch: Option<Handle<SActorForceReactionWeaponTwitchConfig>>,
    /// DCB field: `headRecoil` (Class)
    #[serde(default)]
    pub head_recoil: Option<Handle<SActorForceReactionHeadRecoilConfig>>,
    /// DCB field: `FOVScale` (Class)
    #[serde(default)]
    pub fovscale: Option<Handle<SActorForceReactionFOVScaleConfig>>,
    /// DCB field: `launch` (Class)
    #[serde(default)]
    pub launch: Option<Handle<SActorForceReactionMovementLaunchConfig>>,
    /// DCB field: `blockADS` (Class)
    #[serde(default)]
    pub block_ads: Option<Handle<SActorForceReactionBlockADSConfig>>,
}

impl Pooled for SActorForceReactionKnockdownRangeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_knockdown_range_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_knockdown_range_def }
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
    /// DCB field: `reactionBlock` (Class)
    #[serde(default)]
    pub reaction_block: Option<Handle<SActorForceReactionBlockConfig>>,
    /// DCB field: `knockdownRanges` (Class (array))
    #[serde(default)]
    pub knockdown_ranges: Vec<Handle<SActorForceReactionKnockdownRangeDef>>,
}

impl Pooled for SActorForceReactionKnockdownConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_knockdown_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_knockdown_config_def }
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
    /// DCB field: `forward` (Single)
    #[serde(default)]
    pub forward: f32,
    /// DCB field: `backward` (Single)
    #[serde(default)]
    pub backward: f32,
    /// DCB field: `left` (Single)
    #[serde(default)]
    pub left: f32,
    /// DCB field: `right` (Single)
    #[serde(default)]
    pub right: f32,
}

impl Pooled for SActorForceReactionLeanAngleLimitsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_lean_angle_limits_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_lean_angle_limits_def }
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
    /// DCB field: `filter` (Class)
    #[serde(default)]
    pub filter: Option<Handle<ActorMotionStateFilter>>,
    /// DCB field: `leanAngleLimits` (Class)
    #[serde(default)]
    pub lean_angle_limits: Option<Handle<SActorForceReactionLeanAngleLimitsDef>>,
    /// DCB field: `hipVOffset` (Single)
    #[serde(default)]
    pub hip_voffset: f32,
}

impl Pooled for SActorForceReactionLeanFilterDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_lean_filter_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_lean_filter_def }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_procedural_lean_pose_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_procedural_lean_pose_list }
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
    /// DCB field: `forceSmoothDuration` (Single)
    #[serde(default)]
    pub force_smooth_duration: f32,
    /// DCB field: `minLeanForceForEffort` (Single)
    #[serde(default)]
    pub min_lean_force_for_effort: f32,
    /// DCB field: `leanFilters` (Class (array))
    #[serde(default)]
    pub lean_filters: Vec<Handle<SActorForceReactionLeanFilterDef>>,
    /// DCB field: `procLeanPoseDef` (StrongPointer)
    #[serde(default)]
    pub proc_lean_pose_def: Option<Handle<SActorForceReactionProceduralLeanPoseList>>,
}

impl Pooled for SActorForceReactionLeanConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_lean_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_lean_config_def }
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
    /// DCB field: `minAdditionalSpeed` (Single)
    #[serde(default)]
    pub min_additional_speed: f32,
    /// DCB field: `maxAdditionalSpeed` (Single)
    #[serde(default)]
    pub max_additional_speed: f32,
}

impl Pooled for SActorForceReactionStumbleConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_stumble_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_stumble_config_def }
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
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `sustainedToImpulseRange` (Class)
    #[serde(default)]
    pub sustained_to_impulse_range: Option<Handle<Range>>,
    /// DCB field: `forceSmoothingDuration` (Single)
    #[serde(default)]
    pub force_smoothing_duration: f32,
    /// DCB field: `predictionUncertaintyPercent` (Single)
    #[serde(default)]
    pub prediction_uncertainty_percent: f32,
    /// DCB field: `minForceToTriggerImpulse` (Single)
    #[serde(default)]
    pub min_force_to_trigger_impulse: f32,
}

impl Pooled for SActorForceReactionSustainedImpulseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_sustained_impulse_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_sustained_impulse_def }
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
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `externalCollisionToImpulseRange` (Class)
    #[serde(default)]
    pub external_collision_to_impulse_range: Option<Handle<Range>>,
}

impl Pooled for SActorForceReactionExternalImpulseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_external_impulse_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_external_impulse_def }
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
    /// DCB field: `allowPlayerBump` (Boolean)
    #[serde(default)]
    pub allow_player_bump: bool,
    /// DCB field: `allowPlayerBumpTwitch` (Boolean)
    #[serde(default)]
    pub allow_player_bump_twitch: bool,
    /// DCB field: `allowPlayerBumpStagger` (Boolean)
    #[serde(default)]
    pub allow_player_bump_stagger: bool,
    /// DCB field: `allowPlayerBumpKnockdown` (Boolean)
    #[serde(default)]
    pub allow_player_bump_knockdown: bool,
    /// DCB field: `allowNpcBump` (Boolean)
    #[serde(default)]
    pub allow_npc_bump: bool,
    /// DCB field: `allowNpcBumpTwitch` (Boolean)
    #[serde(default)]
    pub allow_npc_bump_twitch: bool,
    /// DCB field: `allowNpcBumpStagger` (Boolean)
    #[serde(default)]
    pub allow_npc_bump_stagger: bool,
    /// DCB field: `allowNpcBumpKnockdown` (Boolean)
    #[serde(default)]
    pub allow_npc_bump_knockdown: bool,
    /// DCB field: `playerBumpImpulseScale` (Single)
    #[serde(default)]
    pub player_bump_impulse_scale: f32,
    /// DCB field: `npcBumpImpulseScale` (Single)
    #[serde(default)]
    pub npc_bump_impulse_scale: f32,
}

impl Pooled for SActorForceReactionActorBumpDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_actor_bump_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_actor_bump_def }
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
    /// DCB field: `leanForceSmoothing` (Single)
    #[serde(default)]
    pub lean_force_smoothing: f32,
    /// DCB field: `GForceToLeanForceRange` (Class)
    #[serde(default)]
    pub gforce_to_lean_force_range: Option<Handle<Range>>,
    /// DCB field: `windToLeanForceRange` (Class)
    #[serde(default)]
    pub wind_to_lean_force_range: Option<Handle<Range>>,
    /// DCB field: `impulseTriggerDelay` (Single)
    #[serde(default)]
    pub impulse_trigger_delay: f32,
    /// DCB field: `impulseUpdateDelay` (Single)
    #[serde(default)]
    pub impulse_update_delay: f32,
    /// DCB field: `GForceImpulseConfig` (Class)
    #[serde(default)]
    pub gforce_impulse_config: Option<Handle<SActorForceReactionSustainedImpulseDef>>,
    /// DCB field: `windImpulseConfig` (Class)
    #[serde(default)]
    pub wind_impulse_config: Option<Handle<SActorForceReactionSustainedImpulseDef>>,
    /// DCB field: `sustainedForceSmoothing` (Single)
    #[serde(default)]
    pub sustained_force_smoothing: f32,
    /// DCB field: `GForceToSustainedForceRange` (Class)
    #[serde(default)]
    pub gforce_to_sustained_force_range: Option<Handle<Range>>,
    /// DCB field: `windToSustainedForceRange` (Class)
    #[serde(default)]
    pub wind_to_sustained_force_range: Option<Handle<Range>>,
}

impl Pooled for SActorForceReactionSustainedForceDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reaction_sustained_force_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reaction_sustained_force_def }
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
    /// DCB field: `stunToImpulseRate` (Single)
    #[serde(default)]
    pub stun_to_impulse_rate: f32,
    /// DCB field: `minImpulse` (Single)
    #[serde(default)]
    pub min_impulse: f32,
    /// DCB field: `maxImpulseScale` (Single)
    #[serde(default)]
    pub max_impulse_scale: f32,
    /// DCB field: `affectsProjectiles` (Boolean)
    #[serde(default)]
    pub affects_projectiles: bool,
    /// DCB field: `affectsMelee` (Boolean)
    #[serde(default)]
    pub affects_melee: bool,
    /// DCB field: `affectsPhysics` (Boolean)
    #[serde(default)]
    pub affects_physics: bool,
    /// DCB field: `affectsIndirect` (Boolean)
    #[serde(default)]
    pub affects_indirect: bool,
    /// DCB field: `affectsSustainedDelta` (Boolean)
    #[serde(default)]
    pub affects_sustained_delta: bool,
}

impl Pooled for SActorForceReactionsStunDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reactions_stun_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reactions_stun_def }
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
    /// DCB field: `minMass` (Single)
    #[serde(default)]
    pub min_mass: f32,
    /// DCB field: `gForceCutoff` (Single)
    #[serde(default)]
    pub g_force_cutoff: f32,
    /// DCB field: `gForceScale` (Single)
    #[serde(default)]
    pub g_force_scale: f32,
    /// DCB field: `externalImpulseCutoff` (Single)
    #[serde(default)]
    pub external_impulse_cutoff: f32,
    /// DCB field: `externalImpulseScale` (Single)
    #[serde(default)]
    pub external_impulse_scale: f32,
}

impl Pooled for SActorForceReactionsVehicleForceDampeningDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reactions_vehicle_force_dampening_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reactions_vehicle_force_dampening_def }
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
    /// DCB field: `physicsImpulseScale` (Single)
    #[serde(default)]
    pub physics_impulse_scale: f32,
    /// DCB field: `explosionsImpulseScale` (Single)
    #[serde(default)]
    pub explosions_impulse_scale: f32,
    /// DCB field: `meleeImpulseScale` (Single)
    #[serde(default)]
    pub melee_impulse_scale: f32,
    /// DCB field: `bulletImpulseScale` (Single)
    #[serde(default)]
    pub bullet_impulse_scale: f32,
    /// DCB field: `actorBumpImpulseConfig` (Class)
    #[serde(default)]
    pub actor_bump_impulse_config: Option<Handle<SActorForceReactionActorBumpDef>>,
    /// DCB field: `externalImpulseConfig` (Class)
    #[serde(default)]
    pub external_impulse_config: Option<Handle<SActorForceReactionExternalImpulseDef>>,
    /// DCB field: `sustainedForceConfig` (Class)
    #[serde(default)]
    pub sustained_force_config: Option<Handle<SActorForceReactionSustainedForceDef>>,
    /// DCB field: `vehicleForceDampeningConfig` (Class (array))
    #[serde(default)]
    pub vehicle_force_dampening_config: Vec<Handle<SActorForceReactionsVehicleForceDampeningDef>>,
    /// DCB field: `stunConfig` (Class)
    #[serde(default)]
    pub stun_config: Option<Handle<SActorForceReactionsStunDef>>,
    /// DCB field: `reactionLimits` (Class)
    #[serde(default)]
    pub reaction_limits: Option<Handle<SActorForceReactionLimitDef>>,
    /// DCB field: `staggerGlobalConfig` (Class)
    #[serde(default)]
    pub stagger_global_config: Option<Handle<SActorForceReactionGlobalStaggerConfig>>,
    /// DCB field: `impulseAccumulationConfig` (Class)
    #[serde(default)]
    pub impulse_accumulation_config: Option<Handle<SActorForceReactionImpulseAccumulationConfig>>,
    /// DCB field: `effectGlobalConfig` (Class)
    #[serde(default)]
    pub effect_global_config: Option<Handle<SActorForceReactionGlobalEffectConfig>>,
    /// DCB field: `filters` (Class)
    #[serde(default)]
    pub filters: Option<Handle<SActorForceReactionFilterDef>>,
    /// DCB field: `twitchConfigs` (Class (array))
    #[serde(default)]
    pub twitch_configs: Vec<Handle<SActorForceReactionTwitchConfigDef>>,
    /// DCB field: `flinchConfigs` (Class (array))
    #[serde(default)]
    pub flinch_configs: Vec<Handle<SActorForceReactionFlinchConfigDef>>,
    /// DCB field: `sustainedDeltaFlinchConfigs` (Class (array))
    #[serde(default)]
    pub sustained_delta_flinch_configs: Vec<Handle<SActorForceReactionFlinchConfigDef>>,
    /// DCB field: `directStaggerConfig` (Class)
    #[serde(default)]
    pub direct_stagger_config: Option<Handle<SActorForceReactionFilteredStaggerConfigDef>>,
    /// DCB field: `indirectStaggerConfig` (Class)
    #[serde(default)]
    pub indirect_stagger_config: Option<Handle<SActorForceReactionFilteredStaggerConfigDef>>,
    /// DCB field: `sustainedDeltaStaggerConfig` (Class)
    #[serde(default)]
    pub sustained_delta_stagger_config: Option<Handle<SActorForceReactionUnfilteredStaggerConfigDef>>,
    /// DCB field: `directKnockdownConfig` (Class)
    #[serde(default)]
    pub direct_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// DCB field: `indirectKnockdownConfig` (Class)
    #[serde(default)]
    pub indirect_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// DCB field: `sustainedDeltaKnockdownConfig` (Class)
    #[serde(default)]
    pub sustained_delta_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// DCB field: `sustainedKnockdownConfig` (Class)
    #[serde(default)]
    pub sustained_knockdown_config: Option<Handle<SActorForceReactionKnockdownConfigDef>>,
    /// DCB field: `sustainedStumbleConfig` (Class)
    #[serde(default)]
    pub sustained_stumble_config: Option<Handle<SActorForceReactionStumbleConfigDef>>,
    /// DCB field: `leanConfig` (Class)
    #[serde(default)]
    pub lean_config: Option<Handle<SActorForceReactionLeanConfigDef>>,
}

impl Pooled for SActorForceReactionsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reactions_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reactions_def }
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
    /// DCB field: `impactAccumulationTime` (Single)
    #[serde(default)]
    pub impact_accumulation_time: f32,
    /// DCB field: `impactReductionFromMass` (Single)
    #[serde(default)]
    pub impact_reduction_from_mass: f32,
    /// DCB field: `physicsImpactScalePlayer` (Single)
    #[serde(default)]
    pub physics_impact_scale_player: f32,
    /// DCB field: `physicsImpactScaleAI` (Single)
    #[serde(default)]
    pub physics_impact_scale_ai: f32,
    /// DCB field: `deathAnimationInterruptionDelay` (Single)
    #[serde(default)]
    pub death_animation_interruption_delay: f32,
    /// DCB field: `enableHitReactionsLight` (Boolean)
    #[serde(default)]
    pub enable_hit_reactions_light: bool,
    /// DCB field: `enableHitReactionsMedium` (Boolean)
    #[serde(default)]
    pub enable_hit_reactions_medium: bool,
    /// DCB field: `enableHitReactionsHeavy` (Boolean)
    #[serde(default)]
    pub enable_hit_reactions_heavy: bool,
    /// DCB field: `hitThresholdLight` (Single)
    #[serde(default)]
    pub hit_threshold_light: f32,
    /// DCB field: `hitThresholdMedium` (Single)
    #[serde(default)]
    pub hit_threshold_medium: f32,
    /// DCB field: `hitThresholdHeavy` (Single)
    #[serde(default)]
    pub hit_threshold_heavy: f32,
}

impl Pooled for SActorHitReactionsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_hit_reactions_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_hit_reactions_def }
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

/// DCB type: `SActorExternalForceResponseVibrationEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorExternalForceResponseVibrationEntry {
    /// DCB field: `shakes` (Class (array))
    #[serde(default)]
    pub shakes: Vec<Handle<CameraActorVibrationShakeConfig>>,
}

impl Pooled for SActorExternalForceResponseVibrationEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_external_force_response_vibration_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_external_force_response_vibration_entry }
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
    /// DCB field: `vibrationShakes` (Class)
    #[serde(default)]
    pub vibration_shakes: Option<Handle<SActorExternalForceResponseVibrationEntry>>,
    /// DCB field: `roleShakeMultipliers` (Class)
    #[serde(default)]
    pub role_shake_multipliers: Option<Handle<SPlayerRoleShakeMultipliers>>,
}

impl Pooled for SActorExternalForceResponseCameraShakeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_external_force_response_camera_shake_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_external_force_response_camera_shake_def }
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
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `maxLeanForward` (Single)
    #[serde(default)]
    pub max_lean_forward: f32,
    /// DCB field: `maxLeanBackward` (Single)
    #[serde(default)]
    pub max_lean_backward: f32,
    /// DCB field: `maxLeanLeft` (Single)
    #[serde(default)]
    pub max_lean_left: f32,
    /// DCB field: `maxLeanRight` (Single)
    #[serde(default)]
    pub max_lean_right: f32,
    /// DCB field: `moveHips` (Boolean)
    #[serde(default)]
    pub move_hips: bool,
    /// DCB field: `lockHands` (Boolean)
    #[serde(default)]
    pub lock_hands: bool,
    /// DCB field: `pose` (EnumChoice)
    #[serde(default)]
    pub pose: String,
}

impl Pooled for SActorForceReactionsProceduralLeanOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reactions_procedural_lean_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reactions_procedural_lean_override }
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
    /// DCB field: `procLeanOverrides` (Class (array))
    #[serde(default)]
    pub proc_lean_overrides: Vec<Handle<SActorForceReactionsProceduralLeanOverride>>,
}

impl Pooled for SActorForceReactionsPresetRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_force_reactions_preset_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_force_reactions_preset_record }
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

/// DCB type: `SActorLocomotionFidgetSeverityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFidgetSeverityParams {
    /// DCB field: `fragmentTags` (String)
    #[serde(default)]
    pub fragment_tags: String,
    /// DCB field: `severityWeight` (Single)
    #[serde(default)]
    pub severity_weight: f32,
}

impl Pooled for SActorLocomotionFidgetSeverityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_locomotion_fidget_severity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_locomotion_fidget_severity_params }
}

impl<'a> Extract<'a> for SActorLocomotionFidgetSeverityParams {
    const TYPE_NAME: &'static str = "SActorLocomotionFidgetSeverityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment_tags: inst.get_str("fragmentTags").map(String::from).unwrap_or_default(),
            severity_weight: inst.get_f32("severityWeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionFidgetStateFilteredDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFidgetStateFilteredDef {
    /// DCB field: `stateFilter` (Class)
    #[serde(default)]
    pub state_filter: Option<Handle<SLocomotionPersonalityStateFilter>>,
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `cooldownMin` (Single)
    #[serde(default)]
    pub cooldown_min: f32,
    /// DCB field: `cooldownMax` (Single)
    #[serde(default)]
    pub cooldown_max: f32,
    /// DCB field: `severities` (Class (array))
    #[serde(default)]
    pub severities: Vec<Handle<SActorLocomotionFidgetSeverityParams>>,
}

impl Pooled for SActorLocomotionFidgetStateFilteredDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_locomotion_fidget_state_filtered_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_locomotion_fidget_state_filtered_def }
}

impl<'a> Extract<'a> for SActorLocomotionFidgetStateFilteredDef {
    const TYPE_NAME: &'static str = "SActorLocomotionFidgetStateFilteredDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_filter: match inst.get("stateFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLocomotionPersonalityStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SLocomotionPersonalityStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable: inst.get_bool("enable").unwrap_or_default(),
            cooldown_min: inst.get_f32("cooldownMin").unwrap_or_default(),
            cooldown_max: inst.get_f32("cooldownMax").unwrap_or_default(),
            severities: inst.get_array("severities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorLocomotionFidgetSeverityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorLocomotionFidgetSeverityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionFidgetDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFidgetDef {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `stateDefs` (Reference (array))
    #[serde(default)]
    pub state_defs: Vec<CigGuid>,
}

impl Pooled for SActorLocomotionFidgetDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_locomotion_fidget_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_locomotion_fidget_def }
}

impl<'a> Extract<'a> for SActorLocomotionFidgetDef {
    const TYPE_NAME: &'static str = "SActorLocomotionFidgetDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            state_defs: inst.get_array("stateDefs")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionFeatureDef_Slope`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFeatureDef_Slope {
    /// DCB field: `enableStairAnimSet` (Boolean)
    #[serde(default)]
    pub enable_stair_anim_set: bool,
}

impl Pooled for SActorLocomotionFeatureDef_Slope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_locomotion_feature_def_slope }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_locomotion_feature_def_slope }
}

impl<'a> Extract<'a> for SActorLocomotionFeatureDef_Slope {
    const TYPE_NAME: &'static str = "SActorLocomotionFeatureDef_Slope";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_stair_anim_set: inst.get_bool("enableStairAnimSet").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionSubmergedCreatureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionSubmergedCreatureParams {
    /// DCB field: `allowMotionActionTurnOnSpot` (Boolean)
    #[serde(default)]
    pub allow_motion_action_turn_on_spot: bool,
}

impl Pooled for SActorLocomotionSubmergedCreatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_locomotion_submerged_creature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_locomotion_submerged_creature_params }
}

impl<'a> Extract<'a> for SActorLocomotionSubmergedCreatureParams {
    const TYPE_NAME: &'static str = "SActorLocomotionSubmergedCreatureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow_motion_action_turn_on_spot: inst.get_bool("allowMotionActionTurnOnSpot").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorProceduralHandsRecoilCurveDecayModifiersDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralHandsRecoilCurveDecayModifiersDef {
    /// DCB field: `decayTimeMultiplierModifier` (Class)
    #[serde(default)]
    pub decay_time_multiplier_modifier: Option<Handle<Vec3>>,
    /// DCB field: `decayMaxValueModifier` (Class)
    #[serde(default)]
    pub decay_max_value_modifier: Option<Handle<Vec3>>,
    /// DCB field: `decayMinScalingFactorModifier` (Class)
    #[serde(default)]
    pub decay_min_scaling_factor_modifier: Option<Handle<Vec3>>,
}

impl Pooled for SActorProceduralHandsRecoilCurveDecayModifiersDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_hands_recoil_curve_decay_modifiers_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_hands_recoil_curve_decay_modifiers_def }
}

impl<'a> Extract<'a> for SActorProceduralHandsRecoilCurveDecayModifiersDef {
    const TYPE_NAME: &'static str = "SActorProceduralHandsRecoilCurveDecayModifiersDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay_time_multiplier_modifier: match inst.get("decayTimeMultiplierModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_max_value_modifier: match inst.get("decayMaxValueModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_min_scaling_factor_modifier: match inst.get("decayMinScalingFactorModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorProceduralHandsRecoilCurveModifiersDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralHandsRecoilCurveModifiersDef {
    /// DCB field: `recoilTimeModifier` (Single)
    #[serde(default)]
    pub recoil_time_modifier: f32,
    /// DCB field: `positionModifiers` (Class)
    #[serde(default)]
    pub position_modifiers: Option<Handle<SXYZCurvesWithMaxValuesModifer>>,
    /// DCB field: `rotationModifiers` (Class)
    #[serde(default)]
    pub rotation_modifiers: Option<Handle<SXYZCurvesWithMaxValuesModifer>>,
    /// DCB field: `minDecayTimeModifier` (Single)
    #[serde(default)]
    pub min_decay_time_modifier: f32,
    /// DCB field: `maxDecayTimeModifier` (Single)
    #[serde(default)]
    pub max_decay_time_modifier: f32,
    /// DCB field: `positionDecayModifiers` (Class)
    #[serde(default)]
    pub position_decay_modifiers: Option<Handle<SActorProceduralHandsRecoilCurveDecayModifiersDef>>,
    /// DCB field: `rotationDecayModifiers` (Class)
    #[serde(default)]
    pub rotation_decay_modifiers: Option<Handle<SActorProceduralHandsRecoilCurveDecayModifiersDef>>,
}

impl Pooled for SActorProceduralHandsRecoilCurveModifiersDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_hands_recoil_curve_modifiers_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_hands_recoil_curve_modifiers_def }
}

impl<'a> Extract<'a> for SActorProceduralHandsRecoilCurveModifiersDef {
    const TYPE_NAME: &'static str = "SActorProceduralHandsRecoilCurveModifiersDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recoil_time_modifier: inst.get_f32("recoilTimeModifier").unwrap_or_default(),
            position_modifiers: match inst.get("positionModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurvesWithMaxValuesModifer>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurvesWithMaxValuesModifer>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_modifiers: match inst.get("rotationModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurvesWithMaxValuesModifer>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurvesWithMaxValuesModifer>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_decay_time_modifier: inst.get_f32("minDecayTimeModifier").unwrap_or_default(),
            max_decay_time_modifier: inst.get_f32("maxDecayTimeModifier").unwrap_or_default(),
            position_decay_modifiers: match inst.get("positionDecayModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveDecayModifiersDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveDecayModifiersDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_decay_modifiers: match inst.get("rotationDecayModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveDecayModifiersDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveDecayModifiersDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorProceduralHandsRecoilModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralHandsRecoilModifiers {
    /// DCB field: `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// DCB field: `endDecay` (Single)
    #[serde(default)]
    pub end_decay: f32,
    /// DCB field: `fireRecoilTime` (Single)
    #[serde(default)]
    pub fire_recoil_time: f32,
    /// DCB field: `fireRecoilStrengthFirst` (Single)
    #[serde(default)]
    pub fire_recoil_strength_first: f32,
    /// DCB field: `fireRecoilStrength` (Single)
    #[serde(default)]
    pub fire_recoil_strength: f32,
    /// DCB field: `angleRecoilStrength` (Single)
    #[serde(default)]
    pub angle_recoil_strength: f32,
    /// DCB field: `useRandomRotation` (Boolean)
    #[serde(default)]
    pub use_random_rotation: bool,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// DCB field: `randomness` (Single)
    #[serde(default)]
    pub randomness: f32,
    /// DCB field: `randomnessBackPush` (Single)
    #[serde(default)]
    pub randomness_back_push: f32,
    /// DCB field: `frontalOscillationRotation` (Single)
    #[serde(default)]
    pub frontal_oscillation_rotation: f32,
    /// DCB field: `frontalOscillationStrength` (Single)
    #[serde(default)]
    pub frontal_oscillation_strength: f32,
    /// DCB field: `frontalOscillationDecay` (Single)
    #[serde(default)]
    pub frontal_oscillation_decay: f32,
    /// DCB field: `frontalOscillationRandomness` (Single)
    #[serde(default)]
    pub frontal_oscillation_randomness: f32,
    /// DCB field: `resetCurveRecoilWhenApplying` (Boolean)
    #[serde(default)]
    pub reset_curve_recoil_when_applying: bool,
    /// DCB field: `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SActorProceduralHandsRecoilCurveModifiersDef>>,
}

impl Pooled for SActorProceduralHandsRecoilModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_hands_recoil_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_hands_recoil_modifiers }
}

impl<'a> Extract<'a> for SActorProceduralHandsRecoilModifiers {
    const TYPE_NAME: &'static str = "SActorProceduralHandsRecoilModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay: inst.get_f32("decay").unwrap_or_default(),
            end_decay: inst.get_f32("endDecay").unwrap_or_default(),
            fire_recoil_time: inst.get_f32("fireRecoilTime").unwrap_or_default(),
            fire_recoil_strength_first: inst.get_f32("fireRecoilStrengthFirst").unwrap_or_default(),
            fire_recoil_strength: inst.get_f32("fireRecoilStrength").unwrap_or_default(),
            angle_recoil_strength: inst.get_f32("angleRecoilStrength").unwrap_or_default(),
            use_random_rotation: inst.get_bool("useRandomRotation").unwrap_or_default(),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            randomness: inst.get_f32("randomness").unwrap_or_default(),
            randomness_back_push: inst.get_f32("randomnessBackPush").unwrap_or_default(),
            frontal_oscillation_rotation: inst.get_f32("frontalOscillationRotation").unwrap_or_default(),
            frontal_oscillation_strength: inst.get_f32("frontalOscillationStrength").unwrap_or_default(),
            frontal_oscillation_decay: inst.get_f32("frontalOscillationDecay").unwrap_or_default(),
            frontal_oscillation_randomness: inst.get_f32("frontalOscillationRandomness").unwrap_or_default(),
            reset_curve_recoil_when_applying: inst.get_bool("resetCurveRecoilWhenApplying").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveModifiersDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveModifiersDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorProceduralAimRecoilCurveNoiseModifiersDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralAimRecoilCurveNoiseModifiersDef {
    /// DCB field: `yawNoiseMaxValueModifier` (Single)
    #[serde(default)]
    pub yaw_noise_max_value_modifier: f32,
    /// DCB field: `pitchNoiseMaxValueModifier` (Single)
    #[serde(default)]
    pub pitch_noise_max_value_modifier: f32,
    /// DCB field: `rollNoiseMaxValueModifier` (Single)
    #[serde(default)]
    pub roll_noise_max_value_modifier: f32,
}

impl Pooled for SActorProceduralAimRecoilCurveNoiseModifiersDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_aim_recoil_curve_noise_modifiers_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_aim_recoil_curve_noise_modifiers_def }
}

impl<'a> Extract<'a> for SActorProceduralAimRecoilCurveNoiseModifiersDef {
    const TYPE_NAME: &'static str = "SActorProceduralAimRecoilCurveNoiseModifiersDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            yaw_noise_max_value_modifier: inst.get_f32("yawNoiseMaxValueModifier").unwrap_or_default(),
            pitch_noise_max_value_modifier: inst.get_f32("pitchNoiseMaxValueModifier").unwrap_or_default(),
            roll_noise_max_value_modifier: inst.get_f32("rollNoiseMaxValueModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorProceduralAimRecoilCurveModifiersDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralAimRecoilCurveModifiersDef {
    /// DCB field: `yawMaxDegreesModifier` (Single)
    #[serde(default)]
    pub yaw_max_degrees_modifier: f32,
    /// DCB field: `pitchMaxDegreesModifier` (Single)
    #[serde(default)]
    pub pitch_max_degrees_modifier: f32,
    /// DCB field: `rollMaxDegreesModifier` (Single)
    #[serde(default)]
    pub roll_max_degrees_modifier: f32,
    /// DCB field: `maxFireTimeModifier` (Single)
    #[serde(default)]
    pub max_fire_time_modifier: f32,
    /// DCB field: `recoilSmoothTimeModifier` (Single)
    #[serde(default)]
    pub recoil_smooth_time_modifier: f32,
    /// DCB field: `minLimitsModifier` (Class)
    #[serde(default)]
    pub min_limits_modifier: Option<Handle<Vec3>>,
    /// DCB field: `maxLimitsModifier` (Class)
    #[serde(default)]
    pub max_limits_modifier: Option<Handle<Vec3>>,
    /// DCB field: `decayStartTimeModifier` (Single)
    #[serde(default)]
    pub decay_start_time_modifier: f32,
    /// DCB field: `minDecayTimeModifier` (Single)
    #[serde(default)]
    pub min_decay_time_modifier: f32,
    /// DCB field: `maxDecayTimeModifier` (Single)
    #[serde(default)]
    pub max_decay_time_modifier: f32,
    /// DCB field: `noiseCurvesModifier` (Class)
    #[serde(default)]
    pub noise_curves_modifier: Option<Handle<SActorProceduralAimRecoilCurveNoiseModifiersDef>>,
}

impl Pooled for SActorProceduralAimRecoilCurveModifiersDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_aim_recoil_curve_modifiers_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_aim_recoil_curve_modifiers_def }
}

impl<'a> Extract<'a> for SActorProceduralAimRecoilCurveModifiersDef {
    const TYPE_NAME: &'static str = "SActorProceduralAimRecoilCurveModifiersDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            yaw_max_degrees_modifier: inst.get_f32("yawMaxDegreesModifier").unwrap_or_default(),
            pitch_max_degrees_modifier: inst.get_f32("pitchMaxDegreesModifier").unwrap_or_default(),
            roll_max_degrees_modifier: inst.get_f32("rollMaxDegreesModifier").unwrap_or_default(),
            max_fire_time_modifier: inst.get_f32("maxFireTimeModifier").unwrap_or_default(),
            recoil_smooth_time_modifier: inst.get_f32("recoilSmoothTimeModifier").unwrap_or_default(),
            min_limits_modifier: match inst.get("minLimitsModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_limits_modifier: match inst.get("maxLimitsModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_start_time_modifier: inst.get_f32("decayStartTimeModifier").unwrap_or_default(),
            min_decay_time_modifier: inst.get_f32("minDecayTimeModifier").unwrap_or_default(),
            max_decay_time_modifier: inst.get_f32("maxDecayTimeModifier").unwrap_or_default(),
            noise_curves_modifier: match inst.get("noiseCurvesModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralAimRecoilCurveNoiseModifiersDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralAimRecoilCurveNoiseModifiersDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorProceduralAimRecoilModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralAimRecoilModifiers {
    /// DCB field: `max` (Class)
    #[serde(default)]
    pub max: Option<Handle<Vec2>>,
    /// DCB field: `pull_left_percentage` (Single)
    #[serde(default)]
    pub pull_left_percentage: f32,
    /// DCB field: `shot_kick_first` (Class)
    #[serde(default)]
    pub shot_kick_first: Option<Handle<Vec2>>,
    /// DCB field: `shot_kick` (Class)
    #[serde(default)]
    pub shot_kick: Option<Handle<Vec2>>,
    /// DCB field: `random_pitch` (Single)
    #[serde(default)]
    pub random_pitch: f32,
    /// DCB field: `random_yaw` (Single)
    #[serde(default)]
    pub random_yaw: f32,
    /// DCB field: `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// DCB field: `end_decay` (Single)
    #[serde(default)]
    pub end_decay: f32,
    /// DCB field: `recoil_time` (Single)
    #[serde(default)]
    pub recoil_time: f32,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
    /// DCB field: `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SActorProceduralAimRecoilCurveModifiersDef>>,
}

impl Pooled for SActorProceduralAimRecoilModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_aim_recoil_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_aim_recoil_modifiers }
}

impl<'a> Extract<'a> for SActorProceduralAimRecoilModifiers {
    const TYPE_NAME: &'static str = "SActorProceduralAimRecoilModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max: match inst.get("max") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pull_left_percentage: inst.get_f32("pull_left_percentage").unwrap_or_default(),
            shot_kick_first: match inst.get("shot_kick_first") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shot_kick: match inst.get("shot_kick") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_pitch: inst.get_f32("random_pitch").unwrap_or_default(),
            random_yaw: inst.get_f32("random_yaw").unwrap_or_default(),
            decay: inst.get_f32("decay").unwrap_or_default(),
            end_decay: inst.get_f32("end_decay").unwrap_or_default(),
            recoil_time: inst.get_f32("recoil_time").unwrap_or_default(),
            delay: inst.get_f32("delay").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralAimRecoilCurveModifiersDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralAimRecoilCurveModifiersDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorProceduralBodyRecoilModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralBodyRecoilModifiers {
    /// DCB field: `hipsPushForce` (Single)
    #[serde(default)]
    pub hips_push_force: f32,
    /// DCB field: `hipsDampStrength` (Single)
    #[serde(default)]
    pub hips_damp_strength: f32,
    /// DCB field: `hipsDampStrengthEnd` (Single)
    #[serde(default)]
    pub hips_damp_strength_end: f32,
    /// DCB field: `spinePushForceFirst` (Single)
    #[serde(default)]
    pub spine_push_force_first: f32,
    /// DCB field: `spinePushForce` (Single)
    #[serde(default)]
    pub spine_push_force: f32,
    /// DCB field: `spineDampStrength` (Single)
    #[serde(default)]
    pub spine_damp_strength: f32,
    /// DCB field: `spineDampStrengthEnd` (Single)
    #[serde(default)]
    pub spine_damp_strength_end: f32,
}

impl Pooled for SActorProceduralBodyRecoilModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_body_recoil_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_body_recoil_modifiers }
}

impl<'a> Extract<'a> for SActorProceduralBodyRecoilModifiers {
    const TYPE_NAME: &'static str = "SActorProceduralBodyRecoilModifiers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hips_push_force: inst.get_f32("hipsPushForce").unwrap_or_default(),
            hips_damp_strength: inst.get_f32("hipsDampStrength").unwrap_or_default(),
            hips_damp_strength_end: inst.get_f32("hipsDampStrengthEnd").unwrap_or_default(),
            spine_push_force_first: inst.get_f32("spinePushForceFirst").unwrap_or_default(),
            spine_push_force: inst.get_f32("spinePushForce").unwrap_or_default(),
            spine_damp_strength: inst.get_f32("spineDampStrength").unwrap_or_default(),
            spine_damp_strength_end: inst.get_f32("spineDampStrengthEnd").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorProceduralHeadRecoilModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorProceduralHeadRecoilModifiers {
    /// DCB field: `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// DCB field: `smoothFactor` (Single)
    #[serde(default)]
    pub smooth_factor: f32,
    /// DCB field: `frequencyNoiseFactor` (Single)
    #[serde(default)]
    pub frequency_noise_factor: f32,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// DCB field: `phase` (Single)
    #[serde(default)]
    pub phase: f32,
    /// DCB field: `translation` (Class)
    #[serde(default)]
    pub translation: Option<Handle<Vec3>>,
    /// DCB field: `translationNoise` (Single)
    #[serde(default)]
    pub translation_noise: f32,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// DCB field: `rotationNoise` (Single)
    #[serde(default)]
    pub rotation_noise: f32,
    /// DCB field: `usePerlinNoise` (Boolean)
    #[serde(default)]
    pub use_perlin_noise: bool,
    /// DCB field: `referenceSpeed` (Single)
    #[serde(default)]
    pub reference_speed: f32,
    /// DCB field: `minSpeed` (Single)
    #[serde(default)]
    pub min_speed: f32,
    /// DCB field: `minScale` (Single)
    #[serde(default)]
    pub min_scale: f32,
    /// DCB field: `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
    /// DCB field: `maxScale` (Single)
    #[serde(default)]
    pub max_scale: f32,
    /// DCB field: `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SWeaponProceduralHeadRecoilCurveModifierDef>>,
}

impl Pooled for SActorProceduralHeadRecoilModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_procedural_head_recoil_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_procedural_head_recoil_modifiers }
}

impl<'a> Extract<'a> for SActorProceduralHeadRecoilModifiers {
    const TYPE_NAME: &'static str = "SActorProceduralHeadRecoilModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            smooth_factor: inst.get_f32("smoothFactor").unwrap_or_default(),
            frequency_noise_factor: inst.get_f32("frequencyNoiseFactor").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            phase: inst.get_f32("phase").unwrap_or_default(),
            translation: match inst.get("translation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            translation_noise: inst.get_f32("translationNoise").unwrap_or_default(),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_noise: inst.get_f32("rotationNoise").unwrap_or_default(),
            use_perlin_noise: inst.get_bool("usePerlinNoise").unwrap_or_default(),
            reference_speed: inst.get_f32("referenceSpeed").unwrap_or_default(),
            min_speed: inst.get_f32("minSpeed").unwrap_or_default(),
            min_scale: inst.get_f32("minScale").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
            max_scale: inst.get_f32("maxScale").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveModifierDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveModifierDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorStanceDimensionsExtraDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorStanceDimensionsExtraDef {
}

impl Pooled for SActorStanceDimensionsExtraDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_stance_dimensions_extra_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_stance_dimensions_extra_def }
}

impl<'a> Extract<'a> for SActorStanceDimensionsExtraDef {
    const TYPE_NAME: &'static str = "SActorStanceDimensionsExtraDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SAudioBreathParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAudioBreathParameters {
    /// DCB field: `BreathHeldExhaleMax` (Single)
    #[serde(default)]
    pub breath_held_exhale_max: f32,
}

impl Pooled for SAudioBreathParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saudio_breath_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saudio_breath_parameters }
}

impl<'a> Extract<'a> for SAudioBreathParameters {
    const TYPE_NAME: &'static str = "SAudioBreathParameters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            breath_held_exhale_max: inst.get_f32("BreathHeldExhaleMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorCarryConfigTagSwitch`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorCarryConfigTagSwitch {
    /// DCB field: `existingTag` (Reference)
    #[serde(default)]
    pub existing_tag: Option<CigGuid>,
    /// DCB field: `replaceWithTag` (Reference)
    #[serde(default)]
    pub replace_with_tag: Option<CigGuid>,
    /// DCB field: `onState` (EnumChoice)
    #[serde(default)]
    pub on_state: String,
}

impl Pooled for SActorCarryConfigTagSwitch {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sactor_carry_config_tag_switch }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sactor_carry_config_tag_switch }
}

impl<'a> Extract<'a> for SActorCarryConfigTagSwitch {
    const TYPE_NAME: &'static str = "SActorCarryConfigTagSwitch";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            existing_tag: inst.get("existingTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            replace_with_tag: inst.get("replaceWithTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            on_state: inst.get_str("onState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SArchetypeAssetDefBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SArchetypeAssetDefBase {
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
}

impl Pooled for SArchetypeAssetDefBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sarchetype_asset_def_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sarchetype_asset_def_base }
}

impl<'a> Extract<'a> for SArchetypeAssetDefBase {
    const TYPE_NAME: &'static str = "SArchetypeAssetDefBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SArchetypeAssetList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SArchetypeAssetList {
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `assets` (StrongPointer (array))
    #[serde(default)]
    pub assets: Vec<Handle<SArchetypeAssetDefBase>>,
}

impl Pooled for SArchetypeAssetList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sarchetype_asset_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sarchetype_asset_list }
}

impl<'a> Extract<'a> for SArchetypeAssetList {
    const TYPE_NAME: &'static str = "SArchetypeAssetList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            assets: inst.get_array("assets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SArchetypeAssetDefBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SArchetypeAssetDefBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SAssetListCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAssetListCondition {
    /// DCB field: `requiredTags` (Class)
    #[serde(default)]
    pub required_tags: Option<Handle<TagList>>,
    /// DCB field: `forbiddenTags` (Class)
    #[serde(default)]
    pub forbidden_tags: Option<Handle<TagList>>,
}

impl Pooled for SAssetListCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sasset_list_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sasset_list_condition }
}

impl<'a> Extract<'a> for SAssetListCondition {
    const TYPE_NAME: &'static str = "SAssetListCondition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            required_tags: match inst.get("requiredTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            forbidden_tags: match inst.get("forbiddenTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAutoLoadingBoxSizePrices`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAutoLoadingBoxSizePrices {
    /// DCB field: `one_eighthSCU` (Int32)
    #[serde(default)]
    pub one_eighth_scu: i32,
    /// DCB field: `one_quarterSCU` (Int32)
    #[serde(default)]
    pub one_quarter_scu: i32,
    /// DCB field: `one_halfSCU` (Int32)
    #[serde(default)]
    pub one_half_scu: i32,
    /// DCB field: `oneSCU` (Int32)
    #[serde(default)]
    pub one_scu: i32,
    /// DCB field: `twoSCU` (Int32)
    #[serde(default)]
    pub two_scu: i32,
    /// DCB field: `fourSCU` (Int32)
    #[serde(default)]
    pub four_scu: i32,
    /// DCB field: `eightSCU` (Int32)
    #[serde(default)]
    pub eight_scu: i32,
    /// DCB field: `sixteenSCU` (Int32)
    #[serde(default)]
    pub sixteen_scu: i32,
    /// DCB field: `twentyFourSCU` (Int32)
    #[serde(default)]
    pub twenty_four_scu: i32,
    /// DCB field: `thirtyTwoSCU` (Int32)
    #[serde(default)]
    pub thirty_two_scu: i32,
}

impl Pooled for SAutoLoadingBoxSizePrices {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sauto_loading_box_size_prices }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sauto_loading_box_size_prices }
}

impl<'a> Extract<'a> for SAutoLoadingBoxSizePrices {
    const TYPE_NAME: &'static str = "SAutoLoadingBoxSizePrices";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            one_eighth_scu: inst.get_i32("one_eighthSCU").unwrap_or_default(),
            one_quarter_scu: inst.get_i32("one_quarterSCU").unwrap_or_default(),
            one_half_scu: inst.get_i32("one_halfSCU").unwrap_or_default(),
            one_scu: inst.get_i32("oneSCU").unwrap_or_default(),
            two_scu: inst.get_i32("twoSCU").unwrap_or_default(),
            four_scu: inst.get_i32("fourSCU").unwrap_or_default(),
            eight_scu: inst.get_i32("eightSCU").unwrap_or_default(),
            sixteen_scu: inst.get_i32("sixteenSCU").unwrap_or_default(),
            twenty_four_scu: inst.get_i32("twentyFourSCU").unwrap_or_default(),
            thirty_two_scu: inst.get_i32("thirtyTwoSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAngleConstraint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAngleConstraint {
    /// DCB field: `forwardDirOffset` (Class)
    #[serde(default)]
    pub forward_dir_offset: Option<Handle<Vec3>>,
    /// DCB field: `angleRange` (Single)
    #[serde(default)]
    pub angle_range: f32,
    /// DCB field: `viewAngleLimit` (Single)
    #[serde(default)]
    pub view_angle_limit: f32,
}

impl Pooled for SAngleConstraint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sangle_constraint }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sangle_constraint }
}

impl<'a> Extract<'a> for SAngleConstraint {
    const TYPE_NAME: &'static str = "SAngleConstraint";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            forward_dir_offset: match inst.get("forwardDirOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angle_range: inst.get_f32("angleRange").unwrap_or_default(),
            view_angle_limit: inst.get_f32("viewAngleLimit").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAutoLoadingBoxSizeLoadingTime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAutoLoadingBoxSizeLoadingTime {
    /// DCB field: `one_eighthSCU` (Single)
    #[serde(default)]
    pub one_eighth_scu: f32,
    /// DCB field: `one_quarterSCU` (Single)
    #[serde(default)]
    pub one_quarter_scu: f32,
    /// DCB field: `one_halfSCU` (Single)
    #[serde(default)]
    pub one_half_scu: f32,
    /// DCB field: `oneSCU` (Single)
    #[serde(default)]
    pub one_scu: f32,
    /// DCB field: `twoSCU` (Single)
    #[serde(default)]
    pub two_scu: f32,
    /// DCB field: `fourSCU` (Single)
    #[serde(default)]
    pub four_scu: f32,
    /// DCB field: `eightSCU` (Single)
    #[serde(default)]
    pub eight_scu: f32,
    /// DCB field: `sixteenSCU` (Single)
    #[serde(default)]
    pub sixteen_scu: f32,
    /// DCB field: `twentyFourSCU` (Single)
    #[serde(default)]
    pub twenty_four_scu: f32,
    /// DCB field: `thirtyTwoSCU` (Single)
    #[serde(default)]
    pub thirty_two_scu: f32,
}

impl Pooled for SAutoLoadingBoxSizeLoadingTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sauto_loading_box_size_loading_time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sauto_loading_box_size_loading_time }
}

impl<'a> Extract<'a> for SAutoLoadingBoxSizeLoadingTime {
    const TYPE_NAME: &'static str = "SAutoLoadingBoxSizeLoadingTime";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            one_eighth_scu: inst.get_f32("one_eighthSCU").unwrap_or_default(),
            one_quarter_scu: inst.get_f32("one_quarterSCU").unwrap_or_default(),
            one_half_scu: inst.get_f32("one_halfSCU").unwrap_or_default(),
            one_scu: inst.get_f32("oneSCU").unwrap_or_default(),
            two_scu: inst.get_f32("twoSCU").unwrap_or_default(),
            four_scu: inst.get_f32("fourSCU").unwrap_or_default(),
            eight_scu: inst.get_f32("eightSCU").unwrap_or_default(),
            sixteen_scu: inst.get_f32("sixteenSCU").unwrap_or_default(),
            twenty_four_scu: inst.get_f32("twentyFourSCU").unwrap_or_default(),
            thirty_two_scu: inst.get_f32("thirtyTwoSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `SandboxInfractionBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInfractionBaseDef {
}

impl Pooled for SandboxInfractionBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sandbox_infraction_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sandbox_infraction_base_def }
}

impl<'a> Extract<'a> for SandboxInfractionBaseDef {
    const TYPE_NAME: &'static str = "SandboxInfractionBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SandboxTriggerBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTriggerBaseDef {
}

impl Pooled for SandboxTriggerBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sandbox_trigger_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sandbox_trigger_base_def }
}

impl<'a> Extract<'a> for SandboxTriggerBaseDef {
    const TYPE_NAME: &'static str = "SandboxTriggerBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SandboxTriggerManualParams`
///
/// Inherits from: `SandboxTriggerBaseDef` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTriggerManualParams {
    /// DCB field: `infractionTrigger` (StrongPointer)
    #[serde(default)]
    pub infraction_trigger: Option<Handle<SandboxInfractionBaseDef>>,
    /// DCB field: `triggerOnInnocentsOnly` (Boolean)
    #[serde(default)]
    pub trigger_on_innocents_only: bool,
    /// DCB field: `outcomes` (StrongPointer (array))
    #[serde(default)]
    pub outcomes: Vec<Handle<ReputationRewardBaseDef>>,
}

impl Pooled for SandboxTriggerManualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sandbox_trigger_manual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sandbox_trigger_manual_params }
}

impl<'a> Extract<'a> for SandboxTriggerManualParams {
    const TYPE_NAME: &'static str = "SandboxTriggerManualParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            infraction_trigger: match inst.get("infractionTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SandboxInfractionBaseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SandboxInfractionBaseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trigger_on_innocents_only: inst.get_bool("triggerOnInnocentsOnly").unwrap_or_default(),
            outcomes: inst.get_array("outcomes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ReputationRewardBaseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ReputationRewardBaseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SandboxTriggerRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTriggerRecord {
    /// DCB field: `triggerManualParams` (Class (array))
    #[serde(default)]
    pub trigger_manual_params: Vec<Handle<SandboxTriggerManualParams>>,
}

impl Pooled for SandboxTriggerRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sandbox_trigger_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sandbox_trigger_record }
}

impl<'a> Extract<'a> for SandboxTriggerRecord {
    const TYPE_NAME: &'static str = "SandboxTriggerRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_manual_params: inst.get_array("triggerManualParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SandboxTriggerManualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SandboxTriggerManualParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SAtmosphericCompositionParams`
///
/// Inherits from: `SAtmosphericCompositionBaseParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAtmosphericCompositionParams {
    /// DCB field: `gases` (Class (array))
    #[serde(default)]
    pub gases: Vec<Handle<SGasAtmosphereEntryParams>>,
}

impl Pooled for SAtmosphericCompositionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.satmospheric_composition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.satmospheric_composition_params }
}

impl<'a> Extract<'a> for SAtmosphericCompositionParams {
    const TYPE_NAME: &'static str = "SAtmosphericCompositionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            gases: inst.get_array("gases")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SGasAtmosphereEntryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SGasAtmosphereEntryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SAnalyticsEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAnalyticsEvent {
    /// DCB field: `eventName` (String)
    #[serde(default)]
    pub event_name: String,
    /// DCB field: `includePlayerData` (Boolean)
    #[serde(default)]
    pub include_player_data: bool,
    /// DCB field: `includeLocationData` (Boolean)
    #[serde(default)]
    pub include_location_data: bool,
    /// DCB field: `thisEntityNameField` (String)
    #[serde(default)]
    pub this_entity_name_field: String,
}

impl Pooled for SAnalyticsEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sanalytics_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sanalytics_event }
}

impl<'a> Extract<'a> for SAnalyticsEvent {
    const TYPE_NAME: &'static str = "SAnalyticsEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            event_name: inst.get_str("eventName").map(String::from).unwrap_or_default(),
            include_player_data: inst.get_bool("includePlayerData").unwrap_or_default(),
            include_location_data: inst.get_bool("includeLocationData").unwrap_or_default(),
            this_entity_name_field: inst.get_str("thisEntityNameField").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SAnalyticsEventDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAnalyticsEventDatabase {
    /// DCB field: `analyticsEvents` (Reference (array))
    #[serde(default)]
    pub analytics_events: Vec<CigGuid>,
}

impl Pooled for SAnalyticsEventDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.sanalytics_event_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.sanalytics_event_database }
}

impl<'a> Extract<'a> for SAnalyticsEventDatabase {
    const TYPE_NAME: &'static str = "SAnalyticsEventDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            analytics_events: inst.get_array("analyticsEvents")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimableGimbalModeLabels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimableGimbalModeLabels {
    /// DCB field: `aimTypeNamesFull` (Locale)
    #[serde(default)]
    pub aim_type_names_full: String,
    /// DCB field: `aimTypeNamesShort` (Locale)
    #[serde(default)]
    pub aim_type_names_short: String,
    /// DCB field: `gimbalStateNamesFull` (Locale)
    #[serde(default)]
    pub gimbal_state_names_full: String,
    /// DCB field: `gimbalStateNamesShort` (Locale)
    #[serde(default)]
    pub gimbal_state_names_short: String,
}

impl Pooled for SAimableGimbalModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_gimbal_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_gimbal_mode_labels }
}

impl<'a> Extract<'a> for SAimableGimbalModeLabels {
    const TYPE_NAME: &'static str = "SAimableGimbalModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            aim_type_names_full: inst.get_str("aimTypeNamesFull").map(String::from).unwrap_or_default(),
            aim_type_names_short: inst.get_str("aimTypeNamesShort").map(String::from).unwrap_or_default(),
            gimbal_state_names_full: inst.get_str("gimbalStateNamesFull").map(String::from).unwrap_or_default(),
            gimbal_state_names_short: inst.get_str("gimbalStateNamesShort").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimableSubTargetingStickiness`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimableSubTargetingStickiness {
    /// DCB field: `enabledByDefault` (Boolean)
    #[serde(default)]
    pub enabled_by_default: bool,
    /// DCB field: `enabledInAds` (Boolean)
    #[serde(default)]
    pub enabled_in_ads: bool,
    /// DCB field: `forcedRelativeMinAngleSizeRatio` (Single)
    #[serde(default)]
    pub forced_relative_min_angle_size_ratio: f32,
    /// DCB field: `minAngleSize` (Single)
    #[serde(default)]
    pub min_angle_size: f32,
    /// DCB field: `maxAngleSize` (Single)
    #[serde(default)]
    pub max_angle_size: f32,
}

impl Pooled for SAimableSubTargetingStickiness {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_sub_targeting_stickiness }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_sub_targeting_stickiness }
}

impl<'a> Extract<'a> for SAimableSubTargetingStickiness {
    const TYPE_NAME: &'static str = "SAimableSubTargetingStickiness";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled_by_default: inst.get_bool("enabledByDefault").unwrap_or_default(),
            enabled_in_ads: inst.get_bool("enabledInAds").unwrap_or_default(),
            forced_relative_min_angle_size_ratio: inst.get_f32("forcedRelativeMinAngleSizeRatio").unwrap_or_default(),
            min_angle_size: inst.get_f32("minAngleSize").unwrap_or_default(),
            max_angle_size: inst.get_f32("maxAngleSize").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimablePipAuto`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimablePipAuto {
    /// DCB field: `enabledByDefault` (Boolean)
    #[serde(default)]
    pub enabled_by_default: bool,
    /// DCB field: `enabledOnGamepads` (Boolean)
    #[serde(default)]
    pub enabled_on_gamepads: bool,
    /// DCB field: `outerTrackingAngle` (Single)
    #[serde(default)]
    pub outer_tracking_angle: f32,
    /// DCB field: `innerTrackingAngleRatio` (Single)
    #[serde(default)]
    pub inner_tracking_angle_ratio: f32,
}

impl Pooled for SAimablePipAuto {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_pip_auto }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_pip_auto }
}

impl<'a> Extract<'a> for SAimablePipAuto {
    const TYPE_NAME: &'static str = "SAimablePipAuto";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled_by_default: inst.get_bool("enabledByDefault").unwrap_or_default(),
            enabled_on_gamepads: inst.get_bool("enabledOnGamepads").unwrap_or_default(),
            outer_tracking_angle: inst.get_f32("outerTrackingAngle").unwrap_or_default(),
            inner_tracking_angle_ratio: inst.get_f32("innerTrackingAngleRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimablePipAiming`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimablePipAiming {
    /// DCB field: `allowAimAssists` (Boolean)
    #[serde(default)]
    pub allow_aim_assists: bool,
    /// DCB field: `pipAuto` (Class)
    #[serde(default)]
    pub pip_auto: Option<Handle<SAimablePipAuto>>,
}

impl Pooled for SAimablePipAiming {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_pip_aiming }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_pip_aiming }
}

impl<'a> Extract<'a> for SAimablePipAiming {
    const TYPE_NAME: &'static str = "SAimablePipAiming";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            allow_aim_assists: inst.get_bool("allowAimAssists").unwrap_or_default(),
            pip_auto: match inst.get("pipAuto") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimablePipAuto>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimablePipAuto>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAimableTargetPainting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimableTargetPainting {
    /// DCB field: `trackingAngleMin` (Single)
    #[serde(default)]
    pub tracking_angle_min: f32,
    /// DCB field: `trackingAngleBuffer` (Single)
    #[serde(default)]
    pub tracking_angle_buffer: f32,
    /// DCB field: `paintingOnlyPossibleViaAdsChange` (Boolean)
    #[serde(default)]
    pub painting_only_possible_via_ads_change: bool,
    /// DCB field: `adsDisableAimAssistForFixedMountedWeapons` (Boolean)
    #[serde(default)]
    pub ads_disable_aim_assist_for_fixed_mounted_weapons: bool,
    /// DCB field: `preferPaintingIfAvailable` (Boolean)
    #[serde(default)]
    pub prefer_painting_if_available: bool,
    /// DCB field: `subtargetingStickiness` (Class)
    #[serde(default)]
    pub subtargeting_stickiness: Option<Handle<SAimableSubTargetingStickiness>>,
}

impl Pooled for SAimableTargetPainting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_target_painting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_target_painting }
}

impl<'a> Extract<'a> for SAimableTargetPainting {
    const TYPE_NAME: &'static str = "SAimableTargetPainting";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tracking_angle_min: inst.get_f32("trackingAngleMin").unwrap_or_default(),
            tracking_angle_buffer: inst.get_f32("trackingAngleBuffer").unwrap_or_default(),
            painting_only_possible_via_ads_change: inst.get_bool("paintingOnlyPossibleViaAdsChange").unwrap_or_default(),
            ads_disable_aim_assist_for_fixed_mounted_weapons: inst.get_bool("adsDisableAimAssistForFixedMountedWeapons").unwrap_or_default(),
            prefer_painting_if_available: inst.get_bool("preferPaintingIfAvailable").unwrap_or_default(),
            subtargeting_stickiness: match inst.get("subtargetingStickiness") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimableSubTargetingStickiness>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimableSubTargetingStickiness>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAimableTargetAuto`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimableTargetAuto {
    /// DCB field: `trackingAngle` (Single)
    #[serde(default)]
    pub tracking_angle: f32,
    /// DCB field: `preferIfAvailable` (Boolean)
    #[serde(default)]
    pub prefer_if_available: bool,
}

impl Pooled for SAimableTargetAuto {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_target_auto }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_target_auto }
}

impl<'a> Extract<'a> for SAimableTargetAuto {
    const TYPE_NAME: &'static str = "SAimableTargetAuto";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tracking_angle: inst.get_f32("trackingAngle").unwrap_or_default(),
            prefer_if_available: inst.get_bool("preferIfAvailable").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimableGameModeRoleParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimableGameModeRoleParams {
    /// DCB field: `allowedAimTypes` (Boolean)
    #[serde(default)]
    pub allowed_aim_types: bool,
    /// DCB field: `allowGimbalUnlocking` (Boolean)
    #[serde(default)]
    pub allow_gimbal_unlocking: bool,
    /// DCB field: `enforceUnlockedGimbalsForAI` (Boolean)
    #[serde(default)]
    pub enforce_unlocked_gimbals_for_ai: bool,
    /// DCB field: `enforcePipAimingForAI` (Boolean)
    #[serde(default)]
    pub enforce_pip_aiming_for_ai: bool,
    /// DCB field: `enforceFixedGimbalsOnGamepads` (Boolean)
    #[serde(default)]
    pub enforce_fixed_gimbals_on_gamepads: bool,
    /// DCB field: `pipAiming` (Class)
    #[serde(default)]
    pub pip_aiming: Option<Handle<SAimablePipAiming>>,
    /// DCB field: `targetPainting` (Class)
    #[serde(default)]
    pub target_painting: Option<Handle<SAimableTargetPainting>>,
    /// DCB field: `targetAuto` (Class)
    #[serde(default)]
    pub target_auto: Option<Handle<SAimableTargetAuto>>,
}

impl Pooled for SAimableGameModeRoleParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_game_mode_role_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_game_mode_role_params }
}

impl<'a> Extract<'a> for SAimableGameModeRoleParams {
    const TYPE_NAME: &'static str = "SAimableGameModeRoleParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            allowed_aim_types: inst.get_bool("allowedAimTypes").unwrap_or_default(),
            allow_gimbal_unlocking: inst.get_bool("allowGimbalUnlocking").unwrap_or_default(),
            enforce_unlocked_gimbals_for_ai: inst.get_bool("enforceUnlockedGimbalsForAI").unwrap_or_default(),
            enforce_pip_aiming_for_ai: inst.get_bool("enforcePipAimingForAI").unwrap_or_default(),
            enforce_fixed_gimbals_on_gamepads: inst.get_bool("enforceFixedGimbalsOnGamepads").unwrap_or_default(),
            pip_aiming: match inst.get("pipAiming") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimablePipAiming>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimablePipAiming>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_painting: match inst.get("targetPainting") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimableTargetPainting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimableTargetPainting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_auto: match inst.get("targetAuto") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimableTargetAuto>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimableTargetAuto>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAimableGameModeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimableGameModeParams {
    /// DCB field: `aimableUsers` (Class)
    #[serde(default)]
    pub aimable_users: Option<Handle<SAimableGameModeRoleParams>>,
    /// DCB field: `enableRadarBasedAimAssist` (Boolean)
    #[serde(default)]
    pub enable_radar_based_aim_assist: bool,
}

impl Pooled for SAimableGameModeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_game_mode_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_game_mode_params }
}

impl<'a> Extract<'a> for SAimableGameModeParams {
    const TYPE_NAME: &'static str = "SAimableGameModeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aimable_users: match inst.get("aimableUsers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimableGameModeRoleParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimableGameModeRoleParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_radar_based_aim_assist: inst.get_bool("enableRadarBasedAimAssist").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimableControllerHudParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimableControllerHudParams {
    /// DCB field: `hudAutoGimbalTrackingMarkerAnimationTime` (Single)
    #[serde(default)]
    pub hud_auto_gimbal_tracking_marker_animation_time: f32,
    /// DCB field: `showAutoGimbalCombinedAllPIP` (Boolean)
    #[serde(default)]
    pub show_auto_gimbal_combined_all_pip: bool,
    /// DCB field: `leadPipFadingAngle` (Single)
    #[serde(default)]
    pub lead_pip_fading_angle: f32,
    /// DCB field: `leadPipFadingCurve` (Class)
    #[serde(default)]
    pub lead_pip_fading_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `lagPipFadingAngle` (Single)
    #[serde(default)]
    pub lag_pip_fading_angle: f32,
    /// DCB field: `lagPipFadingCurve` (Class)
    #[serde(default)]
    pub lag_pip_fading_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `borderOffsetAngleMin` (Single)
    #[serde(default)]
    pub border_offset_angle_min: f32,
    /// DCB field: `borderOffsetAngleMax` (Single)
    #[serde(default)]
    pub border_offset_angle_max: f32,
    /// DCB field: `crosshairShapes` (Int32)
    #[serde(default)]
    pub crosshair_shapes: i32,
    /// DCB field: `gimbalAlignmentAngle` (Single)
    #[serde(default)]
    pub gimbal_alignment_angle: f32,
    /// DCB field: `gimbalAlignmentExcludeOutOfAngle` (Boolean)
    #[serde(default)]
    pub gimbal_alignment_exclude_out_of_angle: bool,
}

impl Pooled for SAimableControllerHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saimable_controller_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saimable_controller_hud_params }
}

impl<'a> Extract<'a> for SAimableControllerHudParams {
    const TYPE_NAME: &'static str = "SAimableControllerHudParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hud_auto_gimbal_tracking_marker_animation_time: inst.get_f32("hudAutoGimbalTrackingMarkerAnimationTime").unwrap_or_default(),
            show_auto_gimbal_combined_all_pip: inst.get_bool("showAutoGimbalCombinedAllPIP").unwrap_or_default(),
            lead_pip_fading_angle: inst.get_f32("leadPipFadingAngle").unwrap_or_default(),
            lead_pip_fading_curve: match inst.get("leadPipFadingCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            lag_pip_fading_angle: inst.get_f32("lagPipFadingAngle").unwrap_or_default(),
            lag_pip_fading_curve: match inst.get("lagPipFadingCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            border_offset_angle_min: inst.get_f32("borderOffsetAngleMin").unwrap_or_default(),
            border_offset_angle_max: inst.get_f32("borderOffsetAngleMax").unwrap_or_default(),
            crosshair_shapes: inst.get_i32("crosshairShapes").unwrap_or_default(),
            gimbal_alignment_angle: inst.get_f32("gimbalAlignmentAngle").unwrap_or_default(),
            gimbal_alignment_exclude_out_of_angle: inst.get_bool("gimbalAlignmentExcludeOutOfAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimRecoilModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimRecoilModifier {
    /// DCB field: `maxMultiplier` (Class)
    #[serde(default)]
    pub max_multiplier: Option<Handle<Vec2>>,
    /// DCB field: `shotKickFirstMultiplier` (Class)
    #[serde(default)]
    pub shot_kick_first_multiplier: Option<Handle<Vec2>>,
    /// DCB field: `shotKickMultiplier` (Class)
    #[serde(default)]
    pub shot_kick_multiplier: Option<Handle<Vec2>>,
    /// DCB field: `randomPitchMultiplier` (Single)
    #[serde(default)]
    pub random_pitch_multiplier: f32,
    /// DCB field: `randomYawMultiplier` (Single)
    #[serde(default)]
    pub random_yaw_multiplier: f32,
    /// DCB field: `decayMultiplier` (Single)
    #[serde(default)]
    pub decay_multiplier: f32,
    /// DCB field: `endDecayMultiplier` (Single)
    #[serde(default)]
    pub end_decay_multiplier: f32,
    /// DCB field: `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SActorProceduralAimRecoilCurveModifiersDef>>,
}

impl Pooled for SAimRecoilModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saim_recoil_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saim_recoil_modifier }
}

impl<'a> Extract<'a> for SAimRecoilModifier {
    const TYPE_NAME: &'static str = "SAimRecoilModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_multiplier: match inst.get("maxMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shot_kick_first_multiplier: match inst.get("shotKickFirstMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shot_kick_multiplier: match inst.get("shotKickMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_pitch_multiplier: inst.get_f32("randomPitchMultiplier").unwrap_or_default(),
            random_yaw_multiplier: inst.get_f32("randomYawMultiplier").unwrap_or_default(),
            decay_multiplier: inst.get_f32("decayMultiplier").unwrap_or_default(),
            end_decay_multiplier: inst.get_f32("endDecayMultiplier").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralAimRecoilCurveModifiersDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralAimRecoilCurveModifiersDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAimModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimModifier {
    /// DCB field: `zoomScale` (Single)
    #[serde(default)]
    pub zoom_scale: f32,
    /// DCB field: `secondZoomScale` (Single)
    #[serde(default)]
    pub second_zoom_scale: f32,
    /// DCB field: `zoomTimeScale` (Single)
    #[serde(default)]
    pub zoom_time_scale: f32,
    /// DCB field: `hideWeaponInADS` (Boolean)
    #[serde(default)]
    pub hide_weapon_in_ads: bool,
    /// DCB field: `fstopMultiplier` (Single)
    #[serde(default)]
    pub fstop_multiplier: f32,
}

impl Pooled for SAimModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saim_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saim_modifier }
}

impl<'a> Extract<'a> for SAimModifier {
    const TYPE_NAME: &'static str = "SAimModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            zoom_scale: inst.get_f32("zoomScale").unwrap_or_default(),
            second_zoom_scale: inst.get_f32("secondZoomScale").unwrap_or_default(),
            zoom_time_scale: inst.get_f32("zoomTimeScale").unwrap_or_default(),
            hide_weapon_in_ads: inst.get_bool("hideWeaponInADS").unwrap_or_default(),
            fstop_multiplier: inst.get_f32("fstopMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimRecoilNoiseCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimRecoilNoiseCurves {
    /// DCB field: `yawNoiseMaxValue` (Single)
    #[serde(default)]
    pub yaw_noise_max_value: f32,
    /// DCB field: `pitchNoiseMaxValue` (Single)
    #[serde(default)]
    pub pitch_noise_max_value: f32,
    /// DCB field: `rollNoiseMaxValue` (Single)
    #[serde(default)]
    pub roll_noise_max_value: f32,
    /// DCB field: `yawPitchRollNoiseCurves` (StrongPointer)
    #[serde(default)]
    pub yaw_pitch_roll_noise_curves: Option<Handle<SYawPitchRollCurves>>,
}

impl Pooled for SAimRecoilNoiseCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.saim_recoil_noise_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.saim_recoil_noise_curves }
}

impl<'a> Extract<'a> for SAimRecoilNoiseCurves {
    const TYPE_NAME: &'static str = "SAimRecoilNoiseCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            yaw_noise_max_value: inst.get_f32("yawNoiseMaxValue").unwrap_or_default(),
            pitch_noise_max_value: inst.get_f32("pitchNoiseMaxValue").unwrap_or_default(),
            roll_noise_max_value: inst.get_f32("rollNoiseMaxValue").unwrap_or_default(),
            yaw_pitch_roll_noise_curves: match inst.get("yawPitchRollNoiseCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SYawPitchRollCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SYawPitchRollCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAmplitudeFreqencyDecayCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAmplitudeFreqencyDecayCurves {
    /// DCB field: `frequencyDecayCurve` (Class)
    #[serde(default)]
    pub frequency_decay_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `amplitudeDecayCurve` (Class)
    #[serde(default)]
    pub amplitude_decay_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SAmplitudeFreqencyDecayCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sa.samplitude_freqency_decay_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sa.samplitude_freqency_decay_curves }
}

impl<'a> Extract<'a> for SAmplitudeFreqencyDecayCurves {
    const TYPE_NAME: &'static str = "SAmplitudeFreqencyDecayCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequency_decay_curve: match inst.get("frequencyDecayCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            amplitude_decay_curve: match inst.get("amplitudeDecayCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

