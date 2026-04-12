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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CinematicConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicConfig {
    /// `globalState` (String)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackDetectionConfig {
    /// `numHitsToClassAsAnAttack` (Int32)
    #[serde(default)]
    pub num_hits_to_class_as_an_attack: i32,
    /// `attackDetectionTimeWindow` (Single)
    #[serde(default)]
    pub attack_detection_time_window: f32,
    /// `attackDetectionTimeout` (Single)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyAwarenessConfig {
    /// `numAwareEnemiesParameter` (Reference)
    #[serde(default)]
    pub num_aware_enemies_parameter: Option<CigGuid>,
    /// `numInCombatEnemiesParameter` (Reference)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistRNGConfig {
    /// `parameters` (Reference (array))
    #[serde(default)]
    pub parameters: Vec<CigGuid>,
    /// `maxDeviation` (Single)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMusicConfig {
    /// `wwiseEventPrefix` (String)
    #[serde(default)]
    pub wwise_event_prefix: String,
    /// `musicEventPrefix` (String)
    #[serde(default)]
    pub music_event_prefix: String,
    /// `wwiseEventPrefixStarSystem` (String)
    #[serde(default)]
    pub wwise_event_prefix_star_system: String,
    /// `musicEventPrefixStarSystem` (String)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicConfig {
    /// `cinematicConfig` (Class)
    #[serde(default)]
    pub cinematic_config: Option<Handle<CinematicConfig>>,
    /// `shipAttackDetectionConfig` (Class)
    #[serde(default)]
    pub ship_attack_detection_config: Option<Handle<AttackDetectionConfig>>,
    /// `fpsAttackDetectionConfig` (Class)
    #[serde(default)]
    pub fps_attack_detection_config: Option<Handle<AttackDetectionConfig>>,
    /// `playlistRNGConfig` (Class)
    #[serde(default)]
    pub playlist_rngconfig: Option<Handle<PlaylistRNGConfig>>,
    /// `locationMusicConfig` (Class)
    #[serde(default)]
    pub location_music_config: Option<Handle<LocationMusicConfig>>,
    /// `enemyAwarenessConfig` (Class)
    #[serde(default)]
    pub enemy_awareness_config: Option<Handle<EnemyAwarenessConfig>>,
    /// `triggerParentMusicAreaOnLeave` (Boolean)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CinematicConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_attack_detection_config: match inst.get("shipAttackDetectionConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AttackDetectionConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AttackDetectionConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fps_attack_detection_config: match inst.get("fpsAttackDetectionConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AttackDetectionConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AttackDetectionConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            playlist_rngconfig: match inst.get("playlistRNGConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlaylistRNGConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlaylistRNGConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_music_config: match inst.get("locationMusicConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LocationMusicConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LocationMusicConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enemy_awareness_config: match inst.get("enemyAwarenessConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EnemyAwarenessConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EnemyAwarenessConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trigger_parent_music_area_on_leave: inst.get_bool("triggerParentMusicAreaOnLeave").unwrap_or_default(),
        }
    }
}

/// DCB type: `MusicLogicParameter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicParameter {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// `max` (Single)
    #[serde(default)]
    pub max: f32,
    /// `defaultValue` (Single)
    #[serde(default)]
    pub default_value: f32,
    /// `decayRate` (Single)
    #[serde(default)]
    pub decay_rate: f32,
    /// `decayIsPercentage` (Boolean)
    #[serde(default)]
    pub decay_is_percentage: bool,
    /// `scaleModifier` (Single)
    #[serde(default)]
    pub scale_modifier: f32,
    /// `shiftModifier` (Single)
    #[serde(default)]
    pub shift_modifier: f32,
    /// `inverted` (Boolean)
    #[serde(default)]
    pub inverted: bool,
    /// `rtpc` (String)
    #[serde(default)]
    pub rtpc: String,
    /// `rtpcIsGlobal` (Boolean)
    #[serde(default)]
    pub rtpc_is_global: bool,
    /// `contributors` (Reference (array))
    #[serde(default)]
    pub contributors: Vec<CigGuid>,
    /// `debugColour` (Class)
    #[serde(default)]
    pub debug_colour: Option<Handle<RGB>>,
}

impl Pooled for MusicLogicParameter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.music_logic_parameter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.music_logic_parameter }
}

impl<'a> Extract<'a> for MusicLogicParameter {
    const TYPE_NAME: &'static str = "MusicLogicParameter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
            default_value: inst.get_f32("defaultValue").unwrap_or_default(),
            decay_rate: inst.get_f32("decayRate").unwrap_or_default(),
            decay_is_percentage: inst.get_bool("decayIsPercentage").unwrap_or_default(),
            scale_modifier: inst.get_f32("scaleModifier").unwrap_or_default(),
            shift_modifier: inst.get_f32("shiftModifier").unwrap_or_default(),
            inverted: inst.get_bool("inverted").unwrap_or_default(),
            rtpc: inst.get_str("rtpc").map(String::from).unwrap_or_default(),
            rtpc_is_global: inst.get_bool("rtpcIsGlobal").unwrap_or_default(),
            contributors: inst.get_array("contributors")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            debug_colour: match inst.get("debugColour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MusicLogicEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicEvent {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `retriggerDelay` (Single)
    #[serde(default)]
    pub retrigger_delay: f32,
}

impl Pooled for MusicLogicEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.music_logic_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.music_logic_event }
}

impl<'a> Extract<'a> for MusicLogicEvent {
    const TYPE_NAME: &'static str = "MusicLogicEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            retrigger_delay: inst.get_f32("retriggerDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `MusicLogicEventList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicEventList {
    /// `events` (Reference (array))
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicSwitchValue {
    /// `switchValue` (String)
    #[serde(default)]
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

/// DCB type: `MusicLogicNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicNode {
}

impl Pooled for MusicLogicNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.music_logic_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.music_logic_node }
}

impl<'a> Extract<'a> for MusicLogicNode {
    const TYPE_NAME: &'static str = "MusicLogicNode";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MusicLogicSuite`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicSuite {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `musicLogicConfig` (Reference)
    #[serde(default)]
    pub music_logic_config: Option<CigGuid>,
    /// `nodes` (StrongPointer (array))
    #[serde(default)]
    pub nodes: Vec<Handle<MusicLogicNode>>,
}

impl Pooled for MusicLogicSuite {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.musiclogic.music_logic_suite }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.musiclogic.music_logic_suite }
}

impl<'a> Extract<'a> for MusicLogicSuite {
    const TYPE_NAME: &'static str = "MusicLogicSuite";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            music_logic_config: inst.get("musicLogicConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            nodes: inst.get_array("nodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MusicLogicNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MusicLogicNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

