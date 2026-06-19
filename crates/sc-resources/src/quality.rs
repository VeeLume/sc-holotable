//! Resource quality — the distribution / quantization / location-override model.
//!
//! Quality is an **intrinsic property of a `ResourceType`** (the data attaches it
//! via `ResourceType.properties[ResourceTypeCraftingData]`). The "Crafting" in
//! CIG's record names is historical — crafting was the first consumer, but
//! quality also drives (future) sell price and anything else keyed off how good a
//! gathered resource is. So the model lives here, in the resource domain, and
//! every consumer (crafting, gathering, economy) reads it from the catalog they
//! already depend on rather than from each other.
//!
//! This module is the standalone-record **catalog**
//! (`CraftingQuality{Distribution,LocationOverride,Quantization}Record`); the
//! per-resource link (`ResourceType` → these records) is the bridge in `lib.rs`.

use std::collections::HashMap;

use sc_extract::generated::{
    CraftingQualityDistribution_Base_NonRefPtr, CraftingQualityDistribution_BasePtr,
    CraftingQualityLocationOverride_Base_NonRefPtr, CraftingQualityLocationOverride_BasePtr,
    CraftingQualityQuantization_Base_NonRefPtr, CraftingQualityQuantization_BasePtr, DataPools,
    ResourceType, ResourceTypePropertiesPtr,
};
use sc_extract::{Guid, RecordStore};
use serde::{Deserialize, Serialize};

/// Standalone quality records under `libs/foundry/records/crafting/`:
/// the catalog of `CraftingQualityDistributionRecord` /
/// `CraftingQualityLocationOverrideRecord` /
/// `CraftingQualityQuantizationRecord`. Cross-referenced from
/// `ResourceTypeCraftingData` (per-resource links) via the `_RecordRef`
/// polymorphic variants — resolve those via [`Quality::distribution`] etc.
///
/// SC 4.8 counts: **10 distributions** (100% Normal), **12 location
/// overrides** (134 total entries across the 12), **38 quantizations**
/// (304 total bands across the 38).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Quality {
    distributions: HashMap<Guid, QualityDistribution>,
    location_overrides: HashMap<Guid, QualityLocationOverride>,
    quantizations: HashMap<Guid, QualityQuantization>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityDistribution {
    pub guid: Guid,
    /// `None` when the standalone record's `quality_distribution`
    /// strong-ptr is empty (shouldn't happen but kept defensive).
    pub shape: Option<QualityDistributionShape>,
}

/// The concrete shape of a quality distribution. SC 4.8: 100% `Normal`.
/// `Uniform { min, max }` lives in the `dormant` feature (0 records);
/// it surfaces as `Other` until populated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QualityDistributionShape {
    Normal {
        min: i32,
        max: i32,
        mean: f32,
        stddev: f32,
    },
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// A reference to a distribution. The DCB has two shapes: an inline shape
/// (Normal/etc.) or a record-ref pointing at a standalone
/// [`QualityDistribution`] for sharing. Resolve a `Record` via
/// [`Quality::distribution`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DistributionRef {
    Inline(QualityDistributionShape),
    /// → `CraftingQualityDistributionRecord` GUID.
    Record(Guid),
    Other {
        type_name: String,
        struct_index: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityLocationOverride {
    pub guid: Guid,
    /// Per-location distribution overrides. SC 4.8: 12 records carry
    /// ~11 entries each (134 total).
    pub entries: Vec<LocationOverrideEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationOverrideEntry {
    /// → location record (a `StarMapObject`: a system / planet / cluster).
    pub location: Option<Guid>,
    pub distribution: Option<DistributionRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityQuantization {
    pub guid: Guid,
    /// Maps continuous quality ranges to discrete output values. SC 4.8:
    /// 38 records carry ~8 bands each (304 total).
    pub bands: Vec<QuantizationBand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QuantizationBand {
    pub start: i32,
    pub end: i32,
    pub mapped_value: i32,
}

impl Quality {
    /// Build the catalog from a parsed [`RecordStore`] (needs the `crafting`
    /// feature, which gates the standalone quality records).
    pub fn build(store: &RecordStore) -> Self {
        let records = &store.records;
        let pools = &store.pools;
        let mut q = Self::default();
        for (&guid, &handle) in &records.multi_feature.crafting_quality_distribution_record {
            let Some(rec) = handle.get(pools) else {
                continue;
            };
            q.distributions.insert(
                guid,
                QualityDistribution {
                    guid,
                    shape: rec
                        .quality_distribution
                        .as_ref()
                        .map(|d| build_distribution_shape_from_nonref(d, pools)),
                },
            );
        }
        for (&guid, &handle) in &records
            .multi_feature
            .crafting_quality_location_override_record
        {
            let Some(rec) = handle.get(pools) else {
                continue;
            };
            let entries = match rec.location_override.as_ref() {
                Some(
                    CraftingQualityLocationOverride_Base_NonRefPtr::CraftingQualityLocationOverride(
                        h,
                    ),
                ) => h
                    .get(pools)
                    .map(|co| {
                        co.location_override_list
                            .iter()
                            .filter_map(|eh| eh.get(pools))
                            .map(|e| LocationOverrideEntry {
                                location: e.location,
                                distribution: e
                                    .quality_distribution
                                    .as_ref()
                                    .map(|d| build_distribution_ref(d, pools)),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                _ => Vec::new(),
            };
            q.location_overrides
                .insert(guid, QualityLocationOverride { guid, entries });
        }
        for (&guid, &handle) in &records.multi_feature.crafting_quality_quantization_record {
            let Some(rec) = handle.get(pools) else {
                continue;
            };
            let bands = match rec.quality_quantization.as_ref() {
                Some(CraftingQualityQuantization_Base_NonRefPtr::CraftingQualityQuantization(
                    h,
                )) => h
                    .get(pools)
                    .map(|qq| {
                        qq.bands
                            .iter()
                            .filter_map(|bh| bh.get(pools))
                            .map(|b| QuantizationBand {
                                start: b.start,
                                end: b.end,
                                mapped_value: b.mapped_value,
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                _ => Vec::new(),
            };
            q.quantizations
                .insert(guid, QualityQuantization { guid, bands });
        }
        q
    }

    pub fn distribution(&self, guid: &Guid) -> Option<&QualityDistribution> {
        self.distributions.get(guid)
    }

    pub fn location_override(&self, guid: &Guid) -> Option<&QualityLocationOverride> {
        self.location_overrides.get(guid)
    }

    pub fn quantization(&self, guid: &Guid) -> Option<&QualityQuantization> {
        self.quantizations.get(guid)
    }

    pub fn distributions(&self) -> impl Iterator<Item = &QualityDistribution> + '_ {
        self.distributions.values()
    }

    pub fn location_overrides(&self) -> impl Iterator<Item = &QualityLocationOverride> + '_ {
        self.location_overrides.values()
    }

    pub fn quantizations(&self) -> impl Iterator<Item = &QualityQuantization> + '_ {
        self.quantizations.values()
    }
}

fn build_distribution_shape_from_nonref(
    d: &CraftingQualityDistribution_Base_NonRefPtr,
    pools: &DataPools,
) -> QualityDistributionShape {
    use CraftingQualityDistribution_Base_NonRefPtr as P;
    match d {
        P::CraftingQualityDistributionNormal(h) => match h.get(pools) {
            Some(n) => QualityDistributionShape::Normal {
                min: n.min,
                max: n.max,
                mean: n.mean,
                stddev: n.stddev,
            },
            None => QualityDistributionShape::Other {
                type_name: "Normal(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityDistribution_Base_NonRef(_) => QualityDistributionShape::Other {
            type_name: "CraftingQualityDistribution_Base_NonRef".into(),
            struct_index: 0,
        },
        P::Unknown { struct_index, .. } => QualityDistributionShape::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_distribution_ref(
    d: &CraftingQualityDistribution_BasePtr,
    pools: &DataPools,
) -> DistributionRef {
    use CraftingQualityDistribution_BasePtr as P;
    match d {
        P::CraftingQualityDistributionNormal(h) => match h.get(pools) {
            Some(n) => DistributionRef::Inline(QualityDistributionShape::Normal {
                min: n.min,
                max: n.max,
                mean: n.mean,
                stddev: n.stddev,
            }),
            None => DistributionRef::Other {
                type_name: "Normal(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityDistribution_RecordRef(h) => match h.get(pools) {
            Some(r) => match r.quality_distribution_record {
                Some(g) => DistributionRef::Record(g),
                None => DistributionRef::Other {
                    type_name: "RecordRef(none)".into(),
                    struct_index: 0,
                },
            },
            None => DistributionRef::Other {
                type_name: "RecordRef(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityDistribution_Base(_) | P::CraftingQualityDistribution_Base_NonRef(_) => {
            DistributionRef::Other {
                type_name: "CraftingQualityDistribution_Base(_NonRef)".into(),
                struct_index: 0,
            }
        }
        P::Unknown { struct_index, .. } => DistributionRef::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

// ── Per-resource bridge: ResourceType.properties[ResourceTypeCraftingData] ──

/// A resource's quality wiring — the inline-or-record refs read off its
/// `ResourceTypeCraftingData` property. Resolve `Record(guid)` refs against the
/// [`Quality`] catalog; `Inline` carries the shape/bands/entries directly.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceQuality {
    pub distribution: Option<DistributionRef>,
    pub quantization: Option<QuantizationRef>,
    pub location_override: Option<LocationOverrideRef>,
}

/// A reference to a quantization — inline bands or a shared record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantizationRef {
    Inline(Vec<QuantizationBand>),
    /// → `CraftingQualityQuantizationRecord` GUID ([`Quality::quantization`]).
    Record(Guid),
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// A reference to a location override — inline entries or a shared record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocationOverrideRef {
    Inline(Vec<LocationOverrideEntry>),
    /// → `CraftingQualityLocationOverrideRecord` GUID ([`Quality::location_override`]).
    Record(Guid),
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// Resolve `ResourceType.properties[ResourceTypeCraftingData]` into the resource's
/// quality refs. `None` when the resource carries no crafting-data property or it
/// has no quality wiring (most resources).
pub(crate) fn resource_quality_for(
    rt: &ResourceType,
    pools: &DataPools,
) -> Option<ResourceQuality> {
    let data = rt.properties.iter().find_map(|p| match p {
        ResourceTypePropertiesPtr::ResourceTypeCraftingData(h) => h.get(pools),
        _ => None,
    })?;
    let q = ResourceQuality {
        distribution: data
            .quality_distribution
            .as_ref()
            .map(|d| build_distribution_ref(d, pools)),
        quantization: data
            .quality_quantization
            .as_ref()
            .map(|d| build_quantization_ref(d, pools)),
        location_override: data
            .quality_location_override
            .as_ref()
            .map(|d| build_location_override_ref(d, pools)),
    };
    (q != ResourceQuality::default()).then_some(q)
}

fn build_quantization_ref(
    d: &CraftingQualityQuantization_BasePtr,
    pools: &DataPools,
) -> QuantizationRef {
    use CraftingQualityQuantization_BasePtr as P;
    match d {
        P::CraftingQualityQuantization(h) => match h.get(pools) {
            Some(qq) => QuantizationRef::Inline(
                qq.bands
                    .iter()
                    .filter_map(|bh| bh.get(pools))
                    .map(|b| QuantizationBand {
                        start: b.start,
                        end: b.end,
                        mapped_value: b.mapped_value,
                    })
                    .collect(),
            ),
            None => QuantizationRef::Other {
                type_name: "Quantization(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityQuantization_RecordRef(h) => match h.get(pools) {
            Some(r) => match r.quality_quantization_record {
                Some(g) => QuantizationRef::Record(g),
                None => QuantizationRef::Other {
                    type_name: "RecordRef(none)".into(),
                    struct_index: 0,
                },
            },
            None => QuantizationRef::Other {
                type_name: "RecordRef(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityQuantization_Base(_) | P::CraftingQualityQuantization_Base_NonRef(_) => {
            QuantizationRef::Other {
                type_name: "CraftingQualityQuantization_Base(_NonRef)".into(),
                struct_index: 0,
            }
        }
        P::Unknown { struct_index, .. } => QuantizationRef::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}

fn build_location_override_ref(
    d: &CraftingQualityLocationOverride_BasePtr,
    pools: &DataPools,
) -> LocationOverrideRef {
    use CraftingQualityLocationOverride_BasePtr as P;
    match d {
        P::CraftingQualityLocationOverride(h) => match h.get(pools) {
            Some(co) => LocationOverrideRef::Inline(
                co.location_override_list
                    .iter()
                    .filter_map(|eh| eh.get(pools))
                    .map(|e| LocationOverrideEntry {
                        location: e.location,
                        distribution: e
                            .quality_distribution
                            .as_ref()
                            .map(|d| build_distribution_ref(d, pools)),
                    })
                    .collect(),
            ),
            None => LocationOverrideRef::Other {
                type_name: "LocationOverride(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityLocationOverride_RecordRef(h) => match h.get(pools) {
            Some(r) => match r.location_override_record {
                Some(g) => LocationOverrideRef::Record(g),
                None => LocationOverrideRef::Other {
                    type_name: "RecordRef(none)".into(),
                    struct_index: 0,
                },
            },
            None => LocationOverrideRef::Other {
                type_name: "RecordRef(empty)".into(),
                struct_index: 0,
            },
        },
        P::CraftingQualityLocationOverride_Base(_)
        | P::CraftingQualityLocationOverride_Base_NonRef(_) => LocationOverrideRef::Other {
            type_name: "CraftingQualityLocationOverride_Base(_NonRef)".into(),
            struct_index: 0,
        },
        P::Unknown { struct_index, .. } => LocationOverrideRef::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    }
}
