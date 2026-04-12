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

/// DCB type: `SWeaponProceduralHeadRecoilCurveModifierDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHeadRecoilCurveModifierDef {
    /// DCB field: `positionModifier` (Class)
    #[serde(default)]
    pub position_modifier: Option<Handle<SHeadRecoilNoiseModifier>>,
    /// DCB field: `rotationModifier` (Class)
    #[serde(default)]
    pub rotation_modifier: Option<Handle<SHeadRecoilNoiseModifier>>,
    /// DCB field: `headRecoilTimeModifier` (Single)
    #[serde(default)]
    pub head_recoil_time_modifier: f32,
    /// DCB field: `frequencyModifier` (Single)
    #[serde(default)]
    pub frequency_modifier: f32,
    /// DCB field: `smoothingSpeedModifier` (Single)
    #[serde(default)]
    pub smoothing_speed_modifier: f32,
}

impl Pooled for SWeaponProceduralHeadRecoilCurveModifierDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_head_recoil_curve_modifier_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_head_recoil_curve_modifier_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHeadRecoilCurveModifierDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHeadRecoilCurveModifierDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position_modifier: match inst.get("positionModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHeadRecoilNoiseModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHeadRecoilNoiseModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_modifier: match inst.get("rotationModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHeadRecoilNoiseModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHeadRecoilNoiseModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil_time_modifier: inst.get_f32("headRecoilTimeModifier").unwrap_or_default(),
            frequency_modifier: inst.get_f32("frequencyModifier").unwrap_or_default(),
            smoothing_speed_modifier: inst.get_f32("smoothingSpeedModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeaponStats`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponStats {
    /// DCB field: `fireRate` (Int32)
    #[serde(default)]
    pub fire_rate: i32,
    /// DCB field: `fireRateMultiplier` (Single)
    #[serde(default)]
    pub fire_rate_multiplier: f32,
    /// DCB field: `damageMultiplier` (Single)
    #[serde(default)]
    pub damage_multiplier: f32,
    /// DCB field: `damageOverTimeMultiplier` (Single)
    #[serde(default)]
    pub damage_over_time_multiplier: f32,
    /// DCB field: `projectileSpeedMultiplier` (Single)
    #[serde(default)]
    pub projectile_speed_multiplier: f32,
    /// DCB field: `pellets` (Int32)
    #[serde(default)]
    pub pellets: i32,
    /// DCB field: `burstShots` (Int32)
    #[serde(default)]
    pub burst_shots: i32,
    /// DCB field: `ammoCost` (Int32)
    #[serde(default)]
    pub ammo_cost: i32,
    /// DCB field: `ammoCostMultiplier` (Single)
    #[serde(default)]
    pub ammo_cost_multiplier: f32,
    /// DCB field: `heatGenerationMultiplier` (Single)
    #[serde(default)]
    pub heat_generation_multiplier: f32,
    /// DCB field: `soundRadiusMultiplier` (Single)
    #[serde(default)]
    pub sound_radius_multiplier: f32,
    /// DCB field: `chargeTimeMultiplier` (Single)
    #[serde(default)]
    pub charge_time_multiplier: f32,
    /// DCB field: `useAlternateProjectileVisuals` (Boolean)
    #[serde(default)]
    pub use_alternate_projectile_visuals: bool,
    /// DCB field: `useAugmentedRealityProjectiles` (Boolean)
    #[serde(default)]
    pub use_augmented_reality_projectiles: bool,
    /// DCB field: `disableMisfire` (Boolean)
    #[serde(default)]
    pub disable_misfire: bool,
    /// DCB field: `recoilModifier` (Class)
    #[serde(default)]
    pub recoil_modifier: Option<Handle<SRecoilModifier>>,
    /// DCB field: `spreadModifier` (Class)
    #[serde(default)]
    pub spread_modifier: Option<Handle<SSpreadModifier>>,
    /// DCB field: `aimModifier` (Class)
    #[serde(default)]
    pub aim_modifier: Option<Handle<SAimModifier>>,
    /// DCB field: `regenModifier` (Class)
    #[serde(default)]
    pub regen_modifier: Option<Handle<SRegenConsumerModifier>>,
    /// DCB field: `salvageModifier` (Class)
    #[serde(default)]
    pub salvage_modifier: Option<Handle<SSalvageModifier>>,
}

impl Pooled for SWeaponStats {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_stats }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_stats }
}

impl<'a> Extract<'a> for SWeaponStats {
    const TYPE_NAME: &'static str = "SWeaponStats";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fire_rate: inst.get_i32("fireRate").unwrap_or_default(),
            fire_rate_multiplier: inst.get_f32("fireRateMultiplier").unwrap_or_default(),
            damage_multiplier: inst.get_f32("damageMultiplier").unwrap_or_default(),
            damage_over_time_multiplier: inst.get_f32("damageOverTimeMultiplier").unwrap_or_default(),
            projectile_speed_multiplier: inst.get_f32("projectileSpeedMultiplier").unwrap_or_default(),
            pellets: inst.get_i32("pellets").unwrap_or_default(),
            burst_shots: inst.get_i32("burstShots").unwrap_or_default(),
            ammo_cost: inst.get_i32("ammoCost").unwrap_or_default(),
            ammo_cost_multiplier: inst.get_f32("ammoCostMultiplier").unwrap_or_default(),
            heat_generation_multiplier: inst.get_f32("heatGenerationMultiplier").unwrap_or_default(),
            sound_radius_multiplier: inst.get_f32("soundRadiusMultiplier").unwrap_or_default(),
            charge_time_multiplier: inst.get_f32("chargeTimeMultiplier").unwrap_or_default(),
            use_alternate_projectile_visuals: inst.get_bool("useAlternateProjectileVisuals").unwrap_or_default(),
            use_augmented_reality_projectiles: inst.get_bool("useAugmentedRealityProjectiles").unwrap_or_default(),
            disable_misfire: inst.get_bool("disableMisfire").unwrap_or_default(),
            recoil_modifier: match inst.get("recoilModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRecoilModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRecoilModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spread_modifier: match inst.get("spreadModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSpreadModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSpreadModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aim_modifier: match inst.get("aimModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            regen_modifier: match inst.get("regenModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRegenConsumerModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRegenConsumerModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            salvage_modifier: match inst.get("salvageModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponModifierParams {
    /// DCB field: `weaponStats` (Class)
    #[serde(default)]
    pub weapon_stats: Option<Handle<SWeaponStats>>,
}

impl Pooled for SWeaponModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_modifier_params }
}

impl<'a> Extract<'a> for SWeaponModifierParams {
    const TYPE_NAME: &'static str = "SWeaponModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_stats: match inst.get("weaponStats") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponStats>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponStats>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponMisfireEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponMisfireEntry {
    /// DCB field: `misfireProbabilityCurve` (Class)
    #[serde(default)]
    pub misfire_probability_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `damage` (StrongPointer)
    #[serde(default)]
    pub damage: Option<Handle<DamageBase>>,
    /// DCB field: `hitType` (String)
    #[serde(default)]
    pub hit_type: String,
}

impl Pooled for SWeaponMisfireEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_misfire_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_misfire_entry }
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

/// DCB type: `SWeaponProceduralHandsRecoilCurveConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHandsRecoilCurveConfigDef {
    /// DCB field: `totalRecoilTime` (Single)
    #[serde(default)]
    pub total_recoil_time: f32,
    /// DCB field: `positionRecoilTimeModifiers` (Class)
    #[serde(default)]
    pub position_recoil_time_modifiers: Option<Handle<Vec3>>,
    /// DCB field: `rotationRecoilTimeModifiers` (Class)
    #[serde(default)]
    pub rotation_recoil_time_modifiers: Option<Handle<Vec3>>,
    /// DCB field: `limitTransitionTime` (Single)
    #[serde(default)]
    pub limit_transition_time: f32,
    /// DCB field: `positionCurves` (Class)
    #[serde(default)]
    pub position_curves: Option<Handle<SXYZCurvesWithMaxValues>>,
    /// DCB field: `rotationCurves` (Class)
    #[serde(default)]
    pub rotation_curves: Option<Handle<SXYZCurvesWithMaxValues>>,
    /// DCB field: `minDecayTime` (Single)
    #[serde(default)]
    pub min_decay_time: f32,
    /// DCB field: `maxDecayTime` (Single)
    #[serde(default)]
    pub max_decay_time: f32,
    /// DCB field: `positionDecay` (Class)
    #[serde(default)]
    pub position_decay: Option<Handle<SDecayTimesAndCurves>>,
    /// DCB field: `rotationDecay` (Class)
    #[serde(default)]
    pub rotation_decay: Option<Handle<SDecayTimesAndCurves>>,
    /// DCB field: `rotationOffset` (Class)
    #[serde(default)]
    pub rotation_offset: Option<Handle<Vec3>>,
    /// DCB field: `timeModifier` (StrongPointer)
    #[serde(default)]
    pub time_modifier: Option<Handle<SHandsRecoilTimeModifier>>,
}

impl Pooled for SWeaponProceduralHandsRecoilCurveConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_hands_recoil_curve_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_hands_recoil_curve_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHandsRecoilCurveConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHandsRecoilCurveConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            total_recoil_time: inst.get_f32("totalRecoilTime").unwrap_or_default(),
            position_recoil_time_modifiers: match inst.get("positionRecoilTimeModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_recoil_time_modifiers: match inst.get("rotationRecoilTimeModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            limit_transition_time: inst.get_f32("limitTransitionTime").unwrap_or_default(),
            position_curves: match inst.get("positionCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_curves: match inst.get("rotationCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_decay_time: inst.get_f32("minDecayTime").unwrap_or_default(),
            max_decay_time: inst.get_f32("maxDecayTime").unwrap_or_default(),
            position_decay: match inst.get("positionDecay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayTimesAndCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayTimesAndCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_decay: match inst.get("rotationDecay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayTimesAndCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayTimesAndCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_offset: match inst.get("rotationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time_modifier: match inst.get("timeModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHandsRecoilTimeModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHandsRecoilTimeModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralHandsRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHandsRecoilConfigDef {
    /// DCB field: `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// DCB field: `endDecay` (Single)
    #[serde(default)]
    pub end_decay: f32,
    /// DCB field: `fireRecoilTime` (Single)
    #[serde(default)]
    pub fire_recoil_time: f32,
    /// DCB field: `fireRecoilStrengthFirst` (Single)
    #[serde(default)]
    pub fire_recoil_strength_first: f32,
    /// DCB field: `fireRecoilStrength` (Single)
    #[serde(default)]
    pub fire_recoil_strength: f32,
    /// DCB field: `angleRecoilStrength` (Single)
    #[serde(default)]
    pub angle_recoil_strength: f32,
    /// DCB field: `useRandomRotation` (Boolean)
    #[serde(default)]
    pub use_random_rotation: bool,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// DCB field: `randomness` (Single)
    #[serde(default)]
    pub randomness: f32,
    /// DCB field: `randomnessBackPush` (Single)
    #[serde(default)]
    pub randomness_back_push: f32,
    /// DCB field: `frontalOscillationRotation` (Single)
    #[serde(default)]
    pub frontal_oscillation_rotation: f32,
    /// DCB field: `frontalOscillationStrength` (Single)
    #[serde(default)]
    pub frontal_oscillation_strength: f32,
    /// DCB field: `frontalOscillationDecay` (Single)
    #[serde(default)]
    pub frontal_oscillation_decay: f32,
    /// DCB field: `frontalOscillationRandomness` (Single)
    #[serde(default)]
    pub frontal_oscillation_randomness: f32,
    /// DCB field: `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SWeaponProceduralHandsRecoilCurveConfigDef>>,
}

impl Pooled for SWeaponProceduralHandsRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_hands_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_hands_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHandsRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHandsRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay: inst.get_f32("decay").unwrap_or_default(),
            end_decay: inst.get_f32("endDecay").unwrap_or_default(),
            fire_recoil_time: inst.get_f32("fireRecoilTime").unwrap_or_default(),
            fire_recoil_strength_first: inst.get_f32("fireRecoilStrengthFirst").unwrap_or_default(),
            fire_recoil_strength: inst.get_f32("fireRecoilStrength").unwrap_or_default(),
            angle_recoil_strength: inst.get_f32("angleRecoilStrength").unwrap_or_default(),
            use_random_rotation: inst.get_bool("useRandomRotation").unwrap_or_default(),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            randomness: inst.get_f32("randomness").unwrap_or_default(),
            randomness_back_push: inst.get_f32("randomnessBackPush").unwrap_or_default(),
            frontal_oscillation_rotation: inst.get_f32("frontalOscillationRotation").unwrap_or_default(),
            frontal_oscillation_strength: inst.get_f32("frontalOscillationStrength").unwrap_or_default(),
            frontal_oscillation_decay: inst.get_f32("frontalOscillationDecay").unwrap_or_default(),
            frontal_oscillation_randomness: inst.get_f32("frontalOscillationRandomness").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilCurveConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilCurveConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralAimRecoilCurveConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralAimRecoilCurveConfigDef {
    /// DCB field: `yawMaxDegrees` (Single)
    #[serde(default)]
    pub yaw_max_degrees: f32,
    /// DCB field: `pitchMaxDegrees` (Single)
    #[serde(default)]
    pub pitch_max_degrees: f32,
    /// DCB field: `rollMaxDegrees` (Single)
    #[serde(default)]
    pub roll_max_degrees: f32,
    /// DCB field: `maxFireTime` (Single)
    #[serde(default)]
    pub max_fire_time: f32,
    /// DCB field: `recoilSmoothTime` (Single)
    #[serde(default)]
    pub recoil_smooth_time: f32,
    /// DCB field: `minLimits` (Class)
    #[serde(default)]
    pub min_limits: Option<Handle<Vec3>>,
    /// DCB field: `maxLimits` (Class)
    #[serde(default)]
    pub max_limits: Option<Handle<Vec3>>,
    /// DCB field: `yawPitchRollCurves` (StrongPointer)
    #[serde(default)]
    pub yaw_pitch_roll_curves: Option<Handle<SYawPitchRollCurves>>,
    /// DCB field: `decayStartTime` (Single)
    #[serde(default)]
    pub decay_start_time: f32,
    /// DCB field: `minDecayTime` (Single)
    #[serde(default)]
    pub min_decay_time: f32,
    /// DCB field: `maxDecayTime` (Single)
    #[serde(default)]
    pub max_decay_time: f32,
    /// DCB field: `yawPitchRollDecayCurves` (StrongPointer)
    #[serde(default)]
    pub yaw_pitch_roll_decay_curves: Option<Handle<SYawPitchRollCurves>>,
    /// DCB field: `noiseCurves` (Class)
    #[serde(default)]
    pub noise_curves: Option<Handle<SAimRecoilNoiseCurves>>,
}

impl Pooled for SWeaponProceduralAimRecoilCurveConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_aim_recoil_curve_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_aim_recoil_curve_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralAimRecoilCurveConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralAimRecoilCurveConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            yaw_max_degrees: inst.get_f32("yawMaxDegrees").unwrap_or_default(),
            pitch_max_degrees: inst.get_f32("pitchMaxDegrees").unwrap_or_default(),
            roll_max_degrees: inst.get_f32("rollMaxDegrees").unwrap_or_default(),
            max_fire_time: inst.get_f32("maxFireTime").unwrap_or_default(),
            recoil_smooth_time: inst.get_f32("recoilSmoothTime").unwrap_or_default(),
            min_limits: match inst.get("minLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_limits: match inst.get("maxLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw_pitch_roll_curves: match inst.get("yawPitchRollCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SYawPitchRollCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SYawPitchRollCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_start_time: inst.get_f32("decayStartTime").unwrap_or_default(),
            min_decay_time: inst.get_f32("minDecayTime").unwrap_or_default(),
            max_decay_time: inst.get_f32("maxDecayTime").unwrap_or_default(),
            yaw_pitch_roll_decay_curves: match inst.get("yawPitchRollDecayCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SYawPitchRollCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SYawPitchRollCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_curves: match inst.get("noiseCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimRecoilNoiseCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimRecoilNoiseCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralAimRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralAimRecoilConfigDef {
    /// DCB field: `max` (Class)
    #[serde(default)]
    pub max: Option<Handle<Vec2>>,
    /// DCB field: `pull_left_percentage` (Single)
    #[serde(default)]
    pub pull_left_percentage: f32,
    /// DCB field: `shot_kick_first` (Class)
    #[serde(default)]
    pub shot_kick_first: Option<Handle<Vec2>>,
    /// DCB field: `shot_kick` (Class)
    #[serde(default)]
    pub shot_kick: Option<Handle<Vec2>>,
    /// DCB field: `random_pitch` (Single)
    #[serde(default)]
    pub random_pitch: f32,
    /// DCB field: `random_yaw` (Single)
    #[serde(default)]
    pub random_yaw: f32,
    /// DCB field: `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// DCB field: `end_decay` (Single)
    #[serde(default)]
    pub end_decay: f32,
    /// DCB field: `recoil_time` (Single)
    #[serde(default)]
    pub recoil_time: f32,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
    /// DCB field: `curveAimRecoil` (Class)
    #[serde(default)]
    pub curve_aim_recoil: Option<Handle<SWeaponProceduralAimRecoilCurveConfigDef>>,
}

impl Pooled for SWeaponProceduralAimRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_aim_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_aim_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralAimRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralAimRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max: match inst.get("max") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pull_left_percentage: inst.get_f32("pull_left_percentage").unwrap_or_default(),
            shot_kick_first: match inst.get("shot_kick_first") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shot_kick: match inst.get("shot_kick") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_pitch: inst.get_f32("random_pitch").unwrap_or_default(),
            random_yaw: inst.get_f32("random_yaw").unwrap_or_default(),
            decay: inst.get_f32("decay").unwrap_or_default(),
            end_decay: inst.get_f32("end_decay").unwrap_or_default(),
            recoil_time: inst.get_f32("recoil_time").unwrap_or_default(),
            delay: inst.get_f32("delay").unwrap_or_default(),
            curve_aim_recoil: match inst.get("curveAimRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilCurveConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilCurveConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralBodyRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralBodyRecoilConfigDef {
    /// DCB field: `hipsPushForce` (Single)
    #[serde(default)]
    pub hips_push_force: f32,
    /// DCB field: `hipsDampStrength` (Single)
    #[serde(default)]
    pub hips_damp_strength: f32,
    /// DCB field: `hipsDampStrengthEnd` (Single)
    #[serde(default)]
    pub hips_damp_strength_end: f32,
    /// DCB field: `spinePushForceFirst` (Single)
    #[serde(default)]
    pub spine_push_force_first: f32,
    /// DCB field: `spinePushForce` (Single)
    #[serde(default)]
    pub spine_push_force: f32,
    /// DCB field: `spineDampStrength` (Single)
    #[serde(default)]
    pub spine_damp_strength: f32,
    /// DCB field: `spineDampStrengthEnd` (Single)
    #[serde(default)]
    pub spine_damp_strength_end: f32,
}

impl Pooled for SWeaponProceduralBodyRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_body_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_body_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralBodyRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralBodyRecoilConfigDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hips_push_force: inst.get_f32("hipsPushForce").unwrap_or_default(),
            hips_damp_strength: inst.get_f32("hipsDampStrength").unwrap_or_default(),
            hips_damp_strength_end: inst.get_f32("hipsDampStrengthEnd").unwrap_or_default(),
            spine_push_force_first: inst.get_f32("spinePushForceFirst").unwrap_or_default(),
            spine_push_force: inst.get_f32("spinePushForce").unwrap_or_default(),
            spine_damp_strength: inst.get_f32("spineDampStrength").unwrap_or_default(),
            spine_damp_strength_end: inst.get_f32("spineDampStrengthEnd").unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeaponProceduralHeadRecoilCurveConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHeadRecoilCurveConfigDef {
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<SVecWithNoiseParams>>,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<SVecWithNoiseParams>>,
    /// DCB field: `curves` (StrongPointer)
    #[serde(default)]
    pub curves: Option<Handle<SAmplitudeFreqencyDecayCurves>>,
    /// DCB field: `headRecoilTime` (Single)
    #[serde(default)]
    pub head_recoil_time: f32,
    /// DCB field: `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// DCB field: `smoothingSpeed` (Single)
    #[serde(default)]
    pub smoothing_speed: f32,
}

impl Pooled for SWeaponProceduralHeadRecoilCurveConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_head_recoil_curve_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_head_recoil_curve_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHeadRecoilCurveConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHeadRecoilCurveConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVecWithNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVecWithNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVecWithNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVecWithNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            curves: match inst.get("curves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAmplitudeFreqencyDecayCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAmplitudeFreqencyDecayCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil_time: inst.get_f32("headRecoilTime").unwrap_or_default(),
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            smoothing_speed: inst.get_f32("smoothingSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeaponProceduralHeadRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHeadRecoilConfigDef {
    /// DCB field: `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// DCB field: `smoothFactor` (Single)
    #[serde(default)]
    pub smooth_factor: f32,
    /// DCB field: `frequencyNoiseFactor` (Single)
    #[serde(default)]
    pub frequency_noise_factor: f32,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// DCB field: `phase` (Single)
    #[serde(default)]
    pub phase: f32,
    /// DCB field: `translation` (Class)
    #[serde(default)]
    pub translation: Option<Handle<Vec3>>,
    /// DCB field: `translationNoise` (Single)
    #[serde(default)]
    pub translation_noise: f32,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// DCB field: `rotationNoise` (Single)
    #[serde(default)]
    pub rotation_noise: f32,
    /// DCB field: `usePerlinNoise` (Boolean)
    #[serde(default)]
    pub use_perlin_noise: bool,
    /// DCB field: `referenceSpeed` (Single)
    #[serde(default)]
    pub reference_speed: f32,
    /// DCB field: `minSpeed` (Single)
    #[serde(default)]
    pub min_speed: f32,
    /// DCB field: `minScale` (Single)
    #[serde(default)]
    pub min_scale: f32,
    /// DCB field: `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
    /// DCB field: `maxScale` (Single)
    #[serde(default)]
    pub max_scale: f32,
    /// DCB field: `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SWeaponProceduralHeadRecoilCurveConfigDef>>,
}

impl Pooled for SWeaponProceduralHeadRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sw.sweapon_procedural_head_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sw.sweapon_procedural_head_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHeadRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHeadRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            smooth_factor: inst.get_f32("smoothFactor").unwrap_or_default(),
            frequency_noise_factor: inst.get_f32("frequencyNoiseFactor").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            phase: inst.get_f32("phase").unwrap_or_default(),
            translation: match inst.get("translation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            translation_noise: inst.get_f32("translationNoise").unwrap_or_default(),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_noise: inst.get_f32("rotationNoise").unwrap_or_default(),
            use_perlin_noise: inst.get_bool("usePerlinNoise").unwrap_or_default(),
            reference_speed: inst.get_f32("referenceSpeed").unwrap_or_default(),
            min_speed: inst.get_f32("minSpeed").unwrap_or_default(),
            min_scale: inst.get_f32("minScale").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
            max_scale: inst.get_f32("maxScale").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

