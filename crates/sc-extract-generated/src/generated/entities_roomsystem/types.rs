// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-roomsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `FireAreaHazards`
/// Inherits from: `INavigationCostVolumeExtender`
pub struct FireAreaHazards {
    /// `overrideConfig` (Reference)
    pub override_config: Option<CigGuid>,
}

impl Pooled for FireAreaHazards {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.fire_area_hazards }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.fire_area_hazards }
}

impl<'a> Extract<'a> for FireAreaHazards {
    const TYPE_NAME: &'static str = "FireAreaHazards";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            override_config: inst.get("overrideConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `CIGAudioParams`
/// Inherits from: `DataForgeComponentParams`
pub struct CIGAudioParams {
    /// `namingStrategy` (EnumChoice)
    pub naming_strategy: CIGAudioContextNamingStrategy,
    /// `boneNames` (String (array))
    pub bone_names: Vec<String>,
    /// `enableAnimStart` (Boolean)
    pub enable_anim_start: bool,
}

impl Pooled for CIGAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.cigaudio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.cigaudio_params }
}

impl<'a> Extract<'a> for CIGAudioParams {
    const TYPE_NAME: &'static str = "CIGAudioParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            naming_strategy: CIGAudioContextNamingStrategy::from_dcb_str(inst.get_str("namingStrategy").unwrap_or("")),
            bone_names: inst.get_array("boneNames")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            enable_anim_start: inst.get_bool("enableAnimStart").unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityComponentFireArea`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentFireArea {
    /// `repairRate` (Single)
    pub repair_rate: f32,
    /// `combustibility` (Single)
    pub combustibility: f32,
}

impl Pooled for EntityComponentFireArea {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.entity_component_fire_area }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.entity_component_fire_area }
}

impl<'a> Extract<'a> for EntityComponentFireArea {
    const TYPE_NAME: &'static str = "EntityComponentFireArea";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            repair_rate: inst.get_f32("repairRate").unwrap_or_default(),
            combustibility: inst.get_f32("combustibility").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireVoxelSelectionShape_Box`
/// Inherits from: `FireVoxelSelectionShape`
pub struct FireVoxelSelectionShape_Box {
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
}

impl Pooled for FireVoxelSelectionShape_Box {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.fire_voxel_selection_shape_box }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.fire_voxel_selection_shape_box }
}

impl<'a> Extract<'a> for FireVoxelSelectionShape_Box {
    const TYPE_NAME: &'static str = "FireVoxelSelectionShape_Box";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityComponentFireFilter`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentFireFilter {
    /// `mode` (EnumChoice)
    pub mode: FireFilterMode,
    /// `shape` (StrongPointer)
    pub shape: Option<FireVoxelSelectionShapePtr>,
}

impl Pooled for EntityComponentFireFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.entity_component_fire_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.entity_component_fire_filter }
}

impl<'a> Extract<'a> for EntityComponentFireFilter {
    const TYPE_NAME: &'static str = "EntityComponentFireFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mode: FireFilterMode::from_dcb_str(inst.get_str("mode").unwrap_or("")),
            shape: match inst.get("shape") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(FireVoxelSelectionShapePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityComponentExtinguisher`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentExtinguisher {
    /// `enable` (Boolean)
    pub enable: bool,
    /// `strength` (Single)
    pub strength: f32,
    /// `type` (StrongPointer)
    pub r#type: Option<ExtinguishType_BasePtr>,
    /// `extinguishingEffectOverride` (Class)
    pub extinguishing_effect_override: Option<Handle<GlobalResourceParticle>>,
    /// `vectorFieldRadius` (Single)
    pub vector_field_radius: f32,
    /// `vectorFieldWidth` (Single)
    pub vector_field_width: f32,
    /// `vectorFieldFalloff` (Single)
    pub vector_field_falloff: f32,
    /// `vectorFieldMaxStrength` (Single)
    pub vector_field_max_strength: f32,
}

impl Pooled for EntityComponentExtinguisher {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.entity_component_extinguisher }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.entity_component_extinguisher }
}

impl<'a> Extract<'a> for EntityComponentExtinguisher {
    const TYPE_NAME: &'static str = "EntityComponentExtinguisher";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            strength: inst.get_f32("strength").unwrap_or_default(),
            r#type: match inst.get("type") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ExtinguishType_BasePtr::from_ref(b, r)),
                _ => None,
            },
            extinguishing_effect_override: match inst.get("extinguishingEffectOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            vector_field_radius: inst.get_f32("vectorFieldRadius").unwrap_or_default(),
            vector_field_width: inst.get_f32("vectorFieldWidth").unwrap_or_default(),
            vector_field_falloff: inst.get_f32("vectorFieldFalloff").unwrap_or_default(),
            vector_field_max_strength: inst.get_f32("vectorFieldMaxStrength").unwrap_or_default(),
        }
    }
}

/// DCB type: `ExtinguishType_Spray`
/// Inherits from: `ExtinguishType_Base`
pub struct ExtinguishType_Spray {
    /// `maximumDistance` (Single)
    pub maximum_distance: f32,
    /// `coneAngle` (Single)
    pub cone_angle: f32,
}

impl Pooled for ExtinguishType_Spray {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.extinguish_type_spray }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.extinguish_type_spray }
}

impl<'a> Extract<'a> for ExtinguishType_Spray {
    const TYPE_NAME: &'static str = "ExtinguishType_Spray";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            maximum_distance: inst.get_f32("maximumDistance").unwrap_or_default(),
            cone_angle: inst.get_f32("coneAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityComponentFireRepairer`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentFireRepairer {
    /// `enable` (Boolean)
    pub enable: bool,
    /// `repairRate` (Single)
    pub repair_rate: f32,
    /// `radius` (Single)
    pub radius: f32,
    /// `type` (StrongPointer)
    pub r#type: Option<FireRepairerType_BasePtr>,
}

impl Pooled for EntityComponentFireRepairer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.entity_component_fire_repairer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.entity_component_fire_repairer }
}

impl<'a> Extract<'a> for EntityComponentFireRepairer {
    const TYPE_NAME: &'static str = "EntityComponentFireRepairer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            repair_rate: inst.get_f32("repairRate").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            r#type: match inst.get("type") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(FireRepairerType_BasePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireRepairerType_EntityPos`
/// Inherits from: `FireRepairerType_Base`
pub struct FireRepairerType_EntityPos {
}

impl Pooled for FireRepairerType_EntityPos {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.fire_repairer_type_entity_pos }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.fire_repairer_type_entity_pos }
}

impl<'a> Extract<'a> for FireRepairerType_EntityPos {
    const TYPE_NAME: &'static str = "FireRepairerType_EntityPos";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityTemperatureStateModifier`
/// Inherits from: `SRangeStateModifier`
pub struct EntityTemperatureStateModifier {
    /// `stateRanges` (Class (array))
    pub state_ranges: Vec<Handle<SRangeStateLevel>>,
}

impl Pooled for EntityTemperatureStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.entity_temperature_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.entity_temperature_state_modifier }
}

impl<'a> Extract<'a> for EntityTemperatureStateModifier {
    const TYPE_NAME: &'static str = "EntityTemperatureStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_ranges: inst.get_array("stateRanges")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SRangeStateLevel>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SRangeStateLevel>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceDynamicAmountLifeSupport`
/// Inherits from: `ItemResourceDynamicAmountBase`
pub struct ItemResourceDynamicAmountLifeSupport {
}

impl Pooled for ItemResourceDynamicAmountLifeSupport {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.item_resource_dynamic_amount_life_support }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.item_resource_dynamic_amount_life_support }
}

impl<'a> Extract<'a> for ItemResourceDynamicAmountLifeSupport {
    const TYPE_NAME: &'static str = "ItemResourceDynamicAmountLifeSupport";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityComponentRoomGroupParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentRoomGroupParams {
}

impl Pooled for SEntityComponentRoomGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.sentity_component_room_group_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.sentity_component_room_group_params }
}

impl<'a> Extract<'a> for SEntityComponentRoomGroupParams {
    const TYPE_NAME: &'static str = "SEntityComponentRoomGroupParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityComponentRoomFadeVolumeParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentRoomFadeVolumeParams {
    /// `areaVolume` (StrongPointer)
    pub area_volume: Option<VolumeShapePtr>,
    /// `fadeZone` (Single)
    pub fade_zone: f32,
}

impl Pooled for EntityComponentRoomFadeVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.entity_component_room_fade_volume_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.entity_component_room_fade_volume_params }
}

impl<'a> Extract<'a> for EntityComponentRoomFadeVolumeParams {
    const TYPE_NAME: &'static str = "EntityComponentRoomFadeVolumeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            area_volume: match inst.get("areaVolume") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(VolumeShapePtr::from_ref(b, r)),
                _ => None,
            },
            fade_zone: inst.get_f32("fadeZone").unwrap_or_default(),
        }
    }
}

/// DCB type: `VolumeShape_Sphere`
/// Inherits from: `VolumeShape`
pub struct VolumeShape_Sphere {
    /// `radius` (Single)
    pub radius: f32,
}

impl Pooled for VolumeShape_Sphere {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.volume_shape_sphere }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.volume_shape_sphere }
}

impl<'a> Extract<'a> for VolumeShape_Sphere {
    const TYPE_NAME: &'static str = "VolumeShape_Sphere";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAtmosphericCompositionInherit`
/// Inherits from: `SAtmosphericCompositionBaseParams`
pub struct SAtmosphericCompositionInherit {
}

impl Pooled for SAtmosphericCompositionInherit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.satmospheric_composition_inherit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.satmospheric_composition_inherit }
}

impl<'a> Extract<'a> for SAtmosphericCompositionInherit {
    const TYPE_NAME: &'static str = "SAtmosphericCompositionInherit";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AsteroidState`
/// Inherits from: `AsteroidStateBase`
pub struct AsteroidState {
    /// `debrisDensityMod` (EnumChoice)
    pub debris_density_mod: RoomStateModifyType,
    /// `debrisDensity` (Single)
    pub debris_density: f32,
}

impl Pooled for AsteroidState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_roomsystem.asteroid_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_roomsystem.asteroid_state }
}

impl<'a> Extract<'a> for AsteroidState {
    const TYPE_NAME: &'static str = "AsteroidState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debris_density_mod: RoomStateModifyType::from_dcb_str(inst.get_str("debrisDensityMod").unwrap_or("")),
            debris_density: inst.get_f32("debrisDensity").unwrap_or_default(),
        }
    }
}

