// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-quantumcolorshift`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `QuantumColorShiftParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumColorShiftParams {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `strength` (Single)
    #[serde(default)]
    pub strength: f32,
    /// `startDistance` (Single)
    #[serde(default)]
    pub start_distance: f32,
}

impl Pooled for QuantumColorShiftParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_quantumcolorshift.quantum_color_shift_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_quantumcolorshift.quantum_color_shift_params }
}

impl<'a> Extract<'a> for QuantumColorShiftParams {
    const TYPE_NAME: &'static str = "QuantumColorShiftParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            strength: inst.get_f32("strength").unwrap_or_default(),
            start_distance: inst.get_f32("startDistance").unwrap_or_default(),
        }
    }
}

