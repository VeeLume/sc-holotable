//! Resource catalog over the DCB's `ResourceType` records.
//!
//! Every resource entity in the game — raw ores (`Aluminum`, `Quantainium`,
//! …), refined materials, drugs, commodities, ship-ammo SKUs, …  — is a
//! `ResourceType` record in
//! `libs/foundry/records/resourcetypedatabase/resourcetypedatabase.xml`.
//! SC 4.8 holds ~206 records (ores, refined metals, drugs, commodities,
//! ship ammunition sizes 1–9, infrastructure resources, …); ~30 of those
//! form the raw→refined graph via [`Resource::refined_version`].
//!
//! # Who references this
//!
//! - `sc-crafting`: every `CraftingCost_Resource` and `CraftingResult_Resource`
//!   carries a resource GUID and a [`CargoQuantity`]. The crafting recipe
//!   ingredient model bottoms out here.
//! - `sc-crafting` quality subsystem: per-resource [`Resource::crafting_data`]
//!   is the inline wiring; sc-crafting reads `ResourceType.properties` for
//!   the standalone `CraftingQualityDistribution`/`LocationOverride`/
//!   `Quantization` references — that machinery lives in sc-crafting.
//! - `sc-crafting` global params: `dismantleBlacklistResources` references
//!   resource GUIDs.
//!
//! # Refining graph
//!
//! [`Resource::refined_version`] is currently *the* refining mechanism — a
//! raw resource points at its refined counterpart. [`Resources::refined_version_of`]
//! walks the graph forwards. The schema also defines a per-blueprint
//! `CraftingProcess_Refining` but it is dormant + 0 records in SC 4.8.
//!
//! # Coverage in SC 4.8 (live-validated by `examples/resource_dump.rs`)
//!
//! - **206 records** total.
//! - **205 resolve names** via the locale map.
//! - **30 refining edges** (raw ore → refined metal pairs).
//! - **206 / 206** carry a [`Density`] (always the concrete
//!   `ResourceTypeDensity` variant).
//! - **1 / 206** carries a [`Volatility`] property.

use std::collections::HashMap;

use sc_extract::generated::{RecordLookup, ResourceType};
use sc_extract::{DataPools, Guid, LocaleKey, RecordStore};
use serde::{Deserialize, Serialize};

// ── Cargo quantity ──────────────────────────────────────────────────────
//
// `SBaseCargoUnit` is a 4-variant unit-of-measure hierarchy:
//   - `SStandardCargoUnit { standard_cargo_units: f32 }` (1 SCU)
//   - `SCentiCargoUnit    { centi_scu: i32 }`            (1/100 SCU)
//   - `SMicroCargoUnit    { micro_scu: i32 }`            (1/1_000_000 SCU)
//   - `SBaseCargoUnit {}`                                (empty base)
//
// Consumers convert to a normalized SCU value via [`CargoQuantity::to_scu`].

/// A cargo quantity expressed in one of the DCB's unit types. The raw
/// variants are preserved so a consumer can show the original unit; use
/// [`CargoQuantity::to_scu`] to normalize.
///
/// The projection from the raw `SBaseCargoUnitPtr` is in sc-crafting
/// (which enables the `crafting` feature that gates the Centi/Micro
/// pool types — under sc-resources's `resourcetypedatabase` alone, only
/// `SStandardCargoUnit` would be reachable).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CargoQuantity {
    Standard(f32),
    Centi(i32),
    Micro(i32),
    /// Polymorphic-fallback for a variant compiled out (dormant) or a
    /// future type the generator doesn't recognise yet.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

impl CargoQuantity {
    /// Normalize to SCU. `Centi` and `Micro` scale by 1/100 and 1/1_000_000.
    /// `Other` returns `None` — the unit shape isn't known.
    pub fn to_scu(&self) -> Option<f32> {
        match self {
            Self::Standard(n) => Some(*n),
            Self::Centi(n) => Some(*n as f32 / 100.0),
            Self::Micro(n) => Some(*n as f32 / 1_000_000.0),
            Self::Other { .. } => None,
        }
    }
}

// ── Density ─────────────────────────────────────────────────────────────
//
// `ResourceType.density_type` is `ResourceTypeDensityType_*`; the only
// populated concrete is `ResourceTypeDensity { density_unit:
// BaseDensityUnit_* }`. `BaseDensityUnit` has 3 variants:
//   - `BaseDensityUnit {}` (empty marker, base)
//   - `GramsPerCubicCentimeter { grams_per_cubic_centimeter: f32 }` (multi_feature)
//   - `KilogramsPerCubicMeter { kilograms_per_cubic_meter: f32 }` (dormant)
//
// Conversion: 1 g/cm³ = 1000 kg/m³.

/// A density value with its original unit preserved.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DensityUnit {
    GramsPerCm3(f32),
    /// `KilogramsPerCubicMeter` is in the dormant feature; reachable
    /// only as `Other` until a regen-after-population promotes it.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

impl DensityUnit {
    /// Normalize to kg/m³. 1 g/cm³ = 1000 kg/m³. `Other` returns `None`.
    pub fn to_kg_per_m3(&self) -> Option<f32> {
        match self {
            Self::GramsPerCm3(v) => Some(v * 1000.0),
            Self::Other { .. } => None,
        }
    }
}

/// A resource's density, with the underlying unit preserved.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Density {
    pub unit: Option<DensityUnit>,
}

// ── Volatility ──────────────────────────────────────────────────────────

/// Tracks how quickly the resource decays. Carried by 1 / 206 resources
/// in SC 4.8 (CIG hasn't populated this widely yet).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volatility {
    /// `name` field on the typed record — the property bucket label
    /// (`"Default"` in the one sample we see).
    pub name: String,
    /// Volatility coefficient. SC 4.8 sample: 1.0.
    pub volatility: f32,
    /// Health decay per second. SC 4.8 sample: 0.0675.
    pub health_decay_per_second: f32,
}

// ── Resource ────────────────────────────────────────────────────────────

/// A single resource entry, projected from the typed `ResourceType` pool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// The resource's GUID. Stable across patches.
    pub guid: Guid,
    /// Display-name locale key.
    pub name_key: LocaleKey,
    /// Description locale key.
    pub description_key: LocaleKey,
    /// Default raster thumbnail asset path.
    pub thumbnail_path: String,
    /// Default SVG thumbnail asset path.
    pub thumbnail_path_svg: String,
    /// Entity class used for the runtime-rendered thumbnail (RTT), if any.
    pub rtt_thumbnail_entity_class: Option<Guid>,
    /// **The refining edge:** the resource produced by refining `self`.
    /// `None` for already-refined / non-refinable resources.
    pub refined_version: Option<Guid>,
    /// True when this resource is subject to default-cargo-box validation.
    pub validate_default_cargo_box: bool,
    /// Density — every SC 4.8 ResourceType carries one.
    pub density: Option<Density>,
    /// Decay/volatility, if defined. SC 4.8: 1 / 206 carry this.
    pub volatility: Option<Volatility>,
    // `default_cargo_containers: Option<Handle<SResourceTypeDefaultCargoContainers>>`
    // on the raw record is an inline nested struct (Handle, not Guid). Not
    // surfaced — add when a consumer needs cargo-box defaults.
    //
    // `ResourceTypeCraftingData` (the inline per-resource quality wiring)
    // is sc-crafting's domain; it reads `ResourceType.properties` directly
    // and builds its own typed surface on top.
}

/// Flat lookup over every `ResourceType` record in the DCB. Build once,
/// share by reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resources {
    by_guid: HashMap<Guid, Resource>,
}

impl Resources {
    /// Construct an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the catalog from a parsed [`RecordStore`] (typed
    /// `ResourceType` pool). Cheap — ~200 records in SC 4.8.
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let mut catalog = Self::new();
        for (&guid, &handle) in &store.records.multi_feature.resource_type {
            let Some(rt) = handle.get(pools) else {
                continue;
            };
            catalog.insert(resource_for(guid, rt, pools));
        }
        catalog
    }

    /// Insert or replace a resource entry.
    pub fn insert(&mut self, resource: Resource) {
        self.by_guid.insert(resource.guid, resource);
    }

    /// Look up a resource by GUID.
    pub fn get(&self, guid: &Guid) -> Option<&Resource> {
        self.by_guid.get(guid)
    }

    /// One step along the refining graph: the resource that `guid`
    /// refines into, or `None` if there's no refined version (or the
    /// referenced resource isn't in the catalog).
    pub fn refined_version_of(&self, guid: &Guid) -> Option<&Resource> {
        let next = self.by_guid.get(guid)?.refined_version?;
        self.by_guid.get(&next)
    }

    /// Iterate over every resource. Order is unspecified.
    pub fn all(&self) -> impl Iterator<Item = &Resource> + '_ {
        self.by_guid.values()
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}

/// Project one typed `ResourceType` into a [`Resource`]. Shared by
/// [`Resources::build`] and [`ResourcesBuilder`].
fn resource_for(guid: Guid, rt: &ResourceType, pools: &DataPools) -> Resource {
    Resource {
        guid,
        name_key: rt.display_name.clone(),
        description_key: rt.description.clone(),
        thumbnail_path: rt.default_thumbnail_path.clone(),
        thumbnail_path_svg: rt.default_thumbnail_path_svg.clone(),
        rtt_thumbnail_entity_class: rt.rtt_thumbnail_entity_class,
        refined_version: rt.refined_version,
        validate_default_cargo_box: rt.validate_default_cargo_box,
        density: extract_density(rt, pools),
        volatility: extract_volatility(rt, pools),
    }
}

/// Resolve `ResourceType.density_type` → `ResourceTypeDensity` →
/// `BaseDensityUnit*` into a [`Density`].
fn extract_density(rt: &ResourceType, pools: &DataPools) -> Option<Density> {
    use sc_extract::generated::{BaseDensityUnitPtr, ResourceTypeDensityTypePtr};
    let dt = rt.density_type.as_ref()?;
    let ResourceTypeDensityTypePtr::ResourceTypeDensity(h) = dt else {
        // Base marker or Unknown — no usable unit data.
        return Some(Density { unit: None });
    };
    let density_rec = h.get(pools)?;
    let unit = density_rec.density_unit.as_ref().map(|u| match u {
        BaseDensityUnitPtr::GramsPerCubicCentimeter(uh) => uh
            .get(pools)
            .map(|gpcc| DensityUnit::GramsPerCm3(gpcc.grams_per_cubic_centimeter))
            .unwrap_or(DensityUnit::GramsPerCm3(0.0)),
        BaseDensityUnitPtr::BaseDensityUnit(_) => DensityUnit::Other {
            type_name: "BaseDensityUnit".into(),
            struct_index: 0,
        },
        BaseDensityUnitPtr::Unknown { struct_index, .. } => DensityUnit::Other {
            type_name: format!("struct#{struct_index}"),
            struct_index: *struct_index,
        },
    });
    Some(Density { unit })
}

/// Resolve `ResourceType.properties[ResourceTypeVolatility]` into a
/// [`Volatility`]. Returns the first volatility property found (only one
/// is populated in SC 4.8).
fn extract_volatility(rt: &ResourceType, pools: &DataPools) -> Option<Volatility> {
    use sc_extract::generated::ResourceTypePropertiesPtr as P;
    for p in &rt.properties {
        if let P::ResourceTypeVolatility(h) = p
            && let Some(v) = h.get(pools)
        {
            return Some(Volatility {
                name: v.name.clone(),
                volatility: v.volatility,
                health_decay_per_second: v.health_decay_per_second,
            });
        }
    }
    None
}

/// [`sc_extract::RecordVisitor`] that builds a [`Resources`] catalog
/// inside a bundled walk. Declares interest in `ResourceType` records.
/// Equivalent to [`Resources::build`] but fusible with other visitors.
#[derive(Default)]
pub struct ResourcesBuilder {
    inner: Resources,
}

impl sc_extract::RecordVisitor for ResourcesBuilder {
    type Output = Resources;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["ResourceType"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let Some(handle) = ResourceType::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(rt) = handle.get(&store.pools) else {
            return;
        };
        self.inner.insert(resource_for(item.guid, rt, &store.pools));
    }

    fn finish(self) -> Resources {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(byte: u8) -> Guid {
        Guid::from_bytes([byte; 16])
    }

    fn make(guid: Guid, name: &str, refined: Option<Guid>) -> Resource {
        Resource {
            guid,
            name_key: LocaleKey::from(format!("@resource_{name}")),
            description_key: LocaleKey::from(format!("@resource_{name}_desc")),
            thumbnail_path: format!("textures/{name}.dds"),
            thumbnail_path_svg: format!("textures/{name}.svg"),
            rtt_thumbnail_entity_class: None,
            refined_version: refined,
            validate_default_cargo_box: true,
            density: Some(Density {
                unit: Some(DensityUnit::GramsPerCm3(2.7)),
            }),
            volatility: None,
        }
    }

    #[test]
    fn new_catalog_is_empty() {
        let cat = Resources::new();
        assert!(cat.is_empty());
        assert_eq!(cat.len(), 0);
    }

    #[test]
    fn insert_and_lookup() {
        let mut cat = Resources::new();
        let raw = g(1);
        let refined = g(2);
        cat.insert(make(raw, "Aluminum_Raw", Some(refined)));
        cat.insert(make(refined, "Aluminum", None));
        assert_eq!(cat.len(), 2);
        assert_eq!(cat.get(&raw).map(|r| r.guid), Some(raw));
        assert_eq!(cat.refined_version_of(&raw).map(|r| r.guid), Some(refined));
        assert!(cat.refined_version_of(&refined).is_none());
    }

    #[test]
    fn refined_version_of_unknown_returns_none() {
        let cat = Resources::new();
        assert!(cat.refined_version_of(&g(99)).is_none());
    }

    #[test]
    fn refined_version_of_dangling_pointer_returns_none() {
        let mut cat = Resources::new();
        cat.insert(make(g(1), "Orphan", Some(g(255))));
        assert!(cat.refined_version_of(&g(1)).is_none());
    }

    #[test]
    fn cargo_quantity_to_scu_normalizes_units() {
        assert_eq!(CargoQuantity::Standard(2.5).to_scu(), Some(2.5));
        assert_eq!(CargoQuantity::Centi(150).to_scu(), Some(1.5));
        assert_eq!(CargoQuantity::Micro(1_500_000).to_scu(), Some(1.5));
        assert_eq!(
            CargoQuantity::Other {
                type_name: "X".into(),
                struct_index: 0,
            }
            .to_scu(),
            None
        );
    }

    #[test]
    fn density_unit_normalizes_to_kg_per_m3() {
        assert_eq!(DensityUnit::GramsPerCm3(2.7).to_kg_per_m3(), Some(2700.0));
        assert_eq!(
            DensityUnit::Other {
                type_name: "X".into(),
                struct_index: 0,
            }
            .to_kg_per_m3(),
            None
        );
    }

    #[test]
    fn serde_round_trip() {
        let mut cat = Resources::new();
        cat.insert(make(g(1), "Gold", None));
        let json = serde_json::to_string(&cat).unwrap();
        let decoded: Resources = serde_json::from_str(&json).unwrap();
        assert_eq!(
            decoded.get(&g(1)).map(|r| r.name_key.as_ref()),
            Some("@resource_Gold")
        );
        // density survives the round-trip
        assert_eq!(
            decoded
                .get(&g(1))
                .and_then(|r| r.density.as_ref())
                .and_then(|d| d.unit.clone()),
            Some(DensityUnit::GramsPerCm3(2.7))
        );
    }
}
