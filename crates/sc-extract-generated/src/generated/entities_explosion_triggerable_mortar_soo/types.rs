// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-explosion_triggerable_mortar_soo`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ExplosionGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
pub struct ExplosionGameplayTrigger {
    /// `damageScalar` (Single)
    pub damage_scalar: f32,
}

impl Pooled for ExplosionGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_explosion_triggerable_mortar_soo
            .explosion_gameplay_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_explosion_triggerable_mortar_soo
            .explosion_gameplay_trigger
    }
}

impl<'a> Extract<'a> for ExplosionGameplayTrigger {
    const TYPE_NAME: &'static str = "ExplosionGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_scalar: inst.get_f32("damageScalar").unwrap_or_default(),
        }
    }
}
