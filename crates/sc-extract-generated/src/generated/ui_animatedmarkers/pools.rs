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

/// Pool storage for the `ui-animatedmarkers` feature.
#[derive(Default)]
pub struct UiAnimatedmarkersPools {
    pub animated_marker_marker: Vec<Option<AnimatedMarker_Marker>>,
    pub animated_marker: Vec<Option<AnimatedMarker>>,
    pub combat_marker: Vec<Option<CombatMarker>>,
    pub animation_graph_key_frame: Vec<Option<AnimationGraph_KeyFrame>>,
    pub animation_graph_track: Vec<Option<AnimationGraph_Track>>,
    pub animation_graph_timer: Vec<Option<AnimationGraph_Timer>>,
    pub animation_graph_timeline: Vec<Option<AnimationGraph_Timeline>>,
}
