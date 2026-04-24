// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-carryables`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SSequencerChangeStanceCarryableTaskParams`
/// Inherits from: `SSequencerCarryableTaskParams`
pub struct SSequencerChangeStanceCarryableTaskParams {
    /// `stance` (EnumChoice)
    pub stance: AgentStance,
}

impl Pooled for SSequencerChangeStanceCarryableTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_carryables
            .ssequencer_change_stance_carryable_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_carryables
            .ssequencer_change_stance_carryable_task_params
    }
}

impl<'a> Extract<'a> for SSequencerChangeStanceCarryableTaskParams {
    const TYPE_NAME: &'static str = "SSequencerChangeStanceCarryableTaskParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stance: AgentStance::from_dcb_str(inst.get_str("stance").unwrap_or("")),
        }
    }
}

/// DCB type: `SSimulationParamsSpringEllipsoid`
/// Inherits from: `SSimulationParamsBase`
pub struct SSimulationParamsSpringEllipsoid {
    /// `UseSimulation` (Boolean)
    pub use_simulation: bool,
    /// `UseDebugSetup` (Boolean)
    pub use_debug_setup: bool,
    /// `UseDebugText` (Boolean)
    pub use_debug_text: bool,
    /// `UseRedirect` (Boolean)
    pub use_redirect: bool,
    /// `SimFPS` (Byte)
    pub sim_fps: u32,
    /// `PivotOffset` (Class)
    pub pivot_offset: Option<Handle<Vec3>>,
    /// `Mass` (Single)
    pub mass: f32,
    /// `Gravity` (Single)
    pub gravity: f32,
    /// `Damping` (Single)
    pub damping: f32,
    /// `Stiffness` (Single)
    pub stiffness: f32,
    /// `StiffnessTarget` (Class)
    pub stiffness_target: Option<Handle<Vec3>>,
    /// `DiskRadius` (Single)
    pub disk_radius: f32,
    /// `SphereScale` (Class)
    pub sphere_scale: Option<Handle<Vec2>>,
    /// `DiskRotation` (Class)
    pub disk_rotation: Option<Handle<Vec2>>,
    /// `ProjectionType` (EnumChoice)
    pub projection_type: EProjectionSelection3,
    /// `Radius` (Single)
    pub radius: f32,
    /// `AvailableCollisionProxies` (EnumChoice (array))
    pub available_collision_proxies: Vec<EAuxiliaryProxy>,
}

impl Pooled for SSimulationParamsSpringEllipsoid {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_carryables
            .ssimulation_params_spring_ellipsoid
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_carryables
            .ssimulation_params_spring_ellipsoid
    }
}

impl<'a> Extract<'a> for SSimulationParamsSpringEllipsoid {
    const TYPE_NAME: &'static str = "SSimulationParamsSpringEllipsoid";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_simulation: inst.get_bool("UseSimulation").unwrap_or_default(),
            use_debug_setup: inst.get_bool("UseDebugSetup").unwrap_or_default(),
            use_debug_text: inst.get_bool("UseDebugText").unwrap_or_default(),
            use_redirect: inst.get_bool("UseRedirect").unwrap_or_default(),
            sim_fps: inst.get_u32("SimFPS").unwrap_or_default(),
            pivot_offset: match inst.get("PivotOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            mass: inst.get_f32("Mass").unwrap_or_default(),
            gravity: inst.get_f32("Gravity").unwrap_or_default(),
            damping: inst.get_f32("Damping").unwrap_or_default(),
            stiffness: inst.get_f32("Stiffness").unwrap_or_default(),
            stiffness_target: match inst.get("StiffnessTarget") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            disk_radius: inst.get_f32("DiskRadius").unwrap_or_default(),
            sphere_scale: match inst.get("SphereScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            disk_rotation: match inst.get("DiskRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            projection_type: EProjectionSelection3::from_dcb_str(
                inst.get_str("ProjectionType").unwrap_or(""),
            ),
            radius: inst.get_f32("Radius").unwrap_or_default(),
            available_collision_proxies: inst
                .get_array("AvailableCollisionProxies")
                .map(|arr| {
                    arr.filter_map(|v| v.as_str().map(EAuxiliaryProxy::from_dcb_str))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}
