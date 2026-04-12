// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `aiprofile`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ActivityBehaviorRequestCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBehaviorRequestCondition {
}

impl Pooled for ActivityBehaviorRequestCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.activity_behavior_request_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.activity_behavior_request_condition }
}

impl<'a> Extract<'a> for ActivityBehaviorRequestCondition {
    const TYPE_NAME: &'static str = "ActivityBehaviorRequestCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ActivityBehaviorRequest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBehaviorRequest {
    /// `requestName` (String)
    #[serde(default)]
    pub request_name: String,
    /// `condition` (StrongPointer)
    #[serde(default)]
    pub condition: Option<Handle<ActivityBehaviorRequestCondition>>,
}

impl Pooled for ActivityBehaviorRequest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.activity_behavior_request }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.activity_behavior_request }
}

impl<'a> Extract<'a> for ActivityBehaviorRequest {
    const TYPE_NAME: &'static str = "ActivityBehaviorRequest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            request_name: inst.get_str("requestName").map(String::from).unwrap_or_default(),
            condition: match inst.get("condition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActivityBehaviorRequestCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActivityBehaviorRequestCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActivityDataRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDataRecord {
    /// `activityBehaviorRequests` (StrongPointer (array))
    #[serde(default)]
    pub activity_behavior_requests: Vec<Handle<ActivityBehaviorRequest>>,
}

impl Pooled for ActivityDataRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.activity_data_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.activity_data_record }
}

impl<'a> Extract<'a> for ActivityDataRecord {
    const TYPE_NAME: &'static str = "ActivityDataRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            activity_behavior_requests: inst.get_array("activityBehaviorRequests")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActivityBehaviorRequest>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActivityBehaviorRequest>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActivityData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityData {
    /// `dataRecords` (StrongPointer (array))
    #[serde(default)]
    pub data_records: Vec<Handle<ActivityDataRecord>>,
}

impl Pooled for ActivityData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.activity_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.activity_data }
}

impl<'a> Extract<'a> for ActivityData {
    const TYPE_NAME: &'static str = "ActivityData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            data_records: inst.get_array("dataRecords")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActivityDataRecord>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActivityDataRecord>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SAIPerceptionDisturbanceTypeSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAIPerceptionDisturbanceTypeSettings {
    /// `disturbanceScoreThreshold` (Single)
    #[serde(default)]
    pub disturbance_score_threshold: f32,
    /// `disturbanceScore` (UInt32)
    #[serde(default)]
    pub disturbance_score: u32,
}

impl Pooled for SAIPerceptionDisturbanceTypeSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.saiperception_disturbance_type_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.saiperception_disturbance_type_settings }
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
    /// `curiousPerceptionBehaviourThresholds` (Single)
    #[serde(default)]
    pub curious_perception_behaviour_thresholds: f32,
    /// `threatenedPerceptionBehaviourThresholds` (Single)
    #[serde(default)]
    pub threatened_perception_behaviour_thresholds: f32,
    /// `threatenedDisturbanceThreshold` (UInt32)
    #[serde(default)]
    pub threatened_disturbance_threshold: u32,
    /// `meterDecayRate` (Single)
    #[serde(default)]
    pub meter_decay_rate: f32,
    /// `disturbanceTypeSettings` (Class)
    #[serde(default)]
    pub disturbance_type_settings: Option<Handle<SAIPerceptionDisturbanceTypeSettings>>,
}

impl Pooled for SAIPerceptionMeterSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.saiperception_meter_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.saiperception_meter_settings }
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
    /// `audioIntensityCurve` (Class)
    #[serde(default)]
    pub audio_intensity_curve: Option<Handle<BezierCurve>>,
    /// `perceptionMeterCap` (Single)
    #[serde(default)]
    pub perception_meter_cap: f32,
    /// `isCombatAudioEvent` (Boolean)
    #[serde(default)]
    pub is_combat_audio_event: bool,
    /// `isDistractionAudioEvent` (Boolean)
    #[serde(default)]
    pub is_distraction_audio_event: bool,
}

impl Pooled for SAIPerceptionAudioTypeSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.saiperception_audio_type_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.saiperception_audio_type_settings }
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
    /// `audioTypeSettings` (Class)
    #[serde(default)]
    pub audio_type_settings: Option<Handle<SAIPerceptionAudioTypeSettings>>,
}

impl Pooled for SAIPerceptionAudioSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.saiperception_audio_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.saiperception_audio_settings }
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
    /// `firstSeenPerceptionMeterDistanceCurve` (Class)
    #[serde(default)]
    pub first_seen_perception_meter_distance_curve: Option<Handle<BezierCurve>>,
    /// `visualIntensityDistanceCurve` (Class)
    #[serde(default)]
    pub visual_intensity_distance_curve: Option<Handle<BezierCurve>>,
    /// `disguisedVisualIntensityScaleDistanceCurve` (Class)
    #[serde(default)]
    pub disguised_visual_intensity_scale_distance_curve: Option<Handle<BezierCurve>>,
    /// `lightIntensityVisualIntensityScaleCurve` (Class)
    #[serde(default)]
    pub light_intensity_visual_intensity_scale_curve: Option<Handle<BezierCurve>>,
    /// `lowStanceVisualIntensityScale` (Single)
    #[serde(default)]
    pub low_stance_visual_intensity_scale: f32,
    /// `peripheralFOVVisualIntensityScale` (Single)
    #[serde(default)]
    pub peripheral_fovvisual_intensity_scale: f32,
    /// `sixthSenseFOVVisualIntensityScale` (Single)
    #[serde(default)]
    pub sixth_sense_fovvisual_intensity_scale: f32,
}

impl Pooled for SAIPerceptionVisualSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.saiperception_visual_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.saiperception_visual_settings }
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

/// DCB type: `AIPerceptionProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPerceptionProfile {
    /// `meterSettings` (Class)
    #[serde(default)]
    pub meter_settings: Option<Handle<SAIPerceptionMeterSettings>>,
    /// `audioSettings` (Class)
    #[serde(default)]
    pub audio_settings: Option<Handle<SAIPerceptionAudioSettings>>,
    /// `visualSettings` (Class)
    #[serde(default)]
    pub visual_settings: Option<Handle<SAIPerceptionVisualSettings>>,
}

impl Pooled for AIPerceptionProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aiperception_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aiperception_profile }
}

impl<'a> Extract<'a> for AIPerceptionProfile {
    const TYPE_NAME: &'static str = "AIPerceptionProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            meter_settings: match inst.get("meterSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionMeterSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionMeterSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_settings: match inst.get("audioSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionAudioSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionAudioSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            visual_settings: match inst.get("visualSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionVisualSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionVisualSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AIMercyTimerSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMercyTimerSettings {
    /// `activationThreshold` (Single)
    #[serde(default)]
    pub activation_threshold: f32,
    /// `coolDownTimeSeconds` (Single)
    #[serde(default)]
    pub cool_down_time_seconds: f32,
    /// `durationSeconds` (Single)
    #[serde(default)]
    pub duration_seconds: f32,
}

impl Pooled for AIMercyTimerSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aimercy_timer_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aimercy_timer_settings }
}

impl<'a> Extract<'a> for AIMercyTimerSettings {
    const TYPE_NAME: &'static str = "AIMercyTimerSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activation_threshold: inst.get_f32("activationThreshold").unwrap_or_default(),
            cool_down_time_seconds: inst.get_f32("coolDownTimeSeconds").unwrap_or_default(),
            duration_seconds: inst.get_f32("durationSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIVisualFieldParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIVisualFieldParams {
    /// `sightRange` (Single)
    #[serde(default)]
    pub sight_range: f32,
    /// `sixthSenseRange` (Single)
    #[serde(default)]
    pub sixth_sense_range: f32,
    /// `clampDistanceForHorizontalFOV` (Single)
    #[serde(default)]
    pub clamp_distance_for_horizontal_fov: f32,
    /// `clampDistanceForVerticalFOV` (Single)
    #[serde(default)]
    pub clamp_distance_for_vertical_fov: f32,
    /// `FOVHorizontal` (Single)
    #[serde(default)]
    pub fovhorizontal: f32,
    /// `FOVVertical` (Single)
    #[serde(default)]
    pub fovvertical: f32,
    /// `PrimaryFOVHorizontal` (Single)
    #[serde(default)]
    pub primary_fovhorizontal: f32,
    /// `PrimaryFOVVertical` (Single)
    #[serde(default)]
    pub primary_fovvertical: f32,
}

impl Pooled for AIVisualFieldParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aivisual_field_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aivisual_field_params }
}

impl<'a> Extract<'a> for AIVisualFieldParams {
    const TYPE_NAME: &'static str = "AIVisualFieldParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sight_range: inst.get_f32("sightRange").unwrap_or_default(),
            sixth_sense_range: inst.get_f32("sixthSenseRange").unwrap_or_default(),
            clamp_distance_for_horizontal_fov: inst.get_f32("clampDistanceForHorizontalFOV").unwrap_or_default(),
            clamp_distance_for_vertical_fov: inst.get_f32("clampDistanceForVerticalFOV").unwrap_or_default(),
            fovhorizontal: inst.get_f32("FOVHorizontal").unwrap_or_default(),
            fovvertical: inst.get_f32("FOVVertical").unwrap_or_default(),
            primary_fovhorizontal: inst.get_f32("PrimaryFOVHorizontal").unwrap_or_default(),
            primary_fovvertical: inst.get_f32("PrimaryFOVVertical").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIContextualVisualFieldProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContextualVisualFieldProfile {
    /// `defaultProfile` (Reference)
    #[serde(default)]
    pub default_profile: Option<CigGuid>,
    /// `awarenessLevelOverrides` (Reference)
    #[serde(default)]
    pub awareness_level_overrides: Option<CigGuid>,
}

impl Pooled for AIContextualVisualFieldProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aicontextual_visual_field_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aicontextual_visual_field_profile }
}

impl<'a> Extract<'a> for AIContextualVisualFieldProfile {
    const TYPE_NAME: &'static str = "AIContextualVisualFieldProfile";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_profile: inst.get("defaultProfile").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            awareness_level_overrides: inst.get("awarenessLevelOverrides").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AIVisualFieldProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIVisualFieldProfile {
    /// `profile` (Class)
    #[serde(default)]
    pub profile: Option<Handle<AIContextualVisualFieldProfile>>,
}

impl Pooled for AIVisualFieldProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aivisual_field_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aivisual_field_profile }
}

impl<'a> Extract<'a> for AIVisualFieldProfile {
    const TYPE_NAME: &'static str = "AIVisualFieldProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            profile: match inst.get("profile") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIContextualVisualFieldProfile>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIContextualVisualFieldProfile>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AIObservableFilterFlags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIObservableFilterFlags {
    /// `typeFlags` (EnumChoice (array))
    #[serde(default)]
    pub type_flags: Vec<String>,
    /// `statusFlags` (EnumChoice (array))
    #[serde(default)]
    pub status_flags: Vec<String>,
}

impl Pooled for AIObservableFilterFlags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aiobservable_filter_flags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aiobservable_filter_flags }
}

impl<'a> Extract<'a> for AIObservableFilterFlags {
    const TYPE_NAME: &'static str = "AIObservableFilterFlags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            type_flags: inst.get_array("typeFlags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            status_flags: inst.get_array("statusFlags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIObservableFilters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIObservableFilters {
    /// `typesAndStatus` (Reference)
    #[serde(default)]
    pub types_and_status: Option<CigGuid>,
}

impl Pooled for AIObservableFilters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aiobservable_filters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aiobservable_filters }
}

impl<'a> Extract<'a> for AIObservableFilters {
    const TYPE_NAME: &'static str = "AIObservableFilters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types_and_status: inst.get("typesAndStatus").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AIObservableFiltersProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIObservableFiltersProfile {
    /// `observableFilters` (Class (array))
    #[serde(default)]
    pub observable_filters: Vec<Handle<AIObservableFilters>>,
}

impl Pooled for AIObservableFiltersProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aiobservable_filters_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aiobservable_filters_profile }
}

impl<'a> Extract<'a> for AIObservableFiltersProfile {
    const TYPE_NAME: &'static str = "AIObservableFiltersProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            observable_filters: inst.get_array("observableFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIObservableFilters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIObservableFilters>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MovementSystemAdditionalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSystemAdditionalParams {
}

impl Pooled for MovementSystemAdditionalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.movement_system_additional_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.movement_system_additional_params }
}

impl<'a> Extract<'a> for MovementSystemAdditionalParams {
    const TYPE_NAME: &'static str = "MovementSystemAdditionalParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MovementSystemAdditionalParamsRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSystemAdditionalParamsRecord {
    /// `params` (StrongPointer)
    #[serde(default)]
    pub params: Option<Handle<MovementSystemAdditionalParams>>,
}

impl Pooled for MovementSystemAdditionalParamsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.movement_system_additional_params_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.movement_system_additional_params_record }
}

impl<'a> Extract<'a> for MovementSystemAdditionalParamsRecord {
    const TYPE_NAME: &'static str = "MovementSystemAdditionalParamsRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MovementSystemAdditionalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MovementSystemAdditionalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AISpecialRangedAttackConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISpecialRangedAttackConfig {
    /// `attackName` (String)
    #[serde(default)]
    pub attack_name: String,
    /// `maxElevationDeg` (Single)
    #[serde(default)]
    pub max_elevation_deg: f32,
    /// `verticalAttackFragmentTag` (String)
    #[serde(default)]
    pub vertical_attack_fragment_tag: String,
}

impl Pooled for AISpecialRangedAttackConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aispecial_ranged_attack_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aispecial_ranged_attack_config }
}

impl<'a> Extract<'a> for AISpecialRangedAttackConfig {
    const TYPE_NAME: &'static str = "AISpecialRangedAttackConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attack_name: inst.get_str("attackName").map(String::from).unwrap_or_default(),
            max_elevation_deg: inst.get_f32("maxElevationDeg").unwrap_or_default(),
            vertical_attack_fragment_tag: inst.get_str("verticalAttackFragmentTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AIAvailableSpecialRangedAttacksConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAvailableSpecialRangedAttacksConfig {
    /// `availableSpecialRangedAttacks` (Reference (array))
    #[serde(default)]
    pub available_special_ranged_attacks: Vec<CigGuid>,
}

impl Pooled for AIAvailableSpecialRangedAttacksConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aiavailable_special_ranged_attacks_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aiavailable_special_ranged_attacks_config }
}

impl<'a> Extract<'a> for AIAvailableSpecialRangedAttacksConfig {
    const TYPE_NAME: &'static str = "AIAvailableSpecialRangedAttacksConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            available_special_ranged_attacks: inst.get_array("availableSpecialRangedAttacks")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProfile {
    /// `character` (Class)
    #[serde(default)]
    pub character: Option<Handle<CharacterSkills>>,
    /// `seatOperator` (Class)
    #[serde(default)]
    pub seat_operator: Option<Handle<SeatOperatorSkills>>,
    /// `tacticSelectionScores` (Class)
    #[serde(default)]
    pub tactic_selection_scores: Option<Handle<TacticScoringProfile>>,
    /// `sharedTacticParams` (Class)
    #[serde(default)]
    pub shared_tactic_params: Option<Handle<SharedTacticParams>>,
    /// `shootingParams` (Class)
    #[serde(default)]
    pub shooting_params: Option<Handle<ShootingParams>>,
}

impl Pooled for AIProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aiprofile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aiprofile }
}

impl<'a> Extract<'a> for AIProfile {
    const TYPE_NAME: &'static str = "AIProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            character: match inst.get("character") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CharacterSkills>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CharacterSkills>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            seat_operator: match inst.get("seatOperator") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SeatOperatorSkills>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SeatOperatorSkills>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tactic_selection_scores: match inst.get("tacticSelectionScores") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TacticScoringProfile>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TacticScoringProfile>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shared_tactic_params: match inst.get("sharedTacticParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SharedTacticParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SharedTacticParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shooting_params: match inst.get("shootingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ShootingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ShootingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Aiming`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aiming {
    /// `accuracyConeAngle` (Single)
    #[serde(default)]
    pub accuracy_cone_angle: f32,
    /// `maxTargetAngle` (Single)
    #[serde(default)]
    pub max_target_angle: f32,
    /// `maxTargetAngleInfluence` (Single)
    #[serde(default)]
    pub max_target_angle_influence: f32,
    /// `chanceOfHittingTarget` (Single)
    #[serde(default)]
    pub chance_of_hitting_target: f32,
    /// `missileCooldownMin` (Single)
    #[serde(default)]
    pub missile_cooldown_min: f32,
    /// `missileCooldownMax` (Single)
    #[serde(default)]
    pub missile_cooldown_max: f32,
    /// `countermeasureReactionDelayMin` (Single)
    #[serde(default)]
    pub countermeasure_reaction_delay_min: f32,
    /// `countermeasureReactionDelayMax` (Single)
    #[serde(default)]
    pub countermeasure_reaction_delay_max: f32,
    /// `countermeasureCooldownMin` (Single)
    #[serde(default)]
    pub countermeasure_cooldown_min: f32,
    /// `countermeasureCooldownMax` (Single)
    #[serde(default)]
    pub countermeasure_cooldown_max: f32,
    /// `countermeasureLaunchChanceMin` (Single)
    #[serde(default)]
    pub countermeasure_launch_chance_min: f32,
    /// `countermeasureLaunchChanceIncreaseRatio` (Single)
    #[serde(default)]
    pub countermeasure_launch_chance_increase_ratio: f32,
    /// `flareBurstSizeMultiplierMin` (Single)
    #[serde(default)]
    pub flare_burst_size_multiplier_min: f32,
    /// `flareBurstSizeMultiplierMax` (Single)
    #[serde(default)]
    pub flare_burst_size_multiplier_max: f32,
    /// `defaultInAimAngleThreshold` (Single)
    #[serde(default)]
    pub default_in_aim_angle_threshold: f32,
    /// `disciplinedInAimAngleThreshold` (Single)
    #[serde(default)]
    pub disciplined_in_aim_angle_threshold: f32,
    /// `unDisciplinedInAimAngleThreshold` (Single)
    #[serde(default)]
    pub un_disciplined_in_aim_angle_threshold: f32,
}

impl Pooled for Aiming {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aiming }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aiming }
}

impl<'a> Extract<'a> for Aiming {
    const TYPE_NAME: &'static str = "Aiming";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            accuracy_cone_angle: inst.get_f32("accuracyConeAngle").unwrap_or_default(),
            max_target_angle: inst.get_f32("maxTargetAngle").unwrap_or_default(),
            max_target_angle_influence: inst.get_f32("maxTargetAngleInfluence").unwrap_or_default(),
            chance_of_hitting_target: inst.get_f32("chanceOfHittingTarget").unwrap_or_default(),
            missile_cooldown_min: inst.get_f32("missileCooldownMin").unwrap_or_default(),
            missile_cooldown_max: inst.get_f32("missileCooldownMax").unwrap_or_default(),
            countermeasure_reaction_delay_min: inst.get_f32("countermeasureReactionDelayMin").unwrap_or_default(),
            countermeasure_reaction_delay_max: inst.get_f32("countermeasureReactionDelayMax").unwrap_or_default(),
            countermeasure_cooldown_min: inst.get_f32("countermeasureCooldownMin").unwrap_or_default(),
            countermeasure_cooldown_max: inst.get_f32("countermeasureCooldownMax").unwrap_or_default(),
            countermeasure_launch_chance_min: inst.get_f32("countermeasureLaunchChanceMin").unwrap_or_default(),
            countermeasure_launch_chance_increase_ratio: inst.get_f32("countermeasureLaunchChanceIncreaseRatio").unwrap_or_default(),
            flare_burst_size_multiplier_min: inst.get_f32("flareBurstSizeMultiplierMin").unwrap_or_default(),
            flare_burst_size_multiplier_max: inst.get_f32("flareBurstSizeMultiplierMax").unwrap_or_default(),
            default_in_aim_angle_threshold: inst.get_f32("defaultInAimAngleThreshold").unwrap_or_default(),
            disciplined_in_aim_angle_threshold: inst.get_f32("disciplinedInAimAngleThreshold").unwrap_or_default(),
            un_disciplined_in_aim_angle_threshold: inst.get_f32("unDisciplinedInAimAngleThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `Burst`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Burst {
    /// `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// `minRateOfFire` (Single)
    #[serde(default)]
    pub min_rate_of_fire: f32,
    /// `minBurstLength` (Single)
    #[serde(default)]
    pub min_burst_length: f32,
    /// `minShotCount` (Single)
    #[serde(default)]
    pub min_shot_count: f32,
    /// `gapLengthMultiplier` (Single)
    #[serde(default)]
    pub gap_length_multiplier: f32,
    /// `burstVariationMultiplier` (Single)
    #[serde(default)]
    pub burst_variation_multiplier: f32,
}

impl Pooled for Burst {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.burst }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.burst }
}

impl<'a> Extract<'a> for Burst {
    const TYPE_NAME: &'static str = "Burst";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            min_rate_of_fire: inst.get_f32("minRateOfFire").unwrap_or_default(),
            min_burst_length: inst.get_f32("minBurstLength").unwrap_or_default(),
            min_shot_count: inst.get_f32("minShotCount").unwrap_or_default(),
            gap_length_multiplier: inst.get_f32("gapLengthMultiplier").unwrap_or_default(),
            burst_variation_multiplier: inst.get_f32("burstVariationMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `AITimeSinceTargetSeen`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITimeSinceTargetSeen {
    /// `decayDelayTimeSeconds` (Single)
    #[serde(default)]
    pub decay_delay_time_seconds: f32,
    /// `decayRate` (Single)
    #[serde(default)]
    pub decay_rate: f32,
    /// `capTimeSeconds` (Single)
    #[serde(default)]
    pub cap_time_seconds: f32,
    /// `accuracyOverTimeCurve` (Class)
    #[serde(default)]
    pub accuracy_over_time_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for AITimeSinceTargetSeen {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.aitime_since_target_seen }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.aitime_since_target_seen }
}

impl<'a> Extract<'a> for AITimeSinceTargetSeen {
    const TYPE_NAME: &'static str = "AITimeSinceTargetSeen";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay_delay_time_seconds: inst.get_f32("decayDelayTimeSeconds").unwrap_or_default(),
            decay_rate: inst.get_f32("decayRate").unwrap_or_default(),
            cap_time_seconds: inst.get_f32("capTimeSeconds").unwrap_or_default(),
            accuracy_over_time_curve: match inst.get("accuracyOverTimeCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CharacterAccuracyModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAccuracyModifiers {
    /// `timeSinceTargetSeen` (Class)
    #[serde(default)]
    pub time_since_target_seen: Option<Handle<AITimeSinceTargetSeen>>,
    /// `targetStanceModifier` (Single)
    #[serde(default)]
    pub target_stance_modifier: f32,
}

impl Pooled for CharacterAccuracyModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.character_accuracy_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.character_accuracy_modifiers }
}

impl<'a> Extract<'a> for CharacterAccuracyModifiers {
    const TYPE_NAME: &'static str = "CharacterAccuracyModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            time_since_target_seen: match inst.get("timeSinceTargetSeen") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AITimeSinceTargetSeen>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AITimeSinceTargetSeen>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_stance_modifier: inst.get_f32("targetStanceModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `CharacterSkills`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkills {
    /// `aiming` (Class)
    #[serde(default)]
    pub aiming: Option<Handle<Aiming>>,
    /// `modifiers` (Class)
    #[serde(default)]
    pub modifiers: Option<Handle<CharacterAccuracyModifiers>>,
}

impl Pooled for CharacterSkills {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.character_skills }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.character_skills }
}

impl<'a> Extract<'a> for CharacterSkills {
    const TYPE_NAME: &'static str = "CharacterSkills";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aiming: match inst.get("aiming") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Aiming>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Aiming>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifiers: match inst.get("modifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CharacterAccuracyModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CharacterAccuracyModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SeatOperatorSkills`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatOperatorSkills {
    /// `aiming` (Class)
    #[serde(default)]
    pub aiming: Option<Handle<Aiming>>,
    /// `burst` (Class)
    #[serde(default)]
    pub burst: Option<Handle<Burst>>,
}

impl Pooled for SeatOperatorSkills {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.seat_operator_skills }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.seat_operator_skills }
}

impl<'a> Extract<'a> for SeatOperatorSkills {
    const TYPE_NAME: &'static str = "SeatOperatorSkills";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aiming: match inst.get("aiming") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Aiming>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Aiming>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            burst: match inst.get("burst") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Burst>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Burst>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommonTargetingSameTargetScore`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTargetingSameTargetScore {
    /// `targetedByOneNPC` (Single)
    #[serde(default)]
    pub targeted_by_one_npc: f32,
    /// `targetedByTwoNPC` (Single)
    #[serde(default)]
    pub targeted_by_two_npc: f32,
    /// `targetedByThreeNPC` (Single)
    #[serde(default)]
    pub targeted_by_three_npc: f32,
    /// `targetedByFourNPC` (Single)
    #[serde(default)]
    pub targeted_by_four_npc: f32,
    /// `targetedByFiveNPC` (Single)
    #[serde(default)]
    pub targeted_by_five_npc: f32,
}

impl Pooled for CommonTargetingSameTargetScore {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.common_targeting_same_target_score }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.common_targeting_same_target_score }
}

impl<'a> Extract<'a> for CommonTargetingSameTargetScore {
    const TYPE_NAME: &'static str = "CommonTargetingSameTargetScore";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeted_by_one_npc: inst.get_f32("targetedByOneNPC").unwrap_or_default(),
            targeted_by_two_npc: inst.get_f32("targetedByTwoNPC").unwrap_or_default(),
            targeted_by_three_npc: inst.get_f32("targetedByThreeNPC").unwrap_or_default(),
            targeted_by_four_npc: inst.get_f32("targetedByFourNPC").unwrap_or_default(),
            targeted_by_five_npc: inst.get_f32("targetedByFiveNPC").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommonTargetVisibilityScore`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTargetVisibilityScore {
    /// `isVisible` (Single)
    #[serde(default)]
    pub is_visible: f32,
    /// `isNotVisible` (Single)
    #[serde(default)]
    pub is_not_visible: f32,
}

impl Pooled for CommonTargetVisibilityScore {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.common_target_visibility_score }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.common_target_visibility_score }
}

impl<'a> Extract<'a> for CommonTargetVisibilityScore {
    const TYPE_NAME: &'static str = "CommonTargetVisibilityScore";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_visible: inst.get_f32("isVisible").unwrap_or_default(),
            is_not_visible: inst.get_f32("isNotVisible").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommonCurrentTargetDistanceScore`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonCurrentTargetDistanceScore {
    /// `lowDistToTarget` (Single)
    #[serde(default)]
    pub low_dist_to_target: f32,
    /// `mediumDistToTarget` (Single)
    #[serde(default)]
    pub medium_dist_to_target: f32,
    /// `highDistToTarget` (Single)
    #[serde(default)]
    pub high_dist_to_target: f32,
}

impl Pooled for CommonCurrentTargetDistanceScore {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.common_current_target_distance_score }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.common_current_target_distance_score }
}

impl<'a> Extract<'a> for CommonCurrentTargetDistanceScore {
    const TYPE_NAME: &'static str = "CommonCurrentTargetDistanceScore";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            low_dist_to_target: inst.get_f32("lowDistToTarget").unwrap_or_default(),
            medium_dist_to_target: inst.get_f32("mediumDistToTarget").unwrap_or_default(),
            high_dist_to_target: inst.get_f32("highDistToTarget").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommonTacticScores`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTacticScores {
    /// `tacticName` (String)
    #[serde(default)]
    pub tactic_name: String,
    /// `amountOfEntitiesTargetingSameTargetScore` (Class)
    #[serde(default)]
    pub amount_of_entities_targeting_same_target_score: Option<Handle<CommonTargetingSameTargetScore>>,
    /// `isCurrentTargetVisibleScore` (Class)
    #[serde(default)]
    pub is_current_target_visible_score: Option<Handle<CommonTargetVisibilityScore>>,
    /// `currentDistanceToTargetScore` (Class)
    #[serde(default)]
    pub current_distance_to_target_score: Option<Handle<CommonCurrentTargetDistanceScore>>,
}

impl Pooled for CommonTacticScores {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.common_tactic_scores }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.common_tactic_scores }
}

impl<'a> Extract<'a> for CommonTacticScores {
    const TYPE_NAME: &'static str = "CommonTacticScores";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tactic_name: inst.get_str("tacticName").map(String::from).unwrap_or_default(),
            amount_of_entities_targeting_same_target_score: match inst.get("amountOfEntitiesTargetingSameTargetScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommonTargetingSameTargetScore>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommonTargetingSameTargetScore>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_current_target_visible_score: match inst.get("isCurrentTargetVisibleScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommonTargetVisibilityScore>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommonTargetVisibilityScore>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            current_distance_to_target_score: match inst.get("currentDistanceToTargetScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommonCurrentTargetDistanceScore>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommonCurrentTargetDistanceScore>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TacticScoringProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticScoringProfile {
    /// `tactics` (StrongPointer (array))
    #[serde(default)]
    pub tactics: Vec<Handle<CommonTacticScores>>,
}

impl Pooled for TacticScoringProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.tactic_scoring_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.tactic_scoring_profile }
}

impl<'a> Extract<'a> for TacticScoringProfile {
    const TYPE_NAME: &'static str = "TacticScoringProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tactics: inst.get_array("tactics")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommonTacticScores>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommonTacticScores>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SharedTacticParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTacticParams {
    /// `distanceToTargetThreshold` (Class)
    #[serde(default)]
    pub distance_to_target_threshold: Option<Handle<TacticPlayerDistance>>,
}

impl Pooled for SharedTacticParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.shared_tactic_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.shared_tactic_params }
}

impl<'a> Extract<'a> for SharedTacticParams {
    const TYPE_NAME: &'static str = "SharedTacticParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distance_to_target_threshold: match inst.get("distanceToTargetThreshold") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TacticPlayerDistance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TacticPlayerDistance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ShootingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShootingParams {
    /// `shortCombatDistanceShootingThreshold` (Single)
    #[serde(default)]
    pub short_combat_distance_shooting_threshold: f32,
    /// `dogfightCloseCombatDistanceThreshold` (Single)
    #[serde(default)]
    pub dogfight_close_combat_distance_threshold: f32,
    /// `undisciplinedTriggerFingerMultiplier` (Single)
    #[serde(default)]
    pub undisciplined_trigger_finger_multiplier: f32,
    /// `triggerDisciplinedMultiplier` (Single)
    #[serde(default)]
    pub trigger_disciplined_multiplier: f32,
}

impl Pooled for ShootingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.shooting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.shooting_params }
}

impl<'a> Extract<'a> for ShootingParams {
    const TYPE_NAME: &'static str = "ShootingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            short_combat_distance_shooting_threshold: inst.get_f32("shortCombatDistanceShootingThreshold").unwrap_or_default(),
            dogfight_close_combat_distance_threshold: inst.get_f32("dogfightCloseCombatDistanceThreshold").unwrap_or_default(),
            undisciplined_trigger_finger_multiplier: inst.get_f32("undisciplinedTriggerFingerMultiplier").unwrap_or_default(),
            trigger_disciplined_multiplier: inst.get_f32("triggerDisciplinedMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `TacticPlayerDistance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticPlayerDistance {
    /// `shortDistanceTreshold` (Single)
    #[serde(default)]
    pub short_distance_treshold: f32,
    /// `mediumDistanceTreshhold` (Single)
    #[serde(default)]
    pub medium_distance_treshhold: f32,
}

impl Pooled for TacticPlayerDistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.tactic_player_distance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.tactic_player_distance }
}

impl<'a> Extract<'a> for TacticPlayerDistance {
    const TYPE_NAME: &'static str = "TacticPlayerDistance";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            short_distance_treshold: inst.get_f32("shortDistanceTreshold").unwrap_or_default(),
            medium_distance_treshhold: inst.get_f32("mediumDistanceTreshhold").unwrap_or_default(),
        }
    }
}

/// DCB type: `SkillDefinitions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinitions {
    /// `descriptionTags` (Class)
    #[serde(default)]
    pub description_tags: Option<Handle<TagList>>,
    /// `traits` (Class)
    #[serde(default)]
    pub traits: Option<Handle<TagList>>,
    /// `skills` (Class (array))
    #[serde(default)]
    pub skills: Vec<Handle<Skill>>,
}

impl Pooled for SkillDefinitions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.skill_definitions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.skill_definitions }
}

impl<'a> Extract<'a> for SkillDefinitions {
    const TYPE_NAME: &'static str = "SkillDefinitions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description_tags: match inst.get("descriptionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            traits: match inst.get("traits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            skills: inst.get_array("skills")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Skill>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Skill>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Skill`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// `isSkill` (Boolean)
    #[serde(default)]
    pub is_skill: bool,
    /// `optional` (Boolean)
    #[serde(default)]
    pub optional: bool,
    /// `skillType` (EnumChoice)
    #[serde(default)]
    pub skill_type: String,
    /// `skillLevel` (Single)
    #[serde(default)]
    pub skill_level: f32,
    /// `skillTag` (Reference)
    #[serde(default)]
    pub skill_tag: Option<CigGuid>,
    /// `categoryName` (String)
    #[serde(default)]
    pub category_name: String,
    /// `skills` (Class (array))
    #[serde(default)]
    pub skills: Vec<Handle<Skill>>,
}

impl Pooled for Skill {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.skill }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.skill }
}

impl<'a> Extract<'a> for Skill {
    const TYPE_NAME: &'static str = "Skill";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            is_skill: inst.get_bool("isSkill").unwrap_or_default(),
            optional: inst.get_bool("optional").unwrap_or_default(),
            skill_type: inst.get_str("skillType").map(String::from).unwrap_or_default(),
            skill_level: inst.get_f32("skillLevel").unwrap_or_default(),
            skill_tag: inst.get("skillTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            category_name: inst.get_str("categoryName").map(String::from).unwrap_or_default(),
            skills: inst.get_array("skills")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Skill>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Skill>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatDefinitions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatDefinitions {
    /// `stats` (Class (array))
    #[serde(default)]
    pub stats: Vec<Handle<Stat>>,
}

impl Pooled for StatDefinitions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.stat_definitions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.stat_definitions }
}

impl<'a> Extract<'a> for StatDefinitions {
    const TYPE_NAME: &'static str = "StatDefinitions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stats: inst.get_array("stats")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Stat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Stat>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Stat`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    /// `statTag` (Reference)
    #[serde(default)]
    pub stat_tag: Option<CigGuid>,
    /// `minimumValue` (Single)
    #[serde(default)]
    pub minimum_value: f32,
    /// `influences` (Class (array))
    #[serde(default)]
    pub influences: Vec<Handle<StatInfluence>>,
}

impl Pooled for Stat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.stat }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.stat }
}

impl<'a> Extract<'a> for Stat {
    const TYPE_NAME: &'static str = "Stat";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stat_tag: inst.get("statTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            minimum_value: inst.get_f32("minimumValue").unwrap_or_default(),
            influences: inst.get_array("influences")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatInfluence>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatInfluence>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatInfluence`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatInfluence {
    /// `skillTag` (Reference)
    #[serde(default)]
    pub skill_tag: Option<CigGuid>,
    /// `percentage` (Int32)
    #[serde(default)]
    pub percentage: i32,
}

impl Pooled for StatInfluence {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiprofile.stat_influence }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiprofile.stat_influence }
}

impl<'a> Extract<'a> for StatInfluence {
    const TYPE_NAME: &'static str = "StatInfluence";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            skill_tag: inst.get("skillTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            percentage: inst.get_i32("percentage").unwrap_or_default(),
        }
    }
}

