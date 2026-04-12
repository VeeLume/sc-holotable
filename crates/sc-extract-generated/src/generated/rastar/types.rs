// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `rastar`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `HologramParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HologramParams {
    /// `ValidMaterial` (String)
    #[serde(default)]
    pub valid_material: String,
    /// `InvalidMaterial` (String)
    #[serde(default)]
    pub invalid_material: String,
}

impl Pooled for HologramParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rastar.hologram_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rastar.hologram_params }
}

impl<'a> Extract<'a> for HologramParams {
    const TYPE_NAME: &'static str = "HologramParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            valid_material: inst.get_str("ValidMaterial").map(String::from).unwrap_or_default(),
            invalid_material: inst.get_str("InvalidMaterial").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RaSTaRLibraryElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaSTaRLibraryElement {
    /// `filePath` (String)
    #[serde(default)]
    pub file_path: String,
    /// `displayName` (String)
    #[serde(default)]
    pub display_name: String,
    /// `isMainModule` (Boolean)
    #[serde(default)]
    pub is_main_module: bool,
    /// `isIndependantModule` (Boolean)
    #[serde(default)]
    pub is_independant_module: bool,
}

impl Pooled for RaSTaRLibraryElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rastar.ra_sta_rlibrary_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rastar.ra_sta_rlibrary_element }
}

impl<'a> Extract<'a> for RaSTaRLibraryElement {
    const TYPE_NAME: &'static str = "RaSTaRLibraryElement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            file_path: inst.get_str("filePath").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            is_main_module: inst.get_bool("isMainModule").unwrap_or_default(),
            is_independant_module: inst.get_bool("isIndependantModule").unwrap_or_default(),
        }
    }
}

/// DCB type: `RaSTaRLibraryCategory`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaSTaRLibraryCategory {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `elements` (Reference (array))
    #[serde(default)]
    pub elements: Vec<CigGuid>,
}

impl Pooled for RaSTaRLibraryCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rastar.ra_sta_rlibrary_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rastar.ra_sta_rlibrary_category }
}

impl<'a> Extract<'a> for RaSTaRLibraryCategory {
    const TYPE_NAME: &'static str = "RaSTaRLibraryCategory";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            elements: inst.get_array("elements")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RaSTaRLibrary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaSTaRLibrary {
    /// `categories` (Class (array))
    #[serde(default)]
    pub categories: Vec<Handle<RaSTaRLibraryCategory>>,
}

impl Pooled for RaSTaRLibrary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rastar.ra_sta_rlibrary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rastar.ra_sta_rlibrary }
}

impl<'a> Extract<'a> for RaSTaRLibrary {
    const TYPE_NAME: &'static str = "RaSTaRLibrary";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RaSTaRLibraryCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RaSTaRLibraryCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

