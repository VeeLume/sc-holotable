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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `TagEntityFilter`
/// Inherits from: `EntityFilter`
pub struct TagEntityFilter {
    /// `entityTags` (Class)
    pub entity_tags: Option<Handle<TagList>>,
}

impl Pooled for TagEntityFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_mastercontrollerentities.tag_entity_filter
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_mastercontrollerentities.tag_entity_filter
    }
}

impl<'a> Extract<'a> for TagEntityFilter {
    const TYPE_NAME: &'static str = "TagEntityFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_tags: match inst.get("entityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `UserVariableCheckIntGreater`
/// Inherits from: `UserVariableCheck`
pub struct UserVariableCheckIntGreater {
    /// `variableName` (String)
    pub variable_name: String,
    /// `valueToCheck` (Int32)
    pub value_to_check: i32,
}

impl Pooled for UserVariableCheckIntGreater {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_mastercontrollerentities
            .user_variable_check_int_greater
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_mastercontrollerentities
            .user_variable_check_int_greater
    }
}

impl<'a> Extract<'a> for UserVariableCheckIntGreater {
    const TYPE_NAME: &'static str = "UserVariableCheckIntGreater";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            value_to_check: inst.get_i32("valueToCheck").unwrap_or_default(),
        }
    }
}
