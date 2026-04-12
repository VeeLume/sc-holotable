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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PlanetDayNightTemperatureParams`
/// Inherits from: `PlanetDayNightTemperatureBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetDayNightTemperatureParams {
    /// `DayNightCoolingBaseRate` (Single)
    #[serde(default)]
    pub day_night_cooling_base_rate: f32,
    /// `DayNightCoolingHumidityModifier` (Single)
    #[serde(default)]
    pub day_night_cooling_humidity_modifier: f32,
    /// `DayNightCoolingHumidityMultiplier` (Single)
    #[serde(default)]
    pub day_night_cooling_humidity_multiplier: f32,
}

impl Pooled for PlanetDayNightTemperatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.planetdaynighttemperatureparams.planet_day_night_temperature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.planetdaynighttemperatureparams.planet_day_night_temperature_params }
}

impl<'a> Extract<'a> for PlanetDayNightTemperatureParams {
    const TYPE_NAME: &'static str = "PlanetDayNightTemperatureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            day_night_cooling_base_rate: inst.get_f32("DayNightCoolingBaseRate").unwrap_or_default(),
            day_night_cooling_humidity_modifier: inst.get_f32("DayNightCoolingHumidityModifier").unwrap_or_default(),
            day_night_cooling_humidity_multiplier: inst.get_f32("DayNightCoolingHumidityMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetDayNightTemperatureTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetDayNightTemperatureTemplate {
    /// `dayNightTemperatureParams` (Class)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlanetDayNightTemperatureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

