// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-playerecggraph_config_playerecggraphconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `PlayerECGGraph_Config`
pub struct PlayerECGGraph_Config {
    /// `idleAmpRangeMax` (Single)
    pub idle_amp_range_max: f32,
    /// `spikeFrameDuration` (Byte)
    pub spike_frame_duration: u32,
    /// `spikeFrameDurationRandFactor` (Single)
    pub spike_frame_duration_rand_factor: f32,
    /// `waveFreq` (Byte)
    pub wave_freq: u32,
    /// `pulseAmpMin` (Single)
    pub pulse_amp_min: f32,
    /// `pulseAmpMax` (Single)
    pub pulse_amp_max: f32,
    /// `updateRate` (Byte)
    pub update_rate: u32,
}

impl Pooled for PlayerECGGraph_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_playerecggraph_config_playerecggraphconfig
            .player_ecggraph_config
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_playerecggraph_config_playerecggraphconfig
            .player_ecggraph_config
    }
}

impl<'a> Extract<'a> for PlayerECGGraph_Config {
    const TYPE_NAME: &'static str = "PlayerECGGraph_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            idle_amp_range_max: inst.get_f32("idleAmpRangeMax").unwrap_or_default(),
            spike_frame_duration: inst.get_u32("spikeFrameDuration").unwrap_or_default(),
            spike_frame_duration_rand_factor: inst
                .get_f32("spikeFrameDurationRandFactor")
                .unwrap_or_default(),
            wave_freq: inst.get_u32("waveFreq").unwrap_or_default(),
            pulse_amp_min: inst.get_f32("pulseAmpMin").unwrap_or_default(),
            pulse_amp_max: inst.get_f32("pulseAmpMax").unwrap_or_default(),
            update_rate: inst.get_u32("updateRate").unwrap_or_default(),
        }
    }
}
