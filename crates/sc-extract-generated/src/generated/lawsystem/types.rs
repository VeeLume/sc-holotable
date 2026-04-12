// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `lawsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SHostilityRules`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHostilityRules {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `useReputationHostility` (Boolean)
    #[serde(default)]
    pub use_reputation_hostility: bool,
    /// `usePersonalHostility` (Boolean)
    #[serde(default)]
    pub use_personal_hostility: bool,
    /// `useFactionHostility` (Boolean)
    #[serde(default)]
    pub use_faction_hostility: bool,
    /// `policesLawfulTrespass` (Boolean)
    #[serde(default)]
    pub polices_lawful_trespass: bool,
    /// `hostileToCriminals` (Boolean)
    #[serde(default)]
    pub hostile_to_criminals: bool,
    /// `hostileToStolenVehicles` (Boolean)
    #[serde(default)]
    pub hostile_to_stolen_vehicles: bool,
    /// `factionOverride` (Reference)
    #[serde(default)]
    pub faction_override: Option<CigGuid>,
    /// `factionToOverride` (Reference)
    #[serde(default)]
    pub faction_to_override: Option<CigGuid>,
}

impl Pooled for SHostilityRules {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.shostility_rules }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.shostility_rules }
}

impl<'a> Extract<'a> for SHostilityRules {
    const TYPE_NAME: &'static str = "SHostilityRules";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            use_reputation_hostility: inst.get_bool("useReputationHostility").unwrap_or_default(),
            use_personal_hostility: inst.get_bool("usePersonalHostility").unwrap_or_default(),
            use_faction_hostility: inst.get_bool("useFactionHostility").unwrap_or_default(),
            polices_lawful_trespass: inst.get_bool("policesLawfulTrespass").unwrap_or_default(),
            hostile_to_criminals: inst.get_bool("hostileToCriminals").unwrap_or_default(),
            hostile_to_stolen_vehicles: inst.get_bool("hostileToStolenVehicles").unwrap_or_default(),
            faction_override: inst.get("factionOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            faction_to_override: inst.get("factionToOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InfractionParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfractionParameters {
    /// `isFelony` (Int32)
    #[serde(default)]
    pub is_felony: i32,
    /// `graceAllowance` (Int32)
    #[serde(default)]
    pub grace_allowance: i32,
    /// `graceAllowanceCooldown` (Single)
    #[serde(default)]
    pub grace_allowance_cooldown: f32,
    /// `gracePeriod` (Single)
    #[serde(default)]
    pub grace_period: f32,
    /// `graceCooloffScale` (Single)
    #[serde(default)]
    pub grace_cooloff_scale: f32,
    /// `graceWarnings` (Locale (array))
    #[serde(default)]
    pub grace_warnings: Vec<String>,
    /// `displayGraceTime` (Int32)
    #[serde(default)]
    pub display_grace_time: i32,
    /// `escalatedPaymentFineMultiplier` (Single)
    #[serde(default)]
    pub escalated_payment_fine_multiplier: f32,
    /// `earlyPaymentPeriod` (Single)
    #[serde(default)]
    pub early_payment_period: f32,
    /// `lifetime` (Single)
    #[serde(default)]
    pub lifetime: f32,
    /// `coolOffTime` (Single)
    #[serde(default)]
    pub cool_off_time: f32,
    /// `pressChargesNotificationTime` (Single)
    #[serde(default)]
    pub press_charges_notification_time: f32,
    /// `removeTimeSeconds` (Single)
    #[serde(default)]
    pub remove_time_seconds: f32,
    /// `felonyMerits` (Int32)
    #[serde(default)]
    pub felony_merits: i32,
    /// `ignoreIfAgainstPartyMember` (Int32)
    #[serde(default)]
    pub ignore_if_against_party_member: i32,
    /// `hideCrimeNotification` (Int32)
    #[serde(default)]
    pub hide_crime_notification: i32,
    /// `hideCrimeInJournal` (Int32)
    #[serde(default)]
    pub hide_crime_in_journal: i32,
}

impl Pooled for InfractionParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.infraction_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.infraction_parameters }
}

impl<'a> Extract<'a> for InfractionParameters {
    const TYPE_NAME: &'static str = "InfractionParameters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_felony: inst.get_i32("isFelony").unwrap_or_default(),
            grace_allowance: inst.get_i32("graceAllowance").unwrap_or_default(),
            grace_allowance_cooldown: inst.get_f32("graceAllowanceCooldown").unwrap_or_default(),
            grace_period: inst.get_f32("gracePeriod").unwrap_or_default(),
            grace_cooloff_scale: inst.get_f32("graceCooloffScale").unwrap_or_default(),
            grace_warnings: inst.get_array("graceWarnings")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            display_grace_time: inst.get_i32("displayGraceTime").unwrap_or_default(),
            escalated_payment_fine_multiplier: inst.get_f32("escalatedPaymentFineMultiplier").unwrap_or_default(),
            early_payment_period: inst.get_f32("earlyPaymentPeriod").unwrap_or_default(),
            lifetime: inst.get_f32("lifetime").unwrap_or_default(),
            cool_off_time: inst.get_f32("coolOffTime").unwrap_or_default(),
            press_charges_notification_time: inst.get_f32("pressChargesNotificationTime").unwrap_or_default(),
            remove_time_seconds: inst.get_f32("removeTimeSeconds").unwrap_or_default(),
            felony_merits: inst.get_i32("felonyMerits").unwrap_or_default(),
            ignore_if_against_party_member: inst.get_i32("ignoreIfAgainstPartyMember").unwrap_or_default(),
            hide_crime_notification: inst.get_i32("hideCrimeNotification").unwrap_or_default(),
            hide_crime_in_journal: inst.get_i32("hideCrimeInJournal").unwrap_or_default(),
        }
    }
}

/// DCB type: `InfractionDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfractionDefinition {
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `triggers` (EnumChoice (array))
    #[serde(default)]
    pub triggers: Vec<String>,
    /// `defaultParameters` (Class)
    #[serde(default)]
    pub default_parameters: Option<Handle<InfractionParameters>>,
}

impl Pooled for InfractionDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.infraction_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.infraction_definition }
}

impl<'a> Extract<'a> for InfractionDefinition {
    const TYPE_NAME: &'static str = "InfractionDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            triggers: inst.get_array("triggers")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            default_parameters: match inst.get("defaultParameters") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InfractionParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InfractionParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Infraction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infraction {
    /// `definition` (Reference)
    #[serde(default)]
    pub definition: Option<CigGuid>,
    /// `parameterOverrides` (Class)
    #[serde(default)]
    pub parameter_overrides: Option<Handle<InfractionParameters>>,
}

impl Pooled for Infraction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.infraction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.infraction }
}

impl<'a> Extract<'a> for Infraction {
    const TYPE_NAME: &'static str = "Infraction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            definition: inst.get("definition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            parameter_overrides: match inst.get("parameterOverrides") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InfractionParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InfractionParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ImpoundingDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpoundingDefinition {
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `trigger` (EnumChoice)
    #[serde(default)]
    pub trigger: String,
    /// `impoundingTimeSeconds` (Single)
    #[serde(default)]
    pub impounding_time_seconds: f32,
    /// `impoundingFineUEC` (Int32)
    #[serde(default)]
    pub impounding_fine_uec: i32,
    /// `ignoreIfAgainstPartyMember` (Boolean)
    #[serde(default)]
    pub ignore_if_against_party_member: bool,
}

impl Pooled for ImpoundingDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.impounding_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.impounding_definition }
}

impl<'a> Extract<'a> for ImpoundingDefinition {
    const TYPE_NAME: &'static str = "ImpoundingDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            trigger: inst.get_str("trigger").map(String::from).unwrap_or_default(),
            impounding_time_seconds: inst.get_f32("impoundingTimeSeconds").unwrap_or_default(),
            impounding_fine_uec: inst.get_i32("impoundingFineUEC").unwrap_or_default(),
            ignore_if_against_party_member: inst.get_bool("ignoreIfAgainstPartyMember").unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlledSubstanceClass`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledSubstanceClass {
    /// `commodities` (Reference (array))
    #[serde(default)]
    pub commodities: Vec<CigGuid>,
    /// `resources` (Reference (array))
    #[serde(default)]
    pub resources: Vec<CigGuid>,
    /// `maxPossessionSCU` (Single)
    #[serde(default)]
    pub max_possession_scu: f32,
}

impl Pooled for ControlledSubstanceClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.controlled_substance_class }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.controlled_substance_class }
}

impl<'a> Extract<'a> for ControlledSubstanceClass {
    const TYPE_NAME: &'static str = "ControlledSubstanceClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            commodities: inst.get_array("commodities")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            resources: inst.get_array("resources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            max_possession_scu: inst.get_f32("maxPossessionSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `Jurisdiction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// `subsumptionJurisdiction` (String)
    #[serde(default)]
    pub subsumption_jurisdiction: String,
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `logoPath` (String)
    #[serde(default)]
    pub logo_path: String,
    /// `parentJurisdiction` (Reference)
    #[serde(default)]
    pub parent_jurisdiction: Option<CigGuid>,
    /// `respectsParentJurisdictionLaws` (Boolean)
    #[serde(default)]
    pub respects_parent_jurisdiction_laws: bool,
    /// `infractions` (Class (array))
    #[serde(default)]
    pub infractions: Vec<Handle<Infraction>>,
    /// `infractionSets` (Reference (array))
    #[serde(default)]
    pub infraction_sets: Vec<CigGuid>,
    /// `journalEntry` (Reference)
    #[serde(default)]
    pub journal_entry: Option<CigGuid>,
    /// `maxStolenGoodsPossessionSCU` (Single)
    #[serde(default)]
    pub max_stolen_goods_possession_scu: f32,
    /// `prohibitedGoods` (Reference (array))
    #[serde(default)]
    pub prohibited_goods: Vec<CigGuid>,
    /// `prohibitedResources` (Reference (array))
    #[serde(default)]
    pub prohibited_resources: Vec<CigGuid>,
    /// `controlledSubstanceClasses` (Class)
    #[serde(default)]
    pub controlled_substance_classes: Option<Handle<ControlledSubstanceClass>>,
    /// `baseFine` (Int32)
    #[serde(default)]
    pub base_fine: i32,
    /// `earlyPaymentPeriod` (Single)
    #[serde(default)]
    pub early_payment_period: f32,
    /// `isPrison` (Boolean)
    #[serde(default)]
    pub is_prison: bool,
    /// `impoundingDefinitions` (Class (array))
    #[serde(default)]
    pub impounding_definitions: Vec<Handle<ImpoundingDefinition>>,
}

impl Pooled for Jurisdiction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.jurisdiction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.jurisdiction }
}

impl<'a> Extract<'a> for Jurisdiction {
    const TYPE_NAME: &'static str = "Jurisdiction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            subsumption_jurisdiction: inst.get_str("subsumptionJurisdiction").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            logo_path: inst.get_str("logoPath").map(String::from).unwrap_or_default(),
            parent_jurisdiction: inst.get("parentJurisdiction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            respects_parent_jurisdiction_laws: inst.get_bool("respectsParentJurisdictionLaws").unwrap_or_default(),
            infractions: inst.get_array("infractions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Infraction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Infraction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            infraction_sets: inst.get_array("infractionSets")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            journal_entry: inst.get("journalEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            max_stolen_goods_possession_scu: inst.get_f32("maxStolenGoodsPossessionSCU").unwrap_or_default(),
            prohibited_goods: inst.get_array("prohibitedGoods")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            prohibited_resources: inst.get_array("prohibitedResources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            controlled_substance_classes: match inst.get("controlledSubstanceClasses") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlledSubstanceClass>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlledSubstanceClass>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            base_fine: inst.get_i32("baseFine").unwrap_or_default(),
            early_payment_period: inst.get_f32("earlyPaymentPeriod").unwrap_or_default(),
            is_prison: inst.get_bool("isPrison").unwrap_or_default(),
            impounding_definitions: inst.get_array("impoundingDefinitions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ImpoundingDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ImpoundingDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SecurityNetworkPermissions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkPermissions {
    /// `access` (EnumChoice)
    #[serde(default)]
    pub access: String,
    /// `trespass` (EnumChoice)
    #[serde(default)]
    pub trespass: String,
}

impl Pooled for SecurityNetworkPermissions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_permissions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_permissions }
}

impl<'a> Extract<'a> for SecurityNetworkPermissions {
    const TYPE_NAME: &'static str = "SecurityNetworkPermissions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            access: inst.get_str("access").map(String::from).unwrap_or_default(),
            trespass: inst.get_str("trespass").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SecurityClearanceTokenData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityClearanceTokenData {
    /// `conditionFailedTag` (Reference)
    #[serde(default)]
    pub condition_failed_tag: Option<CigGuid>,
}

impl Pooled for SecurityClearanceTokenData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_clearance_token_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_clearance_token_data }
}

impl<'a> Extract<'a> for SecurityClearanceTokenData {
    const TYPE_NAME: &'static str = "SecurityClearanceTokenData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            condition_failed_tag: inst.get("conditionFailedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SecurityManualInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityManualInput {
}

impl Pooled for SecurityManualInput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_manual_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_manual_input }
}

impl<'a> Extract<'a> for SecurityManualInput {
    const TYPE_NAME: &'static str = "SecurityManualInput";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityNotifications`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNotifications {
}

impl Pooled for SecurityNotifications {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_notifications }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_notifications }
}

impl<'a> Extract<'a> for SecurityNotifications {
    const TYPE_NAME: &'static str = "SecurityNotifications";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityClearanceToken`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityClearanceToken {
    /// `customCondition` (StrongPointer)
    #[serde(default)]
    pub custom_condition: Option<Handle<SecurityClearanceTokenData>>,
    /// `manualInput` (StrongPointer)
    #[serde(default)]
    pub manual_input: Option<Handle<SecurityManualInput>>,
    /// `missionNote` (StrongPointer)
    #[serde(default)]
    pub mission_note: Option<Handle<MobiGlasMissionNote>>,
    /// `notifications` (StrongPointer)
    #[serde(default)]
    pub notifications: Option<Handle<SecurityNotifications>>,
}

impl Pooled for SecurityClearanceToken {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_clearance_token }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_clearance_token }
}

impl<'a> Extract<'a> for SecurityClearanceToken {
    const TYPE_NAME: &'static str = "SecurityClearanceToken";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            custom_condition: match inst.get("customCondition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityClearanceTokenData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityClearanceTokenData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            manual_input: match inst.get("manualInput") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityManualInput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityManualInput>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mission_note: match inst.get("missionNote") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasMissionNote>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MobiGlasMissionNote>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notifications: match inst.get("notifications") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNotifications>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNotifications>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SecurityClearanceConditions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityClearanceConditions {
    /// `tokens` (Reference (array))
    #[serde(default)]
    pub tokens: Vec<CigGuid>,
}

impl Pooled for SecurityClearanceConditions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_clearance_conditions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_clearance_conditions }
}

impl<'a> Extract<'a> for SecurityClearanceConditions {
    const TYPE_NAME: &'static str = "SecurityClearanceConditions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tokens: inst.get_array("tokens")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SecurityNetworkProtocol`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkProtocol {
    /// `permissionsWithClearance` (Class)
    #[serde(default)]
    pub permissions_with_clearance: Option<Handle<SecurityNetworkPermissions>>,
    /// `permissionsWithoutClearance` (Class)
    #[serde(default)]
    pub permissions_without_clearance: Option<Handle<SecurityNetworkPermissions>>,
    /// `acceptedClearance` (Class)
    #[serde(default)]
    pub accepted_clearance: Option<Handle<SecurityClearanceConditions>>,
    /// `ownerFaction` (Reference)
    #[serde(default)]
    pub owner_faction: Option<CigGuid>,
}

impl Pooled for SecurityNetworkProtocol {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_protocol }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_protocol }
}

impl<'a> Extract<'a> for SecurityNetworkProtocol {
    const TYPE_NAME: &'static str = "SecurityNetworkProtocol";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            permissions_with_clearance: match inst.get("permissionsWithClearance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkPermissions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNetworkPermissions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            permissions_without_clearance: match inst.get("permissionsWithoutClearance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkPermissions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNetworkPermissions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            accepted_clearance: match inst.get("acceptedClearance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityClearanceConditions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityClearanceConditions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            owner_faction: inst.get("ownerFaction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SecurityNetworkRoomSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkRoomSettings {
    /// `defaultProtocol` (Class)
    #[serde(default)]
    pub default_protocol: Option<Handle<SecurityNetworkProtocol>>,
    /// `roomIdentifier` (Reference)
    #[serde(default)]
    pub room_identifier: Option<CigGuid>,
    /// `trespassWarningSeconds` (Single)
    #[serde(default)]
    pub trespass_warning_seconds: f32,
    /// `trespassRevokeWarningSeconds` (Single)
    #[serde(default)]
    pub trespass_revoke_warning_seconds: f32,
}

impl Pooled for SecurityNetworkRoomSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_room_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_room_settings }
}

impl<'a> Extract<'a> for SecurityNetworkRoomSettings {
    const TYPE_NAME: &'static str = "SecurityNetworkRoomSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_protocol: match inst.get("defaultProtocol") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkProtocol>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNetworkProtocol>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_identifier: inst.get("roomIdentifier").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trespass_warning_seconds: inst.get_f32("trespassWarningSeconds").unwrap_or_default(),
            trespass_revoke_warning_seconds: inst.get_f32("trespassRevokeWarningSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `SecurityNetworkProtocolOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkProtocolOverride {
    /// `roomIdentifiers` (Class)
    #[serde(default)]
    pub room_identifiers: Option<Handle<TagList>>,
    /// `protocol` (Class)
    #[serde(default)]
    pub protocol: Option<Handle<SecurityNetworkProtocol>>,
}

impl Pooled for SecurityNetworkProtocolOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_protocol_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_protocol_override }
}

impl<'a> Extract<'a> for SecurityNetworkProtocolOverride {
    const TYPE_NAME: &'static str = "SecurityNetworkProtocolOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            room_identifiers: match inst.get("roomIdentifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            protocol: match inst.get("protocol") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkProtocol>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNetworkProtocol>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SecurityNetworkManifest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkManifest {
    /// `defaultProtocol` (Class)
    #[serde(default)]
    pub default_protocol: Option<Handle<SecurityNetworkProtocol>>,
    /// `roomProtocols` (Class (array))
    #[serde(default)]
    pub room_protocols: Vec<Handle<SecurityNetworkProtocolOverride>>,
    /// `variables` (Class (array))
    #[serde(default)]
    pub variables: Vec<Handle<SecurityNetworkVariable>>,
    /// `hostilityRules` (Class (array))
    #[serde(default)]
    pub hostility_rules: Vec<Handle<SHostilityRules>>,
    /// `variableEffects` (Class (array))
    #[serde(default)]
    pub variable_effects: Vec<Handle<SecurityNetworkVariableEffects>>,
    /// `isNeutralTerritory` (Boolean)
    #[serde(default)]
    pub is_neutral_territory: bool,
    /// `teleportPlayerOut` (Boolean)
    #[serde(default)]
    pub teleport_player_out: bool,
}

impl Pooled for SecurityNetworkManifest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_manifest }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_manifest }
}

impl<'a> Extract<'a> for SecurityNetworkManifest {
    const TYPE_NAME: &'static str = "SecurityNetworkManifest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_protocol: match inst.get("defaultProtocol") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkProtocol>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNetworkProtocol>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_protocols: inst.get_array("roomProtocols")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SecurityNetworkProtocolOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SecurityNetworkProtocolOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            variables: inst.get_array("variables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SecurityNetworkVariable>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SecurityNetworkVariable>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            hostility_rules: inst.get_array("hostilityRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHostilityRules>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHostilityRules>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            variable_effects: inst.get_array("variableEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SecurityNetworkVariableEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SecurityNetworkVariableEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            is_neutral_territory: inst.get_bool("isNeutralTerritory").unwrap_or_default(),
            teleport_player_out: inst.get_bool("teleportPlayerOut").unwrap_or_default(),
        }
    }
}

/// DCB type: `SecurityNetworkVariableValue_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkVariableValue_Base {
}

impl Pooled for SecurityNetworkVariableValue_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_variable_value_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_variable_value_base }
}

impl<'a> Extract<'a> for SecurityNetworkVariableValue_Base {
    const TYPE_NAME: &'static str = "SecurityNetworkVariableValue_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityNetworkVariable`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkVariable {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `value` (StrongPointer)
    #[serde(default)]
    pub value: Option<Handle<SecurityNetworkVariableValue_Base>>,
}

impl Pooled for SecurityNetworkVariable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_variable }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_variable }
}

impl<'a> Extract<'a> for SecurityNetworkVariable {
    const TYPE_NAME: &'static str = "SecurityNetworkVariable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkVariableValue_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNetworkVariableValue_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SecurityNetworkVariableEffect_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkVariableEffect_Base {
}

impl Pooled for SecurityNetworkVariableEffect_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_variable_effect_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_variable_effect_base }
}

impl<'a> Extract<'a> for SecurityNetworkVariableEffect_Base {
    const TYPE_NAME: &'static str = "SecurityNetworkVariableEffect_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityNetworkVariableEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkVariableEffects {
    /// `variableValue` (Class)
    #[serde(default)]
    pub variable_value: Option<Handle<SecurityNetworkVariable>>,
    /// `effects` (StrongPointer (array))
    #[serde(default)]
    pub effects: Vec<Handle<SecurityNetworkVariableEffect_Base>>,
}

impl Pooled for SecurityNetworkVariableEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_variable_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_variable_effects }
}

impl<'a> Extract<'a> for SecurityNetworkVariableEffects {
    const TYPE_NAME: &'static str = "SecurityNetworkVariableEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variable_value: match inst.get("variableValue") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkVariable>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SecurityNetworkVariable>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            effects: inst.get_array("effects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SecurityNetworkVariableEffect_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SecurityNetworkVariableEffect_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MobiGlasMissionNote`
/// Inherits from: `MobiGlasAppDataBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasMissionNote {
    /// `noteTitle` (Locale)
    #[serde(default)]
    pub note_title: String,
    /// `noteContent` (Locale (array))
    #[serde(default)]
    pub note_content: Vec<String>,
    /// `appLink` (Class)
    #[serde(default)]
    pub app_link: Option<Handle<SMobiGlasAppLink>>,
}

impl Pooled for MobiGlasMissionNote {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.mobi_glas_mission_note }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.mobi_glas_mission_note }
}

impl<'a> Extract<'a> for MobiGlasMissionNote {
    const TYPE_NAME: &'static str = "MobiGlasMissionNote";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            note_title: inst.get_str("noteTitle").map(String::from).unwrap_or_default(),
            note_content: inst.get_array("noteContent")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            app_link: match inst.get("appLink") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMobiGlasAppLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMobiGlasAppLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

