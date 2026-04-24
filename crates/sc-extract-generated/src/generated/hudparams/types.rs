// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `hudparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `STargetSelectorColorHighlighting`
pub struct STargetSelectorColorHighlighting {
    /// `highlightColor` (Class)
    pub highlight_color: Option<Handle<RGB>>,
    /// `occludedAlpha` (Single)
    pub occluded_alpha: f32,
    /// `outlineOnly` (Boolean)
    pub outline_only: bool,
    /// `outlineWidth` (Single)
    pub outline_width: f32,
    /// `interferenceAmount` (Single)
    pub interference_amount: f32,
    /// `interferenceSpeed` (Single)
    pub interference_speed: f32,
    /// `interferenceTiling` (Single)
    pub interference_tiling: f32,
    /// `interferenceBrightness` (Single)
    pub interference_brightness: f32,
    /// `useHostilityColor` (Boolean)
    pub use_hostility_color: bool,
}

impl Pooled for STargetSelectorColorHighlighting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.starget_selector_color_highlighting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.starget_selector_color_highlighting }
}

impl<'a> Extract<'a> for STargetSelectorColorHighlighting {
    const TYPE_NAME: &'static str = "STargetSelectorColorHighlighting";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            highlight_color: match inst.get("highlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            occluded_alpha: inst.get_f32("occludedAlpha").unwrap_or_default(),
            outline_only: inst.get_bool("outlineOnly").unwrap_or_default(),
            outline_width: inst.get_f32("outlineWidth").unwrap_or_default(),
            interference_amount: inst.get_f32("interferenceAmount").unwrap_or_default(),
            interference_speed: inst.get_f32("interferenceSpeed").unwrap_or_default(),
            interference_tiling: inst.get_f32("interferenceTiling").unwrap_or_default(),
            interference_brightness: inst.get_f32("interferenceBrightness").unwrap_or_default(),
            use_hostility_color: inst.get_bool("useHostilityColor").unwrap_or_default(),
        }
    }
}

/// DCB type: `STargetSelectorHudParams`
pub struct STargetSelectorHudParams {
    /// `calculateLockedTargetBracket` (Boolean)
    pub calculate_locked_target_bracket: bool,
    /// `calculateSelectedTargetBracket` (Boolean)
    pub calculate_selected_target_bracket: bool,
    /// `hudTargetPointerAngleOffset` (Single)
    pub hud_target_pointer_angle_offset: f32,
    /// `hudTargetPointerHeadFollowAngleInner` (Single)
    pub hud_target_pointer_head_follow_angle_inner: f32,
    /// `hudTargetPointerHeadFollowAngleOuter` (Single)
    pub hud_target_pointer_head_follow_angle_outer: f32,
    /// `hudTargetPointerHeadFollowSwapTime` (Single)
    pub hud_target_pointer_head_follow_swap_time: f32,
    /// `relativeAttitudePointerPosition` (Single)
    pub relative_attitude_pointer_position: f32,
    /// `showAllSubtargets` (Boolean)
    pub show_all_subtargets: bool,
    /// `targetPointerAlpha` (Class)
    pub target_pointer_alpha: Option<Handle<BezierCurve>>,
    /// `outlineSubtargetsLocked` (Class)
    pub outline_subtargets_locked: Option<Handle<STargetSelectorColorHighlighting>>,
    /// `outlineSubtargetsAvailable` (Class)
    pub outline_subtargets_available: Option<Handle<STargetSelectorColorHighlighting>>,
    /// `outlineSubtargetsObjective` (Class)
    pub outline_subtargets_objective: Option<Handle<STargetSelectorColorHighlighting>>,
    /// `outlineSubtargetsObjectiveLocked` (Class)
    pub outline_subtargets_objective_locked: Option<Handle<STargetSelectorColorHighlighting>>,
}

impl Pooled for STargetSelectorHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.starget_selector_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.starget_selector_hud_params }
}

impl<'a> Extract<'a> for STargetSelectorHudParams {
    const TYPE_NAME: &'static str = "STargetSelectorHudParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            calculate_locked_target_bracket: inst.get_bool("calculateLockedTargetBracket").unwrap_or_default(),
            calculate_selected_target_bracket: inst.get_bool("calculateSelectedTargetBracket").unwrap_or_default(),
            hud_target_pointer_angle_offset: inst.get_f32("hudTargetPointerAngleOffset").unwrap_or_default(),
            hud_target_pointer_head_follow_angle_inner: inst.get_f32("hudTargetPointerHeadFollowAngleInner").unwrap_or_default(),
            hud_target_pointer_head_follow_angle_outer: inst.get_f32("hudTargetPointerHeadFollowAngleOuter").unwrap_or_default(),
            hud_target_pointer_head_follow_swap_time: inst.get_f32("hudTargetPointerHeadFollowSwapTime").unwrap_or_default(),
            relative_attitude_pointer_position: inst.get_f32("relativeAttitudePointerPosition").unwrap_or_default(),
            show_all_subtargets: inst.get_bool("showAllSubtargets").unwrap_or_default(),
            target_pointer_alpha: match inst.get("targetPointerAlpha") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            outline_subtargets_locked: match inst.get("outlineSubtargetsLocked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            outline_subtargets_available: match inst.get("outlineSubtargetsAvailable") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            outline_subtargets_objective: match inst.get("outlineSubtargetsObjective") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            outline_subtargets_objective_locked: match inst.get("outlineSubtargetsObjectiveLocked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SProjectedPitchLadderParams`
pub struct SProjectedPitchLadderParams {
    /// `visibleSizeAngle` (Single)
    pub visible_size_angle: f32,
    /// `visibleFadeRatio` (Single)
    pub visible_fade_ratio: f32,
    /// `incrementAngle` (Single)
    pub increment_angle: f32,
    /// `centersEnabled` (Boolean)
    pub centers_enabled: bool,
    /// `centersAlignmentType` (EnumChoice)
    pub centers_alignment_type: EProjectedHudAlignmentType,
    /// `sidesEnabled` (Boolean)
    pub sides_enabled: bool,
    /// `sidesHorizontalOffsetAngle` (Single)
    pub sides_horizontal_offset_angle: f32,
    /// `sidesPositionType` (EnumChoice)
    pub sides_position_type: EProjectedHudPositionType,
    /// `sidesAlignmentType` (EnumChoice)
    pub sides_alignment_type: EProjectedHudAlignmentType,
    /// `labelsEnabled` (Boolean)
    pub labels_enabled: bool,
    /// `labelsHorizontalOffsetAngle` (Single)
    pub labels_horizontal_offset_angle: f32,
    /// `labelsPositionType` (EnumChoice)
    pub labels_position_type: EProjectedHudPositionType,
    /// `labelsAlignmentType` (EnumChoice)
    pub labels_alignment_type: EProjectedHudAlignmentType,
    /// `enableZeroPitchElements` (Boolean)
    pub enable_zero_pitch_elements: bool,
}

impl Pooled for SProjectedPitchLadderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.sprojected_pitch_ladder_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.sprojected_pitch_ladder_params }
}

impl<'a> Extract<'a> for SProjectedPitchLadderParams {
    const TYPE_NAME: &'static str = "SProjectedPitchLadderParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            visible_size_angle: inst.get_f32("visibleSizeAngle").unwrap_or_default(),
            visible_fade_ratio: inst.get_f32("visibleFadeRatio").unwrap_or_default(),
            increment_angle: inst.get_f32("incrementAngle").unwrap_or_default(),
            centers_enabled: inst.get_bool("centersEnabled").unwrap_or_default(),
            centers_alignment_type: EProjectedHudAlignmentType::from_dcb_str(inst.get_str("centersAlignmentType").unwrap_or("")),
            sides_enabled: inst.get_bool("sidesEnabled").unwrap_or_default(),
            sides_horizontal_offset_angle: inst.get_f32("sidesHorizontalOffsetAngle").unwrap_or_default(),
            sides_position_type: EProjectedHudPositionType::from_dcb_str(inst.get_str("sidesPositionType").unwrap_or("")),
            sides_alignment_type: EProjectedHudAlignmentType::from_dcb_str(inst.get_str("sidesAlignmentType").unwrap_or("")),
            labels_enabled: inst.get_bool("labelsEnabled").unwrap_or_default(),
            labels_horizontal_offset_angle: inst.get_f32("labelsHorizontalOffsetAngle").unwrap_or_default(),
            labels_position_type: EProjectedHudPositionType::from_dcb_str(inst.get_str("labelsPositionType").unwrap_or("")),
            labels_alignment_type: EProjectedHudAlignmentType::from_dcb_str(inst.get_str("labelsAlignmentType").unwrap_or("")),
            enable_zero_pitch_elements: inst.get_bool("enableZeroPitchElements").unwrap_or_default(),
        }
    }
}

/// DCB type: `SProjectedYawLineParams`
pub struct SProjectedYawLineParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `startAngle` (Single)
    pub start_angle: f32,
    /// `endAngle` (Single)
    pub end_angle: f32,
    /// `ticksEnabled` (Boolean)
    pub ticks_enabled: bool,
    /// `tickForwardFadeStartAngle` (Single)
    pub tick_forward_fade_start_angle: f32,
    /// `tickForwardFadeEndAngle` (Single)
    pub tick_forward_fade_end_angle: f32,
    /// `tickBorderFadeAngle` (Single)
    pub tick_border_fade_angle: f32,
    /// `tickIncrementAngle` (Single)
    pub tick_increment_angle: f32,
    /// `tickIncrementVisualAngleRatio` (Single)
    pub tick_increment_visual_angle_ratio: f32,
    /// `tickAlignmentType` (EnumChoice)
    pub tick_alignment_type: EProjectedHudAlignmentType,
    /// `ticksAddCorners` (Boolean)
    pub ticks_add_corners: bool,
    /// `ticksAsFullCircle` (Boolean)
    pub ticks_as_full_circle: bool,
    /// `fixYawLineToAngle` (Boolean)
    pub fix_yaw_line_to_angle: bool,
    /// `fixedAngle` (Single)
    pub fixed_angle: f32,
    /// `anchorType` (EnumChoice)
    pub anchor_type: EProjectedHudYawLineAnchorType,
}

impl Pooled for SProjectedYawLineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.sprojected_yaw_line_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.sprojected_yaw_line_params }
}

impl<'a> Extract<'a> for SProjectedYawLineParams {
    const TYPE_NAME: &'static str = "SProjectedYawLineParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            start_angle: inst.get_f32("startAngle").unwrap_or_default(),
            end_angle: inst.get_f32("endAngle").unwrap_or_default(),
            ticks_enabled: inst.get_bool("ticksEnabled").unwrap_or_default(),
            tick_forward_fade_start_angle: inst.get_f32("tickForwardFadeStartAngle").unwrap_or_default(),
            tick_forward_fade_end_angle: inst.get_f32("tickForwardFadeEndAngle").unwrap_or_default(),
            tick_border_fade_angle: inst.get_f32("tickBorderFadeAngle").unwrap_or_default(),
            tick_increment_angle: inst.get_f32("tickIncrementAngle").unwrap_or_default(),
            tick_increment_visual_angle_ratio: inst.get_f32("tickIncrementVisualAngleRatio").unwrap_or_default(),
            tick_alignment_type: EProjectedHudAlignmentType::from_dcb_str(inst.get_str("tickAlignmentType").unwrap_or("")),
            ticks_add_corners: inst.get_bool("ticksAddCorners").unwrap_or_default(),
            ticks_as_full_circle: inst.get_bool("ticksAsFullCircle").unwrap_or_default(),
            fix_yaw_line_to_angle: inst.get_bool("fixYawLineToAngle").unwrap_or_default(),
            fixed_angle: inst.get_f32("fixedAngle").unwrap_or_default(),
            anchor_type: EProjectedHudYawLineAnchorType::from_dcb_str(inst.get_str("anchorType").unwrap_or("")),
        }
    }
}

/// DCB type: `SProjectedDisplayParams`
pub struct SProjectedDisplayParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `pitchOffset` (Single)
    pub pitch_offset: f32,
    /// `yawOffset` (Single)
    pub yaw_offset: f32,
    /// `alignmentType` (EnumChoice)
    pub alignment_type: EProjectedHudAlignmentType,
}

impl Pooled for SProjectedDisplayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.sprojected_display_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.sprojected_display_params }
}

impl<'a> Extract<'a> for SProjectedDisplayParams {
    const TYPE_NAME: &'static str = "SProjectedDisplayParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            pitch_offset: inst.get_f32("pitchOffset").unwrap_or_default(),
            yaw_offset: inst.get_f32("yawOffset").unwrap_or_default(),
            alignment_type: EProjectedHudAlignmentType::from_dcb_str(inst.get_str("alignmentType").unwrap_or("")),
        }
    }
}

/// DCB type: `SHudTapeParams`
pub struct SHudTapeParams {
    /// `range` (Single)
    pub range: f32,
    /// `mainTickIncrement` (Single)
    pub main_tick_increment: f32,
    /// `subTicks` (Int32)
    pub sub_ticks: i32,
}

impl Pooled for SHudTapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.shud_tape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.shud_tape_params }
}

impl<'a> Extract<'a> for SHudTapeParams {
    const TYPE_NAME: &'static str = "SHudTapeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            range: inst.get_f32("range").unwrap_or_default(),
            main_tick_increment: inst.get_f32("mainTickIncrement").unwrap_or_default(),
            sub_ticks: inst.get_i32("subTicks").unwrap_or_default(),
        }
    }
}

/// DCB type: `SProjectedHudParams`
pub struct SProjectedHudParams {
    /// `pitchLadder` (Class)
    pub pitch_ladder: Option<Handle<SProjectedPitchLadderParams>>,
    /// `yawLine` (Class)
    pub yaw_line: Option<Handle<SProjectedYawLineParams>>,
    /// `display` (Class)
    pub display: Option<Handle<SProjectedDisplayParams>>,
    /// `coilArrowShow` (Boolean)
    pub coil_arrow_show: bool,
    /// `coilArrowOffsetAngle` (Single)
    pub coil_arrow_offset_angle: f32,
    /// `coilArrowRotatesToTarget` (Boolean)
    pub coil_arrow_rotates_to_target: bool,
}

impl Pooled for SProjectedHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.sprojected_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.sprojected_hud_params }
}

impl<'a> Extract<'a> for SProjectedHudParams {
    const TYPE_NAME: &'static str = "SProjectedHudParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pitch_ladder: match inst.get("pitchLadder") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SProjectedPitchLadderParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            yaw_line: match inst.get("yawLine") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SProjectedYawLineParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            display: match inst.get("display") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SProjectedDisplayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            coil_arrow_show: inst.get_bool("coilArrowShow").unwrap_or_default(),
            coil_arrow_offset_angle: inst.get_f32("coilArrowOffsetAngle").unwrap_or_default(),
            coil_arrow_rotates_to_target: inst.get_bool("coilArrowRotatesToTarget").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVehicleHudParams`
pub struct SVehicleHudParams {
    /// `altitudeTape` (Class)
    pub altitude_tape: Option<Handle<SHudTapeParams>>,
    /// `radarAltimeterWidgetThreshold` (Single)
    pub radar_altimeter_widget_threshold: f32,
    /// `compassTape` (Class)
    pub compass_tape: Option<Handle<SHudTapeParams>>,
}

impl Pooled for SVehicleHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.svehicle_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.svehicle_hud_params }
}

impl<'a> Extract<'a> for SVehicleHudParams {
    const TYPE_NAME: &'static str = "SVehicleHudParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            altitude_tape: match inst.get("altitudeTape") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHudTapeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            radar_altimeter_widget_threshold: inst.get_f32("radarAltimeterWidgetThreshold").unwrap_or_default(),
            compass_tape: match inst.get("compassTape") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHudTapeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAimableGimbalModeLabels`
pub struct SAimableGimbalModeLabels {
    /// `aimTypeNamesFull` (Locale)
    pub aim_type_names_full: LocaleKey,
    /// `aimTypeNamesShort` (Locale)
    pub aim_type_names_short: LocaleKey,
    /// `gimbalStateNamesFull` (Locale)
    pub gimbal_state_names_full: LocaleKey,
    /// `gimbalStateNamesShort` (Locale)
    pub gimbal_state_names_short: LocaleKey,
}

impl Pooled for SAimableGimbalModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.saimable_gimbal_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.saimable_gimbal_mode_labels }
}

impl<'a> Extract<'a> for SAimableGimbalModeLabels {
    const TYPE_NAME: &'static str = "SAimableGimbalModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            aim_type_names_full: inst.get_str("aimTypeNamesFull").map(LocaleKey::from).unwrap_or_default(),
            aim_type_names_short: inst.get_str("aimTypeNamesShort").map(LocaleKey::from).unwrap_or_default(),
            gimbal_state_names_full: inst.get_str("gimbalStateNamesFull").map(LocaleKey::from).unwrap_or_default(),
            gimbal_state_names_short: inst.get_str("gimbalStateNamesShort").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SAimableControllerHudParams`
pub struct SAimableControllerHudParams {
    /// `hudAutoGimbalTrackingMarkerAnimationTime` (Single)
    pub hud_auto_gimbal_tracking_marker_animation_time: f32,
    /// `showAutoGimbalCombinedAllPIP` (Boolean)
    pub show_auto_gimbal_combined_all_pip: bool,
    /// `leadPipFadingAngle` (Single)
    pub lead_pip_fading_angle: f32,
    /// `leadPipFadingCurve` (Class)
    pub lead_pip_fading_curve: Option<Handle<BezierCurve>>,
    /// `lagPipFadingAngle` (Single)
    pub lag_pip_fading_angle: f32,
    /// `lagPipFadingCurve` (Class)
    pub lag_pip_fading_curve: Option<Handle<BezierCurve>>,
    /// `borderOffsetAngleMin` (Single)
    pub border_offset_angle_min: f32,
    /// `borderOffsetAngleMax` (Single)
    pub border_offset_angle_max: f32,
    /// `crosshairShapes` (Int32)
    pub crosshair_shapes: i32,
    /// `gimbalAlignmentAngle` (Single)
    pub gimbal_alignment_angle: f32,
    /// `gimbalAlignmentExcludeOutOfAngle` (Boolean)
    pub gimbal_alignment_exclude_out_of_angle: bool,
}

impl Pooled for SAimableControllerHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hudparams.saimable_controller_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hudparams.saimable_controller_hud_params }
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
                _ => None,
            },
            lag_pip_fading_angle: inst.get_f32("lagPipFadingAngle").unwrap_or_default(),
            lag_pip_fading_curve: match inst.get("lagPipFadingCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
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

