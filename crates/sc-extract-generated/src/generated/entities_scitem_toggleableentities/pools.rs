// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-scitem-toggleableentities` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemToggleableentitiesPools {
    #[serde(default)]
    pub set_lightning_enabled_state_gameplay_trigger: Vec<Option<SetLightningEnabledStateGameplayTrigger>>,
    #[serde(default)]
    pub sset_particle_enabled_state_gameplay_trigger: Vec<Option<SSetParticleEnabledStateGameplayTrigger>>,
}
