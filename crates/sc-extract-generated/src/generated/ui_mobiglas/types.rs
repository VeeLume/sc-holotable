// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-mobiglas`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ARModeSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARModeSettings {
    /// `playerLabelOffsetX` (Single)
    #[serde(default)]
    pub player_label_offset_x: f32,
    /// `playerLabelOffsetY` (Single)
    #[serde(default)]
    pub player_label_offset_y: f32,
    /// `playerLabelOffsetZ` (Single)
    #[serde(default)]
    pub player_label_offset_z: f32,
    /// `starMapParams` (Reference)
    #[serde(default)]
    pub star_map_params: Option<CigGuid>,
}

impl Pooled for ARModeSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_mobiglas.armode_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_mobiglas.armode_settings }
}

impl<'a> Extract<'a> for ARModeSettings {
    const TYPE_NAME: &'static str = "ARModeSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            player_label_offset_x: inst.get_f32("playerLabelOffsetX").unwrap_or_default(),
            player_label_offset_y: inst.get_f32("playerLabelOffsetY").unwrap_or_default(),
            player_label_offset_z: inst.get_f32("playerLabelOffsetZ").unwrap_or_default(),
            star_map_params: inst.get("starMapParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `mobiGlasApp`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct mobiGlasApp {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `bindingsNamespace` (String)
    #[serde(default)]
    pub bindings_namespace: String,
    /// `appCanvas` (Reference)
    #[serde(default)]
    pub app_canvas: Option<CigGuid>,
    /// `homeCanvas` (Reference)
    #[serde(default)]
    pub home_canvas: Option<CigGuid>,
    /// `displayIcon` (String)
    #[serde(default)]
    pub display_icon: String,
    /// `displayIcon3d` (String)
    #[serde(default)]
    pub display_icon3d: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `appColor` (Class)
    #[serde(default)]
    pub app_color: Option<Handle<SRGBA8>>,
    /// `isHidden` (Boolean)
    #[serde(default)]
    pub is_hidden: bool,
    /// `defaultAppData` (Reference (array))
    #[serde(default)]
    pub default_app_data: Vec<CigGuid>,
    /// `appParams` (StrongPointer)
    #[serde(default)]
    pub app_params: Option<Handle<SMobiGlasAppParamsBase>>,
    /// `hostingApps` (Reference (array))
    #[serde(default)]
    pub hosting_apps: Vec<CigGuid>,
    /// `legacyApp` (Boolean)
    #[serde(default)]
    pub legacy_app: bool,
}

impl Pooled for mobiGlasApp {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_mobiglas.mobi_glas_app }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_mobiglas.mobi_glas_app }
}

impl<'a> Extract<'a> for mobiGlasApp {
    const TYPE_NAME: &'static str = "mobiGlasApp";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            bindings_namespace: inst.get_str("bindingsNamespace").map(String::from).unwrap_or_default(),
            app_canvas: inst.get("appCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            home_canvas: inst.get("homeCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_icon: inst.get_str("displayIcon").map(String::from).unwrap_or_default(),
            display_icon3d: inst.get_str("displayIcon3d").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            app_color: match inst.get("appColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_hidden: inst.get_bool("isHidden").unwrap_or_default(),
            default_app_data: inst.get_array("defaultAppData")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            app_params: match inst.get("appParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMobiGlasAppParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMobiGlasAppParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hosting_apps: inst.get_array("hostingApps")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            legacy_app: inst.get_bool("legacyApp").unwrap_or_default(),
        }
    }
}

/// DCB type: `MobiGlasAppData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasAppData {
    /// `appData` (StrongPointer)
    #[serde(default)]
    pub app_data: Option<Handle<MobiGlasAppDataBase>>,
}

impl Pooled for MobiGlasAppData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_mobiglas.mobi_glas_app_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_mobiglas.mobi_glas_app_data }
}

impl<'a> Extract<'a> for MobiGlasAppData {
    const TYPE_NAME: &'static str = "MobiGlasAppData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            app_data: match inst.get("appData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasAppDataBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MobiGlasAppDataBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MobiGlasAppDataBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasAppDataBase {
}

impl Pooled for MobiGlasAppDataBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_mobiglas.mobi_glas_app_data_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_mobiglas.mobi_glas_app_data_base }
}

impl<'a> Extract<'a> for MobiGlasAppDataBase {
    const TYPE_NAME: &'static str = "MobiGlasAppDataBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMobiGlasAppParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMobiGlasAppParamsBase {
}

impl Pooled for SMobiGlasAppParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_mobiglas.smobi_glas_app_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_mobiglas.smobi_glas_app_params_base }
}

impl<'a> Extract<'a> for SMobiGlasAppParamsBase {
    const TYPE_NAME: &'static str = "SMobiGlasAppParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

