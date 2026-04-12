// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `aiwavecollection`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `AIWaveCollection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIWaveCollection {
    /// `waves` (Class (array))
    #[serde(default)]
    pub waves: Vec<Handle<AIWave>>,
}

impl Pooled for AIWaveCollection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiwavecollection.aiwave_collection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiwavecollection.aiwave_collection }
}

impl<'a> Extract<'a> for AIWaveCollection {
    const TYPE_NAME: &'static str = "AIWaveCollection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            waves: inst.get_array("waves")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIWave>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIWave>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIWave`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIWave {
    /// `id` (Int32)
    #[serde(default)]
    pub id: i32,
    /// `textId` (String)
    #[serde(default)]
    pub text_id: String,
    /// `staggerTime` (Single)
    #[serde(default)]
    pub stagger_time: f32,
    /// `members` (Class (array))
    #[serde(default)]
    pub members: Vec<Handle<AIWaveMember>>,
}

impl Pooled for AIWave {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiwavecollection.aiwave }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiwavecollection.aiwave }
}

impl<'a> Extract<'a> for AIWave {
    const TYPE_NAME: &'static str = "AIWave";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_i32("id").unwrap_or_default(),
            text_id: inst.get_str("textId").map(String::from).unwrap_or_default(),
            stagger_time: inst.get_f32("staggerTime").unwrap_or_default(),
            members: inst.get_array("members")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIWaveMember>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIWaveMember>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIWaveMember`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIWaveMember {
    /// `archetype` (String)
    #[serde(default)]
    pub archetype: String,
    /// `entityClassDefinition` (Reference)
    #[serde(default)]
    pub entity_class_definition: Option<CigGuid>,
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `vehicleName` (String)
    #[serde(default)]
    pub vehicle_name: String,
    /// `amount` (Int32)
    #[serde(default)]
    pub amount: i32,
    /// `minAmount` (Int32)
    #[serde(default)]
    pub min_amount: i32,
    /// `midAmount` (Int32)
    #[serde(default)]
    pub mid_amount: i32,
    /// `maxAmount` (Int32)
    #[serde(default)]
    pub max_amount: i32,
    /// `crewManifestOverride` (Reference)
    #[serde(default)]
    pub crew_manifest_override: Option<CigGuid>,
    /// `skillsetOverride` (Reference)
    #[serde(default)]
    pub skillset_override: Option<CigGuid>,
    /// `cargoManifests` (Reference (array))
    #[serde(default)]
    pub cargo_manifests: Vec<CigGuid>,
    /// `aiDamageModifiersOverride` (Reference)
    #[serde(default)]
    pub ai_damage_modifiers_override: Option<CigGuid>,
}

impl Pooled for AIWaveMember {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiwavecollection.aiwave_member }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiwavecollection.aiwave_member }
}

impl<'a> Extract<'a> for AIWaveMember {
    const TYPE_NAME: &'static str = "AIWaveMember";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            archetype: inst.get_str("archetype").map(String::from).unwrap_or_default(),
            entity_class_definition: inst.get("entityClassDefinition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            vehicle_name: inst.get_str("vehicleName").map(String::from).unwrap_or_default(),
            amount: inst.get_i32("amount").unwrap_or_default(),
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            mid_amount: inst.get_i32("midAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
            crew_manifest_override: inst.get("crewManifestOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            skillset_override: inst.get("skillsetOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            cargo_manifests: inst.get_array("cargoManifests")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            ai_damage_modifiers_override: inst.get("aiDamageModifiersOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

