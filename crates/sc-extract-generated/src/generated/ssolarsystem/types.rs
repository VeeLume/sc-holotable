// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ssolarsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SSolarSystem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSolarSystem {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `landingZoneInventory` (Reference)
    #[serde(default)]
    pub landing_zone_inventory: Option<CigGuid>,
    /// `DefaultLocation` (Reference)
    #[serde(default)]
    pub default_location: Option<CigGuid>,
    /// `SolarSystemRecord` (Reference)
    #[serde(default)]
    pub solar_system_record: Option<CigGuid>,
    /// `galacticPosition` (Class)
    #[serde(default)]
    pub galactic_position: Option<Handle<Vec3>>,
    /// `organicFallbackBedrockTint` (Class)
    #[serde(default)]
    pub organic_fallback_bedrock_tint: Option<Handle<RGB>>,
    /// `organicFallbackSoilTint` (Class)
    #[serde(default)]
    pub organic_fallback_soil_tint: Option<Handle<RGB>>,
}

impl Pooled for SSolarSystem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ssolarsystem.ssolar_system }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ssolarsystem.ssolar_system }
}

impl<'a> Extract<'a> for SSolarSystem {
    const TYPE_NAME: &'static str = "SSolarSystem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            landing_zone_inventory: inst.get("landingZoneInventory").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_location: inst.get("DefaultLocation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            solar_system_record: inst.get("SolarSystemRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            galactic_position: match inst.get("galacticPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            organic_fallback_bedrock_tint: match inst.get("organicFallbackBedrockTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            organic_fallback_soil_tint: match inst.get("organicFallbackSoilTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

