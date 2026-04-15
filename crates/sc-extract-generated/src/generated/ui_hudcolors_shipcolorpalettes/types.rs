// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-hudcolors_shipcolorpalettes`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `HudColors`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudColors {
    /// `HoloMatParams` (Class)
    #[serde(default)]
    pub holo_mat_params: Option<Handle<HudColor_HoloParam>>,
    /// `Palettes` (Class (array))
    #[serde(default)]
    pub palettes: Vec<Handle<HudColor_Palette>>,
}

impl Pooled for HudColors {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_hudcolors_shipcolorpalettes.hud_colors }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_hudcolors_shipcolorpalettes.hud_colors }
}

impl<'a> Extract<'a> for HudColors {
    const TYPE_NAME: &'static str = "HudColors";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            holo_mat_params: match inst.get("HoloMatParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HudColor_HoloParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            palettes: inst.get_array("Palettes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HudColor_Palette>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HudColor_Palette>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HudColor_Palette`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudColor_Palette {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `StandardEntries` (Class)
    #[serde(default)]
    pub standard_entries: Option<Handle<HudColor_Entry>>,
    /// `CustomEntries` (Class (array))
    #[serde(default)]
    pub custom_entries: Vec<Handle<HudColor_CustomEntry>>,
}

impl Pooled for HudColor_Palette {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_hudcolors_shipcolorpalettes.hud_color_palette }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_hudcolors_shipcolorpalettes.hud_color_palette }
}

impl<'a> Extract<'a> for HudColor_Palette {
    const TYPE_NAME: &'static str = "HudColor_Palette";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            standard_entries: match inst.get("StandardEntries") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HudColor_Entry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            custom_entries: inst.get_array("CustomEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HudColor_CustomEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<HudColor_CustomEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HudColor_Entry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudColor_Entry {
    /// `FlashColor` (StrongPointer)
    #[serde(default)]
    pub flash_color: Option<Handle<SRGBA8>>,
    /// `HoloMatColors` (Class)
    #[serde(default)]
    pub holo_mat_colors: Option<Handle<HudColor_HoloMatColors>>,
}

impl Pooled for HudColor_Entry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_hudcolors_shipcolorpalettes.hud_color_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_hudcolors_shipcolorpalettes.hud_color_entry }
}

impl<'a> Extract<'a> for HudColor_Entry {
    const TYPE_NAME: &'static str = "HudColor_Entry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flash_color: match inst.get("FlashColor") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            holo_mat_colors: match inst.get("HoloMatColors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HudColor_HoloMatColors>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HudColor_CustomEntry`
/// Inherits from: `HudColor_Entry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudColor_CustomEntry {
    /// `FlashColor` (StrongPointer)
    #[serde(default)]
    pub flash_color: Option<Handle<SRGBA8>>,
    /// `HoloMatColors` (Class)
    #[serde(default)]
    pub holo_mat_colors: Option<Handle<HudColor_HoloMatColors>>,
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for HudColor_CustomEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_hudcolors_shipcolorpalettes.hud_color_custom_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_hudcolors_shipcolorpalettes.hud_color_custom_entry }
}

impl<'a> Extract<'a> for HudColor_CustomEntry {
    const TYPE_NAME: &'static str = "HudColor_CustomEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flash_color: match inst.get("FlashColor") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            holo_mat_colors: match inst.get("HoloMatColors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HudColor_HoloMatColors>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HudColor_HoloParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudColor_HoloParam {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `Opacity` (Single)
    #[serde(default)]
    pub opacity: f32,
    /// `Glow` (Single)
    #[serde(default)]
    pub glow: f32,
    /// `DiffuseOpacity` (Single)
    #[serde(default)]
    pub diffuse_opacity: f32,
    /// `RimOpacity` (Single)
    #[serde(default)]
    pub rim_opacity: f32,
    /// `SilhouetteOpacity` (Single)
    #[serde(default)]
    pub silhouette_opacity: f32,
}

impl Pooled for HudColor_HoloParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_hudcolors_shipcolorpalettes.hud_color_holo_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_hudcolors_shipcolorpalettes.hud_color_holo_param }
}

impl<'a> Extract<'a> for HudColor_HoloParam {
    const TYPE_NAME: &'static str = "HudColor_HoloParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            opacity: inst.get_f32("Opacity").unwrap_or_default(),
            glow: inst.get_f32("Glow").unwrap_or_default(),
            diffuse_opacity: inst.get_f32("DiffuseOpacity").unwrap_or_default(),
            rim_opacity: inst.get_f32("RimOpacity").unwrap_or_default(),
            silhouette_opacity: inst.get_f32("SilhouetteOpacity").unwrap_or_default(),
        }
    }
}

/// DCB type: `HudColor_HoloMatColors`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudColor_HoloMatColors {
    /// `Diffuse` (Class)
    #[serde(default)]
    pub diffuse: Option<Handle<SRGB8>>,
    /// `Emissive` (Class)
    #[serde(default)]
    pub emissive: Option<Handle<SRGB8>>,
    /// `RimColor` (Class)
    #[serde(default)]
    pub rim_color: Option<Handle<SRGB8>>,
    /// `SilhouetteColor` (Class)
    #[serde(default)]
    pub silhouette_color: Option<Handle<SRGB8>>,
    /// `Textures` (StrongPointer)
    #[serde(default)]
    pub textures: Option<Handle<HudColor_HoloMatTextures>>,
}

impl Pooled for HudColor_HoloMatColors {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_hudcolors_shipcolorpalettes.hud_color_holo_mat_colors }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_hudcolors_shipcolorpalettes.hud_color_holo_mat_colors }
}

impl<'a> Extract<'a> for HudColor_HoloMatColors {
    const TYPE_NAME: &'static str = "HudColor_HoloMatColors";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            diffuse: match inst.get("Diffuse") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            emissive: match inst.get("Emissive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rim_color: match inst.get("RimColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            silhouette_color: match inst.get("SilhouetteColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            textures: match inst.get("Textures") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HudColor_HoloMatTextures>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HudColor_HoloMatTextures`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudColor_HoloMatTextures {
    /// `DiffuseName` (String)
    #[serde(default)]
    pub diffuse_name: String,
}

impl Pooled for HudColor_HoloMatTextures {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_hudcolors_shipcolorpalettes.hud_color_holo_mat_textures }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_hudcolors_shipcolorpalettes.hud_color_holo_mat_textures }
}

impl<'a> Extract<'a> for HudColor_HoloMatTextures {
    const TYPE_NAME: &'static str = "HudColor_HoloMatTextures";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            diffuse_name: inst.get_str("DiffuseName").map(String::from).unwrap_or_default(),
        }
    }
}

