// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `procedurallandingsetup`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ProceduralLandingFilter`
/// Inherits from: `ActorStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLandingFilter {
    /// `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// `filterByLandingStrength` (EnumChoice)
    #[serde(default)]
    pub filter_by_landing_strength: String,
    /// `animationRecord` (Reference)
    #[serde(default)]
    pub animation_record: Option<CigGuid>,
}

impl Pooled for ProceduralLandingFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallandingsetup.procedural_landing_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallandingsetup.procedural_landing_filter }
}

impl<'a> Extract<'a> for ProceduralLandingFilter {
    const TYPE_NAME: &'static str = "ProceduralLandingFilter";
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
            filter_by_landing_strength: inst.get_str("filterByLandingStrength").map(String::from).unwrap_or_default(),
            animation_record: inst.get("animationRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ProceduralLandingSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLandingSetup {
    /// `landingAnimations` (Class (array))
    #[serde(default)]
    pub landing_animations: Vec<Handle<ProceduralLandingFilter>>,
    /// `firstPersonLandingAnimations` (Class (array))
    #[serde(default)]
    pub first_person_landing_animations: Vec<Handle<ProceduralLandingFilter>>,
}

impl Pooled for ProceduralLandingSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallandingsetup.procedural_landing_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallandingsetup.procedural_landing_setup }
}

impl<'a> Extract<'a> for ProceduralLandingSetup {
    const TYPE_NAME: &'static str = "ProceduralLandingSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            landing_animations: inst.get_array("landingAnimations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLandingFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralLandingFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            first_person_landing_animations: inst.get_array("firstPersonLandingAnimations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLandingFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralLandingFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

