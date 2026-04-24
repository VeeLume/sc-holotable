// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-digitalsignage`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `DigitalSignageContent`
pub struct DigitalSignageContent {
    /// `contentAspectRatio` (Reference)
    pub content_aspect_ratio: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
}

impl Pooled for DigitalSignageContent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_digitalsignage.digital_signage_content }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_digitalsignage.digital_signage_content }
}

impl<'a> Extract<'a> for DigitalSignageContent {
    const TYPE_NAME: &'static str = "DigitalSignageContent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            content_aspect_ratio: inst.get("contentAspectRatio").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `DigitalSignageContentSet`
pub struct DigitalSignageContentSet {
    /// `content` (Class (array))
    pub content: Vec<Handle<DigitalSignageContent>>,
}

impl Pooled for DigitalSignageContentSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_digitalsignage.digital_signage_content_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_digitalsignage.digital_signage_content_set }
}

impl<'a> Extract<'a> for DigitalSignageContentSet {
    const TYPE_NAME: &'static str = "DigitalSignageContentSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            content: inst.get_array("content")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DigitalSignageContent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<DigitalSignageContent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

