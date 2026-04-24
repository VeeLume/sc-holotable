// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-displayscreens`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SelfStateChange`
/// Inherits from: `SelfCommunicationMessage`
pub struct SelfStateChange {
    /// `targetSelfState` (WeakPointer)
    pub target_self_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SelfStateChange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_displayscreens.self_state_change }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_displayscreens.self_state_change }
}

impl<'a> Extract<'a> for SelfStateChange {
    const TYPE_NAME: &'static str = "SelfStateChange";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            target_self_state: match inst.get("targetSelfState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

