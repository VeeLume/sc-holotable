// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `unittest_unittestb` feature.
#[derive(Default)]
pub struct Unittest_unittestbIndex {
    pub unit_test_sub_record: HashMap<CigGuid, Handle<UnitTestSubRecord>>,
}

impl Unittest_unittestbIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.unit_test_sub_record.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
