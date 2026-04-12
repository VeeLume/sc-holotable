// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `initialdamageoverrides`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SInitialDamageSpecifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInitialDamageSpecifierBase {
}

impl Pooled for SInitialDamageSpecifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.initialdamageoverrides.sinitial_damage_specifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.initialdamageoverrides.sinitial_damage_specifier_base }
}

impl<'a> Extract<'a> for SInitialDamageSpecifierBase {
    const TYPE_NAME: &'static str = "SInitialDamageSpecifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SInitialDamage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInitialDamage {
    /// `RandomSeed` (Int32)
    #[serde(default)]
    pub random_seed: i32,
    /// `Damage` (StrongPointer)
    #[serde(default)]
    pub damage: Option<Handle<SInitialDamageSpecifierBase>>,
    /// `BoundingBoxScale` (Class)
    #[serde(default)]
    pub bounding_box_scale: Option<Handle<Vec3>>,
    /// `MaxDamageRatio` (Single)
    #[serde(default)]
    pub max_damage_ratio: f32,
    /// `MinHitCount` (Int32)
    #[serde(default)]
    pub min_hit_count: i32,
    /// `MaxHitCount` (Int32)
    #[serde(default)]
    pub max_hit_count: i32,
    /// `MinHitRadiusFraction` (Single)
    #[serde(default)]
    pub min_hit_radius_fraction: f32,
    /// `MaxHitRadiusFraction` (Single)
    #[serde(default)]
    pub max_hit_radius_fraction: f32,
    /// `HitDamageVariationFactor` (Single)
    #[serde(default)]
    pub hit_damage_variation_factor: f32,
    /// `DamageMapDamageScale` (Single)
    #[serde(default)]
    pub damage_map_damage_scale: f32,
    /// `DamageMapNoiseStrength` (Class)
    #[serde(default)]
    pub damage_map_noise_strength: Option<Handle<DamageMapChannels>>,
}

impl Pooled for SInitialDamage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.initialdamageoverrides.sinitial_damage }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.initialdamageoverrides.sinitial_damage }
}

impl<'a> Extract<'a> for SInitialDamage {
    const TYPE_NAME: &'static str = "SInitialDamage";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            random_seed: inst.get_i32("RandomSeed").unwrap_or_default(),
            damage: match inst.get("Damage") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInitialDamageSpecifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInitialDamageSpecifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bounding_box_scale: match inst.get("BoundingBoxScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_damage_ratio: inst.get_f32("MaxDamageRatio").unwrap_or_default(),
            min_hit_count: inst.get_i32("MinHitCount").unwrap_or_default(),
            max_hit_count: inst.get_i32("MaxHitCount").unwrap_or_default(),
            min_hit_radius_fraction: inst.get_f32("MinHitRadiusFraction").unwrap_or_default(),
            max_hit_radius_fraction: inst.get_f32("MaxHitRadiusFraction").unwrap_or_default(),
            hit_damage_variation_factor: inst.get_f32("HitDamageVariationFactor").unwrap_or_default(),
            damage_map_damage_scale: inst.get_f32("DamageMapDamageScale").unwrap_or_default(),
            damage_map_noise_strength: match inst.get("DamageMapNoiseStrength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapChannels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InitialDamageOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialDamageOverride {
    /// `initialDmgOverride` (Class)
    #[serde(default)]
    pub initial_dmg_override: Option<Handle<SInitialDamage>>,
}

impl Pooled for InitialDamageOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.initialdamageoverrides.initial_damage_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.initialdamageoverrides.initial_damage_override }
}

impl<'a> Extract<'a> for InitialDamageOverride {
    const TYPE_NAME: &'static str = "InitialDamageOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            initial_dmg_override: match inst.get("initialDmgOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInitialDamage>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInitialDamage>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

