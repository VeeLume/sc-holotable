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

/// Record index for the `weaponmisfiredef` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WeaponmisfiredefIndex {
    #[serde(default)]
    pub weapon_misfire_def: HashMap<CigGuid, Handle<WeaponMisfireDef>>,
}

impl WeaponmisfiredefIndex {
    pub fn len(&self) -> usize {
        self.weapon_misfire_def.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
