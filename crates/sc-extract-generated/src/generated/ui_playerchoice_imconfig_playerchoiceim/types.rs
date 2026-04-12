// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-playerchoice_imconfig_playerchoiceim`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PlayerChoice_RemoteCommsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_RemoteCommsConfig {
    /// `mobiglasConfig` (Reference)
    #[serde(default)]
    pub mobiglas_config: Option<CigGuid>,
    /// `MFDConfig` (Reference)
    #[serde(default)]
    pub mfdconfig: Option<CigGuid>,
    /// `visorConfig` (Reference)
    #[serde(default)]
    pub visor_config: Option<CigGuid>,
    /// `hologramConfig` (Reference)
    #[serde(default)]
    pub hologram_config: Option<CigGuid>,
}

impl Pooled for PlayerChoice_RemoteCommsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_imconfig_playerchoiceim.player_choice_remote_comms_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_imconfig_playerchoiceim.player_choice_remote_comms_config }
}

impl<'a> Extract<'a> for PlayerChoice_RemoteCommsConfig {
    const TYPE_NAME: &'static str = "PlayerChoice_RemoteCommsConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mobiglas_config: inst.get("mobiglasConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mfdconfig: inst.get("MFDConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            visor_config: inst.get("visorConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            hologram_config: inst.get("hologramConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PlayerChoice_HeadLookParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_HeadLookParams {
    /// `innerZoneSize` (Single)
    #[serde(default)]
    pub inner_zone_size: f32,
    /// `innerZoneFactor` (Single)
    #[serde(default)]
    pub inner_zone_factor: f32,
    /// `outerZoneFactor` (Single)
    #[serde(default)]
    pub outer_zone_factor: f32,
    /// `smoothZones` (Boolean)
    #[serde(default)]
    pub smooth_zones: bool,
}

impl Pooled for PlayerChoice_HeadLookParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_imconfig_playerchoiceim.player_choice_head_look_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_imconfig_playerchoiceim.player_choice_head_look_params }
}

impl<'a> Extract<'a> for PlayerChoice_HeadLookParams {
    const TYPE_NAME: &'static str = "PlayerChoice_HeadLookParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            inner_zone_size: inst.get_f32("innerZoneSize").unwrap_or_default(),
            inner_zone_factor: inst.get_f32("innerZoneFactor").unwrap_or_default(),
            outer_zone_factor: inst.get_f32("outerZoneFactor").unwrap_or_default(),
            smooth_zones: inst.get_bool("smoothZones").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_IMConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_IMConfig {
    /// `cursorScreenPercentage` (Int32)
    #[serde(default)]
    pub cursor_screen_percentage: i32,
    /// `interactionScreenPercentage` (Int32)
    #[serde(default)]
    pub interaction_screen_percentage: i32,
    /// `signalConfig` (Reference)
    #[serde(default)]
    pub signal_config: Option<CigGuid>,
    /// `interactionConfig` (Reference)
    #[serde(default)]
    pub interaction_config: Option<CigGuid>,
    /// `conversationConfig` (Reference)
    #[serde(default)]
    pub conversation_config: Option<CigGuid>,
    /// `remoteConfig` (Class)
    #[serde(default)]
    pub remote_config: Option<Handle<PlayerChoice_RemoteCommsConfig>>,
    /// `IMFOVFactor` (Single)
    #[serde(default)]
    pub imfovfactor: f32,
    /// `FOVLerpSpeed` (Single)
    #[serde(default)]
    pub fovlerp_speed: f32,
    /// `focusModeLerpSpeed` (Single)
    #[serde(default)]
    pub focus_mode_lerp_speed: f32,
    /// `focusModeFOVFactor` (Single)
    #[serde(default)]
    pub focus_mode_fovfactor: f32,
    /// `useFocusModeDOF` (Boolean)
    #[serde(default)]
    pub use_focus_mode_dof: bool,
    /// `maxMouseDistanceToThought` (Single)
    #[serde(default)]
    pub max_mouse_distance_to_thought: f32,
    /// `mouseDistanceToDriveSelection` (Single)
    #[serde(default)]
    pub mouse_distance_to_drive_selection: f32,
    /// `showItemInnerThoughtsInNormalMode` (Boolean)
    #[serde(default)]
    pub show_item_inner_thoughts_in_normal_mode: bool,
    /// `interactionPointStickinessFactor` (Single)
    #[serde(default)]
    pub interaction_point_stickiness_factor: f32,
    /// `maxSelectableIPAngle` (Single)
    #[serde(default)]
    pub max_selectable_ipangle: f32,
    /// `maxAlwaysSelectRadius` (Single)
    #[serde(default)]
    pub max_always_select_radius: f32,
    /// `quickInteractUseScoreSystem` (Boolean)
    #[serde(default)]
    pub quick_interact_use_score_system: bool,
    /// `quickInteractOnScreenPoints` (Single)
    #[serde(default)]
    pub quick_interact_on_screen_points: f32,
    /// `quickInteractAnglePoints` (Single)
    #[serde(default)]
    pub quick_interact_angle_points: f32,
    /// `quickInteractDistancePoints` (Single)
    #[serde(default)]
    pub quick_interact_distance_points: f32,
    /// `thoughtPosClampFactor` (Single)
    #[serde(default)]
    pub thought_pos_clamp_factor: f32,
    /// `throwPercentageIncreaseFactor` (Single)
    #[serde(default)]
    pub throw_percentage_increase_factor: f32,
    /// `quickPlaceThrowPercentage` (Single)
    #[serde(default)]
    pub quick_place_throw_percentage: f32,
    /// `quickPlaceDropTime` (Single)
    #[serde(default)]
    pub quick_place_drop_time: f32,
    /// `throwMaxImpulse` (Single)
    #[serde(default)]
    pub throw_max_impulse: f32,
    /// `throwMaxEpsilon` (Single)
    #[serde(default)]
    pub throw_max_epsilon: f32,
    /// `loosePlacementColor` (Class)
    #[serde(default)]
    pub loose_placement_color: Option<Handle<RGBA>>,
    /// `snapPlacementColor` (Class)
    #[serde(default)]
    pub snap_placement_color: Option<Handle<RGBA>>,
    /// `placementAttacherValidColor` (Class)
    #[serde(default)]
    pub placement_attacher_valid_color: Option<Handle<RGBA>>,
    /// `soundsRecord` (Reference)
    #[serde(default)]
    pub sounds_record: Option<CigGuid>,
    /// `holoSnapAudioID` (String)
    #[serde(default)]
    pub holo_snap_audio_id: String,
    /// `screenSpaceHoloSnapDistance` (Single)
    #[serde(default)]
    pub screen_space_holo_snap_distance: f32,
    /// `cursorSpeedMouseFactorIM` (Single)
    #[serde(default)]
    pub cursor_speed_mouse_factor_im: f32,
    /// `cursorSpeedMouseFactorFM` (Single)
    #[serde(default)]
    pub cursor_speed_mouse_factor_fm: f32,
    /// `cursorSpeedPadFactorIM` (Single)
    #[serde(default)]
    pub cursor_speed_pad_factor_im: f32,
    /// `cursorSpeedPadFactorFM` (Single)
    #[serde(default)]
    pub cursor_speed_pad_factor_fm: f32,
    /// `cursorSpeedAimingVehicleFactor` (Single)
    #[serde(default)]
    pub cursor_speed_aiming_vehicle_factor: f32,
    /// `allowedCameraViewsForIM` (EnumChoice (array))
    #[serde(default)]
    pub allowed_camera_views_for_im: Vec<String>,
    /// `snapToFirstPerson` (Boolean)
    #[serde(default)]
    pub snap_to_first_person: bool,
    /// `procBreathingSetup` (Reference)
    #[serde(default)]
    pub proc_breathing_setup: Option<CigGuid>,
    /// `xDirTowardsCenterFactor` (Single)
    #[serde(default)]
    pub x_dir_towards_center_factor: f32,
    /// `xDirAwayFromCenterFactor` (Single)
    #[serde(default)]
    pub x_dir_away_from_center_factor: f32,
    /// `yDirTowardsCenterFactor` (Single)
    #[serde(default)]
    pub y_dir_towards_center_factor: f32,
    /// `yDirAwayFromCenterFactor` (Single)
    #[serde(default)]
    pub y_dir_away_from_center_factor: f32,
    /// `zoomChangePerInput` (Single)
    #[serde(default)]
    pub zoom_change_per_input: f32,
    /// `zoomAccPeriod` (Single)
    #[serde(default)]
    pub zoom_acc_period: f32,
    /// `zoomAccFactor` (Single)
    #[serde(default)]
    pub zoom_acc_factor: f32,
    /// `IMFOVFactorMin` (Single)
    #[serde(default)]
    pub imfovfactor_min: f32,
    /// `focusModeDeadzone` (Class)
    #[serde(default)]
    pub focus_mode_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `IMDefaultZoomDeadzone` (Class)
    #[serde(default)]
    pub imdefault_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `IMFullZoomDeadzone` (Class)
    #[serde(default)]
    pub imfull_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `IMDefaultVehicleSeatZoomDeadzone` (Class)
    #[serde(default)]
    pub imdefault_vehicle_seat_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `IMFullVehicleSeatZoomDeadzone` (Class)
    #[serde(default)]
    pub imfull_vehicle_seat_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `conversationDeadzone` (Class)
    #[serde(default)]
    pub conversation_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `PITDeadzone` (Class)
    #[serde(default)]
    pub pitdeadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `worldDisplayDeadzone` (Class)
    #[serde(default)]
    pub world_display_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `screenFocusDeadzone` (Class)
    #[serde(default)]
    pub screen_focus_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// `screenFocusLerpSpeed` (Single)
    #[serde(default)]
    pub screen_focus_lerp_speed: f32,
    /// `screenFocusCancelDistance` (Single)
    #[serde(default)]
    pub screen_focus_cancel_distance: f32,
    /// `screenFocusSelectionAngleRange` (Single)
    #[serde(default)]
    pub screen_focus_selection_angle_range: f32,
    /// `blockedConditionCheckTimer` (Single)
    #[serde(default)]
    pub blocked_condition_check_timer: f32,
    /// `cursorEdgeRotationSpeed` (Single)
    #[serde(default)]
    pub cursor_edge_rotation_speed: f32,
    /// `includeOriginalTextInBlockedText` (Boolean)
    #[serde(default)]
    pub include_original_text_in_blocked_text: bool,
    /// `quickSelectTimer` (Single)
    #[serde(default)]
    pub quick_select_timer: f32,
    /// `heldEntityMaxCursorDistance` (Single)
    #[serde(default)]
    pub held_entity_max_cursor_distance: f32,
    /// `promptDisplayFullRadius` (Single)
    #[serde(default)]
    pub prompt_display_full_radius: f32,
    /// `promptDisplayCentreRadius` (Single)
    #[serde(default)]
    pub prompt_display_centre_radius: f32,
    /// `IMSelectionScoreCutoff` (Single)
    #[serde(default)]
    pub imselection_score_cutoff: f32,
    /// `interactionPromptPreviousUpdateTime` (Single)
    #[serde(default)]
    pub interaction_prompt_previous_update_time: f32,
}

impl Pooled for PlayerChoice_IMConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_imconfig_playerchoiceim.player_choice_imconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_imconfig_playerchoiceim.player_choice_imconfig }
}

impl<'a> Extract<'a> for PlayerChoice_IMConfig {
    const TYPE_NAME: &'static str = "PlayerChoice_IMConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cursor_screen_percentage: inst.get_i32("cursorScreenPercentage").unwrap_or_default(),
            interaction_screen_percentage: inst.get_i32("interactionScreenPercentage").unwrap_or_default(),
            signal_config: inst.get("signalConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interaction_config: inst.get("interactionConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            conversation_config: inst.get("conversationConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            remote_config: match inst.get("remoteConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_RemoteCommsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_RemoteCommsConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imfovfactor: inst.get_f32("IMFOVFactor").unwrap_or_default(),
            fovlerp_speed: inst.get_f32("FOVLerpSpeed").unwrap_or_default(),
            focus_mode_lerp_speed: inst.get_f32("focusModeLerpSpeed").unwrap_or_default(),
            focus_mode_fovfactor: inst.get_f32("focusModeFOVFactor").unwrap_or_default(),
            use_focus_mode_dof: inst.get_bool("useFocusModeDOF").unwrap_or_default(),
            max_mouse_distance_to_thought: inst.get_f32("maxMouseDistanceToThought").unwrap_or_default(),
            mouse_distance_to_drive_selection: inst.get_f32("mouseDistanceToDriveSelection").unwrap_or_default(),
            show_item_inner_thoughts_in_normal_mode: inst.get_bool("showItemInnerThoughtsInNormalMode").unwrap_or_default(),
            interaction_point_stickiness_factor: inst.get_f32("interactionPointStickinessFactor").unwrap_or_default(),
            max_selectable_ipangle: inst.get_f32("maxSelectableIPAngle").unwrap_or_default(),
            max_always_select_radius: inst.get_f32("maxAlwaysSelectRadius").unwrap_or_default(),
            quick_interact_use_score_system: inst.get_bool("quickInteractUseScoreSystem").unwrap_or_default(),
            quick_interact_on_screen_points: inst.get_f32("quickInteractOnScreenPoints").unwrap_or_default(),
            quick_interact_angle_points: inst.get_f32("quickInteractAnglePoints").unwrap_or_default(),
            quick_interact_distance_points: inst.get_f32("quickInteractDistancePoints").unwrap_or_default(),
            thought_pos_clamp_factor: inst.get_f32("thoughtPosClampFactor").unwrap_or_default(),
            throw_percentage_increase_factor: inst.get_f32("throwPercentageIncreaseFactor").unwrap_or_default(),
            quick_place_throw_percentage: inst.get_f32("quickPlaceThrowPercentage").unwrap_or_default(),
            quick_place_drop_time: inst.get_f32("quickPlaceDropTime").unwrap_or_default(),
            throw_max_impulse: inst.get_f32("throwMaxImpulse").unwrap_or_default(),
            throw_max_epsilon: inst.get_f32("throwMaxEpsilon").unwrap_or_default(),
            loose_placement_color: match inst.get("loosePlacementColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            snap_placement_color: match inst.get("snapPlacementColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            placement_attacher_valid_color: match inst.get("placementAttacherValidColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sounds_record: inst.get("soundsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            holo_snap_audio_id: inst.get_str("holoSnapAudioID").map(String::from).unwrap_or_default(),
            screen_space_holo_snap_distance: inst.get_f32("screenSpaceHoloSnapDistance").unwrap_or_default(),
            cursor_speed_mouse_factor_im: inst.get_f32("cursorSpeedMouseFactorIM").unwrap_or_default(),
            cursor_speed_mouse_factor_fm: inst.get_f32("cursorSpeedMouseFactorFM").unwrap_or_default(),
            cursor_speed_pad_factor_im: inst.get_f32("cursorSpeedPadFactorIM").unwrap_or_default(),
            cursor_speed_pad_factor_fm: inst.get_f32("cursorSpeedPadFactorFM").unwrap_or_default(),
            cursor_speed_aiming_vehicle_factor: inst.get_f32("cursorSpeedAimingVehicleFactor").unwrap_or_default(),
            allowed_camera_views_for_im: inst.get_array("allowedCameraViewsForIM")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            snap_to_first_person: inst.get_bool("snapToFirstPerson").unwrap_or_default(),
            proc_breathing_setup: inst.get("procBreathingSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            x_dir_towards_center_factor: inst.get_f32("xDirTowardsCenterFactor").unwrap_or_default(),
            x_dir_away_from_center_factor: inst.get_f32("xDirAwayFromCenterFactor").unwrap_or_default(),
            y_dir_towards_center_factor: inst.get_f32("yDirTowardsCenterFactor").unwrap_or_default(),
            y_dir_away_from_center_factor: inst.get_f32("yDirAwayFromCenterFactor").unwrap_or_default(),
            zoom_change_per_input: inst.get_f32("zoomChangePerInput").unwrap_or_default(),
            zoom_acc_period: inst.get_f32("zoomAccPeriod").unwrap_or_default(),
            zoom_acc_factor: inst.get_f32("zoomAccFactor").unwrap_or_default(),
            imfovfactor_min: inst.get_f32("IMFOVFactorMin").unwrap_or_default(),
            focus_mode_deadzone: match inst.get("focusModeDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imdefault_zoom_deadzone: match inst.get("IMDefaultZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imfull_zoom_deadzone: match inst.get("IMFullZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imdefault_vehicle_seat_zoom_deadzone: match inst.get("IMDefaultVehicleSeatZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imfull_vehicle_seat_zoom_deadzone: match inst.get("IMFullVehicleSeatZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            conversation_deadzone: match inst.get("conversationDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitdeadzone: match inst.get("PITDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            world_display_deadzone: match inst.get("worldDisplayDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            screen_focus_deadzone: match inst.get("screenFocusDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            screen_focus_lerp_speed: inst.get_f32("screenFocusLerpSpeed").unwrap_or_default(),
            screen_focus_cancel_distance: inst.get_f32("screenFocusCancelDistance").unwrap_or_default(),
            screen_focus_selection_angle_range: inst.get_f32("screenFocusSelectionAngleRange").unwrap_or_default(),
            blocked_condition_check_timer: inst.get_f32("blockedConditionCheckTimer").unwrap_or_default(),
            cursor_edge_rotation_speed: inst.get_f32("cursorEdgeRotationSpeed").unwrap_or_default(),
            include_original_text_in_blocked_text: inst.get_bool("includeOriginalTextInBlockedText").unwrap_or_default(),
            quick_select_timer: inst.get_f32("quickSelectTimer").unwrap_or_default(),
            held_entity_max_cursor_distance: inst.get_f32("heldEntityMaxCursorDistance").unwrap_or_default(),
            prompt_display_full_radius: inst.get_f32("promptDisplayFullRadius").unwrap_or_default(),
            prompt_display_centre_radius: inst.get_f32("promptDisplayCentreRadius").unwrap_or_default(),
            imselection_score_cutoff: inst.get_f32("IMSelectionScoreCutoff").unwrap_or_default(),
            interaction_prompt_previous_update_time: inst.get_f32("interactionPromptPreviousUpdateTime").unwrap_or_default(),
        }
    }
}

