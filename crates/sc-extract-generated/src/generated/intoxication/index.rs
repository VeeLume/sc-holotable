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

/// Record index for the `intoxication` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntoxicationIndex {
    #[serde(default)]
    pub intoxication_ifcsmodifier_params: HashMap<CigGuid, Handle<IntoxicationIFCSModifierParams>>,
    #[serde(default)]
    pub intoxication_turret_modifier_params: HashMap<CigGuid, Handle<IntoxicationTurretModifierParams>>,
    #[serde(default)]
    pub intoxication_wheeled_modifier_params: HashMap<CigGuid, Handle<IntoxicationWheeledModifierParams>>,
}

impl IntoxicationIndex {
    pub fn len(&self) -> usize {
        self.intoxication_ifcsmodifier_params.len()
            + self.intoxication_turret_modifier_params.len()
            + self.intoxication_wheeled_modifier_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
