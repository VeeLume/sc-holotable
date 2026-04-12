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

/// Record index for the `entities-scitem-usables` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemUsablesIndex {
    #[serde(default)]
    pub usable_archetype: HashMap<CigGuid, Handle<UsableArchetype>>,
    #[serde(default)]
    pub use_channel_archetype: HashMap<CigGuid, Handle<UseChannelArchetype>>,
    #[serde(default)]
    pub usable_archetypes: HashMap<CigGuid, Handle<UsableArchetypes>>,
}

impl EntitiesScitemUsablesIndex {
    pub fn len(&self) -> usize {
        self.usable_archetype.len()
            + self.use_channel_archetype.len()
            + self.usable_archetypes.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
