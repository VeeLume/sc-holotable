// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `weaponmisfiredef`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SWeaponMisfireEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponMisfireEntry {
    /// `misfireProbabilityCurve` (Class)
    #[serde(default)]
    pub misfire_probability_curve: Option<Handle<BezierCurve>>,
    /// `damage` (StrongPointer)
    #[serde(default)]
    pub damage: Option<Handle<DamageBase>>,
    /// `hitType` (String)
    #[serde(default)]
    pub hit_type: String,
}

impl Pooled for SWeaponMisfireEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponmisfiredef.sweapon_misfire_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponmisfiredef.sweapon_misfire_entry }
}

impl<'a> Extract<'a> for SWeaponMisfireEntry {
    const TYPE_NAME: &'static str = "SWeaponMisfireEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            misfire_probability_curve: match inst.get("misfireProbabilityCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damage: match inst.get("damage") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hit_type: inst.get_str("hitType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `WeaponMisfireDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponMisfireDef {
    /// `minorMisfire` (Class)
    #[serde(default)]
    pub minor_misfire: Option<Handle<SWeaponMisfireEntry>>,
    /// `minorMisfireDuration` (Single)
    #[serde(default)]
    pub minor_misfire_duration: f32,
    /// `minorMisfireCooldown` (Single)
    #[serde(default)]
    pub minor_misfire_cooldown: f32,
    /// `majorMisfire` (Class)
    #[serde(default)]
    pub major_misfire: Option<Handle<SWeaponMisfireEntry>>,
    /// `majorMisfireCooldown` (Single)
    #[serde(default)]
    pub major_misfire_cooldown: f32,
    /// `criticalMisfire` (Class)
    #[serde(default)]
    pub critical_misfire: Option<Handle<SWeaponMisfireEntry>>,
}

impl Pooled for WeaponMisfireDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponmisfiredef.weapon_misfire_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponmisfiredef.weapon_misfire_def }
}

impl<'a> Extract<'a> for WeaponMisfireDef {
    const TYPE_NAME: &'static str = "WeaponMisfireDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minor_misfire: match inst.get("minorMisfire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponMisfireEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponMisfireEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            minor_misfire_duration: inst.get_f32("minorMisfireDuration").unwrap_or_default(),
            minor_misfire_cooldown: inst.get_f32("minorMisfireCooldown").unwrap_or_default(),
            major_misfire: match inst.get("majorMisfire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponMisfireEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponMisfireEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            major_misfire_cooldown: inst.get_f32("majorMisfireCooldown").unwrap_or_default(),
            critical_misfire: match inst.get("criticalMisfire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponMisfireEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponMisfireEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

