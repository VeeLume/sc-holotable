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

/// DCB type: `NpcBreathingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcBreathingParams {
    /// DCB field: `breathVolumeParams` (Class)
    #[serde(default)]
    pub breath_volume_params: Option<Handle<BreathVolumeParams>>,
    /// DCB field: `minStaminaVolumeRatio` (Single)
    #[serde(default)]
    pub min_stamina_volume_ratio: f32,
    /// DCB field: `maxStaminaVolumeRatio` (Single)
    #[serde(default)]
    pub max_stamina_volume_ratio: f32,
    /// DCB field: `minStaminaBreathDuration` (Single)
    #[serde(default)]
    pub min_stamina_breath_duration: f32,
    /// DCB field: `maxStaminaBreathDuration` (Single)
    #[serde(default)]
    pub max_stamina_breath_duration: f32,
}

impl Pooled for NpcBreathingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_np.npc_breathing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_np.npc_breathing_params }
}

impl<'a> Extract<'a> for NpcBreathingParams {
    const TYPE_NAME: &'static str = "NpcBreathingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            breath_volume_params: match inst.get("breathVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BreathVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BreathVolumeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_stamina_volume_ratio: inst.get_f32("minStaminaVolumeRatio").unwrap_or_default(),
            max_stamina_volume_ratio: inst.get_f32("maxStaminaVolumeRatio").unwrap_or_default(),
            min_stamina_breath_duration: inst.get_f32("minStaminaBreathDuration").unwrap_or_default(),
            max_stamina_breath_duration: inst.get_f32("maxStaminaBreathDuration").unwrap_or_default(),
        }
    }
}

