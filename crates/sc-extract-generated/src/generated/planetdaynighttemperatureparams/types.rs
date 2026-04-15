// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `planetdaynighttemperatureparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `PlanetDayNightTemperatureTemplate`
pub struct PlanetDayNightTemperatureTemplate {
    /// `dayNightTemperatureParams` (Class)
    pub day_night_temperature_params: Option<Handle<PlanetDayNightTemperatureParams>>,
}

impl Pooled for PlanetDayNightTemperatureTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.planetdaynighttemperatureparams.planet_day_night_temperature_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.planetdaynighttemperatureparams.planet_day_night_temperature_template }
}

impl<'a> Extract<'a> for PlanetDayNightTemperatureTemplate {
    const TYPE_NAME: &'static str = "PlanetDayNightTemperatureTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            day_night_temperature_params: match inst.get("dayNightTemperatureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlanetDayNightTemperatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

