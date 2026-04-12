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

/// Pool storage for the `ammoparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AmmoparamsPools {
    #[serde(default)]
    pub projectile_params: Vec<Option<ProjectileParams>>,
    #[serde(default)]
    pub projectile_spawned_entity_params: Vec<Option<ProjectileSpawnedEntityParams>>,
    #[serde(default)]
    pub projectile_detonation_params: Vec<Option<ProjectileDetonationParams>>,
    #[serde(default)]
    pub projectile_proximity_trigger_params: Vec<Option<ProjectileProximityTriggerParams>>,
    #[serde(default)]
    pub geometry_transform_params: Vec<Option<GeometryTransformParams>>,
    #[serde(default)]
    pub ammo_params: Vec<Option<AmmoParams>>,
    #[serde(default)]
    pub ssoftbody_geometry_params: Vec<Option<SSoftbodyGeometryParams>>,
    #[serde(default)]
    pub sgeometry_data_params: Vec<Option<SGeometryDataParams>>,
    #[serde(default)]
    pub sgeometry_node_params: Vec<Option<SGeometryNodeParams>>,
    #[serde(default)]
    pub smaterial_node_params: Vec<Option<SMaterialNodeParams>>,
    #[serde(default)]
    pub sgeometry_meshsetup_params: Vec<Option<SGeometryMeshsetupParams>>,
    #[serde(default)]
    pub sgeometry_resource_params: Vec<Option<SGeometryResourceParams>>,
    #[serde(default)]
    pub pooled_light_data: Vec<Option<PooledLightData>>,
    #[serde(default)]
    pub sentity_physics_controller_params: Vec<Option<SEntityPhysicsControllerParams>>,
    #[serde(default)]
    pub sgame_collision_class: Vec<Option<SGameCollisionClass>>,
    #[serde(default)]
    pub sbreakable_physics_params: Vec<Option<SBreakablePhysicsParams>>,
    #[serde(default)]
    pub sentity_base_physics_controller_params: Vec<Option<SEntityBasePhysicsControllerParams>>,
    #[serde(default)]
    pub sentity_soft_ex_physics_controller_params: Vec<Option<SEntitySoftExPhysicsControllerParams>>,
    #[serde(default)]
    pub sscsignature_system_audio_params: Vec<Option<SSCSignatureSystemAudioParams>>,
    #[serde(default)]
    pub sscsignature_system_params: Vec<Option<SSCSignatureSystemParams>>,
    #[serde(default)]
    pub tint_palette_swizzle: Vec<Option<TintPaletteSwizzle>>,
    #[serde(default)]
    pub tint_palette_ref: Vec<Option<TintPaletteRef>>,
}
