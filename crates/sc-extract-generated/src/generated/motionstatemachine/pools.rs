// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `motionstatemachine` feature.
#[derive(Default)]
pub struct MotionstatemachinePools {
    pub motion_connection: Vec<Option<MotionConnection>>,
    pub motion_state: Vec<Option<MotionState>>,
    pub procedural_idle_to_move_params: Vec<Option<ProceduralIdleToMoveParams>>,
    pub motion_smoothing_params: Vec<Option<MotionSmoothingParams>>,
    pub motion_juke_params: Vec<Option<MotionJukeParams>>,
    pub motion_turn_params: Vec<Option<MotionTurnParams>>,
    pub motion_turn_setup_filtered: Vec<Option<MotionTurnSetupFiltered>>,
    pub motion_turn_setup_list: Vec<Option<MotionTurnSetupList>>,
    pub motion_foot_pinning_params: Vec<Option<MotionFootPinningParams>>,
    pub motion_graph: Vec<Option<MotionGraph>>,
    pub scprone_motion_graph_def: Vec<Option<SCProneMotionGraphDef>>,
}
