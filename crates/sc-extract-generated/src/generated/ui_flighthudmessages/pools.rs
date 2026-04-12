// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-flighthudmessages` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiFlighthudmessagesPools {
    #[serde(default)]
    pub flight_huduimessage: Vec<Option<FlightHUDUIMessage>>,
    #[serde(default)]
    pub flight_huduiview_config: Vec<Option<FlightHUDUIView_Config>>,
}
