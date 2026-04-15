// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `vehicle`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SOperatorModeLabels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOperatorModeLabels {
    /// `fullNames` (Locale)
    #[serde(default)]
    pub full_names: LocaleKey,
    /// `shortNames` (Locale)
    #[serde(default)]
    pub short_names: LocaleKey,
}

impl Pooled for SOperatorModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.soperator_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.soperator_mode_labels }
}

impl<'a> Extract<'a> for SOperatorModeLabels {
    const TYPE_NAME: &'static str = "SOperatorModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            full_names: inst.get_str("fullNames").map(LocaleKey::from).unwrap_or_default(),
            short_names: inst.get_str("shortNames").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMasterModeLabels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMasterModeLabels {
    /// `fullNames` (Locale)
    #[serde(default)]
    pub full_names: LocaleKey,
    /// `shortNames` (Locale)
    #[serde(default)]
    pub short_names: LocaleKey,
}

impl Pooled for SMasterModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.smaster_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.smaster_mode_labels }
}

impl<'a> Extract<'a> for SMasterModeLabels {
    const TYPE_NAME: &'static str = "SMasterModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            full_names: inst.get_str("fullNames").map(LocaleKey::from).unwrap_or_default(),
            short_names: inst.get_str("shortNames").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleCareerList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleCareerList {
    /// `careerList` (Reference (array))
    #[serde(default)]
    pub career_list: Vec<CigGuid>,
}

impl Pooled for VehicleCareerList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.vehicle_career_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.vehicle_career_list }
}

impl<'a> Extract<'a> for VehicleCareerList {
    const TYPE_NAME: &'static str = "VehicleCareerList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            career_list: inst.get_array("careerList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

