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

/// DCB type: `SDensityClassLifetimeOverrideEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDensityClassLifetimeOverrideEntry {
    /// DCB field: `densityClass` (Reference)
    #[serde(default)]
    pub density_class: Option<CigGuid>,
    /// DCB field: `lifetimeOverride` (StrongPointer)
    #[serde(default)]
    pub lifetime_override: Option<Handle<TimeValue_Base>>,
}

impl Pooled for SDensityClassLifetimeOverrideEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sd.sdensity_class_lifetime_override_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sd.sdensity_class_lifetime_override_entry }
}

impl<'a> Extract<'a> for SDensityClassLifetimeOverrideEntry {
    const TYPE_NAME: &'static str = "SDensityClassLifetimeOverrideEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            density_class: inst.get("densityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            lifetime_override: match inst.get("lifetimeOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TimeValue_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TimeValue_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SDefaultDensityClassLifetimeOverrides`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDefaultDensityClassLifetimeOverrides {
    /// DCB field: `lifetimeOverride` (StrongPointer)
    #[serde(default)]
    pub lifetime_override: Option<Handle<TimeValue_Base>>,
    /// DCB field: `excludes` (Reference (array))
    #[serde(default)]
    pub excludes: Vec<CigGuid>,
}

impl Pooled for SDefaultDensityClassLifetimeOverrides {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sd.sdefault_density_class_lifetime_overrides }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sd.sdefault_density_class_lifetime_overrides }
}

impl<'a> Extract<'a> for SDefaultDensityClassLifetimeOverrides {
    const TYPE_NAME: &'static str = "SDefaultDensityClassLifetimeOverrides";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lifetime_override: match inst.get("lifetimeOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TimeValue_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TimeValue_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            excludes: inst.get_array("excludes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SDecayCurveMaxValueParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDecayCurveMaxValueParams {
    /// DCB field: `maxValue` (Single)
    #[serde(default)]
    pub max_value: f32,
    /// DCB field: `minScalingFactor` (Single)
    #[serde(default)]
    pub min_scaling_factor: f32,
    /// DCB field: `useDecayScaling` (Boolean)
    #[serde(default)]
    pub use_decay_scaling: bool,
    /// DCB field: `useWeaponOrientation` (Boolean)
    #[serde(default)]
    pub use_weapon_orientation: bool,
}

impl Pooled for SDecayCurveMaxValueParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sd.sdecay_curve_max_value_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sd.sdecay_curve_max_value_params }
}

impl<'a> Extract<'a> for SDecayCurveMaxValueParams {
    const TYPE_NAME: &'static str = "SDecayCurveMaxValueParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
            min_scaling_factor: inst.get_f32("minScalingFactor").unwrap_or_default(),
            use_decay_scaling: inst.get_bool("useDecayScaling").unwrap_or_default(),
            use_weapon_orientation: inst.get_bool("useWeaponOrientation").unwrap_or_default(),
        }
    }
}

/// DCB type: `SDecayCurveMaxValues`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDecayCurveMaxValues {
    /// DCB field: `xMaxValueParams` (Class)
    #[serde(default)]
    pub x_max_value_params: Option<Handle<SDecayCurveMaxValueParams>>,
    /// DCB field: `yMaxValueParams` (Class)
    #[serde(default)]
    pub y_max_value_params: Option<Handle<SDecayCurveMaxValueParams>>,
    /// DCB field: `zMaxValueParams` (Class)
    #[serde(default)]
    pub z_max_value_params: Option<Handle<SDecayCurveMaxValueParams>>,
}

impl Pooled for SDecayCurveMaxValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sd.sdecay_curve_max_values }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sd.sdecay_curve_max_values }
}

impl<'a> Extract<'a> for SDecayCurveMaxValues {
    const TYPE_NAME: &'static str = "SDecayCurveMaxValues";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            x_max_value_params: match inst.get("xMaxValueParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            y_max_value_params: match inst.get("yMaxValueParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            z_max_value_params: match inst.get("zMaxValueParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SDecayTimesAndCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDecayTimesAndCurves {
    /// DCB field: `decayTimeMultipliers` (Class)
    #[serde(default)]
    pub decay_time_multipliers: Option<Handle<Vec3>>,
    /// DCB field: `decayCurveMaxValues` (Class)
    #[serde(default)]
    pub decay_curve_max_values: Option<Handle<SDecayCurveMaxValues>>,
    /// DCB field: `decayCurves` (StrongPointer)
    #[serde(default)]
    pub decay_curves: Option<Handle<SXYZCurves>>,
}

impl Pooled for SDecayTimesAndCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sd.sdecay_times_and_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sd.sdecay_times_and_curves }
}

impl<'a> Extract<'a> for SDecayTimesAndCurves {
    const TYPE_NAME: &'static str = "SDecayTimesAndCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay_time_multipliers: match inst.get("decayTimeMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_curve_max_values: match inst.get("decayCurveMaxValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_curves: match inst.get("decayCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

