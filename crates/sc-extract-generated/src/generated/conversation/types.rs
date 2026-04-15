// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `conversation`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `StickyFilterMovementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyFilterMovementParams {
    /// `matchNPCSpeed` (Boolean)
    #[serde(default)]
    pub match_npcspeed: bool,
    /// `constantFOV` (Boolean)
    #[serde(default)]
    pub constant_fov: bool,
    /// `approachingOuterSpeed` (Single)
    #[serde(default)]
    pub approaching_outer_speed: f32,
    /// `approachingInnerSpeed` (Single)
    #[serde(default)]
    pub approaching_inner_speed: f32,
    /// `retreatOuterSpeed` (Single)
    #[serde(default)]
    pub retreat_outer_speed: f32,
    /// `retreatIntermediateSpeed` (Single)
    #[serde(default)]
    pub retreat_intermediate_speed: f32,
    /// `innerRadius` (Single)
    #[serde(default)]
    pub inner_radius: f32,
    /// `intermediateRadius` (Single)
    #[serde(default)]
    pub intermediate_radius: f32,
    /// `outerRadius` (Single)
    #[serde(default)]
    pub outer_radius: f32,
    /// `breakRadius` (Single)
    #[serde(default)]
    pub break_radius: f32,
    /// `nudgeFraction` (Single)
    #[serde(default)]
    pub nudge_fraction: f32,
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `lockOffsetRotation` (Boolean)
    #[serde(default)]
    pub lock_offset_rotation: bool,
    /// `lerpTimeToFullSpeedOnStop` (Single)
    #[serde(default)]
    pub lerp_time_to_full_speed_on_stop: f32,
    /// `minMovementThreshold` (Single)
    #[serde(default)]
    pub min_movement_threshold: f32,
}

impl Pooled for StickyFilterMovementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.conversation.sticky_filter_movement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.conversation.sticky_filter_movement_params }
}

impl<'a> Extract<'a> for StickyFilterMovementParams {
    const TYPE_NAME: &'static str = "StickyFilterMovementParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            match_npcspeed: inst.get_bool("matchNPCSpeed").unwrap_or_default(),
            constant_fov: inst.get_bool("constantFOV").unwrap_or_default(),
            approaching_outer_speed: inst.get_f32("approachingOuterSpeed").unwrap_or_default(),
            approaching_inner_speed: inst.get_f32("approachingInnerSpeed").unwrap_or_default(),
            retreat_outer_speed: inst.get_f32("retreatOuterSpeed").unwrap_or_default(),
            retreat_intermediate_speed: inst.get_f32("retreatIntermediateSpeed").unwrap_or_default(),
            inner_radius: inst.get_f32("innerRadius").unwrap_or_default(),
            intermediate_radius: inst.get_f32("intermediateRadius").unwrap_or_default(),
            outer_radius: inst.get_f32("outerRadius").unwrap_or_default(),
            break_radius: inst.get_f32("breakRadius").unwrap_or_default(),
            nudge_fraction: inst.get_f32("nudgeFraction").unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            lock_offset_rotation: inst.get_bool("lockOffsetRotation").unwrap_or_default(),
            lerp_time_to_full_speed_on_stop: inst.get_f32("lerpTimeToFullSpeedOnStop").unwrap_or_default(),
            min_movement_threshold: inst.get_f32("minMovementThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `StickyFilterRotationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyFilterRotationParams {
    /// `outerRotationSpeed` (Single)
    #[serde(default)]
    pub outer_rotation_speed: f32,
    /// `innerRotationSpeed` (Single)
    #[serde(default)]
    pub inner_rotation_speed: f32,
    /// `minRadiusAngle` (Single)
    #[serde(default)]
    pub min_radius_angle: f32,
    /// `maxRadiusAngle` (Single)
    #[serde(default)]
    pub max_radius_angle: f32,
    /// `breakRadiusAngle` (Single)
    #[serde(default)]
    pub break_radius_angle: f32,
    /// `deadZoneRadiusAngle` (Single)
    #[serde(default)]
    pub dead_zone_radius_angle: f32,
}

impl Pooled for StickyFilterRotationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.conversation.sticky_filter_rotation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.conversation.sticky_filter_rotation_params }
}

impl<'a> Extract<'a> for StickyFilterRotationParams {
    const TYPE_NAME: &'static str = "StickyFilterRotationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            outer_rotation_speed: inst.get_f32("outerRotationSpeed").unwrap_or_default(),
            inner_rotation_speed: inst.get_f32("innerRotationSpeed").unwrap_or_default(),
            min_radius_angle: inst.get_f32("minRadiusAngle").unwrap_or_default(),
            max_radius_angle: inst.get_f32("maxRadiusAngle").unwrap_or_default(),
            break_radius_angle: inst.get_f32("breakRadiusAngle").unwrap_or_default(),
            dead_zone_radius_angle: inst.get_f32("deadZoneRadiusAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `StickyFilterAutocenterParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyFilterAutocenterParams {
    /// `idleTimeBeforeRecenter` (Single)
    #[serde(default)]
    pub idle_time_before_recenter: f32,
    /// `timeRecenterAtMinAngle` (Single)
    #[serde(default)]
    pub time_recenter_at_min_angle: f32,
    /// `timeRecenterAtMaxAngle` (Single)
    #[serde(default)]
    pub time_recenter_at_max_angle: f32,
    /// `timeRecenterAtMinAngleMoving` (Single)
    #[serde(default)]
    pub time_recenter_at_min_angle_moving: f32,
    /// `timeRecenterAtMaxAngleMoving` (Single)
    #[serde(default)]
    pub time_recenter_at_max_angle_moving: f32,
    /// `eyeOffsetAtMinDistance` (Class)
    #[serde(default)]
    pub eye_offset_at_min_distance: Option<Handle<Vec3>>,
    /// `eyeOffsetAtMaxDistance` (Class)
    #[serde(default)]
    pub eye_offset_at_max_distance: Option<Handle<Vec3>>,
}

impl Pooled for StickyFilterAutocenterParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.conversation.sticky_filter_autocenter_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.conversation.sticky_filter_autocenter_params }
}

impl<'a> Extract<'a> for StickyFilterAutocenterParams {
    const TYPE_NAME: &'static str = "StickyFilterAutocenterParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            idle_time_before_recenter: inst.get_f32("idleTimeBeforeRecenter").unwrap_or_default(),
            time_recenter_at_min_angle: inst.get_f32("timeRecenterAtMinAngle").unwrap_or_default(),
            time_recenter_at_max_angle: inst.get_f32("timeRecenterAtMaxAngle").unwrap_or_default(),
            time_recenter_at_min_angle_moving: inst.get_f32("timeRecenterAtMinAngleMoving").unwrap_or_default(),
            time_recenter_at_max_angle_moving: inst.get_f32("timeRecenterAtMaxAngleMoving").unwrap_or_default(),
            eye_offset_at_min_distance: match inst.get("eyeOffsetAtMinDistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            eye_offset_at_max_distance: match inst.get("eyeOffsetAtMaxDistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ConversationStickyFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationStickyFilter {
    /// `movementParams` (Class)
    #[serde(default)]
    pub movement_params: Option<Handle<StickyFilterMovementParams>>,
    /// `rotationParams` (Class)
    #[serde(default)]
    pub rotation_params: Option<Handle<StickyFilterRotationParams>>,
    /// `autoCenterParams` (Class)
    #[serde(default)]
    pub auto_center_params: Option<Handle<StickyFilterAutocenterParams>>,
    /// `dynamicCameraEffectsParams` (Reference)
    #[serde(default)]
    pub dynamic_camera_effects_params: Option<CigGuid>,
    /// `highPriorityCameraEffects` (Boolean)
    #[serde(default)]
    pub high_priority_camera_effects: bool,
}

impl Pooled for ConversationStickyFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.conversation.conversation_sticky_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.conversation.conversation_sticky_filter }
}

impl<'a> Extract<'a> for ConversationStickyFilter {
    const TYPE_NAME: &'static str = "ConversationStickyFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movement_params: match inst.get("movementParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StickyFilterMovementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rotation_params: match inst.get("rotationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StickyFilterRotationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            auto_center_params: match inst.get("autoCenterParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StickyFilterAutocenterParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            dynamic_camera_effects_params: inst.get("dynamicCameraEffectsParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            high_priority_camera_effects: inst.get_bool("highPriorityCameraEffects").unwrap_or_default(),
        }
    }
}

/// DCB type: `CinematicConversationSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicConversationSettings {
    /// `widthAspectRatio` (Single)
    #[serde(default)]
    pub width_aspect_ratio: f32,
    /// `heightAspectRatio` (Single)
    #[serde(default)]
    pub height_aspect_ratio: f32,
    /// `lerpInTime` (Single)
    #[serde(default)]
    pub lerp_in_time: f32,
    /// `lerpOutTime` (Single)
    #[serde(default)]
    pub lerp_out_time: f32,
}

impl Pooled for CinematicConversationSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.conversation.cinematic_conversation_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.conversation.cinematic_conversation_settings }
}

impl<'a> Extract<'a> for CinematicConversationSettings {
    const TYPE_NAME: &'static str = "CinematicConversationSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            width_aspect_ratio: inst.get_f32("widthAspectRatio").unwrap_or_default(),
            height_aspect_ratio: inst.get_f32("heightAspectRatio").unwrap_or_default(),
            lerp_in_time: inst.get_f32("lerpInTime").unwrap_or_default(),
            lerp_out_time: inst.get_f32("lerpOutTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `SConversationIconParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SConversationIconParams {
    /// `useConversationIcon` (Boolean)
    #[serde(default)]
    pub use_conversation_icon: bool,
    /// `entitySuperGUID` (String)
    #[serde(default)]
    pub entity_super_guid: String,
    /// `iconVisibleGUID` (String)
    #[serde(default)]
    pub icon_visible_guid: String,
    /// `distanceToSwitchToText` (Single)
    #[serde(default)]
    pub distance_to_switch_to_text: f32,
    /// `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
    /// `rotationOffset` (Class)
    #[serde(default)]
    pub rotation_offset: Option<Handle<Vec3>>,
    /// `iconScale` (Single)
    #[serde(default)]
    pub icon_scale: f32,
    /// `textScale` (Single)
    #[serde(default)]
    pub text_scale: f32,
    /// `alwaysFacePlayer` (Boolean)
    #[serde(default)]
    pub always_face_player: bool,
    /// `maintainPosition` (Boolean)
    #[serde(default)]
    pub maintain_position: bool,
    /// `usePlayerAsReferenceEntity` (Boolean)
    #[serde(default)]
    pub use_player_as_reference_entity: bool,
}

impl Pooled for SConversationIconParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.conversation.sconversation_icon_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.conversation.sconversation_icon_params }
}

impl<'a> Extract<'a> for SConversationIconParams {
    const TYPE_NAME: &'static str = "SConversationIconParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_conversation_icon: inst.get_bool("useConversationIcon").unwrap_or_default(),
            entity_super_guid: inst.get_str("entitySuperGUID").map(String::from).unwrap_or_default(),
            icon_visible_guid: inst.get_str("iconVisibleGUID").map(String::from).unwrap_or_default(),
            distance_to_switch_to_text: inst.get_f32("distanceToSwitchToText").unwrap_or_default(),
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rotation_offset: match inst.get("rotationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            icon_scale: inst.get_f32("iconScale").unwrap_or_default(),
            text_scale: inst.get_f32("textScale").unwrap_or_default(),
            always_face_player: inst.get_bool("alwaysFacePlayer").unwrap_or_default(),
            maintain_position: inst.get_bool("maintainPosition").unwrap_or_default(),
            use_player_as_reference_entity: inst.get_bool("usePlayerAsReferenceEntity").unwrap_or_default(),
        }
    }
}

/// DCB type: `SScenePlayerChoiceSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScenePlayerChoiceSettings {
    /// `iconParams` (Class)
    #[serde(default)]
    pub icon_params: Option<Handle<SConversationIconParams>>,
}

impl Pooled for SScenePlayerChoiceSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.conversation.sscene_player_choice_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.conversation.sscene_player_choice_settings }
}

impl<'a> Extract<'a> for SScenePlayerChoiceSettings {
    const TYPE_NAME: &'static str = "SScenePlayerChoiceSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            icon_params: match inst.get("iconParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SConversationIconParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

