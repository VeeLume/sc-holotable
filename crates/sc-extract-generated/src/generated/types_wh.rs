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

/// DCB type: `WheelAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelAudioParams {
    /// DCB field: `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for WheelAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_wh.wheel_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_wh.wheel_audio_params }
}

impl<'a> Extract<'a> for WheelAudioParams {
    const TYPE_NAME: &'static str = "WheelAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loop_start: match inst.get("loopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loop_stop: match inst.get("loopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WheelAudioSurfaceMapping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelAudioSurfaceMapping {
    /// DCB field: `surfaceName` (String)
    #[serde(default)]
    pub surface_name: String,
    /// DCB field: `audio` (Class)
    #[serde(default)]
    pub audio: Option<Handle<WheelAudioParams>>,
}

impl Pooled for WheelAudioSurfaceMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_wh.wheel_audio_surface_mapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_wh.wheel_audio_surface_mapping }
}

impl<'a> Extract<'a> for WheelAudioSurfaceMapping {
    const TYPE_NAME: &'static str = "WheelAudioSurfaceMapping";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_name: inst.get_str("surfaceName").map(String::from).unwrap_or_default(),
            audio: match inst.get("audio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WheelAudioSurfaceMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelAudioSurfaceMap {
    /// DCB field: `generic` (Class)
    #[serde(default)]
    pub generic: Option<Handle<WheelAudioParams>>,
    /// DCB field: `default` (Class)
    #[serde(default)]
    pub default: Option<Handle<WheelAudioParams>>,
    /// DCB field: `invalidSurface` (Class)
    #[serde(default)]
    pub invalid_surface: Option<Handle<WheelAudioParams>>,
    /// DCB field: `surfaceMappings` (Class (array))
    #[serde(default)]
    pub surface_mappings: Vec<Handle<WheelAudioSurfaceMapping>>,
}

impl Pooled for WheelAudioSurfaceMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_wh.wheel_audio_surface_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_wh.wheel_audio_surface_map }
}

impl<'a> Extract<'a> for WheelAudioSurfaceMap {
    const TYPE_NAME: &'static str = "WheelAudioSurfaceMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            generic: match inst.get("generic") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default: match inst.get("default") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invalid_surface: match inst.get("invalidSurface") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            surface_mappings: inst.get_array("surfaceMappings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WheelAudioSurfaceMapping>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WheelAudioSurfaceMapping>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

