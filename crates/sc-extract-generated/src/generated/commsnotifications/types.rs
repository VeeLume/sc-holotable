// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `commsnotifications`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CommsNotification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotification {
    /// `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<TagList>>,
    /// `communicationName` (Reference)
    #[serde(default)]
    pub communication_name: Option<CigGuid>,
    /// `characterEntityClass` (Reference)
    #[serde(default)]
    pub character_entity_class: Option<CigGuid>,
    /// `fakeCommsAudioEntityClass` (Reference)
    #[serde(default)]
    pub fake_comms_audio_entity_class: Option<CigGuid>,
    /// `stage` (Reference)
    #[serde(default)]
    pub stage: Option<CigGuid>,
}

impl Pooled for CommsNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.commsnotifications.comms_notification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.commsnotifications.comms_notification }
}

impl<'a> Extract<'a> for CommsNotification {
    const TYPE_NAME: &'static str = "CommsNotification";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            communication_name: inst.get("communicationName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            character_entity_class: inst.get("characterEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fake_comms_audio_entity_class: inst.get("fakeCommsAudioEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            stage: inst.get("stage").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

