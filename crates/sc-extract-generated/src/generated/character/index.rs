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

/// Record index for the `character` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterIndex {
    #[serde(default)]
    pub character: HashMap<CigGuid, Handle<Character>>,
    #[serde(default)]
    pub default_player_loadout_entitlement_record: HashMap<CigGuid, Handle<DefaultPlayerLoadoutEntitlementRecord>>,
}

impl CharacterIndex {
    pub fn len(&self) -> usize {
        self.character.len()
            + self.default_player_loadout_entitlement_record.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
