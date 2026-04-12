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

/// DCB type: `HygieneParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HygieneParams {
    /// DCB field: `sweatingParams` (StrongPointer)
    #[serde(default)]
    pub sweating_params: Option<Handle<StatusSweatingParams>>,
    /// DCB field: `bloodParams` (StrongPointer)
    #[serde(default)]
    pub blood_params: Option<Handle<StatusBloodParams>>,
}

impl Pooled for HygieneParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_hy.hygiene_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_hy.hygiene_params }
}

impl<'a> Extract<'a> for HygieneParams {
    const TYPE_NAME: &'static str = "HygieneParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sweating_params: match inst.get("sweatingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusSweatingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusSweatingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blood_params: match inst.get("bloodParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusBloodParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusBloodParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

