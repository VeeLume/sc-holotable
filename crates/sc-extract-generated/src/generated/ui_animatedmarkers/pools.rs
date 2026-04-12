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

/// Pool storage for the `ui-animatedmarkers` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiAnimatedmarkersPools {
    #[serde(default)]
    pub animated_marker_marker: Vec<Option<AnimatedMarker_Marker>>,
    #[serde(default)]
    pub animated_marker: Vec<Option<AnimatedMarker>>,
    #[serde(default)]
    pub combat_marker: Vec<Option<CombatMarker>>,
    #[serde(default)]
    pub animation_graph_key_frame: Vec<Option<AnimationGraph_KeyFrame>>,
    #[serde(default)]
    pub animation_graph_track: Vec<Option<AnimationGraph_Track>>,
    #[serde(default)]
    pub animation_graph_timer: Vec<Option<AnimationGraph_Timer>>,
    #[serde(default)]
    pub animation_graph_timeline: Vec<Option<AnimationGraph_Timeline>>,
}
