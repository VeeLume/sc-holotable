// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-elementsounds_deprecated`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `UIElementSoundsRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElementSoundsRecord {
    /// `SoundDBs` (Class (array))
    #[serde(default)]
    pub sound_dbs: Vec<Handle<UIElementSoundEntry>>,
}

impl Pooled for UIElementSoundsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_elementsounds_deprecated.uielement_sounds_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_elementsounds_deprecated.uielement_sounds_record }
}

impl<'a> Extract<'a> for UIElementSoundsRecord {
    const TYPE_NAME: &'static str = "UIElementSoundsRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sound_dbs: inst.get_array("SoundDBs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIElementSoundEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIElementSoundEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UIElementSoundEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElementSoundEntry {
    /// `SoundID` (String)
    #[serde(default)]
    pub sound_id: String,
    /// `SoundPath` (String)
    #[serde(default)]
    pub sound_path: String,
}

impl Pooled for UIElementSoundEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_elementsounds_deprecated.uielement_sound_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_elementsounds_deprecated.uielement_sound_entry }
}

impl<'a> Extract<'a> for UIElementSoundEntry {
    const TYPE_NAME: &'static str = "UIElementSoundEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sound_id: inst.get_str("SoundID").map(String::from).unwrap_or_default(),
            sound_path: inst.get_str("SoundPath").map(String::from).unwrap_or_default(),
        }
    }
}

