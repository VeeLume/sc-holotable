// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-loadingplatformmanager`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SCLoadingPlatformEntityReferences`
pub struct SCLoadingPlatformEntityReferences {
    /// `loadingGate` (String)
    pub loading_gate: String,
    /// `loadingPlatform` (String)
    pub loading_platform: String,
    /// `frontGate` (String)
    pub front_gate: String,
    /// `kiosk` (String)
    pub kiosk: String,
    /// `obstructionCheckBounds` (String)
    pub obstruction_check_bounds: String,
}

impl Pooled for SCLoadingPlatformEntityReferences {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_loadingplatformmanager.scloading_platform_entity_references }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_loadingplatformmanager.scloading_platform_entity_references }
}

impl<'a> Extract<'a> for SCLoadingPlatformEntityReferences {
    const TYPE_NAME: &'static str = "SCLoadingPlatformEntityReferences";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            loading_gate: inst.get_str("loadingGate").map(String::from).unwrap_or_default(),
            loading_platform: inst.get_str("loadingPlatform").map(String::from).unwrap_or_default(),
            front_gate: inst.get_str("frontGate").map(String::from).unwrap_or_default(),
            kiosk: inst.get_str("kiosk").map(String::from).unwrap_or_default(),
            obstruction_check_bounds: inst.get_str("obstructionCheckBounds").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCLoadingPlatformLightGroupParams`
pub struct SCLoadingPlatformLightGroupParams {
    /// `lightGroupEntityReference` (String)
    pub light_group_entity_reference: String,
    /// `platformIdleClosedLightState` (EnumChoice)
    pub platform_idle_closed_light_state: ELightState,
    /// `platformInTransitLightState` (EnumChoice)
    pub platform_in_transit_light_state: ELightState,
    /// `platformIdleOpenLightState` (EnumChoice)
    pub platform_idle_open_light_state: ELightState,
    /// `platformObstructedLightState` (EnumChoice)
    pub platform_obstructed_light_state: ELightState,
}

impl Pooled for SCLoadingPlatformLightGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_loadingplatformmanager.scloading_platform_light_group_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_loadingplatformmanager.scloading_platform_light_group_params }
}

impl<'a> Extract<'a> for SCLoadingPlatformLightGroupParams {
    const TYPE_NAME: &'static str = "SCLoadingPlatformLightGroupParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            light_group_entity_reference: inst.get_str("lightGroupEntityReference").map(String::from).unwrap_or_default(),
            platform_idle_closed_light_state: ELightState::from_dcb_str(inst.get_str("platformIdleClosedLightState").unwrap_or("")),
            platform_in_transit_light_state: ELightState::from_dcb_str(inst.get_str("platformInTransitLightState").unwrap_or("")),
            platform_idle_open_light_state: ELightState::from_dcb_str(inst.get_str("platformIdleOpenLightState").unwrap_or("")),
            platform_obstructed_light_state: ELightState::from_dcb_str(inst.get_str("platformObstructedLightState").unwrap_or("")),
        }
    }
}

/// DCB type: `SCLoadingPlatformEffectParams`
pub struct SCLoadingPlatformEffectParams {
    /// `raisingLoadingPlatformTrigger` (Reference (array))
    pub raising_loading_platform_trigger: Vec<CigGuid>,
    /// `openingLoadingGateTrigger` (Reference (array))
    pub opening_loading_gate_trigger: Vec<CigGuid>,
    /// `openingFrontGateTrigger` (Reference (array))
    pub opening_front_gate_trigger: Vec<CigGuid>,
    /// `loweringLoadingPlatformTrigger` (Reference (array))
    pub lowering_loading_platform_trigger: Vec<CigGuid>,
    /// `closingLoadingGateTrigger` (Reference (array))
    pub closing_loading_gate_trigger: Vec<CigGuid>,
    /// `closingFrontGateTrigger` (Reference (array))
    pub closing_front_gate_trigger: Vec<CigGuid>,
}

impl Pooled for SCLoadingPlatformEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_loadingplatformmanager.scloading_platform_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_loadingplatformmanager.scloading_platform_effect_params }
}

impl<'a> Extract<'a> for SCLoadingPlatformEffectParams {
    const TYPE_NAME: &'static str = "SCLoadingPlatformEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            raising_loading_platform_trigger: inst.get_array("raisingLoadingPlatformTrigger")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            opening_loading_gate_trigger: inst.get_array("openingLoadingGateTrigger")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            opening_front_gate_trigger: inst.get_array("openingFrontGateTrigger")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            lowering_loading_platform_trigger: inst.get_array("loweringLoadingPlatformTrigger")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            closing_loading_gate_trigger: inst.get_array("closingLoadingGateTrigger")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            closing_front_gate_trigger: inst.get_array("closingFrontGateTrigger")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCLoadingPlatformTrackviewParams`
pub struct SCLoadingPlatformTrackviewParams {
    /// `openingLoadingPlatformTrack` (String)
    pub opening_loading_platform_track: String,
    /// `closingLoadingPlatformTrack` (String)
    pub closing_loading_platform_track: String,
}

impl Pooled for SCLoadingPlatformTrackviewParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_loadingplatformmanager.scloading_platform_trackview_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_loadingplatformmanager.scloading_platform_trackview_params }
}

impl<'a> Extract<'a> for SCLoadingPlatformTrackviewParams {
    const TYPE_NAME: &'static str = "SCLoadingPlatformTrackviewParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            opening_loading_platform_track: inst.get_str("openingLoadingPlatformTrack").map(String::from).unwrap_or_default(),
            closing_loading_platform_track: inst.get_str("closingLoadingPlatformTrack").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCLoadingPlatformManagerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SCLoadingPlatformManagerParams {
    /// `loadingPlatformEntityReferences` (Class)
    pub loading_platform_entity_references: Option<Handle<SCLoadingPlatformEntityReferences>>,
    /// `lightGroupParams` (Class)
    pub light_group_params: Option<Handle<SCLoadingPlatformLightGroupParams>>,
    /// `effectParams` (Class)
    pub effect_params: Option<Handle<SCLoadingPlatformEffectParams>>,
    /// `trackviewParams` (Class)
    pub trackview_params: Option<Handle<SCLoadingPlatformTrackviewParams>>,
    /// `hasFrontGate` (Boolean)
    pub has_front_gate: bool,
    /// `hasLoadingGate` (Boolean)
    pub has_loading_gate: bool,
    /// `hasCargoGrid` (Boolean)
    pub has_cargo_grid: bool,
    /// `hasLandingArea` (Boolean)
    pub has_landing_area: bool,
}

impl Pooled for SCLoadingPlatformManagerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_loadingplatformmanager.scloading_platform_manager_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_loadingplatformmanager.scloading_platform_manager_params }
}

impl<'a> Extract<'a> for SCLoadingPlatformManagerParams {
    const TYPE_NAME: &'static str = "SCLoadingPlatformManagerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loading_platform_entity_references: match inst.get("loadingPlatformEntityReferences") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCLoadingPlatformEntityReferences>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_params: match inst.get("lightGroupParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCLoadingPlatformLightGroupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCLoadingPlatformEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            trackview_params: match inst.get("trackviewParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCLoadingPlatformTrackviewParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            has_front_gate: inst.get_bool("hasFrontGate").unwrap_or_default(),
            has_loading_gate: inst.get_bool("hasLoadingGate").unwrap_or_default(),
            has_cargo_grid: inst.get_bool("hasCargoGrid").unwrap_or_default(),
            has_landing_area: inst.get_bool("hasLandingArea").unwrap_or_default(),
        }
    }
}

