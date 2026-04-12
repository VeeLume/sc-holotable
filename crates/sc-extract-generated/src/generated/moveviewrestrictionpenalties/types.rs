// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `moveviewrestrictionpenalties`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MoveViewRestrictionPenalty`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveViewRestrictionPenalty {
    /// `restrictedMotionPenalty` (Single)
    #[serde(default)]
    pub restricted_motion_penalty: f32,
    /// `restrictedViewPenalty` (Single)
    #[serde(default)]
    pub restricted_view_penalty: f32,
}

impl Pooled for MoveViewRestrictionPenalty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.moveviewrestrictionpenalties.move_view_restriction_penalty }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.moveviewrestrictionpenalties.move_view_restriction_penalty }
}

impl<'a> Extract<'a> for MoveViewRestrictionPenalty {
    const TYPE_NAME: &'static str = "MoveViewRestrictionPenalty";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            restricted_motion_penalty: inst.get_f32("restrictedMotionPenalty").unwrap_or_default(),
            restricted_view_penalty: inst.get_f32("restrictedViewPenalty").unwrap_or_default(),
        }
    }
}

/// DCB type: `MoveViewRestrictionWeighting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveViewRestrictionWeighting {
    /// `HelmetWeighting` (Single)
    #[serde(default)]
    pub helmet_weighting: f32,
    /// `CoreWeighting` (Single)
    #[serde(default)]
    pub core_weighting: f32,
    /// `LegsWeighting` (Single)
    #[serde(default)]
    pub legs_weighting: f32,
    /// `ArmsWeighting` (Single)
    #[serde(default)]
    pub arms_weighting: f32,
}

impl Pooled for MoveViewRestrictionWeighting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.moveviewrestrictionpenalties.move_view_restriction_weighting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.moveviewrestrictionpenalties.move_view_restriction_weighting }
}

impl<'a> Extract<'a> for MoveViewRestrictionWeighting {
    const TYPE_NAME: &'static str = "MoveViewRestrictionWeighting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            helmet_weighting: inst.get_f32("HelmetWeighting").unwrap_or_default(),
            core_weighting: inst.get_f32("CoreWeighting").unwrap_or_default(),
            legs_weighting: inst.get_f32("LegsWeighting").unwrap_or_default(),
            arms_weighting: inst.get_f32("ArmsWeighting").unwrap_or_default(),
        }
    }
}

/// DCB type: `ArmorMoveViewRestrictions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorMoveViewRestrictions {
    /// `ViewRestriction` (Class)
    #[serde(default)]
    pub view_restriction: Option<Handle<MoveViewRestrictionWeighting>>,
    /// `MoveRestriction` (Class)
    #[serde(default)]
    pub move_restriction: Option<Handle<MoveViewRestrictionWeighting>>,
}

impl Pooled for ArmorMoveViewRestrictions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.moveviewrestrictionpenalties.armor_move_view_restrictions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.moveviewrestrictionpenalties.armor_move_view_restrictions }
}

impl<'a> Extract<'a> for ArmorMoveViewRestrictions {
    const TYPE_NAME: &'static str = "ArmorMoveViewRestrictions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            view_restriction: match inst.get("ViewRestriction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            move_restriction: match inst.get("MoveRestriction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MoveViewRestrictionWeighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

