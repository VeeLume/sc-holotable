// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `dialoguerealm`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `DialogueRealm`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueRealm {
    /// `defaultTrigger` (Class)
    #[serde(default)]
    pub default_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `defaultFemaleTrigger` (Class)
    #[serde(default)]
    pub default_female_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for DialogueRealm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dialoguerealm.dialogue_realm }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dialoguerealm.dialogue_realm }
}

impl<'a> Extract<'a> for DialogueRealm {
    const TYPE_NAME: &'static str = "DialogueRealm";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_trigger: match inst.get("defaultTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_female_trigger: match inst.get("defaultFemaleTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

