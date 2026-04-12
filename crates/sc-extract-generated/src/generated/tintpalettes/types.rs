// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `tintpalettes`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `TintEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintEntry {
    /// `tintColor` (Class)
    #[serde(default)]
    pub tint_color: Option<Handle<SRGB8>>,
    /// `specColor` (Class)
    #[serde(default)]
    pub spec_color: Option<Handle<SRGB8>>,
    /// `glossiness` (Byte)
    #[serde(default)]
    pub glossiness: u32,
}

impl Pooled for TintEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tintpalettes.tint_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tintpalettes.tint_entry }
}

impl<'a> Extract<'a> for TintEntry {
    const TYPE_NAME: &'static str = "TintEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tint_color: match inst.get("tintColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spec_color: match inst.get("specColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            glossiness: inst.get_u32("glossiness").unwrap_or_default(),
        }
    }
}

/// DCB type: `TintPalette`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintPalette {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `children` (Class (array))
    #[serde(default)]
    pub children: Vec<Handle<TintPalette>>,
    /// `inheritsParams` (Boolean)
    #[serde(default)]
    pub inherits_params: bool,
    /// `entryA` (Class)
    #[serde(default)]
    pub entry_a: Option<Handle<TintEntry>>,
    /// `entryB` (Class)
    #[serde(default)]
    pub entry_b: Option<Handle<TintEntry>>,
    /// `entryC` (Class)
    #[serde(default)]
    pub entry_c: Option<Handle<TintEntry>>,
    /// `glassColor` (Class)
    #[serde(default)]
    pub glass_color: Option<Handle<SRGB8>>,
    /// `decalTexture` (String)
    #[serde(default)]
    pub decal_texture: String,
    /// `decalColorR` (Class)
    #[serde(default)]
    pub decal_color_r: Option<Handle<SRGB8>>,
    /// `decalColorG` (Class)
    #[serde(default)]
    pub decal_color_g: Option<Handle<SRGB8>>,
    /// `decalColorB` (Class)
    #[serde(default)]
    pub decal_color_b: Option<Handle<SRGB8>>,
}

impl Pooled for TintPalette {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tintpalettes.tint_palette }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tintpalettes.tint_palette }
}

impl<'a> Extract<'a> for TintPalette {
    const TYPE_NAME: &'static str = "TintPalette";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            children: inst.get_array("children")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TintPalette>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TintPalette>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            inherits_params: inst.get_bool("inheritsParams").unwrap_or_default(),
            entry_a: match inst.get("entryA") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entry_b: match inst.get("entryB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entry_c: match inst.get("entryC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            glass_color: match inst.get("glassColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decal_texture: inst.get_str("decalTexture").map(String::from).unwrap_or_default(),
            decal_color_r: match inst.get("decalColorR") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decal_color_g: match inst.get("decalColorG") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decal_color_b: match inst.get("decalColorB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TintPaletteTree`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintPaletteTree {
    /// `root` (Class)
    #[serde(default)]
    pub root: Option<Handle<TintPalette>>,
    /// `templateTree` (Reference)
    #[serde(default)]
    pub template_tree: Option<CigGuid>,
}

impl Pooled for TintPaletteTree {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tintpalettes.tint_palette_tree }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tintpalettes.tint_palette_tree }
}

impl<'a> Extract<'a> for TintPaletteTree {
    const TYPE_NAME: &'static str = "TintPaletteTree";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            root: match inst.get("root") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintPalette>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintPalette>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            template_tree: inst.get("templateTree").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

