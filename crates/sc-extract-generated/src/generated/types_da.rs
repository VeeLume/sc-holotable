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

/// DCB type: `DamageBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageBase {
}

impl Pooled for DamageBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_base }
}

impl<'a> Extract<'a> for DamageBase {
    const TYPE_NAME: &'static str = "DamageBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DamageInfo`
///
/// Inherits from: `DamageBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageInfo {
    /// DCB field: `DamagePhysical` (Single)
    #[serde(default)]
    pub damage_physical: f32,
    /// DCB field: `DamageEnergy` (Single)
    #[serde(default)]
    pub damage_energy: f32,
    /// DCB field: `DamageDistortion` (Single)
    #[serde(default)]
    pub damage_distortion: f32,
    /// DCB field: `DamageThermal` (Single)
    #[serde(default)]
    pub damage_thermal: f32,
    /// DCB field: `DamageBiochemical` (Single)
    #[serde(default)]
    pub damage_biochemical: f32,
    /// DCB field: `DamageStun` (Single)
    #[serde(default)]
    pub damage_stun: f32,
}

impl Pooled for DamageInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_info }
}

impl<'a> Extract<'a> for DamageInfo {
    const TYPE_NAME: &'static str = "DamageInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_physical: inst.get_f32("DamagePhysical").unwrap_or_default(),
            damage_energy: inst.get_f32("DamageEnergy").unwrap_or_default(),
            damage_distortion: inst.get_f32("DamageDistortion").unwrap_or_default(),
            damage_thermal: inst.get_f32("DamageThermal").unwrap_or_default(),
            damage_biochemical: inst.get_f32("DamageBiochemical").unwrap_or_default(),
            damage_stun: inst.get_f32("DamageStun").unwrap_or_default(),
        }
    }
}

/// DCB type: `DamageMacro`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageMacro {
    /// DCB field: `damageInfo` (Class)
    #[serde(default)]
    pub damage_info: Option<Handle<DamageInfo>>,
}

impl Pooled for DamageMacro {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_macro }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_macro }
}

impl<'a> Extract<'a> for DamageMacro {
    const TYPE_NAME: &'static str = "DamageMacro";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_info: match inst.get("damageInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_resistance_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_resistance_base }
}

impl<'a> Extract<'a> for DamageResistanceBase {
    const TYPE_NAME: &'static str = "DamageResistanceBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DamageResistanceEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistanceEntry {
    /// DCB field: `Multiplier` (Single)
    #[serde(default)]
    pub multiplier: f32,
    /// DCB field: `Threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
    /// DCB field: `DamageCap` (Single)
    #[serde(default)]
    pub damage_cap: f32,
}

impl Pooled for DamageResistanceEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_resistance_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_resistance_entry }
}

impl<'a> Extract<'a> for DamageResistanceEntry {
    const TYPE_NAME: &'static str = "DamageResistanceEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            multiplier: inst.get_f32("Multiplier").unwrap_or_default(),
            threshold: inst.get_f32("Threshold").unwrap_or_default(),
            damage_cap: inst.get_f32("DamageCap").unwrap_or_default(),
        }
    }
}

/// DCB type: `DamageResistance`
///
/// Inherits from: `DamageResistanceBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistance {
    /// DCB field: `IgnoreMeleeDamage` (Boolean)
    #[serde(default)]
    pub ignore_melee_damage: bool,
    /// DCB field: `PhysicalResistance` (Class)
    #[serde(default)]
    pub physical_resistance: Option<Handle<DamageResistanceEntry>>,
    /// DCB field: `EnergyResistance` (Class)
    #[serde(default)]
    pub energy_resistance: Option<Handle<DamageResistanceEntry>>,
    /// DCB field: `DistortionResistance` (Class)
    #[serde(default)]
    pub distortion_resistance: Option<Handle<DamageResistanceEntry>>,
    /// DCB field: `ThermalResistance` (Class)
    #[serde(default)]
    pub thermal_resistance: Option<Handle<DamageResistanceEntry>>,
    /// DCB field: `BiochemicalResistance` (Class)
    #[serde(default)]
    pub biochemical_resistance: Option<Handle<DamageResistanceEntry>>,
    /// DCB field: `StunResistance` (Class)
    #[serde(default)]
    pub stun_resistance: Option<Handle<DamageResistanceEntry>>,
}

impl Pooled for DamageResistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_resistance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_resistance }
}

impl<'a> Extract<'a> for DamageResistance {
    const TYPE_NAME: &'static str = "DamageResistance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ignore_melee_damage: inst.get_bool("IgnoreMeleeDamage").unwrap_or_default(),
            physical_resistance: match inst.get("PhysicalResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            energy_resistance: match inst.get("EnergyResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distortion_resistance: match inst.get("DistortionResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            thermal_resistance: match inst.get("ThermalResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            biochemical_resistance: match inst.get("BiochemicalResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stun_resistance: match inst.get("StunResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DamageResistanceMacro`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistanceMacro {
    /// DCB field: `damageResistance` (Class)
    #[serde(default)]
    pub damage_resistance: Option<Handle<DamageResistance>>,
    /// DCB field: `impactForceResistance` (Class)
    #[serde(default)]
    pub impact_force_resistance: Option<Handle<ImpactForceResistance>>,
}

impl Pooled for DamageResistanceMacro {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_resistance_macro }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_resistance_macro }
}

impl<'a> Extract<'a> for DamageResistanceMacro {
    const TYPE_NAME: &'static str = "DamageResistanceMacro";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_resistance: match inst.get("damageResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            impact_force_resistance: match inst.get("impactForceResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ImpactForceResistance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ImpactForceResistance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DamageMapChannels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageMapChannels {
    /// DCB field: `thickness` (Single)
    #[serde(default)]
    pub thickness: f32,
    /// DCB field: `deformation` (Single)
    #[serde(default)]
    pub deformation: f32,
    /// DCB field: `burn` (Single)
    #[serde(default)]
    pub burn: f32,
    /// DCB field: `temperature` (Single)
    #[serde(default)]
    pub temperature: f32,
}

impl Pooled for DamageMapChannels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_map_channels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_map_channels }
}

impl<'a> Extract<'a> for DamageMapChannels {
    const TYPE_NAME: &'static str = "DamageMapChannels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            thickness: inst.get_f32("thickness").unwrap_or_default(),
            deformation: inst.get_f32("deformation").unwrap_or_default(),
            burn: inst.get_f32("burn").unwrap_or_default(),
            temperature: inst.get_f32("temperature").unwrap_or_default(),
        }
    }
}

/// DCB type: `DamageMapDamageTypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageMapDamageTypes {
    /// DCB field: `physical` (Class)
    #[serde(default)]
    pub physical: Option<Handle<DamageMapChannels>>,
    /// DCB field: `energy` (Class)
    #[serde(default)]
    pub energy: Option<Handle<DamageMapChannels>>,
}

impl Pooled for DamageMapDamageTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_map_damage_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_map_damage_types }
}

impl<'a> Extract<'a> for DamageMapDamageTypes {
    const TYPE_NAME: &'static str = "DamageMapDamageTypes";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            physical: match inst.get("physical") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapChannels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            energy: match inst.get("energy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapChannels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DamageMapDamageForm`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageMapDamageForm {
    /// DCB field: `damageStrengthMultipliers` (Class)
    #[serde(default)]
    pub damage_strength_multipliers: Option<Handle<DamageMapDamageTypes>>,
    /// DCB field: `innerRadiusMultipliers` (Class)
    #[serde(default)]
    pub inner_radius_multipliers: Option<Handle<DamageMapChannels>>,
    /// DCB field: `outerRadiusMultipliers` (Class)
    #[serde(default)]
    pub outer_radius_multipliers: Option<Handle<DamageMapChannels>>,
    /// DCB field: `noiseStrength` (Class)
    #[serde(default)]
    pub noise_strength: Option<Handle<DamageMapChannels>>,
}

impl Pooled for DamageMapDamageForm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_map_damage_form }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_map_damage_form }
}

impl<'a> Extract<'a> for DamageMapDamageForm {
    const TYPE_NAME: &'static str = "DamageMapDamageForm";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_strength_multipliers: match inst.get("damageStrengthMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapDamageTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inner_radius_multipliers: match inst.get("innerRadiusMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapChannels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outer_radius_multipliers: match inst.get("outerRadiusMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapChannels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_strength: match inst.get("noiseStrength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapChannels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DamageMapGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageMapGlobalParams {
    /// DCB field: `impact` (Class)
    #[serde(default)]
    pub impact: Option<Handle<DamageMapDamageForm>>,
    /// DCB field: `squib` (Class)
    #[serde(default)]
    pub squib: Option<Handle<DamageMapDamageForm>>,
    /// DCB field: `explosions` (Class)
    #[serde(default)]
    pub explosions: Option<Handle<DamageMapDamageForm>>,
}

impl Pooled for DamageMapGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.damage_map_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.damage_map_global_params }
}

impl<'a> Extract<'a> for DamageMapGlobalParams {
    const TYPE_NAME: &'static str = "DamageMapGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            impact: match inst.get("impact") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageForm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapDamageForm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            squib: match inst.get("squib") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageForm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapDamageForm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            explosions: match inst.get("explosions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageForm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapDamageForm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Date`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Date {
    /// DCB field: `day` (Int32)
    #[serde(default)]
    pub day: i32,
    /// DCB field: `month` (EnumChoice)
    #[serde(default)]
    pub month: String,
    /// DCB field: `year` (Int32)
    #[serde(default)]
    pub year: i32,
}

impl Pooled for Date {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.date }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.date }
}

impl<'a> Extract<'a> for Date {
    const TYPE_NAME: &'static str = "Date";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            day: inst.get_i32("day").unwrap_or_default(),
            month: inst.get_str("month").map(String::from).unwrap_or_default(),
            year: inst.get_i32("year").unwrap_or_default(),
        }
    }
}

/// DCB type: `DateTime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
    /// DCB field: `date` (Class)
    #[serde(default)]
    pub date: Option<Handle<Date>>,
    /// DCB field: `time` (Class)
    #[serde(default)]
    pub time: Option<Handle<Time>>,
}

impl Pooled for DateTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.date_time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.date_time }
}

impl<'a> Extract<'a> for DateTime {
    const TYPE_NAME: &'static str = "DateTime";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            date: match inst.get("date") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Date>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Date>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time: match inst.get("time") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Time>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Time>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DateTimeSchedule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeSchedule {
    /// DCB field: `start` (Class)
    #[serde(default)]
    pub start: Option<Handle<DateTime>>,
    /// DCB field: `end` (Class)
    #[serde(default)]
    pub end: Option<Handle<DateTime>>,
    /// DCB field: `repeatType` (EnumChoice)
    #[serde(default)]
    pub repeat_type: String,
    /// DCB field: `repeatEnd` (Class)
    #[serde(default)]
    pub repeat_end: Option<Handle<Date>>,
}

impl Pooled for DateTimeSchedule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.date_time_schedule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.date_time_schedule }
}

impl<'a> Extract<'a> for DateTimeSchedule {
    const TYPE_NAME: &'static str = "DateTimeSchedule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start: match inst.get("start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DateTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DateTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end: match inst.get("end") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DateTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DateTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            repeat_type: inst.get_str("repeatType").map(String::from).unwrap_or_default(),
            repeat_end: match inst.get("repeatEnd") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Date>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Date>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataForgeComponentParams {
}

impl Pooled for DataForgeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_da.data_forge_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_da.data_forge_component_params }
}

impl<'a> Extract<'a> for DataForgeComponentParams {
    const TYPE_NAME: &'static str = "DataForgeComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

