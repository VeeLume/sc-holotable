// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `BasicStatusEffectApplicationType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicStatusEffectApplicationType {
    /// DCB field: `valueType` (EnumChoice)
    #[serde(default)]
    pub value_type: String,
}

impl Pooled for BasicStatusEffectApplicationType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ba.basic_status_effect_application_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ba.basic_status_effect_application_type }
}

impl<'a> Extract<'a> for BasicStatusEffectApplicationType {
    const TYPE_NAME: &'static str = "BasicStatusEffectApplicationType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value_type: inst.get_str("valueType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BaseCargoFillCapacityValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCargoFillCapacityValue {
}

impl Pooled for BaseCargoFillCapacityValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ba.base_cargo_fill_capacity_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ba.base_cargo_fill_capacity_value }
}

impl<'a> Extract<'a> for BaseCargoFillCapacityValue {
    const TYPE_NAME: &'static str = "BaseCargoFillCapacityValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItem {
}

impl Pooled for BaseItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ba.base_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ba.base_item }
}

impl<'a> Extract<'a> for BaseItem {
    const TYPE_NAME: &'static str = "BaseItem";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseJournalEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseJournalEntry {
}

impl Pooled for BaseJournalEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ba.base_journal_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ba.base_journal_entry }
}

impl<'a> Extract<'a> for BaseJournalEntry {
    const TYPE_NAME: &'static str = "BaseJournalEntry";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseService`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseService {
    /// DCB field: `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `productName` (Locale)
    #[serde(default)]
    pub product_name: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `serviceDelayTime` (Single)
    #[serde(default)]
    pub service_delay_time: f32,
    /// DCB field: `hudMessage` (Locale)
    #[serde(default)]
    pub hud_message: String,
}

impl Pooled for BaseService {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ba.base_service }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ba.base_service }
}

impl<'a> Extract<'a> for BaseService {
    const TYPE_NAME: &'static str = "BaseService";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            product_name: inst.get_str("productName").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            service_delay_time: inst.get_f32("serviceDelayTime").unwrap_or_default(),
            hud_message: inst.get_str("hudMessage").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BaseMissionPropertyValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseMissionPropertyValue {
}

impl Pooled for BaseMissionPropertyValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ba.base_mission_property_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ba.base_mission_property_value }
}

impl<'a> Extract<'a> for BaseMissionPropertyValue {
    const TYPE_NAME: &'static str = "BaseMissionPropertyValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseMissionModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseMissionModifier {
    /// DCB field: `modifierName` (String)
    #[serde(default)]
    pub modifier_name: String,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for BaseMissionModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ba.base_mission_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ba.base_mission_modifier }
}

impl<'a> Extract<'a> for BaseMissionModifier {
    const TYPE_NAME: &'static str = "BaseMissionModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_name: inst.get_str("modifierName").map(String::from).unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
        }
    }
}

