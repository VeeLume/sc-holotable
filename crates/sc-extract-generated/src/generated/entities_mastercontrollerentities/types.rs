// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-mastercontrollerentities`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `TagEntityFilter`
/// Inherits from: `EntityFilter`
pub struct TagEntityFilter {
    /// `entityTags` (Class)
    pub entity_tags: Option<Handle<TagList>>,
}

impl Pooled for TagEntityFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_mastercontrollerentities.tag_entity_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_mastercontrollerentities.tag_entity_filter }
}

impl<'a> Extract<'a> for TagEntityFilter {
    const TYPE_NAME: &'static str = "TagEntityFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_tags: match inst.get("entityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

