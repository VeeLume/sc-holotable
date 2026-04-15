// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-test`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `UIPaintRenderNodeEntityComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPaintRenderNodeEntityComponentParams {
    /// `flashOverride` (Boolean)
    #[serde(default)]
    pub flash_override: bool,
    /// `defaultPreviewScene` (StrongPointer)
    #[serde(default)]
    pub default_preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
}

impl Pooled for UIPaintRenderNodeEntityComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_test.uipaint_render_node_entity_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_test.uipaint_render_node_entity_component_params }
}

impl<'a> Extract<'a> for UIPaintRenderNodeEntityComponentParams {
    const TYPE_NAME: &'static str = "UIPaintRenderNodeEntityComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flash_override: inst.get_bool("flashOverride").unwrap_or_default(),
            default_preview_scene: match inst.get("defaultPreviewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

