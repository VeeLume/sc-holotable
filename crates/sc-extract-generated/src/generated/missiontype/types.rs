// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missiontype`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MissionType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionType {
    /// `IconName` (String)
    #[serde(default)]
    pub icon_name: String,
    /// `LocalisedTypeName` (Locale)
    #[serde(default)]
    pub localised_type_name: String,
    /// `svgIconPath` (String)
    #[serde(default)]
    pub svg_icon_path: String,
    /// `DisplayTime` (Single)
    #[serde(default)]
    pub display_time: f32,
}

impl Pooled for MissionType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiontype.mission_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiontype.mission_type }
}

impl<'a> Extract<'a> for MissionType {
    const TYPE_NAME: &'static str = "MissionType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            icon_name: inst.get_str("IconName").map(String::from).unwrap_or_default(),
            localised_type_name: inst.get_str("LocalisedTypeName").map(String::from).unwrap_or_default(),
            svg_icon_path: inst.get_str("svgIconPath").map(String::from).unwrap_or_default(),
            display_time: inst.get_f32("DisplayTime").unwrap_or_default(),
        }
    }
}

