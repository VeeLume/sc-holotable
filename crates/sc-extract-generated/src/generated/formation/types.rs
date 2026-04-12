// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `formation`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Formation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formation {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `offsets` (Class (array))
    #[serde(default)]
    pub offsets: Vec<Handle<FormationOffset>>,
    /// `playerFormationParams` (Class)
    #[serde(default)]
    pub player_formation_params: Option<Handle<PlayerFormationParams>>,
    /// `formationTag` (Reference)
    #[serde(default)]
    pub formation_tag: Option<CigGuid>,
}

impl Pooled for Formation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.formation.formation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.formation.formation }
}

impl<'a> Extract<'a> for Formation {
    const TYPE_NAME: &'static str = "Formation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            offsets: inst.get_array("offsets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FormationOffset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FormationOffset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            player_formation_params: match inst.get("playerFormationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerFormationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerFormationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            formation_tag: inst.get("formationTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `FormationOffset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationOffset {
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `offsetTag` (Reference)
    #[serde(default)]
    pub offset_tag: Option<CigGuid>,
    /// `offsetVariation` (Class)
    #[serde(default)]
    pub offset_variation: Option<Handle<Vec3>>,
}

impl Pooled for FormationOffset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.formation.formation_offset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.formation.formation_offset }
}

impl<'a> Extract<'a> for FormationOffset {
    const TYPE_NAME: &'static str = "FormationOffset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_tag: inst.get("offsetTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            offset_variation: match inst.get("offsetVariation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerFormationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerFormationParams {
    /// `targetRadius` (Single)
    #[serde(default)]
    pub target_radius: f32,
    /// `breakRadius` (Single)
    #[serde(default)]
    pub break_radius: f32,
    /// `abandonFormationAtBreakRadius` (Boolean)
    #[serde(default)]
    pub abandon_formation_at_break_radius: bool,
    /// `targetVelocityTolerance` (Single)
    #[serde(default)]
    pub target_velocity_tolerance: f32,
}

impl Pooled for PlayerFormationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.formation.player_formation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.formation.player_formation_params }
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

