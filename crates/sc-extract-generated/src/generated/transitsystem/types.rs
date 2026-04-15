// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `transitsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `TransitNavigationLink`
/// Inherits from: `NavigationLinkType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitNavigationLink {
    /// `linkValidForAgentType` (String)
    #[serde(default)]
    pub link_valid_for_agent_type: String,
    /// `costMultiplierSetup` (Class)
    #[serde(default)]
    pub cost_multiplier_setup: Option<Handle<NavigationLinkCostCustomization>>,
    /// `linkingType` (EnumChoice)
    #[serde(default)]
    pub linking_type: ENavigationLinkLinkingType,
    /// `useChannel` (WeakPointer)
    #[serde(default)]
    pub use_channel: Option<Handle<UsableUseChannelInstance>>,
}

impl Pooled for TransitNavigationLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_navigation_link }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_navigation_link }
}

impl<'a> Extract<'a> for TransitNavigationLink {
    const TYPE_NAME: &'static str = "TransitNavigationLink";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            link_valid_for_agent_type: inst.get_str("linkValidForAgentType").map(String::from).unwrap_or_default(),
            cost_multiplier_setup: match inst.get("costMultiplierSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<NavigationLinkCostCustomization>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            linking_type: ENavigationLinkLinkingType::from_dcb_str(inst.get_str("linkingType").unwrap_or("")),
            use_channel: match inst.get("useChannel") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UsableUseChannelInstance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HospitalEmergencyScreenComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HospitalEmergencyScreenComponentParams {
    /// `openDoorsInteraction` (String)
    #[serde(default)]
    pub open_doors_interaction: String,
    /// `closeDoorsInteraction` (String)
    #[serde(default)]
    pub close_doors_interaction: String,
}

impl Pooled for HospitalEmergencyScreenComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.hospital_emergency_screen_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.hospital_emergency_screen_component_params }
}

impl<'a> Extract<'a> for HospitalEmergencyScreenComponentParams {
    const TYPE_NAME: &'static str = "HospitalEmergencyScreenComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            open_doors_interaction: inst.get_str("openDoorsInteraction").map(String::from).unwrap_or_default(),
            close_doors_interaction: inst.get_str("closeDoorsInteraction").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SecurityClearanceGiverComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityClearanceGiverComponentParams {
    /// `securityTokens` (Reference (array))
    #[serde(default)]
    pub security_tokens: Vec<CigGuid>,
    /// `grantWhenInsideHostedZone` (Boolean)
    #[serde(default)]
    pub grant_when_inside_hosted_zone: bool,
    /// `grantWhenInsideLinkedArea` (Boolean)
    #[serde(default)]
    pub grant_when_inside_linked_area: bool,
}

impl Pooled for SecurityClearanceGiverComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.security_clearance_giver_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.security_clearance_giver_component_params }
}

impl<'a> Extract<'a> for SecurityClearanceGiverComponentParams {
    const TYPE_NAME: &'static str = "SecurityClearanceGiverComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            security_tokens: inst.get_array("securityTokens")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            grant_when_inside_hosted_zone: inst.get_bool("grantWhenInsideHostedZone").unwrap_or_default(),
            grant_when_inside_linked_area: inst.get_bool("grantWhenInsideLinkedArea").unwrap_or_default(),
        }
    }
}

/// DCB type: `SBindingTriggerGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBindingTriggerGameplayTrigger {
    /// `triggerName` (String)
    #[serde(default)]
    pub trigger_name: String,
}

impl Pooled for SBindingTriggerGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.sbinding_trigger_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.sbinding_trigger_gameplay_trigger }
}

impl<'a> Extract<'a> for SBindingTriggerGameplayTrigger {
    const TYPE_NAME: &'static str = "SBindingTriggerGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger_name: inst.get_str("triggerName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitNodeDialogueContext`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitNodeDialogueContext {
    /// `triggerName` (String)
    #[serde(default)]
    pub trigger_name: String,
    /// `dialogueContext` (Reference)
    #[serde(default)]
    pub dialogue_context: Option<CigGuid>,
}

impl Pooled for TransitNodeDialogueContext {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_node_dialogue_context }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_node_dialogue_context }
}

impl<'a> Extract<'a> for TransitNodeDialogueContext {
    const TYPE_NAME: &'static str = "TransitNodeDialogueContext";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger_name: inst.get_str("triggerName").map(String::from).unwrap_or_default(),
            dialogue_context: inst.get("dialogueContext").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `TransitCarriageAudio`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitCarriageAudio {
    /// `startTriggerID` (Class)
    #[serde(default)]
    pub start_trigger_id: Option<Handle<GlobalResourceAudio>>,
    /// `startTriggerIDOneShot` (Class)
    #[serde(default)]
    pub start_trigger_idone_shot: Option<Handle<GlobalResourceAudio>>,
    /// `stoppingTriggerID` (Class)
    #[serde(default)]
    pub stopping_trigger_id: Option<Handle<GlobalResourceAudio>>,
    /// `maxStoppingTime` (Single)
    #[serde(default)]
    pub max_stopping_time: f32,
    /// `stopTriggerID` (Class)
    #[serde(default)]
    pub stop_trigger_id: Option<Handle<GlobalResourceAudio>>,
    /// `stopTriggerIDOneShot` (Class)
    #[serde(default)]
    pub stop_trigger_idone_shot: Option<Handle<GlobalResourceAudio>>,
    /// `speedRTPC` (Class)
    #[serde(default)]
    pub speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `turnRTPC` (Class)
    #[serde(default)]
    pub turn_rtpc: Option<Handle<AudioRtpc>>,
    /// `turnRTPCScaler` (Single)
    #[serde(default)]
    pub turn_rtpcscaler: f32,
    /// `dialogueContexts` (Class (array))
    #[serde(default)]
    pub dialogue_contexts: Vec<Handle<TransitNodeDialogueContext>>,
}

impl Pooled for TransitCarriageAudio {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_carriage_audio }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_carriage_audio }
}

impl<'a> Extract<'a> for TransitCarriageAudio {
    const TYPE_NAME: &'static str = "TransitCarriageAudio";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_trigger_id: match inst.get("startTriggerID") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_trigger_idone_shot: match inst.get("startTriggerIDOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stopping_trigger_id: match inst.get("stoppingTriggerID") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_stopping_time: inst.get_f32("maxStoppingTime").unwrap_or_default(),
            stop_trigger_id: match inst.get("stopTriggerID") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_trigger_idone_shot: match inst.get("stopTriggerIDOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            speed_rtpc: match inst.get("speedRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            turn_rtpc: match inst.get("turnRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            turn_rtpcscaler: inst.get_f32("turnRTPCScaler").unwrap_or_default(),
            dialogue_contexts: inst.get_array("dialogueContexts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TransitNodeDialogueContext>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TransitNodeDialogueContext>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitCarriageEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitCarriageEffects {
    /// `inTransitTag` (Reference)
    #[serde(default)]
    pub in_transit_tag: Option<CigGuid>,
}

impl Pooled for TransitCarriageEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_carriage_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_carriage_effects }
}

impl<'a> Extract<'a> for TransitCarriageEffects {
    const TYPE_NAME: &'static str = "TransitCarriageEffects";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            in_transit_tag: inst.get("inTransitTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `TransitCarriageParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitCarriageParams {
    /// `defaultInteriorOC` (Class)
    #[serde(default)]
    pub default_interior_oc: Option<Handle<GlobalResourceObjectContainer>>,
    /// `audio` (Class)
    #[serde(default)]
    pub audio: Option<Handle<TransitCarriageAudio>>,
    /// `effects` (Class)
    #[serde(default)]
    pub effects: Option<Handle<TransitCarriageEffects>>,
    /// `hasExternalAnimations` (Boolean)
    #[serde(default)]
    pub has_external_animations: bool,
    /// `animationLength` (Single)
    #[serde(default)]
    pub animation_length: f32,
    /// `fragment` (String)
    #[serde(default)]
    pub fragment: String,
    /// `fragmentArrived` (String)
    #[serde(default)]
    pub fragment_arrived: String,
    /// `fragmentArriving` (String)
    #[serde(default)]
    pub fragment_arriving: String,
    /// `fragmentDeparting` (String)
    #[serde(default)]
    pub fragment_departing: String,
    /// `fragmentDeparted` (String)
    #[serde(default)]
    pub fragment_departed: String,
    /// `causesCertainDeath` (Boolean)
    #[serde(default)]
    pub causes_certain_death: bool,
    /// `collisionDamage` (Single)
    #[serde(default)]
    pub collision_damage: f32,
    /// `matchSplineOrientation` (Boolean)
    #[serde(default)]
    pub match_spline_orientation: bool,
}

impl Pooled for TransitCarriageParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_carriage_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_carriage_params }
}

impl<'a> Extract<'a> for TransitCarriageParams {
    const TYPE_NAME: &'static str = "TransitCarriageParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_interior_oc: match inst.get("defaultInteriorOC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceObjectContainer>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio: match inst.get("audio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TransitCarriageAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            effects: match inst.get("effects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TransitCarriageEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            has_external_animations: inst.get_bool("hasExternalAnimations").unwrap_or_default(),
            animation_length: inst.get_f32("animationLength").unwrap_or_default(),
            fragment: inst.get_str("fragment").map(String::from).unwrap_or_default(),
            fragment_arrived: inst.get_str("fragmentArrived").map(String::from).unwrap_or_default(),
            fragment_arriving: inst.get_str("fragmentArriving").map(String::from).unwrap_or_default(),
            fragment_departing: inst.get_str("fragmentDeparting").map(String::from).unwrap_or_default(),
            fragment_departed: inst.get_str("fragmentDeparted").map(String::from).unwrap_or_default(),
            causes_certain_death: inst.get_bool("causesCertainDeath").unwrap_or_default(),
            collision_damage: inst.get_f32("collisionDamage").unwrap_or_default(),
            match_spline_orientation: inst.get_bool("matchSplineOrientation").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitLimboNodeParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitLimboNodeParams {
}

impl Pooled for TransitLimboNodeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_limbo_node_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_limbo_node_params }
}

impl<'a> Extract<'a> for TransitLimboNodeParams {
    const TYPE_NAME: &'static str = "TransitLimboNodeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TransitGatewayParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitGatewayParams {
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `safeTeleportOffset` (Class)
    #[serde(default)]
    pub safe_teleport_offset: Option<Handle<Vec3>>,
}

impl Pooled for TransitGatewayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_gateway_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_gateway_params }
}

impl<'a> Extract<'a> for TransitGatewayParams {
    const TYPE_NAME: &'static str = "TransitGatewayParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            radius: inst.get_f32("radius").unwrap_or_default(),
            safe_teleport_offset: match inst.get("safeTeleportOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TransitInteractionPanelParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitInteractionPanelParams {
}

impl Pooled for TransitInteractionPanelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_interaction_panel_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_interaction_panel_params }
}

impl<'a> Extract<'a> for TransitInteractionPanelParams {
    const TYPE_NAME: &'static str = "TransitInteractionPanelParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TransitDestinationParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitDestinationParams {
    /// `Name` (Locale)
    #[serde(default)]
    pub name: LocaleKey,
    /// `importantLocation` (Boolean)
    #[serde(default)]
    pub important_location: bool,
    /// `enabledByDefault` (Boolean)
    #[serde(default)]
    pub enabled_by_default: bool,
    /// `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `unlockedManufacturers` (Reference (array))
    #[serde(default)]
    pub unlocked_manufacturers: Vec<CigGuid>,
}

impl Pooled for TransitDestinationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_destination_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_destination_params }
}

impl<'a> Extract<'a> for TransitDestinationParams {
    const TYPE_NAME: &'static str = "TransitDestinationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(LocaleKey::from).unwrap_or_default(),
            important_location: inst.get_bool("importantLocation").unwrap_or_default(),
            enabled_by_default: inst.get_bool("enabledByDefault").unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            unlocked_manufacturers: inst.get_array("unlockedManufacturers")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitDynamicDestinationParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitDynamicDestinationParams {
    /// `Name` (Locale)
    #[serde(default)]
    pub name: LocaleKey,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
}

impl Pooled for TransitDynamicDestinationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_dynamic_destination_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_dynamic_destination_params }
}

impl<'a> Extract<'a> for TransitDynamicDestinationParams {
    const TYPE_NAME: &'static str = "TransitDynamicDestinationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(LocaleKey::from).unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitManagerParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitManagerParams {
    /// `carriageType` (Reference)
    #[serde(default)]
    pub carriage_type: Option<CigGuid>,
    /// `carriageDisplayTypeId` (Locale)
    #[serde(default)]
    pub carriage_display_type_id: LocaleKey,
    /// `carriageInterior` (Class)
    #[serde(default)]
    pub carriage_interior: Option<Handle<GlobalResourceObjectContainer>>,
    /// `tagFilter` (Class)
    #[serde(default)]
    pub tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `carriageWaitTime` (Single)
    #[serde(default)]
    pub carriage_wait_time: f32,
    /// `carriageDoorTimeout` (Single)
    #[serde(default)]
    pub carriage_door_timeout: f32,
    /// `carriageAcceleration` (Single)
    #[serde(default)]
    pub carriage_acceleration: f32,
    /// `idealTimeBetweenArrivals` (Single)
    #[serde(default)]
    pub ideal_time_between_arrivals: f32,
    /// `pauseDistance` (Single)
    #[serde(default)]
    pub pause_distance: f32,
    /// `forceStreamableTransit` (Boolean)
    #[serde(default)]
    pub force_streamable_transit: bool,
    /// `forceCarriageAttachToParent` (Boolean)
    #[serde(default)]
    pub force_carriage_attach_to_parent: bool,
    /// `onByDefault` (Boolean)
    #[serde(default)]
    pub on_by_default: bool,
    /// `enabledForAI` (Boolean)
    #[serde(default)]
    pub enabled_for_ai: bool,
    /// `sortDestinationsByName` (Boolean)
    #[serde(default)]
    pub sort_destinations_by_name: bool,
    /// `sortDestinationsByPriority` (Boolean)
    #[serde(default)]
    pub sort_destinations_by_priority: bool,
    /// `ignoreDoorProximity` (Boolean)
    #[serde(default)]
    pub ignore_door_proximity: bool,
    /// `carriageDoorOpenByDefault` (Boolean)
    #[serde(default)]
    pub carriage_door_open_by_default: bool,
    /// `doorFindDistance` (Single)
    #[serde(default)]
    pub door_find_distance: f32,
    /// `automateTransit` (Boolean)
    #[serde(default)]
    pub automate_transit: bool,
    /// `reverseDisplayIndexOrder` (Boolean)
    #[serde(default)]
    pub reverse_display_index_order: bool,
    /// `turnOnInteractionName` (String)
    #[serde(default)]
    pub turn_on_interaction_name: String,
    /// `turnOffInteractionName` (String)
    #[serde(default)]
    pub turn_off_interaction_name: String,
    /// `displayIndexOffset` (SByte)
    #[serde(default)]
    pub display_index_offset: i32,
    /// `updateDestinationsForDistantTrains` (Boolean)
    #[serde(default)]
    pub update_destinations_for_distant_trains: bool,
    /// `persistDestinationEnabledState` (Boolean)
    #[serde(default)]
    pub persist_destination_enabled_state: bool,
    /// `allowOneDestination` (Boolean)
    #[serde(default)]
    pub allow_one_destination: bool,
    /// `manufacturer` (Reference)
    #[serde(default)]
    pub manufacturer: Option<CigGuid>,
}

impl Pooled for TransitManagerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.transitsystem.transit_manager_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.transitsystem.transit_manager_params }
}

impl<'a> Extract<'a> for TransitManagerParams {
    const TYPE_NAME: &'static str = "TransitManagerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            carriage_type: inst.get("carriageType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            carriage_display_type_id: inst.get_str("carriageDisplayTypeId").map(LocaleKey::from).unwrap_or_default(),
            carriage_interior: match inst.get("carriageInterior") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceObjectContainer>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tag_filter: match inst.get("tagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            carriage_wait_time: inst.get_f32("carriageWaitTime").unwrap_or_default(),
            carriage_door_timeout: inst.get_f32("carriageDoorTimeout").unwrap_or_default(),
            carriage_acceleration: inst.get_f32("carriageAcceleration").unwrap_or_default(),
            ideal_time_between_arrivals: inst.get_f32("idealTimeBetweenArrivals").unwrap_or_default(),
            pause_distance: inst.get_f32("pauseDistance").unwrap_or_default(),
            force_streamable_transit: inst.get_bool("forceStreamableTransit").unwrap_or_default(),
            force_carriage_attach_to_parent: inst.get_bool("forceCarriageAttachToParent").unwrap_or_default(),
            on_by_default: inst.get_bool("onByDefault").unwrap_or_default(),
            enabled_for_ai: inst.get_bool("enabledForAI").unwrap_or_default(),
            sort_destinations_by_name: inst.get_bool("sortDestinationsByName").unwrap_or_default(),
            sort_destinations_by_priority: inst.get_bool("sortDestinationsByPriority").unwrap_or_default(),
            ignore_door_proximity: inst.get_bool("ignoreDoorProximity").unwrap_or_default(),
            carriage_door_open_by_default: inst.get_bool("carriageDoorOpenByDefault").unwrap_or_default(),
            door_find_distance: inst.get_f32("doorFindDistance").unwrap_or_default(),
            automate_transit: inst.get_bool("automateTransit").unwrap_or_default(),
            reverse_display_index_order: inst.get_bool("reverseDisplayIndexOrder").unwrap_or_default(),
            turn_on_interaction_name: inst.get_str("turnOnInteractionName").map(String::from).unwrap_or_default(),
            turn_off_interaction_name: inst.get_str("turnOffInteractionName").map(String::from).unwrap_or_default(),
            display_index_offset: inst.get_i32("displayIndexOffset").unwrap_or_default(),
            update_destinations_for_distant_trains: inst.get_bool("updateDestinationsForDistantTrains").unwrap_or_default(),
            persist_destination_enabled_state: inst.get_bool("persistDestinationEnabledState").unwrap_or_default(),
            allow_one_destination: inst.get_bool("allowOneDestination").unwrap_or_default(),
            manufacturer: inst.get("manufacturer").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

