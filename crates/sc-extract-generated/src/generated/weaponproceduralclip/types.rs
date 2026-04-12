// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `weaponproceduralclip`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `WeaponProceduralClip`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralClip {
    /// `weaponProceduralClipBase` (StrongPointer)
    #[serde(default)]
    pub weapon_procedural_clip_base: Option<Handle<WeaponProceduralClipBase>>,
}

impl Pooled for WeaponProceduralClip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralclip.weapon_procedural_clip }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralclip.weapon_procedural_clip }
}

impl<'a> Extract<'a> for WeaponProceduralClip {
    const TYPE_NAME: &'static str = "WeaponProceduralClip";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_procedural_clip_base: match inst.get("weaponProceduralClipBase") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WeaponProceduralClipBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WeaponProceduralClipBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponProceduralClipBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralClipBase {
    /// `blendTime` (Single)
    #[serde(default)]
    pub blend_time: f32,
}

impl Pooled for WeaponProceduralClipBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralclip.weapon_procedural_clip_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralclip.weapon_procedural_clip_base }
}

impl<'a> Extract<'a> for WeaponProceduralClipBase {
    const TYPE_NAME: &'static str = "WeaponProceduralClipBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blend_time: inst.get_f32("blendTime").unwrap_or_default(),
        }
    }
}

