// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-shiptraffic`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ShipTrafficComponentParams`
/// Inherits from: `DataForgeComponentParams`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct ShipTrafficComponentParams {
    /// `Geometry` (Class)
    pub geometry: Option<Handle<GlobalResourceCGF>>,
    /// `Material` (Class)
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// `Contrail` (Class)
    pub contrail: Option<Handle<GlobalResourceParticle>>,
    /// `Weight` (Single)
    pub weight: f32,
    /// `Geometry2` (Class)
    pub geometry2: Option<Handle<GlobalResourceCGF>>,
    /// `Material2` (Class)
    pub material2: Option<Handle<GlobalResourceMaterial>>,
    /// `Contrail2` (Class)
    pub contrail2: Option<Handle<GlobalResourceParticle>>,
    /// `Weight2` (Single)
    pub weight2: f32,
    /// `Geometry3` (Class)
    pub geometry3: Option<Handle<GlobalResourceCGF>>,
    /// `Material3` (Class)
    pub material3: Option<Handle<GlobalResourceMaterial>>,
    /// `Contrail3` (Class)
    pub contrail3: Option<Handle<GlobalResourceParticle>>,
    /// `Weight3` (Single)
    pub weight3: f32,
    /// `Seed` (Int32)
    pub seed: i32,
    /// `TrafficRadius` (Single)
    pub traffic_radius: f32,
    /// `MaxUpdateDistance` (Single)
    pub max_update_distance: f32,
    /// `ShipCount` (Int32)
    pub ship_count: i32,
    /// `MaxLiveInstances` (Int32)
    pub max_live_instances: i32,
    /// `AltitudeMin` (Single)
    pub altitude_min: f32,
    /// `AltitudeMax` (Single)
    pub altitude_max: f32,
    /// `SpeedMin` (Single)
    pub speed_min: f32,
    /// `SpeedMax` (Single)
    pub speed_max: f32,
    /// `KeepOutRadius` (Single)
    pub keep_out_radius: f32,
    /// `TargetFPS` (Single)
    pub target_fps: f32,
    /// `CastShadows` (Boolean)
    pub cast_shadows: bool,
}

impl Pooled for ShipTrafficComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_shiptraffic.ship_traffic_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_shiptraffic.ship_traffic_component_params
    }
}

impl<'a> Extract<'a> for ShipTrafficComponentParams {
    const TYPE_NAME: &'static str = "ShipTrafficComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceCGF>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceMaterial>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contrail: match inst.get("Contrail") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceParticle>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            weight: inst.get_f32("Weight").unwrap_or_default(),
            geometry2: match inst.get("Geometry2") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceCGF>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            material2: match inst.get("Material2") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceMaterial>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contrail2: match inst.get("Contrail2") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceParticle>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            weight2: inst.get_f32("Weight2").unwrap_or_default(),
            geometry3: match inst.get("Geometry3") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceCGF>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            material3: match inst.get("Material3") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceMaterial>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contrail3: match inst.get("Contrail3") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceParticle>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            weight3: inst.get_f32("Weight3").unwrap_or_default(),
            seed: inst.get_i32("Seed").unwrap_or_default(),
            traffic_radius: inst.get_f32("TrafficRadius").unwrap_or_default(),
            max_update_distance: inst.get_f32("MaxUpdateDistance").unwrap_or_default(),
            ship_count: inst.get_i32("ShipCount").unwrap_or_default(),
            max_live_instances: inst.get_i32("MaxLiveInstances").unwrap_or_default(),
            altitude_min: inst.get_f32("AltitudeMin").unwrap_or_default(),
            altitude_max: inst.get_f32("AltitudeMax").unwrap_or_default(),
            speed_min: inst.get_f32("SpeedMin").unwrap_or_default(),
            speed_max: inst.get_f32("SpeedMax").unwrap_or_default(),
            keep_out_radius: inst.get_f32("KeepOutRadius").unwrap_or_default(),
            target_fps: inst.get_f32("TargetFPS").unwrap_or_default(),
            cast_shadows: inst.get_bool("CastShadows").unwrap_or_default(),
        }
    }
}
