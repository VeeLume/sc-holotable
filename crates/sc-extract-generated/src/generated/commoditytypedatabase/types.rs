// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `commoditytypedatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `TemperatureDamageControl`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureDamageControl {
}

impl Pooled for TemperatureDamageControl {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commoditytypedatabase.temperature_damage_control }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commoditytypedatabase.temperature_damage_control }
}

impl<'a> Extract<'a> for TemperatureDamageControl {
    const TYPE_NAME: &'static str = "TemperatureDamageControl";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CommodityTemperatureTolerance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityTemperatureTolerance {
    /// `optimalTempMin` (Single)
    #[serde(default)]
    pub optimal_temp_min: f32,
    /// `optimalTempMax` (Single)
    #[serde(default)]
    pub optimal_temp_max: f32,
    /// `OptimalTempFallOff` (Single)
    #[serde(default)]
    pub optimal_temp_fall_off: f32,
    /// `damageCurveControl` (StrongPointer)
    #[serde(default)]
    pub damage_curve_control: Option<Handle<TemperatureDamageControl>>,
}

impl Pooled for CommodityTemperatureTolerance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commoditytypedatabase.commodity_temperature_tolerance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commoditytypedatabase.commodity_temperature_tolerance }
}

impl<'a> Extract<'a> for CommodityTemperatureTolerance {
    const TYPE_NAME: &'static str = "CommodityTemperatureTolerance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            optimal_temp_min: inst.get_f32("optimalTempMin").unwrap_or_default(),
            optimal_temp_max: inst.get_f32("optimalTempMax").unwrap_or_default(),
            optimal_temp_fall_off: inst.get_f32("OptimalTempFallOff").unwrap_or_default(),
            damage_curve_control: match inst.get("damageCurveControl") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TemperatureDamageControl>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TemperatureDamageControl>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DamageResistanceBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistanceBase {
}

impl Pooled for DamageResistanceBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commoditytypedatabase.damage_resistance_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commoditytypedatabase.damage_resistance_base }
}

impl<'a> Extract<'a> for DamageResistanceBase {
    const TYPE_NAME: &'static str = "DamageResistanceBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CommodityType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityType {
    /// `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `defaultThumbnailPath` (String)
    #[serde(default)]
    pub default_thumbnail_path: String,
}

impl Pooled for CommodityType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commoditytypedatabase.commodity_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commoditytypedatabase.commodity_type }
}

impl<'a> Extract<'a> for CommodityType {
    const TYPE_NAME: &'static str = "CommodityType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            default_thumbnail_path: inst.get_str("defaultThumbnailPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommoditySubtype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommoditySubtype {
    /// `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `symbol` (String)
    #[serde(default)]
    pub symbol: String,
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGBA8>>,
    /// `commodity` (Reference)
    #[serde(default)]
    pub commodity: Option<CigGuid>,
    /// `volatility` (Single)
    #[serde(default)]
    pub volatility: f32,
    /// `gForceTolerance` (Single)
    #[serde(default)]
    pub g_force_tolerance: f32,
    /// `gForceDeltaToDamage` (Single)
    #[serde(default)]
    pub g_force_delta_to_damage: f32,
    /// `HealthDecayOverTime` (Single)
    #[serde(default)]
    pub health_decay_over_time: f32,
    /// `temperatureTolerance` (Class)
    #[serde(default)]
    pub temperature_tolerance: Option<Handle<CommodityTemperatureTolerance>>,
    /// `damageResistance` (StrongPointer)
    #[serde(default)]
    pub damage_resistance: Option<Handle<DamageResistanceBase>>,
    /// `refineOutput` (Reference)
    #[serde(default)]
    pub refine_output: Option<CigGuid>,
}

impl Pooled for CommoditySubtype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commoditytypedatabase.commodity_subtype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commoditytypedatabase.commodity_subtype }
}

impl<'a> Extract<'a> for CommoditySubtype {
    const TYPE_NAME: &'static str = "CommoditySubtype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            symbol: inst.get_str("symbol").map(String::from).unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            commodity: inst.get("commodity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            volatility: inst.get_f32("volatility").unwrap_or_default(),
            g_force_tolerance: inst.get_f32("gForceTolerance").unwrap_or_default(),
            g_force_delta_to_damage: inst.get_f32("gForceDeltaToDamage").unwrap_or_default(),
            health_decay_over_time: inst.get_f32("HealthDecayOverTime").unwrap_or_default(),
            temperature_tolerance: match inst.get("temperatureTolerance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommodityTemperatureTolerance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommodityTemperatureTolerance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damage_resistance: match inst.get("damageResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            refine_output: inst.get("refineOutput").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `CommodityTypeDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityTypeDatabase {
    /// `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
    /// `subtypes` (Reference (array))
    #[serde(default)]
    pub subtypes: Vec<CigGuid>,
}

impl Pooled for CommodityTypeDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commoditytypedatabase.commodity_type_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commoditytypedatabase.commodity_type_database }
}

impl<'a> Extract<'a> for CommodityTypeDatabase {
    const TYPE_NAME: &'static str = "CommodityTypeDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            subtypes: inst.get_array("subtypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

