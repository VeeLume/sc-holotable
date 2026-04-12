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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PersonalInnerThoughtActionRulePreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInnerThoughtActionRulePreset {
    /// `rules` (StrongPointer (array))
    #[serde(default)]
    pub rules: Vec<Handle<ActionRuleParams>>,
}

impl Pooled for PersonalInnerThoughtActionRulePreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.personalinnerthoughtrules.personal_inner_thought_action_rule_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.personalinnerthoughtrules.personal_inner_thought_action_rule_preset }
}

impl<'a> Extract<'a> for PersonalInnerThoughtActionRulePreset {
    const TYPE_NAME: &'static str = "PersonalInnerThoughtActionRulePreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActionRuleParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActionRuleParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

