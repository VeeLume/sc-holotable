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

/// Pool storage for the `musiclogic` feature.
#[derive(Default)]
pub struct MusiclogicPools {
    pub cinematic_config: Vec<Option<CinematicConfig>>,
    pub attack_detection_config: Vec<Option<AttackDetectionConfig>>,
    pub enemy_awareness_config: Vec<Option<EnemyAwarenessConfig>>,
    pub playlist_rngconfig: Vec<Option<PlaylistRNGConfig>>,
    pub location_music_config: Vec<Option<LocationMusicConfig>>,
    pub music_logic_config: Vec<Option<MusicLogicConfig>>,
    pub music_logic_event_list: Vec<Option<MusicLogicEventList>>,
    pub music_logic_switch_value: Vec<Option<MusicLogicSwitchValue>>,
}
