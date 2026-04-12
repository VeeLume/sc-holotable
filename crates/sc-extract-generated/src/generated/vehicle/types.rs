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
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SOperatorModeLabels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOperatorModeLabels {
    /// `fullNames` (Locale)
    #[serde(default)]
    pub full_names: String,
    /// `shortNames` (Locale)
    #[serde(default)]
    pub short_names: String,
}

impl Pooled for SOperatorModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.soperator_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.soperator_mode_labels }
}

impl<'a> Extract<'a> for SOperatorModeLabels {
    const TYPE_NAME: &'static str = "SOperatorModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            full_names: inst.get_str("fullNames").map(String::from).unwrap_or_default(),
            short_names: inst.get_str("shortNames").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMasterModeLabels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMasterModeLabels {
    /// `fullNames` (Locale)
    #[serde(default)]
    pub full_names: String,
    /// `shortNames` (Locale)
    #[serde(default)]
    pub short_names: String,
}

impl Pooled for SMasterModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.smaster_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.smaster_mode_labels }
}

impl<'a> Extract<'a> for SMasterModeLabels {
    const TYPE_NAME: &'static str = "SMasterModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            full_names: inst.get_str("fullNames").map(String::from).unwrap_or_default(),
            short_names: inst.get_str("shortNames").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleSerialNumberCharacterType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSerialNumberCharacterType {
    /// `possibleCharacters` (String)
    #[serde(default)]
    pub possible_characters: String,
}

impl Pooled for VehicleSerialNumberCharacterType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.vehicle_serial_number_character_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.vehicle_serial_number_character_type }
}

impl<'a> Extract<'a> for VehicleSerialNumberCharacterType {
    const TYPE_NAME: &'static str = "VehicleSerialNumberCharacterType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            possible_characters: inst.get_str("possibleCharacters").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleSerialNumberFormat`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSerialNumberFormat {
    /// `characterTypes` (Reference (array))
    #[serde(default)]
    pub character_types: Vec<CigGuid>,
    /// `format` (Reference (array))
    #[serde(default)]
    pub format: Vec<CigGuid>,
}

impl Pooled for VehicleSerialNumberFormat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.vehicle_serial_number_format }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.vehicle_serial_number_format }
}

impl<'a> Extract<'a> for VehicleSerialNumberFormat {
    const TYPE_NAME: &'static str = "VehicleSerialNumberFormat";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            character_types: inst.get_array("characterTypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            format: inst.get_array("format")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleRole`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleRole {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
}

impl Pooled for VehicleRole {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.vehicle_role }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.vehicle_role }
}

impl<'a> Extract<'a> for VehicleRole {
    const TYPE_NAME: &'static str = "VehicleRole";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleCareer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleCareer {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `roleList` (Reference (array))
    #[serde(default)]
    pub role_list: Vec<CigGuid>,
}

impl Pooled for VehicleCareer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vehicle.vehicle_career }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vehicle.vehicle_career }
}

impl<'a> Extract<'a> for VehicleCareer {
    const TYPE_NAME: &'static str = "VehicleCareer";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            role_list: inst.get_array("roleList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
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

