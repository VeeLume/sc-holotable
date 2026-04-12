// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalinteractionparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CarryableInteractionsMetadataDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryableInteractionsMetadataDef {
    /// `ignoreDefaultActionWhenBlocked` (Boolean)
    #[serde(default)]
    pub ignore_default_action_when_blocked: bool,
}

impl Pooled for CarryableInteractionsMetadataDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalinteractionparams.carryable_interactions_metadata_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalinteractionparams.carryable_interactions_metadata_def }
}

impl<'a> Extract<'a> for CarryableInteractionsMetadataDef {
    const TYPE_NAME: &'static str = "CarryableInteractionsMetadataDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ignore_default_action_when_blocked: inst.get_bool("ignoreDefaultActionWhenBlocked").unwrap_or_default(),
        }
    }
}

/// DCB type: `CarryableInteractionsMetadataConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryableInteractionsMetadataConfigDef {
    /// `metadata` (StrongPointer)
    #[serde(default)]
    pub metadata: Option<Handle<CarryableInteractionsMetadataDef>>,
}

impl Pooled for CarryableInteractionsMetadataConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalinteractionparams.carryable_interactions_metadata_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalinteractionparams.carryable_interactions_metadata_config_def }
}

impl<'a> Extract<'a> for CarryableInteractionsMetadataConfigDef {
    const TYPE_NAME: &'static str = "CarryableInteractionsMetadataConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            metadata: match inst.get("metadata") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CarryableInteractionsMetadataDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CarryableInteractionsMetadataDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SkinInteractableTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInteractableTemplate {
    /// `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `InteractionPoints` (Class (array))
    #[serde(default)]
    pub interaction_points: Vec<Handle<SInteractionPointParams>>,
}

impl Pooled for SkinInteractableTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalinteractionparams.skin_interactable_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalinteractionparams.skin_interactable_template }
}

impl<'a> Extract<'a> for SkinInteractableTemplate {
    const TYPE_NAME: &'static str = "SkinInteractableTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            interaction_points: inst.get_array("InteractionPoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SInteractionPointParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SInteractionPointParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SkinInteractableTemplates`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInteractableTemplates {
    /// `Templates` (Class (array))
    #[serde(default)]
    pub templates: Vec<Handle<SkinInteractableTemplate>>,
}

impl Pooled for SkinInteractableTemplates {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalinteractionparams.skin_interactable_templates }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalinteractionparams.skin_interactable_templates }
}

impl<'a> Extract<'a> for SkinInteractableTemplates {
    const TYPE_NAME: &'static str = "SkinInteractableTemplates";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            templates: inst.get_array("Templates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SkinInteractableTemplate>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SkinInteractableTemplate>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

