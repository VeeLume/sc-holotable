// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `damage` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DamageIndex {
    #[serde(default)]
    pub damage_macro: HashMap<CigGuid, Handle<DamageMacro>>,
    #[serde(default)]
    pub damage_resistance_macro: HashMap<CigGuid, Handle<DamageResistanceMacro>>,
}

impl DamageIndex {
    pub fn len(&self) -> usize {
        self.damage_macro.len()
            + self.damage_resistance_macro.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
