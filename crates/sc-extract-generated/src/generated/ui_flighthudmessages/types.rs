// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-flighthudmessages`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `FlightHUDUIMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightHUDUIMessage {
    /// `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// `priority` (Byte)
    #[serde(default)]
    pub priority: u32,
}

impl Pooled for FlightHUDUIMessage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_flighthudmessages.flight_huduimessage }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_flighthudmessages.flight_huduimessage }
}

impl<'a> Extract<'a> for FlightHUDUIMessage {
    const TYPE_NAME: &'static str = "FlightHUDUIMessage";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            priority: inst.get_u32("priority").unwrap_or_default(),
        }
    }
}

/// DCB type: `FlightHUDUIView_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightHUDUIView_Config {
    /// `messages` (Class)
    #[serde(default)]
    pub messages: Option<Handle<FlightHUDUIMessage>>,
}

impl Pooled for FlightHUDUIView_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_flighthudmessages.flight_huduiview_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_flighthudmessages.flight_huduiview_config }
}

impl<'a> Extract<'a> for FlightHUDUIView_Config {
    const TYPE_NAME: &'static str = "FlightHUDUIView_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            messages: match inst.get("messages") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FlightHUDUIMessage>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FlightHUDUIMessage>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

