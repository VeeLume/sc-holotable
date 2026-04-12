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

/// DCB type: `AnimatedMarker_Marker`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedMarker_Marker {
    /// DCB field: `matrixStackID` (String (array))
    #[serde(default)]
    pub matrix_stack_id: Vec<String>,
    /// DCB field: `timelines` (Class (array))
    #[serde(default)]
    pub timelines: Vec<Handle<AnimationGraph_Timeline>>,
    /// DCB field: `timers` (Class (array))
    #[serde(default)]
    pub timers: Vec<Handle<AnimationGraph_Timer>>,
}

impl Pooled for AnimatedMarker_Marker {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animated_marker_marker }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animated_marker_marker }
}

impl<'a> Extract<'a> for AnimatedMarker_Marker {
    const TYPE_NAME: &'static str = "AnimatedMarker_Marker";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            matrix_stack_id: inst.get_array("matrixStackID")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            timelines: inst.get_array("timelines")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnimationGraph_Timeline>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnimationGraph_Timeline>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            timers: inst.get_array("timers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnimationGraph_Timer>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnimationGraph_Timer>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimatedMarker`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedMarker {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `objectName` (String)
    #[serde(default)]
    pub object_name: String,
    /// DCB field: `doOriginOffsetScale` (Boolean)
    #[serde(default)]
    pub do_origin_offset_scale: bool,
    /// DCB field: `originOffsetScaleMin` (Single)
    #[serde(default)]
    pub origin_offset_scale_min: f32,
    /// DCB field: `originOffsetTargetBoundInc` (Single)
    #[serde(default)]
    pub origin_offset_target_bound_inc: f32,
    /// DCB field: `matrixBlendRate` (Single)
    #[serde(default)]
    pub matrix_blend_rate: f32,
    /// DCB field: `lockLostLength` (Single)
    #[serde(default)]
    pub lock_lost_length: f32,
    /// DCB field: `additionAttachments` (UInt32)
    #[serde(default)]
    pub addition_attachments: u32,
    /// DCB field: `markers` (Class)
    #[serde(default)]
    pub markers: Option<Handle<AnimatedMarker_Marker>>,
}

impl Pooled for AnimatedMarker {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animated_marker }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animated_marker }
}

impl<'a> Extract<'a> for AnimatedMarker {
    const TYPE_NAME: &'static str = "AnimatedMarker";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            object_name: inst.get_str("objectName").map(String::from).unwrap_or_default(),
            do_origin_offset_scale: inst.get_bool("doOriginOffsetScale").unwrap_or_default(),
            origin_offset_scale_min: inst.get_f32("originOffsetScaleMin").unwrap_or_default(),
            origin_offset_target_bound_inc: inst.get_f32("originOffsetTargetBoundInc").unwrap_or_default(),
            matrix_blend_rate: inst.get_f32("matrixBlendRate").unwrap_or_default(),
            lock_lost_length: inst.get_f32("lockLostLength").unwrap_or_default(),
            addition_attachments: inst.get_u32("additionAttachments").unwrap_or_default(),
            markers: match inst.get("markers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AnimatedMarker_Marker>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AnimatedMarker_Marker>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AnimationGraph_KeyFrame`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationGraph_KeyFrame {
    /// DCB field: `frame` (UInt32)
    #[serde(default)]
    pub frame: u32,
    /// DCB field: `value` (Single)
    #[serde(default)]
    pub value: f32,
    /// DCB field: `timeModifier` (EnumChoice)
    #[serde(default)]
    pub time_modifier: String,
    /// DCB field: `easeType` (EnumChoice)
    #[serde(default)]
    pub ease_type: String,
}

impl Pooled for AnimationGraph_KeyFrame {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animation_graph_key_frame }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animation_graph_key_frame }
}

impl<'a> Extract<'a> for AnimationGraph_KeyFrame {
    const TYPE_NAME: &'static str = "AnimationGraph_KeyFrame";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frame: inst.get_u32("frame").unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
            time_modifier: inst.get_str("timeModifier").map(String::from).unwrap_or_default(),
            ease_type: inst.get_str("easeType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimationGraph_Track`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationGraph_Track {
    /// DCB field: `trackName` (String)
    #[serde(default)]
    pub track_name: String,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `keyFrames` (Class (array))
    #[serde(default)]
    pub key_frames: Vec<Handle<AnimationGraph_KeyFrame>>,
}

impl Pooled for AnimationGraph_Track {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animation_graph_track }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animation_graph_track }
}

impl<'a> Extract<'a> for AnimationGraph_Track {
    const TYPE_NAME: &'static str = "AnimationGraph_Track";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            track_name: inst.get_str("trackName").map(String::from).unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            key_frames: inst.get_array("keyFrames")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnimationGraph_KeyFrame>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnimationGraph_KeyFrame>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimationGraph_Timer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationGraph_Timer {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `timeStart` (Single)
    #[serde(default)]
    pub time_start: f32,
    /// DCB field: `timeEnd` (Single)
    #[serde(default)]
    pub time_end: f32,
    /// DCB field: `count` (UInt32)
    #[serde(default)]
    pub count: u32,
}

impl Pooled for AnimationGraph_Timer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animation_graph_timer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animation_graph_timer }
}

impl<'a> Extract<'a> for AnimationGraph_Timer {
    const TYPE_NAME: &'static str = "AnimationGraph_Timer";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            time_start: inst.get_f32("timeStart").unwrap_or_default(),
            time_end: inst.get_f32("timeEnd").unwrap_or_default(),
            count: inst.get_u32("count").unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimationGraph_Timeline`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationGraph_Timeline {
    /// DCB field: `timerID` (String)
    #[serde(default)]
    pub timer_id: String,
    /// DCB field: `frameRate` (UInt32)
    #[serde(default)]
    pub frame_rate: u32,
    /// DCB field: `tracks` (Class (array))
    #[serde(default)]
    pub tracks: Vec<Handle<AnimationGraph_Track>>,
}

impl Pooled for AnimationGraph_Timeline {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animation_graph_timeline }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animation_graph_timeline }
}

impl<'a> Extract<'a> for AnimationGraph_Timeline {
    const TYPE_NAME: &'static str = "AnimationGraph_Timeline";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            timer_id: inst.get_str("timerID").map(String::from).unwrap_or_default(),
            frame_rate: inst.get_u32("frameRate").unwrap_or_default(),
            tracks: inst.get_array("tracks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnimationGraph_Track>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnimationGraph_Track>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AnnouncementGameToken`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementGameToken {
    /// DCB field: `gameToken` (String)
    #[serde(default)]
    pub game_token: String,
    /// DCB field: `gameTokenType` (EnumChoice)
    #[serde(default)]
    pub game_token_type: String,
}

impl Pooled for AnnouncementGameToken {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.announcement_game_token }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.announcement_game_token }
}

impl<'a> Extract<'a> for AnnouncementGameToken {
    const TYPE_NAME: &'static str = "AnnouncementGameToken";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            game_token: inst.get_str("gameToken").map(String::from).unwrap_or_default(),
            game_token_type: inst.get_str("gameTokenType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `Announcement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `conversation` (Reference)
    #[serde(default)]
    pub conversation: Option<CigGuid>,
    /// DCB field: `gameToken` (String)
    #[serde(default)]
    pub game_token: String,
    /// DCB field: `gameTokenType` (EnumChoice)
    #[serde(default)]
    pub game_token_type: String,
    /// DCB field: `gameTokens` (Class (array))
    #[serde(default)]
    pub game_tokens: Vec<Handle<AnnouncementGameToken>>,
    /// DCB field: `priority` (EnumChoice)
    #[serde(default)]
    pub priority: String,
    /// DCB field: `retriggerDelay` (Single)
    #[serde(default)]
    pub retrigger_delay: f32,
    /// DCB field: `playProbability` (Single)
    #[serde(default)]
    pub play_probability: f32,
    /// DCB field: `playWhenDead` (Boolean)
    #[serde(default)]
    pub play_when_dead: bool,
    /// DCB field: `playWhenSpectating` (Boolean)
    #[serde(default)]
    pub play_when_spectating: bool,
}

impl Pooled for Announcement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.announcement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.announcement }
}

impl<'a> Extract<'a> for Announcement {
    const TYPE_NAME: &'static str = "Announcement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            conversation: inst.get("conversation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            game_token: inst.get_str("gameToken").map(String::from).unwrap_or_default(),
            game_token_type: inst.get_str("gameTokenType").map(String::from).unwrap_or_default(),
            game_tokens: inst.get_array("gameTokens")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnnouncementGameToken>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnnouncementGameToken>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            priority: inst.get_str("priority").map(String::from).unwrap_or_default(),
            retrigger_delay: inst.get_f32("retriggerDelay").unwrap_or_default(),
            play_probability: inst.get_f32("playProbability").unwrap_or_default(),
            play_when_dead: inst.get_bool("playWhenDead").unwrap_or_default(),
            play_when_spectating: inst.get_bool("playWhenSpectating").unwrap_or_default(),
        }
    }
}

/// DCB type: `Announcer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcer {
    /// DCB field: `base` (Reference)
    #[serde(default)]
    pub base: Option<CigGuid>,
    /// DCB field: `announcements` (Class (array))
    #[serde(default)]
    pub announcements: Vec<Handle<Announcement>>,
}

impl Pooled for Announcer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.announcer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.announcer }
}

impl<'a> Extract<'a> for Announcer {
    const TYPE_NAME: &'static str = "Announcer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base: inst.get("base").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            announcements: inst.get_array("announcements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Announcement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Announcement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Ang3`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ang3 {
    /// DCB field: `x` (Single)
    #[serde(default)]
    pub x: f32,
    /// DCB field: `y` (Single)
    #[serde(default)]
    pub y: f32,
    /// DCB field: `z` (Single)
    #[serde(default)]
    pub z: f32,
}

impl Pooled for Ang3 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.ang3 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.ang3 }
}

impl<'a> Extract<'a> for Ang3 {
    const TYPE_NAME: &'static str = "Ang3";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x: inst.get_f32("x").unwrap_or_default(),
            y: inst.get_f32("y").unwrap_or_default(),
            z: inst.get_f32("z").unwrap_or_default(),
        }
    }
}

/// DCB type: `AngYPR`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AngYPR {
    /// DCB field: `yaw` (Single)
    #[serde(default)]
    pub yaw: f32,
    /// DCB field: `pitch` (Single)
    #[serde(default)]
    pub pitch: f32,
    /// DCB field: `roll` (Single)
    #[serde(default)]
    pub roll: f32,
}

impl Pooled for AngYPR {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.ang_ypr }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.ang_ypr }
}

impl<'a> Extract<'a> for AngYPR {
    const TYPE_NAME: &'static str = "AngYPR";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            yaw: inst.get_f32("yaw").unwrap_or_default(),
            pitch: inst.get_f32("pitch").unwrap_or_default(),
            roll: inst.get_f32("roll").unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimatedAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedAction {
    /// DCB field: `name` (Class)
    #[serde(default)]
    pub name: Option<Handle<InputAction>>,
    /// DCB field: `playerActionAnimType` (EnumChoice)
    #[serde(default)]
    pub player_action_anim_type: String,
}

impl Pooled for AnimatedAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animated_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animated_action }
}

impl<'a> Extract<'a> for AnimatedAction {
    const TYPE_NAME: &'static str = "AnimatedAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: match inst.get("name") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_action_anim_type: inst.get_str("playerActionAnimType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimatedHelmetParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedHelmetParams {
    /// DCB field: `stateMachines` (Class (array))
    #[serde(default)]
    pub state_machines: Vec<Handle<SHelmetStateMachineParams>>,
}

impl Pooled for AnimatedHelmetParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_an.animated_helmet_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_an.animated_helmet_params }
}

impl<'a> Extract<'a> for AnimatedHelmetParams {
    const TYPE_NAME: &'static str = "AnimatedHelmetParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_machines: inst.get_array("stateMachines")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHelmetStateMachineParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHelmetStateMachineParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

