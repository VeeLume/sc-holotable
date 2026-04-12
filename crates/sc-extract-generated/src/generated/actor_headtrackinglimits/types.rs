// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-headtrackinglimits`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCItemSeatHeadTrackingPositionLimitParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSeatHeadTrackingPositionLimitParams {
    /// `leftRight` (Single)
    #[serde(default)]
    pub left_right: f32,
    /// `up` (Single)
    #[serde(default)]
    pub up: f32,
    /// `down` (Single)
    #[serde(default)]
    pub down: f32,
    /// `forward` (Single)
    #[serde(default)]
    pub forward: f32,
    /// `backward` (Single)
    #[serde(default)]
    pub backward: f32,
}

impl Pooled for SCItemSeatHeadTrackingPositionLimitParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_headtrackinglimits.scitem_seat_head_tracking_position_limit_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_headtrackinglimits.scitem_seat_head_tracking_position_limit_params }
}

impl<'a> Extract<'a> for SCItemSeatHeadTrackingPositionLimitParams {
    const TYPE_NAME: &'static str = "SCItemSeatHeadTrackingPositionLimitParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            left_right: inst.get_f32("leftRight").unwrap_or_default(),
            up: inst.get_f32("up").unwrap_or_default(),
            down: inst.get_f32("down").unwrap_or_default(),
            forward: inst.get_f32("forward").unwrap_or_default(),
            backward: inst.get_f32("backward").unwrap_or_default(),
        }
    }
}

