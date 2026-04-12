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

/// DCB type: `VibrationTypeData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationTypeData {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `fallOffTime` (Single)
    #[serde(default)]
    pub fall_off_time: f32,
}

impl Pooled for VibrationTypeData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.vibration_type_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.vibration_type_data }
}

impl<'a> Extract<'a> for VibrationTypeData {
    const TYPE_NAME: &'static str = "VibrationTypeData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            fall_off_time: inst.get_f32("fallOffTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `VibrationAudioEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationAudioEntry {
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `vibrationRtpcs` (Class (array))
    #[serde(default)]
    pub vibration_rtpcs: Vec<Handle<AudioRtpcWithBehaviour>>,
    /// DCB field: `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `jostleEvent` (Class)
    #[serde(default)]
    pub jostle_event: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `jostleCooldown` (Single)
    #[serde(default)]
    pub jostle_cooldown: f32,
    /// DCB field: `jostleThreshold` (Single)
    #[serde(default)]
    pub jostle_threshold: f32,
    /// DCB field: `jostleRtpc` (Class)
    #[serde(default)]
    pub jostle_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `usedVibrationTypes` (EnumChoice (array))
    #[serde(default)]
    pub used_vibration_types: Vec<String>,
    /// DCB field: `vibrationInputModifiers` (Single)
    #[serde(default)]
    pub vibration_input_modifiers: f32,
    /// DCB field: `calculationType` (EnumChoice)
    #[serde(default)]
    pub calculation_type: String,
}

impl Pooled for VibrationAudioEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.vibration_audio_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.vibration_audio_entry }
}

impl<'a> Extract<'a> for VibrationAudioEntry {
    const TYPE_NAME: &'static str = "VibrationAudioEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            vibration_rtpcs: inst.get_array("vibrationRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpcWithBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioRtpcWithBehaviour>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            loop_start: match inst.get("loopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loop_stop: match inst.get("loopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jostle_event: match inst.get("jostleEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jostle_cooldown: inst.get_f32("jostleCooldown").unwrap_or_default(),
            jostle_threshold: inst.get_f32("jostleThreshold").unwrap_or_default(),
            jostle_rtpc: match inst.get("jostleRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            used_vibration_types: inst.get_array("usedVibrationTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            vibration_input_modifiers: inst.get_f32("vibrationInputModifiers").unwrap_or_default(),
            calculation_type: inst.get_str("calculationType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VibrationAudioPointDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationAudioPointDef {
    /// DCB field: `vibrationAudioEntries` (Class (array))
    #[serde(default)]
    pub vibration_audio_entries: Vec<Handle<VibrationAudioEntry>>,
    /// DCB field: `customFalloff` (Single)
    #[serde(default)]
    pub custom_falloff: f32,
}

impl Pooled for VibrationAudioPointDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.vibration_audio_point_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.vibration_audio_point_def }
}

impl<'a> Extract<'a> for VibrationAudioPointDef {
    const TYPE_NAME: &'static str = "VibrationAudioPointDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vibration_audio_entries: inst.get_array("vibrationAudioEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<VibrationAudioEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<VibrationAudioEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            custom_falloff: inst.get_f32("customFalloff").unwrap_or_default(),
        }
    }
}

/// DCB type: `VirtualCursorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCursorParams {
    /// DCB field: `accelerationEnabled` (Boolean)
    #[serde(default)]
    pub acceleration_enabled: bool,
    /// DCB field: `accelerationMax` (Single)
    #[serde(default)]
    pub acceleration_max: f32,
    /// DCB field: `accelerationRate` (Single)
    #[serde(default)]
    pub acceleration_rate: f32,
    /// DCB field: `accelerationDeadZone` (Single)
    #[serde(default)]
    pub acceleration_dead_zone: f32,
    /// DCB field: `sensitivity` (Single)
    #[serde(default)]
    pub sensitivity: f32,
    /// DCB field: `sensitivityPower` (Single)
    #[serde(default)]
    pub sensitivity_power: f32,
    /// DCB field: `deadZone` (Single)
    #[serde(default)]
    pub dead_zone: f32,
    /// DCB field: `consoleCursorEnable` (Boolean)
    #[serde(default)]
    pub console_cursor_enable: bool,
    /// DCB field: `consoleCursorScale` (Single)
    #[serde(default)]
    pub console_cursor_scale: f32,
}

impl Pooled for VirtualCursorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.virtual_cursor_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.virtual_cursor_params }
}

impl<'a> Extract<'a> for VirtualCursorParams {
    const TYPE_NAME: &'static str = "VirtualCursorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            acceleration_enabled: inst.get_bool("accelerationEnabled").unwrap_or_default(),
            acceleration_max: inst.get_f32("accelerationMax").unwrap_or_default(),
            acceleration_rate: inst.get_f32("accelerationRate").unwrap_or_default(),
            acceleration_dead_zone: inst.get_f32("accelerationDeadZone").unwrap_or_default(),
            sensitivity: inst.get_f32("sensitivity").unwrap_or_default(),
            sensitivity_power: inst.get_f32("sensitivityPower").unwrap_or_default(),
            dead_zone: inst.get_f32("deadZone").unwrap_or_default(),
            console_cursor_enable: inst.get_bool("consoleCursorEnable").unwrap_or_default(),
            console_cursor_scale: inst.get_f32("consoleCursorScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `VirtualCursorHoverFrictionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCursorHoverFrictionParams {
    /// DCB field: `maxFriction` (Single)
    #[serde(default)]
    pub max_friction: f32,
    /// DCB field: `elementSizeTreshold` (Single)
    #[serde(default)]
    pub element_size_treshold: f32,
}

impl Pooled for VirtualCursorHoverFrictionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.virtual_cursor_hover_friction_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.virtual_cursor_hover_friction_params }
}

impl<'a> Extract<'a> for VirtualCursorHoverFrictionParams {
    const TYPE_NAME: &'static str = "VirtualCursorHoverFrictionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_friction: inst.get_f32("maxFriction").unwrap_or_default(),
            element_size_treshold: inst.get_f32("elementSizeTreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `VirtualCursorWheelParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCursorWheelParams {
    /// DCB field: `speed` (Single)
    #[serde(default)]
    pub speed: f32,
    /// DCB field: `timeMax` (Single)
    #[serde(default)]
    pub time_max: f32,
    /// DCB field: `timeMultiplier` (Single)
    #[serde(default)]
    pub time_multiplier: f32,
    /// DCB field: `linearTime` (Single)
    #[serde(default)]
    pub linear_time: f32,
    /// DCB field: `secondStepDelay` (Single)
    #[serde(default)]
    pub second_step_delay: f32,
    /// DCB field: `deadZone` (Single)
    #[serde(default)]
    pub dead_zone: f32,
}

impl Pooled for VirtualCursorWheelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.virtual_cursor_wheel_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.virtual_cursor_wheel_params }
}

impl<'a> Extract<'a> for VirtualCursorWheelParams {
    const TYPE_NAME: &'static str = "VirtualCursorWheelParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            speed: inst.get_f32("speed").unwrap_or_default(),
            time_max: inst.get_f32("timeMax").unwrap_or_default(),
            time_multiplier: inst.get_f32("timeMultiplier").unwrap_or_default(),
            linear_time: inst.get_f32("linearTime").unwrap_or_default(),
            second_step_delay: inst.get_f32("secondStepDelay").unwrap_or_default(),
            dead_zone: inst.get_f32("deadZone").unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorHUD_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorHUD_Config {
    /// DCB field: `controlHintsDef` (Reference)
    #[serde(default)]
    pub control_hints_def: Option<CigGuid>,
}

impl Pooled for VisorHUD_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.visor_hud_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.visor_hud_config }
}

impl<'a> Extract<'a> for VisorHUD_Config {
    const TYPE_NAME: &'static str = "VisorHUD_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            control_hints_def: inst.get("controlHintsDef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `Visor_ControlHintsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visor_ControlHintsConfig {
    /// DCB field: `controlHintsPU` (Class)
    #[serde(default)]
    pub control_hints_pu: Option<Handle<ControlHintGameModeRecords>>,
    /// DCB field: `controlHintsSQ42` (Class)
    #[serde(default)]
    pub control_hints_sq42: Option<Handle<ControlHintGameModeRecords>>,
}

impl Pooled for Visor_ControlHintsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.visor_control_hints_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.visor_control_hints_config }
}

impl<'a> Extract<'a> for Visor_ControlHintsConfig {
    const TYPE_NAME: &'static str = "Visor_ControlHintsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            control_hints_pu: match inst.get("controlHintsPU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintGameModeRecords>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintGameModeRecords>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            control_hints_sq42: match inst.get("controlHintsSQ42") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintGameModeRecords>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintGameModeRecords>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VideoComms`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoComms {
    /// DCB field: `filename169` (String)
    #[serde(default)]
    pub filename169: String,
    /// DCB field: `filename43` (String)
    #[serde(default)]
    pub filename43: String,
}

impl Pooled for VideoComms {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.video_comms }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.video_comms }
}

impl<'a> Extract<'a> for VideoComms {
    const TYPE_NAME: &'static str = "VideoComms";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filename169: inst.get_str("filename169").map(String::from).unwrap_or_default(),
            filename43: inst.get_str("filename43").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VideoCommsShaderParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCommsShaderParams {
    /// DCB field: `fadeInSplineHighTech` (Class)
    #[serde(default)]
    pub fade_in_spline_high_tech: Option<Handle<BezierCurve>>,
    /// DCB field: `fadeOutSplineHighTech` (Class)
    #[serde(default)]
    pub fade_out_spline_high_tech: Option<Handle<BezierCurve>>,
    /// DCB field: `switchCommsSplineHighTech` (Class)
    #[serde(default)]
    pub switch_comms_spline_high_tech: Option<Handle<BezierCurve>>,
    /// DCB field: `fadeInSplineLowTech` (Class)
    #[serde(default)]
    pub fade_in_spline_low_tech: Option<Handle<BezierCurve>>,
    /// DCB field: `fadeOutSplineLowTech` (Class)
    #[serde(default)]
    pub fade_out_spline_low_tech: Option<Handle<BezierCurve>>,
    /// DCB field: `switchCommsSplineLowTech` (Class)
    #[serde(default)]
    pub switch_comms_spline_low_tech: Option<Handle<BezierCurve>>,
    /// DCB field: `lowTechMaterial` (String)
    #[serde(default)]
    pub low_tech_material: String,
    /// DCB field: `highTechMaterial` (String)
    #[serde(default)]
    pub high_tech_material: String,
}

impl Pooled for VideoCommsShaderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.video_comms_shader_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.video_comms_shader_params }
}

impl<'a> Extract<'a> for VideoCommsShaderParams {
    const TYPE_NAME: &'static str = "VideoCommsShaderParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fade_in_spline_high_tech: match inst.get("fadeInSplineHighTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fade_out_spline_high_tech: match inst.get("fadeOutSplineHighTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            switch_comms_spline_high_tech: match inst.get("switchCommsSplineHighTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fade_in_spline_low_tech: match inst.get("fadeInSplineLowTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fade_out_spline_low_tech: match inst.get("fadeOutSplineLowTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            switch_comms_spline_low_tech: match inst.get("switchCommsSplineLowTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            low_tech_material: inst.get_str("lowTechMaterial").map(String::from).unwrap_or_default(),
            high_tech_material: inst.get_str("highTechMaterial").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VideoCommsAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCommsAudioParams {
    /// DCB field: `lowTechInterferenceAudioRTPC` (Class)
    #[serde(default)]
    pub low_tech_interference_audio_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `highTechInterferenceAudioRTPC` (Class)
    #[serde(default)]
    pub high_tech_interference_audio_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for VideoCommsAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.video_comms_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.video_comms_audio_params }
}

impl<'a> Extract<'a> for VideoCommsAudioParams {
    const TYPE_NAME: &'static str = "VideoCommsAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            low_tech_interference_audio_rtpc: match inst.get("lowTechInterferenceAudioRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            high_tech_interference_audio_rtpc: match inst.get("highTechInterferenceAudioRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VisorLens_Layout`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorLens_Layout {
    /// DCB field: `regions` (Reference (array))
    #[serde(default)]
    pub regions: Vec<CigGuid>,
}

impl Pooled for VisorLens_Layout {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.visor_lens_layout }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.visor_lens_layout }
}

impl<'a> Extract<'a> for VisorLens_Layout {
    const TYPE_NAME: &'static str = "VisorLens_Layout";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            regions: inst.get_array("regions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorLens_Region`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorLens_Region {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `orientation` (Class)
    #[serde(default)]
    pub orientation: Option<Handle<Deg3>>,
    /// DCB field: `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// DCB field: `anchor` (Class)
    #[serde(default)]
    pub anchor: Option<Handle<Vec3>>,
    /// DCB field: `pivot` (Class)
    #[serde(default)]
    pub pivot: Option<Handle<Vec3>>,
    /// DCB field: `flexDirection` (EnumChoice)
    #[serde(default)]
    pub flex_direction: String,
    /// DCB field: `flexAxisJustification` (EnumChoice)
    #[serde(default)]
    pub flex_axis_justification: String,
    /// DCB field: `flexCrossAxisJustification` (EnumChoice)
    #[serde(default)]
    pub flex_cross_axis_justification: String,
    /// DCB field: `flexItemAlignment` (EnumChoice)
    #[serde(default)]
    pub flex_item_alignment: String,
    /// DCB field: `widgets` (Class (array))
    #[serde(default)]
    pub widgets: Vec<Handle<VisorLens_Widget>>,
}

impl Pooled for VisorLens_Region {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.visor_lens_region }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.visor_lens_region }
}

impl<'a> Extract<'a> for VisorLens_Region {
    const TYPE_NAME: &'static str = "VisorLens_Region";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Deg3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flex_direction: inst.get_str("flexDirection").map(String::from).unwrap_or_default(),
            flex_axis_justification: inst.get_str("flexAxisJustification").map(String::from).unwrap_or_default(),
            flex_cross_axis_justification: inst.get_str("flexCrossAxisJustification").map(String::from).unwrap_or_default(),
            flex_item_alignment: inst.get_str("flexItemAlignment").map(String::from).unwrap_or_default(),
            widgets: inst.get_array("widgets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<VisorLens_Widget>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<VisorLens_Widget>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorLens_Widget`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorLens_Widget {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `namespace` (String)
    #[serde(default)]
    pub namespace: String,
    /// DCB field: `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
    /// DCB field: `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// DCB field: `orientation` (Class)
    #[serde(default)]
    pub orientation: Option<Handle<Ang3>>,
    /// DCB field: `slot` (Int32)
    #[serde(default)]
    pub slot: i32,
    /// DCB field: `showTags` (Reference (array))
    #[serde(default)]
    pub show_tags: Vec<CigGuid>,
    /// DCB field: `hideTags` (Reference (array))
    #[serde(default)]
    pub hide_tags: Vec<CigGuid>,
}

impl Pooled for VisorLens_Widget {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vi.visor_lens_widget }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vi.visor_lens_widget }
}

impl<'a> Extract<'a> for VisorLens_Widget {
    const TYPE_NAME: &'static str = "VisorLens_Widget";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            namespace: inst.get_str("namespace").map(String::from).unwrap_or_default(),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            slot: inst.get_i32("slot").unwrap_or_default(),
            show_tags: inst.get_array("showTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            hide_tags: inst.get_array("hideTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

