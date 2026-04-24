// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `aiprofile`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `AIMercyTimerSettings`
pub struct AIMercyTimerSettings {
    /// `activationThreshold` (Single)
    pub activation_threshold: f32,
    /// `coolDownTimeSeconds` (Single)
    pub cool_down_time_seconds: f32,
    /// `durationSeconds` (Single)
    pub duration_seconds: f32,
}

impl Pooled for AIMercyTimerSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.aiprofile.aimercy_timer_settings
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.aiprofile.aimercy_timer_settings
    }
}

impl<'a> Extract<'a> for AIMercyTimerSettings {
    const TYPE_NAME: &'static str = "AIMercyTimerSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activation_threshold: inst.get_f32("activationThreshold").unwrap_or_default(),
            cool_down_time_seconds: inst.get_f32("coolDownTimeSeconds").unwrap_or_default(),
            duration_seconds: inst.get_f32("durationSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `StatDefinitions`
pub struct StatDefinitions {
    /// `stats` (Class (array))
    pub stats: Vec<Handle<Stat>>,
}

impl Pooled for StatDefinitions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.aiprofile.stat_definitions
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.aiprofile.stat_definitions
    }
}

impl<'a> Extract<'a> for StatDefinitions {
    const TYPE_NAME: &'static str = "StatDefinitions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stats: inst
                .get_array("stats")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Stat>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Stat>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Stat`
pub struct Stat {
    /// `statTag` (Reference)
    pub stat_tag: Option<CigGuid>,
    /// `minimumValue` (Single)
    pub minimum_value: f32,
    /// `influences` (Class (array))
    pub influences: Vec<Handle<StatInfluence>>,
}

impl Pooled for Stat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.aiprofile.stat
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.aiprofile.stat
    }
}

impl<'a> Extract<'a> for Stat {
    const TYPE_NAME: &'static str = "Stat";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stat_tag: inst
                .get("statTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            minimum_value: inst.get_f32("minimumValue").unwrap_or_default(),
            influences: inst
                .get_array("influences")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<StatInfluence>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<StatInfluence>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatInfluence`
pub struct StatInfluence {
    /// `skillTag` (Reference)
    pub skill_tag: Option<CigGuid>,
    /// `percentage` (Int32)
    pub percentage: i32,
}

impl Pooled for StatInfluence {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.aiprofile.stat_influence
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.aiprofile.stat_influence
    }
}

impl<'a> Extract<'a> for StatInfluence {
    const TYPE_NAME: &'static str = "StatInfluence";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            skill_tag: inst
                .get("skillTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            percentage: inst.get_i32("percentage").unwrap_or_default(),
        }
    }
}
