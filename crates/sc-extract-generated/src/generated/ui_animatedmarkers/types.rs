// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-animatedmarkers`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `AnimatedMarker_Marker`
pub struct AnimatedMarker_Marker {
    /// `matrixStackID` (String (array))
    pub matrix_stack_id: Vec<String>,
    /// `timelines` (Class (array))
    pub timelines: Vec<Handle<AnimationGraph_Timeline>>,
    /// `timers` (Class (array))
    pub timers: Vec<Handle<AnimationGraph_Timer>>,
}

impl Pooled for AnimatedMarker_Marker {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_animatedmarkers.animated_marker_marker
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_animatedmarkers.animated_marker_marker
    }
}

impl<'a> Extract<'a> for AnimatedMarker_Marker {
    const TYPE_NAME: &'static str = "AnimatedMarker_Marker";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            matrix_stack_id: inst
                .get_array("matrixStackID")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            timelines: inst
                .get_array("timelines")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<AnimationGraph_Timeline>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<AnimationGraph_Timeline>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            timers: inst
                .get_array("timers")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<AnimationGraph_Timer>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<AnimationGraph_Timer>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimatedMarker`
pub struct AnimatedMarker {
    /// `name` (String)
    pub name: String,
    /// `objectName` (String)
    pub object_name: String,
    /// `doOriginOffsetScale` (Boolean)
    pub do_origin_offset_scale: bool,
    /// `originOffsetScaleMin` (Single)
    pub origin_offset_scale_min: f32,
    /// `originOffsetTargetBoundInc` (Single)
    pub origin_offset_target_bound_inc: f32,
    /// `matrixBlendRate` (Single)
    pub matrix_blend_rate: f32,
    /// `lockLostLength` (Single)
    pub lock_lost_length: f32,
    /// `additionAttachments` (UInt32)
    pub addition_attachments: u32,
    /// `markers` (Class)
    pub markers: Option<Handle<AnimatedMarker_Marker>>,
}

impl Pooled for AnimatedMarker {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_animatedmarkers.animated_marker
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_animatedmarkers.animated_marker
    }
}

impl<'a> Extract<'a> for AnimatedMarker {
    const TYPE_NAME: &'static str = "AnimatedMarker";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            object_name: inst
                .get_str("objectName")
                .map(String::from)
                .unwrap_or_default(),
            do_origin_offset_scale: inst.get_bool("doOriginOffsetScale").unwrap_or_default(),
            origin_offset_scale_min: inst.get_f32("originOffsetScaleMin").unwrap_or_default(),
            origin_offset_target_bound_inc: inst
                .get_f32("originOffsetTargetBoundInc")
                .unwrap_or_default(),
            matrix_blend_rate: inst.get_f32("matrixBlendRate").unwrap_or_default(),
            lock_lost_length: inst.get_f32("lockLostLength").unwrap_or_default(),
            addition_attachments: inst.get_u32("additionAttachments").unwrap_or_default(),
            markers: match inst.get("markers") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<AnimatedMarker_Marker>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `CombatMarker`
pub struct CombatMarker {
    /// `objectName` (String)
    pub object_name: String,
    /// `minimumScale` (Single)
    pub minimum_scale: f32,
    /// `inverseScaleMultiplier` (Single)
    pub inverse_scale_multiplier: f32,
    /// `hitAnimTotalTime` (Single)
    pub hit_anim_total_time: f32,
    /// `hitAnimationOffsetSize` (Single)
    pub hit_animation_offset_size: f32,
    /// `easeType` (EnumChoice)
    pub ease_type: InterpolationMode,
    /// `textOffset` (Single)
    pub text_offset: f32,
    /// `introAnimTime` (Single)
    pub intro_anim_time: f32,
    /// `introAnimPitchRotationFrequency` (Single)
    pub intro_anim_pitch_rotation_frequency: f32,
    /// `introAnimYawRotationFrequency` (Single)
    pub intro_anim_yaw_rotation_frequency: f32,
    /// `introAnimRollRotationFrequency` (Single)
    pub intro_anim_roll_rotation_frequency: f32,
    /// `introAnimEaseType` (EnumChoice)
    pub intro_anim_ease_type: InterpolationMode,
    /// `introStartingScale` (Single)
    pub intro_starting_scale: f32,
    /// `introStartingOffsetScale` (Single)
    pub intro_starting_offset_scale: f32,
    /// `introAnimOffset` (Single)
    pub intro_anim_offset: f32,
    /// `transitionAnimLength` (Single)
    pub transition_anim_length: f32,
    /// `transitionAnimEaseType` (EnumChoice)
    pub transition_anim_ease_type: InterpolationMode,
    /// `rotationalAnimationClamp` (Single)
    pub rotational_animation_clamp: f32,
    /// `rotationalAnimationIntegrationTime` (Single)
    pub rotational_animation_integration_time: f32,
    /// `signalLostAnimationTime` (Single)
    pub signal_lost_animation_time: f32,
    /// `signalLostAnimationPulseFrequency` (Single)
    pub signal_lost_animation_pulse_frequency: f32,
    /// `unfocusedObjectName` (String)
    pub unfocused_object_name: String,
    /// `unfocusedMarkerScale` (Single)
    pub unfocused_marker_scale: f32,
    /// `hitAnimationColor` (Class)
    pub hit_animation_color: Option<Handle<RGB>>,
    /// `hitAnimationFlickerTime` (Single)
    pub hit_animation_flicker_time: f32,
    /// `hitAnimOffsetFactor` (Single)
    pub hit_anim_offset_factor: f32,
    /// `transitionScaleCurve` (Class)
    pub transition_scale_curve: Option<Handle<BezierCurve>>,
    /// `gainedFocusAnimTotalTime` (Single)
    pub gained_focus_anim_total_time: f32,
    /// `gainedFocusAnimFlickerTime` (Single)
    pub gained_focus_anim_flicker_time: f32,
    /// `unFocusedRotationFactor` (Single)
    pub un_focused_rotation_factor: f32,
}

impl Pooled for CombatMarker {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_animatedmarkers.combat_marker
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_animatedmarkers.combat_marker
    }
}

impl<'a> Extract<'a> for CombatMarker {
    const TYPE_NAME: &'static str = "CombatMarker";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            object_name: inst
                .get_str("objectName")
                .map(String::from)
                .unwrap_or_default(),
            minimum_scale: inst.get_f32("minimumScale").unwrap_or_default(),
            inverse_scale_multiplier: inst.get_f32("inverseScaleMultiplier").unwrap_or_default(),
            hit_anim_total_time: inst.get_f32("hitAnimTotalTime").unwrap_or_default(),
            hit_animation_offset_size: inst.get_f32("hitAnimationOffsetSize").unwrap_or_default(),
            ease_type: InterpolationMode::from_dcb_str(inst.get_str("easeType").unwrap_or("")),
            text_offset: inst.get_f32("textOffset").unwrap_or_default(),
            intro_anim_time: inst.get_f32("introAnimTime").unwrap_or_default(),
            intro_anim_pitch_rotation_frequency: inst
                .get_f32("introAnimPitchRotationFrequency")
                .unwrap_or_default(),
            intro_anim_yaw_rotation_frequency: inst
                .get_f32("introAnimYawRotationFrequency")
                .unwrap_or_default(),
            intro_anim_roll_rotation_frequency: inst
                .get_f32("introAnimRollRotationFrequency")
                .unwrap_or_default(),
            intro_anim_ease_type: InterpolationMode::from_dcb_str(
                inst.get_str("introAnimEaseType").unwrap_or(""),
            ),
            intro_starting_scale: inst.get_f32("introStartingScale").unwrap_or_default(),
            intro_starting_offset_scale: inst
                .get_f32("introStartingOffsetScale")
                .unwrap_or_default(),
            intro_anim_offset: inst.get_f32("introAnimOffset").unwrap_or_default(),
            transition_anim_length: inst.get_f32("transitionAnimLength").unwrap_or_default(),
            transition_anim_ease_type: InterpolationMode::from_dcb_str(
                inst.get_str("transitionAnimEaseType").unwrap_or(""),
            ),
            rotational_animation_clamp: inst
                .get_f32("rotationalAnimationClamp")
                .unwrap_or_default(),
            rotational_animation_integration_time: inst
                .get_f32("rotationalAnimationIntegrationTime")
                .unwrap_or_default(),
            signal_lost_animation_time: inst.get_f32("signalLostAnimationTime").unwrap_or_default(),
            signal_lost_animation_pulse_frequency: inst
                .get_f32("signalLostAnimationPulseFrequency")
                .unwrap_or_default(),
            unfocused_object_name: inst
                .get_str("unfocusedObjectName")
                .map(String::from)
                .unwrap_or_default(),
            unfocused_marker_scale: inst.get_f32("unfocusedMarkerScale").unwrap_or_default(),
            hit_animation_color: match inst.get("hitAnimationColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            hit_animation_flicker_time: inst.get_f32("hitAnimationFlickerTime").unwrap_or_default(),
            hit_anim_offset_factor: inst.get_f32("hitAnimOffsetFactor").unwrap_or_default(),
            transition_scale_curve: match inst.get("transitionScaleCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            gained_focus_anim_total_time: inst
                .get_f32("gainedFocusAnimTotalTime")
                .unwrap_or_default(),
            gained_focus_anim_flicker_time: inst
                .get_f32("gainedFocusAnimFlickerTime")
                .unwrap_or_default(),
            un_focused_rotation_factor: inst.get_f32("unFocusedRotationFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimationGraph_KeyFrame`
pub struct AnimationGraph_KeyFrame {
    /// `frame` (UInt32)
    pub frame: u32,
    /// `value` (Single)
    pub value: f32,
    /// `timeModifier` (EnumChoice)
    pub time_modifier: AnimationGraph_TimeModifier,
    /// `easeType` (EnumChoice)
    pub ease_type: InterpolationMode,
}

impl Pooled for AnimationGraph_KeyFrame {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_animatedmarkers.animation_graph_key_frame
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_animatedmarkers.animation_graph_key_frame
    }
}

impl<'a> Extract<'a> for AnimationGraph_KeyFrame {
    const TYPE_NAME: &'static str = "AnimationGraph_KeyFrame";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frame: inst.get_u32("frame").unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
            time_modifier: AnimationGraph_TimeModifier::from_dcb_str(
                inst.get_str("timeModifier").unwrap_or(""),
            ),
            ease_type: InterpolationMode::from_dcb_str(inst.get_str("easeType").unwrap_or("")),
        }
    }
}

/// DCB type: `AnimationGraph_Track`
pub struct AnimationGraph_Track {
    /// `trackName` (String)
    pub track_name: String,
    /// `type` (EnumChoice)
    pub r#type: AnimationGraph_TrackType,
    /// `keyFrames` (Class (array))
    pub key_frames: Vec<Handle<AnimationGraph_KeyFrame>>,
}

impl Pooled for AnimationGraph_Track {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_animatedmarkers.animation_graph_track
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_animatedmarkers.animation_graph_track
    }
}

impl<'a> Extract<'a> for AnimationGraph_Track {
    const TYPE_NAME: &'static str = "AnimationGraph_Track";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            track_name: inst
                .get_str("trackName")
                .map(String::from)
                .unwrap_or_default(),
            r#type: AnimationGraph_TrackType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            key_frames: inst
                .get_array("keyFrames")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<AnimationGraph_KeyFrame>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<AnimationGraph_KeyFrame>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimationGraph_Timer`
pub struct AnimationGraph_Timer {
    /// `name` (String)
    pub name: String,
    /// `timeStart` (Single)
    pub time_start: f32,
    /// `timeEnd` (Single)
    pub time_end: f32,
    /// `count` (UInt32)
    pub count: u32,
}

impl Pooled for AnimationGraph_Timer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_animatedmarkers.animation_graph_timer
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_animatedmarkers.animation_graph_timer
    }
}

impl<'a> Extract<'a> for AnimationGraph_Timer {
    const TYPE_NAME: &'static str = "AnimationGraph_Timer";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            time_start: inst.get_f32("timeStart").unwrap_or_default(),
            time_end: inst.get_f32("timeEnd").unwrap_or_default(),
            count: inst.get_u32("count").unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimationGraph_Timeline`
pub struct AnimationGraph_Timeline {
    /// `timerID` (String)
    pub timer_id: String,
    /// `frameRate` (UInt32)
    pub frame_rate: u32,
    /// `tracks` (Class (array))
    pub tracks: Vec<Handle<AnimationGraph_Track>>,
}

impl Pooled for AnimationGraph_Timeline {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_animatedmarkers.animation_graph_timeline
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_animatedmarkers.animation_graph_timeline
    }
}

impl<'a> Extract<'a> for AnimationGraph_Timeline {
    const TYPE_NAME: &'static str = "AnimationGraph_Timeline";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            timer_id: inst
                .get_str("timerID")
                .map(String::from)
                .unwrap_or_default(),
            frame_rate: inst.get_u32("frameRate").unwrap_or_default(),
            tracks: inst
                .get_array("tracks")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<AnimationGraph_Track>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<AnimationGraph_Track>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
