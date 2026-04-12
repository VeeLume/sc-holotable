// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `weaponproceduralrecoil` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WeaponproceduralrecoilPools {
    #[serde(default)]
    pub sxyzcurves: Vec<Option<SXYZCurves>>,
    #[serde(default)]
    pub scurve: Vec<Option<SCurve>>,
    #[serde(default)]
    pub sxyzcurves_arrays: Vec<Option<SXYZCurvesArrays>>,
    #[serde(default)]
    pub shands_recoil_curve_noise_params: Vec<Option<SHandsRecoilCurveNoiseParams>>,
    #[serde(default)]
    pub sxyzcurves_with_max_values: Vec<Option<SXYZCurvesWithMaxValues>>,
    #[serde(default)]
    pub sdecay_curve_max_value_params: Vec<Option<SDecayCurveMaxValueParams>>,
    #[serde(default)]
    pub sdecay_curve_max_values: Vec<Option<SDecayCurveMaxValues>>,
    #[serde(default)]
    pub sdecay_times_and_curves: Vec<Option<SDecayTimesAndCurves>>,
    #[serde(default)]
    pub shands_recoil_time_modifier: Vec<Option<SHandsRecoilTimeModifier>>,
    #[serde(default)]
    pub sweapon_procedural_hands_recoil_curve_config_def: Vec<Option<SWeaponProceduralHandsRecoilCurveConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_hands_recoil_config_def: Vec<Option<SWeaponProceduralHandsRecoilConfigDef>>,
    #[serde(default)]
    pub syaw_pitch_roll_curves: Vec<Option<SYawPitchRollCurves>>,
    #[serde(default)]
    pub saim_recoil_noise_curves: Vec<Option<SAimRecoilNoiseCurves>>,
    #[serde(default)]
    pub sweapon_procedural_aim_recoil_curve_config_def: Vec<Option<SWeaponProceduralAimRecoilCurveConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_aim_recoil_config_def: Vec<Option<SWeaponProceduralAimRecoilConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_body_recoil_config_def: Vec<Option<SWeaponProceduralBodyRecoilConfigDef>>,
    #[serde(default)]
    pub shead_recoil_noise_params: Vec<Option<SHeadRecoilNoiseParams>>,
    #[serde(default)]
    pub svec_with_noise_params: Vec<Option<SVecWithNoiseParams>>,
    #[serde(default)]
    pub samplitude_freqency_decay_curves: Vec<Option<SAmplitudeFreqencyDecayCurves>>,
    #[serde(default)]
    pub sweapon_procedural_head_recoil_curve_config_def: Vec<Option<SWeaponProceduralHeadRecoilCurveConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_head_recoil_config_def: Vec<Option<SWeaponProceduralHeadRecoilConfigDef>>,
    #[serde(default)]
    pub weapon_procedural_recoil_config_def: Vec<Option<WeaponProceduralRecoilConfigDef>>,
}
