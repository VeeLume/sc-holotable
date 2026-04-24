// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-destruction`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `DamageResistanceReference`
/// Inherits from: `DamageResistanceBase`
pub struct DamageResistanceReference {
    /// `damageResistanceRecord` (Reference)
    pub damage_resistance_record: Option<CigGuid>,
}

impl Pooled for DamageResistanceReference {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_destruction.damage_resistance_reference
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_destruction.damage_resistance_reference
    }
}

impl<'a> Extract<'a> for DamageResistanceReference {
    const TYPE_NAME: &'static str = "DamageResistanceReference";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_resistance_record: inst
                .get("damageResistanceRecord")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}
