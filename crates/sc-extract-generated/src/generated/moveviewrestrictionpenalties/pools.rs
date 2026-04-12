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

/// Pool storage for the `moveviewrestrictionpenalties` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MoveviewrestrictionpenaltiesPools {
    #[serde(default)]
    pub move_view_restriction_penalty: Vec<Option<MoveViewRestrictionPenalty>>,
    #[serde(default)]
    pub move_view_restriction_weighting: Vec<Option<MoveViewRestrictionWeighting>>,
    #[serde(default)]
    pub armor_move_view_restrictions: Vec<Option<ArmorMoveViewRestrictions>>,
}
