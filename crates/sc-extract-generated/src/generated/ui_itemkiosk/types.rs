// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-itemkiosk`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ItemKioskBrand`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemKioskBrand {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `logoPath` (String)
    #[serde(default)]
    pub logo_path: String,
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGBA8>>,
}

impl Pooled for ItemKioskBrand {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itemkiosk.item_kiosk_brand }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itemkiosk.item_kiosk_brand }
}

impl<'a> Extract<'a> for ItemKioskBrand {
    const TYPE_NAME: &'static str = "ItemKioskBrand";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            logo_path: inst.get_str("logoPath").map(String::from).unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

