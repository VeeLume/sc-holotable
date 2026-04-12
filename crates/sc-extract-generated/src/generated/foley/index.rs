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

/// Record index for the `foley` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoleyIndex {
    #[serde(default)]
    pub foley_definition: HashMap<CigGuid, Handle<FoleyDefinition>>,
    #[serde(default)]
    pub foley_bone: HashMap<CigGuid, Handle<FoleyBone>>,
    #[serde(default)]
    pub foley_axis: HashMap<CigGuid, Handle<FoleyAxis>>,
    #[serde(default)]
    pub foley_footstep_definition: HashMap<CigGuid, Handle<FoleyFootstepDefinition>>,
}

impl FoleyIndex {
    pub fn len(&self) -> usize {
        self.foley_definition.len()
            + self.foley_bone.len()
            + self.foley_axis.len()
            + self.foley_footstep_definition.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
