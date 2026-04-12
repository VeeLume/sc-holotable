// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `loadoutkits`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `LoadoutKit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutKit {
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for LoadoutKit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.loadoutkits.loadout_kit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.loadoutkits.loadout_kit }
}

impl<'a> Extract<'a> for LoadoutKit {
    const TYPE_NAME: &'static str = "LoadoutKit";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            loadout: match inst.get("loadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

