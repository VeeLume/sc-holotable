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

/// DCB type: `LadderAnimationOffset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderAnimationOffset {
    /// DCB field: `stopClimbDistanceFromTopBack` (Single)
    #[serde(default)]
    pub stop_climb_distance_from_top_back: f32,
    /// DCB field: `stopClimbDistanceFromTopLeft` (Single)
    #[serde(default)]
    pub stop_climb_distance_from_top_left: f32,
    /// DCB field: `stopClimbDistanceFromTopRight` (Single)
    #[serde(default)]
    pub stop_climb_distance_from_top_right: f32,
    /// DCB field: `stopClimbDistanceFromBottom` (Single)
    #[serde(default)]
    pub stop_climb_distance_from_bottom: f32,
    /// DCB field: `mountClimbDistanceFromBottom` (Single)
    #[serde(default)]
    pub mount_climb_distance_from_bottom: f32,
    /// DCB field: `animationTravelDistance` (Single)
    #[serde(default)]
    pub animation_travel_distance: f32,
}

impl Pooled for LadderAnimationOffset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.ladder_animation_offset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.ladder_animation_offset }
}

impl<'a> Extract<'a> for LadderAnimationOffset {
    const TYPE_NAME: &'static str = "LadderAnimationOffset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stop_climb_distance_from_top_back: inst.get_f32("stopClimbDistanceFromTopBack").unwrap_or_default(),
            stop_climb_distance_from_top_left: inst.get_f32("stopClimbDistanceFromTopLeft").unwrap_or_default(),
            stop_climb_distance_from_top_right: inst.get_f32("stopClimbDistanceFromTopRight").unwrap_or_default(),
            stop_climb_distance_from_bottom: inst.get_f32("stopClimbDistanceFromBottom").unwrap_or_default(),
            mount_climb_distance_from_bottom: inst.get_f32("mountClimbDistanceFromBottom").unwrap_or_default(),
            animation_travel_distance: inst.get_f32("animationTravelDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `LadderAnimationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderAnimationParams {
    /// DCB field: `animationOffsets` (Class)
    #[serde(default)]
    pub animation_offsets: Option<Handle<LadderAnimationOffset>>,
    /// DCB field: `animationTravelDistanceTransWalkToRun` (Single)
    #[serde(default)]
    pub animation_travel_distance_trans_walk_to_run: f32,
    /// DCB field: `animationTravelDistanceTransRunToWalk` (Single)
    #[serde(default)]
    pub animation_travel_distance_trans_run_to_walk: f32,
    /// DCB field: `animationTravelDistanceTransRunToSprint` (Single)
    #[serde(default)]
    pub animation_travel_distance_trans_run_to_sprint: f32,
    /// DCB field: `animationTravelDistanceTransSprintToRun` (Single)
    #[serde(default)]
    pub animation_travel_distance_trans_sprint_to_run: f32,
    /// DCB field: `playerHorizontalOffset` (Single)
    #[serde(default)]
    pub player_horizontal_offset: f32,
    /// DCB field: `getOnDistanceAwayTop` (Single)
    #[serde(default)]
    pub get_on_distance_away_top: f32,
    /// DCB field: `getOnDistanceAwayTopSide` (Single)
    #[serde(default)]
    pub get_on_distance_away_top_side: f32,
    /// DCB field: `getOnDistanceAwayTopSideWalk` (Single)
    #[serde(default)]
    pub get_on_distance_away_top_side_walk: f32,
    /// DCB field: `getOnDistanceAwayTopSideRun` (Single)
    #[serde(default)]
    pub get_on_distance_away_top_side_run: f32,
    /// DCB field: `getOnDistanceAwayTopSideSprint` (Single)
    #[serde(default)]
    pub get_on_distance_away_top_side_sprint: f32,
    /// DCB field: `getOnDistanceAwayVerticalTopSide` (Single)
    #[serde(default)]
    pub get_on_distance_away_vertical_top_side: f32,
    /// DCB field: `getOnDistanceAwayBottom` (Single)
    #[serde(default)]
    pub get_on_distance_away_bottom: f32,
    /// DCB field: `getOnForwardDistanceAwayMiddle` (Single)
    #[serde(default)]
    pub get_on_forward_distance_away_middle: f32,
    /// DCB field: `getOnDistanceAwayMiddleSide` (Single)
    #[serde(default)]
    pub get_on_distance_away_middle_side: f32,
}

impl Pooled for LadderAnimationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.ladder_animation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.ladder_animation_params }
}

impl<'a> Extract<'a> for LadderAnimationParams {
    const TYPE_NAME: &'static str = "LadderAnimationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_offsets: match inst.get("animationOffsets") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderAnimationOffset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LadderAnimationOffset>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            animation_travel_distance_trans_walk_to_run: inst.get_f32("animationTravelDistanceTransWalkToRun").unwrap_or_default(),
            animation_travel_distance_trans_run_to_walk: inst.get_f32("animationTravelDistanceTransRunToWalk").unwrap_or_default(),
            animation_travel_distance_trans_run_to_sprint: inst.get_f32("animationTravelDistanceTransRunToSprint").unwrap_or_default(),
            animation_travel_distance_trans_sprint_to_run: inst.get_f32("animationTravelDistanceTransSprintToRun").unwrap_or_default(),
            player_horizontal_offset: inst.get_f32("playerHorizontalOffset").unwrap_or_default(),
            get_on_distance_away_top: inst.get_f32("getOnDistanceAwayTop").unwrap_or_default(),
            get_on_distance_away_top_side: inst.get_f32("getOnDistanceAwayTopSide").unwrap_or_default(),
            get_on_distance_away_top_side_walk: inst.get_f32("getOnDistanceAwayTopSideWalk").unwrap_or_default(),
            get_on_distance_away_top_side_run: inst.get_f32("getOnDistanceAwayTopSideRun").unwrap_or_default(),
            get_on_distance_away_top_side_sprint: inst.get_f32("getOnDistanceAwayTopSideSprint").unwrap_or_default(),
            get_on_distance_away_vertical_top_side: inst.get_f32("getOnDistanceAwayVerticalTopSide").unwrap_or_default(),
            get_on_distance_away_bottom: inst.get_f32("getOnDistanceAwayBottom").unwrap_or_default(),
            get_on_forward_distance_away_middle: inst.get_f32("getOnForwardDistanceAwayMiddle").unwrap_or_default(),
            get_on_distance_away_middle_side: inst.get_f32("getOnDistanceAwayMiddleSide").unwrap_or_default(),
        }
    }
}

/// DCB type: `LadderMovementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderMovementParams {
    /// DCB field: `movementAcceleration` (Single)
    #[serde(default)]
    pub movement_acceleration: f32,
    /// DCB field: `movementSpeedUpwards` (Single)
    #[serde(default)]
    pub movement_speed_upwards: f32,
    /// DCB field: `movementSpeedDownwards` (Single)
    #[serde(default)]
    pub movement_speed_downwards: f32,
    /// DCB field: `movementSprintUpSpeedScale` (Single)
    #[serde(default)]
    pub movement_sprint_up_speed_scale: f32,
    /// DCB field: `movementSprintDownSpeedScale` (Single)
    #[serde(default)]
    pub movement_sprint_down_speed_scale: f32,
    /// DCB field: `movementQuickSettleSpeedScale` (Single)
    #[serde(default)]
    pub movement_quick_settle_speed_scale: f32,
    /// DCB field: `movementSlideSpeedMaxVel` (Single)
    #[serde(default)]
    pub movement_slide_speed_max_vel: f32,
    /// DCB field: `movementSlideFriction` (Single)
    #[serde(default)]
    pub movement_slide_friction: f32,
    /// DCB field: `movementSlideMinGravity` (Single)
    #[serde(default)]
    pub movement_slide_min_gravity: f32,
    /// DCB field: `movementSettleSpeed` (Single)
    #[serde(default)]
    pub movement_settle_speed: f32,
    /// DCB field: `movementInertiaDecayRate` (Single)
    #[serde(default)]
    pub movement_inertia_decay_rate: f32,
}

impl Pooled for LadderMovementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.ladder_movement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.ladder_movement_params }
}

impl<'a> Extract<'a> for LadderMovementParams {
    const TYPE_NAME: &'static str = "LadderMovementParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            movement_acceleration: inst.get_f32("movementAcceleration").unwrap_or_default(),
            movement_speed_upwards: inst.get_f32("movementSpeedUpwards").unwrap_or_default(),
            movement_speed_downwards: inst.get_f32("movementSpeedDownwards").unwrap_or_default(),
            movement_sprint_up_speed_scale: inst.get_f32("movementSprintUpSpeedScale").unwrap_or_default(),
            movement_sprint_down_speed_scale: inst.get_f32("movementSprintDownSpeedScale").unwrap_or_default(),
            movement_quick_settle_speed_scale: inst.get_f32("movementQuickSettleSpeedScale").unwrap_or_default(),
            movement_slide_speed_max_vel: inst.get_f32("movementSlideSpeedMaxVel").unwrap_or_default(),
            movement_slide_friction: inst.get_f32("movementSlideFriction").unwrap_or_default(),
            movement_slide_min_gravity: inst.get_f32("movementSlideMinGravity").unwrap_or_default(),
            movement_settle_speed: inst.get_f32("movementSettleSpeed").unwrap_or_default(),
            movement_inertia_decay_rate: inst.get_f32("movementInertiaDecayRate").unwrap_or_default(),
        }
    }
}

/// DCB type: `LadderJumpParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderJumpParams {
    /// DCB field: `yawCutoffAngle` (Single)
    #[serde(default)]
    pub yaw_cutoff_angle: f32,
    /// DCB field: `minForwardVelocity` (Single)
    #[serde(default)]
    pub min_forward_velocity: f32,
    /// DCB field: `maxForwardVelocity` (Single)
    #[serde(default)]
    pub max_forward_velocity: f32,
    /// DCB field: `pitchCutoffAngle` (Single)
    #[serde(default)]
    pub pitch_cutoff_angle: f32,
    /// DCB field: `minUpVelocity` (Single)
    #[serde(default)]
    pub min_up_velocity: f32,
    /// DCB field: `maxUpVelocity` (Single)
    #[serde(default)]
    pub max_up_velocity: f32,
}

impl Pooled for LadderJumpParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.ladder_jump_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.ladder_jump_params }
}

impl<'a> Extract<'a> for LadderJumpParams {
    const TYPE_NAME: &'static str = "LadderJumpParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            yaw_cutoff_angle: inst.get_f32("yawCutoffAngle").unwrap_or_default(),
            min_forward_velocity: inst.get_f32("minForwardVelocity").unwrap_or_default(),
            max_forward_velocity: inst.get_f32("maxForwardVelocity").unwrap_or_default(),
            pitch_cutoff_angle: inst.get_f32("pitchCutoffAngle").unwrap_or_default(),
            min_up_velocity: inst.get_f32("minUpVelocity").unwrap_or_default(),
            max_up_velocity: inst.get_f32("maxUpVelocity").unwrap_or_default(),
        }
    }
}

/// DCB type: `LadderLookAroundParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderLookAroundParams {
    /// DCB field: `blendBackToClimbingLerpSpeed` (Single)
    #[serde(default)]
    pub blend_back_to_climbing_lerp_speed: f32,
    /// DCB field: `blendBackToClimbingLimitAngle` (Single)
    #[serde(default)]
    pub blend_back_to_climbing_limit_angle: f32,
}

impl Pooled for LadderLookAroundParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.ladder_look_around_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.ladder_look_around_params }
}

impl<'a> Extract<'a> for LadderLookAroundParams {
    const TYPE_NAME: &'static str = "LadderLookAroundParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blend_back_to_climbing_lerp_speed: inst.get_f32("blendBackToClimbingLerpSpeed").unwrap_or_default(),
            blend_back_to_climbing_limit_angle: inst.get_f32("blendBackToClimbingLimitAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `LadderConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderConfig {
    /// DCB field: `animationParams` (Class)
    #[serde(default)]
    pub animation_params: Option<Handle<LadderAnimationParams>>,
    /// DCB field: `movementParams` (Class)
    #[serde(default)]
    pub movement_params: Option<Handle<LadderMovementParams>>,
    /// DCB field: `horizontalViewLimit` (Single)
    #[serde(default)]
    pub horizontal_view_limit: f32,
    /// DCB field: `verticalUpViewLimit` (Single)
    #[serde(default)]
    pub vertical_up_view_limit: f32,
    /// DCB field: `leaningViewLimits` (Class)
    #[serde(default)]
    pub leaning_view_limits: Option<Handle<ActorViewLimits>>,
    /// DCB field: `jumpParams` (Class)
    #[serde(default)]
    pub jump_params: Option<Handle<LadderJumpParams>>,
    /// DCB field: `lookAroundParams` (Class)
    #[serde(default)]
    pub look_around_params: Option<Handle<LadderLookAroundParams>>,
}

impl Pooled for LadderConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.ladder_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.ladder_config }
}

impl<'a> Extract<'a> for LadderConfig {
    const TYPE_NAME: &'static str = "LadderConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_params: match inst.get("animationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderAnimationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LadderAnimationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            movement_params: match inst.get("movementParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderMovementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LadderMovementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            horizontal_view_limit: inst.get_f32("horizontalViewLimit").unwrap_or_default(),
            vertical_up_view_limit: inst.get_f32("verticalUpViewLimit").unwrap_or_default(),
            leaning_view_limits: match inst.get("leaningViewLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorViewLimits>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorViewLimits>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jump_params: match inst.get("jumpParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderJumpParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LadderJumpParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            look_around_params: match inst.get("lookAroundParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderLookAroundParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LadderLookAroundParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LandingZoneInventoryRedirect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingZoneInventoryRedirect {
    /// DCB field: `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// DCB field: `inventoryLocation` (Reference)
    #[serde(default)]
    pub inventory_location: Option<CigGuid>,
}

impl Pooled for LandingZoneInventoryRedirect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.landing_zone_inventory_redirect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.landing_zone_inventory_redirect }
}

impl<'a> Extract<'a> for LandingZoneInventoryRedirect {
    const TYPE_NAME: &'static str = "LandingZoneInventoryRedirect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            inventory_location: inst.get("inventoryLocation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `LandingZoneInventory`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingZoneInventory {
    /// DCB field: `containerParams` (Reference)
    #[serde(default)]
    pub container_params: Option<CigGuid>,
    /// DCB field: `locations` (Class (array))
    #[serde(default)]
    pub locations: Vec<Handle<InventoryLocation>>,
    /// DCB field: `locationRedirects` (Class (array))
    #[serde(default)]
    pub location_redirects: Vec<Handle<LandingZoneInventoryRedirect>>,
}

impl Pooled for LandingZoneInventory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.landing_zone_inventory }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.landing_zone_inventory }
}

impl<'a> Extract<'a> for LandingZoneInventory {
    const TYPE_NAME: &'static str = "LandingZoneInventory";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            container_params: inst.get("containerParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            locations: inst.get_array("locations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryLocation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryLocation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            location_redirects: inst.get_array("locationRedirects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LandingZoneInventoryRedirect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LandingZoneInventoryRedirect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingSelection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingSelection {
    /// DCB field: `animEventName` (String)
    #[serde(default)]
    pub anim_event_name: String,
    /// DCB field: `landing` (WeakPointer)
    #[serde(default)]
    pub landing: Option<Handle<ActorLandingNode>>,
}

impl Pooled for LandingSelection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.landing_selection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.landing_selection }
}

impl<'a> Extract<'a> for LandingSelection {
    const TYPE_NAME: &'static str = "LandingSelection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            anim_event_name: inst.get_str("animEventName").map(String::from).unwrap_or_default(),
            landing: match inst.get("landing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLandingNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLandingNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LandingAnimationSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingAnimationSetup {
    /// DCB field: `landingStrength` (EnumChoice)
    #[serde(default)]
    pub landing_strength: String,
    /// DCB field: `minVelocity` (Single)
    #[serde(default)]
    pub min_velocity: f32,
    /// DCB field: `condition` (EnumChoice)
    #[serde(default)]
    pub condition: String,
    /// DCB field: `fragment` (Class)
    #[serde(default)]
    pub fragment: Option<Handle<FragmentInfo>>,
    /// DCB field: `playProceduralLanding` (Boolean)
    #[serde(default)]
    pub play_procedural_landing: bool,
    /// DCB field: `lockMovement` (Boolean)
    #[serde(default)]
    pub lock_movement: bool,
    /// DCB field: `exitStance` (EnumChoice)
    #[serde(default)]
    pub exit_stance: String,
}

impl Pooled for LandingAnimationSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.landing_animation_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.landing_animation_setup }
}

impl<'a> Extract<'a> for LandingAnimationSetup {
    const TYPE_NAME: &'static str = "LandingAnimationSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            landing_strength: inst.get_str("landingStrength").map(String::from).unwrap_or_default(),
            min_velocity: inst.get_f32("minVelocity").unwrap_or_default(),
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
            fragment: match inst.get("fragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            play_procedural_landing: inst.get_bool("playProceduralLanding").unwrap_or_default(),
            lock_movement: inst.get_bool("lockMovement").unwrap_or_default(),
            exit_stance: inst.get_str("exitStance").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingPadSize`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingPadSize {
    /// DCB field: `id` (Int32)
    #[serde(default)]
    pub id: i32,
    /// DCB field: `shipSize` (Class)
    #[serde(default)]
    pub ship_size: Option<Handle<Vec3>>,
    /// DCB field: `groundVehicleSize` (Class)
    #[serde(default)]
    pub ground_vehicle_size: Option<Handle<Vec3>>,
}

impl Pooled for LandingPadSize {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_la.landing_pad_size }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_la.landing_pad_size }
}

impl<'a> Extract<'a> for LandingPadSize {
    const TYPE_NAME: &'static str = "LandingPadSize";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_i32("id").unwrap_or_default(),
            ship_size: match inst.get("shipSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ground_vehicle_size: match inst.get("groundVehicleSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

