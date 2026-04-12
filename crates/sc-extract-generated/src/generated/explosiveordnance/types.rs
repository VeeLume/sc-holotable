// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `explosiveordnance`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ExplosiveOrdnancePingVFX`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosiveOrdnancePingVFX {
    /// `pingSphereGeometry` (Class)
    #[serde(default)]
    pub ping_sphere_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `pingMaterial` (Class)
    #[serde(default)]
    pub ping_material: Option<Handle<GlobalResourceMaterial>>,
    /// `pingColor` (Class)
    #[serde(default)]
    pub ping_color: Option<Handle<RGB>>,
    /// `pingBrightness` (Single)
    #[serde(default)]
    pub ping_brightness: f32,
}

impl Pooled for ExplosiveOrdnancePingVFX {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.explosiveordnance.explosive_ordnance_ping_vfx }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.explosiveordnance.explosive_ordnance_ping_vfx }
}

impl<'a> Extract<'a> for ExplosiveOrdnancePingVFX {
    const TYPE_NAME: &'static str = "ExplosiveOrdnancePingVFX";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ping_sphere_geometry: match inst.get("pingSphereGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_material: match inst.get("pingMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_color: match inst.get("pingColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_brightness: inst.get_f32("pingBrightness").unwrap_or_default(),
        }
    }
}

/// DCB type: `ExplosiveOrdnancePingGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosiveOrdnancePingGlobalParams {
    /// `vfxGhostPingParams` (Class)
    #[serde(default)]
    pub vfx_ghost_ping_params: Option<Handle<ExplosiveOrdnancePingVFX>>,
    /// `vfxDesiredPingParams` (Class)
    #[serde(default)]
    pub vfx_desired_ping_params: Option<Handle<ExplosiveOrdnancePingVFX>>,
    /// `vfxPredictedPingParams` (Class)
    #[serde(default)]
    pub vfx_predicted_ping_params: Option<Handle<ExplosiveOrdnancePingVFX>>,
}

impl Pooled for ExplosiveOrdnancePingGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.explosiveordnance.explosive_ordnance_ping_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.explosiveordnance.explosive_ordnance_ping_global_params }
}

impl<'a> Extract<'a> for ExplosiveOrdnancePingGlobalParams {
    const TYPE_NAME: &'static str = "ExplosiveOrdnancePingGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vfx_ghost_ping_params: match inst.get("vfxGhostPingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vfx_desired_ping_params: match inst.get("vfxDesiredPingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vfx_predicted_ping_params: match inst.get("vfxPredictedPingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

