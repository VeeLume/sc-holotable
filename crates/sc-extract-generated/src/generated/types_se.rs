// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `SeatOperatorSkills`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatOperatorSkills {
    /// DCB field: `aiming` (Class)
    #[serde(default)]
    pub aiming: Option<Handle<Aiming>>,
    /// DCB field: `burst` (Class)
    #[serde(default)]
    pub burst: Option<Handle<Burst>>,
}

impl Pooled for SeatOperatorSkills {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seat_operator_skills }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seat_operator_skills }
}

impl<'a> Extract<'a> for SeatOperatorSkills {
    const TYPE_NAME: &'static str = "SeatOperatorSkills";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aiming: match inst.get("aiming") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Aiming>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Aiming>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            burst: match inst.get("burst") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Burst>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Burst>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityAudioControllerParams`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityAudioControllerParams {
    /// DCB field: `audioControllerEntityType` (EnumChoice)
    #[serde(default)]
    pub audio_controller_entity_type: String,
    /// DCB field: `randomSeedCount` (Int32)
    #[serde(default)]
    pub random_seed_count: i32,
    /// DCB field: `fullLODDistance` (Single)
    #[serde(default)]
    pub full_loddistance: f32,
    /// DCB field: `fullToLowLODDistance` (Single)
    #[serde(default)]
    pub full_to_low_loddistance: f32,
    /// DCB field: `offToLowLODDistance` (Single)
    #[serde(default)]
    pub off_to_low_loddistance: f32,
    /// DCB field: `offLODDistance` (Single)
    #[serde(default)]
    pub off_loddistance: f32,
    /// DCB field: `occlusionAttenuationScaler` (Single)
    #[serde(default)]
    pub occlusion_attenuation_scaler: f32,
    /// DCB field: `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<TagList>>,
    /// DCB field: `tagTriggers` (Class (array))
    #[serde(default)]
    pub tag_triggers: Vec<Handle<TagTrigger>>,
}

impl Pooled for SEntityAudioControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_audio_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_audio_controller_params }
}

impl<'a> Extract<'a> for SEntityAudioControllerParams {
    const TYPE_NAME: &'static str = "SEntityAudioControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_entity_type: inst.get_str("audioControllerEntityType").map(String::from).unwrap_or_default(),
            random_seed_count: inst.get_i32("randomSeedCount").unwrap_or_default(),
            full_loddistance: inst.get_f32("fullLODDistance").unwrap_or_default(),
            full_to_low_loddistance: inst.get_f32("fullToLowLODDistance").unwrap_or_default(),
            off_to_low_loddistance: inst.get_f32("offToLowLODDistance").unwrap_or_default(),
            off_loddistance: inst.get_f32("offLODDistance").unwrap_or_default(),
            occlusion_attenuation_scaler: inst.get_f32("occlusionAttenuationScaler").unwrap_or_default(),
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tag_triggers: inst.get_array("tagTriggers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagTrigger>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagTrigger>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEAPlayerLoadoutSnapshotEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAPlayerLoadoutSnapshotEntry {
    /// DCB field: `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for SEAPlayerLoadoutSnapshotEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seaplayer_loadout_snapshot_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seaplayer_loadout_snapshot_entry }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshotEntry {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshotEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadout: match inst.get("loadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEAPlayerLoadoutSnapshots`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAPlayerLoadoutSnapshots {
    /// DCB field: `entries` (Class)
    #[serde(default)]
    pub entries: Option<Handle<SEAPlayerLoadoutSnapshotEntry>>,
}

impl Pooled for SEAPlayerLoadoutSnapshots {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seaplayer_loadout_snapshots }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seaplayer_loadout_snapshots }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshots {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshots";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: match inst.get("entries") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEAPlayerLoadoutSnapshotEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEAPlayerLoadoutSnapshotEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEALoadoutAttachment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutAttachment {
    /// DCB field: `attachmentSlot` (EnumChoice)
    #[serde(default)]
    pub attachment_slot: String,
    /// DCB field: `attachementClass` (Reference)
    #[serde(default)]
    pub attachement_class: Option<CigGuid>,
}

impl Pooled for SEALoadoutAttachment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sealoadout_attachment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sealoadout_attachment }
}

impl<'a> Extract<'a> for SEALoadoutAttachment {
    const TYPE_NAME: &'static str = "SEALoadoutAttachment";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attachment_slot: inst.get_str("attachmentSlot").map(String::from).unwrap_or_default(),
            attachement_class: inst.get("attachementClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEALoadoutExplicit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutExplicit {
    /// DCB field: `itemPort` (String)
    #[serde(default)]
    pub item_port: String,
    /// DCB field: `itemClass` (Reference)
    #[serde(default)]
    pub item_class: Option<CigGuid>,
}

impl Pooled for SEALoadoutExplicit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sealoadout_explicit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sealoadout_explicit }
}

impl<'a> Extract<'a> for SEALoadoutExplicit {
    const TYPE_NAME: &'static str = "SEALoadoutExplicit";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_port: inst.get_str("itemPort").map(String::from).unwrap_or_default(),
            item_class: inst.get("itemClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEALoadoutItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutItem {
    /// DCB field: `itemSlot` (EnumChoice)
    #[serde(default)]
    pub item_slot: String,
    /// DCB field: `itemClass` (Reference)
    #[serde(default)]
    pub item_class: Option<CigGuid>,
    /// DCB field: `itemAttachements` (Class (array))
    #[serde(default)]
    pub item_attachements: Vec<Handle<SEALoadoutAttachment>>,
    /// DCB field: `itemExplicit` (Class (array))
    #[serde(default)]
    pub item_explicit: Vec<Handle<SEALoadoutExplicit>>,
}

impl Pooled for SEALoadoutItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sealoadout_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sealoadout_item }
}

impl<'a> Extract<'a> for SEALoadoutItem {
    const TYPE_NAME: &'static str = "SEALoadoutItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_slot: inst.get_str("itemSlot").map(String::from).unwrap_or_default(),
            item_class: inst.get("itemClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            item_attachements: inst.get_array("itemAttachements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutAttachment>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutAttachment>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_explicit: inst.get_array("itemExplicit")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutExplicit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutExplicit>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEALoadoutSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutSet {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `itemSlots` (Class (array))
    #[serde(default)]
    pub item_slots: Vec<Handle<SEALoadoutItem>>,
}

impl Pooled for SEALoadoutSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sealoadout_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sealoadout_set }
}

impl<'a> Extract<'a> for SEALoadoutSet {
    const TYPE_NAME: &'static str = "SEALoadoutSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            item_slots: inst.get_array("itemSlots")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEALoadoutCollection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutCollection {
    /// DCB field: `availableLoadouts` (Class (array))
    #[serde(default)]
    pub available_loadouts: Vec<Handle<SEALoadoutSet>>,
}

impl Pooled for SEALoadoutCollection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sealoadout_collection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sealoadout_collection }
}

impl<'a> Extract<'a> for SEALoadoutCollection {
    const TYPE_NAME: &'static str = "SEALoadoutCollection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            available_loadouts: inst.get_array("availableLoadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEAGlobalSpecialLoadout`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAGlobalSpecialLoadout {
    /// DCB field: `sharedLoadouts` (Class (array))
    #[serde(default)]
    pub shared_loadouts: Vec<Handle<SEALoadoutSet>>,
    /// DCB field: `teamLoadouts` (Class (array))
    #[serde(default)]
    pub team_loadouts: Vec<Handle<SEALoadoutCollection>>,
}

impl Pooled for SEAGlobalSpecialLoadout {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seaglobal_special_loadout }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seaglobal_special_loadout }
}

impl<'a> Extract<'a> for SEAGlobalSpecialLoadout {
    const TYPE_NAME: &'static str = "SEAGlobalSpecialLoadout";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            shared_loadouts: inst.get_array("sharedLoadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            team_loadouts: inst.get_array("teamLoadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutCollection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutCollection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEAGlobalEventLoadouts`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAGlobalEventLoadouts {
    /// DCB field: `loadouts` (Reference)
    #[serde(default)]
    pub loadouts: Option<CigGuid>,
}

impl Pooled for SEAGlobalEventLoadouts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seaglobal_event_loadouts }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seaglobal_event_loadouts }
}

impl<'a> Extract<'a> for SEAGlobalEventLoadouts {
    const TYPE_NAME: &'static str = "SEAGlobalEventLoadouts";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            loadouts: inst.get("loadouts").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEntrancesDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntrancesDef {
    /// DCB field: `boundingBoxScale` (Class)
    #[serde(default)]
    pub bounding_box_scale: Option<Handle<Vec3>>,
    /// DCB field: `boundingBoxOffset` (Class)
    #[serde(default)]
    pub bounding_box_offset: Option<Handle<Vec3>>,
    /// DCB field: `entranceArray` (Class (array))
    #[serde(default)]
    pub entrance_array: Vec<Handle<SCEntranceItem>>,
}

impl Pooled for SEntrancesDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentrances_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentrances_def }
}

impl<'a> Extract<'a> for SEntrancesDef {
    const TYPE_NAME: &'static str = "SEntrancesDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bounding_box_scale: match inst.get("boundingBoxScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bounding_box_offset: match inst.get("boundingBoxOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entrance_array: inst.get_array("entranceArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCEntranceItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCEntranceItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityEffectSystem_Attachment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityEffectSystem_Attachment {
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// DCB field: `offsetParameter` (Reference)
    #[serde(default)]
    pub offset_parameter: Option<CigGuid>,
}

impl Pooled for SEntityEffectSystem_Attachment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_effect_system_attachment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_effect_system_attachment }
}

impl<'a> Extract<'a> for SEntityEffectSystem_Attachment {
    const TYPE_NAME: &'static str = "SEntityEffectSystem_Attachment";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_parameter: inst.get("offsetParameter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEntityEffectSystem_PropertyModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityEffectSystem_PropertyModifier {
}

impl Pooled for SEntityEffectSystem_PropertyModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_effect_system_property_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_effect_system_property_modifier }
}

impl<'a> Extract<'a> for SEntityEffectSystem_PropertyModifier {
    const TYPE_NAME: &'static str = "SEntityEffectSystem_PropertyModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityEffectSystem_ParticleCategory`
///
/// Inherits from: `SEntityEffectSystem_Category` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityEffectSystem_ParticleCategory {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `allowMultipleTags` (Boolean)
    #[serde(default)]
    pub allow_multiple_tags: bool,
    /// DCB field: `tagEffects` (Class (array))
    #[serde(default)]
    pub tag_effects: Vec<Handle<SEntityEffectSystem_ParticleTagEffect>>,
    /// DCB field: `triggerEffects` (Class (array))
    #[serde(default)]
    pub trigger_effects: Vec<Handle<SEntityEffectSystem_ParticleTriggerEffect>>,
}

impl Pooled for SEntityEffectSystem_ParticleCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_effect_system_particle_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_effect_system_particle_category }
}

impl<'a> Extract<'a> for SEntityEffectSystem_ParticleCategory {
    const TYPE_NAME: &'static str = "SEntityEffectSystem_ParticleCategory";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            allow_multiple_tags: inst.get_bool("allowMultipleTags").unwrap_or_default(),
            tag_effects: inst.get_array("tagEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEntityEffectSystem_ParticleTagEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEntityEffectSystem_ParticleTagEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            trigger_effects: inst.get_array("triggerEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEntityEffectSystem_ParticleTriggerEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEntityEffectSystem_ParticleTriggerEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityEffectSystem_ParticleTagEffect`
///
/// Inherits from: `SEntityEffectSystem_ParticleEffect` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityEffectSystem_ParticleTagEffect {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `primed` (Boolean)
    #[serde(default)]
    pub primed: bool,
    /// DCB field: `holographic` (Boolean)
    #[serde(default)]
    pub holographic: bool,
    /// DCB field: `updateEntityBounds` (Boolean)
    #[serde(default)]
    pub update_entity_bounds: bool,
    /// DCB field: `clipToVisArea` (EnumChoice)
    #[serde(default)]
    pub clip_to_vis_area: String,
    /// DCB field: `gpuVisAreaCullingMode` (EnumChoice)
    #[serde(default)]
    pub gpu_vis_area_culling_mode: String,
    /// DCB field: `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `attachmentSettings` (StrongPointer)
    #[serde(default)]
    pub attachment_settings: Option<Handle<SEntityEffectSystem_Attachment>>,
    /// DCB field: `propertyLinks` (Class (array))
    #[serde(default)]
    pub property_links: Vec<Handle<SEntityEffectSystem_ParticlePropertyLink>>,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
}

impl Pooled for SEntityEffectSystem_ParticleTagEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_effect_system_particle_tag_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_effect_system_particle_tag_effect }
}

impl<'a> Extract<'a> for SEntityEffectSystem_ParticleTagEffect {
    const TYPE_NAME: &'static str = "SEntityEffectSystem_ParticleTagEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            primed: inst.get_bool("primed").unwrap_or_default(),
            holographic: inst.get_bool("holographic").unwrap_or_default(),
            update_entity_bounds: inst.get_bool("updateEntityBounds").unwrap_or_default(),
            clip_to_vis_area: inst.get_str("clipToVisArea").map(String::from).unwrap_or_default(),
            gpu_vis_area_culling_mode: inst.get_str("gpuVisAreaCullingMode").map(String::from).unwrap_or_default(),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            attachment_settings: match inst.get("attachmentSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityEffectSystem_Attachment>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityEffectSystem_Attachment>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            property_links: inst.get_array("propertyLinks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEntityEffectSystem_ParticlePropertyLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEntityEffectSystem_ParticlePropertyLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEntityEffectSystem_ParticleTriggerEffect`
///
/// Inherits from: `SEntityEffectSystem_ParticleEffect` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityEffectSystem_ParticleTriggerEffect {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `primed` (Boolean)
    #[serde(default)]
    pub primed: bool,
    /// DCB field: `holographic` (Boolean)
    #[serde(default)]
    pub holographic: bool,
    /// DCB field: `updateEntityBounds` (Boolean)
    #[serde(default)]
    pub update_entity_bounds: bool,
    /// DCB field: `clipToVisArea` (EnumChoice)
    #[serde(default)]
    pub clip_to_vis_area: String,
    /// DCB field: `gpuVisAreaCullingMode` (EnumChoice)
    #[serde(default)]
    pub gpu_vis_area_culling_mode: String,
    /// DCB field: `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `attachmentSettings` (StrongPointer)
    #[serde(default)]
    pub attachment_settings: Option<Handle<SEntityEffectSystem_Attachment>>,
    /// DCB field: `propertyLinks` (Class (array))
    #[serde(default)]
    pub property_links: Vec<Handle<SEntityEffectSystem_ParticlePropertyLink>>,
    /// DCB field: `trigger` (Reference)
    #[serde(default)]
    pub trigger: Option<CigGuid>,
}

impl Pooled for SEntityEffectSystem_ParticleTriggerEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_effect_system_particle_trigger_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_effect_system_particle_trigger_effect }
}

impl<'a> Extract<'a> for SEntityEffectSystem_ParticleTriggerEffect {
    const TYPE_NAME: &'static str = "SEntityEffectSystem_ParticleTriggerEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            primed: inst.get_bool("primed").unwrap_or_default(),
            holographic: inst.get_bool("holographic").unwrap_or_default(),
            update_entity_bounds: inst.get_bool("updateEntityBounds").unwrap_or_default(),
            clip_to_vis_area: inst.get_str("clipToVisArea").map(String::from).unwrap_or_default(),
            gpu_vis_area_culling_mode: inst.get_str("gpuVisAreaCullingMode").map(String::from).unwrap_or_default(),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            attachment_settings: match inst.get("attachmentSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityEffectSystem_Attachment>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityEffectSystem_Attachment>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            property_links: inst.get_array("propertyLinks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEntityEffectSystem_ParticlePropertyLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEntityEffectSystem_ParticlePropertyLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            trigger: inst.get("trigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEntityEffectSystem_ParticlePropertyLink`
///
/// Inherits from: `SEntityEffectSystem_PropertyLink` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityEffectSystem_ParticlePropertyLink {
    /// DCB field: `modifiers` (StrongPointer (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<SEntityEffectSystem_PropertyModifier>>,
    /// DCB field: `parameterName` (Reference)
    #[serde(default)]
    pub parameter_name: Option<CigGuid>,
    /// DCB field: `particleProperty` (EnumChoice)
    #[serde(default)]
    pub particle_property: String,
}

impl Pooled for SEntityEffectSystem_ParticlePropertyLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_effect_system_particle_property_link }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_effect_system_particle_property_link }
}

impl<'a> Extract<'a> for SEntityEffectSystem_ParticlePropertyLink {
    const TYPE_NAME: &'static str = "SEntityEffectSystem_ParticlePropertyLink";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEntityEffectSystem_PropertyModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEntityEffectSystem_PropertyModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            parameter_name: inst.get("parameterName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            particle_property: inst.get_str("particleProperty").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityDensityClass`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityDensityClass {
    /// DCB field: `clusterDetectionRadius` (Single)
    #[serde(default)]
    pub cluster_detection_radius: f32,
    /// DCB field: `clusterUpperObjectCountDGS` (UInt32)
    #[serde(default)]
    pub cluster_upper_object_count_dgs: u32,
    /// DCB field: `clusterUpperObjectCountPersistence` (UInt32)
    #[serde(default)]
    pub cluster_upper_object_count_persistence: u32,
    /// DCB field: `clusterPersistenceTimeout` (Single)
    #[serde(default)]
    pub cluster_persistence_timeout: f32,
    /// DCB field: `entityMaxIdleLifeTime` (StrongPointer)
    #[serde(default)]
    pub entity_max_idle_life_time: Option<Handle<TimeValue_Base>>,
    /// DCB field: `resetLifetimeOnMove` (Boolean)
    #[serde(default)]
    pub reset_lifetime_on_move: bool,
    /// DCB field: `entityIdleBuryOnly` (Boolean)
    #[serde(default)]
    pub entity_idle_bury_only: bool,
}

impl Pooled for SEntityDensityClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_density_class }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_density_class }
}

impl<'a> Extract<'a> for SEntityDensityClass {
    const TYPE_NAME: &'static str = "SEntityDensityClass";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cluster_detection_radius: inst.get_f32("clusterDetectionRadius").unwrap_or_default(),
            cluster_upper_object_count_dgs: inst.get_u32("clusterUpperObjectCountDGS").unwrap_or_default(),
            cluster_upper_object_count_persistence: inst.get_u32("clusterUpperObjectCountPersistence").unwrap_or_default(),
            cluster_persistence_timeout: inst.get_f32("clusterPersistenceTimeout").unwrap_or_default(),
            entity_max_idle_life_time: match inst.get("entityMaxIdleLifeTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TimeValue_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TimeValue_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reset_lifetime_on_move: inst.get_bool("resetLifetimeOnMove").unwrap_or_default(),
            entity_idle_bury_only: inst.get_bool("entityIdleBuryOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityDensityClassOverridesRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityDensityClassOverridesRecord {
    /// DCB field: `overrides` (Class)
    #[serde(default)]
    pub overrides: Option<Handle<SEntityDensityClassOverridesManual>>,
}

impl Pooled for SEntityDensityClassOverridesRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_density_class_overrides_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_density_class_overrides_record }
}

impl<'a> Extract<'a> for SEntityDensityClassOverridesRecord {
    const TYPE_NAME: &'static str = "SEntityDensityClassOverridesRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            overrides: match inst.get("overrides") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityDensityClassOverridesManual>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityDensityClassOverridesManual>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityDensityClassOverridesManual`
///
/// Inherits from: `SEntityDensityClassOverridesBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityDensityClassOverridesManual {
    /// DCB field: `defaults` (StrongPointer)
    #[serde(default)]
    pub defaults: Option<Handle<SDefaultDensityClassLifetimeOverrides>>,
    /// DCB field: `densityClassLifetimeOverrides` (Class (array))
    #[serde(default)]
    pub density_class_lifetime_overrides: Vec<Handle<SDensityClassLifetimeOverrideEntry>>,
}

impl Pooled for SEntityDensityClassOverridesManual {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_density_class_overrides_manual }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_density_class_overrides_manual }
}

impl<'a> Extract<'a> for SEntityDensityClassOverridesManual {
    const TYPE_NAME: &'static str = "SEntityDensityClassOverridesManual";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            defaults: match inst.get("defaults") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDefaultDensityClassLifetimeOverrides>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDefaultDensityClassLifetimeOverrides>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            density_class_lifetime_overrides: inst.get_array("densityClassLifetimeOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SDensityClassLifetimeOverrideEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SDensityClassLifetimeOverrideEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityTraversingNodeId`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityTraversingNodeId {
}

impl Pooled for SEntityTraversingNodeId {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_traversing_node_id }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_traversing_node_id }
}

impl<'a> Extract<'a> for SEntityTraversingNodeId {
    const TYPE_NAME: &'static str = "SEntityTraversingNodeId";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityPhysicsControllerParams`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityPhysicsControllerParams {
    /// DCB field: `PhysType` (StrongPointer)
    #[serde(default)]
    pub phys_type: Option<Handle<SEntityBasePhysicsControllerParams>>,
}

impl Pooled for SEntityPhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_physics_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_physics_controller_params }
}

impl<'a> Extract<'a> for SEntityPhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntityPhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            phys_type: match inst.get("PhysType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityBasePhysicsControllerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityBasePhysicsControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityBasePhysicsControllerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityBasePhysicsControllerParams {
    /// DCB field: `Mass` (Single)
    #[serde(default)]
    pub mass: f32,
    /// DCB field: `compoundingAllowed` (Boolean)
    #[serde(default)]
    pub compounding_allowed: bool,
    /// DCB field: `breakableParams` (StrongPointer)
    #[serde(default)]
    pub breakable_params: Option<Handle<SBreakablePhysicsParams>>,
    /// DCB field: `gameCollisionClass` (StrongPointer)
    #[serde(default)]
    pub game_collision_class: Option<Handle<SGameCollisionClass>>,
    /// DCB field: `spawnBoxScale` (Single)
    #[serde(default)]
    pub spawn_box_scale: f32,
}

impl Pooled for SEntityBasePhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_base_physics_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_base_physics_controller_params }
}

impl<'a> Extract<'a> for SEntityBasePhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntityBasePhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mass: inst.get_f32("Mass").unwrap_or_default(),
            compounding_allowed: inst.get_bool("compoundingAllowed").unwrap_or_default(),
            breakable_params: match inst.get("breakableParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBreakablePhysicsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBreakablePhysicsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_collision_class: match inst.get("gameCollisionClass") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGameCollisionClass>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGameCollisionClass>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spawn_box_scale: inst.get_f32("spawnBoxScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntitySoftExPhysicsControllerParams`
///
/// Inherits from: `SEntityBasePhysicsControllerParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntitySoftExPhysicsControllerParams {
    /// DCB field: `Mass` (Single)
    #[serde(default)]
    pub mass: f32,
    /// DCB field: `compoundingAllowed` (Boolean)
    #[serde(default)]
    pub compounding_allowed: bool,
    /// DCB field: `breakableParams` (StrongPointer)
    #[serde(default)]
    pub breakable_params: Option<Handle<SBreakablePhysicsParams>>,
    /// DCB field: `gameCollisionClass` (StrongPointer)
    #[serde(default)]
    pub game_collision_class: Option<Handle<SGameCollisionClass>>,
    /// DCB field: `spawnBoxScale` (Single)
    #[serde(default)]
    pub spawn_box_scale: f32,
    /// DCB field: `damping` (Single)
    #[serde(default)]
    pub damping: f32,
    /// DCB field: `stretchStiffness` (Single)
    #[serde(default)]
    pub stretch_stiffness: f32,
    /// DCB field: `compressStiffness` (Single)
    #[serde(default)]
    pub compress_stiffness: f32,
    /// DCB field: `bendingStiffness` (Single)
    #[serde(default)]
    pub bending_stiffness: f32,
    /// DCB field: `cosseratShearXStiffness` (Single)
    #[serde(default)]
    pub cosserat_shear_xstiffness: f32,
    /// DCB field: `cosseratShearYStiffness` (Single)
    #[serde(default)]
    pub cosserat_shear_ystiffness: f32,
    /// DCB field: `cosseratStretchStiffness` (Single)
    #[serde(default)]
    pub cosserat_stretch_stiffness: f32,
    /// DCB field: `cosseratBendXStiffness` (Single)
    #[serde(default)]
    pub cosserat_bend_xstiffness: f32,
    /// DCB field: `cosseratBendYStiffness` (Single)
    #[serde(default)]
    pub cosserat_bend_ystiffness: f32,
    /// DCB field: `cosseratTwistingStiffness` (Single)
    #[serde(default)]
    pub cosserat_twisting_stiffness: f32,
    /// DCB field: `attachmentInfluence` (Single)
    #[serde(default)]
    pub attachment_influence: f32,
    /// DCB field: `maxDisplacementInfluence` (Single)
    #[serde(default)]
    pub max_displacement_influence: f32,
    /// DCB field: `maxStretchAttach` (Single)
    #[serde(default)]
    pub max_stretch_attach: f32,
    /// DCB field: `tetraVolStiffness` (Single)
    #[serde(default)]
    pub tetra_vol_stiffness: f32,
    /// DCB field: `collisionGap` (Single)
    #[serde(default)]
    pub collision_gap: f32,
    /// DCB field: `collisionThicknessFactor` (Single)
    #[serde(default)]
    pub collision_thickness_factor: f32,
    /// DCB field: `staticFriction` (Single)
    #[serde(default)]
    pub static_friction: f32,
    /// DCB field: `dynamicFriction` (Single)
    #[serde(default)]
    pub dynamic_friction: f32,
    /// DCB field: `fixedStep` (Single)
    #[serde(default)]
    pub fixed_step: f32,
    /// DCB field: `totalMass` (Single)
    #[serde(default)]
    pub total_mass: f32,
    /// DCB field: `relativeDeltaScale` (Single)
    #[serde(default)]
    pub relative_delta_scale: f32,
    /// DCB field: `bindingOffset` (Single)
    #[serde(default)]
    pub binding_offset: f32,
    /// DCB field: `enforceLength` (Boolean)
    #[serde(default)]
    pub enforce_length: bool,
    /// DCB field: `enforceBending` (Boolean)
    #[serde(default)]
    pub enforce_bending: bool,
    /// DCB field: `enforceCosseratStrechShear` (Boolean)
    #[serde(default)]
    pub enforce_cosserat_strech_shear: bool,
    /// DCB field: `enforceCosseratBendTwist` (Boolean)
    #[serde(default)]
    pub enforce_cosserat_bend_twist: bool,
    /// DCB field: `enforceAttachment` (Boolean)
    #[serde(default)]
    pub enforce_attachment: bool,
    /// DCB field: `enforceMaxDisplacement` (Boolean)
    #[serde(default)]
    pub enforce_max_displacement: bool,
    /// DCB field: `enforceTetraVol` (Boolean)
    #[serde(default)]
    pub enforce_tetra_vol: bool,
    /// DCB field: `enableCollisions` (Boolean)
    #[serde(default)]
    pub enable_collisions: bool,
    /// DCB field: `enableFriction` (Boolean)
    #[serde(default)]
    pub enable_friction: bool,
    /// DCB field: `enableSelfCollision` (Boolean)
    #[serde(default)]
    pub enable_self_collision: bool,
    /// DCB field: `normalizeParticleSize` (Boolean)
    #[serde(default)]
    pub normalize_particle_size: bool,
    /// DCB field: `maxDisplacementRestrictToPositiveHemisphere` (Boolean)
    #[serde(default)]
    pub max_displacement_restrict_to_positive_hemisphere: bool,
    /// DCB field: `iterations` (Int32)
    #[serde(default)]
    pub iterations: i32,
    /// DCB field: `gridDim` (Int32)
    #[serde(default)]
    pub grid_dim: i32,
    /// DCB field: `lift` (Single)
    #[serde(default)]
    pub lift: f32,
    /// DCB field: `drag` (Single)
    #[serde(default)]
    pub drag: f32,
    /// DCB field: `windVariance` (Single)
    #[serde(default)]
    pub wind_variance: f32,
    /// DCB field: `airResistance` (Single)
    #[serde(default)]
    pub air_resistance: f32,
    /// DCB field: `waterResistance` (Single)
    #[serde(default)]
    pub water_resistance: f32,
    /// DCB field: `substepMode` (EnumChoice)
    #[serde(default)]
    pub substep_mode: String,
    /// DCB field: `visualBindingMode` (EnumChoice)
    #[serde(default)]
    pub visual_binding_mode: String,
}

impl Pooled for SEntitySoftExPhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.sentity_soft_ex_physics_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.sentity_soft_ex_physics_controller_params }
}

impl<'a> Extract<'a> for SEntitySoftExPhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntitySoftExPhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mass: inst.get_f32("Mass").unwrap_or_default(),
            compounding_allowed: inst.get_bool("compoundingAllowed").unwrap_or_default(),
            breakable_params: match inst.get("breakableParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBreakablePhysicsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBreakablePhysicsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_collision_class: match inst.get("gameCollisionClass") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGameCollisionClass>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGameCollisionClass>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spawn_box_scale: inst.get_f32("spawnBoxScale").unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            stretch_stiffness: inst.get_f32("stretchStiffness").unwrap_or_default(),
            compress_stiffness: inst.get_f32("compressStiffness").unwrap_or_default(),
            bending_stiffness: inst.get_f32("bendingStiffness").unwrap_or_default(),
            cosserat_shear_xstiffness: inst.get_f32("cosseratShearXStiffness").unwrap_or_default(),
            cosserat_shear_ystiffness: inst.get_f32("cosseratShearYStiffness").unwrap_or_default(),
            cosserat_stretch_stiffness: inst.get_f32("cosseratStretchStiffness").unwrap_or_default(),
            cosserat_bend_xstiffness: inst.get_f32("cosseratBendXStiffness").unwrap_or_default(),
            cosserat_bend_ystiffness: inst.get_f32("cosseratBendYStiffness").unwrap_or_default(),
            cosserat_twisting_stiffness: inst.get_f32("cosseratTwistingStiffness").unwrap_or_default(),
            attachment_influence: inst.get_f32("attachmentInfluence").unwrap_or_default(),
            max_displacement_influence: inst.get_f32("maxDisplacementInfluence").unwrap_or_default(),
            max_stretch_attach: inst.get_f32("maxStretchAttach").unwrap_or_default(),
            tetra_vol_stiffness: inst.get_f32("tetraVolStiffness").unwrap_or_default(),
            collision_gap: inst.get_f32("collisionGap").unwrap_or_default(),
            collision_thickness_factor: inst.get_f32("collisionThicknessFactor").unwrap_or_default(),
            static_friction: inst.get_f32("staticFriction").unwrap_or_default(),
            dynamic_friction: inst.get_f32("dynamicFriction").unwrap_or_default(),
            fixed_step: inst.get_f32("fixedStep").unwrap_or_default(),
            total_mass: inst.get_f32("totalMass").unwrap_or_default(),
            relative_delta_scale: inst.get_f32("relativeDeltaScale").unwrap_or_default(),
            binding_offset: inst.get_f32("bindingOffset").unwrap_or_default(),
            enforce_length: inst.get_bool("enforceLength").unwrap_or_default(),
            enforce_bending: inst.get_bool("enforceBending").unwrap_or_default(),
            enforce_cosserat_strech_shear: inst.get_bool("enforceCosseratStrechShear").unwrap_or_default(),
            enforce_cosserat_bend_twist: inst.get_bool("enforceCosseratBendTwist").unwrap_or_default(),
            enforce_attachment: inst.get_bool("enforceAttachment").unwrap_or_default(),
            enforce_max_displacement: inst.get_bool("enforceMaxDisplacement").unwrap_or_default(),
            enforce_tetra_vol: inst.get_bool("enforceTetraVol").unwrap_or_default(),
            enable_collisions: inst.get_bool("enableCollisions").unwrap_or_default(),
            enable_friction: inst.get_bool("enableFriction").unwrap_or_default(),
            enable_self_collision: inst.get_bool("enableSelfCollision").unwrap_or_default(),
            normalize_particle_size: inst.get_bool("normalizeParticleSize").unwrap_or_default(),
            max_displacement_restrict_to_positive_hemisphere: inst.get_bool("maxDisplacementRestrictToPositiveHemisphere").unwrap_or_default(),
            iterations: inst.get_i32("iterations").unwrap_or_default(),
            grid_dim: inst.get_i32("gridDim").unwrap_or_default(),
            lift: inst.get_f32("lift").unwrap_or_default(),
            drag: inst.get_f32("drag").unwrap_or_default(),
            wind_variance: inst.get_f32("windVariance").unwrap_or_default(),
            air_resistance: inst.get_f32("airResistance").unwrap_or_default(),
            water_resistance: inst.get_f32("waterResistance").unwrap_or_default(),
            substep_mode: inst.get_str("substepMode").map(String::from).unwrap_or_default(),
            visual_binding_mode: inst.get_str("visualBindingMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SeatReticleArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatReticleArchetype {
    /// DCB field: `fixed` (Boolean)
    #[serde(default)]
    pub fixed: bool,
    /// DCB field: `look` (Boolean)
    #[serde(default)]
    pub look: bool,
    /// DCB field: `velocity` (Boolean)
    #[serde(default)]
    pub velocity: bool,
    /// DCB field: `control` (Boolean)
    #[serde(default)]
    pub control: bool,
    /// DCB field: `atmospheric` (Boolean)
    #[serde(default)]
    pub atmospheric: bool,
}

impl Pooled for SeatReticleArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seat_reticle_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seat_reticle_archetype }
}

impl<'a> Extract<'a> for SeatReticleArchetype {
    const TYPE_NAME: &'static str = "SeatReticleArchetype";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fixed: inst.get_bool("fixed").unwrap_or_default(),
            look: inst.get_bool("look").unwrap_or_default(),
            velocity: inst.get_bool("velocity").unwrap_or_default(),
            control: inst.get_bool("control").unwrap_or_default(),
            atmospheric: inst.get_bool("atmospheric").unwrap_or_default(),
        }
    }
}

/// DCB type: `SeatAdsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatAdsDef {
    /// DCB field: `zoomSpeed` (Single)
    #[serde(default)]
    pub zoom_speed: f32,
    /// DCB field: `minFovWithTarget` (Single)
    #[serde(default)]
    pub min_fov_with_target: f32,
    /// DCB field: `minFovWithoutTarget` (Single)
    #[serde(default)]
    pub min_fov_without_target: f32,
    /// DCB field: `minFovStableZoom` (Single)
    #[serde(default)]
    pub min_fov_stable_zoom: f32,
    /// DCB field: `crosshairAngle` (Single)
    #[serde(default)]
    pub crosshair_angle: f32,
    /// DCB field: `maxPitch` (Single)
    #[serde(default)]
    pub max_pitch: f32,
    /// DCB field: `maxYaw` (Single)
    #[serde(default)]
    pub max_yaw: f32,
    /// DCB field: `timeToKeepZoomAfterTargetLoss` (Single)
    #[serde(default)]
    pub time_to_keep_zoom_after_target_loss: f32,
    /// DCB field: `timeToKeepZoomAfterHoverTargetLoss` (Single)
    #[serde(default)]
    pub time_to_keep_zoom_after_hover_target_loss: f32,
    /// DCB field: `timeToDisallowZoomAfterHoverTargetGain` (Single)
    #[serde(default)]
    pub time_to_disallow_zoom_after_hover_target_gain: f32,
    /// DCB field: `trackSubtargets` (Boolean)
    #[serde(default)]
    pub track_subtargets: bool,
    /// DCB field: `useScalingOutsideAds` (Boolean)
    #[serde(default)]
    pub use_scaling_outside_ads: bool,
    /// DCB field: `allowedForOperatorMode` (Boolean)
    #[serde(default)]
    pub allowed_for_operator_mode: bool,
    /// DCB field: `recoilFovCurve` (Class)
    #[serde(default)]
    pub recoil_fov_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SeatAdsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seat_ads_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seat_ads_def }
}

impl<'a> Extract<'a> for SeatAdsDef {
    const TYPE_NAME: &'static str = "SeatAdsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zoom_speed: inst.get_f32("zoomSpeed").unwrap_or_default(),
            min_fov_with_target: inst.get_f32("minFovWithTarget").unwrap_or_default(),
            min_fov_without_target: inst.get_f32("minFovWithoutTarget").unwrap_or_default(),
            min_fov_stable_zoom: inst.get_f32("minFovStableZoom").unwrap_or_default(),
            crosshair_angle: inst.get_f32("crosshairAngle").unwrap_or_default(),
            max_pitch: inst.get_f32("maxPitch").unwrap_or_default(),
            max_yaw: inst.get_f32("maxYaw").unwrap_or_default(),
            time_to_keep_zoom_after_target_loss: inst.get_f32("timeToKeepZoomAfterTargetLoss").unwrap_or_default(),
            time_to_keep_zoom_after_hover_target_loss: inst.get_f32("timeToKeepZoomAfterHoverTargetLoss").unwrap_or_default(),
            time_to_disallow_zoom_after_hover_target_gain: inst.get_f32("timeToDisallowZoomAfterHoverTargetGain").unwrap_or_default(),
            track_subtargets: inst.get_bool("trackSubtargets").unwrap_or_default(),
            use_scaling_outside_ads: inst.get_bool("useScalingOutsideAds").unwrap_or_default(),
            allowed_for_operator_mode: inst.get_bool("allowedForOperatorMode").unwrap_or_default(),
            recoil_fov_curve: match inst.get("recoilFovCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SeatUserActorCDIKConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatUserActorCDIKConfig {
    /// DCB field: `cdikTargetName` (String)
    #[serde(default)]
    pub cdik_target_name: String,
    /// DCB field: `IKLimbHandle` (String)
    #[serde(default)]
    pub iklimb_handle: String,
    /// DCB field: `parentJointName` (String)
    #[serde(default)]
    pub parent_joint_name: String,
    /// DCB field: `cdikTargetOffset` (Class)
    #[serde(default)]
    pub cdik_target_offset: Option<Handle<QuatT>>,
    /// DCB field: `userCDIKReferenceJoint` (String)
    #[serde(default)]
    pub user_cdikreference_joint: String,
}

impl Pooled for SeatUserActorCDIKConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seat_user_actor_cdikconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seat_user_actor_cdikconfig }
}

impl<'a> Extract<'a> for SeatUserActorCDIKConfig {
    const TYPE_NAME: &'static str = "SeatUserActorCDIKConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cdik_target_name: inst.get_str("cdikTargetName").map(String::from).unwrap_or_default(),
            iklimb_handle: inst.get_str("IKLimbHandle").map(String::from).unwrap_or_default(),
            parent_joint_name: inst.get_str("parentJointName").map(String::from).unwrap_or_default(),
            cdik_target_offset: match inst.get("cdikTargetOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            user_cdikreference_joint: inst.get_str("userCDIKReferenceJoint").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SeatUserActorCDIKMapping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatUserActorCDIKMapping {
    /// DCB field: `userSkeleton` (Reference)
    #[serde(default)]
    pub user_skeleton: Option<CigGuid>,
    /// DCB field: `defaultCDIKTargets` (Class (array))
    #[serde(default)]
    pub default_cdiktargets: Vec<Handle<SeatUserActorCDIKConfig>>,
}

impl Pooled for SeatUserActorCDIKMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seat_user_actor_cdikmapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seat_user_actor_cdikmapping }
}

impl<'a> Extract<'a> for SeatUserActorCDIKMapping {
    const TYPE_NAME: &'static str = "SeatUserActorCDIKMapping";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            user_skeleton: inst.get("userSkeleton").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_cdiktargets: inst.get_array("defaultCDIKTargets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SeatUserActorCDIKConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SeatUserActorCDIKConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SeatUserActorCDIKRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatUserActorCDIKRecord {
    /// DCB field: `filters` (Class (array))
    #[serde(default)]
    pub filters: Vec<Handle<SeatUserActorCDIKMapping>>,
}

impl Pooled for SeatUserActorCDIKRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.seat_user_actor_cdikrecord }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.seat_user_actor_cdikrecord }
}

impl<'a> Extract<'a> for SeatUserActorCDIKRecord {
    const TYPE_NAME: &'static str = "SeatUserActorCDIKRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filters: inst.get_array("filters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SeatUserActorCDIKMapping>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SeatUserActorCDIKMapping>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SecurityNetworkPermissions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkPermissions {
    /// DCB field: `access` (EnumChoice)
    #[serde(default)]
    pub access: String,
    /// DCB field: `trespass` (EnumChoice)
    #[serde(default)]
    pub trespass: String,
}

impl Pooled for SecurityNetworkPermissions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_permissions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_permissions }
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
    /// DCB field: `conditionFailedTag` (Reference)
    #[serde(default)]
    pub condition_failed_tag: Option<CigGuid>,
}

impl Pooled for SecurityClearanceTokenData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_clearance_token_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_clearance_token_data }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_manual_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_manual_input }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_notifications }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_notifications }
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
    /// DCB field: `customCondition` (StrongPointer)
    #[serde(default)]
    pub custom_condition: Option<Handle<SecurityClearanceTokenData>>,
    /// DCB field: `manualInput` (StrongPointer)
    #[serde(default)]
    pub manual_input: Option<Handle<SecurityManualInput>>,
    /// DCB field: `missionNote` (StrongPointer)
    #[serde(default)]
    pub mission_note: Option<Handle<MobiGlasMissionNote>>,
    /// DCB field: `notifications` (StrongPointer)
    #[serde(default)]
    pub notifications: Option<Handle<SecurityNotifications>>,
}

impl Pooled for SecurityClearanceToken {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_clearance_token }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_clearance_token }
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
    /// DCB field: `tokens` (Reference (array))
    #[serde(default)]
    pub tokens: Vec<CigGuid>,
}

impl Pooled for SecurityClearanceConditions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_clearance_conditions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_clearance_conditions }
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
    /// DCB field: `permissionsWithClearance` (Class)
    #[serde(default)]
    pub permissions_with_clearance: Option<Handle<SecurityNetworkPermissions>>,
    /// DCB field: `permissionsWithoutClearance` (Class)
    #[serde(default)]
    pub permissions_without_clearance: Option<Handle<SecurityNetworkPermissions>>,
    /// DCB field: `acceptedClearance` (Class)
    #[serde(default)]
    pub accepted_clearance: Option<Handle<SecurityClearanceConditions>>,
    /// DCB field: `ownerFaction` (Reference)
    #[serde(default)]
    pub owner_faction: Option<CigGuid>,
}

impl Pooled for SecurityNetworkProtocol {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_protocol }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_protocol }
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
    /// DCB field: `defaultProtocol` (Class)
    #[serde(default)]
    pub default_protocol: Option<Handle<SecurityNetworkProtocol>>,
    /// DCB field: `roomIdentifier` (Reference)
    #[serde(default)]
    pub room_identifier: Option<CigGuid>,
    /// DCB field: `trespassWarningSeconds` (Single)
    #[serde(default)]
    pub trespass_warning_seconds: f32,
    /// DCB field: `trespassRevokeWarningSeconds` (Single)
    #[serde(default)]
    pub trespass_revoke_warning_seconds: f32,
}

impl Pooled for SecurityNetworkRoomSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_room_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_room_settings }
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
    /// DCB field: `roomIdentifiers` (Class)
    #[serde(default)]
    pub room_identifiers: Option<Handle<TagList>>,
    /// DCB field: `protocol` (Class)
    #[serde(default)]
    pub protocol: Option<Handle<SecurityNetworkProtocol>>,
}

impl Pooled for SecurityNetworkProtocolOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_protocol_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_protocol_override }
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
    /// DCB field: `defaultProtocol` (Class)
    #[serde(default)]
    pub default_protocol: Option<Handle<SecurityNetworkProtocol>>,
    /// DCB field: `roomProtocols` (Class (array))
    #[serde(default)]
    pub room_protocols: Vec<Handle<SecurityNetworkProtocolOverride>>,
    /// DCB field: `variables` (Class (array))
    #[serde(default)]
    pub variables: Vec<Handle<SecurityNetworkVariable>>,
    /// DCB field: `hostilityRules` (Class (array))
    #[serde(default)]
    pub hostility_rules: Vec<Handle<SHostilityRules>>,
    /// DCB field: `variableEffects` (Class (array))
    #[serde(default)]
    pub variable_effects: Vec<Handle<SecurityNetworkVariableEffects>>,
    /// DCB field: `isNeutralTerritory` (Boolean)
    #[serde(default)]
    pub is_neutral_territory: bool,
    /// DCB field: `teleportPlayerOut` (Boolean)
    #[serde(default)]
    pub teleport_player_out: bool,
}

impl Pooled for SecurityNetworkManifest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_manifest }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_manifest }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_variable_value_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_variable_value_base }
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
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `value` (StrongPointer)
    #[serde(default)]
    pub value: Option<Handle<SecurityNetworkVariableValue_Base>>,
}

impl Pooled for SecurityNetworkVariable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_variable }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_variable }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_variable_effect_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_variable_effect_base }
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
    /// DCB field: `variableValue` (Class)
    #[serde(default)]
    pub variable_value: Option<Handle<SecurityNetworkVariable>>,
    /// DCB field: `effects` (StrongPointer (array))
    #[serde(default)]
    pub effects: Vec<Handle<SecurityNetworkVariableEffect_Base>>,
}

impl Pooled for SecurityNetworkVariableEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.security_network_variable_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.security_network_variable_effects }
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

/// DCB type: `ServiceBeaconBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconBaseParams {
    /// DCB field: `serviceBeaconType` (EnumChoice)
    #[serde(default)]
    pub service_beacon_type: String,
    /// DCB field: `serviceBeaconName` (Locale)
    #[serde(default)]
    pub service_beacon_name: String,
    /// DCB field: `beaconTaxPercentage` (Int32)
    #[serde(default)]
    pub beacon_tax_percentage: i32,
    /// DCB field: `beaconMaxPaymentAmount` (Int32)
    #[serde(default)]
    pub beacon_max_payment_amount: i32,
    /// DCB field: `npcRequesterNameDef` (Class)
    #[serde(default)]
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// DCB field: `playerCreatorParams` (StrongPointer)
    #[serde(default)]
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// DCB field: `npcCreatorParams` (StrongPointer)
    #[serde(default)]
    pub npc_creator_params: Option<Handle<SServiceBeaconCreatorParamsBase>>,
}

impl Pooled for ServiceBeaconBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.service_beacon_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.service_beacon_base_params }
}

impl<'a> Extract<'a> for ServiceBeaconBaseParams {
    const TYPE_NAME: &'static str = "ServiceBeaconBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: inst.get_str("serviceBeaconType").map(String::from).unwrap_or_default(),
            service_beacon_name: inst.get_str("serviceBeaconName").map(String::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SServiceBeaconCreatorParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ServiceBeaconParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconParams {
    /// DCB field: `params` (StrongPointer)
    #[serde(default)]
    pub params: Option<Handle<ServiceBeaconBaseParams>>,
}

impl Pooled for ServiceBeaconParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.service_beacon_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.service_beacon_params }
}

impl<'a> Extract<'a> for ServiceBeaconParams {
    const TYPE_NAME: &'static str = "ServiceBeaconParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ServiceBeaconNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconNotificationParams {
    /// DCB field: `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// DCB field: `screenTimer` (Single)
    #[serde(default)]
    pub screen_timer: f32,
    /// DCB field: `hurryScreenTimer` (Single)
    #[serde(default)]
    pub hurry_screen_timer: f32,
    /// DCB field: `blocking` (Boolean)
    #[serde(default)]
    pub blocking: bool,
    /// DCB field: `dockNotificationParamsOverride` (Reference)
    #[serde(default)]
    pub dock_notification_params_override: Option<CigGuid>,
}

impl Pooled for ServiceBeaconNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.service_beacon_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.service_beacon_notification_params }
}

impl<'a> Extract<'a> for ServiceBeaconNotificationParams {
    const TYPE_NAME: &'static str = "ServiceBeaconNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            screen_timer: inst.get_f32("screenTimer").unwrap_or_default(),
            hurry_screen_timer: inst.get_f32("hurryScreenTimer").unwrap_or_default(),
            blocking: inst.get_bool("blocking").unwrap_or_default(),
            dock_notification_params_override: inst.get("dockNotificationParamsOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ServiceBeaconGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconGlobalParams {
    /// DCB field: `quantumTravelPointClass` (Reference)
    #[serde(default)]
    pub quantum_travel_point_class: Option<CigGuid>,
    /// DCB field: `missionTypeRecord` (Reference)
    #[serde(default)]
    pub mission_type_record: Option<CigGuid>,
    /// DCB field: `personalTransportDetectedNotification` (Class)
    #[serde(default)]
    pub personal_transport_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `combatAssistanceDetectedNotification` (Class)
    #[serde(default)]
    pub combat_assistance_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `escortDetectedNotification` (Class)
    #[serde(default)]
    pub escort_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `refuelDetectedNotification` (Class)
    #[serde(default)]
    pub refuel_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `reviveDetectedNotification` (Class)
    #[serde(default)]
    pub revive_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `healDetectedNotification` (Class)
    #[serde(default)]
    pub heal_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `contractAcceptedNotification` (Class)
    #[serde(default)]
    pub contract_accepted_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `contractCancelledNotification` (Class)
    #[serde(default)]
    pub contract_cancelled_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `contractNoLongerAvailableNotification` (Class)
    #[serde(default)]
    pub contract_no_longer_available_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `contractCompletedInitiatorNotification` (Class)
    #[serde(default)]
    pub contract_completed_initiator_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `contractCompletedProviderNotification` (Class)
    #[serde(default)]
    pub contract_completed_provider_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `providerWithinRangeNotification` (Class)
    #[serde(default)]
    pub provider_within_range_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `spoofedContractOfferedNotification` (Class)
    #[serde(default)]
    pub spoofed_contract_offered_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// DCB field: `contractProviderName` (Locale)
    #[serde(default)]
    pub contract_provider_name: String,
    /// DCB field: `vehicleLocationChosenForPersonalTransport` (Locale)
    #[serde(default)]
    pub vehicle_location_chosen_for_personal_transport: String,
    /// DCB field: `providerNameToken` (String)
    #[serde(default)]
    pub provider_name_token: String,
    /// DCB field: `initiatorNameToken` (String)
    #[serde(default)]
    pub initiator_name_token: String,
    /// DCB field: `selectedDestinationToken` (String)
    #[serde(default)]
    pub selected_destination_token: String,
    /// DCB field: `contractTypeToken` (String)
    #[serde(default)]
    pub contract_type_token: String,
    /// DCB field: `distanceToInitiatorToken` (String)
    #[serde(default)]
    pub distance_to_initiator_token: String,
    /// DCB field: `initiatorLocationToken` (String)
    #[serde(default)]
    pub initiator_location_token: String,
    /// DCB field: `paymentAmountToken` (String)
    #[serde(default)]
    pub payment_amount_token: String,
    /// DCB field: `openSpaceLocationName` (Locale)
    #[serde(default)]
    pub open_space_location_name: String,
    /// DCB field: `allReputationsLabel` (Locale)
    #[serde(default)]
    pub all_reputations_label: String,
    /// DCB field: `oneStarReputationLabel` (Locale)
    #[serde(default)]
    pub one_star_reputation_label: String,
    /// DCB field: `twoStarReputationLabel` (Locale)
    #[serde(default)]
    pub two_star_reputation_label: String,
    /// DCB field: `threeStarReputationLabel` (Locale)
    #[serde(default)]
    pub three_star_reputation_label: String,
    /// DCB field: `fourStarReputationLabel` (Locale)
    #[serde(default)]
    pub four_star_reputation_label: String,
    /// DCB field: `fiveStarReputationLabel` (Locale)
    #[serde(default)]
    pub five_star_reputation_label: String,
    /// DCB field: `invalidTypeErrorMessage` (Locale)
    #[serde(default)]
    pub invalid_type_error_message: String,
    /// DCB field: `invalidReputationErrorMessage` (Locale)
    #[serde(default)]
    pub invalid_reputation_error_message: String,
    /// DCB field: `priceIsZeroErrorMessage` (Locale)
    #[serde(default)]
    pub price_is_zero_error_message: String,
    /// DCB field: `insufficientFundsErrorMessage` (Locale)
    #[serde(default)]
    pub insufficient_funds_error_message: String,
    /// DCB field: `invalidLocationSelectedErrorMessage` (Locale)
    #[serde(default)]
    pub invalid_location_selected_error_message: String,
}

impl Pooled for ServiceBeaconGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_se.service_beacon_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_se.service_beacon_global_params }
}

impl<'a> Extract<'a> for ServiceBeaconGlobalParams {
    const TYPE_NAME: &'static str = "ServiceBeaconGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            quantum_travel_point_class: inst.get("quantumTravelPointClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_type_record: inst.get("missionTypeRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            personal_transport_detected_notification: match inst.get("personalTransportDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            combat_assistance_detected_notification: match inst.get("combatAssistanceDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            escort_detected_notification: match inst.get("escortDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            refuel_detected_notification: match inst.get("refuelDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            revive_detected_notification: match inst.get("reviveDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            heal_detected_notification: match inst.get("healDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_accepted_notification: match inst.get("contractAcceptedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_cancelled_notification: match inst.get("contractCancelledNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_no_longer_available_notification: match inst.get("contractNoLongerAvailableNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_completed_initiator_notification: match inst.get("contractCompletedInitiatorNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_completed_provider_notification: match inst.get("contractCompletedProviderNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            provider_within_range_notification: match inst.get("providerWithinRangeNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spoofed_contract_offered_notification: match inst.get("spoofedContractOfferedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_provider_name: inst.get_str("contractProviderName").map(String::from).unwrap_or_default(),
            vehicle_location_chosen_for_personal_transport: inst.get_str("vehicleLocationChosenForPersonalTransport").map(String::from).unwrap_or_default(),
            provider_name_token: inst.get_str("providerNameToken").map(String::from).unwrap_or_default(),
            initiator_name_token: inst.get_str("initiatorNameToken").map(String::from).unwrap_or_default(),
            selected_destination_token: inst.get_str("selectedDestinationToken").map(String::from).unwrap_or_default(),
            contract_type_token: inst.get_str("contractTypeToken").map(String::from).unwrap_or_default(),
            distance_to_initiator_token: inst.get_str("distanceToInitiatorToken").map(String::from).unwrap_or_default(),
            initiator_location_token: inst.get_str("initiatorLocationToken").map(String::from).unwrap_or_default(),
            payment_amount_token: inst.get_str("paymentAmountToken").map(String::from).unwrap_or_default(),
            open_space_location_name: inst.get_str("openSpaceLocationName").map(String::from).unwrap_or_default(),
            all_reputations_label: inst.get_str("allReputationsLabel").map(String::from).unwrap_or_default(),
            one_star_reputation_label: inst.get_str("oneStarReputationLabel").map(String::from).unwrap_or_default(),
            two_star_reputation_label: inst.get_str("twoStarReputationLabel").map(String::from).unwrap_or_default(),
            three_star_reputation_label: inst.get_str("threeStarReputationLabel").map(String::from).unwrap_or_default(),
            four_star_reputation_label: inst.get_str("fourStarReputationLabel").map(String::from).unwrap_or_default(),
            five_star_reputation_label: inst.get_str("fiveStarReputationLabel").map(String::from).unwrap_or_default(),
            invalid_type_error_message: inst.get_str("invalidTypeErrorMessage").map(String::from).unwrap_or_default(),
            invalid_reputation_error_message: inst.get_str("invalidReputationErrorMessage").map(String::from).unwrap_or_default(),
            price_is_zero_error_message: inst.get_str("priceIsZeroErrorMessage").map(String::from).unwrap_or_default(),
            insufficient_funds_error_message: inst.get_str("insufficientFundsErrorMessage").map(String::from).unwrap_or_default(),
            invalid_location_selected_error_message: inst.get_str("invalidLocationSelectedErrorMessage").map(String::from).unwrap_or_default(),
        }
    }
}

