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

/// DCB type: `PlacedSurfaceEffects_Emitter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedSurfaceEffects_Emitter {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `emitterPosition` (Class)
    #[serde(default)]
    pub emitter_position: Option<Handle<Vec3>>,
    /// DCB field: `linkedToSdf` (Boolean)
    #[serde(default)]
    pub linked_to_sdf: bool,
    /// DCB field: `fadeOutDuration` (Single)
    #[serde(default)]
    pub fade_out_duration: f32,
}

impl Pooled for PlacedSurfaceEffects_Emitter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.placed_surface_effects_emitter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.placed_surface_effects_emitter }
}

impl<'a> Extract<'a> for PlacedSurfaceEffects_Emitter {
    const TYPE_NAME: &'static str = "PlacedSurfaceEffects_Emitter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emitter_position: match inst.get("emitterPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linked_to_sdf: inst.get_bool("linkedToSdf").unwrap_or_default(),
            fade_out_duration: inst.get_f32("fadeOutDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetOceanAudioCheckpoint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetOceanAudioCheckpoint {
    /// DCB field: `beamCount` (Int32)
    #[serde(default)]
    pub beam_count: i32,
    /// DCB field: `range` (Single)
    #[serde(default)]
    pub range: f32,
    /// DCB field: `useDepthAssignment` (Boolean)
    #[serde(default)]
    pub use_depth_assignment: bool,
}

impl Pooled for PlanetOceanAudioCheckpoint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.planet_ocean_audio_checkpoint }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.planet_ocean_audio_checkpoint }
}

impl<'a> Extract<'a> for PlanetOceanAudioCheckpoint {
    const TYPE_NAME: &'static str = "PlanetOceanAudioCheckpoint";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            beam_count: inst.get_i32("beamCount").unwrap_or_default(),
            range: inst.get_f32("range").unwrap_or_default(),
            use_depth_assignment: inst.get_bool("useDepthAssignment").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetOceanDepthAssignment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetOceanDepthAssignment {
    /// DCB field: `waterDepth` (Single)
    #[serde(default)]
    pub water_depth: f32,
    /// DCB field: `assignmentStartTrigger` (Class)
    #[serde(default)]
    pub assignment_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `assignmentStopTrigger` (Class)
    #[serde(default)]
    pub assignment_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `recalculationDistance` (Single)
    #[serde(default)]
    pub recalculation_distance: f32,
}

impl Pooled for PlanetOceanDepthAssignment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.planet_ocean_depth_assignment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.planet_ocean_depth_assignment }
}

impl<'a> Extract<'a> for PlanetOceanDepthAssignment {
    const TYPE_NAME: &'static str = "PlanetOceanDepthAssignment";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            water_depth: inst.get_f32("waterDepth").unwrap_or_default(),
            assignment_start_trigger: match inst.get("assignmentStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            assignment_stop_trigger: match inst.get("assignmentStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            recalculation_distance: inst.get_f32("recalculationDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetOceanAudioData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetOceanAudioData {
    /// DCB field: `checkpoints` (Class (array))
    #[serde(default)]
    pub checkpoints: Vec<Handle<PlanetOceanAudioCheckpoint>>,
    /// DCB field: `assignments` (Class (array))
    #[serde(default)]
    pub assignments: Vec<Handle<PlanetOceanDepthAssignment>>,
    /// DCB field: `waterStartTrigger` (Class)
    #[serde(default)]
    pub water_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `waterStopTrigger` (Class)
    #[serde(default)]
    pub water_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `oceanOffsetRtpc` (Class)
    #[serde(default)]
    pub ocean_offset_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `windRTPC` (Class)
    #[serde(default)]
    pub wind_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `waveHeightRTPC` (Class)
    #[serde(default)]
    pub wave_height_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `atmosphereTag` (Reference)
    #[serde(default)]
    pub atmosphere_tag: Option<CigGuid>,
    /// DCB field: `terrainChecksPerFrame` (Int32)
    #[serde(default)]
    pub terrain_checks_per_frame: i32,
    /// DCB field: `checkOnListenerPosition` (Boolean)
    #[serde(default)]
    pub check_on_listener_position: bool,
    /// DCB field: `listenerPositionUsesAssignment` (Boolean)
    #[serde(default)]
    pub listener_position_uses_assignment: bool,
}

impl Pooled for PlanetOceanAudioData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.planet_ocean_audio_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.planet_ocean_audio_data }
}

impl<'a> Extract<'a> for PlanetOceanAudioData {
    const TYPE_NAME: &'static str = "PlanetOceanAudioData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            checkpoints: inst.get_array("checkpoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlanetOceanAudioCheckpoint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlanetOceanAudioCheckpoint>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            assignments: inst.get_array("assignments")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlanetOceanDepthAssignment>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlanetOceanDepthAssignment>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            water_start_trigger: match inst.get("waterStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            water_stop_trigger: match inst.get("waterStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ocean_offset_rtpc: match inst.get("oceanOffsetRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wind_rtpc: match inst.get("windRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wave_height_rtpc: match inst.get("waveHeightRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmosphere_tag: inst.get("atmosphereTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            terrain_checks_per_frame: inst.get_i32("terrainChecksPerFrame").unwrap_or_default(),
            check_on_listener_position: inst.get_bool("checkOnListenerPosition").unwrap_or_default(),
            listener_position_uses_assignment: inst.get_bool("listenerPositionUsesAssignment").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerFormationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerFormationParams {
    /// DCB field: `targetRadius` (Single)
    #[serde(default)]
    pub target_radius: f32,
    /// DCB field: `breakRadius` (Single)
    #[serde(default)]
    pub break_radius: f32,
    /// DCB field: `abandonFormationAtBreakRadius` (Boolean)
    #[serde(default)]
    pub abandon_formation_at_break_radius: bool,
    /// DCB field: `targetVelocityTolerance` (Single)
    #[serde(default)]
    pub target_velocity_tolerance: f32,
}

impl Pooled for PlayerFormationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_formation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_formation_params }
}

impl<'a> Extract<'a> for PlayerFormationParams {
    const TYPE_NAME: &'static str = "PlayerFormationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            target_radius: inst.get_f32("targetRadius").unwrap_or_default(),
            break_radius: inst.get_f32("breakRadius").unwrap_or_default(),
            abandon_formation_at_break_radius: inst.get_bool("abandonFormationAtBreakRadius").unwrap_or_default(),
            target_velocity_tolerance: inst.get_f32("targetVelocityTolerance").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlaylistRNGConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistRNGConfig {
    /// DCB field: `parameters` (Reference (array))
    #[serde(default)]
    pub parameters: Vec<CigGuid>,
    /// DCB field: `maxDeviation` (Single)
    #[serde(default)]
    pub max_deviation: f32,
}

impl Pooled for PlaylistRNGConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.playlist_rngconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.playlist_rngconfig }
}

impl<'a> Extract<'a> for PlaylistRNGConfig {
    const TYPE_NAME: &'static str = "PlaylistRNGConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            max_deviation: inst.get_f32("maxDeviation").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetEffectLODDistance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetEffectLODDistance {
    /// DCB field: `minCameraDistance` (Single)
    #[serde(default)]
    pub min_camera_distance: f32,
    /// DCB field: `maxCameraDistance` (Single)
    #[serde(default)]
    pub max_camera_distance: f32,
    /// DCB field: `subPatchLength` (Single)
    #[serde(default)]
    pub sub_patch_length: f32,
}

impl Pooled for PlanetEffectLODDistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.planet_effect_loddistance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.planet_effect_loddistance }
}

impl<'a> Extract<'a> for PlanetEffectLODDistance {
    const TYPE_NAME: &'static str = "PlanetEffectLODDistance";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_camera_distance: inst.get_f32("minCameraDistance").unwrap_or_default(),
            max_camera_distance: inst.get_f32("maxCameraDistance").unwrap_or_default(),
            sub_patch_length: inst.get_f32("subPatchLength").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetEffectLOD`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetEffectLOD {
    /// DCB field: `LODList` (Class (array))
    #[serde(default)]
    pub lodlist: Vec<Handle<PlanetEffectLODDistance>>,
    /// DCB field: `globalFogVolume` (Class)
    #[serde(default)]
    pub global_fog_volume: Option<Handle<GlobalFogVolume>>,
    /// DCB field: `tintColorSampleCells` (UInt32)
    #[serde(default)]
    pub tint_color_sample_cells: u32,
    /// DCB field: `sortByViewDistance` (Boolean)
    #[serde(default)]
    pub sort_by_view_distance: bool,
    /// DCB field: `overrideHalfResInsertDepth` (Single)
    #[serde(default)]
    pub override_half_res_insert_depth: f32,
}

impl Pooled for PlanetEffectLOD {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.planet_effect_lod }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.planet_effect_lod }
}

impl<'a> Extract<'a> for PlanetEffectLOD {
    const TYPE_NAME: &'static str = "PlanetEffectLOD";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lodlist: inst.get_array("LODList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlanetEffectLODDistance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlanetEffectLODDistance>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_fog_volume: match inst.get("globalFogVolume") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalFogVolume>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalFogVolume>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tint_color_sample_cells: inst.get_u32("tintColorSampleCells").unwrap_or_default(),
            sort_by_view_distance: inst.get_bool("sortByViewDistance").unwrap_or_default(),
            override_half_res_insert_depth: inst.get_f32("overrideHalfResInsertDepth").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionHandParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionHandParams {
    /// DCB field: `handMode` (EnumChoice)
    #[serde(default)]
    pub hand_mode: String,
}

impl Pooled for PlayerAnimatedInteractionHandParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_animated_interaction_hand_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_animated_interaction_hand_params }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionHandParams {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionHandParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hand_mode: inst.get_str("handMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerAnimatedInteraction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteraction {
    /// DCB field: `supportWalkToAlign` (Boolean)
    #[serde(default)]
    pub support_walk_to_align: bool,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `fragTags` (String (array))
    #[serde(default)]
    pub frag_tags: Vec<String>,
    /// DCB field: `CDIKTargetName` (String)
    #[serde(default)]
    pub cdiktarget_name: String,
    /// DCB field: `handModeParams` (StrongPointer)
    #[serde(default)]
    pub hand_mode_params: Option<Handle<PlayerAnimatedInteractionHandParams>>,
}

impl Pooled for PlayerAnimatedInteraction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_animated_interaction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_animated_interaction }
}

impl<'a> Extract<'a> for PlayerAnimatedInteraction {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteraction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            support_walk_to_align: inst.get_bool("supportWalkToAlign").unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            frag_tags: inst.get_array("fragTags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            cdiktarget_name: inst.get_str("CDIKTargetName").map(String::from).unwrap_or_default(),
            hand_mode_params: match inst.get("handModeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerAnimatedInteractionHandParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerAnimatedInteractionHandParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionBase {
}

impl Pooled for PlayerAnimatedInteractionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_animated_interaction_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_animated_interaction_base }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionBase {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionTemplate {
    /// DCB field: `playerAnimatedInteraction` (Class)
    #[serde(default)]
    pub player_animated_interaction: Option<Handle<PlayerAnimatedInteraction>>,
}

impl Pooled for PlayerAnimatedInteractionTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_animated_interaction_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_animated_interaction_template }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionTemplate {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            player_animated_interaction: match inst.get("playerAnimatedInteraction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerAnimatedInteraction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerAnimatedInteraction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionWalkingRequestParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionWalkingRequestParams {
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `speed` (Single)
    #[serde(default)]
    pub speed: f32,
}

impl Pooled for PlayerAnimatedInteractionWalkingRequestParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_animated_interaction_walking_request_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_animated_interaction_walking_request_params }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionWalkingRequestParams {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionWalkingRequestParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speed: inst.get_f32("speed").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionFiltered`
///
/// Inherits from: `ActorMotionStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionFiltered {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `filterByLocomotionSet` (EnumChoice)
    #[serde(default)]
    pub filter_by_locomotion_set: String,
    /// DCB field: `params` (Class)
    #[serde(default)]
    pub params: Option<Handle<PlayerAnimatedInteractionWalkingRequestParams>>,
}

impl Pooled for PlayerAnimatedInteractionFiltered {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_animated_interaction_filtered }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_animated_interaction_filtered }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionFiltered {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionFiltered";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            filter_by_locomotion_set: inst.get_str("filterByLocomotionSet").map(String::from).unwrap_or_default(),
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerAnimatedInteractionWalkingRequestParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerAnimatedInteractionWalkingRequestParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionConfig {
    /// DCB field: `walkToAlignParams` (Class)
    #[serde(default)]
    pub walk_to_align_params: Option<Handle<WalkToAlignParams>>,
    /// DCB field: `PlayerAnimatedInteractionStanceConfigs` (Class (array))
    #[serde(default)]
    pub player_animated_interaction_stance_configs: Vec<Handle<PlayerAnimatedInteractionFiltered>>,
    /// DCB field: `AnimActionList` (Class (array))
    #[serde(default)]
    pub anim_action_list: Vec<Handle<AnimatedAction>>,
}

impl Pooled for PlayerAnimatedInteractionConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_animated_interaction_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_animated_interaction_config }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionConfig {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            walk_to_align_params: match inst.get("walkToAlignParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WalkToAlignParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WalkToAlignParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_animated_interaction_stance_configs: inst.get_array("PlayerAnimatedInteractionStanceConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerAnimatedInteractionFiltered>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerAnimatedInteractionFiltered>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            anim_action_list: inst.get_array("AnimActionList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnimatedAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnimatedAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_InteractionModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_InteractionModifier {
    /// DCB field: `highlightFactor` (Single)
    #[serde(default)]
    pub highlight_factor: f32,
    /// DCB field: `rangeFactor` (Single)
    #[serde(default)]
    pub range_factor: f32,
}

impl Pooled for PlayerChoice_InteractionModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_interaction_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_interaction_modifier }
}

impl<'a> Extract<'a> for PlayerChoice_InteractionModifier {
    const TYPE_NAME: &'static str = "PlayerChoice_InteractionModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            highlight_factor: inst.get_f32("highlightFactor").unwrap_or_default(),
            range_factor: inst.get_f32("rangeFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_SignalConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_SignalConfig {
    /// DCB field: `zoneQueryFrequency` (Single)
    #[serde(default)]
    pub zone_query_frequency: f32,
    /// DCB field: `distantIndicatorZoneQueryFrequency` (Single)
    #[serde(default)]
    pub distant_indicator_zone_query_frequency: f32,
    /// DCB field: `baseColor` (Class)
    #[serde(default)]
    pub base_color: Option<Handle<RGBA>>,
    /// DCB field: `baseEffectWidth` (Single)
    #[serde(default)]
    pub base_effect_width: f32,
    /// DCB field: `viewMaxDistance` (Single)
    #[serde(default)]
    pub view_max_distance: f32,
    /// DCB field: `viewMaxDistanceIM` (Single)
    #[serde(default)]
    pub view_max_distance_im: f32,
    /// DCB field: `viewMaxDistantIndicatorDist` (Single)
    #[serde(default)]
    pub view_max_distant_indicator_dist: f32,
    /// DCB field: `viewFadeDistance` (Single)
    #[serde(default)]
    pub view_fade_distance: f32,
    /// DCB field: `viewFadeDistanceIM` (Single)
    #[serde(default)]
    pub view_fade_distance_im: f32,
    /// DCB field: `maxVisibilityTraces` (Byte)
    #[serde(default)]
    pub max_visibility_traces: u32,
    /// DCB field: `minVisibilityWait` (Byte)
    #[serde(default)]
    pub min_visibility_wait: u32,
    /// DCB field: `useTwoColors` (Boolean)
    #[serde(default)]
    pub use_two_colors: bool,
    /// DCB field: `secondaryColor` (Class)
    #[serde(default)]
    pub secondary_color: Option<Handle<RGBA>>,
    /// DCB field: `secondaryEffectWidth` (Single)
    #[serde(default)]
    pub secondary_effect_width: f32,
    /// DCB field: `maxDistanceFromCursorHighlight` (Single)
    #[serde(default)]
    pub max_distance_from_cursor_highlight: f32,
    /// DCB field: `maxDistanceFromCursorHighlightIM` (Single)
    #[serde(default)]
    pub max_distance_from_cursor_highlight_im: f32,
    /// DCB field: `showSignalForInspectedItems` (Boolean)
    #[serde(default)]
    pub show_signal_for_inspected_items: bool,
    /// DCB field: `useHipDistance` (Boolean)
    #[serde(default)]
    pub use_hip_distance: bool,
    /// DCB field: `interactionModifiers` (Class)
    #[serde(default)]
    pub interaction_modifiers: Option<Handle<PlayerChoice_InteractionModifier>>,
    /// DCB field: `dashboardSwitchHighlightRadius` (Single)
    #[serde(default)]
    pub dashboard_switch_highlight_radius: f32,
    /// DCB field: `dashboardSwitchHighlightCentreSize` (Single)
    #[serde(default)]
    pub dashboard_switch_highlight_centre_size: f32,
    /// DCB field: `dashboardSwitchItemType` (EnumChoice)
    #[serde(default)]
    pub dashboard_switch_item_type: String,
}

impl Pooled for PlayerChoice_SignalConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_signal_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_signal_config }
}

impl<'a> Extract<'a> for PlayerChoice_SignalConfig {
    const TYPE_NAME: &'static str = "PlayerChoice_SignalConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zone_query_frequency: inst.get_f32("zoneQueryFrequency").unwrap_or_default(),
            distant_indicator_zone_query_frequency: inst.get_f32("distantIndicatorZoneQueryFrequency").unwrap_or_default(),
            base_color: match inst.get("baseColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            base_effect_width: inst.get_f32("baseEffectWidth").unwrap_or_default(),
            view_max_distance: inst.get_f32("viewMaxDistance").unwrap_or_default(),
            view_max_distance_im: inst.get_f32("viewMaxDistanceIM").unwrap_or_default(),
            view_max_distant_indicator_dist: inst.get_f32("viewMaxDistantIndicatorDist").unwrap_or_default(),
            view_fade_distance: inst.get_f32("viewFadeDistance").unwrap_or_default(),
            view_fade_distance_im: inst.get_f32("viewFadeDistanceIM").unwrap_or_default(),
            max_visibility_traces: inst.get_u32("maxVisibilityTraces").unwrap_or_default(),
            min_visibility_wait: inst.get_u32("minVisibilityWait").unwrap_or_default(),
            use_two_colors: inst.get_bool("useTwoColors").unwrap_or_default(),
            secondary_color: match inst.get("secondaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            secondary_effect_width: inst.get_f32("secondaryEffectWidth").unwrap_or_default(),
            max_distance_from_cursor_highlight: inst.get_f32("maxDistanceFromCursorHighlight").unwrap_or_default(),
            max_distance_from_cursor_highlight_im: inst.get_f32("maxDistanceFromCursorHighlightIM").unwrap_or_default(),
            show_signal_for_inspected_items: inst.get_bool("showSignalForInspectedItems").unwrap_or_default(),
            use_hip_distance: inst.get_bool("useHipDistance").unwrap_or_default(),
            interaction_modifiers: match inst.get("interactionModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_InteractionModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_InteractionModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dashboard_switch_highlight_radius: inst.get_f32("dashboardSwitchHighlightRadius").unwrap_or_default(),
            dashboard_switch_highlight_centre_size: inst.get_f32("dashboardSwitchHighlightCentreSize").unwrap_or_default(),
            dashboard_switch_item_type: inst.get_str("dashboardSwitchItemType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_Option`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_Option {
    /// DCB field: `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// DCB field: `id` (Int32)
    #[serde(default)]
    pub id: i32,
    /// DCB field: `isPrimary` (Boolean)
    #[serde(default)]
    pub is_primary: bool,
}

impl Pooled for PlayerChoice_Option {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_option }
}

impl<'a> Extract<'a> for PlayerChoice_Option {
    const TYPE_NAME: &'static str = "PlayerChoice_Option";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            id: inst.get_i32("id").unwrap_or_default(),
            is_primary: inst.get_bool("isPrimary").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_OptionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_OptionList {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `options` (Class (array))
    #[serde(default)]
    pub options: Vec<Handle<PlayerChoice_Option>>,
}

impl Pooled for PlayerChoice_OptionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_option_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_option_list }
}

impl<'a> Extract<'a> for PlayerChoice_OptionList {
    const TYPE_NAME: &'static str = "PlayerChoice_OptionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            options: inst.get_array("options")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerChoice_Option>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerChoice_Option>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_Library`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_Library {
    /// DCB field: `optionLists` (Class (array))
    #[serde(default)]
    pub option_lists: Vec<Handle<PlayerChoice_OptionList>>,
}

impl Pooled for PlayerChoice_Library {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_library }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_library }
}

impl<'a> Extract<'a> for PlayerChoice_Library {
    const TYPE_NAME: &'static str = "PlayerChoice_Library";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            option_lists: inst.get_array("optionLists")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerChoice_OptionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerChoice_OptionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_RemoteCommsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_RemoteCommsConfig {
    /// DCB field: `mobiglasConfig` (Reference)
    #[serde(default)]
    pub mobiglas_config: Option<CigGuid>,
    /// DCB field: `MFDConfig` (Reference)
    #[serde(default)]
    pub mfdconfig: Option<CigGuid>,
    /// DCB field: `visorConfig` (Reference)
    #[serde(default)]
    pub visor_config: Option<CigGuid>,
    /// DCB field: `hologramConfig` (Reference)
    #[serde(default)]
    pub hologram_config: Option<CigGuid>,
}

impl Pooled for PlayerChoice_RemoteCommsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_remote_comms_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_remote_comms_config }
}

impl<'a> Extract<'a> for PlayerChoice_RemoteCommsConfig {
    const TYPE_NAME: &'static str = "PlayerChoice_RemoteCommsConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mobiglas_config: inst.get("mobiglasConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mfdconfig: inst.get("MFDConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            visor_config: inst.get("visorConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            hologram_config: inst.get("hologramConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PlayerChoice_HeadLookParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_HeadLookParams {
    /// DCB field: `innerZoneSize` (Single)
    #[serde(default)]
    pub inner_zone_size: f32,
    /// DCB field: `innerZoneFactor` (Single)
    #[serde(default)]
    pub inner_zone_factor: f32,
    /// DCB field: `outerZoneFactor` (Single)
    #[serde(default)]
    pub outer_zone_factor: f32,
    /// DCB field: `smoothZones` (Boolean)
    #[serde(default)]
    pub smooth_zones: bool,
}

impl Pooled for PlayerChoice_HeadLookParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_head_look_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_head_look_params }
}

impl<'a> Extract<'a> for PlayerChoice_HeadLookParams {
    const TYPE_NAME: &'static str = "PlayerChoice_HeadLookParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            inner_zone_size: inst.get_f32("innerZoneSize").unwrap_or_default(),
            inner_zone_factor: inst.get_f32("innerZoneFactor").unwrap_or_default(),
            outer_zone_factor: inst.get_f32("outerZoneFactor").unwrap_or_default(),
            smooth_zones: inst.get_bool("smoothZones").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_IMConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_IMConfig {
    /// DCB field: `cursorScreenPercentage` (Int32)
    #[serde(default)]
    pub cursor_screen_percentage: i32,
    /// DCB field: `interactionScreenPercentage` (Int32)
    #[serde(default)]
    pub interaction_screen_percentage: i32,
    /// DCB field: `signalConfig` (Reference)
    #[serde(default)]
    pub signal_config: Option<CigGuid>,
    /// DCB field: `interactionConfig` (Reference)
    #[serde(default)]
    pub interaction_config: Option<CigGuid>,
    /// DCB field: `conversationConfig` (Reference)
    #[serde(default)]
    pub conversation_config: Option<CigGuid>,
    /// DCB field: `remoteConfig` (Class)
    #[serde(default)]
    pub remote_config: Option<Handle<PlayerChoice_RemoteCommsConfig>>,
    /// DCB field: `IMFOVFactor` (Single)
    #[serde(default)]
    pub imfovfactor: f32,
    /// DCB field: `FOVLerpSpeed` (Single)
    #[serde(default)]
    pub fovlerp_speed: f32,
    /// DCB field: `focusModeLerpSpeed` (Single)
    #[serde(default)]
    pub focus_mode_lerp_speed: f32,
    /// DCB field: `focusModeFOVFactor` (Single)
    #[serde(default)]
    pub focus_mode_fovfactor: f32,
    /// DCB field: `useFocusModeDOF` (Boolean)
    #[serde(default)]
    pub use_focus_mode_dof: bool,
    /// DCB field: `maxMouseDistanceToThought` (Single)
    #[serde(default)]
    pub max_mouse_distance_to_thought: f32,
    /// DCB field: `mouseDistanceToDriveSelection` (Single)
    #[serde(default)]
    pub mouse_distance_to_drive_selection: f32,
    /// DCB field: `showItemInnerThoughtsInNormalMode` (Boolean)
    #[serde(default)]
    pub show_item_inner_thoughts_in_normal_mode: bool,
    /// DCB field: `interactionPointStickinessFactor` (Single)
    #[serde(default)]
    pub interaction_point_stickiness_factor: f32,
    /// DCB field: `maxSelectableIPAngle` (Single)
    #[serde(default)]
    pub max_selectable_ipangle: f32,
    /// DCB field: `maxAlwaysSelectRadius` (Single)
    #[serde(default)]
    pub max_always_select_radius: f32,
    /// DCB field: `quickInteractUseScoreSystem` (Boolean)
    #[serde(default)]
    pub quick_interact_use_score_system: bool,
    /// DCB field: `quickInteractOnScreenPoints` (Single)
    #[serde(default)]
    pub quick_interact_on_screen_points: f32,
    /// DCB field: `quickInteractAnglePoints` (Single)
    #[serde(default)]
    pub quick_interact_angle_points: f32,
    /// DCB field: `quickInteractDistancePoints` (Single)
    #[serde(default)]
    pub quick_interact_distance_points: f32,
    /// DCB field: `thoughtPosClampFactor` (Single)
    #[serde(default)]
    pub thought_pos_clamp_factor: f32,
    /// DCB field: `throwPercentageIncreaseFactor` (Single)
    #[serde(default)]
    pub throw_percentage_increase_factor: f32,
    /// DCB field: `quickPlaceThrowPercentage` (Single)
    #[serde(default)]
    pub quick_place_throw_percentage: f32,
    /// DCB field: `quickPlaceDropTime` (Single)
    #[serde(default)]
    pub quick_place_drop_time: f32,
    /// DCB field: `throwMaxImpulse` (Single)
    #[serde(default)]
    pub throw_max_impulse: f32,
    /// DCB field: `throwMaxEpsilon` (Single)
    #[serde(default)]
    pub throw_max_epsilon: f32,
    /// DCB field: `loosePlacementColor` (Class)
    #[serde(default)]
    pub loose_placement_color: Option<Handle<RGBA>>,
    /// DCB field: `snapPlacementColor` (Class)
    #[serde(default)]
    pub snap_placement_color: Option<Handle<RGBA>>,
    /// DCB field: `placementAttacherValidColor` (Class)
    #[serde(default)]
    pub placement_attacher_valid_color: Option<Handle<RGBA>>,
    /// DCB field: `soundsRecord` (Reference)
    #[serde(default)]
    pub sounds_record: Option<CigGuid>,
    /// DCB field: `holoSnapAudioID` (String)
    #[serde(default)]
    pub holo_snap_audio_id: String,
    /// DCB field: `screenSpaceHoloSnapDistance` (Single)
    #[serde(default)]
    pub screen_space_holo_snap_distance: f32,
    /// DCB field: `cursorSpeedMouseFactorIM` (Single)
    #[serde(default)]
    pub cursor_speed_mouse_factor_im: f32,
    /// DCB field: `cursorSpeedMouseFactorFM` (Single)
    #[serde(default)]
    pub cursor_speed_mouse_factor_fm: f32,
    /// DCB field: `cursorSpeedPadFactorIM` (Single)
    #[serde(default)]
    pub cursor_speed_pad_factor_im: f32,
    /// DCB field: `cursorSpeedPadFactorFM` (Single)
    #[serde(default)]
    pub cursor_speed_pad_factor_fm: f32,
    /// DCB field: `cursorSpeedAimingVehicleFactor` (Single)
    #[serde(default)]
    pub cursor_speed_aiming_vehicle_factor: f32,
    /// DCB field: `allowedCameraViewsForIM` (EnumChoice (array))
    #[serde(default)]
    pub allowed_camera_views_for_im: Vec<String>,
    /// DCB field: `snapToFirstPerson` (Boolean)
    #[serde(default)]
    pub snap_to_first_person: bool,
    /// DCB field: `procBreathingSetup` (Reference)
    #[serde(default)]
    pub proc_breathing_setup: Option<CigGuid>,
    /// DCB field: `xDirTowardsCenterFactor` (Single)
    #[serde(default)]
    pub x_dir_towards_center_factor: f32,
    /// DCB field: `xDirAwayFromCenterFactor` (Single)
    #[serde(default)]
    pub x_dir_away_from_center_factor: f32,
    /// DCB field: `yDirTowardsCenterFactor` (Single)
    #[serde(default)]
    pub y_dir_towards_center_factor: f32,
    /// DCB field: `yDirAwayFromCenterFactor` (Single)
    #[serde(default)]
    pub y_dir_away_from_center_factor: f32,
    /// DCB field: `zoomChangePerInput` (Single)
    #[serde(default)]
    pub zoom_change_per_input: f32,
    /// DCB field: `zoomAccPeriod` (Single)
    #[serde(default)]
    pub zoom_acc_period: f32,
    /// DCB field: `zoomAccFactor` (Single)
    #[serde(default)]
    pub zoom_acc_factor: f32,
    /// DCB field: `IMFOVFactorMin` (Single)
    #[serde(default)]
    pub imfovfactor_min: f32,
    /// DCB field: `focusModeDeadzone` (Class)
    #[serde(default)]
    pub focus_mode_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `IMDefaultZoomDeadzone` (Class)
    #[serde(default)]
    pub imdefault_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `IMFullZoomDeadzone` (Class)
    #[serde(default)]
    pub imfull_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `IMDefaultVehicleSeatZoomDeadzone` (Class)
    #[serde(default)]
    pub imdefault_vehicle_seat_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `IMFullVehicleSeatZoomDeadzone` (Class)
    #[serde(default)]
    pub imfull_vehicle_seat_zoom_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `conversationDeadzone` (Class)
    #[serde(default)]
    pub conversation_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `PITDeadzone` (Class)
    #[serde(default)]
    pub pitdeadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `worldDisplayDeadzone` (Class)
    #[serde(default)]
    pub world_display_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `screenFocusDeadzone` (Class)
    #[serde(default)]
    pub screen_focus_deadzone: Option<Handle<PlayerChoice_HeadLookParams>>,
    /// DCB field: `screenFocusLerpSpeed` (Single)
    #[serde(default)]
    pub screen_focus_lerp_speed: f32,
    /// DCB field: `screenFocusCancelDistance` (Single)
    #[serde(default)]
    pub screen_focus_cancel_distance: f32,
    /// DCB field: `screenFocusSelectionAngleRange` (Single)
    #[serde(default)]
    pub screen_focus_selection_angle_range: f32,
    /// DCB field: `blockedConditionCheckTimer` (Single)
    #[serde(default)]
    pub blocked_condition_check_timer: f32,
    /// DCB field: `cursorEdgeRotationSpeed` (Single)
    #[serde(default)]
    pub cursor_edge_rotation_speed: f32,
    /// DCB field: `includeOriginalTextInBlockedText` (Boolean)
    #[serde(default)]
    pub include_original_text_in_blocked_text: bool,
    /// DCB field: `quickSelectTimer` (Single)
    #[serde(default)]
    pub quick_select_timer: f32,
    /// DCB field: `heldEntityMaxCursorDistance` (Single)
    #[serde(default)]
    pub held_entity_max_cursor_distance: f32,
    /// DCB field: `promptDisplayFullRadius` (Single)
    #[serde(default)]
    pub prompt_display_full_radius: f32,
    /// DCB field: `promptDisplayCentreRadius` (Single)
    #[serde(default)]
    pub prompt_display_centre_radius: f32,
    /// DCB field: `IMSelectionScoreCutoff` (Single)
    #[serde(default)]
    pub imselection_score_cutoff: f32,
    /// DCB field: `interactionPromptPreviousUpdateTime` (Single)
    #[serde(default)]
    pub interaction_prompt_previous_update_time: f32,
}

impl Pooled for PlayerChoice_IMConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_imconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_imconfig }
}

impl<'a> Extract<'a> for PlayerChoice_IMConfig {
    const TYPE_NAME: &'static str = "PlayerChoice_IMConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cursor_screen_percentage: inst.get_i32("cursorScreenPercentage").unwrap_or_default(),
            interaction_screen_percentage: inst.get_i32("interactionScreenPercentage").unwrap_or_default(),
            signal_config: inst.get("signalConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interaction_config: inst.get("interactionConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            conversation_config: inst.get("conversationConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            remote_config: match inst.get("remoteConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_RemoteCommsConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_RemoteCommsConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imfovfactor: inst.get_f32("IMFOVFactor").unwrap_or_default(),
            fovlerp_speed: inst.get_f32("FOVLerpSpeed").unwrap_or_default(),
            focus_mode_lerp_speed: inst.get_f32("focusModeLerpSpeed").unwrap_or_default(),
            focus_mode_fovfactor: inst.get_f32("focusModeFOVFactor").unwrap_or_default(),
            use_focus_mode_dof: inst.get_bool("useFocusModeDOF").unwrap_or_default(),
            max_mouse_distance_to_thought: inst.get_f32("maxMouseDistanceToThought").unwrap_or_default(),
            mouse_distance_to_drive_selection: inst.get_f32("mouseDistanceToDriveSelection").unwrap_or_default(),
            show_item_inner_thoughts_in_normal_mode: inst.get_bool("showItemInnerThoughtsInNormalMode").unwrap_or_default(),
            interaction_point_stickiness_factor: inst.get_f32("interactionPointStickinessFactor").unwrap_or_default(),
            max_selectable_ipangle: inst.get_f32("maxSelectableIPAngle").unwrap_or_default(),
            max_always_select_radius: inst.get_f32("maxAlwaysSelectRadius").unwrap_or_default(),
            quick_interact_use_score_system: inst.get_bool("quickInteractUseScoreSystem").unwrap_or_default(),
            quick_interact_on_screen_points: inst.get_f32("quickInteractOnScreenPoints").unwrap_or_default(),
            quick_interact_angle_points: inst.get_f32("quickInteractAnglePoints").unwrap_or_default(),
            quick_interact_distance_points: inst.get_f32("quickInteractDistancePoints").unwrap_or_default(),
            thought_pos_clamp_factor: inst.get_f32("thoughtPosClampFactor").unwrap_or_default(),
            throw_percentage_increase_factor: inst.get_f32("throwPercentageIncreaseFactor").unwrap_or_default(),
            quick_place_throw_percentage: inst.get_f32("quickPlaceThrowPercentage").unwrap_or_default(),
            quick_place_drop_time: inst.get_f32("quickPlaceDropTime").unwrap_or_default(),
            throw_max_impulse: inst.get_f32("throwMaxImpulse").unwrap_or_default(),
            throw_max_epsilon: inst.get_f32("throwMaxEpsilon").unwrap_or_default(),
            loose_placement_color: match inst.get("loosePlacementColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            snap_placement_color: match inst.get("snapPlacementColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            placement_attacher_valid_color: match inst.get("placementAttacherValidColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sounds_record: inst.get("soundsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            holo_snap_audio_id: inst.get_str("holoSnapAudioID").map(String::from).unwrap_or_default(),
            screen_space_holo_snap_distance: inst.get_f32("screenSpaceHoloSnapDistance").unwrap_or_default(),
            cursor_speed_mouse_factor_im: inst.get_f32("cursorSpeedMouseFactorIM").unwrap_or_default(),
            cursor_speed_mouse_factor_fm: inst.get_f32("cursorSpeedMouseFactorFM").unwrap_or_default(),
            cursor_speed_pad_factor_im: inst.get_f32("cursorSpeedPadFactorIM").unwrap_or_default(),
            cursor_speed_pad_factor_fm: inst.get_f32("cursorSpeedPadFactorFM").unwrap_or_default(),
            cursor_speed_aiming_vehicle_factor: inst.get_f32("cursorSpeedAimingVehicleFactor").unwrap_or_default(),
            allowed_camera_views_for_im: inst.get_array("allowedCameraViewsForIM")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            snap_to_first_person: inst.get_bool("snapToFirstPerson").unwrap_or_default(),
            proc_breathing_setup: inst.get("procBreathingSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            x_dir_towards_center_factor: inst.get_f32("xDirTowardsCenterFactor").unwrap_or_default(),
            x_dir_away_from_center_factor: inst.get_f32("xDirAwayFromCenterFactor").unwrap_or_default(),
            y_dir_towards_center_factor: inst.get_f32("yDirTowardsCenterFactor").unwrap_or_default(),
            y_dir_away_from_center_factor: inst.get_f32("yDirAwayFromCenterFactor").unwrap_or_default(),
            zoom_change_per_input: inst.get_f32("zoomChangePerInput").unwrap_or_default(),
            zoom_acc_period: inst.get_f32("zoomAccPeriod").unwrap_or_default(),
            zoom_acc_factor: inst.get_f32("zoomAccFactor").unwrap_or_default(),
            imfovfactor_min: inst.get_f32("IMFOVFactorMin").unwrap_or_default(),
            focus_mode_deadzone: match inst.get("focusModeDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imdefault_zoom_deadzone: match inst.get("IMDefaultZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imfull_zoom_deadzone: match inst.get("IMFullZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imdefault_vehicle_seat_zoom_deadzone: match inst.get("IMDefaultVehicleSeatZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            imfull_vehicle_seat_zoom_deadzone: match inst.get("IMFullVehicleSeatZoomDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            conversation_deadzone: match inst.get("conversationDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitdeadzone: match inst.get("PITDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            world_display_deadzone: match inst.get("worldDisplayDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            screen_focus_deadzone: match inst.get("screenFocusDeadzone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_HeadLookParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            screen_focus_lerp_speed: inst.get_f32("screenFocusLerpSpeed").unwrap_or_default(),
            screen_focus_cancel_distance: inst.get_f32("screenFocusCancelDistance").unwrap_or_default(),
            screen_focus_selection_angle_range: inst.get_f32("screenFocusSelectionAngleRange").unwrap_or_default(),
            blocked_condition_check_timer: inst.get_f32("blockedConditionCheckTimer").unwrap_or_default(),
            cursor_edge_rotation_speed: inst.get_f32("cursorEdgeRotationSpeed").unwrap_or_default(),
            include_original_text_in_blocked_text: inst.get_bool("includeOriginalTextInBlockedText").unwrap_or_default(),
            quick_select_timer: inst.get_f32("quickSelectTimer").unwrap_or_default(),
            held_entity_max_cursor_distance: inst.get_f32("heldEntityMaxCursorDistance").unwrap_or_default(),
            prompt_display_full_radius: inst.get_f32("promptDisplayFullRadius").unwrap_or_default(),
            prompt_display_centre_radius: inst.get_f32("promptDisplayCentreRadius").unwrap_or_default(),
            imselection_score_cutoff: inst.get_f32("IMSelectionScoreCutoff").unwrap_or_default(),
            interaction_prompt_previous_update_time: inst.get_f32("interactionPromptPreviousUpdateTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoiceMenuItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoiceMenuItem {
    /// DCB field: `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
}

impl Pooled for PlayerChoiceMenuItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_menu_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_menu_item }
}

impl<'a> Extract<'a> for PlayerChoiceMenuItem {
    const TYPE_NAME: &'static str = "PlayerChoiceMenuItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PlayerChoiceMenuOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoiceMenuOption {
}

impl Pooled for PlayerChoiceMenuOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_menu_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_menu_option }
}

impl<'a> Extract<'a> for PlayerChoiceMenuOption {
    const TYPE_NAME: &'static str = "PlayerChoiceMenuOption";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlayerChoiceMenuItems`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoiceMenuItems {
    /// DCB field: `allAvailableItems` (Reference (array))
    #[serde(default)]
    pub all_available_items: Vec<CigGuid>,
}

impl Pooled for PlayerChoiceMenuItems {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_menu_items }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_menu_items }
}

impl<'a> Extract<'a> for PlayerChoiceMenuItems {
    const TYPE_NAME: &'static str = "PlayerChoiceMenuItems";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            all_available_items: inst.get_array("allAvailableItems")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoiceMenu`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoiceMenu {
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// DCB field: `blockedSelectedColor` (Reference)
    #[serde(default)]
    pub blocked_selected_color: Option<CigGuid>,
    /// DCB field: `blockedUnselectedColor` (Reference)
    #[serde(default)]
    pub blocked_unselected_color: Option<CigGuid>,
    /// DCB field: `options` (StrongPointer (array))
    #[serde(default)]
    pub options: Vec<Handle<PlayerChoiceMenuOption>>,
}

impl Pooled for PlayerChoiceMenu {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_menu }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_menu }
}

impl<'a> Extract<'a> for PlayerChoiceMenu {
    const TYPE_NAME: &'static str = "PlayerChoiceMenu";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            blocked_selected_color: inst.get("blockedSelectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            blocked_unselected_color: inst.get("blockedUnselectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            options: inst.get_array("options")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerChoiceMenuOption>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerChoiceMenuOption>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoiceMenuType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoiceMenuType {
    /// DCB field: `focusedConversation` (Boolean)
    #[serde(default)]
    pub focused_conversation: bool,
    /// DCB field: `menus` (Reference (array))
    #[serde(default)]
    pub menus: Vec<CigGuid>,
}

impl Pooled for PlayerChoiceMenuType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_menu_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_menu_type }
}

impl<'a> Extract<'a> for PlayerChoiceMenuType {
    const TYPE_NAME: &'static str = "PlayerChoiceMenuType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            focused_conversation: inst.get_bool("focusedConversation").unwrap_or_default(),
            menus: inst.get_array("menus")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerDockContextComponentGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerDockContextComponentGlobalParams {
    /// DCB field: `gameNotificationDockItemParams` (Reference)
    #[serde(default)]
    pub game_notification_dock_item_params: Option<CigGuid>,
}

impl Pooled for PlayerDockContextComponentGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_dock_context_component_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_dock_context_component_global_params }
}

impl<'a> Extract<'a> for PlayerDockContextComponentGlobalParams {
    const TYPE_NAME: &'static str = "PlayerDockContextComponentGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            game_notification_dock_item_params: inst.get("gameNotificationDockItemParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PlayerGroupManagerObjectsLocStringParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGroupManagerObjectsLocStringParams {
    /// DCB field: `partyName` (Locale)
    #[serde(default)]
    pub party_name: String,
}

impl Pooled for PlayerGroupManagerObjectsLocStringParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_group_manager_objects_loc_string_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_group_manager_objects_loc_string_params }
}

impl<'a> Extract<'a> for PlayerGroupManagerObjectsLocStringParams {
    const TYPE_NAME: &'static str = "PlayerGroupManagerObjectsLocStringParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            party_name: inst.get_str("partyName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerGroupManagerLocStringParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGroupManagerLocStringParams {
    /// DCB field: `partyNameFormat` (Locale)
    #[serde(default)]
    pub party_name_format: String,
    /// DCB field: `groupNameFormat` (Locale)
    #[serde(default)]
    pub group_name_format: String,
    /// DCB field: `channelNameFormat` (Locale)
    #[serde(default)]
    pub channel_name_format: String,
    /// DCB field: `inviteSent` (Locale)
    #[serde(default)]
    pub invite_sent: String,
    /// DCB field: `inviteReceived` (Locale)
    #[serde(default)]
    pub invite_received: String,
    /// DCB field: `memberJoined` (Locale)
    #[serde(default)]
    pub member_joined: String,
    /// DCB field: `inviteDeclined` (Locale)
    #[serde(default)]
    pub invite_declined: String,
    /// DCB field: `localPlayerJoined` (Locale)
    #[serde(default)]
    pub local_player_joined: String,
    /// DCB field: `localPlayerJoinedAuto` (Locale)
    #[serde(default)]
    pub local_player_joined_auto: String,
    /// DCB field: `localPlayerLeft` (Locale)
    #[serde(default)]
    pub local_player_left: String,
    /// DCB field: `localPlayerKicked` (Locale)
    #[serde(default)]
    pub local_player_kicked: String,
    /// DCB field: `groupDisbanded` (Locale)
    #[serde(default)]
    pub group_disbanded: String,
    /// DCB field: `groupCreated` (Locale)
    #[serde(default)]
    pub group_created: String,
    /// DCB field: `leaderChanged` (Locale)
    #[serde(default)]
    pub leader_changed: String,
    /// DCB field: `memberLeft` (Locale)
    #[serde(default)]
    pub member_left: String,
    /// DCB field: `memberKicked` (Locale)
    #[serde(default)]
    pub member_kicked: String,
    /// DCB field: `inviteTimeout` (Locale)
    #[serde(default)]
    pub invite_timeout: String,
    /// DCB field: `invitationFailed` (Locale)
    #[serde(default)]
    pub invitation_failed: String,
}

impl Pooled for PlayerGroupManagerLocStringParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_group_manager_loc_string_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_group_manager_loc_string_params }
}

impl<'a> Extract<'a> for PlayerGroupManagerLocStringParams {
    const TYPE_NAME: &'static str = "PlayerGroupManagerLocStringParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            party_name_format: inst.get_str("partyNameFormat").map(String::from).unwrap_or_default(),
            group_name_format: inst.get_str("groupNameFormat").map(String::from).unwrap_or_default(),
            channel_name_format: inst.get_str("channelNameFormat").map(String::from).unwrap_or_default(),
            invite_sent: inst.get_str("inviteSent").map(String::from).unwrap_or_default(),
            invite_received: inst.get_str("inviteReceived").map(String::from).unwrap_or_default(),
            member_joined: inst.get_str("memberJoined").map(String::from).unwrap_or_default(),
            invite_declined: inst.get_str("inviteDeclined").map(String::from).unwrap_or_default(),
            local_player_joined: inst.get_str("localPlayerJoined").map(String::from).unwrap_or_default(),
            local_player_joined_auto: inst.get_str("localPlayerJoinedAuto").map(String::from).unwrap_or_default(),
            local_player_left: inst.get_str("localPlayerLeft").map(String::from).unwrap_or_default(),
            local_player_kicked: inst.get_str("localPlayerKicked").map(String::from).unwrap_or_default(),
            group_disbanded: inst.get_str("groupDisbanded").map(String::from).unwrap_or_default(),
            group_created: inst.get_str("groupCreated").map(String::from).unwrap_or_default(),
            leader_changed: inst.get_str("leaderChanged").map(String::from).unwrap_or_default(),
            member_left: inst.get_str("memberLeft").map(String::from).unwrap_or_default(),
            member_kicked: inst.get_str("memberKicked").map(String::from).unwrap_or_default(),
            invite_timeout: inst.get_str("inviteTimeout").map(String::from).unwrap_or_default(),
            invitation_failed: inst.get_str("invitationFailed").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerGroupManagerNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGroupManagerNotificationParams {
    /// DCB field: `notificationParams` (Class)
    #[serde(default)]
    pub notification_params: Option<Handle<PlayerNotificationBannerParams>>,
    /// DCB field: `groupSubscriptionTypes` (EnumChoice (array))
    #[serde(default)]
    pub group_subscription_types: Vec<String>,
}

impl Pooled for PlayerGroupManagerNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_group_manager_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_group_manager_notification_params }
}

impl<'a> Extract<'a> for PlayerGroupManagerNotificationParams {
    const TYPE_NAME: &'static str = "PlayerGroupManagerNotificationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            notification_params: match inst.get("notificationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            group_subscription_types: inst.get_array("groupSubscriptionTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerGroupManagerNotificationsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGroupManagerNotificationsParams {
    /// DCB field: `inviteSent` (Class)
    #[serde(default)]
    pub invite_sent: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `inviteReceived` (Class)
    #[serde(default)]
    pub invite_received: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `memberJoined` (Class)
    #[serde(default)]
    pub member_joined: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `inviteDeclined` (Class)
    #[serde(default)]
    pub invite_declined: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `localPlayerJoined` (Class)
    #[serde(default)]
    pub local_player_joined: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `localPlayerJoinedAuto` (Class)
    #[serde(default)]
    pub local_player_joined_auto: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `localPlayerLeft` (Class)
    #[serde(default)]
    pub local_player_left: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `localPlayerKicked` (Class)
    #[serde(default)]
    pub local_player_kicked: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `groupDisbanded` (Class)
    #[serde(default)]
    pub group_disbanded: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `groupCreated` (Class)
    #[serde(default)]
    pub group_created: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `leaderChanged` (Class)
    #[serde(default)]
    pub leader_changed: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `memberLeft` (Class)
    #[serde(default)]
    pub member_left: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `memberKicked` (Class)
    #[serde(default)]
    pub member_kicked: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `inviteTimeout` (Class)
    #[serde(default)]
    pub invite_timeout: Option<Handle<PlayerGroupManagerNotificationParams>>,
    /// DCB field: `invitationFailed` (Class)
    #[serde(default)]
    pub invitation_failed: Option<Handle<PlayerGroupManagerNotificationParams>>,
}

impl Pooled for PlayerGroupManagerNotificationsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_group_manager_notifications_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_group_manager_notifications_params }
}

impl<'a> Extract<'a> for PlayerGroupManagerNotificationsParams {
    const TYPE_NAME: &'static str = "PlayerGroupManagerNotificationsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            invite_sent: match inst.get("inviteSent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invite_received: match inst.get("inviteReceived") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            member_joined: match inst.get("memberJoined") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invite_declined: match inst.get("inviteDeclined") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            local_player_joined: match inst.get("localPlayerJoined") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            local_player_joined_auto: match inst.get("localPlayerJoinedAuto") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            local_player_left: match inst.get("localPlayerLeft") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            local_player_kicked: match inst.get("localPlayerKicked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            group_disbanded: match inst.get("groupDisbanded") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            group_created: match inst.get("groupCreated") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            leader_changed: match inst.get("leaderChanged") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            member_left: match inst.get("memberLeft") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            member_kicked: match inst.get("memberKicked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invite_timeout: match inst.get("inviteTimeout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invitation_failed: match inst.get("invitationFailed") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerGroupManagerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGroupManagerGlobalParams {
    /// DCB field: `objectLocStrings` (Class)
    #[serde(default)]
    pub object_loc_strings: Option<Handle<PlayerGroupManagerObjectsLocStringParams>>,
    /// DCB field: `localizationStringParams` (Class)
    #[serde(default)]
    pub localization_string_params: Option<Handle<PlayerGroupManagerLocStringParams>>,
    /// DCB field: `notificationsParams` (Class)
    #[serde(default)]
    pub notifications_params: Option<Handle<PlayerGroupManagerNotificationsParams>>,
}

impl Pooled for PlayerGroupManagerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_group_manager_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_group_manager_global_params }
}

impl<'a> Extract<'a> for PlayerGroupManagerGlobalParams {
    const TYPE_NAME: &'static str = "PlayerGroupManagerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            object_loc_strings: match inst.get("objectLocStrings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerObjectsLocStringParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerObjectsLocStringParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            localization_string_params: match inst.get("localizationStringParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerLocStringParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerLocStringParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notifications_params: match inst.get("notificationsParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerGroupManagerNotificationsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerGroupManagerNotificationsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerLimitationsProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLimitationsProfile {
    /// DCB field: `abilitiesToBlock` (EnumChoice (array))
    #[serde(default)]
    pub abilities_to_block: Vec<String>,
}

impl Pooled for PlayerLimitationsProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_limitations_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_limitations_profile }
}

impl<'a> Extract<'a> for PlayerLimitationsProfile {
    const TYPE_NAME: &'static str = "PlayerLimitationsProfile";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            abilities_to_block: inst.get_array("abilitiesToBlock")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerNotificationBannerManagerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerNotificationBannerManagerGlobalParams {
    /// DCB field: `notificationAutoExpirationUpdaterIntervalSeconds` (Single)
    #[serde(default)]
    pub notification_auto_expiration_updater_interval_seconds: f32,
}

impl Pooled for PlayerNotificationBannerManagerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_notification_banner_manager_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_notification_banner_manager_global_params }
}

impl<'a> Extract<'a> for PlayerNotificationBannerManagerGlobalParams {
    const TYPE_NAME: &'static str = "PlayerNotificationBannerManagerGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            notification_auto_expiration_updater_interval_seconds: inst.get_f32("notificationAutoExpirationUpdaterIntervalSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerNotificationBannerOptionsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerNotificationBannerOptionsParams {
    /// DCB field: `autoExpireInSeconds` (Single)
    #[serde(default)]
    pub auto_expire_in_seconds: f32,
}

impl Pooled for PlayerNotificationBannerOptionsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_notification_banner_options_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_notification_banner_options_params }
}

impl<'a> Extract<'a> for PlayerNotificationBannerOptionsParams {
    const TYPE_NAME: &'static str = "PlayerNotificationBannerOptionsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            auto_expire_in_seconds: inst.get_f32("autoExpireInSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerNotificationBannerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerNotificationBannerParams {
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `desc` (Locale)
    #[serde(default)]
    pub desc: String,
    /// DCB field: `prompt` (Locale)
    #[serde(default)]
    pub prompt: String,
    /// DCB field: `options` (Class)
    #[serde(default)]
    pub options: Option<Handle<PlayerNotificationBannerOptionsParams>>,
}

impl Pooled for PlayerNotificationBannerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_notification_banner_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_notification_banner_params }
}

impl<'a> Extract<'a> for PlayerNotificationBannerParams {
    const TYPE_NAME: &'static str = "PlayerNotificationBannerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            desc: inst.get_str("desc").map(String::from).unwrap_or_default(),
            prompt: inst.get_str("prompt").map(String::from).unwrap_or_default(),
            options: match inst.get("options") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerNotificationBannerOptionsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerNotificationBannerOptionsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerShipRespawnShipInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShipRespawnShipInfo {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `RespawnWaitTime` (Int32)
    #[serde(default)]
    pub respawn_wait_time: i32,
    /// DCB field: `InstantRespawnCost` (Int32)
    #[serde(default)]
    pub instant_respawn_cost: i32,
}

impl Pooled for PlayerShipRespawnShipInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_ship_respawn_ship_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_ship_respawn_ship_info }
}

impl<'a> Extract<'a> for PlayerShipRespawnShipInfo {
    const TYPE_NAME: &'static str = "PlayerShipRespawnShipInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            respawn_wait_time: inst.get_i32("RespawnWaitTime").unwrap_or_default(),
            instant_respawn_cost: inst.get_i32("InstantRespawnCost").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerShipRespawn`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShipRespawn {
    /// DCB field: `Ships` (Class (array))
    #[serde(default)]
    pub ships: Vec<Handle<PlayerShipRespawnShipInfo>>,
}

impl Pooled for PlayerShipRespawn {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_ship_respawn }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_ship_respawn }
}

impl<'a> Extract<'a> for PlayerShipRespawn {
    const TYPE_NAME: &'static str = "PlayerShipRespawn";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ships: inst.get_array("Ships")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerTradeNotification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTradeNotification {
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `sendSuccessMessage` (Locale)
    #[serde(default)]
    pub send_success_message: String,
    /// DCB field: `receiveSuccessMessage` (Locale)
    #[serde(default)]
    pub receive_success_message: String,
    /// DCB field: `sendFailedMessage` (Locale)
    #[serde(default)]
    pub send_failed_message: String,
    /// DCB field: `traderNameToken` (String)
    #[serde(default)]
    pub trader_name_token: String,
    /// DCB field: `currencyTypeTokenUEC` (String)
    #[serde(default)]
    pub currency_type_token_uec: String,
    /// DCB field: `currencyTypeTokenMER` (String)
    #[serde(default)]
    pub currency_type_token_mer: String,
    /// DCB field: `currencyValueTokenUEC` (String)
    #[serde(default)]
    pub currency_value_token_uec: String,
    /// DCB field: `currencyValueTokenMER` (String)
    #[serde(default)]
    pub currency_value_token_mer: String,
}

impl Pooled for PlayerTradeNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_trade_notification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_trade_notification }
}

impl<'a> Extract<'a> for PlayerTradeNotification {
    const TYPE_NAME: &'static str = "PlayerTradeNotification";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            send_success_message: inst.get_str("sendSuccessMessage").map(String::from).unwrap_or_default(),
            receive_success_message: inst.get_str("receiveSuccessMessage").map(String::from).unwrap_or_default(),
            send_failed_message: inst.get_str("sendFailedMessage").map(String::from).unwrap_or_default(),
            trader_name_token: inst.get_str("traderNameToken").map(String::from).unwrap_or_default(),
            currency_type_token_uec: inst.get_str("currencyTypeTokenUEC").map(String::from).unwrap_or_default(),
            currency_type_token_mer: inst.get_str("currencyTypeTokenMER").map(String::from).unwrap_or_default(),
            currency_value_token_uec: inst.get_str("currencyValueTokenUEC").map(String::from).unwrap_or_default(),
            currency_value_token_mer: inst.get_str("currencyValueTokenMER").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerTradeGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTradeGlobalParams {
    /// DCB field: `taxRate` (Single)
    #[serde(default)]
    pub tax_rate: f32,
    /// DCB field: `taxText` (Locale)
    #[serde(default)]
    pub tax_text: String,
    /// DCB field: `currencyTextUEC` (Locale)
    #[serde(default)]
    pub currency_text_uec: String,
    /// DCB field: `currencyTextREC` (Locale)
    #[serde(default)]
    pub currency_text_rec: String,
    /// DCB field: `currencyTextMER` (Locale)
    #[serde(default)]
    pub currency_text_mer: String,
    /// DCB field: `searchWindowTitleText` (Locale)
    #[serde(default)]
    pub search_window_title_text: String,
    /// DCB field: `resultsListTitleText` (Locale)
    #[serde(default)]
    pub results_list_title_text: String,
    /// DCB field: `partyListTitleText` (Locale)
    #[serde(default)]
    pub party_list_title_text: String,
    /// DCB field: `contactsListTitleText` (Locale)
    #[serde(default)]
    pub contacts_list_title_text: String,
    /// DCB field: `basketWindowTitleText` (Locale)
    #[serde(default)]
    pub basket_window_title_text: String,
    /// DCB field: `sendStatusWindowTitleText` (Locale)
    #[serde(default)]
    pub send_status_window_title_text: String,
    /// DCB field: `searchDelayInMilliSeconds` (Int32)
    #[serde(default)]
    pub search_delay_in_milli_seconds: i32,
    /// DCB field: `searchResultListLimit` (Int32)
    #[serde(default)]
    pub search_result_list_limit: i32,
    /// DCB field: `notification` (Class)
    #[serde(default)]
    pub notification: Option<Handle<PlayerTradeNotification>>,
}

impl Pooled for PlayerTradeGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_trade_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_trade_global_params }
}

impl<'a> Extract<'a> for PlayerTradeGlobalParams {
    const TYPE_NAME: &'static str = "PlayerTradeGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tax_rate: inst.get_f32("taxRate").unwrap_or_default(),
            tax_text: inst.get_str("taxText").map(String::from).unwrap_or_default(),
            currency_text_uec: inst.get_str("currencyTextUEC").map(String::from).unwrap_or_default(),
            currency_text_rec: inst.get_str("currencyTextREC").map(String::from).unwrap_or_default(),
            currency_text_mer: inst.get_str("currencyTextMER").map(String::from).unwrap_or_default(),
            search_window_title_text: inst.get_str("searchWindowTitleText").map(String::from).unwrap_or_default(),
            results_list_title_text: inst.get_str("resultsListTitleText").map(String::from).unwrap_or_default(),
            party_list_title_text: inst.get_str("partyListTitleText").map(String::from).unwrap_or_default(),
            contacts_list_title_text: inst.get_str("contactsListTitleText").map(String::from).unwrap_or_default(),
            basket_window_title_text: inst.get_str("basketWindowTitleText").map(String::from).unwrap_or_default(),
            send_status_window_title_text: inst.get_str("sendStatusWindowTitleText").map(String::from).unwrap_or_default(),
            search_delay_in_milli_seconds: inst.get_i32("searchDelayInMilliSeconds").unwrap_or_default(),
            search_result_list_limit: inst.get_i32("searchResultListLimit").unwrap_or_default(),
            notification: match inst.get("notification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerTradeNotification>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerTradeNotification>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlanetDayNightTemperatureParams`
///
/// Inherits from: `PlanetDayNightTemperatureBaseParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetDayNightTemperatureParams {
    /// DCB field: `DayNightCoolingBaseRate` (Single)
    #[serde(default)]
    pub day_night_cooling_base_rate: f32,
    /// DCB field: `DayNightCoolingHumidityModifier` (Single)
    #[serde(default)]
    pub day_night_cooling_humidity_modifier: f32,
    /// DCB field: `DayNightCoolingHumidityMultiplier` (Single)
    #[serde(default)]
    pub day_night_cooling_humidity_multiplier: f32,
}

impl Pooled for PlanetDayNightTemperatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.planet_day_night_temperature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.planet_day_night_temperature_params }
}

impl<'a> Extract<'a> for PlanetDayNightTemperatureParams {
    const TYPE_NAME: &'static str = "PlanetDayNightTemperatureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            day_night_cooling_base_rate: inst.get_f32("DayNightCoolingBaseRate").unwrap_or_default(),
            day_night_cooling_humidity_modifier: inst.get_f32("DayNightCoolingHumidityModifier").unwrap_or_default(),
            day_night_cooling_humidity_multiplier: inst.get_f32("DayNightCoolingHumidityMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetDayNightTemperatureTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetDayNightTemperatureTemplate {
    /// DCB field: `dayNightTemperatureParams` (Class)
    #[serde(default)]
    pub day_night_temperature_params: Option<Handle<PlanetDayNightTemperatureParams>>,
}

impl Pooled for PlanetDayNightTemperatureTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.planet_day_night_temperature_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.planet_day_night_temperature_template }
}

impl<'a> Extract<'a> for PlanetDayNightTemperatureTemplate {
    const TYPE_NAME: &'static str = "PlanetDayNightTemperatureTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            day_night_temperature_params: match inst.get("dayNightTemperatureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlanetDayNightTemperatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlanetDayNightTemperatureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerToPlayerCommsCallGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerToPlayerCommsCallGlobalParams {
    /// DCB field: `faceLight` (Class)
    #[serde(default)]
    pub face_light: Option<Handle<WebRTCCommsCallProjectorLightParams>>,
    /// DCB field: `environmentLight` (Class)
    #[serde(default)]
    pub environment_light: Option<Handle<WebRTCCommsCallProjectorLightParams>>,
}

impl Pooled for PlayerToPlayerCommsCallGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_to_player_comms_call_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_to_player_comms_call_global_params }
}

impl<'a> Extract<'a> for PlayerToPlayerCommsCallGlobalParams {
    const TYPE_NAME: &'static str = "PlayerToPlayerCommsCallGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            face_light: match inst.get("faceLight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WebRTCCommsCallProjectorLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WebRTCCommsCallProjectorLightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            environment_light: match inst.get("environmentLight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WebRTCCommsCallProjectorLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WebRTCCommsCallProjectorLightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerChoice_PITConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_PITConfig {
    /// DCB field: `maxMenuSize` (Int32)
    #[serde(default)]
    pub max_menu_size: i32,
    /// DCB field: `showDisabledActions` (Boolean)
    #[serde(default)]
    pub show_disabled_actions: bool,
    /// DCB field: `cameraEffects` (Class)
    #[serde(default)]
    pub camera_effects: Option<Handle<PersonalThoughtCameraEffectsParams>>,
    /// DCB field: `forceCloseActions` (Class)
    #[serde(default)]
    pub force_close_actions: Option<Handle<PersonalThoughtForceCloseActionsParams>>,
    /// DCB field: `hologramParams` (Class)
    #[serde(default)]
    pub hologram_params: Option<Handle<PersonalThoughtHologramParams>>,
    /// DCB field: `softAttachColourParams` (Class)
    #[serde(default)]
    pub soft_attach_colour_params: Option<Handle<HUDSilhouetteParams>>,
    /// DCB field: `inventoryCameraConfig` (Reference)
    #[serde(default)]
    pub inventory_camera_config: Option<CigGuid>,
    /// DCB field: `root` (Class)
    #[serde(default)]
    pub root: Option<Handle<PersonalThoughtCategoryAction>>,
    /// DCB field: `actionDescriptionsList` (Class)
    #[serde(default)]
    pub action_descriptions_list: Option<Handle<PersonalThoughtActionDescriptionsList>>,
    /// DCB field: `hologramActionsList` (Class)
    #[serde(default)]
    pub hologram_actions_list: Option<Handle<PersonalThoughtHologramActionsList>>,
    /// DCB field: `contextualActionsMenus` (Class)
    #[serde(default)]
    pub contextual_actions_menus: Option<Handle<PersonalThoughtContextualActionsMenusParams>>,
    /// DCB field: `inventoryParams` (Class)
    #[serde(default)]
    pub inventory_params: Option<Handle<PersonalThoughtInventoryParams>>,
    /// DCB field: `quickAccessWheelsParams` (Class)
    #[serde(default)]
    pub quick_access_wheels_params: Option<Handle<PersonalThoughtQuickAccessWheels>>,
    /// DCB field: `actionRulesParams` (Class)
    #[serde(default)]
    pub action_rules_params: Option<Handle<PersonalThoughtActionsRulesParams>>,
    /// DCB field: `dropDetachRules` (Class (array))
    #[serde(default)]
    pub drop_detach_rules: Vec<Handle<InventoryDropDetachRules>>,
}

impl Pooled for PlayerChoice_PITConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_choice_pitconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_choice_pitconfig }
}

impl<'a> Extract<'a> for PlayerChoice_PITConfig {
    const TYPE_NAME: &'static str = "PlayerChoice_PITConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_menu_size: inst.get_i32("maxMenuSize").unwrap_or_default(),
            show_disabled_actions: inst.get_bool("showDisabledActions").unwrap_or_default(),
            camera_effects: match inst.get("cameraEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtCameraEffectsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtCameraEffectsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            force_close_actions: match inst.get("forceCloseActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtForceCloseActionsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtForceCloseActionsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hologram_params: match inst.get("hologramParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtHologramParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtHologramParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            soft_attach_colour_params: match inst.get("softAttachColourParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HUDSilhouetteParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HUDSilhouetteParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inventory_camera_config: inst.get("inventoryCameraConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            root: match inst.get("root") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtCategoryAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtCategoryAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            action_descriptions_list: match inst.get("actionDescriptionsList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtActionDescriptionsList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtActionDescriptionsList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hologram_actions_list: match inst.get("hologramActionsList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtHologramActionsList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtHologramActionsList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contextual_actions_menus: match inst.get("contextualActionsMenus") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtContextualActionsMenusParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtContextualActionsMenusParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inventory_params: match inst.get("inventoryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtInventoryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtInventoryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            quick_access_wheels_params: match inst.get("quickAccessWheelsParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtQuickAccessWheels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtQuickAccessWheels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            action_rules_params: match inst.get("actionRulesParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtActionsRulesParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtActionsRulesParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drop_detach_rules: inst.get_array("dropDetachRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryDropDetachRules>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryDropDetachRules>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerECGGraph_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerECGGraph_Config {
    /// DCB field: `idleAmpRangeMax` (Single)
    #[serde(default)]
    pub idle_amp_range_max: f32,
    /// DCB field: `spikeFrameDuration` (Byte)
    #[serde(default)]
    pub spike_frame_duration: u32,
    /// DCB field: `spikeFrameDurationRandFactor` (Single)
    #[serde(default)]
    pub spike_frame_duration_rand_factor: f32,
    /// DCB field: `waveFreq` (Byte)
    #[serde(default)]
    pub wave_freq: u32,
    /// DCB field: `pulseAmpMin` (Single)
    #[serde(default)]
    pub pulse_amp_min: f32,
    /// DCB field: `pulseAmpMax` (Single)
    #[serde(default)]
    pub pulse_amp_max: f32,
    /// DCB field: `updateRate` (Byte)
    #[serde(default)]
    pub update_rate: u32,
}

impl Pooled for PlayerECGGraph_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pl.player_ecggraph_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pl.player_ecggraph_config }
}

impl<'a> Extract<'a> for PlayerECGGraph_Config {
    const TYPE_NAME: &'static str = "PlayerECGGraph_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            idle_amp_range_max: inst.get_f32("idleAmpRangeMax").unwrap_or_default(),
            spike_frame_duration: inst.get_u32("spikeFrameDuration").unwrap_or_default(),
            spike_frame_duration_rand_factor: inst.get_f32("spikeFrameDurationRandFactor").unwrap_or_default(),
            wave_freq: inst.get_u32("waveFreq").unwrap_or_default(),
            pulse_amp_min: inst.get_f32("pulseAmpMin").unwrap_or_default(),
            pulse_amp_max: inst.get_f32("pulseAmpMax").unwrap_or_default(),
            update_rate: inst.get_u32("updateRate").unwrap_or_default(),
        }
    }
}

