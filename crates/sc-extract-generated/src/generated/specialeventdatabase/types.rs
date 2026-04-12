// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `specialeventdatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SpecialEventManufacturer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEventManufacturer {
}

impl Pooled for SpecialEventManufacturer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.specialeventdatabase.special_event_manufacturer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.specialeventdatabase.special_event_manufacturer }
}

impl<'a> Extract<'a> for SpecialEventManufacturer {
    const TYPE_NAME: &'static str = "SpecialEventManufacturer";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SpecialEventDay`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEventDay {
    /// `manufacturers` (Reference (array))
    #[serde(default)]
    pub manufacturers: Vec<CigGuid>,
    /// `displayNames` (Locale (array))
    #[serde(default)]
    pub display_names: Vec<String>,
}

impl Pooled for SpecialEventDay {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.specialeventdatabase.special_event_day }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.specialeventdatabase.special_event_day }
}

impl<'a> Extract<'a> for SpecialEventDay {
    const TYPE_NAME: &'static str = "SpecialEventDay";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            manufacturers: inst.get_array("manufacturers")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            display_names: inst.get_array("displayNames")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SpecialEventDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEventDatabase {
    /// `days` (Reference (array))
    #[serde(default)]
    pub days: Vec<CigGuid>,
}

impl Pooled for SpecialEventDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.specialeventdatabase.special_event_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.specialeventdatabase.special_event_database }
}

impl<'a> Extract<'a> for SpecialEventDatabase {
    const TYPE_NAME: &'static str = "SpecialEventDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            days: inst.get_array("days")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

