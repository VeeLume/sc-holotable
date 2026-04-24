// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-actormovables`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `BuildModeKioskProviderComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct BuildModeKioskProviderComponentParams {}

impl Pooled for BuildModeKioskProviderComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_actormovables
            .build_mode_kiosk_provider_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_actormovables
            .build_mode_kiosk_provider_component_params
    }
}

impl<'a> Extract<'a> for BuildModeKioskProviderComponentParams {
    const TYPE_NAME: &'static str = "BuildModeKioskProviderComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `InteractionConditionHoverPowerStageEqual`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionHoverPowerStageEqual {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
    /// `powerStage` (EnumChoice)
    pub power_stage: EHoverPowerStage,
}

impl Pooled for InteractionConditionHoverPowerStageEqual {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_actormovables
            .interaction_condition_hover_power_stage_equal
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_actormovables
            .interaction_condition_hover_power_stage_equal
    }
}

impl<'a> Extract<'a> for InteractionConditionHoverPowerStageEqual {
    const TYPE_NAME: &'static str = "InteractionConditionHoverPowerStageEqual";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ConditionDisplayParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            power_stage: EHoverPowerStage::from_dcb_str(inst.get_str("powerStage").unwrap_or("")),
        }
    }
}

/// DCB type: `StateModifierHoverPowerStage`
/// Inherits from: `SStateModifier`
pub struct StateModifierHoverPowerStage {
    /// `poweredOff` (WeakPointer)
    pub powered_off: Option<Handle<SInteractionState>>,
    /// `poweringOff` (WeakPointer)
    pub powering_off: Option<Handle<SInteractionState>>,
    /// `poweringOn` (WeakPointer)
    pub powering_on: Option<Handle<SInteractionState>>,
    /// `poweredOn` (WeakPointer)
    pub powered_on: Option<Handle<SInteractionState>>,
}

impl Pooled for StateModifierHoverPowerStage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_actormovables
            .state_modifier_hover_power_stage
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_actormovables
            .state_modifier_hover_power_stage
    }
}

impl<'a> Extract<'a> for StateModifierHoverPowerStage {
    const TYPE_NAME: &'static str = "StateModifierHoverPowerStage";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            powered_off: match inst.get("poweredOff") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            powering_off: match inst.get("poweringOff") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            powering_on: match inst.get("poweringOn") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            powered_on: match inst.get("poweredOn") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SStateModifierMovableMover`
/// Inherits from: `SStateModifier`
pub struct SStateModifierMovableMover {
    /// `hasMoverState` (WeakPointer)
    pub has_mover_state: Option<Handle<SInteractionState>>,
    /// `noMoverState` (WeakPointer)
    pub no_mover_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SStateModifierMovableMover {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_actormovables
            .sstate_modifier_movable_mover
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_actormovables
            .sstate_modifier_movable_mover
    }
}

impl<'a> Extract<'a> for SStateModifierMovableMover {
    const TYPE_NAME: &'static str = "SStateModifierMovableMover";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            has_mover_state: match inst.get("hasMoverState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            no_mover_state: match inst.get("noMoverState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityHoverPhysicsPartialParams`
pub struct SEntityHoverPhysicsPartialParams {
    /// `desiredHoverHeight` (Single)
    pub desired_hover_height: f32,
    /// `desiredHoverMinHeight` (Single)
    pub desired_hover_min_height: f32,
    /// `alignToSurface` (Single)
    pub align_to_surface: f32,
    /// `alignToSurfaceMaxAngle` (Single)
    pub align_to_surface_max_angle: f32,
    /// `alignToSurfaceEasingStartAngle` (Single)
    pub align_to_surface_easing_start_angle: f32,
    /// `alignToSurfaceEasingEndAngle` (Single)
    pub align_to_surface_easing_end_angle: f32,
    /// `maxGroundSearchDepth` (Single)
    pub max_ground_search_depth: f32,
    /// `maxClimbHeight` (Single)
    pub max_climb_height: f32,
    /// `desiredMaxFallSpeed` (Single)
    pub desired_max_fall_speed: f32,
    /// `desiredFallSpeedEasingStartDepth` (Single)
    pub desired_fall_speed_easing_start_depth: f32,
    /// `desiredFallSpeedEasingPower` (Single)
    pub desired_fall_speed_easing_power: f32,
    /// `desiredMaxRaiseSpeed` (Single)
    pub desired_max_raise_speed: f32,
    /// `desiredRaiseSpeedEasingStartHeight` (Single)
    pub desired_raise_speed_easing_start_height: f32,
    /// `desiredRaiseSpeedEasingPower` (Single)
    pub desired_raise_speed_easing_power: f32,
    /// `desiredMaxRotationalAdjustmentSpeed` (Single)
    pub desired_max_rotational_adjustment_speed: f32,
    /// `desiredAngularSpeedEasingStartAngle` (Single)
    pub desired_angular_speed_easing_start_angle: f32,
    /// `desiredAngularSpeedEasingPower` (Single)
    pub desired_angular_speed_easing_power: f32,
    /// `lateralMoveStopMaxSpeed` (Single)
    pub lateral_move_stop_max_speed: f32,
    /// `yawStopMaxSpeed` (Single)
    pub yaw_stop_max_speed: f32,
}

impl Pooled for SEntityHoverPhysicsPartialParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_actormovables
            .sentity_hover_physics_partial_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_actormovables
            .sentity_hover_physics_partial_params
    }
}

impl<'a> Extract<'a> for SEntityHoverPhysicsPartialParams {
    const TYPE_NAME: &'static str = "SEntityHoverPhysicsPartialParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            desired_hover_height: inst.get_f32("desiredHoverHeight").unwrap_or_default(),
            desired_hover_min_height: inst.get_f32("desiredHoverMinHeight").unwrap_or_default(),
            align_to_surface: inst.get_f32("alignToSurface").unwrap_or_default(),
            align_to_surface_max_angle: inst.get_f32("alignToSurfaceMaxAngle").unwrap_or_default(),
            align_to_surface_easing_start_angle: inst
                .get_f32("alignToSurfaceEasingStartAngle")
                .unwrap_or_default(),
            align_to_surface_easing_end_angle: inst
                .get_f32("alignToSurfaceEasingEndAngle")
                .unwrap_or_default(),
            max_ground_search_depth: inst.get_f32("maxGroundSearchDepth").unwrap_or_default(),
            max_climb_height: inst.get_f32("maxClimbHeight").unwrap_or_default(),
            desired_max_fall_speed: inst.get_f32("desiredMaxFallSpeed").unwrap_or_default(),
            desired_fall_speed_easing_start_depth: inst
                .get_f32("desiredFallSpeedEasingStartDepth")
                .unwrap_or_default(),
            desired_fall_speed_easing_power: inst
                .get_f32("desiredFallSpeedEasingPower")
                .unwrap_or_default(),
            desired_max_raise_speed: inst.get_f32("desiredMaxRaiseSpeed").unwrap_or_default(),
            desired_raise_speed_easing_start_height: inst
                .get_f32("desiredRaiseSpeedEasingStartHeight")
                .unwrap_or_default(),
            desired_raise_speed_easing_power: inst
                .get_f32("desiredRaiseSpeedEasingPower")
                .unwrap_or_default(),
            desired_max_rotational_adjustment_speed: inst
                .get_f32("desiredMaxRotationalAdjustmentSpeed")
                .unwrap_or_default(),
            desired_angular_speed_easing_start_angle: inst
                .get_f32("desiredAngularSpeedEasingStartAngle")
                .unwrap_or_default(),
            desired_angular_speed_easing_power: inst
                .get_f32("desiredAngularSpeedEasingPower")
                .unwrap_or_default(),
            lateral_move_stop_max_speed: inst
                .get_f32("lateralMoveStopMaxSpeed")
                .unwrap_or_default(),
            yaw_stop_max_speed: inst.get_f32("yawStopMaxSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityHoverPhysicsControllerParams`
pub struct SEntityHoverPhysicsControllerParams {
    /// `interactionActivate` (WeakPointer)
    pub interaction_activate: Option<Handle<SSharedInteractionParams>>,
    /// `interactionDeactivate` (WeakPointer)
    pub interaction_deactivate: Option<Handle<SSharedInteractionParams>>,
    /// `activateHoverByDefault` (Boolean)
    pub activate_hover_by_default: bool,
    /// `activateHoverOnGripAttached` (Boolean)
    pub activate_hover_on_grip_attached: bool,
    /// `deactivateHoverOnGripDetached` (Boolean)
    pub deactivate_hover_on_grip_detached: bool,
    /// `blendInDuration` (Single)
    pub blend_in_duration: f32,
    /// `blendOutDuration` (Single)
    pub blend_out_duration: f32,
    /// `powerOffDesiredHoverHeight` (Single)
    pub power_off_desired_hover_height: f32,
    /// `maxHoverMass` (Single)
    pub max_hover_mass: f32,
    /// `maxRotationalAdjustmentTorque` (Single)
    pub max_rotational_adjustment_torque: f32,
    /// `basePlaneHalfWidth` (Single)
    pub base_plane_half_width: f32,
    /// `basePlaneHalfLength` (Single)
    pub base_plane_half_length: f32,
    /// `basePlaneOriginOffset` (Class)
    pub base_plane_origin_offset: Option<Handle<Vec3>>,
    /// `hoverPhysicsIdle` (Class)
    pub hover_physics_idle: Option<Handle<SEntityHoverPhysicsPartialParams>>,
    /// `hoverPhysicsGripAttached` (Class)
    pub hover_physics_grip_attached: Option<Handle<SEntityHoverPhysicsPartialParams>>,
}

impl Pooled for SEntityHoverPhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_actormovables
            .sentity_hover_physics_controller_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_actormovables
            .sentity_hover_physics_controller_params
    }
}

impl<'a> Extract<'a> for SEntityHoverPhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntityHoverPhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interaction_activate: match inst.get("interactionActivate") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            interaction_deactivate: match inst.get("interactionDeactivate") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            activate_hover_by_default: inst.get_bool("activateHoverByDefault").unwrap_or_default(),
            activate_hover_on_grip_attached: inst
                .get_bool("activateHoverOnGripAttached")
                .unwrap_or_default(),
            deactivate_hover_on_grip_detached: inst
                .get_bool("deactivateHoverOnGripDetached")
                .unwrap_or_default(),
            blend_in_duration: inst.get_f32("blendInDuration").unwrap_or_default(),
            blend_out_duration: inst.get_f32("blendOutDuration").unwrap_or_default(),
            power_off_desired_hover_height: inst
                .get_f32("powerOffDesiredHoverHeight")
                .unwrap_or_default(),
            max_hover_mass: inst.get_f32("maxHoverMass").unwrap_or_default(),
            max_rotational_adjustment_torque: inst
                .get_f32("maxRotationalAdjustmentTorque")
                .unwrap_or_default(),
            base_plane_half_width: inst.get_f32("basePlaneHalfWidth").unwrap_or_default(),
            base_plane_half_length: inst.get_f32("basePlaneHalfLength").unwrap_or_default(),
            base_plane_origin_offset: match inst.get("basePlaneOriginOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            hover_physics_idle: match inst.get("hoverPhysicsIdle") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SEntityHoverPhysicsPartialParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            hover_physics_grip_attached: match inst.get("hoverPhysicsGripAttached") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SEntityHoverPhysicsPartialParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityHoverPhysicsControllerComponentParams`
/// Inherits from: `SEntityPhysicsControllerParams`
pub struct SEntityHoverPhysicsControllerComponentParams {
    /// `PhysType` (StrongPointer)
    pub phys_type: Option<SEntityBasePhysicsControllerParamsPtr>,
    /// `hover` (Class)
    pub hover: Option<Handle<SEntityHoverPhysicsControllerParams>>,
}

impl Pooled for SEntityHoverPhysicsControllerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_actormovables
            .sentity_hover_physics_controller_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_actormovables
            .sentity_hover_physics_controller_component_params
    }
}

impl<'a> Extract<'a> for SEntityHoverPhysicsControllerComponentParams {
    const TYPE_NAME: &'static str = "SEntityHoverPhysicsControllerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            phys_type: match inst.get("PhysType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SEntityBasePhysicsControllerParamsPtr::from_ref(b, r))
                }
                _ => None,
            },
            hover: match inst.get("hover") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SEntityHoverPhysicsControllerParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
