// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-ladder`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `LadderNavigationLink`
/// Inherits from: `NavigationLinkType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderNavigationLink {
    /// `linkValidForAgentType` (String)
    #[serde(default)]
    pub link_valid_for_agent_type: String,
    /// `costMultiplierSetup` (Class)
    #[serde(default)]
    pub cost_multiplier_setup: Option<Handle<NavigationLinkCostCustomization>>,
    /// `linkingType` (EnumChoice)
    #[serde(default)]
    pub linking_type: ENavigationLinkLinkingType,
    /// `useChannel` (WeakPointer)
    #[serde(default)]
    pub use_channel: Option<Handle<UsableUseChannelInstance>>,
}

impl Pooled for LadderNavigationLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ladder.ladder_navigation_link }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ladder.ladder_navigation_link }
}

impl<'a> Extract<'a> for LadderNavigationLink {
    const TYPE_NAME: &'static str = "LadderNavigationLink";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            link_valid_for_agent_type: inst.get_str("linkValidForAgentType").map(String::from).unwrap_or_default(),
            cost_multiplier_setup: match inst.get("costMultiplierSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<NavigationLinkCostCustomization>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            linking_type: ENavigationLinkLinkingType::from_dcb_str(inst.get_str("linkingType").unwrap_or("")),
            use_channel: match inst.get("useChannel") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UsableUseChannelInstance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ExitCollisionCheckOverrideParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitCollisionCheckOverrideParams {
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `scale` (Single)
    #[serde(default)]
    pub scale: f32,
}

impl Pooled for ExitCollisionCheckOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ladder.exit_collision_check_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ladder.exit_collision_check_override_params }
}

impl<'a> Extract<'a> for ExitCollisionCheckOverrideParams {
    const TYPE_NAME: &'static str = "ExitCollisionCheckOverrideParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            scale: inst.get_f32("scale").unwrap_or_default(),
        }
    }
}

/// DCB type: `AutoMountRadiusParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoMountRadiusParams {
    /// `radiusWhenWalking` (Single)
    #[serde(default)]
    pub radius_when_walking: f32,
    /// `radiusWhenRunning` (Single)
    #[serde(default)]
    pub radius_when_running: f32,
    /// `radiusWhenSprinting` (Single)
    #[serde(default)]
    pub radius_when_sprinting: f32,
    /// `radiusOffset` (Single)
    #[serde(default)]
    pub radius_offset: f32,
}

impl Pooled for AutoMountRadiusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ladder.auto_mount_radius_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ladder.auto_mount_radius_params }
}

impl<'a> Extract<'a> for AutoMountRadiusParams {
    const TYPE_NAME: &'static str = "AutoMountRadiusParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radius_when_walking: inst.get_f32("radiusWhenWalking").unwrap_or_default(),
            radius_when_running: inst.get_f32("radiusWhenRunning").unwrap_or_default(),
            radius_when_sprinting: inst.get_f32("radiusWhenSprinting").unwrap_or_default(),
            radius_offset: inst.get_f32("radiusOffset").unwrap_or_default(),
        }
    }
}

/// DCB type: `LadderAccessClimbParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderAccessClimbParams {
    /// `approachAngle` (Single)
    #[serde(default)]
    pub approach_angle: f32,
    /// `facingAngle` (Single)
    #[serde(default)]
    pub facing_angle: f32,
    /// `autoMountRadiusOffset` (Single)
    #[serde(default)]
    pub auto_mount_radius_offset: f32,
    /// `autoMountRadius` (Single)
    #[serde(default)]
    pub auto_mount_radius: f32,
    /// `exitCollisionCheckOverride` (StrongPointer)
    #[serde(default)]
    pub exit_collision_check_override: Option<Handle<ExitCollisionCheckOverrideParams>>,
    /// `enterAnimWillGoUnderneath` (Boolean)
    #[serde(default)]
    pub enter_anim_will_go_underneath: bool,
}

impl Pooled for LadderAccessClimbParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ladder.ladder_access_climb_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ladder.ladder_access_climb_params }
}

impl<'a> Extract<'a> for LadderAccessClimbParams {
    const TYPE_NAME: &'static str = "LadderAccessClimbParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            approach_angle: inst.get_f32("approachAngle").unwrap_or_default(),
            facing_angle: inst.get_f32("facingAngle").unwrap_or_default(),
            auto_mount_radius_offset: inst.get_f32("autoMountRadiusOffset").unwrap_or_default(),
            auto_mount_radius: inst.get_f32("autoMountRadius").unwrap_or_default(),
            exit_collision_check_override: match inst.get("exitCollisionCheckOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExitCollisionCheckOverrideParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enter_anim_will_go_underneath: inst.get_bool("enterAnimWillGoUnderneath").unwrap_or_default(),
        }
    }
}

/// DCB type: `LadderAccessPointParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderAccessPointParams {
    /// `rungNumber` (Int32)
    #[serde(default)]
    pub rung_number: i32,
    /// `directionIncludesFront` (Boolean)
    #[serde(default)]
    pub direction_includes_front: bool,
    /// `frontEntryParams` (Class)
    #[serde(default)]
    pub front_entry_params: Option<Handle<LadderAccessClimbParams>>,
    /// `directionIncludesLeft` (Boolean)
    #[serde(default)]
    pub direction_includes_left: bool,
    /// `leftEntryParams` (Class)
    #[serde(default)]
    pub left_entry_params: Option<Handle<LadderAccessClimbParams>>,
    /// `directionIncludesRight` (Boolean)
    #[serde(default)]
    pub direction_includes_right: bool,
    /// `rightEntryParams` (Class)
    #[serde(default)]
    pub right_entry_params: Option<Handle<LadderAccessClimbParams>>,
}

impl Pooled for LadderAccessPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ladder.ladder_access_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ladder.ladder_access_point_params }
}

impl<'a> Extract<'a> for LadderAccessPointParams {
    const TYPE_NAME: &'static str = "LadderAccessPointParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rung_number: inst.get_i32("rungNumber").unwrap_or_default(),
            direction_includes_front: inst.get_bool("directionIncludesFront").unwrap_or_default(),
            front_entry_params: match inst.get("frontEntryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderAccessClimbParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            direction_includes_left: inst.get_bool("directionIncludesLeft").unwrap_or_default(),
            left_entry_params: match inst.get("leftEntryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderAccessClimbParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            direction_includes_right: inst.get_bool("directionIncludesRight").unwrap_or_default(),
            right_entry_params: match inst.get("rightEntryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LadderAccessClimbParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LadderComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderComponentParams {
    /// `height` (Single)
    #[serde(default)]
    pub height: f32,
    /// `userIgnoreStaticCollisions` (Boolean)
    #[serde(default)]
    pub user_ignore_static_collisions: bool,
    /// `slideable` (Boolean)
    #[serde(default)]
    pub slideable: bool,
    /// `autoMountable` (Boolean)
    #[serde(default)]
    pub auto_mountable: bool,
    /// `autoMountableRadiusParametersAtBottom` (Class)
    #[serde(default)]
    pub auto_mountable_radius_parameters_at_bottom: Option<Handle<AutoMountRadiusParams>>,
    /// `approachAngleAtBottom` (Single)
    #[serde(default)]
    pub approach_angle_at_bottom: f32,
    /// `facingAngleAtBottom` (Single)
    #[serde(default)]
    pub facing_angle_at_bottom: f32,
    /// `remountDelayAtBottom` (Single)
    #[serde(default)]
    pub remount_delay_at_bottom: f32,
    /// `autoMountableRadiusParametersAtTop` (Class)
    #[serde(default)]
    pub auto_mountable_radius_parameters_at_top: Option<Handle<AutoMountRadiusParams>>,
    /// `approachAngleAtTop` (Single)
    #[serde(default)]
    pub approach_angle_at_top: f32,
    /// `facingAngleAtTop` (Single)
    #[serde(default)]
    pub facing_angle_at_top: f32,
    /// `remountDelayAtTop` (Single)
    #[serde(default)]
    pub remount_delay_at_top: f32,
    /// `directionAtTopIncludesBack` (Boolean)
    #[serde(default)]
    pub direction_at_top_includes_back: bool,
    /// `topBackExitCollisionCheckOverride` (StrongPointer)
    #[serde(default)]
    pub top_back_exit_collision_check_override: Option<Handle<ExitCollisionCheckOverrideParams>>,
    /// `directionAtTopIncludesLeft` (Boolean)
    #[serde(default)]
    pub direction_at_top_includes_left: bool,
    /// `topLeftExitCollisionCheckOverride` (StrongPointer)
    #[serde(default)]
    pub top_left_exit_collision_check_override: Option<Handle<ExitCollisionCheckOverrideParams>>,
    /// `directionAtTopIncludesRight` (Boolean)
    #[serde(default)]
    pub direction_at_top_includes_right: bool,
    /// `topRightExitCollisionCheckOverride` (StrongPointer)
    #[serde(default)]
    pub top_right_exit_collision_check_override: Option<Handle<ExitCollisionCheckOverrideParams>>,
    /// `autoDismountOnHighestMidPointWhenNoTopAvailable` (Boolean)
    #[serde(default)]
    pub auto_dismount_on_highest_mid_point_when_no_top_available: bool,
    /// `midAccessPoints` (Class (array))
    #[serde(default)]
    pub mid_access_points: Vec<Handle<LadderAccessPointParams>>,
    /// `HACKUseParentZoneExitingLadderTopHACK` (Boolean)
    #[serde(default)]
    pub hackuse_parent_zone_exiting_ladder_top_hack: bool,
}

impl Pooled for LadderComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ladder.ladder_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ladder.ladder_component_params }
}

impl<'a> Extract<'a> for LadderComponentParams {
    const TYPE_NAME: &'static str = "LadderComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            height: inst.get_f32("height").unwrap_or_default(),
            user_ignore_static_collisions: inst.get_bool("userIgnoreStaticCollisions").unwrap_or_default(),
            slideable: inst.get_bool("slideable").unwrap_or_default(),
            auto_mountable: inst.get_bool("autoMountable").unwrap_or_default(),
            auto_mountable_radius_parameters_at_bottom: match inst.get("autoMountableRadiusParametersAtBottom") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AutoMountRadiusParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            approach_angle_at_bottom: inst.get_f32("approachAngleAtBottom").unwrap_or_default(),
            facing_angle_at_bottom: inst.get_f32("facingAngleAtBottom").unwrap_or_default(),
            remount_delay_at_bottom: inst.get_f32("remountDelayAtBottom").unwrap_or_default(),
            auto_mountable_radius_parameters_at_top: match inst.get("autoMountableRadiusParametersAtTop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AutoMountRadiusParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            approach_angle_at_top: inst.get_f32("approachAngleAtTop").unwrap_or_default(),
            facing_angle_at_top: inst.get_f32("facingAngleAtTop").unwrap_or_default(),
            remount_delay_at_top: inst.get_f32("remountDelayAtTop").unwrap_or_default(),
            direction_at_top_includes_back: inst.get_bool("directionAtTopIncludesBack").unwrap_or_default(),
            top_back_exit_collision_check_override: match inst.get("topBackExitCollisionCheckOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExitCollisionCheckOverrideParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            direction_at_top_includes_left: inst.get_bool("directionAtTopIncludesLeft").unwrap_or_default(),
            top_left_exit_collision_check_override: match inst.get("topLeftExitCollisionCheckOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExitCollisionCheckOverrideParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            direction_at_top_includes_right: inst.get_bool("directionAtTopIncludesRight").unwrap_or_default(),
            top_right_exit_collision_check_override: match inst.get("topRightExitCollisionCheckOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExitCollisionCheckOverrideParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_dismount_on_highest_mid_point_when_no_top_available: inst.get_bool("autoDismountOnHighestMidPointWhenNoTopAvailable").unwrap_or_default(),
            mid_access_points: inst.get_array("midAccessPoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LadderAccessPointParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LadderAccessPointParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            hackuse_parent_zone_exiting_ladder_top_hack: inst.get_bool("HACKUseParentZoneExitingLadderTopHACK").unwrap_or_default(),
        }
    }
}

