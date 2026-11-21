// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `hcdgrouppreset`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `LocationFactionContextData`
/// Inherits from: `HierarchicalContextualData`
///
/// New in 5.0.0-ptu.13228337; absent from 4.10.3-live.13165946.
pub struct LocationFactionContextData {
    /// `factionIdentity` (Reference)
    pub faction_identity: Option<CigGuid>,
}

impl Pooled for LocationFactionContextData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.hcdgrouppreset.location_faction_context_data
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.hcdgrouppreset.location_faction_context_data
    }
}

impl<'a> Extract<'a> for LocationFactionContextData {
    const TYPE_NAME: &'static str = "LocationFactionContextData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            faction_identity: inst
                .get("factionIdentity")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `LocationLootGenerationContextData`
/// Inherits from: `HierarchicalContextualData`
///
/// New in 5.0.0-ptu.13228337; absent from 4.10.3-live.13165946.
pub struct LocationLootGenerationContextData {
    /// `themes` (Reference (array))
    pub themes: Vec<CigGuid>,
    /// `parentThemesToIgnore` (StrongPointer)
    pub parent_themes_to_ignore: Option<LootThemeSelector_BasePtr>,
}

impl Pooled for LocationLootGenerationContextData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.hcdgrouppreset.location_loot_generation_context_data
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.hcdgrouppreset.location_loot_generation_context_data
    }
}

impl<'a> Extract<'a> for LocationLootGenerationContextData {
    const TYPE_NAME: &'static str = "LocationLootGenerationContextData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            themes: inst
                .get_array("themes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            parent_themes_to_ignore: match inst.get("parentThemesToIgnore") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LootThemeSelector_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `LifetimeOverrideHierarchicalData`
/// Inherits from: `HierarchicalContextualData`
///
/// New in 5.0.0-ptu.13228337; absent from 4.10.3-live.13165946.
pub struct LifetimeOverrideHierarchicalData {
    /// `debugInfo` (String)
    pub debug_info: String,
    /// `densityLifetimeOverrideRef` (Reference)
    pub density_lifetime_override_ref: Option<CigGuid>,
}

impl Pooled for LifetimeOverrideHierarchicalData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.hcdgrouppreset.lifetime_override_hierarchical_data
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.hcdgrouppreset.lifetime_override_hierarchical_data
    }
}

impl<'a> Extract<'a> for LifetimeOverrideHierarchicalData {
    const TYPE_NAME: &'static str = "LifetimeOverrideHierarchicalData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_info: inst
                .get_str("debugInfo")
                .map(String::from)
                .unwrap_or_default(),
            density_lifetime_override_ref: inst
                .get("densityLifetimeOverrideRef")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `GeometryBaseHierarchicalData`
/// Inherits from: `HierarchicalContextualData`
///
/// New in 5.0.0-ptu.13228337; absent from 4.10.3-live.13165946.
pub struct GeometryBaseHierarchicalData {
    /// `ExternalWear` (Single)
    pub external_wear: f32,
    /// `ExternalDirt` (Single)
    pub external_dirt: f32,
    /// `InternalWear` (Single)
    pub internal_wear: f32,
    /// `InternalDirt` (Single)
    pub internal_dirt: f32,
    /// `TintPaletteOverride` (Reference)
    pub tint_palette_override: Option<CigGuid>,
}

impl Pooled for GeometryBaseHierarchicalData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.hcdgrouppreset.geometry_base_hierarchical_data
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.hcdgrouppreset.geometry_base_hierarchical_data
    }
}

impl<'a> Extract<'a> for GeometryBaseHierarchicalData {
    const TYPE_NAME: &'static str = "GeometryBaseHierarchicalData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            external_wear: inst.get_f32("ExternalWear").unwrap_or_default(),
            external_dirt: inst.get_f32("ExternalDirt").unwrap_or_default(),
            internal_wear: inst.get_f32("InternalWear").unwrap_or_default(),
            internal_dirt: inst.get_f32("InternalDirt").unwrap_or_default(),
            tint_palette_override: inst
                .get("TintPaletteOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `HierarchicalContextualDataPresetRecord`
///
/// New in 5.0.0-ptu.13228337; absent from 4.10.3-live.13165946.
pub struct HierarchicalContextualDataPresetRecord {
    /// `HCDs` (StrongPointer (array))
    pub hcds: Vec<HierarchicalContextualDataPtr>,
}

impl Pooled for HierarchicalContextualDataPresetRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .hcdgrouppreset
            .hierarchical_contextual_data_preset_record
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .hcdgrouppreset
            .hierarchical_contextual_data_preset_record
    }
}

impl<'a> Extract<'a> for HierarchicalContextualDataPresetRecord {
    const TYPE_NAME: &'static str = "HierarchicalContextualDataPresetRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hcds: inst
                .get_array("HCDs")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(HierarchicalContextualDataPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
