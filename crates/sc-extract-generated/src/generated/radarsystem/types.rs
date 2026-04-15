// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `radarsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `RadarSystemGlobalParams`
pub struct RadarSystemGlobalParams {
    /// `paramsVersion` (UInt32)
    pub params_version: u32,
    /// `signatureSystemParams` (Class)
    pub signature_system_params: Option<Handle<SignatureSystemGlobalParams>>,
    /// `contactStateParams` (Class)
    pub contact_state_params: Option<Handle<ContactStateGlobalParams>>,
}

impl Pooled for RadarSystemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_system_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_system_global_params }
}

impl<'a> Extract<'a> for RadarSystemGlobalParams {
    const TYPE_NAME: &'static str = "RadarSystemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params_version: inst.get_u32("paramsVersion").unwrap_or_default(),
            signature_system_params: match inst.get("signatureSystemParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureSystemGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contact_state_params: match inst.get("contactStateParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactStateGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MasterModeSwitchDeltaSignatureTypes`
pub struct MasterModeSwitchDeltaSignatureTypes {
    /// `scmToNav` (Reference)
    pub scm_to_nav: Option<CigGuid>,
    /// `navToScm` (Reference)
    pub nav_to_scm: Option<CigGuid>,
}

impl Pooled for MasterModeSwitchDeltaSignatureTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.master_mode_switch_delta_signature_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.master_mode_switch_delta_signature_types }
}

impl<'a> Extract<'a> for MasterModeSwitchDeltaSignatureTypes {
    const TYPE_NAME: &'static str = "MasterModeSwitchDeltaSignatureTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scm_to_nav: inst.get("scmToNav").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            nav_to_scm: inst.get("navToScm").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SignatureSystemGlobalParams`
pub struct SignatureSystemGlobalParams {
    /// `globalDBFactor` (Single)
    pub global_dbfactor: f32,
    /// `globalAmbientIRFactor` (Single)
    pub global_ambient_irfactor: f32,
    /// `dBSignaturePeakHoldTime` (Single)
    pub d_bsignature_peak_hold_time: f32,
    /// `signatureUIParams` (Class)
    pub signature_uiparams: Option<Handle<SignatureUIGlobalParams>>,
    /// `actorMultiplierParams` (Class)
    pub actor_multiplier_params: Option<Handle<ActorSignatureMultiplierGlobalParams>>,
    /// `crossSectionParams` (Class)
    pub cross_section_params: Option<Handle<CrossSectionGlobalParams>>,
    /// `signatureTypeParams` (Class)
    pub signature_type_params: Option<Handle<SignatureTypeGlobalParams>>,
    /// `masterModeDeltaSignatureType` (Class)
    pub master_mode_delta_signature_type: Option<Handle<MasterModeSwitchDeltaSignatureTypes>>,
    /// `scanWaveTriggeredDeltaSignatureType` (Reference)
    pub scan_wave_triggered_delta_signature_type: Option<CigGuid>,
}

impl Pooled for SignatureSystemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.signature_system_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.signature_system_global_params }
}

impl<'a> Extract<'a> for SignatureSystemGlobalParams {
    const TYPE_NAME: &'static str = "SignatureSystemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_dbfactor: inst.get_f32("globalDBFactor").unwrap_or_default(),
            global_ambient_irfactor: inst.get_f32("globalAmbientIRFactor").unwrap_or_default(),
            d_bsignature_peak_hold_time: inst.get_f32("dBSignaturePeakHoldTime").unwrap_or_default(),
            signature_uiparams: match inst.get("signatureUIParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureUIGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            actor_multiplier_params: match inst.get("actorMultiplierParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorSignatureMultiplierGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cross_section_params: match inst.get("crossSectionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CrossSectionGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            signature_type_params: match inst.get("signatureTypeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureTypeGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            master_mode_delta_signature_type: match inst.get("masterModeDeltaSignatureType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MasterModeSwitchDeltaSignatureTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            scan_wave_triggered_delta_signature_type: inst.get("scanWaveTriggeredDeltaSignatureType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContactStateGlobalParams`
pub struct ContactStateGlobalParams {
    /// `contactStateIcons` (String)
    pub contact_state_icons: String,
}

impl Pooled for ContactStateGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.contact_state_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.contact_state_global_params }
}

impl<'a> Extract<'a> for ContactStateGlobalParams {
    const TYPE_NAME: &'static str = "ContactStateGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            contact_state_icons: inst.get_str("contactStateIcons").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SignatureUIGlobalParams`
pub struct SignatureUIGlobalParams {
    /// `warningUnderStateTime` (Single)
    pub warning_under_state_time: f32,
    /// `signatureDisplayTime` (Single)
    pub signature_display_time: f32,
    /// `signatureFadeTime` (Single)
    pub signature_fade_time: f32,
    /// `emissionDisplayIncrease` (Single)
    pub emission_display_increase: f32,
    /// `emissionMemoryTime` (Single)
    pub emission_memory_time: f32,
}

impl Pooled for SignatureUIGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.signature_uiglobal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.signature_uiglobal_params }
}

impl<'a> Extract<'a> for SignatureUIGlobalParams {
    const TYPE_NAME: &'static str = "SignatureUIGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            warning_under_state_time: inst.get_f32("warningUnderStateTime").unwrap_or_default(),
            signature_display_time: inst.get_f32("signatureDisplayTime").unwrap_or_default(),
            signature_fade_time: inst.get_f32("signatureFadeTime").unwrap_or_default(),
            emission_display_increase: inst.get_f32("emissionDisplayIncrease").unwrap_or_default(),
            emission_memory_time: inst.get_f32("emissionMemoryTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorSignatureMultiplierGlobalParams`
pub struct ActorSignatureMultiplierGlobalParams {
    /// `bodyTemperatureToIRMultiplier` (Single)
    pub body_temperature_to_irmultiplier: f32,
    /// `staminaToIRMultiplier` (Single)
    pub stamina_to_irmultiplier: f32,
    /// `staminaToIRDecayDelay` (Single)
    pub stamina_to_irdecay_delay: f32,
    /// `staminaToIRDecayRate` (Single)
    pub stamina_to_irdecay_rate: f32,
}

impl Pooled for ActorSignatureMultiplierGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.actor_signature_multiplier_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.actor_signature_multiplier_global_params }
}

impl<'a> Extract<'a> for ActorSignatureMultiplierGlobalParams {
    const TYPE_NAME: &'static str = "ActorSignatureMultiplierGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            body_temperature_to_irmultiplier: inst.get_f32("bodyTemperatureToIRMultiplier").unwrap_or_default(),
            stamina_to_irmultiplier: inst.get_f32("staminaToIRMultiplier").unwrap_or_default(),
            stamina_to_irdecay_delay: inst.get_f32("staminaToIRDecayDelay").unwrap_or_default(),
            stamina_to_irdecay_rate: inst.get_f32("staminaToIRDecayRate").unwrap_or_default(),
        }
    }
}

/// DCB type: `CrossSectionGlobalParams`
pub struct CrossSectionGlobalParams {
    /// `globalCSFactor` (Class)
    pub global_csfactor: Option<Handle<Vec3>>,
    /// `maxDistance` (Single)
    pub max_distance: f32,
    /// `lineOfSightAngle` (Single)
    pub line_of_sight_angle: f32,
}

impl Pooled for CrossSectionGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.cross_section_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.cross_section_global_params }
}

impl<'a> Extract<'a> for CrossSectionGlobalParams {
    const TYPE_NAME: &'static str = "CrossSectionGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_csfactor: match inst.get("globalCSFactor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            line_of_sight_angle: inst.get_f32("lineOfSightAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `SignatureTypeGlobalParams`
pub struct SignatureTypeGlobalParams {
    /// `displayName` (Locale)
    pub display_name: LocaleKey,
    /// `allowDampening` (Boolean)
    pub allow_dampening: bool,
    /// `allowGenerateUnknownContacts` (Boolean)
    pub allow_generate_unknown_contacts: bool,
    /// `allowVisibleContacts` (Boolean)
    pub allow_visible_contacts: bool,
    /// `allowGenerateBlobs` (Boolean)
    pub allow_generate_blobs: bool,
    /// `nearbyAmbientMultiplier` (Single)
    pub nearby_ambient_multiplier: f32,
}

impl Pooled for SignatureTypeGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.signature_type_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.signature_type_global_params }
}

impl<'a> Extract<'a> for SignatureTypeGlobalParams {
    const TYPE_NAME: &'static str = "SignatureTypeGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(LocaleKey::from).unwrap_or_default(),
            allow_dampening: inst.get_bool("allowDampening").unwrap_or_default(),
            allow_generate_unknown_contacts: inst.get_bool("allowGenerateUnknownContacts").unwrap_or_default(),
            allow_visible_contacts: inst.get_bool("allowVisibleContacts").unwrap_or_default(),
            allow_generate_blobs: inst.get_bool("allowGenerateBlobs").unwrap_or_default(),
            nearby_ambient_multiplier: inst.get_f32("nearbyAmbientMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanInformationTable`
pub struct ScanInformationTable {
    /// `defs` (Reference (array))
    pub defs: Vec<CigGuid>,
}

impl Pooled for ScanInformationTable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_information_table }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_information_table }
}

impl<'a> Extract<'a> for ScanInformationTable {
    const TYPE_NAME: &'static str = "ScanInformationTable";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            defs: inst.get_array("defs")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanDisplayConditionCompareParams`
/// Inherits from: `ScanDisplayConditionBaseParams`
pub struct ScanDisplayConditionCompareParams {
    /// `scanCategory` (EnumChoice)
    pub scan_category: EScanCategory,
    /// `scanInformation` (EnumChoice)
    pub scan_information: EScanInformation,
    /// `thresholdValue` (Single)
    pub threshold_value: f32,
    /// `comparison` (EnumChoice)
    pub comparison: RadarPriorityComparison,
    /// `displayValue` (Locale)
    pub display_value: LocaleKey,
}

impl Pooled for ScanDisplayConditionCompareParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_display_condition_compare_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_display_condition_compare_params }
}

impl<'a> Extract<'a> for ScanDisplayConditionCompareParams {
    const TYPE_NAME: &'static str = "ScanDisplayConditionCompareParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scan_category: EScanCategory::from_dcb_str(inst.get_str("scanCategory").unwrap_or("")),
            scan_information: EScanInformation::from_dcb_str(inst.get_str("scanInformation").unwrap_or("")),
            threshold_value: inst.get_f32("thresholdValue").unwrap_or_default(),
            comparison: RadarPriorityComparison::from_dcb_str(inst.get_str("comparison").unwrap_or("")),
            display_value: inst.get_str("displayValue").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanDisplayConditionVariableParams`
/// Inherits from: `ScanDisplayVariableParams`
pub struct ScanDisplayConditionVariableParams {
    /// `truncateSize` (Byte)
    pub truncate_size: u32,
    /// `displayIsHidden` (Boolean)
    pub display_is_hidden: bool,
    /// `displayInBrackets` (Boolean)
    pub display_in_brackets: bool,
    /// `suffixArrayIndex` (Boolean)
    pub suffix_array_index: bool,
    /// `suffixSemiColon` (Boolean)
    pub suffix_semi_colon: bool,
    /// `auxiliaryFlag` (EnumChoice)
    pub auxiliary_flag: EScanDisplayVariableAuxiliaryType,
    /// `fallback` (StrongPointer)
    pub fallback: Option<ScanDisplayVariableParamsPtr>,
    /// `defaultValue` (Locale)
    pub default_value: LocaleKey,
    /// `conditionValues` (StrongPointer (array))
    pub condition_values: Vec<ScanDisplayConditionBaseParamsPtr>,
}

impl Pooled for ScanDisplayConditionVariableParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_display_condition_variable_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_display_condition_variable_params }
}

impl<'a> Extract<'a> for ScanDisplayConditionVariableParams {
    const TYPE_NAME: &'static str = "ScanDisplayConditionVariableParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            truncate_size: inst.get_u32("truncateSize").unwrap_or_default(),
            display_is_hidden: inst.get_bool("displayIsHidden").unwrap_or_default(),
            display_in_brackets: inst.get_bool("displayInBrackets").unwrap_or_default(),
            suffix_array_index: inst.get_bool("suffixArrayIndex").unwrap_or_default(),
            suffix_semi_colon: inst.get_bool("suffixSemiColon").unwrap_or_default(),
            auxiliary_flag: EScanDisplayVariableAuxiliaryType::from_dcb_str(inst.get_str("auxiliaryFlag").unwrap_or("")),
            fallback: match inst.get("fallback") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ScanDisplayVariableParamsPtr::from_ref(b, r)),
                _ => None,
            },
            default_value: inst.get_str("defaultValue").map(LocaleKey::from).unwrap_or_default(),
            condition_values: inst.get_array("conditionValues")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(ScanDisplayConditionBaseParamsPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarSignatureCategoryDefinition`
pub struct RadarSignatureCategoryDefinition {
    /// `default` (Reference)
    pub default: Option<CigGuid>,
    /// `types` (Reference (array))
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarSignatureCategoryDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_signature_category_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_signature_category_definition }
}

impl<'a> Extract<'a> for RadarSignatureCategoryDefinition {
    const TYPE_NAME: &'static str = "RadarSignatureCategoryDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default: inst.get("default").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarSignatureCategoryEntry`
pub struct RadarSignatureCategoryEntry {
    /// `name` (String)
    pub name: String,
    /// `displayName` (Locale)
    pub display_name: LocaleKey,
}

impl Pooled for RadarSignatureCategoryEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_signature_category_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_signature_category_entry }
}

impl<'a> Extract<'a> for RadarSignatureCategoryEntry {
    const TYPE_NAME: &'static str = "RadarSignatureCategoryEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactTypeDefinition`
pub struct RadarContactTypeDefinition {
    /// `unknownType` (Reference)
    pub unknown_type: Option<CigGuid>,
    /// `defaultAudioType` (Reference)
    pub default_audio_type: Option<CigGuid>,
    /// `types` (Reference (array))
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarContactTypeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_contact_type_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_contact_type_definition }
}

impl<'a> Extract<'a> for RadarContactTypeDefinition {
    const TYPE_NAME: &'static str = "RadarContactTypeDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            unknown_type: inst.get("unknownType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_audio_type: inst.get("defaultAudioType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactGroupDefinition`
pub struct RadarContactGroupDefinition {
    /// `groups` (Reference (array))
    pub groups: Vec<CigGuid>,
}

impl Pooled for RadarContactGroupDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_contact_group_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_contact_group_definition }
}

impl<'a> Extract<'a> for RadarContactGroupDefinition {
    const TYPE_NAME: &'static str = "RadarContactGroupDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            groups: inst.get_array("groups")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDeltaSignatureDefinition`
pub struct RadarDeltaSignatureDefinition {
    /// `types` (Reference (array))
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarDeltaSignatureDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_delta_signature_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_delta_signature_definition }
}

impl<'a> Extract<'a> for RadarDeltaSignatureDefinition {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

