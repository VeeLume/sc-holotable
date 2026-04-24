// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `musiclogic`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `CinematicConfig`
pub struct CinematicConfig {
    /// `globalState` (String)
    pub global_state: String,
}

impl Pooled for CinematicConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.cinematic_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.cinematic_config }
}

impl<'a> Extract<'a> for CinematicConfig {
    const TYPE_NAME: &'static str = "CinematicConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            global_state: inst.get_str("globalState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AttackDetectionConfig`
pub struct AttackDetectionConfig {
    /// `numHitsToClassAsAnAttack` (Int32)
    pub num_hits_to_class_as_an_attack: i32,
    /// `attackDetectionTimeWindow` (Single)
    pub attack_detection_time_window: f32,
    /// `attackDetectionTimeout` (Single)
    pub attack_detection_timeout: f32,
}

impl Pooled for AttackDetectionConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.attack_detection_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.attack_detection_config }
}

impl<'a> Extract<'a> for AttackDetectionConfig {
    const TYPE_NAME: &'static str = "AttackDetectionConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            num_hits_to_class_as_an_attack: inst.get_i32("numHitsToClassAsAnAttack").unwrap_or_default(),
            attack_detection_time_window: inst.get_f32("attackDetectionTimeWindow").unwrap_or_default(),
            attack_detection_timeout: inst.get_f32("attackDetectionTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `EnemyAwarenessConfig`
pub struct EnemyAwarenessConfig {
    /// `numAwareEnemiesParameter` (Reference)
    pub num_aware_enemies_parameter: Option<CigGuid>,
    /// `numInCombatEnemiesParameter` (Reference)
    pub num_in_combat_enemies_parameter: Option<CigGuid>,
}

impl Pooled for EnemyAwarenessConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.enemy_awareness_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.enemy_awareness_config }
}

impl<'a> Extract<'a> for EnemyAwarenessConfig {
    const TYPE_NAME: &'static str = "EnemyAwarenessConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            num_aware_enemies_parameter: inst.get("numAwareEnemiesParameter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            num_in_combat_enemies_parameter: inst.get("numInCombatEnemiesParameter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PlaylistRNGConfig`
pub struct PlaylistRNGConfig {
    /// `parameters` (Reference (array))
    pub parameters: Vec<CigGuid>,
    /// `maxDeviation` (Single)
    pub max_deviation: f32,
}

impl Pooled for PlaylistRNGConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.playlist_rngconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.playlist_rngconfig }
}

impl<'a> Extract<'a> for PlaylistRNGConfig {
    const TYPE_NAME: &'static str = "PlaylistRNGConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            max_deviation: inst.get_f32("maxDeviation").unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationMusicConfig`
pub struct LocationMusicConfig {
    /// `wwiseEventPrefix` (String)
    pub wwise_event_prefix: String,
    /// `musicEventPrefix` (String)
    pub music_event_prefix: String,
    /// `wwiseEventPrefixStarSystem` (String)
    pub wwise_event_prefix_star_system: String,
    /// `musicEventPrefixStarSystem` (String)
    pub music_event_prefix_star_system: String,
}

impl Pooled for LocationMusicConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.location_music_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.location_music_config }
}

impl<'a> Extract<'a> for LocationMusicConfig {
    const TYPE_NAME: &'static str = "LocationMusicConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wwise_event_prefix: inst.get_str("wwiseEventPrefix").map(String::from).unwrap_or_default(),
            music_event_prefix: inst.get_str("musicEventPrefix").map(String::from).unwrap_or_default(),
            wwise_event_prefix_star_system: inst.get_str("wwiseEventPrefixStarSystem").map(String::from).unwrap_or_default(),
            music_event_prefix_star_system: inst.get_str("musicEventPrefixStarSystem").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MusicLogicConfig`
pub struct MusicLogicConfig {
    /// `cinematicConfig` (Class)
    pub cinematic_config: Option<Handle<CinematicConfig>>,
    /// `shipAttackDetectionConfig` (Class)
    pub ship_attack_detection_config: Option<Handle<AttackDetectionConfig>>,
    /// `fpsAttackDetectionConfig` (Class)
    pub fps_attack_detection_config: Option<Handle<AttackDetectionConfig>>,
    /// `playlistRNGConfig` (Class)
    pub playlist_rngconfig: Option<Handle<PlaylistRNGConfig>>,
    /// `locationMusicConfig` (Class)
    pub location_music_config: Option<Handle<LocationMusicConfig>>,
    /// `enemyAwarenessConfig` (Class)
    pub enemy_awareness_config: Option<Handle<EnemyAwarenessConfig>>,
    /// `triggerParentMusicAreaOnLeave` (Boolean)
    pub trigger_parent_music_area_on_leave: bool,
}

impl Pooled for MusicLogicConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.music_logic_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.music_logic_config }
}

impl<'a> Extract<'a> for MusicLogicConfig {
    const TYPE_NAME: &'static str = "MusicLogicConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cinematic_config: match inst.get("cinematicConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CinematicConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_attack_detection_config: match inst.get("shipAttackDetectionConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AttackDetectionConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fps_attack_detection_config: match inst.get("fpsAttackDetectionConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AttackDetectionConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            playlist_rngconfig: match inst.get("playlistRNGConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlaylistRNGConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            location_music_config: match inst.get("locationMusicConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LocationMusicConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            enemy_awareness_config: match inst.get("enemyAwarenessConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EnemyAwarenessConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            trigger_parent_music_area_on_leave: inst.get_bool("triggerParentMusicAreaOnLeave").unwrap_or_default(),
        }
    }
}

/// DCB type: `MusicLogicEventList`
pub struct MusicLogicEventList {
    /// `events` (Reference (array))
    pub events: Vec<CigGuid>,
}

impl Pooled for MusicLogicEventList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.music_logic_event_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.music_logic_event_list }
}

impl<'a> Extract<'a> for MusicLogicEventList {
    const TYPE_NAME: &'static str = "MusicLogicEventList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            events: inst.get_array("events")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MusicLogicSwitchValue`
pub struct MusicLogicSwitchValue {
    /// `switchValue` (String)
    pub switch_value: String,
}

impl Pooled for MusicLogicSwitchValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.music_logic_switch_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.music_logic_switch_value }
}

impl<'a> Extract<'a> for MusicLogicSwitchValue {
    const TYPE_NAME: &'static str = "MusicLogicSwitchValue";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            switch_value: inst.get_str("switchValue").map(String::from).unwrap_or_default(),
        }
    }
}

