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

/// DCB type: `SGroupedLoadouts`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGroupedLoadouts {
    /// DCB field: `Group` (EnumChoice)
    #[serde(default)]
    pub group: String,
    /// DCB field: `Requirement` (StrongPointer)
    #[serde(default)]
    pub requirement: Option<Handle<SLoadoutRequirementBase>>,
    /// DCB field: `Loadouts` (StrongPointer (array))
    #[serde(default)]
    pub loadouts: Vec<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for SGroupedLoadouts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgrouped_loadouts }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgrouped_loadouts }
}

impl<'a> Extract<'a> for SGroupedLoadouts {
    const TYPE_NAME: &'static str = "SGroupedLoadouts";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            group: inst.get_str("Group").map(String::from).unwrap_or_default(),
            requirement: match inst.get("Requirement") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLoadoutRequirementBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SLoadoutRequirementBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loadouts: inst.get_array("Loadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SGeometryViewDistanceRatioCategories`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryViewDistanceRatioCategories {
    /// DCB field: `categories` (Reference (array))
    #[serde(default)]
    pub categories: Vec<CigGuid>,
}

impl Pooled for SGeometryViewDistanceRatioCategories {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgeometry_view_distance_ratio_categories }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgeometry_view_distance_ratio_categories }
}

impl<'a> Extract<'a> for SGeometryViewDistanceRatioCategories {
    const TYPE_NAME: &'static str = "SGeometryViewDistanceRatioCategories";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SGeometryDataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryDataParams {
    /// DCB field: `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `Material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `SimulationGeometry` (StrongPointer)
    #[serde(default)]
    pub simulation_geometry: Option<Handle<SSoftbodyGeometryParams>>,
    /// DCB field: `Palette` (Class)
    #[serde(default)]
    pub palette: Option<Handle<TintPaletteRef>>,
    /// DCB field: `Slot` (EnumChoice)
    #[serde(default)]
    pub slot: String,
    /// DCB field: `MaterialAttachments` (String)
    #[serde(default)]
    pub material_attachments: String,
    /// DCB field: `ProxyCDFPath` (String)
    #[serde(default)]
    pub proxy_cdfpath: String,
    /// DCB field: `ModifiersPath` (String)
    #[serde(default)]
    pub modifiers_path: String,
    /// DCB field: `AttachFlags` (UInt32)
    #[serde(default)]
    pub attach_flags: u32,
    /// DCB field: `DeformerType` (EnumChoice)
    #[serde(default)]
    pub deformer_type: String,
    /// DCB field: `ProtosBShapeExclude` (UInt16)
    #[serde(default)]
    pub protos_bshape_exclude: u32,
    /// DCB field: `VisTP` (Boolean)
    #[serde(default)]
    pub vis_tp: bool,
    /// DCB field: `VisFP` (Boolean)
    #[serde(default)]
    pub vis_fp: bool,
    /// DCB field: `VisShadow` (Boolean)
    #[serde(default)]
    pub vis_shadow: bool,
    /// DCB field: `VisSecondaryViews` (Boolean)
    #[serde(default)]
    pub vis_secondary_views: bool,
    /// DCB field: `WrinkleMap` (Boolean)
    #[serde(default)]
    pub wrinkle_map: bool,
    /// DCB field: `EnableDecalProjection` (Boolean)
    #[serde(default)]
    pub enable_decal_projection: bool,
    /// DCB field: `BBoxJoint` (String)
    #[serde(default)]
    pub bbox_joint: String,
    /// DCB field: `BBoxRadius` (Single)
    #[serde(default)]
    pub bbox_radius: f32,
    /// DCB field: `Wear` (Single)
    #[serde(default)]
    pub wear: f32,
    /// DCB field: `Dirt` (Single)
    #[serde(default)]
    pub dirt: f32,
    /// DCB field: `Interference` (Single)
    #[serde(default)]
    pub interference: f32,
    /// DCB field: `Damage` (Single)
    #[serde(default)]
    pub damage: f32,
    /// DCB field: `RenderLayer` (EnumChoice)
    #[serde(default)]
    pub render_layer: String,
    /// DCB field: `VisAreaMode` (EnumChoice)
    #[serde(default)]
    pub vis_area_mode: String,
    /// DCB field: `SunShadowMode` (EnumChoice)
    #[serde(default)]
    pub sun_shadow_mode: String,
    /// DCB field: `viewDistRatio` (Reference)
    #[serde(default)]
    pub view_dist_ratio: Option<CigGuid>,
    /// DCB field: `LodRatio` (Byte)
    #[serde(default)]
    pub lod_ratio: u32,
}

impl Pooled for SGeometryDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgeometry_data_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgeometry_data_params }
}

impl<'a> Extract<'a> for SGeometryDataParams {
    const TYPE_NAME: &'static str = "SGeometryDataParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            simulation_geometry: match inst.get("SimulationGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSoftbodyGeometryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSoftbodyGeometryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            palette: match inst.get("Palette") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintPaletteRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintPaletteRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            slot: inst.get_str("Slot").map(String::from).unwrap_or_default(),
            material_attachments: inst.get_str("MaterialAttachments").map(String::from).unwrap_or_default(),
            proxy_cdfpath: inst.get_str("ProxyCDFPath").map(String::from).unwrap_or_default(),
            modifiers_path: inst.get_str("ModifiersPath").map(String::from).unwrap_or_default(),
            attach_flags: inst.get_u32("AttachFlags").unwrap_or_default(),
            deformer_type: inst.get_str("DeformerType").map(String::from).unwrap_or_default(),
            protos_bshape_exclude: inst.get_u32("ProtosBShapeExclude").unwrap_or_default(),
            vis_tp: inst.get_bool("VisTP").unwrap_or_default(),
            vis_fp: inst.get_bool("VisFP").unwrap_or_default(),
            vis_shadow: inst.get_bool("VisShadow").unwrap_or_default(),
            vis_secondary_views: inst.get_bool("VisSecondaryViews").unwrap_or_default(),
            wrinkle_map: inst.get_bool("WrinkleMap").unwrap_or_default(),
            enable_decal_projection: inst.get_bool("EnableDecalProjection").unwrap_or_default(),
            bbox_joint: inst.get_str("BBoxJoint").map(String::from).unwrap_or_default(),
            bbox_radius: inst.get_f32("BBoxRadius").unwrap_or_default(),
            wear: inst.get_f32("Wear").unwrap_or_default(),
            dirt: inst.get_f32("Dirt").unwrap_or_default(),
            interference: inst.get_f32("Interference").unwrap_or_default(),
            damage: inst.get_f32("Damage").unwrap_or_default(),
            render_layer: inst.get_str("RenderLayer").map(String::from).unwrap_or_default(),
            vis_area_mode: inst.get_str("VisAreaMode").map(String::from).unwrap_or_default(),
            sun_shadow_mode: inst.get_str("SunShadowMode").map(String::from).unwrap_or_default(),
            view_dist_ratio: inst.get("viewDistRatio").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            lod_ratio: inst.get_u32("LodRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGeometryNodeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryNodeParams {
    /// DCB field: `Tags` (String)
    #[serde(default)]
    pub tags: String,
    /// DCB field: `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<SGeometryDataParams>>,
    /// DCB field: `ScaleMultiplier` (Single)
    #[serde(default)]
    pub scale_multiplier: f32,
    /// DCB field: `SubGeometry` (Class (array))
    #[serde(default)]
    pub sub_geometry: Vec<Handle<SGeometryNodeParams>>,
}

impl Pooled for SGeometryNodeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgeometry_node_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgeometry_node_params }
}

impl<'a> Extract<'a> for SGeometryNodeParams {
    const TYPE_NAME: &'static str = "SGeometryNodeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_str("Tags").map(String::from).unwrap_or_default(),
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryDataParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryDataParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scale_multiplier: inst.get_f32("ScaleMultiplier").unwrap_or_default(),
            sub_geometry: inst.get_array("SubGeometry")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SGeometryNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SGeometryNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SGeometryModelTagBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryModelTagBase {
}

impl Pooled for SGeometryModelTagBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgeometry_model_tag_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgeometry_model_tag_base }
}

impl<'a> Extract<'a> for SGeometryModelTagBase {
    const TYPE_NAME: &'static str = "SGeometryModelTagBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SGeometryMeshsetupParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryMeshsetupParams {
    /// DCB field: `visibilityTags` (Class)
    #[serde(default)]
    pub visibility_tags: Option<Handle<TagList>>,
}

impl Pooled for SGeometryMeshsetupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgeometry_meshsetup_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgeometry_meshsetup_params }
}

impl<'a> Extract<'a> for SGeometryMeshsetupParams {
    const TYPE_NAME: &'static str = "SGeometryMeshsetupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            visibility_tags: match inst.get("visibilityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGeometryResourceParams`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryResourceParams {
    /// DCB field: `ModelTag` (StrongPointer)
    #[serde(default)]
    pub model_tag: Option<Handle<SGeometryModelTagBase>>,
    /// DCB field: `cacheResources` (Boolean)
    #[serde(default)]
    pub cache_resources: bool,
    /// DCB field: `meshsetup` (Class)
    #[serde(default)]
    pub meshsetup: Option<Handle<SGeometryMeshsetupParams>>,
    /// DCB field: `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<SGeometryNodeParams>>,
    /// DCB field: `Material` (Class)
    #[serde(default)]
    pub material: Option<Handle<SMaterialNodeParams>>,
    /// DCB field: `rootOverridePaint` (Boolean)
    #[serde(default)]
    pub root_override_paint: bool,
    /// DCB field: `inheritModelTagFromHost` (Boolean)
    #[serde(default)]
    pub inherit_model_tag_from_host: bool,
}

impl Pooled for SGeometryResourceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgeometry_resource_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgeometry_resource_params }
}

impl<'a> Extract<'a> for SGeometryResourceParams {
    const TYPE_NAME: &'static str = "SGeometryResourceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            model_tag: match inst.get("ModelTag") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryModelTagBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryModelTagBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cache_resources: inst.get_bool("cacheResources").unwrap_or_default(),
            meshsetup: match inst.get("meshsetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryMeshsetupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryMeshsetupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMaterialNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMaterialNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            root_override_paint: inst.get_bool("rootOverridePaint").unwrap_or_default(),
            inherit_model_tag_from_host: inst.get_bool("inheritModelTagFromHost").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalChargeDrainBeamParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalChargeDrainBeamParams {
    /// DCB field: `targetStateOutlineParams` (Class)
    #[serde(default)]
    pub target_state_outline_params: Option<Handle<SChargeDrainTargetStateOutlineParams>>,
    /// DCB field: `targetCardParams` (Class)
    #[serde(default)]
    pub target_card_params: Option<Handle<SChargeDrainCardParams>>,
    /// DCB field: `chargeCardParams` (Class)
    #[serde(default)]
    pub charge_card_params: Option<Handle<SChargeDrainCardParams>>,
}

impl Pooled for SGlobalChargeDrainBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_charge_drain_beam_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_charge_drain_beam_params }
}

impl<'a> Extract<'a> for SGlobalChargeDrainBeamParams {
    const TYPE_NAME: &'static str = "SGlobalChargeDrainBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            target_state_outline_params: match inst.get("targetStateOutlineParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainTargetStateOutlineParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChargeDrainTargetStateOutlineParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_card_params: match inst.get("targetCardParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainCardParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChargeDrainCardParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            charge_card_params: match inst.get("chargeCardParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainCardParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChargeDrainCardParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGlobalCrosshairParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalCrosshairParams {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `lerpSpeed` (Single)
    #[serde(default)]
    pub lerp_speed: f32,
    /// DCB field: `positionSmoothFactor` (Single)
    #[serde(default)]
    pub position_smooth_factor: f32,
    /// DCB field: `distanceSmoothFactor` (Single)
    #[serde(default)]
    pub distance_smooth_factor: f32,
    /// DCB field: `range` (Single)
    #[serde(default)]
    pub range: f32,
    /// DCB field: `hitmarkerTimeForHit` (Single)
    #[serde(default)]
    pub hitmarker_time_for_hit: f32,
    /// DCB field: `hitmarkerTimeForKill` (Single)
    #[serde(default)]
    pub hitmarker_time_for_kill: f32,
    /// DCB field: `killInterruptsPreviousHit` (Boolean)
    #[serde(default)]
    pub kill_interrupts_previous_hit: bool,
    /// DCB field: `hitmarkerPositionMethod` (EnumChoice)
    #[serde(default)]
    pub hitmarker_position_method: String,
    /// DCB field: `crosshairInCombatTime` (Single)
    #[serde(default)]
    pub crosshair_in_combat_time: f32,
    /// DCB field: `hitMarkerSoundHead` (Class)
    #[serde(default)]
    pub hit_marker_sound_head: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `hitMarkerSoundBody` (Class)
    #[serde(default)]
    pub hit_marker_sound_body: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `timeSinceLastHitmarkerRTPC` (Class)
    #[serde(default)]
    pub time_since_last_hitmarker_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `killHitmarkerRTPC` (Class)
    #[serde(default)]
    pub kill_hitmarker_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for SGlobalCrosshairParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_crosshair_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_crosshair_params }
}

impl<'a> Extract<'a> for SGlobalCrosshairParams {
    const TYPE_NAME: &'static str = "SGlobalCrosshairParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            lerp_speed: inst.get_f32("lerpSpeed").unwrap_or_default(),
            position_smooth_factor: inst.get_f32("positionSmoothFactor").unwrap_or_default(),
            distance_smooth_factor: inst.get_f32("distanceSmoothFactor").unwrap_or_default(),
            range: inst.get_f32("range").unwrap_or_default(),
            hitmarker_time_for_hit: inst.get_f32("hitmarkerTimeForHit").unwrap_or_default(),
            hitmarker_time_for_kill: inst.get_f32("hitmarkerTimeForKill").unwrap_or_default(),
            kill_interrupts_previous_hit: inst.get_bool("killInterruptsPreviousHit").unwrap_or_default(),
            hitmarker_position_method: inst.get_str("hitmarkerPositionMethod").map(String::from).unwrap_or_default(),
            crosshair_in_combat_time: inst.get_f32("crosshairInCombatTime").unwrap_or_default(),
            hit_marker_sound_head: match inst.get("hitMarkerSoundHead") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hit_marker_sound_body: match inst.get("hitMarkerSoundBody") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time_since_last_hitmarker_rtpc: match inst.get("timeSinceLastHitmarkerRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            kill_hitmarker_rtpc: match inst.get("killHitmarkerRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGlobalCuttableShapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalCuttableShapeParams {
    /// DCB field: `heatRequiredPerSegment` (Single)
    #[serde(default)]
    pub heat_required_per_segment: f32,
    /// DCB field: `heatDissipationPerSecond` (Single)
    #[serde(default)]
    pub heat_dissipation_per_second: f32,
    /// DCB field: `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `finishedEffect` (Class)
    #[serde(default)]
    pub finished_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `hitRadiusMin` (Single)
    #[serde(default)]
    pub hit_radius_min: f32,
    /// DCB field: `hitRadiusMax` (Single)
    #[serde(default)]
    pub hit_radius_max: f32,
    /// DCB field: `damageMultiplier` (Single)
    #[serde(default)]
    pub damage_multiplier: f32,
    /// DCB field: `impactParticleLifeTime` (Single)
    #[serde(default)]
    pub impact_particle_life_time: f32,
    /// DCB field: `highlightColor` (Class)
    #[serde(default)]
    pub highlight_color: Option<Handle<RGB>>,
    /// DCB field: `highlightOccludedAlpha` (Single)
    #[serde(default)]
    pub highlight_occluded_alpha: f32,
    /// DCB field: `highlightOutlineWidth` (Single)
    #[serde(default)]
    pub highlight_outline_width: f32,
    /// DCB field: `highlightOutlineOnly` (Boolean)
    #[serde(default)]
    pub highlight_outline_only: bool,
}

impl Pooled for SGlobalCuttableShapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_cuttable_shape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_cuttable_shape_params }
}

impl<'a> Extract<'a> for SGlobalCuttableShapeParams {
    const TYPE_NAME: &'static str = "SGlobalCuttableShapeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            heat_required_per_segment: inst.get_f32("heatRequiredPerSegment").unwrap_or_default(),
            heat_dissipation_per_second: inst.get_f32("heatDissipationPerSecond").unwrap_or_default(),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            finished_effect: match inst.get("finishedEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hit_radius_min: inst.get_f32("hitRadiusMin").unwrap_or_default(),
            hit_radius_max: inst.get_f32("hitRadiusMax").unwrap_or_default(),
            damage_multiplier: inst.get_f32("damageMultiplier").unwrap_or_default(),
            impact_particle_life_time: inst.get_f32("impactParticleLifeTime").unwrap_or_default(),
            highlight_color: match inst.get("highlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_occluded_alpha: inst.get_f32("highlightOccludedAlpha").unwrap_or_default(),
            highlight_outline_width: inst.get_f32("highlightOutlineWidth").unwrap_or_default(),
            highlight_outline_only: inst.get_bool("highlightOutlineOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalElectronParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalElectronParams {
    /// DCB field: `time` (Single)
    #[serde(default)]
    pub time: f32,
    /// DCB field: `damagePerCharge` (Single)
    #[serde(default)]
    pub damage_per_charge: f32,
    /// DCB field: `metersPerCharge` (Single)
    #[serde(default)]
    pub meters_per_charge: f32,
    /// DCB field: `damageScalePerJump` (Single)
    #[serde(default)]
    pub damage_scale_per_jump: f32,
    /// DCB field: `cooldownBetweenJumps` (Single)
    #[serde(default)]
    pub cooldown_between_jumps: f32,
    /// DCB field: `residualChargeMultiplier` (Single)
    #[serde(default)]
    pub residual_charge_multiplier: f32,
    /// DCB field: `residualChargeInterference` (Single)
    #[serde(default)]
    pub residual_charge_interference: f32,
    /// DCB field: `explosionParams` (Class)
    #[serde(default)]
    pub explosion_params: Option<Handle<ExplosionParams>>,
    /// DCB field: `chargedTag` (Reference)
    #[serde(default)]
    pub charged_tag: Option<CigGuid>,
    /// DCB field: `chainLightningParticleEffect` (Class)
    #[serde(default)]
    pub chain_lightning_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `chainLightningEffectDuration` (Single)
    #[serde(default)]
    pub chain_lightning_effect_duration: f32,
    /// DCB field: `chainLightningJoint` (String)
    #[serde(default)]
    pub chain_lightning_joint: String,
    /// DCB field: `explosionJoint` (String)
    #[serde(default)]
    pub explosion_joint: String,
    /// DCB field: `residualChargeAudioStartTrigger` (Class)
    #[serde(default)]
    pub residual_charge_audio_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `residualChargeAudioStopTrigger` (Class)
    #[serde(default)]
    pub residual_charge_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `residualChargeTimeRemainingRtpc` (Class)
    #[serde(default)]
    pub residual_charge_time_remaining_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `residualChargeDamageRtpc` (Class)
    #[serde(default)]
    pub residual_charge_damage_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `chainLightningSourceAudioStartTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_source_audio_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `chainLightningSourceAudioStopTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_source_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `chainLightningTargetAudioStartTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_target_audio_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `chainLightningTargetAudioStopTrigger` (Class)
    #[serde(default)]
    pub chain_lightning_target_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `chainLightningTimeRemainingRtpc` (Class)
    #[serde(default)]
    pub chain_lightning_time_remaining_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `chainLightningParticleStrengthRtpc` (Class)
    #[serde(default)]
    pub chain_lightning_particle_strength_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `audioBoneName` (String)
    #[serde(default)]
    pub audio_bone_name: String,
}

impl Pooled for SGlobalElectronParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_electron_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_electron_params }
}

impl<'a> Extract<'a> for SGlobalElectronParams {
    const TYPE_NAME: &'static str = "SGlobalElectronParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            time: inst.get_f32("time").unwrap_or_default(),
            damage_per_charge: inst.get_f32("damagePerCharge").unwrap_or_default(),
            meters_per_charge: inst.get_f32("metersPerCharge").unwrap_or_default(),
            damage_scale_per_jump: inst.get_f32("damageScalePerJump").unwrap_or_default(),
            cooldown_between_jumps: inst.get_f32("cooldownBetweenJumps").unwrap_or_default(),
            residual_charge_multiplier: inst.get_f32("residualChargeMultiplier").unwrap_or_default(),
            residual_charge_interference: inst.get_f32("residualChargeInterference").unwrap_or_default(),
            explosion_params: match inst.get("explosionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            charged_tag: inst.get("chargedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            chain_lightning_particle_effect: match inst.get("chainLightningParticleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chain_lightning_effect_duration: inst.get_f32("chainLightningEffectDuration").unwrap_or_default(),
            chain_lightning_joint: inst.get_str("chainLightningJoint").map(String::from).unwrap_or_default(),
            explosion_joint: inst.get_str("explosionJoint").map(String::from).unwrap_or_default(),
            residual_charge_audio_start_trigger: match inst.get("residualChargeAudioStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            residual_charge_audio_stop_trigger: match inst.get("residualChargeAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            residual_charge_time_remaining_rtpc: match inst.get("residualChargeTimeRemainingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            residual_charge_damage_rtpc: match inst.get("residualChargeDamageRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chain_lightning_source_audio_start_trigger: match inst.get("chainLightningSourceAudioStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chain_lightning_source_audio_stop_trigger: match inst.get("chainLightningSourceAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chain_lightning_target_audio_start_trigger: match inst.get("chainLightningTargetAudioStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chain_lightning_target_audio_stop_trigger: match inst.get("chainLightningTargetAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chain_lightning_time_remaining_rtpc: match inst.get("chainLightningTimeRemainingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chain_lightning_particle_strength_rtpc: match inst.get("chainLightningParticleStrengthRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_bone_name: inst.get_str("audioBoneName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalHealingBeamParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalHealingBeamParams {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `medgunTag` (Reference)
    #[serde(default)]
    pub medgun_tag: Option<CigGuid>,
    /// DCB field: `bodyParts` (Class (array))
    #[serde(default)]
    pub body_parts: Vec<Handle<SHealingBeamBodyPartParams>>,
    /// DCB field: `cardDisplayTimeout` (Single)
    #[serde(default)]
    pub card_display_timeout: f32,
    /// DCB field: `limbSwitchTime` (Single)
    #[serde(default)]
    pub limb_switch_time: f32,
    /// DCB field: `cardPosLerpSpeed` (Single)
    #[serde(default)]
    pub card_pos_lerp_speed: f32,
    /// DCB field: `cardClosingLerpSpeedScalar` (Single)
    #[serde(default)]
    pub card_closing_lerp_speed_scalar: f32,
    /// DCB field: `targetModeActorCardBoneEntry` (Class)
    #[serde(default)]
    pub target_mode_actor_card_bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// DCB field: `selfHealModeActorCardBoneEntry` (Class)
    #[serde(default)]
    pub self_heal_mode_actor_card_bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// DCB field: `selfHealModeLimbCardBoneEntry` (Class)
    #[serde(default)]
    pub self_heal_mode_limb_card_bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// DCB field: `transparentMaterial` (Class)
    #[serde(default)]
    pub transparent_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `injuryHighlightColors` (Class)
    #[serde(default)]
    pub injury_highlight_colors: Option<Handle<RGB>>,
    /// DCB field: `highlightOccludedAlpha` (Single)
    #[serde(default)]
    pub highlight_occluded_alpha: f32,
    /// DCB field: `highlightOutlineWidth` (Single)
    #[serde(default)]
    pub highlight_outline_width: f32,
    /// DCB field: `highlightOutlineOnly` (Boolean)
    #[serde(default)]
    pub highlight_outline_only: bool,
}

impl Pooled for SGlobalHealingBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_healing_beam_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_healing_beam_params }
}

impl<'a> Extract<'a> for SGlobalHealingBeamParams {
    const TYPE_NAME: &'static str = "SGlobalHealingBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            medgun_tag: inst.get("medgunTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            body_parts: inst.get_array("bodyParts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHealingBeamBodyPartParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHealingBeamBodyPartParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            card_display_timeout: inst.get_f32("cardDisplayTimeout").unwrap_or_default(),
            limb_switch_time: inst.get_f32("limbSwitchTime").unwrap_or_default(),
            card_pos_lerp_speed: inst.get_f32("cardPosLerpSpeed").unwrap_or_default(),
            card_closing_lerp_speed_scalar: inst.get_f32("cardClosingLerpSpeedScalar").unwrap_or_default(),
            target_mode_actor_card_bone_entry: match inst.get("targetModeActorCardBoneEntry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            self_heal_mode_actor_card_bone_entry: match inst.get("selfHealModeActorCardBoneEntry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            self_heal_mode_limb_card_bone_entry: match inst.get("selfHealModeLimbCardBoneEntry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            transparent_material: match inst.get("transparentMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            injury_highlight_colors: match inst.get("injuryHighlightColors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_occluded_alpha: inst.get_f32("highlightOccludedAlpha").unwrap_or_default(),
            highlight_outline_width: inst.get_f32("highlightOutlineWidth").unwrap_or_default(),
            highlight_outline_only: inst.get_bool("highlightOutlineOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalSalvageRepairBeamParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalSalvageRepairBeamParams {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `cardParams` (Class)
    #[serde(default)]
    pub card_params: Option<Handle<SSalvageRepairCardParams>>,
    /// DCB field: `highlightParams` (Class)
    #[serde(default)]
    pub highlight_params: Option<Handle<SSalvageRepairHighlightParams>>,
    /// DCB field: `localizationParams` (Class)
    #[serde(default)]
    pub localization_params: Option<Handle<SSalvageRepairLocalizationParams>>,
    /// DCB field: `materialParams` (Class)
    #[serde(default)]
    pub material_params: Option<Handle<SSalvageRepairMaterialParams>>,
    /// DCB field: `globalSalvageAudioParams` (Class)
    #[serde(default)]
    pub global_salvage_audio_params: Option<Handle<SSalvageRepairAudioParams>>,
    /// DCB field: `hitsPerSecond` (Single)
    #[serde(default)]
    pub hits_per_second: f32,
    /// DCB field: `hitDuration` (Single)
    #[serde(default)]
    pub hit_duration: f32,
}

impl Pooled for SGlobalSalvageRepairBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_salvage_repair_beam_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_salvage_repair_beam_params }
}

impl<'a> Extract<'a> for SGlobalSalvageRepairBeamParams {
    const TYPE_NAME: &'static str = "SGlobalSalvageRepairBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            card_params: match inst.get("cardParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairCardParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageRepairCardParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairHighlightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageRepairHighlightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            localization_params: match inst.get("localizationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairLocalizationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageRepairLocalizationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material_params: match inst.get("materialParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairMaterialParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageRepairMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            global_salvage_audio_params: match inst.get("globalSalvageAudioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageRepairAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageRepairAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hits_per_second: inst.get_f32("hitsPerSecond").unwrap_or_default(),
            hit_duration: inst.get_f32("hitDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalShopErrors`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalShopErrors {
    /// DCB field: `confirmation_success` (Locale)
    #[serde(default)]
    pub confirmation_success: String,
    /// DCB field: `confirmation_fail` (Locale)
    #[serde(default)]
    pub confirmation_fail: String,
    /// DCB field: `confirmation_fail_AuthorityError` (Locale)
    #[serde(default)]
    pub confirmation_fail_authority_error: String,
    /// DCB field: `confirmation_fail_TransactionServiceError` (Locale)
    #[serde(default)]
    pub confirmation_fail_transaction_service_error: String,
    /// DCB field: `confirmation_fail_InvalidLocation` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_location: String,
    /// DCB field: `confirmation_fail_InvalidPlayerInventoryId` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_player_inventory_id: String,
    /// DCB field: `confirmation_fail_InventoryContainerRequestFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_inventory_container_request_fail: String,
    /// DCB field: `confirmation_fail_InventoryItemFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_inventory_item_fail: String,
    /// DCB field: `confirmation_fail_InventoryItemContentFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_inventory_item_content_fail: String,
    /// DCB field: `confirmation_fail_InvalidQuantityError` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_quantity_error: String,
    /// DCB field: `confirmation_fail_QuickBuyRestockingError` (Locale)
    #[serde(default)]
    pub confirmation_fail_quick_buy_restocking_error: String,
    /// DCB field: `confirmation_fail_InvalidTransactionFlow` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_transaction_flow: String,
    /// DCB field: `confirmation_fail_InvalidLocationSource` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_location_source: String,
    /// DCB field: `confirmation_fail_InvalidShop` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_shop: String,
    /// DCB field: `confirmation_fail_InvalidShopType` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_shop_type: String,
    /// DCB field: `confirmation_fail_InternalError` (Locale)
    #[serde(default)]
    pub confirmation_fail_internal_error: String,
    /// DCB field: `confirmation_fail_InvalidRentalOption` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_rental_option: String,
    /// DCB field: `confirmation_fail_ShipNotInValidLocation` (Locale)
    #[serde(default)]
    pub confirmation_fail_ship_not_in_valid_location: String,
    /// DCB field: `confirmation_fail_NoItemsInSaleError` (Locale)
    #[serde(default)]
    pub confirmation_fail_no_items_in_sale_error: String,
    /// DCB field: `confirmation_fail_WaitingForPendingResult` (Locale)
    #[serde(default)]
    pub confirmation_fail_waiting_for_pending_result: String,
    /// DCB field: `confirmation_fail_ActorDoesNotOwnSaleItem` (Locale)
    #[serde(default)]
    pub confirmation_fail_actor_does_not_own_sale_item: String,
    /// DCB field: `confirmation_fail_TransactionCostMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_transaction_cost_mismatch: String,
    /// DCB field: `confirmation_fail_ItemMaxStockError` (Locale)
    #[serde(default)]
    pub confirmation_fail_item_max_stock_error: String,
    /// DCB field: `confirmation_fail_ItemNotSellable` (Locale)
    #[serde(default)]
    pub confirmation_fail_item_not_sellable: String,
    /// DCB field: `confirmation_fail_ItemNotBuyable` (Locale)
    #[serde(default)]
    pub confirmation_fail_item_not_buyable: String,
    /// DCB field: `confirmation_fail_TimedOut` (Locale)
    #[serde(default)]
    pub confirmation_fail_timed_out: String,
    /// DCB field: `confirmation_fail_InsuffientStock` (Locale)
    #[serde(default)]
    pub confirmation_fail_insuffient_stock: String,
    /// DCB field: `confirmation_fail_ServiceError` (Locale)
    #[serde(default)]
    pub confirmation_fail_service_error: String,
    /// DCB field: `confirmation_fail_DatabaseError` (Locale)
    #[serde(default)]
    pub confirmation_fail_database_error: String,
    /// DCB field: `confirmation_fail_InvalidBuyer` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_buyer: String,
    /// DCB field: `confirmation_fail_InvalidItem` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_item: String,
    /// DCB field: `confirmation_fail_InvalidRequest` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_request: String,
    /// DCB field: `confirmation_fail_InsufficentFunds` (Locale)
    #[serde(default)]
    pub confirmation_fail_insufficent_funds: String,
    /// DCB field: `confirmation_fail_InvalidEntityClassGUID` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_entity_class_guid: String,
    /// DCB field: `confirmation_fail_InvalidKioskId` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_kiosk_id: String,
    /// DCB field: `confirmation_fail_InvalidSellPrice` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_sell_price: String,
    /// DCB field: `confirmation_fail_InvalidMineableEntry` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_mineable_entry: String,
    /// DCB field: `confirmation_fail_PlayerIdMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_player_id_mismatch: String,
    /// DCB field: `confirmation_fail_CargoCreationFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_cargo_creation_failed: String,
    /// DCB field: `confirmation_fail_WalletNotFound` (Locale)
    #[serde(default)]
    pub confirmation_fail_wallet_not_found: String,
    /// DCB field: `confirmation_fail_MissingResourceDataType` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_resource_data_type: String,
    /// DCB field: `confirmation_fail_PlayerInVehicleDuringCargoTransaction` (Locale)
    #[serde(default)]
    pub confirmation_fail_player_in_vehicle_during_cargo_transaction: String,
    /// DCB field: `confirmation_fail_InvalidParentState` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_parent_state: String,
    /// DCB field: `confirmation_fail_InvalidResourceTypeGUID` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_resource_type_guid: String,
    /// DCB field: `confirmation_fail_CargoRemovalFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_cargo_removal_failed: String,
    /// DCB field: `confirmation_fail_WalletUpdateFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_wallet_update_failed: String,
    /// DCB field: `confirmation_fail_ResourceContainerQueryFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_resource_container_query_failed: String,
    /// DCB field: `confirmation_fail_PricePerUnitMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_price_per_unit_mismatch: String,
    /// DCB field: `confirmation_fail_InvalidContainer` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_container: String,
    /// DCB field: `confirmation_fail_EntityQueryFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_entity_query_failed: String,
    /// DCB field: `confirmation_fail_MissingSnapshot` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_snapshot: String,
    /// DCB field: `confirmation_fail_MissingSnapshotData` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_snapshot_data: String,
    /// DCB field: `confirmation_fail_SnapshotGetFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_snapshot_get_fail: String,
    /// DCB field: `confirmation_fail_ExceededBuyLimit` (Locale)
    #[serde(default)]
    pub confirmation_fail_exceeded_buy_limit: String,
    /// DCB field: `confirmation_fail_LicenseError` (Locale)
    #[serde(default)]
    pub confirmation_fail_license_error: String,
    /// DCB field: `confirmation_fail_InvalidPlayerState` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_player_state: String,
    /// DCB field: `confirmation_fail_InvalidBoxSize` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_box_size: String,
    /// DCB field: `confirmation_fail_InvalidBoxClass` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_box_class: String,
    /// DCB field: `confirmation_fail_MissingAutoLoadRate` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_auto_load_rate: String,
    /// DCB field: `confirmation_fail_AutoLoadPriceMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_auto_load_price_mismatch: String,
    /// DCB field: `confirmation_fail_AutoLoadTimeMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_auto_load_time_mismatch: String,
    /// DCB field: `confirmation_fail_AutoLoadStartFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_auto_load_start_failed: String,
}

impl Pooled for SGlobalShopErrors {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_shop_errors }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_shop_errors }
}

impl<'a> Extract<'a> for SGlobalShopErrors {
    const TYPE_NAME: &'static str = "SGlobalShopErrors";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            confirmation_success: inst.get_str("confirmation_success").map(String::from).unwrap_or_default(),
            confirmation_fail: inst.get_str("confirmation_fail").map(String::from).unwrap_or_default(),
            confirmation_fail_authority_error: inst.get_str("confirmation_fail_AuthorityError").map(String::from).unwrap_or_default(),
            confirmation_fail_transaction_service_error: inst.get_str("confirmation_fail_TransactionServiceError").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_location: inst.get_str("confirmation_fail_InvalidLocation").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_player_inventory_id: inst.get_str("confirmation_fail_InvalidPlayerInventoryId").map(String::from).unwrap_or_default(),
            confirmation_fail_inventory_container_request_fail: inst.get_str("confirmation_fail_InventoryContainerRequestFail").map(String::from).unwrap_or_default(),
            confirmation_fail_inventory_item_fail: inst.get_str("confirmation_fail_InventoryItemFail").map(String::from).unwrap_or_default(),
            confirmation_fail_inventory_item_content_fail: inst.get_str("confirmation_fail_InventoryItemContentFail").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_quantity_error: inst.get_str("confirmation_fail_InvalidQuantityError").map(String::from).unwrap_or_default(),
            confirmation_fail_quick_buy_restocking_error: inst.get_str("confirmation_fail_QuickBuyRestockingError").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_transaction_flow: inst.get_str("confirmation_fail_InvalidTransactionFlow").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_location_source: inst.get_str("confirmation_fail_InvalidLocationSource").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_shop: inst.get_str("confirmation_fail_InvalidShop").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_shop_type: inst.get_str("confirmation_fail_InvalidShopType").map(String::from).unwrap_or_default(),
            confirmation_fail_internal_error: inst.get_str("confirmation_fail_InternalError").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_rental_option: inst.get_str("confirmation_fail_InvalidRentalOption").map(String::from).unwrap_or_default(),
            confirmation_fail_ship_not_in_valid_location: inst.get_str("confirmation_fail_ShipNotInValidLocation").map(String::from).unwrap_or_default(),
            confirmation_fail_no_items_in_sale_error: inst.get_str("confirmation_fail_NoItemsInSaleError").map(String::from).unwrap_or_default(),
            confirmation_fail_waiting_for_pending_result: inst.get_str("confirmation_fail_WaitingForPendingResult").map(String::from).unwrap_or_default(),
            confirmation_fail_actor_does_not_own_sale_item: inst.get_str("confirmation_fail_ActorDoesNotOwnSaleItem").map(String::from).unwrap_or_default(),
            confirmation_fail_transaction_cost_mismatch: inst.get_str("confirmation_fail_TransactionCostMismatch").map(String::from).unwrap_or_default(),
            confirmation_fail_item_max_stock_error: inst.get_str("confirmation_fail_ItemMaxStockError").map(String::from).unwrap_or_default(),
            confirmation_fail_item_not_sellable: inst.get_str("confirmation_fail_ItemNotSellable").map(String::from).unwrap_or_default(),
            confirmation_fail_item_not_buyable: inst.get_str("confirmation_fail_ItemNotBuyable").map(String::from).unwrap_or_default(),
            confirmation_fail_timed_out: inst.get_str("confirmation_fail_TimedOut").map(String::from).unwrap_or_default(),
            confirmation_fail_insuffient_stock: inst.get_str("confirmation_fail_InsuffientStock").map(String::from).unwrap_or_default(),
            confirmation_fail_service_error: inst.get_str("confirmation_fail_ServiceError").map(String::from).unwrap_or_default(),
            confirmation_fail_database_error: inst.get_str("confirmation_fail_DatabaseError").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_buyer: inst.get_str("confirmation_fail_InvalidBuyer").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_item: inst.get_str("confirmation_fail_InvalidItem").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_request: inst.get_str("confirmation_fail_InvalidRequest").map(String::from).unwrap_or_default(),
            confirmation_fail_insufficent_funds: inst.get_str("confirmation_fail_InsufficentFunds").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_entity_class_guid: inst.get_str("confirmation_fail_InvalidEntityClassGUID").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_kiosk_id: inst.get_str("confirmation_fail_InvalidKioskId").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_sell_price: inst.get_str("confirmation_fail_InvalidSellPrice").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_mineable_entry: inst.get_str("confirmation_fail_InvalidMineableEntry").map(String::from).unwrap_or_default(),
            confirmation_fail_player_id_mismatch: inst.get_str("confirmation_fail_PlayerIdMismatch").map(String::from).unwrap_or_default(),
            confirmation_fail_cargo_creation_failed: inst.get_str("confirmation_fail_CargoCreationFailed").map(String::from).unwrap_or_default(),
            confirmation_fail_wallet_not_found: inst.get_str("confirmation_fail_WalletNotFound").map(String::from).unwrap_or_default(),
            confirmation_fail_missing_resource_data_type: inst.get_str("confirmation_fail_MissingResourceDataType").map(String::from).unwrap_or_default(),
            confirmation_fail_player_in_vehicle_during_cargo_transaction: inst.get_str("confirmation_fail_PlayerInVehicleDuringCargoTransaction").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_parent_state: inst.get_str("confirmation_fail_InvalidParentState").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_resource_type_guid: inst.get_str("confirmation_fail_InvalidResourceTypeGUID").map(String::from).unwrap_or_default(),
            confirmation_fail_cargo_removal_failed: inst.get_str("confirmation_fail_CargoRemovalFailed").map(String::from).unwrap_or_default(),
            confirmation_fail_wallet_update_failed: inst.get_str("confirmation_fail_WalletUpdateFailed").map(String::from).unwrap_or_default(),
            confirmation_fail_resource_container_query_failed: inst.get_str("confirmation_fail_ResourceContainerQueryFailed").map(String::from).unwrap_or_default(),
            confirmation_fail_price_per_unit_mismatch: inst.get_str("confirmation_fail_PricePerUnitMismatch").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_container: inst.get_str("confirmation_fail_InvalidContainer").map(String::from).unwrap_or_default(),
            confirmation_fail_entity_query_failed: inst.get_str("confirmation_fail_EntityQueryFailed").map(String::from).unwrap_or_default(),
            confirmation_fail_missing_snapshot: inst.get_str("confirmation_fail_MissingSnapshot").map(String::from).unwrap_or_default(),
            confirmation_fail_missing_snapshot_data: inst.get_str("confirmation_fail_MissingSnapshotData").map(String::from).unwrap_or_default(),
            confirmation_fail_snapshot_get_fail: inst.get_str("confirmation_fail_SnapshotGetFail").map(String::from).unwrap_or_default(),
            confirmation_fail_exceeded_buy_limit: inst.get_str("confirmation_fail_ExceededBuyLimit").map(String::from).unwrap_or_default(),
            confirmation_fail_license_error: inst.get_str("confirmation_fail_LicenseError").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_player_state: inst.get_str("confirmation_fail_InvalidPlayerState").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_box_size: inst.get_str("confirmation_fail_InvalidBoxSize").map(String::from).unwrap_or_default(),
            confirmation_fail_invalid_box_class: inst.get_str("confirmation_fail_InvalidBoxClass").map(String::from).unwrap_or_default(),
            confirmation_fail_missing_auto_load_rate: inst.get_str("confirmation_fail_MissingAutoLoadRate").map(String::from).unwrap_or_default(),
            confirmation_fail_auto_load_price_mismatch: inst.get_str("confirmation_fail_AutoLoadPriceMismatch").map(String::from).unwrap_or_default(),
            confirmation_fail_auto_load_time_mismatch: inst.get_str("confirmation_fail_AutoLoadTimeMismatch").map(String::from).unwrap_or_default(),
            confirmation_fail_auto_load_start_failed: inst.get_str("confirmation_fail_AutoLoadStartFailed").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalTractorBeamParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalTractorBeamParams {
    /// DCB field: `holoVisualParams` (Class)
    #[serde(default)]
    pub holo_visual_params: Option<Handle<STractorBeamHoloVisualParams>>,
    /// DCB field: `outlineVisualParams` (Class)
    #[serde(default)]
    pub outline_visual_params: Option<Handle<STractorBeamOutlineVisualParams>>,
    /// DCB field: `beingTractorBeamedTag` (Reference)
    #[serde(default)]
    pub being_tractor_beamed_tag: Option<CigGuid>,
    /// DCB field: `contractedTag` (Reference)
    #[serde(default)]
    pub contracted_tag: Option<CigGuid>,
    /// DCB field: `showLifetimeTag` (Reference)
    #[serde(default)]
    pub show_lifetime_tag: Option<CigGuid>,
    /// DCB field: `ignoreEntityTag` (Reference)
    #[serde(default)]
    pub ignore_entity_tag: Option<CigGuid>,
    /// DCB field: `magLockedTag` (Reference)
    #[serde(default)]
    pub mag_locked_tag: Option<CigGuid>,
    /// DCB field: `checkParentForIgnoreTag` (Boolean)
    #[serde(default)]
    pub check_parent_for_ignore_tag: bool,
}

impl Pooled for SGlobalTractorBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_tractor_beam_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_tractor_beam_params }
}

impl<'a> Extract<'a> for SGlobalTractorBeamParams {
    const TYPE_NAME: &'static str = "SGlobalTractorBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            holo_visual_params: match inst.get("holoVisualParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STractorBeamHoloVisualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STractorBeamHoloVisualParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outline_visual_params: match inst.get("outlineVisualParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STractorBeamOutlineVisualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STractorBeamOutlineVisualParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            being_tractor_beamed_tag: inst.get("beingTractorBeamedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contracted_tag: inst.get("contractedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            show_lifetime_tag: inst.get("showLifetimeTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ignore_entity_tag: inst.get("ignoreEntityTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mag_locked_tag: inst.get("magLockedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            check_parent_for_ignore_tag: inst.get_bool("checkParentForIgnoreTag").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGrip`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGrip {
    /// DCB field: `gripID` (String)
    #[serde(default)]
    pub grip_id: String,
    /// DCB field: `optionalHelper` (String)
    #[serde(default)]
    pub optional_helper: String,
    /// DCB field: `gripShapeParameters` (Class)
    #[serde(default)]
    pub grip_shape_parameters: Option<Handle<SGripShapeParams>>,
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
}

impl Pooled for SGrip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgrip }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgrip }
}

impl<'a> Extract<'a> for SGrip {
    const TYPE_NAME: &'static str = "SGrip";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            grip_id: inst.get_str("gripID").map(String::from).unwrap_or_default(),
            optional_helper: inst.get_str("optionalHelper").map(String::from).unwrap_or_default(),
            grip_shape_parameters: match inst.get("gripShapeParameters") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGripShapeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGripShapeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGripShapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGripShapeParams {
    /// DCB field: `gripShape` (String)
    #[serde(default)]
    pub grip_shape: String,
    /// DCB field: `dimension` (Single)
    #[serde(default)]
    pub dimension: f32,
    /// DCB field: `wristRotation` (Single)
    #[serde(default)]
    pub wrist_rotation: f32,
}

impl Pooled for SGripShapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgrip_shape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgrip_shape_params }
}

impl<'a> Extract<'a> for SGripShapeParams {
    const TYPE_NAME: &'static str = "SGripShapeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grip_shape: inst.get_str("gripShape").map(String::from).unwrap_or_default(),
            dimension: inst.get_f32("dimension").unwrap_or_default(),
            wrist_rotation: inst.get_f32("wristRotation").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalHitBehaviorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalHitBehaviorParams {
    /// DCB field: `damagePerTickUpperLimit` (Single)
    #[serde(default)]
    pub damage_per_tick_upper_limit: f32,
    /// DCB field: `timeUpperLimit` (Single)
    #[serde(default)]
    pub time_upper_limit: f32,
}

impl Pooled for SGlobalHitBehaviorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sglobal_hit_behavior_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sglobal_hit_behavior_params }
}

impl<'a> Extract<'a> for SGlobalHitBehaviorParams {
    const TYPE_NAME: &'static str = "SGlobalHitBehaviorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_per_tick_upper_limit: inst.get_f32("damagePerTickUpperLimit").unwrap_or_default(),
            time_upper_limit: inst.get_f32("timeUpperLimit").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGameCollisionClass`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGameCollisionClass {
    /// DCB field: `gameCollisionClass` (EnumChoice)
    #[serde(default)]
    pub game_collision_class: String,
    /// DCB field: `gameIgnoreCollisionClass` (EnumChoice)
    #[serde(default)]
    pub game_ignore_collision_class: String,
}

impl Pooled for SGameCollisionClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgame_collision_class }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgame_collision_class }
}

impl<'a> Extract<'a> for SGameCollisionClass {
    const TYPE_NAME: &'static str = "SGameCollisionClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            game_collision_class: inst.get_str("gameCollisionClass").map(String::from).unwrap_or_default(),
            game_ignore_collision_class: inst.get_str("gameIgnoreCollisionClass").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SGasAtmosphereEntryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGasAtmosphereEntryParams {
    /// DCB field: `gasParams` (Reference)
    #[serde(default)]
    pub gas_params: Option<CigGuid>,
    /// DCB field: `parts` (Single)
    #[serde(default)]
    pub parts: f32,
}

impl Pooled for SGasAtmosphereEntryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sg.sgas_atmosphere_entry_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sg.sgas_atmosphere_entry_params }
}

impl<'a> Extract<'a> for SGasAtmosphereEntryParams {
    const TYPE_NAME: &'static str = "SGasAtmosphereEntryParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            gas_params: inst.get("gasParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            parts: inst.get_f32("parts").unwrap_or_default(),
        }
    }
}

