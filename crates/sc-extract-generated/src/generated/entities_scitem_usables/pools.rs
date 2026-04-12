// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-scitem-usables` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemUsablesPools {
    #[serde(default)]
    pub usable_archetype: Vec<Option<UsableArchetype>>,
    #[serde(default)]
    pub use_channel_archetype: Vec<Option<UseChannelArchetype>>,
    #[serde(default)]
    pub usable_archetypes: Vec<Option<UsableArchetypes>>,
}
