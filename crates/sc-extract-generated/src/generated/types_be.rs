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

/// DCB type: `BezierCurve`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BezierCurve {
    /// DCB field: `points` (Class (array))
    #[serde(default)]
    pub points: Vec<Handle<Vec2>>,
    /// DCB field: `useLUT` (Boolean)
    #[serde(default)]
    pub use_lut: bool,
}

impl Pooled for BezierCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_be.bezier_curve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_be.bezier_curve }
}

impl<'a> Extract<'a> for BezierCurve {
    const TYPE_NAME: &'static str = "BezierCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            points: inst.get_array("points")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            use_lut: inst.get_bool("useLUT").unwrap_or_default(),
        }
    }
}

/// DCB type: `BeaconsContracts`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconsContracts {
    /// DCB field: `serviceBeacons` (Reference (array))
    #[serde(default)]
    pub service_beacons: Vec<CigGuid>,
    /// DCB field: `serviceBeaconContractGenerator` (Reference)
    #[serde(default)]
    pub service_beacon_contract_generator: Option<CigGuid>,
}

impl Pooled for BeaconsContracts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_be.beacons_contracts }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_be.beacons_contracts }
}

impl<'a> Extract<'a> for BeaconsContracts {
    const TYPE_NAME: &'static str = "BeaconsContracts";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            service_beacons: inst.get_array("serviceBeacons")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            service_beacon_contract_generator: inst.get("serviceBeaconContractGenerator").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `Behavior_VehicleEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior_VehicleEffectParams {
    /// DCB field: `enableEngineTrails` (Boolean)
    #[serde(default)]
    pub enable_engine_trails: bool,
    /// DCB field: `enableEngineContrails` (Boolean)
    #[serde(default)]
    pub enable_engine_contrails: bool,
    /// DCB field: `customVehicleEffects` (StrongPointer)
    #[serde(default)]
    pub custom_vehicle_effects: Option<Handle<Behavior_CustomVehicleEffectsPreset>>,
}

impl Pooled for Behavior_VehicleEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_be.behavior_vehicle_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_be.behavior_vehicle_effect_params }
}

impl<'a> Extract<'a> for Behavior_VehicleEffectParams {
    const TYPE_NAME: &'static str = "Behavior_VehicleEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_engine_trails: inst.get_bool("enableEngineTrails").unwrap_or_default(),
            enable_engine_contrails: inst.get_bool("enableEngineContrails").unwrap_or_default(),
            custom_vehicle_effects: match inst.get("customVehicleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Behavior_CustomVehicleEffectsPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Behavior_CustomVehicleEffectsPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Behavior_CustomVehicleEffectsPreset`
///
/// Inherits from: `SEntitityEffectSystem_SystemBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior_CustomVehicleEffectsPreset {
    /// DCB field: `particleEffects` (Class)
    #[serde(default)]
    pub particle_effects: Option<Handle<SEntityEffectSystem_ParticleCategory>>,
}

impl Pooled for Behavior_CustomVehicleEffectsPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_be.behavior_custom_vehicle_effects_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_be.behavior_custom_vehicle_effects_preset }
}

impl<'a> Extract<'a> for Behavior_CustomVehicleEffectsPreset {
    const TYPE_NAME: &'static str = "Behavior_CustomVehicleEffectsPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            particle_effects: match inst.get("particleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityEffectSystem_ParticleCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityEffectSystem_ParticleCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Behavior_AtmosphereVehicleEffectParams`
///
/// Inherits from: `Behavior_VehicleEffectParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior_AtmosphereVehicleEffectParams {
    /// DCB field: `enableEngineTrails` (Boolean)
    #[serde(default)]
    pub enable_engine_trails: bool,
    /// DCB field: `enableEngineContrails` (Boolean)
    #[serde(default)]
    pub enable_engine_contrails: bool,
    /// DCB field: `customVehicleEffects` (StrongPointer)
    #[serde(default)]
    pub custom_vehicle_effects: Option<Handle<Behavior_CustomVehicleEffectsPreset>>,
    /// DCB field: `aerodynamicTrailCalculation` (StrongPointer)
    #[serde(default)]
    pub aerodynamic_trail_calculation: Option<Handle<AerodynamicTrailCalculation>>,
}

impl Pooled for Behavior_AtmosphereVehicleEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_be.behavior_atmosphere_vehicle_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_be.behavior_atmosphere_vehicle_effect_params }
}

impl<'a> Extract<'a> for Behavior_AtmosphereVehicleEffectParams {
    const TYPE_NAME: &'static str = "Behavior_AtmosphereVehicleEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_engine_trails: inst.get_bool("enableEngineTrails").unwrap_or_default(),
            enable_engine_contrails: inst.get_bool("enableEngineContrails").unwrap_or_default(),
            custom_vehicle_effects: match inst.get("customVehicleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Behavior_CustomVehicleEffectsPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Behavior_CustomVehicleEffectsPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aerodynamic_trail_calculation: match inst.get("aerodynamicTrailCalculation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AerodynamicTrailCalculation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AerodynamicTrailCalculation>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Behavior_ElectricalVehicleEffectParams`
///
/// Inherits from: `Behavior_VehicleEffectParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior_ElectricalVehicleEffectParams {
    /// DCB field: `enableEngineTrails` (Boolean)
    #[serde(default)]
    pub enable_engine_trails: bool,
    /// DCB field: `enableEngineContrails` (Boolean)
    #[serde(default)]
    pub enable_engine_contrails: bool,
    /// DCB field: `customVehicleEffects` (StrongPointer)
    #[serde(default)]
    pub custom_vehicle_effects: Option<Handle<Behavior_CustomVehicleEffectsPreset>>,
    /// DCB field: `customVehicleCalculation` (Class)
    #[serde(default)]
    pub custom_vehicle_calculation: Option<Handle<ElectricalCalculationPropertyRange>>,
}

impl Pooled for Behavior_ElectricalVehicleEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_be.behavior_electrical_vehicle_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_be.behavior_electrical_vehicle_effect_params }
}

impl<'a> Extract<'a> for Behavior_ElectricalVehicleEffectParams {
    const TYPE_NAME: &'static str = "Behavior_ElectricalVehicleEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_engine_trails: inst.get_bool("enableEngineTrails").unwrap_or_default(),
            enable_engine_contrails: inst.get_bool("enableEngineContrails").unwrap_or_default(),
            custom_vehicle_effects: match inst.get("customVehicleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Behavior_CustomVehicleEffectsPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Behavior_CustomVehicleEffectsPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            custom_vehicle_calculation: match inst.get("customVehicleCalculation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ElectricalCalculationPropertyRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ElectricalCalculationPropertyRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

