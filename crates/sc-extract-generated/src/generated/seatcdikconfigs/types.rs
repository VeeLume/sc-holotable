// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `seatcdikconfigs`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SeatUserActorCDIKConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatUserActorCDIKConfig {
    /// `cdikTargetName` (String)
    #[serde(default)]
    pub cdik_target_name: String,
    /// `IKLimbHandle` (String)
    #[serde(default)]
    pub iklimb_handle: String,
    /// `parentJointName` (String)
    #[serde(default)]
    pub parent_joint_name: String,
    /// `cdikTargetOffset` (Class)
    #[serde(default)]
    pub cdik_target_offset: Option<Handle<QuatT>>,
    /// `userCDIKReferenceJoint` (String)
    #[serde(default)]
    pub user_cdikreference_joint: String,
}

impl Pooled for SeatUserActorCDIKConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.seatcdikconfigs.seat_user_actor_cdikconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.seatcdikconfigs.seat_user_actor_cdikconfig }
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
    /// `userSkeleton` (Reference)
    #[serde(default)]
    pub user_skeleton: Option<CigGuid>,
    /// `defaultCDIKTargets` (Class (array))
    #[serde(default)]
    pub default_cdiktargets: Vec<Handle<SeatUserActorCDIKConfig>>,
}

impl Pooled for SeatUserActorCDIKMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.seatcdikconfigs.seat_user_actor_cdikmapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.seatcdikconfigs.seat_user_actor_cdikmapping }
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
    /// `filters` (Class (array))
    #[serde(default)]
    pub filters: Vec<Handle<SeatUserActorCDIKMapping>>,
}

impl Pooled for SeatUserActorCDIKRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.seatcdikconfigs.seat_user_actor_cdikrecord }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.seatcdikconfigs.seat_user_actor_cdikrecord }
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

