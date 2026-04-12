// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `scitemmanufacturer`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCItemLocalization`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemLocalization {
    /// `Name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `ShortName` (Locale)
    #[serde(default)]
    pub short_name: String,
    /// `Description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `displayFeatures` (Class)
    #[serde(default)]
    pub display_features: Option<Handle<SCExtendedLocalizationLevelParams>>,
}

impl Pooled for SCItemLocalization {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.scitemmanufacturer.scitem_localization }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.scitemmanufacturer.scitem_localization }
}

impl<'a> Extract<'a> for SCItemLocalization {
    const TYPE_NAME: &'static str = "SCItemLocalization";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            short_name: inst.get_str("ShortName").map(String::from).unwrap_or_default(),
            description: inst.get_str("Description").map(String::from).unwrap_or_default(),
            display_features: match inst.get("displayFeatures") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCExtendedLocalizationLevelParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCExtendedLocalizationLevelParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemManufacturer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemManufacturer {
    /// `Localization` (Class)
    #[serde(default)]
    pub localization: Option<Handle<SCItemLocalization>>,
    /// `Logo` (String)
    #[serde(default)]
    pub logo: String,
    /// `LogoFullColor` (String)
    #[serde(default)]
    pub logo_full_color: String,
    /// `LogoSimplifiedWhite` (String)
    #[serde(default)]
    pub logo_simplified_white: String,
    /// `Code` (String)
    #[serde(default)]
    pub code: String,
    /// `DashboardCanvasConfig` (Reference)
    #[serde(default)]
    pub dashboard_canvas_config: Option<CigGuid>,
    /// `BuildingBlocksStyle` (Reference)
    #[serde(default)]
    pub building_blocks_style: Option<CigGuid>,
    /// `AudioManufacturerTag` (Reference)
    #[serde(default)]
    pub audio_manufacturer_tag: Option<CigGuid>,
    /// `LightAmplification` (Reference)
    #[serde(default)]
    pub light_amplification: Option<CigGuid>,
}

impl Pooled for SCItemManufacturer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.scitemmanufacturer.scitem_manufacturer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.scitemmanufacturer.scitem_manufacturer }
}

impl<'a> Extract<'a> for SCItemManufacturer {
    const TYPE_NAME: &'static str = "SCItemManufacturer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            localization: match inst.get("Localization") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCItemLocalization>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCItemLocalization>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            logo: inst.get_str("Logo").map(String::from).unwrap_or_default(),
            logo_full_color: inst.get_str("LogoFullColor").map(String::from).unwrap_or_default(),
            logo_simplified_white: inst.get_str("LogoSimplifiedWhite").map(String::from).unwrap_or_default(),
            code: inst.get_str("Code").map(String::from).unwrap_or_default(),
            dashboard_canvas_config: inst.get("DashboardCanvasConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            building_blocks_style: inst.get("BuildingBlocksStyle").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_manufacturer_tag: inst.get("AudioManufacturerTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            light_amplification: inst.get("LightAmplification").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

