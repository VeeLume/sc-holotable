// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `densityclasses`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SDensityClassLifetimeOverrideEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDensityClassLifetimeOverrideEntry {
    /// `densityClass` (Reference)
    #[serde(default)]
    pub density_class: Option<CigGuid>,
    /// `lifetimeOverride` (StrongPointer)
    #[serde(default)]
    pub lifetime_override: Option<TimeValue_BasePtr>,
}

impl Pooled for SDensityClassLifetimeOverrideEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.densityclasses.sdensity_class_lifetime_override_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.densityclasses.sdensity_class_lifetime_override_entry }
}

impl<'a> Extract<'a> for SDensityClassLifetimeOverrideEntry {
    const TYPE_NAME: &'static str = "SDensityClassLifetimeOverrideEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            density_class: inst.get("densityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            lifetime_override: match inst.get("lifetimeOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(TimeValue_BasePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SDefaultDensityClassLifetimeOverrides`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDefaultDensityClassLifetimeOverrides {
    /// `lifetimeOverride` (StrongPointer)
    #[serde(default)]
    pub lifetime_override: Option<TimeValue_BasePtr>,
    /// `excludes` (Reference (array))
    #[serde(default)]
    pub excludes: Vec<CigGuid>,
}

impl Pooled for SDefaultDensityClassLifetimeOverrides {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.densityclasses.sdefault_density_class_lifetime_overrides }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.densityclasses.sdefault_density_class_lifetime_overrides }
}

impl<'a> Extract<'a> for SDefaultDensityClassLifetimeOverrides {
    const TYPE_NAME: &'static str = "SDefaultDensityClassLifetimeOverrides";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lifetime_override: match inst.get("lifetimeOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(TimeValue_BasePtr::from_ref(b, r)),
                _ => None,
            },
            excludes: inst.get_array("excludes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityDensityClassOverridesRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityDensityClassOverridesRecord {
    /// `overrides` (Class)
    #[serde(default)]
    pub overrides: Option<Handle<SEntityDensityClassOverridesManual>>,
}

impl Pooled for SEntityDensityClassOverridesRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.densityclasses.sentity_density_class_overrides_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.densityclasses.sentity_density_class_overrides_record }
}

impl<'a> Extract<'a> for SEntityDensityClassOverridesRecord {
    const TYPE_NAME: &'static str = "SEntityDensityClassOverridesRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            overrides: match inst.get("overrides") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityDensityClassOverridesManual>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityDensityClassOverridesManual`
/// Inherits from: `SEntityDensityClassOverridesBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityDensityClassOverridesManual {
    /// `defaults` (StrongPointer)
    #[serde(default)]
    pub defaults: Option<Handle<SDefaultDensityClassLifetimeOverrides>>,
    /// `densityClassLifetimeOverrides` (Class (array))
    #[serde(default)]
    pub density_class_lifetime_overrides: Vec<Handle<SDensityClassLifetimeOverrideEntry>>,
}

impl Pooled for SEntityDensityClassOverridesManual {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.densityclasses.sentity_density_class_overrides_manual }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.densityclasses.sentity_density_class_overrides_manual }
}

impl<'a> Extract<'a> for SEntityDensityClassOverridesManual {
    const TYPE_NAME: &'static str = "SEntityDensityClassOverridesManual";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            defaults: match inst.get("defaults") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDefaultDensityClassLifetimeOverrides>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            density_class_lifetime_overrides: inst.get_array("densityClassLifetimeOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SDensityClassLifetimeOverrideEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SDensityClassLifetimeOverrideEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

