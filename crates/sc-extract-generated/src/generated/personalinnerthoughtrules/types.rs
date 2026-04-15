// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `personalinnerthoughtrules`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ActionRuleNotAllowedInContext`
/// Inherits from: `ActionRuleParams`
pub struct ActionRuleNotAllowedInContext {
    /// `ruleDisplay` (StrongPointer)
    pub rule_display: Option<Handle<ActionRuleDisplayParams>>,
    /// `context` (EnumChoice)
    pub context: PersonalThoughtContext,
}

impl Pooled for ActionRuleNotAllowedInContext {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.personalinnerthoughtrules.action_rule_not_allowed_in_context }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.personalinnerthoughtrules.action_rule_not_allowed_in_context }
}

impl<'a> Extract<'a> for ActionRuleNotAllowedInContext {
    const TYPE_NAME: &'static str = "ActionRuleNotAllowedInContext";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rule_display: match inst.get("ruleDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActionRuleDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            context: PersonalThoughtContext::from_dcb_str(inst.get_str("context").unwrap_or("")),
        }
    }
}

