// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `Time`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    /// DCB field: `hour` (Int32)
    #[serde(default)]
    pub hour: i32,
    /// DCB field: `minute` (Int32)
    #[serde(default)]
    pub minute: i32,
    /// DCB field: `second` (Int32)
    #[serde(default)]
    pub second: i32,
}

impl Pooled for Time {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ti.time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ti.time }
}

impl<'a> Extract<'a> for Time {
    const TYPE_NAME: &'static str = "Time";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hour: inst.get_i32("hour").unwrap_or_default(),
            minute: inst.get_i32("minute").unwrap_or_default(),
            second: inst.get_i32("second").unwrap_or_default(),
        }
    }
}

/// DCB type: `TimeValue_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeValue_Base {
}

impl Pooled for TimeValue_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ti.time_value_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ti.time_value_base }
}

impl<'a> Extract<'a> for TimeValue_Base {
    const TYPE_NAME: &'static str = "TimeValue_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TintEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintEntry {
    /// DCB field: `tintColor` (Class)
    #[serde(default)]
    pub tint_color: Option<Handle<SRGB8>>,
    /// DCB field: `specColor` (Class)
    #[serde(default)]
    pub spec_color: Option<Handle<SRGB8>>,
    /// DCB field: `glossiness` (Byte)
    #[serde(default)]
    pub glossiness: u32,
}

impl Pooled for TintEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ti.tint_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ti.tint_entry }
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
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `children` (Class (array))
    #[serde(default)]
    pub children: Vec<Handle<TintPalette>>,
    /// DCB field: `inheritsParams` (Boolean)
    #[serde(default)]
    pub inherits_params: bool,
    /// DCB field: `entryA` (Class)
    #[serde(default)]
    pub entry_a: Option<Handle<TintEntry>>,
    /// DCB field: `entryB` (Class)
    #[serde(default)]
    pub entry_b: Option<Handle<TintEntry>>,
    /// DCB field: `entryC` (Class)
    #[serde(default)]
    pub entry_c: Option<Handle<TintEntry>>,
    /// DCB field: `glassColor` (Class)
    #[serde(default)]
    pub glass_color: Option<Handle<SRGB8>>,
    /// DCB field: `decalTexture` (String)
    #[serde(default)]
    pub decal_texture: String,
    /// DCB field: `decalColorR` (Class)
    #[serde(default)]
    pub decal_color_r: Option<Handle<SRGB8>>,
    /// DCB field: `decalColorG` (Class)
    #[serde(default)]
    pub decal_color_g: Option<Handle<SRGB8>>,
    /// DCB field: `decalColorB` (Class)
    #[serde(default)]
    pub decal_color_b: Option<Handle<SRGB8>>,
}

impl Pooled for TintPalette {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ti.tint_palette }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ti.tint_palette }
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
    /// DCB field: `root` (Class)
    #[serde(default)]
    pub root: Option<Handle<TintPalette>>,
    /// DCB field: `templateTree` (Reference)
    #[serde(default)]
    pub template_tree: Option<CigGuid>,
}

impl Pooled for TintPaletteTree {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ti.tint_palette_tree }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ti.tint_palette_tree }
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

/// DCB type: `TintPaletteSwizzle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintPaletteSwizzle {
    /// DCB field: `primary` (Int32)
    #[serde(default)]
    pub primary: i32,
    /// DCB field: `secondary` (Int32)
    #[serde(default)]
    pub secondary: i32,
    /// DCB field: `tertiary` (Int32)
    #[serde(default)]
    pub tertiary: i32,
}

impl Pooled for TintPaletteSwizzle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ti.tint_palette_swizzle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ti.tint_palette_swizzle }
}

impl<'a> Extract<'a> for TintPaletteSwizzle {
    const TYPE_NAME: &'static str = "TintPaletteSwizzle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            primary: inst.get_i32("primary").unwrap_or_default(),
            secondary: inst.get_i32("secondary").unwrap_or_default(),
            tertiary: inst.get_i32("tertiary").unwrap_or_default(),
        }
    }
}

/// DCB type: `TintPaletteRef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintPaletteRef {
    /// DCB field: `RootRecord` (Reference)
    #[serde(default)]
    pub root_record: Option<CigGuid>,
    /// DCB field: `ChildPath` (String)
    #[serde(default)]
    pub child_path: String,
    /// DCB field: `SwizzleOverride` (StrongPointer)
    #[serde(default)]
    pub swizzle_override: Option<Handle<TintPaletteSwizzle>>,
}

impl Pooled for TintPaletteRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ti.tint_palette_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ti.tint_palette_ref }
}

impl<'a> Extract<'a> for TintPaletteRef {
    const TYPE_NAME: &'static str = "TintPaletteRef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            root_record: inst.get("RootRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            child_path: inst.get_str("ChildPath").map(String::from).unwrap_or_default(),
            swizzle_override: match inst.get("SwizzleOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintPaletteSwizzle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintPaletteSwizzle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

