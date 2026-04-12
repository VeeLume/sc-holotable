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

/// DCB type: `CyclingChannelOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclingChannelOption {
    /// DCB field: `cyclingChannelCount` (UInt32)
    #[serde(default)]
    pub cycling_channel_count: u32,
    /// DCB field: `cycleChannelConnectionWaitSecond` (Single)
    #[serde(default)]
    pub cycle_channel_connection_wait_second: f32,
    /// DCB field: `autoRegisterCyclingChannelTypes` (StrongPointer (array))
    #[serde(default)]
    pub auto_register_cycling_channel_types: Vec<Handle<SChatChannelTypeBase>>,
}

impl Pooled for CyclingChannelOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cy.cycling_channel_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cy.cycling_channel_option }
}

impl<'a> Extract<'a> for CyclingChannelOption {
    const TYPE_NAME: &'static str = "CyclingChannelOption";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cycling_channel_count: inst.get_u32("cyclingChannelCount").unwrap_or_default(),
            cycle_channel_connection_wait_second: inst.get_f32("cycleChannelConnectionWaitSecond").unwrap_or_default(),
            auto_register_cycling_channel_types: inst.get_array("autoRegisterCyclingChannelTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SChatChannelTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SChatChannelTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

