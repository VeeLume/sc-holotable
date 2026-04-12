// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `damage`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `DamageMacro`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageMacro {
    /// `damageInfo` (Class)
    #[serde(default)]
    pub damage_info: Option<Handle<DamageInfo>>,
}

impl Pooled for DamageMacro {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.damage.damage_macro }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.damage.damage_macro }
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

/// DCB type: `DamageResistanceEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistanceEntry {
    /// `Multiplier` (Single)
    #[serde(default)]
    pub multiplier: f32,
    /// `Threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
    /// `DamageCap` (Single)
    #[serde(default)]
    pub damage_cap: f32,
}

impl Pooled for DamageResistanceEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.damage.damage_resistance_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.damage.damage_resistance_entry }
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
/// Inherits from: `DamageResistanceBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistance {
    /// `IgnoreMeleeDamage` (Boolean)
    #[serde(default)]
    pub ignore_melee_damage: bool,
    /// `PhysicalResistance` (Class)
    #[serde(default)]
    pub physical_resistance: Option<Handle<DamageResistanceEntry>>,
    /// `EnergyResistance` (Class)
    #[serde(default)]
    pub energy_resistance: Option<Handle<DamageResistanceEntry>>,
    /// `DistortionResistance` (Class)
    #[serde(default)]
    pub distortion_resistance: Option<Handle<DamageResistanceEntry>>,
    /// `ThermalResistance` (Class)
    #[serde(default)]
    pub thermal_resistance: Option<Handle<DamageResistanceEntry>>,
    /// `BiochemicalResistance` (Class)
    #[serde(default)]
    pub biochemical_resistance: Option<Handle<DamageResistanceEntry>>,
    /// `StunResistance` (Class)
    #[serde(default)]
    pub stun_resistance: Option<Handle<DamageResistanceEntry>>,
}

impl Pooled for DamageResistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.damage.damage_resistance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.damage.damage_resistance }
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

/// DCB type: `ImpactForceResistance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactForceResistance {
    /// `impactForceResistance` (Single)
    #[serde(default)]
    pub impact_force_resistance: f32,
}

impl Pooled for ImpactForceResistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.damage.impact_force_resistance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.damage.impact_force_resistance }
}

impl<'a> Extract<'a> for ImpactForceResistance {
    const TYPE_NAME: &'static str = "ImpactForceResistance";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            impact_force_resistance: inst.get_f32("impactForceResistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `DamageResistanceMacro`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistanceMacro {
    /// `damageResistance` (Class)
    #[serde(default)]
    pub damage_resistance: Option<Handle<DamageResistance>>,
    /// `impactForceResistance` (Class)
    #[serde(default)]
    pub impact_force_resistance: Option<Handle<ImpactForceResistance>>,
}

impl Pooled for DamageResistanceMacro {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.damage.damage_resistance_macro }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.damage.damage_resistance_macro }
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

