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

/// DCB type: `WeightedLootArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedLootArchetype {
    /// DCB field: `archetype` (Reference)
    #[serde(default)]
    pub archetype: Option<CigGuid>,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// DCB field: `numberOfResultsConstraints` (Class)
    #[serde(default)]
    pub number_of_results_constraints: Option<Handle<NumResultsConstraints>>,
}

impl Pooled for WeightedLootArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weighted_loot_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weighted_loot_archetype }
}

impl<'a> Extract<'a> for WeightedLootArchetype {
    const TYPE_NAME: &'static str = "WeightedLootArchetype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            archetype: inst.get("archetype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weight: inst.get_f32("weight").unwrap_or_default(),
            number_of_results_constraints: match inst.get("numberOfResultsConstraints") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<NumResultsConstraints>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<NumResultsConstraints>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponAimableAnglesDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponAimableAnglesDef {
    /// DCB field: `maxNudgeAngle` (Single)
    #[serde(default)]
    pub max_nudge_angle: f32,
    /// DCB field: `innerThresholdAngle` (Single)
    #[serde(default)]
    pub inner_threshold_angle: f32,
    /// DCB field: `outerThresholdAngle` (Single)
    #[serde(default)]
    pub outer_threshold_angle: f32,
    /// DCB field: `closeDistanceMaxRange` (Single)
    #[serde(default)]
    pub close_distance_max_range: f32,
    /// DCB field: `closeDistanceMinRange` (Single)
    #[serde(default)]
    pub close_distance_min_range: f32,
    /// DCB field: `closeDistanceOuterAngle` (Single)
    #[serde(default)]
    pub close_distance_outer_angle: f32,
    /// DCB field: `closeDistanceInnerAngle` (Single)
    #[serde(default)]
    pub close_distance_inner_angle: f32,
    /// DCB field: `assistNudgeMapping` (Class)
    #[serde(default)]
    pub assist_nudge_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for WeaponAimableAnglesDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_aimable_angles_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_aimable_angles_def }
}

impl<'a> Extract<'a> for WeaponAimableAnglesDef {
    const TYPE_NAME: &'static str = "WeaponAimableAnglesDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_nudge_angle: inst.get_f32("maxNudgeAngle").unwrap_or_default(),
            inner_threshold_angle: inst.get_f32("innerThresholdAngle").unwrap_or_default(),
            outer_threshold_angle: inst.get_f32("outerThresholdAngle").unwrap_or_default(),
            close_distance_max_range: inst.get_f32("closeDistanceMaxRange").unwrap_or_default(),
            close_distance_min_range: inst.get_f32("closeDistanceMinRange").unwrap_or_default(),
            close_distance_outer_angle: inst.get_f32("closeDistanceOuterAngle").unwrap_or_default(),
            close_distance_inner_angle: inst.get_f32("closeDistanceInnerAngle").unwrap_or_default(),
            assist_nudge_mapping: match inst.get("assistNudgeMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponGimbalModeModifierDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponGimbalModeModifierDef {
    /// DCB field: `weaponGimbalModeModifiers` (StrongPointer)
    #[serde(default)]
    pub weapon_gimbal_mode_modifiers: Option<Handle<SWeaponModifierParams>>,
}

impl Pooled for WeaponGimbalModeModifierDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_gimbal_mode_modifier_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_gimbal_mode_modifier_def }
}

impl<'a> Extract<'a> for WeaponGimbalModeModifierDef {
    const TYPE_NAME: &'static str = "WeaponGimbalModeModifierDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_gimbal_mode_modifiers: match inst.get("weaponGimbalModeModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponModifierParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponModifierParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponMisfireDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponMisfireDef {
    /// DCB field: `minorMisfire` (Class)
    #[serde(default)]
    pub minor_misfire: Option<Handle<SWeaponMisfireEntry>>,
    /// DCB field: `minorMisfireDuration` (Single)
    #[serde(default)]
    pub minor_misfire_duration: f32,
    /// DCB field: `minorMisfireCooldown` (Single)
    #[serde(default)]
    pub minor_misfire_cooldown: f32,
    /// DCB field: `majorMisfire` (Class)
    #[serde(default)]
    pub major_misfire: Option<Handle<SWeaponMisfireEntry>>,
    /// DCB field: `majorMisfireCooldown` (Single)
    #[serde(default)]
    pub major_misfire_cooldown: f32,
    /// DCB field: `criticalMisfire` (Class)
    #[serde(default)]
    pub critical_misfire: Option<Handle<SWeaponMisfireEntry>>,
}

impl Pooled for WeaponMisfireDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_misfire_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_misfire_def }
}

impl<'a> Extract<'a> for WeaponMisfireDef {
    const TYPE_NAME: &'static str = "WeaponMisfireDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minor_misfire: match inst.get("minorMisfire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponMisfireEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponMisfireEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            minor_misfire_duration: inst.get_f32("minorMisfireDuration").unwrap_or_default(),
            minor_misfire_cooldown: inst.get_f32("minorMisfireCooldown").unwrap_or_default(),
            major_misfire: match inst.get("majorMisfire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponMisfireEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponMisfireEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            major_misfire_cooldown: inst.get_f32("majorMisfireCooldown").unwrap_or_default(),
            critical_misfire: match inst.get("criticalMisfire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponMisfireEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponMisfireEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponARModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponARModifier {
    /// DCB field: `arMuzzleFlashEffect` (Class)
    #[serde(default)]
    pub ar_muzzle_flash_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `arMFXImpact` (String)
    #[serde(default)]
    pub ar_mfximpact: String,
    /// DCB field: `arTriggerTag` (Reference)
    #[serde(default)]
    pub ar_trigger_tag: Option<CigGuid>,
    /// DCB field: `weaponModifier` (Class)
    #[serde(default)]
    pub weapon_modifier: Option<Handle<SWeaponModifierParams>>,
}

impl Pooled for WeaponARModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_armodifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_armodifier }
}

impl<'a> Extract<'a> for WeaponARModifier {
    const TYPE_NAME: &'static str = "WeaponARModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ar_muzzle_flash_effect: match inst.get("arMuzzleFlashEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ar_mfximpact: inst.get_str("arMFXImpact").map(String::from).unwrap_or_default(),
            ar_trigger_tag: inst.get("arTriggerTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weapon_modifier: match inst.get("weaponModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponModifierParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponModifierParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WebRTCCommsCallProjectorLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRTCCommsCallProjectorLightParams {
    /// DCB field: `temperature` (Single)
    #[serde(default)]
    pub temperature: f32,
    /// DCB field: `intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// DCB field: `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// DCB field: `bulbRadius` (Single)
    #[serde(default)]
    pub bulb_radius: f32,
    /// DCB field: `lightFrustumAngle` (Single)
    #[serde(default)]
    pub light_frustum_angle: f32,
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `castShadow` (Boolean)
    #[serde(default)]
    pub cast_shadow: bool,
}

impl Pooled for WebRTCCommsCallProjectorLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.web_rtccomms_call_projector_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.web_rtccomms_call_projector_light_params }
}

impl<'a> Extract<'a> for WebRTCCommsCallProjectorLightParams {
    const TYPE_NAME: &'static str = "WebRTCCommsCallProjectorLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            temperature: inst.get_f32("temperature").unwrap_or_default(),
            intensity: inst.get_f32("intensity").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            bulb_radius: inst.get_f32("bulbRadius").unwrap_or_default(),
            light_frustum_angle: inst.get_f32("lightFrustumAngle").unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cast_shadow: inst.get_bool("castShadow").unwrap_or_default(),
        }
    }
}

/// DCB type: `WeaponProceduralClipsSetUp`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralClipsSetUp {
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
    /// DCB field: `weaponProceduralClips` (Reference (array))
    #[serde(default)]
    pub weapon_procedural_clips: Vec<CigGuid>,
}

impl Pooled for WeaponProceduralClipsSetUp {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_procedural_clips_set_up }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_procedural_clips_set_up }
}

impl<'a> Extract<'a> for WeaponProceduralClipsSetUp {
    const TYPE_NAME: &'static str = "WeaponProceduralClipsSetUp";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
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
            weapon_procedural_clips: inst.get_array("weaponProceduralClips")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WeaponProceduralAnimation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralAnimation {
    /// DCB field: `weaponProceduralClipsSetUp` (Class (array))
    #[serde(default)]
    pub weapon_procedural_clips_set_up: Vec<Handle<WeaponProceduralClipsSetUp>>,
}

impl Pooled for WeaponProceduralAnimation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_procedural_animation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_procedural_animation }
}

impl<'a> Extract<'a> for WeaponProceduralAnimation {
    const TYPE_NAME: &'static str = "WeaponProceduralAnimation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_procedural_clips_set_up: inst.get_array("weaponProceduralClipsSetUp")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WeaponProceduralClipsSetUp>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WeaponProceduralClipsSetUp>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WeaponProceduralClip`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralClip {
    /// DCB field: `weaponProceduralClipBase` (StrongPointer)
    #[serde(default)]
    pub weapon_procedural_clip_base: Option<Handle<WeaponProceduralClipBase>>,
}

impl Pooled for WeaponProceduralClip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_procedural_clip }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_procedural_clip }
}

impl<'a> Extract<'a> for WeaponProceduralClip {
    const TYPE_NAME: &'static str = "WeaponProceduralClip";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_procedural_clip_base: match inst.get("weaponProceduralClipBase") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WeaponProceduralClipBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WeaponProceduralClipBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponProceduralClipBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralClipBase {
    /// DCB field: `blendTime` (Single)
    #[serde(default)]
    pub blend_time: f32,
}

impl Pooled for WeaponProceduralClipBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_procedural_clip_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_procedural_clip_base }
}

impl<'a> Extract<'a> for WeaponProceduralClipBase {
    const TYPE_NAME: &'static str = "WeaponProceduralClipBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blend_time: inst.get_f32("blendTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `WeaponProceduralRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralRecoilConfigDef {
    /// DCB field: `weaponProceduralHandsRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_hands_recoil: Option<Handle<SWeaponProceduralHandsRecoilConfigDef>>,
    /// DCB field: `weaponProceduralAimRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_aim_recoil: Option<Handle<SWeaponProceduralAimRecoilConfigDef>>,
    /// DCB field: `weaponProceduralBodyRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_body_recoil: Option<Handle<SWeaponProceduralBodyRecoilConfigDef>>,
    /// DCB field: `weaponProceduralHeadRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_head_recoil: Option<Handle<SWeaponProceduralHeadRecoilConfigDef>>,
}

impl Pooled for WeaponProceduralRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weapon_procedural_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weapon_procedural_recoil_config_def }
}

impl<'a> Extract<'a> for WeaponProceduralRecoilConfigDef {
    const TYPE_NAME: &'static str = "WeaponProceduralRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_procedural_hands_recoil: match inst.get("weaponProceduralHandsRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_procedural_aim_recoil: match inst.get("weaponProceduralAimRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_procedural_body_recoil: match inst.get("weaponProceduralBodyRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralBodyRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralBodyRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_procedural_head_recoil: match inst.get("weaponProceduralHeadRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeatherEffects_SpaceLoopEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_SpaceLoopEffect {
    /// DCB field: `effect` (Class)
    #[serde(default)]
    pub effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `updateStrength` (Boolean)
    #[serde(default)]
    pub update_strength: bool,
    /// DCB field: `updateCount` (Boolean)
    #[serde(default)]
    pub update_count: bool,
}

impl Pooled for WeatherEffects_SpaceLoopEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weather_effects_space_loop_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weather_effects_space_loop_effect }
}

impl<'a> Extract<'a> for WeatherEffects_SpaceLoopEffect {
    const TYPE_NAME: &'static str = "WeatherEffects_SpaceLoopEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect: match inst.get("effect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            update_strength: inst.get_bool("updateStrength").unwrap_or_default(),
            update_count: inst.get_bool("updateCount").unwrap_or_default(),
        }
    }
}

/// DCB type: `WeatherEffects_Asteroid`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_Asteroid {
}

impl Pooled for WeatherEffects_Asteroid {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weather_effects_asteroid }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weather_effects_asteroid }
}

impl<'a> Extract<'a> for WeatherEffects_Asteroid {
    const TYPE_NAME: &'static str = "WeatherEffects_Asteroid";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `WeatherEffects_Atmosphere`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_Atmosphere {
}

impl Pooled for WeatherEffects_Atmosphere {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.weather_effects_atmosphere }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.weather_effects_atmosphere }
}

impl<'a> Extract<'a> for WeatherEffects_Atmosphere {
    const TYPE_NAME: &'static str = "WeatherEffects_Atmosphere";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `WebCustomizationDebug`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebCustomizationDebug {
    /// DCB field: `debugLoadoutKits` (Class (array))
    #[serde(default)]
    pub debug_loadout_kits: Vec<Handle<DebugLoadoutKit>>,
}

impl Pooled for WebCustomizationDebug {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.web_customization_debug }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.web_customization_debug }
}

impl<'a> Extract<'a> for WebCustomizationDebug {
    const TYPE_NAME: &'static str = "WebCustomizationDebug";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_loadout_kits: inst.get_array("debugLoadoutKits")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DebugLoadoutKit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DebugLoadoutKit>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WebCustomizationItemTypeName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebCustomizationItemTypeName {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `itemTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_types: Vec<String>,
}

impl Pooled for WebCustomizationItemTypeName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.web_customization_item_type_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.web_customization_item_type_name }
}

impl<'a> Extract<'a> for WebCustomizationItemTypeName {
    const TYPE_NAME: &'static str = "WebCustomizationItemTypeName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WebCustomizationGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebCustomizationGlobalParams {
    /// DCB field: `defaultLoadoutKitName` (Locale)
    #[serde(default)]
    pub default_loadout_kit_name: String,
    /// DCB field: `typeNames` (Class (array))
    #[serde(default)]
    pub type_names: Vec<Handle<WebCustomizationItemTypeName>>,
}

impl Pooled for WebCustomizationGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_we.web_customization_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_we.web_customization_global_params }
}

impl<'a> Extract<'a> for WebCustomizationGlobalParams {
    const TYPE_NAME: &'static str = "WebCustomizationGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_loadout_kit_name: inst.get_str("defaultLoadoutKitName").map(String::from).unwrap_or_default(),
            type_names: inst.get_array("typeNames")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WebCustomizationItemTypeName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WebCustomizationItemTypeName>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

