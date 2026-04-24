// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `environments` feature.
#[derive(Default)]
pub struct EnvironmentsIndex {
    pub asteroid_field_composition: HashMap<CigGuid, Handle<AsteroidFieldComposition>>,
}

impl EnvironmentsIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.asteroid_field_composition.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
