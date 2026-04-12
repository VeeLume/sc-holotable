// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `environments`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `AsteroidProcedural`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidProcedural {
    /// `minScale` (Single)
    #[serde(default)]
    pub min_scale: f32,
    /// `maxScale` (Single)
    #[serde(default)]
    pub max_scale: f32,
    /// `minRotationSpeed` (Single)
    #[serde(default)]
    pub min_rotation_speed: f32,
    /// `maxRotationSpeed` (Single)
    #[serde(default)]
    pub max_rotation_speed: f32,
    /// `distributionA` (Single)
    #[serde(default)]
    pub distribution_a: f32,
    /// `distributionB` (Single)
    #[serde(default)]
    pub distribution_b: f32,
    /// `tint` (Class)
    #[serde(default)]
    pub tint: Option<Handle<RGB>>,
    /// `mesh` (Class)
    #[serde(default)]
    pub mesh: Option<Handle<GlobalResourceCGF>>,
    /// `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for AsteroidProcedural {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.environments.asteroid_procedural }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.environments.asteroid_procedural }
}

impl<'a> Extract<'a> for AsteroidProcedural {
    const TYPE_NAME: &'static str = "AsteroidProcedural";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_scale: inst.get_f32("minScale").unwrap_or_default(),
            max_scale: inst.get_f32("maxScale").unwrap_or_default(),
            min_rotation_speed: inst.get_f32("minRotationSpeed").unwrap_or_default(),
            max_rotation_speed: inst.get_f32("maxRotationSpeed").unwrap_or_default(),
            distribution_a: inst.get_f32("distributionA").unwrap_or_default(),
            distribution_b: inst.get_f32("distributionB").unwrap_or_default(),
            tint: match inst.get("tint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mesh: match inst.get("mesh") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceCGF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceCGF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AsteroidFieldComposition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidFieldComposition {
    /// `fogDensity` (Single)
    #[serde(default)]
    pub fog_density: f32,
    /// `fogNoiseScale` (Single)
    #[serde(default)]
    pub fog_noise_scale: f32,
    /// `fogAlbedo` (Class)
    #[serde(default)]
    pub fog_albedo: Option<Handle<RGB>>,
    /// `asteroids` (Class (array))
    #[serde(default)]
    pub asteroids: Vec<Handle<AsteroidProcedural>>,
}

impl Pooled for AsteroidFieldComposition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.environments.asteroid_field_composition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.environments.asteroid_field_composition }
}

impl<'a> Extract<'a> for AsteroidFieldComposition {
    const TYPE_NAME: &'static str = "AsteroidFieldComposition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fog_density: inst.get_f32("fogDensity").unwrap_or_default(),
            fog_noise_scale: inst.get_f32("fogNoiseScale").unwrap_or_default(),
            fog_albedo: match inst.get("fogAlbedo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            asteroids: inst.get_array("asteroids")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AsteroidProcedural>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AsteroidProcedural>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalResourceCGF`
/// Inherits from: `GlobalResourceBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResourceCGF {
    /// `path` (String)
    #[serde(default)]
    pub path: String,
}

impl Pooled for GlobalResourceCGF {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.environments.global_resource_cgf }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.environments.global_resource_cgf }
}

impl<'a> Extract<'a> for GlobalResourceCGF {
    const TYPE_NAME: &'static str = "GlobalResourceCGF";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            path: inst.get_str("path").map(String::from).unwrap_or_default(),
        }
    }
}

