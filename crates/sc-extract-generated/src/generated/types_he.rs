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

/// DCB type: `HealthIconData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIconData {
    /// DCB field: `healthIconStatusEffect` (Class (array))
    #[serde(default)]
    pub health_icon_status_effect: Vec<Handle<HealthIconStatusEffect>>,
    /// DCB field: `checkPlasmaDamage` (Boolean)
    #[serde(default)]
    pub check_plasma_damage: bool,
    /// DCB field: `plasmaDamageIconIndex` (Int32)
    #[serde(default)]
    pub plasma_damage_icon_index: i32,
}

impl Pooled for HealthIconData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_he.health_icon_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_he.health_icon_data }
}

impl<'a> Extract<'a> for HealthIconData {
    const TYPE_NAME: &'static str = "HealthIconData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            health_icon_status_effect: inst.get_array("healthIconStatusEffect")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HealthIconStatusEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HealthIconStatusEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            check_plasma_damage: inst.get_bool("checkPlasmaDamage").unwrap_or_default(),
            plasma_damage_icon_index: inst.get_i32("plasmaDamageIconIndex").unwrap_or_default(),
        }
    }
}

/// DCB type: `HealthIconStatusEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIconStatusEffect {
    /// DCB field: `statusEffectType` (EnumChoice)
    #[serde(default)]
    pub status_effect_type: String,
    /// DCB field: `index` (Int32)
    #[serde(default)]
    pub index: i32,
}

impl Pooled for HealthIconStatusEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_he.health_icon_status_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_he.health_icon_status_effect }
}

impl<'a> Extract<'a> for HealthIconStatusEffect {
    const TYPE_NAME: &'static str = "HealthIconStatusEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            status_effect_type: inst.get_str("statusEffectType").map(String::from).unwrap_or_default(),
            index: inst.get_i32("index").unwrap_or_default(),
        }
    }
}

/// DCB type: `HealthTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTemplate {
    /// DCB field: `bodyMapping` (Reference)
    #[serde(default)]
    pub body_mapping: Option<CigGuid>,
    /// DCB field: `healthConfig` (Reference)
    #[serde(default)]
    pub health_config: Option<CigGuid>,
}

impl Pooled for HealthTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_he.health_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_he.health_template }
}

impl<'a> Extract<'a> for HealthTemplate {
    const TYPE_NAME: &'static str = "HealthTemplate";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            body_mapping: inst.get("bodyMapping").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            health_config: inst.get("healthConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

