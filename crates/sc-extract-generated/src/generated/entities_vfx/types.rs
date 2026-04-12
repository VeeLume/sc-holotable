// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-vfx`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PlacedSurfaceEffects_Emitter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedSurfaceEffects_Emitter {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `emitterPosition` (Class)
    #[serde(default)]
    pub emitter_position: Option<Handle<Vec3>>,
    /// `linkedToSdf` (Boolean)
    #[serde(default)]
    pub linked_to_sdf: bool,
    /// `fadeOutDuration` (Single)
    #[serde(default)]
    pub fade_out_duration: f32,
}

impl Pooled for PlacedSurfaceEffects_Emitter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_vfx.placed_surface_effects_emitter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_vfx.placed_surface_effects_emitter }
}

impl<'a> Extract<'a> for PlacedSurfaceEffects_Emitter {
    const TYPE_NAME: &'static str = "PlacedSurfaceEffects_Emitter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emitter_position: match inst.get("emitterPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linked_to_sdf: inst.get_bool("linkedToSdf").unwrap_or_default(),
            fade_out_duration: inst.get_f32("fadeOutDuration").unwrap_or_default(),
        }
    }
}

