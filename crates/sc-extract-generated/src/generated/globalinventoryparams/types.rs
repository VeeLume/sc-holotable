// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalinventoryparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `InteractionPointTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPointTemplate {
    /// `interactionPoint` (Class)
    #[serde(default)]
    pub interaction_point: Option<Handle<SInteractionPointParams>>,
}

impl Pooled for InteractionPointTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalinventoryparams.interaction_point_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalinventoryparams.interaction_point_template }
}

impl<'a> Extract<'a> for InteractionPointTemplate {
    const TYPE_NAME: &'static str = "InteractionPointTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interaction_point: match inst.get("interactionPoint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInteractionPointParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionPointParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

