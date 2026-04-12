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

/// Pool storage for the `motionstatemachine` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MotionstatemachinePools {
    #[serde(default)]
    pub motion_connection: Vec<Option<MotionConnection>>,
    #[serde(default)]
    pub motion_state: Vec<Option<MotionState>>,
    #[serde(default)]
    pub procedural_idle_to_move_params: Vec<Option<ProceduralIdleToMoveParams>>,
    #[serde(default)]
    pub motion_smoothing_params: Vec<Option<MotionSmoothingParams>>,
    #[serde(default)]
    pub motion_juke_params: Vec<Option<MotionJukeParams>>,
    #[serde(default)]
    pub motion_turn_params: Vec<Option<MotionTurnParams>>,
    #[serde(default)]
    pub motion_turn_setup_filtered: Vec<Option<MotionTurnSetupFiltered>>,
    #[serde(default)]
    pub motion_turn_setup_list: Vec<Option<MotionTurnSetupList>>,
    #[serde(default)]
    pub motion_foot_pinning_params: Vec<Option<MotionFootPinningParams>>,
    #[serde(default)]
    pub motion_graph: Vec<Option<MotionGraph>>,
    #[serde(default)]
    pub scprone_motion_graph_def: Vec<Option<SCProneMotionGraphDef>>,
    #[serde(default)]
    pub imannequin_action_def: Vec<Option<IMannequinActionDef>>,
    #[serde(default)]
    pub smannequin_action_def_record: Vec<Option<SMannequinActionDefRecord>>,
}
