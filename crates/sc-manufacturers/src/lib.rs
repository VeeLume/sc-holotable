//! Manufacturer registry over the DCB's `SCItemManufacturer` records.
//!
//! The DCB contains ~100 manufacturer records referenced from thousands of
//! item/ship/weapon records. [`ManufacturerRegistry`] provides GUID-keyed
//! and code-keyed lookup. Moved out of `sc-extract` (it's domain data, not
//! a generic DCB primitive); build it explicitly via [`ManufacturerRegistry::build`].
//!
//! # Walk
//!
//! Typed pool surface (`scitemmanufacturer` feature): reads the typed
//! `code` field and the `SCItemLocalization` keys directly.
//!
//! # Not yet modelled: manufacturer *kind*
//!
//! `SCItemManufacturer` has **no kind/type field** — yet CIG uses these
//! records in at least three distinct ways (observed in live data):
//!   1. **genuine manufacturers** (ships, weapons, components),
//!   2. **shops** (e.g. CenterMass),
//!   3. **UI / paints / logos** — odd cases; e.g. a paint reuses the *locale*
//!      of the ship manufacturer but is a separate manufacturer entry.
//!
//! The distinction is **emergent from how a manufacturer is referenced**, not
//! a field on the record, so classifying it needs referencing-side analysis.
//! Deferred until a consumer needs it. The typed `logo*` fields and the
//! Reference fields (`building_blocks_style`, …) are also available on the
//! record for the UI/paint use case when that lands.

use std::collections::HashMap;

use sc_extract::generated::{RecordLookup, SCItemManufacturer};
use sc_extract::{DataPools, Guid, RecordStore};
use serde::{Deserialize, Serialize};

/// A single manufacturer entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manufacturer {
    /// The manufacturer's GUID (stable across game patches).
    pub guid: Guid,
    /// Short code, e.g. `"GATS"` / `"AEGS"` (the record's `Code` field).
    pub code: String,
    /// Localization key for the full name, e.g. `"@manufacturer_NameGATS"`.
    /// Resolve via [`sc_extract::LocaleMap::resolve`].
    pub name_key: Option<String>,
    /// Localization key for the description.
    pub description_key: Option<String>,
}

/// Flat lookup over every `SCItemManufacturer` record in the DCB.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManufacturerRegistry {
    by_guid: HashMap<Guid, Manufacturer>,
    by_code: HashMap<String, Guid>,
}

impl ManufacturerRegistry {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the registry from a parsed [`RecordStore`] (typed
    /// `SCItemManufacturer` pool). Build once, share by reference.
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let mut registry = Self::new();
        for (&guid, &handle) in &store.records.multi_feature.scitem_manufacturer {
            let Some(m) = handle.get(pools) else {
                continue;
            };
            registry.insert(manufacturer_for(guid, m, pools));
        }
        registry
    }

    /// Insert or replace a manufacturer. Maintains both indices.
    pub fn insert(&mut self, manufacturer: Manufacturer) {
        if let Some(existing) = self.by_guid.get(&manufacturer.guid)
            && existing.code != manufacturer.code
        {
            self.by_code.remove(&existing.code);
        }
        self.by_code
            .insert(manufacturer.code.clone(), manufacturer.guid);
        self.by_guid.insert(manufacturer.guid, manufacturer);
    }

    /// Look up a manufacturer by GUID.
    pub fn get(&self, guid: &Guid) -> Option<&Manufacturer> {
        self.by_guid.get(guid)
    }

    /// Look up a manufacturer by its short code (case-sensitive).
    pub fn by_code(&self, code: &str) -> Option<&Manufacturer> {
        let guid = self.by_code.get(code)?;
        self.by_guid.get(guid)
    }

    /// Iterate over every manufacturer. Order is unspecified.
    pub fn all(&self) -> impl Iterator<Item = &Manufacturer> + '_ {
        self.by_guid.values()
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}

/// Project one typed `SCItemManufacturer` into a [`Manufacturer`]. Shared by
/// [`ManufacturerRegistry::build`] and [`ManufacturerRegistryBuilder`].
fn manufacturer_for(guid: Guid, m: &SCItemManufacturer, pools: &DataPools) -> Manufacturer {
    let (name_key, description_key) = match m.localization.and_then(|h| h.get(pools)) {
        Some(loc) => (
            non_empty(loc.name.as_str()),
            non_empty(loc.description.as_str()),
        ),
        None => (None, None),
    };
    Manufacturer {
        guid,
        code: m.code.clone(),
        name_key,
        description_key,
    }
}

/// `Some(owned)` only when the string isn't empty.
fn non_empty(s: &str) -> Option<String> {
    (!s.is_empty()).then(|| s.to_string())
}

/// [`sc_extract::RecordVisitor`] that builds a [`ManufacturerRegistry`] in a
/// bundled walk. Declares interest in `SCItemManufacturer` records. Equivalent
/// to [`ManufacturerRegistry::build`] but fusible with other visitors.
#[derive(Default)]
pub struct ManufacturerRegistryBuilder {
    inner: ManufacturerRegistry,
}

impl sc_extract::RecordVisitor for ManufacturerRegistryBuilder {
    type Output = ManufacturerRegistry;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["SCItemManufacturer"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let Some(handle) = SCItemManufacturer::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(m) = handle.get(&store.pools) else {
            return;
        };
        self.inner
            .insert(manufacturer_for(item.guid, m, &store.pools));
    }

    fn finish(self) -> ManufacturerRegistry {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(byte: u8) -> Guid {
        Guid::from_bytes([byte; 16])
    }

    fn make(guid: Guid, code: &str) -> Manufacturer {
        Manufacturer {
            guid,
            code: code.to_string(),
            name_key: Some(format!("@manufacturer_Name{code}")),
            description_key: Some(format!("@manufacturer_Desc{code}")),
        }
    }

    #[test]
    fn new_registry_is_empty() {
        let reg = ManufacturerRegistry::new();
        assert!(reg.is_empty());
        assert_eq!(reg.len(), 0);
    }

    #[test]
    fn insert_and_lookup() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "GATS"));
        reg.insert(make(g(2), "AEGS"));
        assert_eq!(reg.get(&g(1)).map(|m| m.code.as_str()), Some("GATS"));
        assert_eq!(reg.by_code("AEGS").map(|m| m.guid), Some(g(2)));
        assert!(reg.by_code("MISSING").is_none());
        assert_eq!(reg.all().count(), 2);
    }

    #[test]
    fn insert_replaces_and_updates_code_index() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "OldCode"));
        reg.insert(make(g(1), "NewCode"));
        assert_eq!(reg.len(), 1);
        assert!(reg.by_code("OldCode").is_none());
        assert!(reg.by_code("NewCode").is_some());
    }

    #[test]
    fn serde_round_trip() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "GATS"));
        let json = serde_json::to_string(&reg).unwrap();
        let decoded: ManufacturerRegistry = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.by_code("GATS").map(|m| m.guid), Some(g(1)));
    }
}
