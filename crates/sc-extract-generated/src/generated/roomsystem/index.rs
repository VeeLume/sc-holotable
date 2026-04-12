// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `roomsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomsystemIndex {
    #[serde(default)]
    pub fire_hazard_global_params: HashMap<CigGuid, Handle<FireHazardGlobalParams>>,
    #[serde(default)]
    pub gas_params: HashMap<CigGuid, Handle<GasParams>>,
    #[serde(default)]
    pub atmospheric_composition_template: HashMap<CigGuid, Handle<AtmosphericCompositionTemplate>>,
    #[serde(default)]
    pub global_gas_params: HashMap<CigGuid, Handle<GlobalGasParams>>,
    #[serde(default)]
    pub global_room_state_params: HashMap<CigGuid, Handle<GlobalRoomStateParams>>,
    #[serde(default)]
    pub asteroid_state_template: HashMap<CigGuid, Handle<AsteroidStateTemplate>>,
    #[serde(default)]
    pub asteroid_behavior: HashMap<CigGuid, Handle<AsteroidBehavior>>,
    #[serde(default)]
    pub atmosphere_state_template: HashMap<CigGuid, Handle<AtmosphereStateTemplate>>,
    #[serde(default)]
    pub atmosphere_state_pressure_template: HashMap<CigGuid, Handle<AtmosphereStatePressureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_temperature_template: HashMap<CigGuid, Handle<AtmosphereStateTemperatureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_humidity_template: HashMap<CigGuid, Handle<AtmosphereStateHumidityTemplate>>,
    #[serde(default)]
    pub atmosphere_behavior: HashMap<CigGuid, Handle<AtmosphereBehavior>>,
    #[serde(default)]
    pub electrical_state_template: HashMap<CigGuid, Handle<ElectricalStateTemplate>>,
    #[serde(default)]
    pub electrical_behavior: HashMap<CigGuid, Handle<ElectricalBehavior>>,
    #[serde(default)]
    pub radiation_state_template: HashMap<CigGuid, Handle<RadiationStateTemplate>>,
    #[serde(default)]
    pub radiation_behavior: HashMap<CigGuid, Handle<RadiationBehavior>>,
}

impl RoomsystemIndex {
    pub fn len(&self) -> usize {
        self.fire_hazard_global_params.len()
            + self.gas_params.len()
            + self.atmospheric_composition_template.len()
            + self.global_gas_params.len()
            + self.global_room_state_params.len()
            + self.asteroid_state_template.len()
            + self.asteroid_behavior.len()
            + self.atmosphere_state_template.len()
            + self.atmosphere_state_pressure_template.len()
            + self.atmosphere_state_temperature_template.len()
            + self.atmosphere_state_humidity_template.len()
            + self.atmosphere_behavior.len()
            + self.electrical_state_template.len()
            + self.electrical_behavior.len()
            + self.radiation_state_template.len()
            + self.radiation_behavior.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
