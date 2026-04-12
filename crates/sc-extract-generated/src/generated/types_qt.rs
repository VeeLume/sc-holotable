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

/// DCB type: `QTERequestConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QTERequestConfig {
    /// DCB field: `blockActions` (Boolean)
    #[serde(default)]
    pub block_actions: bool,
    /// DCB field: `actionName` (Class)
    #[serde(default)]
    pub action_name: Option<Handle<InputAction>>,
    /// DCB field: `totalPressNum` (Int32)
    #[serde(default)]
    pub total_press_num: i32,
    /// DCB field: `maxQTETime` (Single)
    #[serde(default)]
    pub max_qtetime: f32,
    /// DCB field: `decayGracePeriod` (Single)
    #[serde(default)]
    pub decay_grace_period: f32,
    /// DCB field: `decayPerSecond` (Single)
    #[serde(default)]
    pub decay_per_second: f32,
    /// DCB field: `givewayBehaviour` (EnumChoice)
    #[serde(default)]
    pub giveway_behaviour: String,
    /// DCB field: `ownerName` (String)
    #[serde(default)]
    pub owner_name: String,
    /// DCB field: `QTEPriority` (EnumChoice)
    #[serde(default)]
    pub qtepriority: String,
    /// DCB field: `inputPromptConfig` (Reference)
    #[serde(default)]
    pub input_prompt_config: Option<CigGuid>,
}

impl Pooled for QTERequestConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qt.qterequest_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qt.qterequest_config }
}

impl<'a> Extract<'a> for QTERequestConfig {
    const TYPE_NAME: &'static str = "QTERequestConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            block_actions: inst.get_bool("blockActions").unwrap_or_default(),
            action_name: match inst.get("actionName") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            total_press_num: inst.get_i32("totalPressNum").unwrap_or_default(),
            max_qtetime: inst.get_f32("maxQTETime").unwrap_or_default(),
            decay_grace_period: inst.get_f32("decayGracePeriod").unwrap_or_default(),
            decay_per_second: inst.get_f32("decayPerSecond").unwrap_or_default(),
            giveway_behaviour: inst.get_str("givewayBehaviour").map(String::from).unwrap_or_default(),
            owner_name: inst.get_str("ownerName").map(String::from).unwrap_or_default(),
            qtepriority: inst.get_str("QTEPriority").map(String::from).unwrap_or_default(),
            input_prompt_config: inst.get("inputPromptConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

