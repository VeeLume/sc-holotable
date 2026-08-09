// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-defendarea_inareaassignment`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `GameplayTrigger_TargetType_ActionArea`
/// Inherits from: `GameplayTrigger_TargetType_Base`
pub struct GameplayTrigger_TargetType_ActionArea {
    /// `optionalTarget` (Boolean)
    pub optional_target: bool,
}

impl Pooled for GameplayTrigger_TargetType_ActionArea {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_defendarea_inareaassignment
            .gameplay_trigger_target_type_action_area
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_defendarea_inareaassignment
            .gameplay_trigger_target_type_action_area
    }
}

impl<'a> Extract<'a> for GameplayTrigger_TargetType_ActionArea {
    const TYPE_NAME: &'static str = "GameplayTrigger_TargetType_ActionArea";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            optional_target: inst.get_bool("optionalTarget").unwrap_or_default(),
        }
    }
}
