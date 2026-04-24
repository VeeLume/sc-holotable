// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `handholdgripdatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `HandholdGripDatabase`
pub struct HandholdGripDatabase {
    /// `gripTypes` (Reference (array))
    pub grip_types: Vec<CigGuid>,
}

impl Pooled for HandholdGripDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.handholdgripdatabase.handhold_grip_database
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.handholdgripdatabase.handhold_grip_database
    }
}

impl<'a> Extract<'a> for HandholdGripDatabase {
    const TYPE_NAME: &'static str = "HandholdGripDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grip_types: inst
                .get_array("gripTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
