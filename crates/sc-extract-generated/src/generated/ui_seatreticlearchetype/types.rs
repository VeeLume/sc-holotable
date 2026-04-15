// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-seatreticlearchetype`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SeatReticleArchetype`
pub struct SeatReticleArchetype {
    /// `fixed` (Boolean)
    pub fixed: bool,
    /// `look` (Boolean)
    pub look: bool,
    /// `velocity` (Boolean)
    pub velocity: bool,
    /// `control` (Boolean)
    pub control: bool,
    /// `atmospheric` (Boolean)
    pub atmospheric: bool,
}

impl Pooled for SeatReticleArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_seatreticlearchetype.seat_reticle_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_seatreticlearchetype.seat_reticle_archetype }
}

impl<'a> Extract<'a> for SeatReticleArchetype {
    const TYPE_NAME: &'static str = "SeatReticleArchetype";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fixed: inst.get_bool("fixed").unwrap_or_default(),
            look: inst.get_bool("look").unwrap_or_default(),
            velocity: inst.get_bool("velocity").unwrap_or_default(),
            control: inst.get_bool("control").unwrap_or_default(),
            atmospheric: inst.get_bool("atmospheric").unwrap_or_default(),
        }
    }
}

