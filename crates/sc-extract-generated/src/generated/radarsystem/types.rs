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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `RadarSystemGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSystemGlobalParams {
    /// `paramsVersion` (UInt32)
    #[serde(default)]
    pub params_version: u32,
    /// `signatureSystemParams` (Class)
    #[serde(default)]
    pub signature_system_params: Option<Handle<SignatureSystemGlobalParams>>,
    /// `contactStateParams` (Class)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SignatureSystemGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contact_state_params: match inst.get("contactStateParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactStateGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactStateGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MasterModeSwitchDeltaSignatureTypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterModeSwitchDeltaSignatureTypes {
    /// `scmToNav` (Reference)
    #[serde(default)]
    pub scm_to_nav: Option<CigGuid>,
    /// `navToScm` (Reference)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureSystemGlobalParams {
    /// `globalDBFactor` (Single)
    #[serde(default)]
    pub global_dbfactor: f32,
    /// `globalAmbientIRFactor` (Single)
    #[serde(default)]
    pub global_ambient_irfactor: f32,
    /// `dBSignaturePeakHoldTime` (Single)
    #[serde(default)]
    pub d_bsignature_peak_hold_time: f32,
    /// `signatureUIParams` (Class)
    #[serde(default)]
    pub signature_uiparams: Option<Handle<SignatureUIGlobalParams>>,
    /// `actorMultiplierParams` (Class)
    #[serde(default)]
    pub actor_multiplier_params: Option<Handle<ActorSignatureMultiplierGlobalParams>>,
    /// `crossSectionParams` (Class)
    #[serde(default)]
    pub cross_section_params: Option<Handle<CrossSectionGlobalParams>>,
    /// `signatureTypeParams` (Class)
    #[serde(default)]
    pub signature_type_params: Option<Handle<SignatureTypeGlobalParams>>,
    /// `masterModeDeltaSignatureType` (Class)
    #[serde(default)]
    pub master_mode_delta_signature_type: Option<Handle<MasterModeSwitchDeltaSignatureTypes>>,
    /// `scanWaveTriggeredDeltaSignatureType` (Reference)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SignatureUIGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_multiplier_params: match inst.get("actorMultiplierParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorSignatureMultiplierGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorSignatureMultiplierGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cross_section_params: match inst.get("crossSectionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CrossSectionGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CrossSectionGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            signature_type_params: match inst.get("signatureTypeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureTypeGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SignatureTypeGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            master_mode_delta_signature_type: match inst.get("masterModeDeltaSignatureType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MasterModeSwitchDeltaSignatureTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MasterModeSwitchDeltaSignatureTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_wave_triggered_delta_signature_type: inst.get("scanWaveTriggeredDeltaSignatureType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContactStateGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactStateGlobalParams {
    /// `contactStateIcons` (String)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureUIGlobalParams {
    /// `warningUnderStateTime` (Single)
    #[serde(default)]
    pub warning_under_state_time: f32,
    /// `signatureDisplayTime` (Single)
    #[serde(default)]
    pub signature_display_time: f32,
    /// `signatureFadeTime` (Single)
    #[serde(default)]
    pub signature_fade_time: f32,
    /// `emissionDisplayIncrease` (Single)
    #[serde(default)]
    pub emission_display_increase: f32,
    /// `emissionMemoryTime` (Single)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSignatureMultiplierGlobalParams {
    /// `bodyTemperatureToIRMultiplier` (Single)
    #[serde(default)]
    pub body_temperature_to_irmultiplier: f32,
    /// `staminaToIRMultiplier` (Single)
    #[serde(default)]
    pub stamina_to_irmultiplier: f32,
    /// `staminaToIRDecayDelay` (Single)
    #[serde(default)]
    pub stamina_to_irdecay_delay: f32,
    /// `staminaToIRDecayRate` (Single)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSectionGlobalParams {
    /// `globalCSFactor` (Class)
    #[serde(default)]
    pub global_csfactor: Option<Handle<Vec3>>,
    /// `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// `lineOfSightAngle` (Single)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            line_of_sight_angle: inst.get_f32("lineOfSightAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `SignatureTypeGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureTypeGlobalParams {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `allowDampening` (Boolean)
    #[serde(default)]
    pub allow_dampening: bool,
    /// `allowGenerateUnknownContacts` (Boolean)
    #[serde(default)]
    pub allow_generate_unknown_contacts: bool,
    /// `allowVisibleContacts` (Boolean)
    #[serde(default)]
    pub allow_visible_contacts: bool,
    /// `allowGenerateBlobs` (Boolean)
    #[serde(default)]
    pub allow_generate_blobs: bool,
    /// `nearbyAmbientMultiplier` (Single)
    #[serde(default)]
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
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            allow_dampening: inst.get_bool("allowDampening").unwrap_or_default(),
            allow_generate_unknown_contacts: inst.get_bool("allowGenerateUnknownContacts").unwrap_or_default(),
            allow_visible_contacts: inst.get_bool("allowVisibleContacts").unwrap_or_default(),
            allow_generate_blobs: inst.get_bool("allowGenerateBlobs").unwrap_or_default(),
            nearby_ambient_multiplier: inst.get_f32("nearbyAmbientMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanWaveDetectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanWaveDetectionParams {
    /// `requireFullChargeDetection` (Boolean)
    #[serde(default)]
    pub require_full_charge_detection: bool,
    /// `reflectScanWaveChargeLevel` (Boolean)
    #[serde(default)]
    pub reflect_scan_wave_charge_level: bool,
}

impl Pooled for ScanWaveDetectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_wave_detection_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_wave_detection_params }
}

impl<'a> Extract<'a> for ScanWaveDetectionParams {
    const TYPE_NAME: &'static str = "ScanWaveDetectionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            require_full_charge_detection: inst.get_bool("requireFullChargeDetection").unwrap_or_default(),
            reflect_scan_wave_charge_level: inst.get_bool("reflectScanWaveChargeLevel").unwrap_or_default(),
        }
    }
}

/// DCB type: `DeltaSignatureSensitivityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSignatureSensitivityParams {
    /// `sensitivity` (Single)
    #[serde(default)]
    pub sensitivity: f32,
    /// `pierce` (Single)
    #[serde(default)]
    pub pierce: f32,
}

impl Pooled for DeltaSignatureSensitivityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.delta_signature_sensitivity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.delta_signature_sensitivity_params }
}

impl<'a> Extract<'a> for DeltaSignatureSensitivityParams {
    const TYPE_NAME: &'static str = "DeltaSignatureSensitivityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sensitivity: inst.get_f32("sensitivity").unwrap_or_default(),
            pierce: inst.get_f32("pierce").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDeltaSignatureDetectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureDetectionParams {
    /// `deltaSignatureDetection` (Reference)
    #[serde(default)]
    pub delta_signature_detection: Option<CigGuid>,
    /// `alwaysDetect` (Boolean)
    #[serde(default)]
    pub always_detect: bool,
    /// `markActiveDetection` (Boolean)
    #[serde(default)]
    pub mark_active_detection: bool,
    /// `detectionChargeLevel` (EnumChoice)
    #[serde(default)]
    pub detection_charge_level: String,
    /// `emissionModifier` (Single)
    #[serde(default)]
    pub emission_modifier: f32,
    /// `scanWaveDetectionParams` (StrongPointer)
    #[serde(default)]
    pub scan_wave_detection_params: Option<Handle<ScanWaveDetectionParams>>,
}

impl Pooled for RadarDeltaSignatureDetectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_delta_signature_detection_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_delta_signature_detection_params }
}

impl<'a> Extract<'a> for RadarDeltaSignatureDetectionParams {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureDetectionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            delta_signature_detection: inst.get("deltaSignatureDetection").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            always_detect: inst.get_bool("alwaysDetect").unwrap_or_default(),
            mark_active_detection: inst.get_bool("markActiveDetection").unwrap_or_default(),
            detection_charge_level: inst.get_str("detectionChargeLevel").map(String::from).unwrap_or_default(),
            emission_modifier: inst.get_f32("emissionModifier").unwrap_or_default(),
            scan_wave_detection_params: match inst.get("scanWaveDetectionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanWaveDetectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanWaveDetectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DeltaSignatureSpikeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSignatureSpikeParams {
    /// `operationType` (EnumChoice)
    #[serde(default)]
    pub operation_type: String,
    /// `spikeValue` (Single)
    #[serde(default)]
    pub spike_value: f32,
}

impl Pooled for DeltaSignatureSpikeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.delta_signature_spike_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.delta_signature_spike_params }
}

impl<'a> Extract<'a> for DeltaSignatureSpikeParams {
    const TYPE_NAME: &'static str = "DeltaSignatureSpikeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            operation_type: inst.get_str("operationType").map(String::from).unwrap_or_default(),
            spike_value: inst.get_f32("spikeValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `RoomTraversalOperationsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTraversalOperationsParams {
    /// `operationType` (EnumChoice)
    #[serde(default)]
    pub operation_type: String,
    /// `traversalValue` (Single)
    #[serde(default)]
    pub traversal_value: f32,
}

impl Pooled for RoomTraversalOperationsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.room_traversal_operations_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.room_traversal_operations_params }
}

impl<'a> Extract<'a> for RoomTraversalOperationsParams {
    const TYPE_NAME: &'static str = "RoomTraversalOperationsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            operation_type: inst.get_str("operationType").map(String::from).unwrap_or_default(),
            traversal_value: inst.get_f32("traversalValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `RoomTraversalContactTypeEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTraversalContactTypeEntry {
    /// `contactType` (Reference)
    #[serde(default)]
    pub contact_type: Option<CigGuid>,
    /// `operations` (Class)
    #[serde(default)]
    pub operations: Option<Handle<RoomTraversalOperationsParams>>,
}

impl Pooled for RoomTraversalContactTypeEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.room_traversal_contact_type_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.room_traversal_contact_type_entry }
}

impl<'a> Extract<'a> for RoomTraversalContactTypeEntry {
    const TYPE_NAME: &'static str = "RoomTraversalContactTypeEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            contact_type: inst.get("contactType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            operations: match inst.get("operations") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RoomTraversalOperationsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RoomTraversalOperationsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RoomTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTraversalParams {
    /// `maxTraversalNumber` (UInt32)
    #[serde(default)]
    pub max_traversal_number: u32,
    /// `contactTypeEntry` (Class (array))
    #[serde(default)]
    pub contact_type_entry: Vec<Handle<RoomTraversalContactTypeEntry>>,
}

impl Pooled for RoomTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.room_traversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.room_traversal_params }
}

impl<'a> Extract<'a> for RoomTraversalParams {
    const TYPE_NAME: &'static str = "RoomTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_traversal_number: inst.get_u32("maxTraversalNumber").unwrap_or_default(),
            contact_type_entry: inst.get_array("contactTypeEntry")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RoomTraversalContactTypeEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RoomTraversalContactTypeEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RoomSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSharedParams {
    /// `enableInteriorVsExteriorCheck` (Boolean)
    #[serde(default)]
    pub enable_interior_vs_exterior_check: bool,
    /// `roomTraversalParams` (Class)
    #[serde(default)]
    pub room_traversal_params: Option<Handle<RoomTraversalParams>>,
}

impl Pooled for RoomSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.room_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.room_shared_params }
}

impl<'a> Extract<'a> for RoomSharedParams {
    const TYPE_NAME: &'static str = "RoomSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_interior_vs_exterior_check: inst.get_bool("enableInteriorVsExteriorCheck").unwrap_or_default(),
            room_traversal_params: match inst.get("roomTraversalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RoomTraversalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RoomTraversalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarSystemSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSystemSharedParams {
    /// `paramsVersion` (UInt32)
    #[serde(default)]
    pub params_version: u32,
    /// `focusAngles` (Single)
    #[serde(default)]
    pub focus_angles: f32,
    /// `radarParams` (StrongPointer)
    #[serde(default)]
    pub radar_params: Option<Handle<RadarSharedParams>>,
    /// `scanParams` (StrongPointer)
    #[serde(default)]
    pub scan_params: Option<Handle<ScanSharedParams>>,
    /// `pingParams` (StrongPointer)
    #[serde(default)]
    pub ping_params: Option<Handle<PingSharedParams>>,
    /// `highlightParams` (StrongPointer)
    #[serde(default)]
    pub highlight_params: Option<Handle<ContactHighlightSharedParams>>,
    /// `taggingParams` (StrongPointer)
    #[serde(default)]
    pub tagging_params: Option<Handle<ContactTaggingSharedParams>>,
    /// `occlusionParams` (StrongPointer)
    #[serde(default)]
    pub occlusion_params: Option<Handle<OcclusionCheckSharedParams>>,
    /// `roomSharedParams` (StrongPointer)
    #[serde(default)]
    pub room_shared_params: Option<Handle<RoomSharedParams>>,
    /// `displayRadarContactMarkers` (Boolean)
    #[serde(default)]
    pub display_radar_contact_markers: bool,
    /// `applyNearbyAmbientSignatures` (Boolean)
    #[serde(default)]
    pub apply_nearby_ambient_signatures: bool,
    /// `radarJammerParams` (StrongPointer)
    #[serde(default)]
    pub radar_jammer_params: Option<Handle<RadarJammerSharedParams>>,
    /// `deltaSignatureDetectionParams` (Class (array))
    #[serde(default)]
    pub delta_signature_detection_params: Vec<Handle<RadarDeltaSignatureDetectionParams>>,
    /// `deltaSignatureBaseSpike` (Class)
    #[serde(default)]
    pub delta_signature_base_spike: Option<Handle<DeltaSignatureSpikeParams>>,
    /// `deltaSignatureSensitivityParams` (Class)
    #[serde(default)]
    pub delta_signature_sensitivity_params: Option<Handle<DeltaSignatureSensitivityParams>>,
}

impl Pooled for RadarSystemSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_system_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_system_shared_params }
}

impl<'a> Extract<'a> for RadarSystemSharedParams {
    const TYPE_NAME: &'static str = "RadarSystemSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params_version: inst.get_u32("paramsVersion").unwrap_or_default(),
            focus_angles: inst.get_f32("focusAngles").unwrap_or_default(),
            radar_params: match inst.get("radarParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_params: match inst.get("scanParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_params: match inst.get("pingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tagging_params: match inst.get("taggingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactTaggingSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactTaggingSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occlusion_params: match inst.get("occlusionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<OcclusionCheckSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<OcclusionCheckSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_shared_params: match inst.get("roomSharedParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RoomSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RoomSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_radar_contact_markers: inst.get_bool("displayRadarContactMarkers").unwrap_or_default(),
            apply_nearby_ambient_signatures: inst.get_bool("applyNearbyAmbientSignatures").unwrap_or_default(),
            radar_jammer_params: match inst.get("radarJammerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarJammerSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarJammerSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delta_signature_detection_params: inst.get_array("deltaSignatureDetectionParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RadarDeltaSignatureDetectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RadarDeltaSignatureDetectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            delta_signature_base_spike: match inst.get("deltaSignatureBaseSpike") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DeltaSignatureSpikeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DeltaSignatureSpikeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delta_signature_sensitivity_params: match inst.get("deltaSignatureSensitivityParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DeltaSignatureSensitivityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DeltaSignatureSensitivityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSharedParams {
    /// `maxPassiveDistance` (Single)
    #[serde(default)]
    pub max_passive_distance: f32,
    /// `useRoomGraphForPassiveDetection` (Boolean)
    #[serde(default)]
    pub use_room_graph_for_passive_detection: bool,
}

impl Pooled for RadarSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_shared_params }
}

impl<'a> Extract<'a> for RadarSharedParams {
    const TYPE_NAME: &'static str = "RadarSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_passive_distance: inst.get_f32("maxPassiveDistance").unwrap_or_default(),
            use_room_graph_for_passive_detection: inst.get_bool("useRoomGraphForPassiveDetection").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarJammerSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarJammerSharedParams {
    /// `generalHUDDistortionStrength` (Single)
    #[serde(default)]
    pub general_huddistortion_strength: f32,
    /// `minimapDistortionStrength` (Single)
    #[serde(default)]
    pub minimap_distortion_strength: f32,
    /// `jammedActiveDetectionRange` (Single)
    #[serde(default)]
    pub jammed_active_detection_range: f32,
    /// `jammedPingNotificationText` (Locale)
    #[serde(default)]
    pub jammed_ping_notification_text: String,
    /// `jammedPingNotificationAudioTag` (String)
    #[serde(default)]
    pub jammed_ping_notification_audio_tag: String,
}

impl Pooled for RadarJammerSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_jammer_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_jammer_shared_params }
}

impl<'a> Extract<'a> for RadarJammerSharedParams {
    const TYPE_NAME: &'static str = "RadarJammerSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            general_huddistortion_strength: inst.get_f32("generalHUDDistortionStrength").unwrap_or_default(),
            minimap_distortion_strength: inst.get_f32("minimapDistortionStrength").unwrap_or_default(),
            jammed_active_detection_range: inst.get_f32("jammedActiveDetectionRange").unwrap_or_default(),
            jammed_ping_notification_text: inst.get_str("jammedPingNotificationText").map(String::from).unwrap_or_default(),
            jammed_ping_notification_audio_tag: inst.get_str("jammedPingNotificationAudioTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSharedParams {
    /// `enableNewBindings` (Boolean)
    #[serde(default)]
    pub enable_new_bindings: bool,
    /// `requireLockedTarget` (Boolean)
    #[serde(default)]
    pub require_locked_target: bool,
    /// `enableManualFocusScan` (Boolean)
    #[serde(default)]
    pub enable_manual_focus_scan: bool,
    /// `enableAutoFocusScan` (Boolean)
    #[serde(default)]
    pub enable_auto_focus_scan: bool,
    /// `enablePassiveScan` (Boolean)
    #[serde(default)]
    pub enable_passive_scan: bool,
    /// `enablePingWaveScan` (Boolean)
    #[serde(default)]
    pub enable_ping_wave_scan: bool,
    /// `allowPassiveUnlockContactType` (Boolean)
    #[serde(default)]
    pub allow_passive_unlock_contact_type: bool,
    /// `enableActiveUnlockName` (Boolean)
    #[serde(default)]
    pub enable_active_unlock_name: bool,
    /// `unscannedText` (Locale)
    #[serde(default)]
    pub unscanned_text: String,
    /// `sfxParams` (Class)
    #[serde(default)]
    pub sfx_params: Option<Handle<ScanSFXSharedParams>>,
}

impl Pooled for ScanSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_shared_params }
}

impl<'a> Extract<'a> for ScanSharedParams {
    const TYPE_NAME: &'static str = "ScanSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_new_bindings: inst.get_bool("enableNewBindings").unwrap_or_default(),
            require_locked_target: inst.get_bool("requireLockedTarget").unwrap_or_default(),
            enable_manual_focus_scan: inst.get_bool("enableManualFocusScan").unwrap_or_default(),
            enable_auto_focus_scan: inst.get_bool("enableAutoFocusScan").unwrap_or_default(),
            enable_passive_scan: inst.get_bool("enablePassiveScan").unwrap_or_default(),
            enable_ping_wave_scan: inst.get_bool("enablePingWaveScan").unwrap_or_default(),
            allow_passive_unlock_contact_type: inst.get_bool("allowPassiveUnlockContactType").unwrap_or_default(),
            enable_active_unlock_name: inst.get_bool("enableActiveUnlockName").unwrap_or_default(),
            unscanned_text: inst.get_str("unscannedText").map(String::from).unwrap_or_default(),
            sfx_params: match inst.get("sfxParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanSFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanSFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanSFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSFXSharedParams {
    /// `startScan` (Class)
    #[serde(default)]
    pub start_scan: Option<Handle<GlobalResourceAudio>>,
    /// `stopScan` (Class)
    #[serde(default)]
    pub stop_scan: Option<Handle<GlobalResourceAudio>>,
    /// `infoPopulate` (Class)
    #[serde(default)]
    pub info_populate: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for ScanSFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_sfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_sfxshared_params }
}

impl<'a> Extract<'a> for ScanSFXSharedParams {
    const TYPE_NAME: &'static str = "ScanSFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_scan: match inst.get("startScan") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_scan: match inst.get("stopScan") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            info_populate: match inst.get("infoPopulate") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PingSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingSharedParams {
    /// `blobLifeTime` (Class)
    #[serde(default)]
    pub blob_life_time: Option<Handle<PingBlobLifeTime>>,
    /// `blobOffsetScalar` (Single)
    #[serde(default)]
    pub blob_offset_scalar: f32,
    /// `blobSizeScalar` (Single)
    #[serde(default)]
    pub blob_size_scalar: f32,
    /// `blobScaleFov` (Single)
    #[serde(default)]
    pub blob_scale_fov: f32,
    /// `blobScaleMinPixels` (Single)
    #[serde(default)]
    pub blob_scale_min_pixels: f32,
    /// `blobScaleMaxPixels` (Single)
    #[serde(default)]
    pub blob_scale_max_pixels: f32,
    /// `blobScaleFixedResolution` (Single)
    #[serde(default)]
    pub blob_scale_fixed_resolution: f32,
    /// `contactLifeTime` (Class)
    #[serde(default)]
    pub contact_life_time: Option<Handle<PingContactLifeTime>>,
    /// `extendedLifeTimeParams` (StrongPointer)
    #[serde(default)]
    pub extended_life_time_params: Option<Handle<PingExtendedLifeTimeParams>>,
    /// `pingWaveAcceleration` (Single)
    #[serde(default)]
    pub ping_wave_acceleration: f32,
    /// `pingWaveJerk` (Single)
    #[serde(default)]
    pub ping_wave_jerk: f32,
    /// `pingWaveJerkDistance` (Single)
    #[serde(default)]
    pub ping_wave_jerk_distance: f32,
    /// `useADSMode` (Boolean)
    #[serde(default)]
    pub use_adsmode: bool,
    /// `ADSFovFocusAngleMultiplier` (Single)
    #[serde(default)]
    pub adsfov_focus_angle_multiplier: f32,
    /// `blobVFXParams` (StrongPointer)
    #[serde(default)]
    pub blob_vfxparams: Option<Handle<BlobVFXSharedParams>>,
    /// `pingSFXParams` (StrongPointer)
    #[serde(default)]
    pub ping_sfxparams: Option<Handle<PingSFXSharedParams>>,
    /// `pingVFXParams` (StrongPointer)
    #[serde(default)]
    pub ping_vfxparams: Option<Handle<PingVFXSharedParams>>,
    /// `pingTypeParams` (StrongPointer)
    #[serde(default)]
    pub ping_type_params: Option<Handle<PingTypeParams>>,
}

impl Pooled for PingSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_shared_params }
}

impl<'a> Extract<'a> for PingSharedParams {
    const TYPE_NAME: &'static str = "PingSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blob_life_time: match inst.get("blobLifeTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingBlobLifeTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingBlobLifeTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blob_offset_scalar: inst.get_f32("blobOffsetScalar").unwrap_or_default(),
            blob_size_scalar: inst.get_f32("blobSizeScalar").unwrap_or_default(),
            blob_scale_fov: inst.get_f32("blobScaleFov").unwrap_or_default(),
            blob_scale_min_pixels: inst.get_f32("blobScaleMinPixels").unwrap_or_default(),
            blob_scale_max_pixels: inst.get_f32("blobScaleMaxPixels").unwrap_or_default(),
            blob_scale_fixed_resolution: inst.get_f32("blobScaleFixedResolution").unwrap_or_default(),
            contact_life_time: match inst.get("contactLifeTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingContactLifeTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingContactLifeTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            extended_life_time_params: match inst.get("extendedLifeTimeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingExtendedLifeTimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingExtendedLifeTimeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_wave_acceleration: inst.get_f32("pingWaveAcceleration").unwrap_or_default(),
            ping_wave_jerk: inst.get_f32("pingWaveJerk").unwrap_or_default(),
            ping_wave_jerk_distance: inst.get_f32("pingWaveJerkDistance").unwrap_or_default(),
            use_adsmode: inst.get_bool("useADSMode").unwrap_or_default(),
            adsfov_focus_angle_multiplier: inst.get_f32("ADSFovFocusAngleMultiplier").unwrap_or_default(),
            blob_vfxparams: match inst.get("blobVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlobVFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlobVFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_sfxparams: match inst.get("pingSFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingSFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingSFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_vfxparams: match inst.get("pingVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingVFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingVFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_type_params: match inst.get("pingTypeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingTypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingTypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PingBlobLifeTime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingBlobLifeTime {
    /// `lifeTimeMin` (Single)
    #[serde(default)]
    pub life_time_min: f32,
    /// `lifeTimeMax` (Single)
    #[serde(default)]
    pub life_time_max: f32,
}

impl Pooled for PingBlobLifeTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_blob_life_time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_blob_life_time }
}

impl<'a> Extract<'a> for PingBlobLifeTime {
    const TYPE_NAME: &'static str = "PingBlobLifeTime";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            life_time_min: inst.get_f32("lifeTimeMin").unwrap_or_default(),
            life_time_max: inst.get_f32("lifeTimeMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingExtendedLifeTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingExtendedLifeTimeParams {
    /// `extendedContactTypes` (Reference (array))
    #[serde(default)]
    pub extended_contact_types: Vec<CigGuid>,
    /// `extendedLifeTimeDuration` (Single)
    #[serde(default)]
    pub extended_life_time_duration: f32,
    /// `extendForOccludedContacts` (Boolean)
    #[serde(default)]
    pub extend_for_occluded_contacts: bool,
    /// `disableQuickScanHighlightOnOcclusion` (Boolean)
    #[serde(default)]
    pub disable_quick_scan_highlight_on_occlusion: bool,
    /// `resetDetectionOnDeath` (Boolean)
    #[serde(default)]
    pub reset_detection_on_death: bool,
    /// `extendForDeadContacts` (Boolean)
    #[serde(default)]
    pub extend_for_dead_contacts: bool,
}

impl Pooled for PingExtendedLifeTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_extended_life_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_extended_life_time_params }
}

impl<'a> Extract<'a> for PingExtendedLifeTimeParams {
    const TYPE_NAME: &'static str = "PingExtendedLifeTimeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            extended_contact_types: inst.get_array("extendedContactTypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            extended_life_time_duration: inst.get_f32("extendedLifeTimeDuration").unwrap_or_default(),
            extend_for_occluded_contacts: inst.get_bool("extendForOccludedContacts").unwrap_or_default(),
            disable_quick_scan_highlight_on_occlusion: inst.get_bool("disableQuickScanHighlightOnOcclusion").unwrap_or_default(),
            reset_detection_on_death: inst.get_bool("resetDetectionOnDeath").unwrap_or_default(),
            extend_for_dead_contacts: inst.get_bool("extendForDeadContacts").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingContactLifeTime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingContactLifeTime {
    /// `lifeTimeMin` (Single)
    #[serde(default)]
    pub life_time_min: f32,
    /// `lifeTimeMax` (Single)
    #[serde(default)]
    pub life_time_max: f32,
}

impl Pooled for PingContactLifeTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_contact_life_time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_contact_life_time }
}

impl<'a> Extract<'a> for PingContactLifeTime {
    const TYPE_NAME: &'static str = "PingContactLifeTime";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            life_time_min: inst.get_f32("lifeTimeMin").unwrap_or_default(),
            life_time_max: inst.get_f32("lifeTimeMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `BlobVFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobVFXSharedParams {
    /// `snapBlobTimescaleToLifeTime` (Boolean)
    #[serde(default)]
    pub snap_blob_timescale_to_life_time: bool,
    /// `blobVFXs` (Class (array))
    #[serde(default)]
    pub blob_vfxs: Vec<Handle<BlobVFXDistanceParams>>,
}

impl Pooled for BlobVFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.blob_vfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.blob_vfxshared_params }
}

impl<'a> Extract<'a> for BlobVFXSharedParams {
    const TYPE_NAME: &'static str = "BlobVFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            snap_blob_timescale_to_life_time: inst.get_bool("snapBlobTimescaleToLifeTime").unwrap_or_default(),
            blob_vfxs: inst.get_array("blobVFXs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BlobVFXDistanceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BlobVFXDistanceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BlobVFXDistanceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobVFXDistanceParams {
    /// `effect` (Class)
    #[serde(default)]
    pub effect: Option<Handle<GlobalResourceParticle>>,
    /// `distance` (Double)
    #[serde(default)]
    pub distance: f64,
}

impl Pooled for BlobVFXDistanceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.blob_vfxdistance_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.blob_vfxdistance_params }
}

impl<'a> Extract<'a> for BlobVFXDistanceParams {
    const TYPE_NAME: &'static str = "BlobVFXDistanceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect: match inst.get("effect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance: inst.get_f64("distance").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingSFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingSFXSharedParams {
    /// `invokePing` (Class)
    #[serde(default)]
    pub invoke_ping: Option<Handle<GlobalResourceAudio>>,
    /// `invokePingCharged` (Class)
    #[serde(default)]
    pub invoke_ping_charged: Option<Handle<GlobalResourceAudio>>,
    /// `invokePingBlocked` (Class)
    #[serde(default)]
    pub invoke_ping_blocked: Option<Handle<GlobalResourceAudio>>,
    /// `invokePingCooldown` (Class)
    #[serde(default)]
    pub invoke_ping_cooldown: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for PingSFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_sfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_sfxshared_params }
}

impl<'a> Extract<'a> for PingSFXSharedParams {
    const TYPE_NAME: &'static str = "PingSFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            invoke_ping: match inst.get("invokePing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invoke_ping_charged: match inst.get("invokePingCharged") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invoke_ping_blocked: match inst.get("invokePingBlocked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invoke_ping_cooldown: match inst.get("invokePingCooldown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PingWaveVFXParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingWaveVFXParams {
    /// `waveParticle` (Class)
    #[serde(default)]
    pub wave_particle: Option<Handle<GlobalResourceParticle>>,
    /// `pulseSphereGeometry` (Class)
    #[serde(default)]
    pub pulse_sphere_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `pulseHalfSphereGeometry` (Class)
    #[serde(default)]
    pub pulse_half_sphere_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `pulseConeGeometry` (Class)
    #[serde(default)]
    pub pulse_cone_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `pulseMaterial` (Class)
    #[serde(default)]
    pub pulse_material: Option<Handle<GlobalResourceMaterial>>,
    /// `conePulseMaterial` (Class)
    #[serde(default)]
    pub cone_pulse_material: Option<Handle<GlobalResourceMaterial>>,
    /// `visualAcceleration` (Single)
    #[serde(default)]
    pub visual_acceleration: f32,
    /// `visualMaxDistance` (Single)
    #[serde(default)]
    pub visual_max_distance: f32,
    /// `snapWaveTimescaleToLifetime` (Boolean)
    #[serde(default)]
    pub snap_wave_timescale_to_lifetime: bool,
    /// `use360PingwaveEffectForAllLevels` (Boolean)
    #[serde(default)]
    pub use360_pingwave_effect_for_all_levels: bool,
    /// `attachPingwaveEffectToHost` (Boolean)
    #[serde(default)]
    pub attach_pingwave_effect_to_host: bool,
}

impl Pooled for PingWaveVFXParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_wave_vfxparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_wave_vfxparams }
}

impl<'a> Extract<'a> for PingWaveVFXParams {
    const TYPE_NAME: &'static str = "PingWaveVFXParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wave_particle: match inst.get("waveParticle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_sphere_geometry: match inst.get("pulseSphereGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_half_sphere_geometry: match inst.get("pulseHalfSphereGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_cone_geometry: match inst.get("pulseConeGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_material: match inst.get("pulseMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cone_pulse_material: match inst.get("conePulseMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            visual_acceleration: inst.get_f32("visualAcceleration").unwrap_or_default(),
            visual_max_distance: inst.get_f32("visualMaxDistance").unwrap_or_default(),
            snap_wave_timescale_to_lifetime: inst.get_bool("snapWaveTimescaleToLifetime").unwrap_or_default(),
            use360_pingwave_effect_for_all_levels: inst.get_bool("use360PingwaveEffectForAllLevels").unwrap_or_default(),
            attach_pingwave_effect_to_host: inst.get_bool("attachPingwaveEffectToHost").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingVFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingVFXSharedParams {
    /// `quickPingWaveVFXParams` (StrongPointer)
    #[serde(default)]
    pub quick_ping_wave_vfxparams: Option<Handle<PingWaveVFXParams>>,
    /// `chargedPingWaveVFXParams` (StrongPointer)
    #[serde(default)]
    pub charged_ping_wave_vfxparams: Option<Handle<PingWaveVFXParams>>,
    /// `blockedPingWaveVFXParams` (StrongPointer)
    #[serde(default)]
    pub blocked_ping_wave_vfxparams: Option<Handle<PingWaveVFXParams>>,
}

impl Pooled for PingVFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_vfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_vfxshared_params }
}

impl<'a> Extract<'a> for PingVFXSharedParams {
    const TYPE_NAME: &'static str = "PingVFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            quick_ping_wave_vfxparams: match inst.get("quickPingWaveVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingWaveVFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingWaveVFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            charged_ping_wave_vfxparams: match inst.get("chargedPingWaveVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingWaveVFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingWaveVFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blocked_ping_wave_vfxparams: match inst.get("blockedPingWaveVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingWaveVFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingWaveVFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PingTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingTypeParams {
    /// `pingChargeUIShowTime` (Single)
    #[serde(default)]
    pub ping_charge_uishow_time: f32,
    /// `pingChargeUIJammedTime` (Single)
    #[serde(default)]
    pub ping_charge_uijammed_time: f32,
    /// `pingChargeUIHideTime` (Single)
    #[serde(default)]
    pub ping_charge_uihide_time: f32,
    /// `pingChargeUIUnavailableTime` (Single)
    #[serde(default)]
    pub ping_charge_uiunavailable_time: f32,
}

impl Pooled for PingTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.ping_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.ping_type_params }
}

impl<'a> Extract<'a> for PingTypeParams {
    const TYPE_NAME: &'static str = "PingTypeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ping_charge_uishow_time: inst.get_f32("pingChargeUIShowTime").unwrap_or_default(),
            ping_charge_uijammed_time: inst.get_f32("pingChargeUIJammedTime").unwrap_or_default(),
            ping_charge_uihide_time: inst.get_f32("pingChargeUIHideTime").unwrap_or_default(),
            ping_charge_uiunavailable_time: inst.get_f32("pingChargeUIUnavailableTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContactHighlightLayersParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightLayersParams {
    /// `highlightParams` (Class)
    #[serde(default)]
    pub highlight_params: Option<Handle<ContactHighlightStateParams>>,
}

impl Pooled for ContactHighlightLayersParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.contact_highlight_layers_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.contact_highlight_layers_params }
}

impl<'a> Extract<'a> for ContactHighlightLayersParams {
    const TYPE_NAME: &'static str = "ContactHighlightLayersParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightStateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightStateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContactHighlightSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightSharedParams {
    /// `enableMaterialHighlight` (Boolean)
    #[serde(default)]
    pub enable_material_highlight: bool,
    /// `lifeTimeTotal` (Single)
    #[serde(default)]
    pub life_time_total: f32,
    /// `lifeTimeInterference` (Single)
    #[serde(default)]
    pub life_time_interference: f32,
    /// `lifeTimeFlare` (Single)
    #[serde(default)]
    pub life_time_flare: f32,
    /// `maxHighlightDistance` (Single)
    #[serde(default)]
    pub max_highlight_distance: f32,
    /// `minHighlightDistance` (Single)
    #[serde(default)]
    pub min_highlight_distance: f32,
    /// `maxHighlightOpacity` (Single)
    #[serde(default)]
    pub max_highlight_opacity: f32,
    /// `minHighlightOpacity` (Single)
    #[serde(default)]
    pub min_highlight_opacity: f32,
    /// `opacityFadeInDelay` (Single)
    #[serde(default)]
    pub opacity_fade_in_delay: f32,
    /// `opacityFadeInDuration` (Single)
    #[serde(default)]
    pub opacity_fade_in_duration: f32,
    /// `opacityFadeOutDuration` (Single)
    #[serde(default)]
    pub opacity_fade_out_duration: f32,
    /// `highlightLayers` (Class)
    #[serde(default)]
    pub highlight_layers: Option<Handle<ContactHighlightLayersParams>>,
    /// `audioTriggerTags` (Reference)
    #[serde(default)]
    pub audio_trigger_tags: Option<CigGuid>,
}

impl Pooled for ContactHighlightSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.contact_highlight_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.contact_highlight_shared_params }
}

impl<'a> Extract<'a> for ContactHighlightSharedParams {
    const TYPE_NAME: &'static str = "ContactHighlightSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_material_highlight: inst.get_bool("enableMaterialHighlight").unwrap_or_default(),
            life_time_total: inst.get_f32("lifeTimeTotal").unwrap_or_default(),
            life_time_interference: inst.get_f32("lifeTimeInterference").unwrap_or_default(),
            life_time_flare: inst.get_f32("lifeTimeFlare").unwrap_or_default(),
            max_highlight_distance: inst.get_f32("maxHighlightDistance").unwrap_or_default(),
            min_highlight_distance: inst.get_f32("minHighlightDistance").unwrap_or_default(),
            max_highlight_opacity: inst.get_f32("maxHighlightOpacity").unwrap_or_default(),
            min_highlight_opacity: inst.get_f32("minHighlightOpacity").unwrap_or_default(),
            opacity_fade_in_delay: inst.get_f32("opacityFadeInDelay").unwrap_or_default(),
            opacity_fade_in_duration: inst.get_f32("opacityFadeInDuration").unwrap_or_default(),
            opacity_fade_out_duration: inst.get_f32("opacityFadeOutDuration").unwrap_or_default(),
            highlight_layers: match inst.get("highlightLayers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightLayersParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightLayersParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_trigger_tags: inst.get("audioTriggerTags").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContactHighlightVisualBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightVisualBaseParams {
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGBA>>,
}

impl Pooled for ContactHighlightVisualBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.contact_highlight_visual_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.contact_highlight_visual_base_params }
}

impl<'a> Extract<'a> for ContactHighlightVisualBaseParams {
    const TYPE_NAME: &'static str = "ContactHighlightVisualBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContactHighlightStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightStateParams {
    /// `defaultParams` (StrongPointer)
    #[serde(default)]
    pub default_params: Option<Handle<ContactHighlightVisualBaseParams>>,
    /// `occludedParams` (StrongPointer)
    #[serde(default)]
    pub occluded_params: Option<Handle<ContactHighlightVisualBaseParams>>,
    /// `flareParams` (StrongPointer)
    #[serde(default)]
    pub flare_params: Option<Handle<ContactHighlightVisualBaseParams>>,
    /// `occludedFlareParams` (StrongPointer)
    #[serde(default)]
    pub occluded_flare_params: Option<Handle<ContactHighlightVisualBaseParams>>,
}

impl Pooled for ContactHighlightStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.contact_highlight_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.contact_highlight_state_params }
}

impl<'a> Extract<'a> for ContactHighlightStateParams {
    const TYPE_NAME: &'static str = "ContactHighlightStateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_params: match inst.get("defaultParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occluded_params: match inst.get("occludedParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flare_params: match inst.get("flareParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occluded_flare_params: match inst.get("occludedFlareParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContactTaggingSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactTaggingSharedParams {
    /// `viewAngle` (Single)
    #[serde(default)]
    pub view_angle: f32,
    /// `maxTaggingDistance` (Single)
    #[serde(default)]
    pub max_tagging_distance: f32,
}

impl Pooled for ContactTaggingSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.contact_tagging_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.contact_tagging_shared_params }
}

impl<'a> Extract<'a> for ContactTaggingSharedParams {
    const TYPE_NAME: &'static str = "ContactTaggingSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            view_angle: inst.get_f32("viewAngle").unwrap_or_default(),
            max_tagging_distance: inst.get_f32("maxTaggingDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `OcclusionCheckSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcclusionCheckSharedParams {
    /// `enablePassiveDetectionOcclusion` (Boolean)
    #[serde(default)]
    pub enable_passive_detection_occlusion: bool,
    /// `enableActiveDetectionOcclusion` (Boolean)
    #[serde(default)]
    pub enable_active_detection_occlusion: bool,
    /// `activeOcclusionIgnoreRange` (Single)
    #[serde(default)]
    pub active_occlusion_ignore_range: f32,
}

impl Pooled for OcclusionCheckSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.occlusion_check_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.occlusion_check_shared_params }
}

impl<'a> Extract<'a> for OcclusionCheckSharedParams {
    const TYPE_NAME: &'static str = "OcclusionCheckSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_passive_detection_occlusion: inst.get_bool("enablePassiveDetectionOcclusion").unwrap_or_default(),
            enable_active_detection_occlusion: inst.get_bool("enableActiveDetectionOcclusion").unwrap_or_default(),
            active_occlusion_ignore_range: inst.get_f32("activeOcclusionIgnoreRange").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanInformationDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInformationDef {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `scanDisplayLayout` (Reference)
    #[serde(default)]
    pub scan_display_layout: Option<CigGuid>,
    /// `scanProcedures` (StrongPointer (array))
    #[serde(default)]
    pub scan_procedures: Vec<Handle<ScanProcedureParams>>,
}

impl Pooled for ScanInformationDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_information_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_information_def }
}

impl<'a> Extract<'a> for ScanInformationDef {
    const TYPE_NAME: &'static str = "ScanInformationDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            scan_display_layout: inst.get("scanDisplayLayout").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            scan_procedures: inst.get_array("scanProcedures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScanProcedureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScanProcedureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanInformationTable`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInformationTable {
    /// `defs` (Reference (array))
    #[serde(default)]
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

/// DCB type: `ScanBoxoutIconDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanBoxoutIconDef {
    /// `showConditions` (EnumChoice (array))
    #[serde(default)]
    pub show_conditions: Vec<String>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
}

impl Pooled for ScanBoxoutIconDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_boxout_icon_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_boxout_icon_def }
}

impl<'a> Extract<'a> for ScanBoxoutIconDef {
    const TYPE_NAME: &'static str = "ScanBoxoutIconDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            show_conditions: inst.get_array("showConditions")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanCustomDataDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCustomDataDef {
    /// `info` (Class)
    #[serde(default)]
    pub info: Option<Handle<ScanCustomDataInfo>>,
}

impl Pooled for ScanCustomDataDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_custom_data_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_custom_data_def }
}

impl<'a> Extract<'a> for ScanCustomDataDef {
    const TYPE_NAME: &'static str = "ScanCustomDataDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            info: match inst.get("info") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanCustomDataInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanCustomDataInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanDisplaySectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDisplaySectionParams {
    /// `displayInstances` (Reference (array))
    #[serde(default)]
    pub display_instances: Vec<CigGuid>,
}

impl Pooled for ScanDisplaySectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_display_section_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_display_section_params }
}

impl<'a> Extract<'a> for ScanDisplaySectionParams {
    const TYPE_NAME: &'static str = "ScanDisplaySectionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_instances: inst.get_array("displayInstances")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanDisplayLayoutParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDisplayLayoutParams {
    /// `displaySections` (Class)
    #[serde(default)]
    pub display_sections: Option<Handle<ScanDisplaySectionParams>>,
    /// `contactDisplay` (StrongPointer)
    #[serde(default)]
    pub contact_display: Option<Handle<ScanDisplaySetupParams>>,
    /// `boxoutIconDefs` (Class (array))
    #[serde(default)]
    pub boxout_icon_defs: Vec<Handle<ScanBoxoutIconDef>>,
}

impl Pooled for ScanDisplayLayoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_display_layout_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_display_layout_params }
}

impl<'a> Extract<'a> for ScanDisplayLayoutParams {
    const TYPE_NAME: &'static str = "ScanDisplayLayoutParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_sections: match inst.get("displaySections") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanDisplaySectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanDisplaySectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contact_display: match inst.get("contactDisplay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanDisplaySetupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanDisplaySetupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            boxout_icon_defs: inst.get_array("boxoutIconDefs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScanBoxoutIconDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScanBoxoutIconDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactGamePlayProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactGamePlayProperties {
    /// `perceivedByAISense` (Boolean)
    #[serde(default)]
    pub perceived_by_aisense: bool,
}

impl Pooled for RadarContactGamePlayProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_contact_game_play_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_contact_game_play_properties }
}

impl<'a> Extract<'a> for RadarContactGamePlayProperties {
    const TYPE_NAME: &'static str = "RadarContactGamePlayProperties";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            perceived_by_aisense: inst.get_bool("perceivedByAISense").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarSignatureCategoryDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSignatureCategoryDefinition {
    /// `default` (Reference)
    #[serde(default)]
    pub default: Option<CigGuid>,
    /// `types` (Reference (array))
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSignatureCategoryEntry {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
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
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactTypeDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactTypeDefinition {
    /// `unknownType` (Reference)
    #[serde(default)]
    pub unknown_type: Option<CigGuid>,
    /// `defaultAudioType` (Reference)
    #[serde(default)]
    pub default_audio_type: Option<CigGuid>,
    /// `types` (Reference (array))
    #[serde(default)]
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

/// DCB type: `RadarContactTypeEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactTypeEntry {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `scanDef` (Reference)
    #[serde(default)]
    pub scan_def: Option<CigGuid>,
    /// `gameplayProperties` (Class)
    #[serde(default)]
    pub gameplay_properties: Option<Handle<RadarContactGamePlayProperties>>,
    /// `trackerType` (EnumChoice)
    #[serde(default)]
    pub tracker_type: String,
    /// `markerConfig` (Reference)
    #[serde(default)]
    pub marker_config: Option<CigGuid>,
    /// `contactHoloMinScreenSize` (Single)
    #[serde(default)]
    pub contact_holo_min_screen_size: f32,
    /// `contactHighlightCategory` (EnumChoice)
    #[serde(default)]
    pub contact_highlight_category: String,
    /// `contactTaggingCategory` (EnumChoice)
    #[serde(default)]
    pub contact_tagging_category: String,
    /// `isObjectOfInterest` (Boolean)
    #[serde(default)]
    pub is_object_of_interest: bool,
    /// `minimumInfluenceFactor` (Single)
    #[serde(default)]
    pub minimum_influence_factor: f32,
    /// `minimumInfluenceOperation` (EnumChoice)
    #[serde(default)]
    pub minimum_influence_operation: String,
    /// `onlyDetectableFromSameZone` (Boolean)
    #[serde(default)]
    pub only_detectable_from_same_zone: bool,
}

impl Pooled for RadarContactTypeEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_contact_type_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_contact_type_entry }
}

impl<'a> Extract<'a> for RadarContactTypeEntry {
    const TYPE_NAME: &'static str = "RadarContactTypeEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            scan_def: inst.get("scanDef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gameplay_properties: match inst.get("gameplayProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarContactGamePlayProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarContactGamePlayProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tracker_type: inst.get_str("trackerType").map(String::from).unwrap_or_default(),
            marker_config: inst.get("markerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contact_holo_min_screen_size: inst.get_f32("contactHoloMinScreenSize").unwrap_or_default(),
            contact_highlight_category: inst.get_str("contactHighlightCategory").map(String::from).unwrap_or_default(),
            contact_tagging_category: inst.get_str("contactTaggingCategory").map(String::from).unwrap_or_default(),
            is_object_of_interest: inst.get_bool("isObjectOfInterest").unwrap_or_default(),
            minimum_influence_factor: inst.get_f32("minimumInfluenceFactor").unwrap_or_default(),
            minimum_influence_operation: inst.get_str("minimumInfluenceOperation").map(String::from).unwrap_or_default(),
            only_detectable_from_same_zone: inst.get_bool("onlyDetectableFromSameZone").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactGroupDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactGroupDefinition {
    /// `groups` (Reference (array))
    #[serde(default)]
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

/// DCB type: `RadarContactGroupEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactGroupEntry {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
    /// `children` (Class (array))
    #[serde(default)]
    pub children: Vec<Handle<RadarContactSubGroupEntry>>,
}

impl Pooled for RadarContactGroupEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_contact_group_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_contact_group_entry }
}

impl<'a> Extract<'a> for RadarContactGroupEntry {
    const TYPE_NAME: &'static str = "RadarContactGroupEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            children: inst.get_array("children")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RadarContactSubGroupEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RadarContactSubGroupEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactSubGroupEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactSubGroupEntry {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarContactSubGroupEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_contact_sub_group_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_contact_sub_group_entry }
}

impl<'a> Extract<'a> for RadarContactSubGroupEntry {
    const TYPE_NAME: &'static str = "RadarContactSubGroupEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDeltaSignatureNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureNotificationParams {
    /// `requireLockedTarget` (Boolean)
    #[serde(default)]
    pub require_locked_target: bool,
    /// `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `globalCooldown` (Single)
    #[serde(default)]
    pub global_cooldown: f32,
    /// `individualCooldown` (Single)
    #[serde(default)]
    pub individual_cooldown: f32,
}

impl Pooled for RadarDeltaSignatureNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_delta_signature_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_delta_signature_notification_params }
}

impl<'a> Extract<'a> for RadarDeltaSignatureNotificationParams {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            require_locked_target: inst.get_bool("requireLockedTarget").unwrap_or_default(),
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            global_cooldown: inst.get_f32("globalCooldown").unwrap_or_default(),
            individual_cooldown: inst.get_f32("individualCooldown").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDeltaSignatureDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureDefinition {
    /// `types` (Reference (array))
    #[serde(default)]
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

/// DCB type: `RadarDeltaSignatureEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureEntry {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `priority` (EnumChoice)
    #[serde(default)]
    pub priority: String,
    /// `expireTime` (Single)
    #[serde(default)]
    pub expire_time: f32,
    /// `notificationParams` (Class)
    #[serde(default)]
    pub notification_params: Option<Handle<RadarDeltaSignatureNotificationParams>>,
}

impl Pooled for RadarDeltaSignatureEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.radar_delta_signature_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.radar_delta_signature_entry }
}

impl<'a> Extract<'a> for RadarDeltaSignatureEntry {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            priority: inst.get_str("priority").map(String::from).unwrap_or_default(),
            expire_time: inst.get_f32("expireTime").unwrap_or_default(),
            notification_params: match inst.get("notificationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarDeltaSignatureNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarDeltaSignatureNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanProcedureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProcedureParams {
    /// `requiredScanTag` (Reference)
    #[serde(default)]
    pub required_scan_tag: Option<CigGuid>,
    /// `emissionBaseline` (Single)
    #[serde(default)]
    pub emission_baseline: f32,
    /// `runtimeDuration` (Single)
    #[serde(default)]
    pub runtime_duration: f32,
    /// `allowedInAIAutoScan` (Boolean)
    #[serde(default)]
    pub allowed_in_aiauto_scan: bool,
    /// `allowedInFocalPointScan` (Boolean)
    #[serde(default)]
    pub allowed_in_focal_point_scan: bool,
    /// `allowedInPingBroadScan` (Boolean)
    #[serde(default)]
    pub allowed_in_ping_broad_scan: bool,
    /// `allowedInPingFocusScan` (Boolean)
    #[serde(default)]
    pub allowed_in_ping_focus_scan: bool,
    /// `allowedInPassiveScan` (Boolean)
    #[serde(default)]
    pub allowed_in_passive_scan: bool,
}

impl Pooled for ScanProcedureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.radarsystem.scan_procedure_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.radarsystem.scan_procedure_params }
}

impl<'a> Extract<'a> for ScanProcedureParams {
    const TYPE_NAME: &'static str = "ScanProcedureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            required_scan_tag: inst.get("requiredScanTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            emission_baseline: inst.get_f32("emissionBaseline").unwrap_or_default(),
            runtime_duration: inst.get_f32("runtimeDuration").unwrap_or_default(),
            allowed_in_aiauto_scan: inst.get_bool("allowedInAIAutoScan").unwrap_or_default(),
            allowed_in_focal_point_scan: inst.get_bool("allowedInFocalPointScan").unwrap_or_default(),
            allowed_in_ping_broad_scan: inst.get_bool("allowedInPingBroadScan").unwrap_or_default(),
            allowed_in_ping_focus_scan: inst.get_bool("allowedInPingFocusScan").unwrap_or_default(),
            allowed_in_passive_scan: inst.get_bool("allowedInPassiveScan").unwrap_or_default(),
        }
    }
}

