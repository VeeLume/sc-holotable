// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `rendererpresets`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CameraLensStreak`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensStreak {
    /// `Intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// `Width` (Single)
    #[serde(default)]
    pub width: f32,
    /// `Tint` (Class)
    #[serde(default)]
    pub tint: Option<Handle<RGB>>,
}

impl Pooled for CameraLensStreak {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rendererpresets.camera_lens_streak }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rendererpresets.camera_lens_streak }
}

impl<'a> Extract<'a> for CameraLensStreak {
    const TYPE_NAME: &'static str = "CameraLensStreak";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            intensity: inst.get_f32("Intensity").unwrap_or_default(),
            width: inst.get_f32("Width").unwrap_or_default(),
            tint: match inst.get("Tint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraLensDistortion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensDistortion {
    /// `Radial` (Single)
    #[serde(default)]
    pub radial: f32,
    /// `Spherical` (Single)
    #[serde(default)]
    pub spherical: f32,
    /// `Coma` (Single)
    #[serde(default)]
    pub coma: f32,
    /// `Curvature` (Single)
    #[serde(default)]
    pub curvature: f32,
}

impl Pooled for CameraLensDistortion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rendererpresets.camera_lens_distortion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rendererpresets.camera_lens_distortion }
}

impl<'a> Extract<'a> for CameraLensDistortion {
    const TYPE_NAME: &'static str = "CameraLensDistortion";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radial: inst.get_f32("Radial").unwrap_or_default(),
            spherical: inst.get_f32("Spherical").unwrap_or_default(),
            coma: inst.get_f32("Coma").unwrap_or_default(),
            curvature: inst.get_f32("Curvature").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraLensChromaticAberration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensChromaticAberration {
    /// `Transverse` (Single)
    #[serde(default)]
    pub transverse: f32,
    /// `Axial` (Single)
    #[serde(default)]
    pub axial: f32,
}

impl Pooled for CameraLensChromaticAberration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rendererpresets.camera_lens_chromatic_aberration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rendererpresets.camera_lens_chromatic_aberration }
}

impl<'a> Extract<'a> for CameraLensChromaticAberration {
    const TYPE_NAME: &'static str = "CameraLensChromaticAberration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            transverse: inst.get_f32("Transverse").unwrap_or_default(),
            axial: inst.get_f32("Axial").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraLensGhostInstance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensGhostInstance {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `Position` (Single)
    #[serde(default)]
    pub position: f32,
    /// `Intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// `Tint` (Class)
    #[serde(default)]
    pub tint: Option<Handle<RGB>>,
}

impl Pooled for CameraLensGhostInstance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rendererpresets.camera_lens_ghost_instance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rendererpresets.camera_lens_ghost_instance }
}

impl<'a> Extract<'a> for CameraLensGhostInstance {
    const TYPE_NAME: &'static str = "CameraLensGhostInstance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            position: inst.get_f32("Position").unwrap_or_default(),
            intensity: inst.get_f32("Intensity").unwrap_or_default(),
            tint: match inst.get("Tint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CameraLensGhostSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensGhostSet {
    /// `SetName` (String)
    #[serde(default)]
    pub set_name: String,
    /// `Radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `Distortion` (StrongPointer)
    #[serde(default)]
    pub distortion: Option<Handle<CameraLensDistortion>>,
    /// `ChromaticAberration` (StrongPointer)
    #[serde(default)]
    pub chromatic_aberration: Option<Handle<CameraLensChromaticAberration>>,
    /// `Instances` (Class (array))
    #[serde(default)]
    pub instances: Vec<Handle<CameraLensGhostInstance>>,
}

impl Pooled for CameraLensGhostSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rendererpresets.camera_lens_ghost_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rendererpresets.camera_lens_ghost_set }
}

impl<'a> Extract<'a> for CameraLensGhostSet {
    const TYPE_NAME: &'static str = "CameraLensGhostSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            set_name: inst.get_str("SetName").map(String::from).unwrap_or_default(),
            radius: inst.get_f32("Radius").unwrap_or_default(),
            distortion: match inst.get("Distortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraLensDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraLensDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chromatic_aberration: match inst.get("ChromaticAberration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraLensChromaticAberration>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraLensChromaticAberration>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            instances: inst.get_array("Instances")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CameraLensGhostInstance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CameraLensGhostInstance>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraLensParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraLensParams {
    /// `BloomIntensity` (Single)
    #[serde(default)]
    pub bloom_intensity: f32,
    /// `FlareIntensity` (Single)
    #[serde(default)]
    pub flare_intensity: f32,
    /// `Streak` (StrongPointer)
    #[serde(default)]
    pub streak: Option<Handle<CameraLensStreak>>,
    /// `GhostSets` (Class (array))
    #[serde(default)]
    pub ghost_sets: Vec<Handle<CameraLensGhostSet>>,
}

impl Pooled for CameraLensParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rendererpresets.camera_lens_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rendererpresets.camera_lens_params }
}

impl<'a> Extract<'a> for CameraLensParams {
    const TYPE_NAME: &'static str = "CameraLensParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bloom_intensity: inst.get_f32("BloomIntensity").unwrap_or_default(),
            flare_intensity: inst.get_f32("FlareIntensity").unwrap_or_default(),
            streak: match inst.get("Streak") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraLensStreak>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraLensStreak>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ghost_sets: inst.get_array("GhostSets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CameraLensGhostSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CameraLensGhostSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

