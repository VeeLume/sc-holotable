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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SkinInteractableTemplate`
pub struct SkinInteractableTemplate {
    /// `Type` (EnumChoice)
    pub r#type: EItemType,
    /// `InteractionPoints` (Class (array))
    pub interaction_points: Vec<Handle<SInteractionPointParams>>,
}

impl Pooled for SkinInteractableTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.globalinteractionparams.skin_interactable_template
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.globalinteractionparams.skin_interactable_template
    }
}

impl<'a> Extract<'a> for SkinInteractableTemplate {
    const TYPE_NAME: &'static str = "SkinInteractableTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: EItemType::from_dcb_str(inst.get_str("Type").unwrap_or("")),
            interaction_points: inst
                .get_array("InteractionPoints")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SInteractionPointParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SInteractionPointParams>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SkinInteractableTemplates`
pub struct SkinInteractableTemplates {
    /// `Templates` (Class (array))
    pub templates: Vec<Handle<SkinInteractableTemplate>>,
}

impl Pooled for SkinInteractableTemplates {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.globalinteractionparams.skin_interactable_templates
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.globalinteractionparams.skin_interactable_templates
    }
}

impl<'a> Extract<'a> for SkinInteractableTemplates {
    const TYPE_NAME: &'static str = "SkinInteractableTemplates";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            templates: inst
                .get_array("Templates")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SkinInteractableTemplate>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SkinInteractableTemplate>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
