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

/// DCB type: `AsteroidProcedural`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidProcedural {
    /// DCB field: `minScale` (Single)
    #[serde(default)]
    pub min_scale: f32,
    /// DCB field: `maxScale` (Single)
    #[serde(default)]
    pub max_scale: f32,
    /// DCB field: `minRotationSpeed` (Single)
    #[serde(default)]
    pub min_rotation_speed: f32,
    /// DCB field: `maxRotationSpeed` (Single)
    #[serde(default)]
    pub max_rotation_speed: f32,
    /// DCB field: `distributionA` (Single)
    #[serde(default)]
    pub distribution_a: f32,
    /// DCB field: `distributionB` (Single)
    #[serde(default)]
    pub distribution_b: f32,
    /// DCB field: `tint` (Class)
    #[serde(default)]
    pub tint: Option<Handle<RGB>>,
    /// DCB field: `mesh` (Class)
    #[serde(default)]
    pub mesh: Option<Handle<GlobalResourceCGF>>,
    /// DCB field: `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for AsteroidProcedural {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_as.asteroid_procedural }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_as.asteroid_procedural }
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
    /// DCB field: `fogDensity` (Single)
    #[serde(default)]
    pub fog_density: f32,
    /// DCB field: `fogNoiseScale` (Single)
    #[serde(default)]
    pub fog_noise_scale: f32,
    /// DCB field: `fogAlbedo` (Class)
    #[serde(default)]
    pub fog_albedo: Option<Handle<RGB>>,
    /// DCB field: `asteroids` (Class (array))
    #[serde(default)]
    pub asteroids: Vec<Handle<AsteroidProcedural>>,
}

impl Pooled for AsteroidFieldComposition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_as.asteroid_field_composition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_as.asteroid_field_composition }
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

/// DCB type: `AsteroidStateTemplateInternal`
///
/// Inherits from: `AsteroidState` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidStateTemplateInternal {
    /// DCB field: `debrisDensityMod` (EnumChoice)
    #[serde(default)]
    pub debris_density_mod: String,
    /// DCB field: `debrisDensity` (Single)
    #[serde(default)]
    pub debris_density: f32,
}

impl Pooled for AsteroidStateTemplateInternal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_as.asteroid_state_template_internal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_as.asteroid_state_template_internal }
}

impl<'a> Extract<'a> for AsteroidStateTemplateInternal {
    const TYPE_NAME: &'static str = "AsteroidStateTemplateInternal";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debris_density_mod: inst.get_str("debrisDensityMod").map(String::from).unwrap_or_default(),
            debris_density: inst.get_f32("debrisDensity").unwrap_or_default(),
        }
    }
}

/// DCB type: `AsteroidStateTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidStateTemplate {
    /// DCB field: `state` (Class)
    #[serde(default)]
    pub state: Option<Handle<AsteroidStateTemplateInternal>>,
}

impl Pooled for AsteroidStateTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_as.asteroid_state_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_as.asteroid_state_template }
}

impl<'a> Extract<'a> for AsteroidStateTemplate {
    const TYPE_NAME: &'static str = "AsteroidStateTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: match inst.get("state") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AsteroidStateTemplateInternal>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AsteroidStateTemplateInternal>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AsteroidBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidBehavior {
    /// DCB field: `vehicleEffects` (Class)
    #[serde(default)]
    pub vehicle_effects: Option<Handle<Behavior_VehicleEffectParams>>,
    /// DCB field: `weather` (StrongPointer)
    #[serde(default)]
    pub weather: Option<Handle<AsteroidBehavior_WeatherParams>>,
    /// DCB field: `asteroidEnvironmentTag` (Reference)
    #[serde(default)]
    pub asteroid_environment_tag: Option<CigGuid>,
    /// DCB field: `asteroidDensityRtpc` (Class)
    #[serde(default)]
    pub asteroid_density_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for AsteroidBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_as.asteroid_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_as.asteroid_behavior }
}

impl<'a> Extract<'a> for AsteroidBehavior {
    const TYPE_NAME: &'static str = "AsteroidBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vehicle_effects: match inst.get("vehicleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Behavior_VehicleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Behavior_VehicleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weather: match inst.get("weather") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AsteroidBehavior_WeatherParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AsteroidBehavior_WeatherParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            asteroid_environment_tag: inst.get("asteroidEnvironmentTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            asteroid_density_rtpc: match inst.get("asteroidDensityRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AsteroidBehavior_WeatherParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidBehavior_WeatherParams {
    /// DCB field: `defaultDustFade` (Boolean)
    #[serde(default)]
    pub default_dust_fade: bool,
    /// DCB field: `defaultDustFadeRange` (Class)
    #[serde(default)]
    pub default_dust_fade_range: Option<Handle<Range>>,
    /// DCB field: `effects` (StrongPointer (array))
    #[serde(default)]
    pub effects: Vec<Handle<WeatherEffects_Asteroid>>,
}

impl Pooled for AsteroidBehavior_WeatherParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_as.asteroid_behavior_weather_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_as.asteroid_behavior_weather_params }
}

impl<'a> Extract<'a> for AsteroidBehavior_WeatherParams {
    const TYPE_NAME: &'static str = "AsteroidBehavior_WeatherParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_dust_fade: inst.get_bool("defaultDustFade").unwrap_or_default(),
            default_dust_fade_range: match inst.get("defaultDustFadeRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            effects: inst.get_array("effects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WeatherEffects_Asteroid>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WeatherEffects_Asteroid>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

