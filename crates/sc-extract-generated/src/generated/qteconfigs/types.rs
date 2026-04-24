// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `qteconfigs`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `QTERequestConfig`
pub struct QTERequestConfig {
    /// `blockActions` (Boolean)
    pub block_actions: bool,
    /// `actionName` (Class)
    pub action_name: Option<Handle<InputAction>>,
    /// `totalPressNum` (Int32)
    pub total_press_num: i32,
    /// `maxQTETime` (Single)
    pub max_qtetime: f32,
    /// `decayGracePeriod` (Single)
    pub decay_grace_period: f32,
    /// `decayPerSecond` (Single)
    pub decay_per_second: f32,
    /// `givewayBehaviour` (EnumChoice)
    pub giveway_behaviour: EQTEPriorityGivewayBehaviour,
    /// `ownerName` (String)
    pub owner_name: String,
    /// `QTEPriority` (EnumChoice)
    pub qtepriority: EQTEPriority,
    /// `inputPromptConfig` (Reference)
    pub input_prompt_config: Option<CigGuid>,
}

impl Pooled for QTERequestConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.qteconfigs.qterequest_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.qteconfigs.qterequest_config }
}

impl<'a> Extract<'a> for QTERequestConfig {
    const TYPE_NAME: &'static str = "QTERequestConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            block_actions: inst.get_bool("blockActions").unwrap_or_default(),
            action_name: match inst.get("actionName") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            total_press_num: inst.get_i32("totalPressNum").unwrap_or_default(),
            max_qtetime: inst.get_f32("maxQTETime").unwrap_or_default(),
            decay_grace_period: inst.get_f32("decayGracePeriod").unwrap_or_default(),
            decay_per_second: inst.get_f32("decayPerSecond").unwrap_or_default(),
            giveway_behaviour: EQTEPriorityGivewayBehaviour::from_dcb_str(inst.get_str("givewayBehaviour").unwrap_or("")),
            owner_name: inst.get_str("ownerName").map(String::from).unwrap_or_default(),
            qtepriority: EQTEPriority::from_dcb_str(inst.get_str("QTEPriority").unwrap_or("")),
            input_prompt_config: inst.get("inputPromptConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

