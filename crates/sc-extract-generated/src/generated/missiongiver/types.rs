// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missiongiver`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MissionGiver`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionGiver {
    /// `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
    /// `reputation` (Reference)
    #[serde(default)]
    pub reputation: Option<CigGuid>,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `headquarters` (Locale)
    #[serde(default)]
    pub headquarters: String,
    /// `invitationTimeout` (Single)
    #[serde(default)]
    pub invitation_timeout: f32,
    /// `visitTimeout` (Single)
    #[serde(default)]
    pub visit_timeout: f32,
    /// `shortCooldown` (Single)
    #[serde(default)]
    pub short_cooldown: f32,
    /// `mediumCooldown` (Single)
    #[serde(default)]
    pub medium_cooldown: f32,
    /// `longCooldown` (Single)
    #[serde(default)]
    pub long_cooldown: f32,
    /// `Allies` (Locale (array))
    #[serde(default)]
    pub allies: Vec<String>,
    /// `Enemies` (Locale (array))
    #[serde(default)]
    pub enemies: Vec<String>,
    /// `propertiesBB` (Class (array))
    #[serde(default)]
    pub properties_bb: Vec<Handle<SReputationContextBBPropertyParams>>,
}

impl Pooled for MissionGiver {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missiongiver.mission_giver }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missiongiver.mission_giver }
}

impl<'a> Extract<'a> for MissionGiver {
    const TYPE_NAME: &'static str = "MissionGiver";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation: inst.get("reputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            headquarters: inst.get_str("headquarters").map(String::from).unwrap_or_default(),
            invitation_timeout: inst.get_f32("invitationTimeout").unwrap_or_default(),
            visit_timeout: inst.get_f32("visitTimeout").unwrap_or_default(),
            short_cooldown: inst.get_f32("shortCooldown").unwrap_or_default(),
            medium_cooldown: inst.get_f32("mediumCooldown").unwrap_or_default(),
            long_cooldown: inst.get_f32("longCooldown").unwrap_or_default(),
            allies: inst.get_array("Allies")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            enemies: inst.get_array("Enemies")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            properties_bb: inst.get_array("propertiesBB")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

