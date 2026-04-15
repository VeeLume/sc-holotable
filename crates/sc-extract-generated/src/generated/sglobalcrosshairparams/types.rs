// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sglobalcrosshairparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SGlobalCrosshairParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalCrosshairParams {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `lerpSpeed` (Single)
    #[serde(default)]
    pub lerp_speed: f32,
    /// `positionSmoothFactor` (Single)
    #[serde(default)]
    pub position_smooth_factor: f32,
    /// `distanceSmoothFactor` (Single)
    #[serde(default)]
    pub distance_smooth_factor: f32,
    /// `range` (Single)
    #[serde(default)]
    pub range: f32,
    /// `hitmarkerTimeForHit` (Single)
    #[serde(default)]
    pub hitmarker_time_for_hit: f32,
    /// `hitmarkerTimeForKill` (Single)
    #[serde(default)]
    pub hitmarker_time_for_kill: f32,
    /// `killInterruptsPreviousHit` (Boolean)
    #[serde(default)]
    pub kill_interrupts_previous_hit: bool,
    /// `hitmarkerPositionMethod` (EnumChoice)
    #[serde(default)]
    pub hitmarker_position_method: EHitmarkerPositionMethod,
    /// `crosshairInCombatTime` (Single)
    #[serde(default)]
    pub crosshair_in_combat_time: f32,
    /// `hitMarkerSoundHead` (Class)
    #[serde(default)]
    pub hit_marker_sound_head: Option<Handle<GlobalResourceAudio>>,
    /// `hitMarkerSoundBody` (Class)
    #[serde(default)]
    pub hit_marker_sound_body: Option<Handle<GlobalResourceAudio>>,
    /// `timeSinceLastHitmarkerRTPC` (Class)
    #[serde(default)]
    pub time_since_last_hitmarker_rtpc: Option<Handle<AudioRtpc>>,
    /// `killHitmarkerRTPC` (Class)
    #[serde(default)]
    pub kill_hitmarker_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for SGlobalCrosshairParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobalcrosshairparams.sglobal_crosshair_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobalcrosshairparams.sglobal_crosshair_params }
}

impl<'a> Extract<'a> for SGlobalCrosshairParams {
    const TYPE_NAME: &'static str = "SGlobalCrosshairParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            lerp_speed: inst.get_f32("lerpSpeed").unwrap_or_default(),
            position_smooth_factor: inst.get_f32("positionSmoothFactor").unwrap_or_default(),
            distance_smooth_factor: inst.get_f32("distanceSmoothFactor").unwrap_or_default(),
            range: inst.get_f32("range").unwrap_or_default(),
            hitmarker_time_for_hit: inst.get_f32("hitmarkerTimeForHit").unwrap_or_default(),
            hitmarker_time_for_kill: inst.get_f32("hitmarkerTimeForKill").unwrap_or_default(),
            kill_interrupts_previous_hit: inst.get_bool("killInterruptsPreviousHit").unwrap_or_default(),
            hitmarker_position_method: EHitmarkerPositionMethod::from_dcb_str(inst.get_str("hitmarkerPositionMethod").unwrap_or("")),
            crosshair_in_combat_time: inst.get_f32("crosshairInCombatTime").unwrap_or_default(),
            hit_marker_sound_head: match inst.get("hitMarkerSoundHead") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hit_marker_sound_body: match inst.get("hitMarkerSoundBody") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            time_since_last_hitmarker_rtpc: match inst.get("timeSinceLastHitmarkerRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            kill_hitmarker_rtpc: match inst.get("killHitmarkerRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

