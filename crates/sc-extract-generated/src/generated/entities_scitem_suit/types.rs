// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-suit`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `LegacyMobiGlasParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyMobiGlasParams {
    /// `baseXOffset` (Single)
    #[serde(default)]
    pub base_xoffset: f32,
    /// `baseXOffsetMultiplier` (Single)
    #[serde(default)]
    pub base_xoffset_multiplier: f32,
    /// `baseYOffset` (Single)
    #[serde(default)]
    pub base_yoffset: f32,
    /// `baseYOffsetMultiplier` (Single)
    #[serde(default)]
    pub base_yoffset_multiplier: f32,
    /// `baseXAngleMultiplier` (Single)
    #[serde(default)]
    pub base_xangle_multiplier: f32,
    /// `baseYAngleMultiplier` (Single)
    #[serde(default)]
    pub base_yangle_multiplier: f32,
}

impl Pooled for LegacyMobiGlasParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.legacy_mobi_glas_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.legacy_mobi_glas_params }
}

impl<'a> Extract<'a> for LegacyMobiGlasParams {
    const TYPE_NAME: &'static str = "LegacyMobiGlasParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            base_xoffset: inst.get_f32("baseXOffset").unwrap_or_default(),
            base_xoffset_multiplier: inst.get_f32("baseXOffsetMultiplier").unwrap_or_default(),
            base_yoffset: inst.get_f32("baseYOffset").unwrap_or_default(),
            base_yoffset_multiplier: inst.get_f32("baseYOffsetMultiplier").unwrap_or_default(),
            base_xangle_multiplier: inst.get_f32("baseXAngleMultiplier").unwrap_or_default(),
            base_yangle_multiplier: inst.get_f32("baseYAngleMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingAreaServicesParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingAreaServicesParams {
}

impl Pooled for LandingAreaServicesParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.landing_area_services_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.landing_area_services_params }
}

impl<'a> Extract<'a> for LandingAreaServicesParams {
    const TYPE_NAME: &'static str = "LandingAreaServicesParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityComponentDockingHubParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentDockingHubParams {
}

impl Pooled for EntityComponentDockingHubParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.entity_component_docking_hub_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.entity_component_docking_hub_params }
}

impl<'a> Extract<'a> for EntityComponentDockingHubParams {
    const TYPE_NAME: &'static str = "EntityComponentDockingHubParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMissionBrokerInterfaceComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMissionBrokerInterfaceComponentParams {
}

impl Pooled for SMissionBrokerInterfaceComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.smission_broker_interface_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.smission_broker_interface_component_params }
}

impl<'a> Extract<'a> for SMissionBrokerInterfaceComponentParams {
    const TYPE_NAME: &'static str = "SMissionBrokerInterfaceComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MobiGlasPositionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasPositionParams {
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `scale` (Single)
    #[serde(default)]
    pub scale: f32,
    /// `angle` (Single)
    #[serde(default)]
    pub angle: f32,
    /// `tilt` (Single)
    #[serde(default)]
    pub tilt: f32,
}

impl Pooled for MobiGlasPositionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.mobi_glas_position_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.mobi_glas_position_params }
}

impl<'a> Extract<'a> for MobiGlasPositionParams {
    const TYPE_NAME: &'static str = "MobiGlasPositionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            scale: inst.get_f32("scale").unwrap_or_default(),
            angle: inst.get_f32("angle").unwrap_or_default(),
            tilt: inst.get_f32("tilt").unwrap_or_default(),
        }
    }
}

/// DCB type: `MobiGlasClientParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasClientParams {
    /// `armPositionParams` (Class)
    #[serde(default)]
    pub arm_position_params: Option<Handle<MobiGlasPositionParams>>,
    /// `headPositionParams` (Class)
    #[serde(default)]
    pub head_position_params: Option<Handle<MobiGlasPositionParams>>,
    /// `procBreathingSetup` (Reference)
    #[serde(default)]
    pub proc_breathing_setup: Option<CigGuid>,
}

impl Pooled for MobiGlasClientParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.mobi_glas_client_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.mobi_glas_client_params }
}

impl<'a> Extract<'a> for MobiGlasClientParams {
    const TYPE_NAME: &'static str = "MobiGlasClientParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            arm_position_params: match inst.get("armPositionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasPositionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            head_position_params: match inst.get("headPositionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasPositionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            proc_breathing_setup: inst.get("procBreathingSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MobiGlasRemoteParams`
/// Inherits from: `MobiGlasClientParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasRemoteParams {
    /// `armPositionParams` (Class)
    #[serde(default)]
    pub arm_position_params: Option<Handle<MobiGlasPositionParams>>,
    /// `headPositionParams` (Class)
    #[serde(default)]
    pub head_position_params: Option<Handle<MobiGlasPositionParams>>,
    /// `procBreathingSetup` (Reference)
    #[serde(default)]
    pub proc_breathing_setup: Option<CigGuid>,
    /// `materialGlow` (Single)
    #[serde(default)]
    pub material_glow: f32,
    /// `objModel` (Class)
    #[serde(default)]
    pub obj_model: Option<Handle<GlobalResourceGeometry>>,
}

impl Pooled for MobiGlasRemoteParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.mobi_glas_remote_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.mobi_glas_remote_params }
}

impl<'a> Extract<'a> for MobiGlasRemoteParams {
    const TYPE_NAME: &'static str = "MobiGlasRemoteParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            arm_position_params: match inst.get("armPositionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasPositionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            head_position_params: match inst.get("headPositionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasPositionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            proc_breathing_setup: inst.get("procBreathingSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            material_glow: inst.get_f32("materialGlow").unwrap_or_default(),
            obj_model: match inst.get("objModel") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MobiGlasEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasEffectParams {
    /// `projectionFadeInDuration` (Single)
    #[serde(default)]
    pub projection_fade_in_duration: f32,
    /// `projectionFadeOutDuration` (Single)
    #[serde(default)]
    pub projection_fade_out_duration: f32,
}

impl Pooled for MobiGlasEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.mobi_glas_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.mobi_glas_effect_params }
}

impl<'a> Extract<'a> for MobiGlasEffectParams {
    const TYPE_NAME: &'static str = "MobiGlasEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            projection_fade_in_duration: inst.get_f32("projectionFadeInDuration").unwrap_or_default(),
            projection_fade_out_duration: inst.get_f32("projectionFadeOutDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemMobiGlasParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemMobiGlasParams {
    /// `clientParams` (Class)
    #[serde(default)]
    pub client_params: Option<Handle<MobiGlasClientParams>>,
    /// `remoteParams` (Class)
    #[serde(default)]
    pub remote_params: Option<Handle<MobiGlasRemoteParams>>,
    /// `effectSettings` (Class)
    #[serde(default)]
    pub effect_settings: Option<Handle<MobiGlasEffectParams>>,
}

impl Pooled for SCItemMobiGlasParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.scitem_mobi_glas_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.scitem_mobi_glas_params }
}

impl<'a> Extract<'a> for SCItemMobiGlasParams {
    const TYPE_NAME: &'static str = "SCItemMobiGlasParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            client_params: match inst.get("clientParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasClientParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            remote_params: match inst.get("remoteParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasRemoteParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            effect_settings: match inst.get("effectSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerAssetManagerProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAssetManagerProviderParams {
    /// `inventoryItemsPerPage` (UInt32)
    #[serde(default)]
    pub inventory_items_per_page: u32,
    /// `inventoryPaintGeoTag` (String)
    #[serde(default)]
    pub inventory_paint_geo_tag: String,
}

impl Pooled for PlayerAssetManagerProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.player_asset_manager_provider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.player_asset_manager_provider_params }
}

impl<'a> Extract<'a> for PlayerAssetManagerProviderParams {
    const TYPE_NAME: &'static str = "PlayerAssetManagerProviderParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            inventory_items_per_page: inst.get_u32("inventoryItemsPerPage").unwrap_or_default(),
            inventory_paint_geo_tag: inst.get_str("inventoryPaintGeoTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerTradeProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTradeProviderParams {
}

impl Pooled for PlayerTradeProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.player_trade_provider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.player_trade_provider_params }
}

impl<'a> Extract<'a> for PlayerTradeProviderParams {
    const TYPE_NAME: &'static str = "PlayerTradeProviderParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ReputationProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationProviderParams {
}

impl Pooled for ReputationProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_suit.reputation_provider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_suit.reputation_provider_params }
}

impl<'a> Extract<'a> for ReputationProviderParams {
    const TYPE_NAME: &'static str = "ReputationProviderParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

