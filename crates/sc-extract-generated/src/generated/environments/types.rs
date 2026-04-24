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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `AsteroidProcedural`
pub struct AsteroidProcedural {
    /// `minScale` (Single)
    pub min_scale: f32,
    /// `maxScale` (Single)
    pub max_scale: f32,
    /// `minRotationSpeed` (Single)
    pub min_rotation_speed: f32,
    /// `maxRotationSpeed` (Single)
    pub max_rotation_speed: f32,
    /// `distributionA` (Single)
    pub distribution_a: f32,
    /// `distributionB` (Single)
    pub distribution_b: f32,
    /// `tint` (Class)
    pub tint: Option<Handle<RGB>>,
    /// `mesh` (Class)
    pub mesh: Option<Handle<GlobalResourceCGF>>,
    /// `material` (Class)
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
                _ => None,
            },
            mesh: match inst.get("mesh") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceCGF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AsteroidFieldComposition`
pub struct AsteroidFieldComposition {
    /// `fogDensity` (Single)
    pub fog_density: f32,
    /// `fogNoiseScale` (Single)
    pub fog_noise_scale: f32,
    /// `fogAlbedo` (Class)
    pub fog_albedo: Option<Handle<RGB>>,
    /// `asteroids` (Class (array))
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
                _ => None,
            },
            asteroids: inst.get_array("asteroids")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AsteroidProcedural>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AsteroidProcedural>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

