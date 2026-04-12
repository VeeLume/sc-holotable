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

/// Pool storage for the `aianimationdata` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AianimationdataPools {
    #[serde(default)]
    pub attack_category_params_base: Vec<Option<AttackCategoryParamsBase>>,
    #[serde(default)]
    pub melee_combo_chain_link: Vec<Option<MeleeComboChainLink>>,
    #[serde(default)]
    pub melee_attack_combo: Vec<Option<MeleeAttackCombo>>,
    #[serde(default)]
    pub aimelee_attack: Vec<Option<AIMeleeAttack>>,
    #[serde(default)]
    pub aimelee_combat_config: Vec<Option<AIMeleeCombatConfig>>,
}
