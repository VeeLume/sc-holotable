// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-actorzerogtraversalparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCZeroGLaunchParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCZeroGLaunchParams {
    /// `maxLaunchSpeed` (Single)
    #[serde(default)]
    pub max_launch_speed: f32,
    /// `launchRotationDuration` (Single)
    #[serde(default)]
    pub launch_rotation_duration: f32,
    /// `launchEdgeCheckRadius` (Single)
    #[serde(default)]
    pub launch_edge_check_radius: f32,
    /// `launchEdgeCheckDistance` (Single)
    #[serde(default)]
    pub launch_edge_check_distance: f32,
    /// `launchEdgeSurfaceHoverCheckRadius` (Single)
    #[serde(default)]
    pub launch_edge_surface_hover_check_radius: f32,
    /// `launchEdgeSurfaceHoverCheckDistance` (Single)
    #[serde(default)]
    pub launch_edge_surface_hover_check_distance: f32,
}

impl Pooled for SCZeroGLaunchParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorzerogtraversalparams.sczero_glaunch_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorzerogtraversalparams.sczero_glaunch_params }
}

impl<'a> Extract<'a> for SCZeroGLaunchParams {
    const TYPE_NAME: &'static str = "SCZeroGLaunchParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_launch_speed: inst.get_f32("maxLaunchSpeed").unwrap_or_default(),
            launch_rotation_duration: inst.get_f32("launchRotationDuration").unwrap_or_default(),
            launch_edge_check_radius: inst.get_f32("launchEdgeCheckRadius").unwrap_or_default(),
            launch_edge_check_distance: inst.get_f32("launchEdgeCheckDistance").unwrap_or_default(),
            launch_edge_surface_hover_check_radius: inst.get_f32("launchEdgeSurfaceHoverCheckRadius").unwrap_or_default(),
            launch_edge_surface_hover_check_distance: inst.get_f32("launchEdgeSurfaceHoverCheckDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCDefaultZeroGTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDefaultZeroGTraversalParams {
    /// `zeroGLaunchParams` (Class)
    #[serde(default)]
    pub zero_glaunch_params: Option<Handle<SCZeroGLaunchParams>>,
}

impl Pooled for SCDefaultZeroGTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorzerogtraversalparams.scdefault_zero_gtraversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorzerogtraversalparams.scdefault_zero_gtraversal_params }
}

impl<'a> Extract<'a> for SCDefaultZeroGTraversalParams {
    const TYPE_NAME: &'static str = "SCDefaultZeroGTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zero_glaunch_params: match inst.get("zeroGLaunchParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCZeroGLaunchParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCZeroGLaunchParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCOptionalZeroGTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCOptionalZeroGTraversalParams {
    /// `activationTag` (Reference)
    #[serde(default)]
    pub activation_tag: Option<CigGuid>,
    /// `zeroGLaunchParams` (StrongPointer)
    #[serde(default)]
    pub zero_glaunch_params: Option<Handle<SCZeroGLaunchParams>>,
}

impl Pooled for SCOptionalZeroGTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorzerogtraversalparams.scoptional_zero_gtraversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorzerogtraversalparams.scoptional_zero_gtraversal_params }
}

impl<'a> Extract<'a> for SCOptionalZeroGTraversalParams {
    const TYPE_NAME: &'static str = "SCOptionalZeroGTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            activation_tag: inst.get("activationTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            zero_glaunch_params: match inst.get("zeroGLaunchParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCZeroGLaunchParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCZeroGLaunchParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorZeroGTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorZeroGTraversalParams {
    /// `defaultZeroGTraversalParams` (Class)
    #[serde(default)]
    pub default_zero_gtraversal_params: Option<Handle<SCDefaultZeroGTraversalParams>>,
    /// `optionalZeroGTraversalParams` (Class (array))
    #[serde(default)]
    pub optional_zero_gtraversal_params: Vec<Handle<SCOptionalZeroGTraversalParams>>,
}

impl Pooled for ActorZeroGTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorzerogtraversalparams.actor_zero_gtraversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorzerogtraversalparams.actor_zero_gtraversal_params }
}

impl<'a> Extract<'a> for ActorZeroGTraversalParams {
    const TYPE_NAME: &'static str = "ActorZeroGTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_zero_gtraversal_params: match inst.get("defaultZeroGTraversalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCDefaultZeroGTraversalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCDefaultZeroGTraversalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            optional_zero_gtraversal_params: inst.get_array("optionalZeroGTraversalParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCOptionalZeroGTraversalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCOptionalZeroGTraversalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

