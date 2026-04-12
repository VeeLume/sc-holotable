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

/// DCB type: `FPSReticle_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FPSReticle_Config {
    /// DCB field: `AimAveragePoints` (Byte)
    #[serde(default)]
    pub aim_average_points: u32,
    /// DCB field: `FlashWidth` (UInt16)
    #[serde(default)]
    pub flash_width: u32,
    /// DCB field: `FlashHeight` (UInt16)
    #[serde(default)]
    pub flash_height: u32,
    /// DCB field: `SpreadSize` (Single)
    #[serde(default)]
    pub spread_size: f32,
    /// DCB field: `SpreadScaleMax` (Single)
    #[serde(default)]
    pub spread_scale_max: f32,
    /// DCB field: `SpreadScaleInterpNeg` (Single)
    #[serde(default)]
    pub spread_scale_interp_neg: f32,
    /// DCB field: `SpreadScaleInterpPos` (Single)
    #[serde(default)]
    pub spread_scale_interp_pos: f32,
    /// DCB field: `SpreadAlphaInterpNeg` (Single)
    #[serde(default)]
    pub spread_alpha_interp_neg: f32,
    /// DCB field: `SpreadAlphaInterpPos` (Single)
    #[serde(default)]
    pub spread_alpha_interp_pos: f32,
    /// DCB field: `HiddenAlphaInterpNeg` (Single)
    #[serde(default)]
    pub hidden_alpha_interp_neg: f32,
    /// DCB field: `MoveAlphaMinimum` (Single)
    #[serde(default)]
    pub move_alpha_minimum: f32,
    /// DCB field: `MoveAlphaRange` (Single)
    #[serde(default)]
    pub move_alpha_range: f32,
}

impl Pooled for FPSReticle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fp.fpsreticle_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fp.fpsreticle_config }
}

impl<'a> Extract<'a> for FPSReticle_Config {
    const TYPE_NAME: &'static str = "FPSReticle_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            aim_average_points: inst.get_u32("AimAveragePoints").unwrap_or_default(),
            flash_width: inst.get_u32("FlashWidth").unwrap_or_default(),
            flash_height: inst.get_u32("FlashHeight").unwrap_or_default(),
            spread_size: inst.get_f32("SpreadSize").unwrap_or_default(),
            spread_scale_max: inst.get_f32("SpreadScaleMax").unwrap_or_default(),
            spread_scale_interp_neg: inst.get_f32("SpreadScaleInterpNeg").unwrap_or_default(),
            spread_scale_interp_pos: inst.get_f32("SpreadScaleInterpPos").unwrap_or_default(),
            spread_alpha_interp_neg: inst.get_f32("SpreadAlphaInterpNeg").unwrap_or_default(),
            spread_alpha_interp_pos: inst.get_f32("SpreadAlphaInterpPos").unwrap_or_default(),
            hidden_alpha_interp_neg: inst.get_f32("HiddenAlphaInterpNeg").unwrap_or_default(),
            move_alpha_minimum: inst.get_f32("MoveAlphaMinimum").unwrap_or_default(),
            move_alpha_range: inst.get_f32("MoveAlphaRange").unwrap_or_default(),
        }
    }
}

