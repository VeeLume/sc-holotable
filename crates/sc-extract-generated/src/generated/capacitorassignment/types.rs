// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `capacitorassignment`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CapacitorAssignmentInputOutputDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacitorAssignmentInputOutputDef {
    /// `inputOutputMapping` (Class)
    #[serde(default)]
    pub input_output_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for CapacitorAssignmentInputOutputDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.capacitorassignment.capacitor_assignment_input_output_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.capacitorassignment.capacitor_assignment_input_output_def }
}

impl<'a> Extract<'a> for CapacitorAssignmentInputOutputDef {
    const TYPE_NAME: &'static str = "CapacitorAssignmentInputOutputDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_output_mapping: match inst.get("inputOutputMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

