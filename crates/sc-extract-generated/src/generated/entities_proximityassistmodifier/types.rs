// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-proximityassistmodifier`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ProximityAssistModifierParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ProximityAssistModifierParams {
    /// `strengthModifier` (Single)
    pub strength_modifier: f32,
}

impl Pooled for ProximityAssistModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_proximityassistmodifier.proximity_assist_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_proximityassistmodifier.proximity_assist_modifier_params }
}

impl<'a> Extract<'a> for ProximityAssistModifierParams {
    const TYPE_NAME: &'static str = "ProximityAssistModifierParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strength_modifier: inst.get_f32("strengthModifier").unwrap_or_default(),
        }
    }
}

