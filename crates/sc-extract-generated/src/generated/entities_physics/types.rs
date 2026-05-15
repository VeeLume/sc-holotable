// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-physics`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `DeadBodyParams`
/// Inherits from: `DataForgeComponentParams`
pub struct DeadBodyParams {
    /// `kinematic` (Boolean)
    pub kinematic: bool,
}

impl Pooled for DeadBodyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_physics.dead_body_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_physics.dead_body_params
    }
}

impl<'a> Extract<'a> for DeadBodyParams {
    const TYPE_NAME: &'static str = "DeadBodyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            kinematic: inst.get_bool("kinematic").unwrap_or_default(),
        }
    }
}

/// DCB type: `SRopeProxyParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SRopeProxyParams {}

impl Pooled for SRopeProxyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_physics.srope_proxy_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_physics.srope_proxy_params
    }
}

impl<'a> Extract<'a> for SRopeProxyParams {
    const TYPE_NAME: &'static str = "SRopeProxyParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `WindAreaParams`
/// Inherits from: `DataForgeComponentParams`
pub struct WindAreaParams {
    /// `defaultActive` (Boolean)
    pub default_active: bool,
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
    /// `ellipsoidal` (Boolean)
    pub ellipsoidal: bool,
    /// `falloffInner` (Single)
    pub falloff_inner: f32,
    /// `direction` (Class)
    pub direction: Option<Handle<Vec3>>,
    /// `speed` (Single)
    pub speed: f32,
    /// `airResistance` (Single)
    pub air_resistance: f32,
    /// `airDensity` (Single)
    pub air_density: f32,
}

impl Pooled for WindAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_physics.wind_area_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_physics.wind_area_params
    }
}

impl<'a> Extract<'a> for WindAreaParams {
    const TYPE_NAME: &'static str = "WindAreaParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_active: inst.get_bool("defaultActive").unwrap_or_default(),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            ellipsoidal: inst.get_bool("ellipsoidal").unwrap_or_default(),
            falloff_inner: inst.get_f32("falloffInner").unwrap_or_default(),
            direction: match inst.get("direction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            speed: inst.get_f32("speed").unwrap_or_default(),
            air_resistance: inst.get_f32("airResistance").unwrap_or_default(),
            air_density: inst.get_f32("airDensity").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityComponentConstraintPartnerComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentConstraintPartnerComponentParams {}

impl Pooled for SEntityComponentConstraintPartnerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_physics
            .sentity_component_constraint_partner_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_physics
            .sentity_component_constraint_partner_component_params
    }
}

impl<'a> Extract<'a> for SEntityComponentConstraintPartnerComponentParams {
    const TYPE_NAME: &'static str = "SEntityComponentConstraintPartnerComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SLayerEntitiesGroupComponentParams`
/// Inherits from: `SEntitiesGroupComponentParams`
pub struct SLayerEntitiesGroupComponentParams {
    /// `allowPhysicsChange` (Boolean)
    pub allow_physics_change: bool,
    /// `allowStreaming` (Boolean)
    pub allow_streaming: bool,
    /// `useLayerData` (Boolean)
    pub use_layer_data: bool,
    /// `useMovablesInOCData` (Boolean)
    pub use_movables_in_ocdata: bool,
    /// `useUnreferencedEntities` (Boolean)
    pub use_unreferenced_entities: bool,
    /// `startStreamedOut` (Boolean)
    pub start_streamed_out: bool,
    /// `startHidden` (Boolean)
    pub start_hidden: bool,
    /// `isMaster` (Boolean)
    pub is_master: bool,
}

impl Pooled for SLayerEntitiesGroupComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_physics
            .slayer_entities_group_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_physics
            .slayer_entities_group_component_params
    }
}

impl<'a> Extract<'a> for SLayerEntitiesGroupComponentParams {
    const TYPE_NAME: &'static str = "SLayerEntitiesGroupComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow_physics_change: inst.get_bool("allowPhysicsChange").unwrap_or_default(),
            allow_streaming: inst.get_bool("allowStreaming").unwrap_or_default(),
            use_layer_data: inst.get_bool("useLayerData").unwrap_or_default(),
            use_movables_in_ocdata: inst.get_bool("useMovablesInOCData").unwrap_or_default(),
            use_unreferenced_entities: inst.get_bool("useUnreferencedEntities").unwrap_or_default(),
            start_streamed_out: inst.get_bool("startStreamedOut").unwrap_or_default(),
            start_hidden: inst.get_bool("startHidden").unwrap_or_default(),
            is_master: inst.get_bool("isMaster").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityArticulatedPhysicsControllerParams`
/// Inherits from: `SEntityBasePhysicsControllerParams`
pub struct SEntityArticulatedPhysicsControllerParams {
    /// `Mass` (Single)
    pub mass: f32,
    /// `compoundingAllowed` (Boolean)
    pub compounding_allowed: bool,
    /// `breakableParams` (StrongPointer)
    pub breakable_params: Option<Handle<SBreakablePhysicsParams>>,
    /// `gameCollisionClass` (StrongPointer)
    pub game_collision_class: Option<Handle<SGameCollisionClass>>,
    /// `spawnBoxScale` (Single)
    pub spawn_box_scale: f32,
    /// `IsRagdoll` (Boolean)
    pub is_ragdoll: bool,
    /// `Stiffness` (Single)
    pub stiffness: f32,
    /// `MaxTimeStep` (Single)
    pub max_time_step: f32,
    /// `MaxLoggedCollisions` (Int32)
    pub max_logged_collisions: i32,
    /// `SleepSpeed` (Single)
    pub sleep_speed: f32,
    /// `Damping` (Single)
    pub damping: f32,
    /// `DampingFreefall` (Single)
    pub damping_freefall: f32,
    /// `LyingModeNColls` (Int32)
    pub lying_mode_ncolls: i32,
    /// `LyingDamping` (Single)
    pub lying_damping: f32,
    /// `LyingSleepSpeed` (Single)
    pub lying_sleep_speed: f32,
    /// `Resting` (Boolean)
    pub resting: bool,
    /// `aiNavigationType` (EnumChoice)
    pub ai_navigation_type: EAINavigationGeneration,
}

impl Pooled for SEntityArticulatedPhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_physics
            .sentity_articulated_physics_controller_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_physics
            .sentity_articulated_physics_controller_params
    }
}

impl<'a> Extract<'a> for SEntityArticulatedPhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntityArticulatedPhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mass: inst.get_f32("Mass").unwrap_or_default(),
            compounding_allowed: inst.get_bool("compoundingAllowed").unwrap_or_default(),
            breakable_params: match inst.get("breakableParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SBreakablePhysicsParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            game_collision_class: match inst.get("gameCollisionClass") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SGameCollisionClass>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            spawn_box_scale: inst.get_f32("spawnBoxScale").unwrap_or_default(),
            is_ragdoll: inst.get_bool("IsRagdoll").unwrap_or_default(),
            stiffness: inst.get_f32("Stiffness").unwrap_or_default(),
            max_time_step: inst.get_f32("MaxTimeStep").unwrap_or_default(),
            max_logged_collisions: inst.get_i32("MaxLoggedCollisions").unwrap_or_default(),
            sleep_speed: inst.get_f32("SleepSpeed").unwrap_or_default(),
            damping: inst.get_f32("Damping").unwrap_or_default(),
            damping_freefall: inst.get_f32("DampingFreefall").unwrap_or_default(),
            lying_mode_ncolls: inst.get_i32("LyingModeNColls").unwrap_or_default(),
            lying_damping: inst.get_f32("LyingDamping").unwrap_or_default(),
            lying_sleep_speed: inst.get_f32("LyingSleepSpeed").unwrap_or_default(),
            resting: inst.get_bool("Resting").unwrap_or_default(),
            ai_navigation_type: EAINavigationGeneration::from_dcb_str(
                inst.get_str("aiNavigationType").unwrap_or(""),
            ),
        }
    }
}
