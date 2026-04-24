// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sglobalhitbehaviorparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SGlobalHitBehaviorParams`
pub struct SGlobalHitBehaviorParams {
    /// `damagePerTickUpperLimit` (Single)
    pub damage_per_tick_upper_limit: f32,
    /// `timeUpperLimit` (Single)
    pub time_upper_limit: f32,
}

impl Pooled for SGlobalHitBehaviorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalhitbehaviorparams.sglobal_hit_behavior_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalhitbehaviorparams.sglobal_hit_behavior_params }
}

impl<'a> Extract<'a> for SGlobalHitBehaviorParams {
    const TYPE_NAME: &'static str = "SGlobalHitBehaviorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_per_tick_upper_limit: inst.get_f32("damagePerTickUpperLimit").unwrap_or_default(),
            time_upper_limit: inst.get_f32("timeUpperLimit").unwrap_or_default(),
        }
    }
}

