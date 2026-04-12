//! Manufacturer registry — flat lookup over the DCB's `SCItemManufacturer` records.
//!
//! The DCB contains ~100 manufacturer records (Aegis, Anvil, Drake, GATS,
//! Behring, Crusader, etc.) that are referenced from thousands of item
//! records. [`ManufacturerRegistry`] provides both GUID-keyed and
//! code-keyed lookup.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::svarog_datacore::DataCoreDatabase;
use crate::Guid;

/// A single manufacturer entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manufacturer {
    /// The manufacturer's GUID (stable across game patches).
    pub guid: Guid,
    /// Short code / short name, e.g. `"GATS"` or `"AEGS"`.
    /// Derived from the DCB record name by stripping the `SCItemManufacturer.`
    /// type prefix.
    pub code: String,
    /// Localization key for the full name, e.g. `"@manufacturer_NameGATS"`.
    /// Look up via [`crate::LocaleMap::resolve`] to get the display name.
    pub name_key: Option<String>,
    /// Localization key for the description, e.g. `"@manufacturer_DescGATS"`.
    pub description_key: Option<String>,
}

/// Flat lookup table over every `SCItemManufacturer` record in the DCB.
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

    /// Insert or replace a manufacturer. Maintains both indices.
    pub fn insert(&mut self, manufacturer: Manufacturer) {
        // Update the code index if the guid already exists under a different code.
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
    /// `"GATS"`, `"AEGS"`, `"BEHR"`, etc.
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

    // ── Construction ────────────────────────────────────────────────────

    /// Walk the DCB and populate the registry from `SCItemManufacturer`
    /// records.
    ///
    /// For each record:
    /// - GUID comes from the record itself
    /// - Code is derived from the record name by stripping the
    ///   `"SCItemManufacturer."` prefix (e.g. `"SCItemManufacturer.GATS"` → `"GATS"`)
    /// - `name_key` and `description_key` come from
    ///   `Localization.Name` / `Localization.Description` on the instance
    ///   (the `@`-prefixed localization keys)
    ///
    /// Records that can't be read (missing name, malformed instance) are
    /// silently skipped with a `debug!` log. The real DCB has ~100 manufacturer
    /// records; losing a handful shouldn't block extraction.
    pub fn from_database(db: &DataCoreDatabase) -> Self {
        let mut registry = Self::new();

        for record in db.records_by_type("SCItemManufacturer") {
            let guid = record.id();
            let Some(full_name) = record.name() else {
                tracing::debug!("skipping manufacturer without name: {guid}");
                continue;
            };

            // Strip the "SCItemManufacturer." type prefix to get the code.
            let code = full_name
                .strip_prefix("SCItemManufacturer.")
                .unwrap_or(full_name)
                .to_string();

            let localization = record.get_instance("Localization");
            let name_key = localization
                .as_ref()
                .and_then(|l| l.get_str("Name"))
                .map(str::to_string);
            let description_key = localization
                .as_ref()
                .and_then(|l| l.get_str("Description"))
                .map(str::to_string);

            registry.insert(Manufacturer {
                guid,
                code,
                name_key,
                description_key,
            });
        }

        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svarog_common::CigGuid;

    fn g(byte: u8) -> Guid {
        CigGuid::from_bytes([byte; 16])
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
    fn insert_and_lookup_by_guid() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "GATS"));
        assert_eq!(reg.len(), 1);
        assert_eq!(reg.get(&g(1)).map(|m| m.code.as_str()), Some("GATS"));
    }

    #[test]
    fn lookup_by_code() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "GATS"));
        reg.insert(make(g(2), "AEGS"));

        assert_eq!(reg.by_code("GATS").map(|m| m.guid), Some(g(1)));
        assert_eq!(reg.by_code("AEGS").map(|m| m.guid), Some(g(2)));
        assert!(reg.by_code("MISSING").is_none());
    }

    #[test]
    fn insert_replaces_existing_and_updates_code_index() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "OldCode"));
        assert!(reg.by_code("OldCode").is_some());

        reg.insert(make(g(1), "NewCode"));
        assert_eq!(reg.len(), 1);
        assert!(reg.by_code("OldCode").is_none());
        assert!(reg.by_code("NewCode").is_some());
    }

    #[test]
    fn all_iterates_every_manufacturer() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "A"));
        reg.insert(make(g(2), "B"));
        reg.insert(make(g(3), "C"));
        assert_eq!(reg.all().count(), 3);
    }

    #[test]
    fn name_and_description_keys_stored() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "GATS"));
        let m = reg.get(&g(1)).unwrap();
        assert_eq!(m.name_key.as_deref(), Some("@manufacturer_NameGATS"));
        assert_eq!(m.description_key.as_deref(), Some("@manufacturer_DescGATS"));
    }

    #[test]
    fn serde_round_trip() {
        let mut reg = ManufacturerRegistry::new();
        reg.insert(make(g(1), "GATS"));
        reg.insert(make(g(2), "AEGS"));

        let json = serde_json::to_string(&reg).unwrap();
        let decoded: ManufacturerRegistry = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(
            decoded.by_code("GATS").map(|m| m.guid),
            Some(g(1))
        );
    }
}
