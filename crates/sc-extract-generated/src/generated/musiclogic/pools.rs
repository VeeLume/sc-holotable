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

/// Pool storage for the `musiclogic` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MusiclogicPools {
    #[serde(default)]
    pub cinematic_config: Vec<Option<CinematicConfig>>,
    #[serde(default)]
    pub attack_detection_config: Vec<Option<AttackDetectionConfig>>,
    #[serde(default)]
    pub enemy_awareness_config: Vec<Option<EnemyAwarenessConfig>>,
    #[serde(default)]
    pub playlist_rngconfig: Vec<Option<PlaylistRNGConfig>>,
    #[serde(default)]
    pub location_music_config: Vec<Option<LocationMusicConfig>>,
    #[serde(default)]
    pub music_logic_config: Vec<Option<MusicLogicConfig>>,
    #[serde(default)]
    pub music_logic_parameter: Vec<Option<MusicLogicParameter>>,
    #[serde(default)]
    pub music_logic_event: Vec<Option<MusicLogicEvent>>,
    #[serde(default)]
    pub music_logic_event_list: Vec<Option<MusicLogicEventList>>,
    #[serde(default)]
    pub music_logic_switch_value: Vec<Option<MusicLogicSwitchValue>>,
    #[serde(default)]
    pub music_logic_node: Vec<Option<MusicLogicNode>>,
    #[serde(default)]
    pub music_logic_suite: Vec<Option<MusicLogicSuite>>,
}
