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

/// Pool storage for the `actor-playerdefaultactionsconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorPlayerdefaultactionsconfigPools {
    #[serde(default)]
    pub default_action_def: Vec<Option<DefaultActionDef>>,
    #[serde(default)]
    pub default_actions_params: Vec<Option<DefaultActionsParams>>,
    #[serde(default)]
    pub default_actions: Vec<Option<DefaultActions>>,
    #[serde(default)]
    pub default_action_description_override: Vec<Option<DefaultActionDescriptionOverride>>,
    #[serde(default)]
    pub default_actions_entry: Vec<Option<DefaultActionsEntry>>,
    #[serde(default)]
    pub actor_default_actions_config: Vec<Option<ActorDefaultActionsConfig>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition: Vec<Option<DefaultActionsEntityEntryCondition>>,
    #[serde(default)]
    pub default_actions_entity_state: Vec<Option<DefaultActionsEntityState>>,
}
