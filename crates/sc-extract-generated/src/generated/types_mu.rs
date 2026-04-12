// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `MusicLogicConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicConfig {
    /// DCB field: `cinematicConfig` (Class)
    #[serde(default)]
    pub cinematic_config: Option<Handle<CinematicConfig>>,
    /// DCB field: `shipAttackDetectionConfig` (Class)
    #[serde(default)]
    pub ship_attack_detection_config: Option<Handle<AttackDetectionConfig>>,
    /// DCB field: `fpsAttackDetectionConfig` (Class)
    #[serde(default)]
    pub fps_attack_detection_config: Option<Handle<AttackDetectionConfig>>,
    /// DCB field: `playlistRNGConfig` (Class)
    #[serde(default)]
    pub playlist_rngconfig: Option<Handle<PlaylistRNGConfig>>,
    /// DCB field: `locationMusicConfig` (Class)
    #[serde(default)]
    pub location_music_config: Option<Handle<LocationMusicConfig>>,
    /// DCB field: `enemyAwarenessConfig` (Class)
    #[serde(default)]
    pub enemy_awareness_config: Option<Handle<EnemyAwarenessConfig>>,
    /// DCB field: `triggerParentMusicAreaOnLeave` (Boolean)
    #[serde(default)]
    pub trigger_parent_music_area_on_leave: bool,
}

impl Pooled for MusicLogicConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mu.music_logic_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mu.music_logic_config }
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
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// DCB field: `max` (Single)
    #[serde(default)]
    pub max: f32,
    /// DCB field: `defaultValue` (Single)
    #[serde(default)]
    pub default_value: f32,
    /// DCB field: `decayRate` (Single)
    #[serde(default)]
    pub decay_rate: f32,
    /// DCB field: `decayIsPercentage` (Boolean)
    #[serde(default)]
    pub decay_is_percentage: bool,
    /// DCB field: `scaleModifier` (Single)
    #[serde(default)]
    pub scale_modifier: f32,
    /// DCB field: `shiftModifier` (Single)
    #[serde(default)]
    pub shift_modifier: f32,
    /// DCB field: `inverted` (Boolean)
    #[serde(default)]
    pub inverted: bool,
    /// DCB field: `rtpc` (String)
    #[serde(default)]
    pub rtpc: String,
    /// DCB field: `rtpcIsGlobal` (Boolean)
    #[serde(default)]
    pub rtpc_is_global: bool,
    /// DCB field: `contributors` (Reference (array))
    #[serde(default)]
    pub contributors: Vec<CigGuid>,
    /// DCB field: `debugColour` (Class)
    #[serde(default)]
    pub debug_colour: Option<Handle<RGB>>,
}

impl Pooled for MusicLogicParameter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mu.music_logic_parameter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mu.music_logic_parameter }
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
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `retriggerDelay` (Single)
    #[serde(default)]
    pub retrigger_delay: f32,
}

impl Pooled for MusicLogicEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mu.music_logic_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mu.music_logic_event }
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
    /// DCB field: `events` (Reference (array))
    #[serde(default)]
    pub events: Vec<CigGuid>,
}

impl Pooled for MusicLogicEventList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mu.music_logic_event_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mu.music_logic_event_list }
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
    /// DCB field: `switchValue` (String)
    #[serde(default)]
    pub switch_value: String,
}

impl Pooled for MusicLogicSwitchValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mu.music_logic_switch_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mu.music_logic_switch_value }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mu.music_logic_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mu.music_logic_node }
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
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `musicLogicConfig` (Reference)
    #[serde(default)]
    pub music_logic_config: Option<CigGuid>,
    /// DCB field: `nodes` (StrongPointer (array))
    #[serde(default)]
    pub nodes: Vec<Handle<MusicLogicNode>>,
}

impl Pooled for MusicLogicSuite {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mu.music_logic_suite }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mu.music_logic_suite }
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

