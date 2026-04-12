// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `lawsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LawsystemPools {
    #[serde(default)]
    pub shostility_rules: Vec<Option<SHostilityRules>>,
    #[serde(default)]
    pub infraction_parameters: Vec<Option<InfractionParameters>>,
    #[serde(default)]
    pub infraction_definition: Vec<Option<InfractionDefinition>>,
    #[serde(default)]
    pub infraction: Vec<Option<Infraction>>,
    #[serde(default)]
    pub impounding_definition: Vec<Option<ImpoundingDefinition>>,
    #[serde(default)]
    pub controlled_substance_class: Vec<Option<ControlledSubstanceClass>>,
    #[serde(default)]
    pub jurisdiction: Vec<Option<Jurisdiction>>,
    #[serde(default)]
    pub security_network_permissions: Vec<Option<SecurityNetworkPermissions>>,
    #[serde(default)]
    pub security_clearance_token_data: Vec<Option<SecurityClearanceTokenData>>,
    #[serde(default)]
    pub security_manual_input: Vec<Option<SecurityManualInput>>,
    #[serde(default)]
    pub security_notifications: Vec<Option<SecurityNotifications>>,
    #[serde(default)]
    pub security_clearance_token: Vec<Option<SecurityClearanceToken>>,
    #[serde(default)]
    pub security_clearance_conditions: Vec<Option<SecurityClearanceConditions>>,
    #[serde(default)]
    pub security_network_protocol: Vec<Option<SecurityNetworkProtocol>>,
    #[serde(default)]
    pub security_network_room_settings: Vec<Option<SecurityNetworkRoomSettings>>,
    #[serde(default)]
    pub security_network_protocol_override: Vec<Option<SecurityNetworkProtocolOverride>>,
    #[serde(default)]
    pub security_network_manifest: Vec<Option<SecurityNetworkManifest>>,
    #[serde(default)]
    pub security_network_variable_value_base: Vec<Option<SecurityNetworkVariableValue_Base>>,
    #[serde(default)]
    pub security_network_variable: Vec<Option<SecurityNetworkVariable>>,
    #[serde(default)]
    pub security_network_variable_effect_base: Vec<Option<SecurityNetworkVariableEffect_Base>>,
    #[serde(default)]
    pub security_network_variable_effects: Vec<Option<SecurityNetworkVariableEffects>>,
    #[serde(default)]
    pub mobi_glas_mission_note: Vec<Option<MobiGlasMissionNote>>,
}
