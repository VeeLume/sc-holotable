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

/// Record index for the `entities-scitem-usables` feature.
#[derive(Default)]
pub struct EntitiesScitemUsablesIndex {
    pub usable_archetypes: HashMap<CigGuid, Handle<UsableArchetypes>>,
}

impl EntitiesScitemUsablesIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.usable_archetypes.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
