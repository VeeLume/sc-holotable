// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `unittest_unittestb`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `UnitTestSubRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTestSubRecord {
}

impl Pooled for UnitTestSubRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.unittest_unittestb.unit_test_sub_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.unittest_unittestb.unit_test_sub_record }
}

impl<'a> Extract<'a> for UnitTestSubRecord {
    const TYPE_NAME: &'static str = "UnitTestSubRecord";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

