// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-graphs`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `UIGraph_BlockingMessagePopUpComponent`
/// Inherits from: `CtxGraph_Component`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIGraph_BlockingMessagePopUpComponent {
    /// `errorFormat` (String)
    #[serde(default)]
    pub error_format: String,
    /// `provider` (EnumChoice)
    #[serde(default)]
    pub provider: UIGraph_BlockingMessagePopUpProvider,
}

impl Pooled for UIGraph_BlockingMessagePopUpComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_graphs.uigraph_blocking_message_pop_up_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_graphs.uigraph_blocking_message_pop_up_component }
}

impl<'a> Extract<'a> for UIGraph_BlockingMessagePopUpComponent {
    const TYPE_NAME: &'static str = "UIGraph_BlockingMessagePopUpComponent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            error_format: inst.get_str("errorFormat").map(String::from).unwrap_or_default(),
            provider: UIGraph_BlockingMessagePopUpProvider::from_dcb_str(inst.get_str("provider").unwrap_or("")),
        }
    }
}

