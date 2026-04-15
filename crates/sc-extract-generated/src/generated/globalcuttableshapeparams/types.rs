// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalcuttableshapeparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SGlobalCuttableShapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalCuttableShapeParams {
    /// `heatRequiredPerSegment` (Single)
    #[serde(default)]
    pub heat_required_per_segment: f32,
    /// `heatDissipationPerSecond` (Single)
    #[serde(default)]
    pub heat_dissipation_per_second: f32,
    /// `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `finishedEffect` (Class)
    #[serde(default)]
    pub finished_effect: Option<Handle<GlobalResourceParticle>>,
    /// `hitRadiusMin` (Single)
    #[serde(default)]
    pub hit_radius_min: f32,
    /// `hitRadiusMax` (Single)
    #[serde(default)]
    pub hit_radius_max: f32,
    /// `damageMultiplier` (Single)
    #[serde(default)]
    pub damage_multiplier: f32,
    /// `impactParticleLifeTime` (Single)
    #[serde(default)]
    pub impact_particle_life_time: f32,
    /// `highlightColor` (Class)
    #[serde(default)]
    pub highlight_color: Option<Handle<RGB>>,
    /// `highlightOccludedAlpha` (Single)
    #[serde(default)]
    pub highlight_occluded_alpha: f32,
    /// `highlightOutlineWidth` (Single)
    #[serde(default)]
    pub highlight_outline_width: f32,
    /// `highlightOutlineOnly` (Boolean)
    #[serde(default)]
    pub highlight_outline_only: bool,
}

impl Pooled for SGlobalCuttableShapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalcuttableshapeparams.sglobal_cuttable_shape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalcuttableshapeparams.sglobal_cuttable_shape_params }
}

impl<'a> Extract<'a> for SGlobalCuttableShapeParams {
    const TYPE_NAME: &'static str = "SGlobalCuttableShapeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            heat_required_per_segment: inst.get_f32("heatRequiredPerSegment").unwrap_or_default(),
            heat_dissipation_per_second: inst.get_f32("heatDissipationPerSecond").unwrap_or_default(),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            finished_effect: match inst.get("finishedEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hit_radius_min: inst.get_f32("hitRadiusMin").unwrap_or_default(),
            hit_radius_max: inst.get_f32("hitRadiusMax").unwrap_or_default(),
            damage_multiplier: inst.get_f32("damageMultiplier").unwrap_or_default(),
            impact_particle_life_time: inst.get_f32("impactParticleLifeTime").unwrap_or_default(),
            highlight_color: match inst.get("highlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            highlight_occluded_alpha: inst.get_f32("highlightOccludedAlpha").unwrap_or_default(),
            highlight_outline_width: inst.get_f32("highlightOutlineWidth").unwrap_or_default(),
            highlight_outline_only: inst.get_bool("highlightOutlineOnly").unwrap_or_default(),
        }
    }
}

