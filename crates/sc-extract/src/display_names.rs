//! Pre-computed display-name cache for entity records.
//!
//! Resolving an entity's display name requires walking:
//!
//! ```text
//! EntityClassDefinition.Components
//!   → SAttachableComponentParams
//!     → AttachDef
//!       → Localization
//!         → Name  ("@item_NameGATS_Ballistic_S4")
//!           → strip '@' → look up in LocaleMap
//!             → resolved string ("CF-117 Bulldog")
//! ```
//!
//! Every consumer (bulkhead UI, sc-langpatch label enrichment, sc-weapons
//! display) needs this value, and every consumer currently walks the
//! chain independently. sc-extract does the walk once during parse and
//! caches the resolved strings in [`DisplayNameCache`], which is
//! serialized as part of the snapshot so `load_snapshot` apps get the
//! cached values for free.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::locale::{LocaleKey, LocaleMap};
use crate::svarog_datacore::{DataCoreDatabase, Instance, Value};
use crate::Guid;

// Bring `FromBytes` etc. into scope implicitly via the re-export at the
// crate root; no explicit use statement needed for them here.

/// Pre-computed display names for every entity record that has one.
///
/// Built once during `parse_from_p4k` and serialized in the snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplayNameCache {
    by_record: HashMap<Guid, String>,
}

impl DisplayNameCache {
    /// Construct an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace the display name for a record.
    pub fn insert(&mut self, guid: Guid, name: String) {
        self.by_record.insert(guid, name);
    }

    /// Look up the display name for a record GUID.
    pub fn get(&self, guid: &Guid) -> Option<&str> {
        self.by_record.get(guid).map(String::as_str)
    }

    /// Number of records with cached display names.
    pub fn len(&self) -> usize {
        self.by_record.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_record.is_empty()
    }

    /// Iterate over every (guid, name) pair.
    pub fn iter(&self) -> impl Iterator<Item = (&Guid, &str)> + '_ {
        self.by_record.iter().map(|(g, n)| (g, n.as_str()))
    }

    // ── Construction ────────────────────────────────────────────────────

    /// Walk every `EntityClassDefinition` record in the DCB and resolve
    /// each one's display name against the given [`LocaleMap`]. Records
    /// that don't have a display name (placeholders, NPCs, etc.) are
    /// simply omitted from the cache.
    pub fn from_database(db: &DataCoreDatabase, locale: &LocaleMap) -> Self {
        let mut cache = Self::new();

        for record in db.records_by_type("EntityClassDefinition") {
            if let Some(name) = resolve_entity_display_name(&record.as_instance(), db, locale) {
                cache.insert(record.id(), name);
            }
        }

        cache
    }
}

/// Resolve an entity's display name by walking its Components array,
/// finding `SAttachableComponentParams`, and looking up its localization
/// name key in the locale map.
///
/// Returns `None` if:
/// - The entity has no `Components` array
/// - No component is `SAttachableComponentParams`
/// - `AttachDef.Localization.Name` is missing or empty
/// - The resolved key isn't in the locale map
///
/// This function is exposed publicly for consumers that want to resolve
/// a single entity's name without building a whole cache. The `db`
/// parameter is required because svarog's `Instance` doesn't expose its
/// internal database handle and we need it to materialize array-element
/// class values.
pub fn resolve_entity_display_name(
    inst: &Instance<'_>,
    db: &DataCoreDatabase,
    locale: &LocaleMap,
) -> Option<String> {
    // Walk the Components array, looking for SAttachableComponentParams.
    let components = inst.get_array("Components")?;

    for value in components {
        let Some(component) = value_to_instance(&value, db) else {
            continue;
        };

        // We only care about SAttachableComponentParams.
        if component.type_name() != Some("SAttachableComponentParams") {
            continue;
        }

        // Walk component → AttachDef → Localization → Name. The Name
        // field is a DCB `Locale` — wrap it in a typed [`LocKey`] so the
        // rest of this function treats it as a localization reference,
        // not an arbitrary string.
        let attach_def = component.get_instance("AttachDef")?;
        let localization: Instance<'_> = attach_def.get_instance("Localization")?;
        let name_key = LocaleKey::new(localization.get_str("Name").unwrap_or(""));
        if name_key.stripped().is_empty() {
            continue;
        }

        // `LocaleMap::resolve` accepts `&LocKey` via `AsRef<str>` and
        // handles the leading `@` prefix internally.
        if let Some(resolved) = locale.resolve(&name_key) {
            return Some(resolved.to_string());
        }
    }

    None
}

/// Convert a [`Value`] that might be a Class, ClassRef, StrongPointer, or
/// WeakPointer into an [`Instance`].
fn value_to_instance<'a>(value: &Value<'a>, db: &'a DataCoreDatabase) -> Option<Instance<'a>> {
    match value {
        Value::Class { struct_index, data } => {
            Some(Instance::from_inline_data(db, *struct_index, data))
        }
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svarog_common::CigGuid;

    fn g(byte: u8) -> Guid {
        CigGuid::from_bytes([byte; 16])
    }

    #[test]
    fn new_cache_is_empty() {
        let cache = DisplayNameCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn insert_and_get() {
        let mut cache = DisplayNameCache::new();
        cache.insert(g(1), "Gladius".to_string());
        cache.insert(g(2), "Sabre".to_string());

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&g(1)), Some("Gladius"));
        assert_eq!(cache.get(&g(2)), Some("Sabre"));
        assert_eq!(cache.get(&g(3)), None);
    }

    #[test]
    fn insert_replaces_existing() {
        let mut cache = DisplayNameCache::new();
        cache.insert(g(1), "Old".to_string());
        cache.insert(g(1), "New".to_string());
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&g(1)), Some("New"));
    }

    #[test]
    fn iter_yields_all_entries() {
        let mut cache = DisplayNameCache::new();
        cache.insert(g(1), "A".to_string());
        cache.insert(g(2), "B".to_string());
        cache.insert(g(3), "C".to_string());
        assert_eq!(cache.iter().count(), 3);
    }

    #[test]
    fn serde_round_trip() {
        let mut cache = DisplayNameCache::new();
        cache.insert(g(1), "Gladius".to_string());
        cache.insert(g(2), "Sabre".to_string());

        let json = serde_json::to_string(&cache).unwrap();
        let decoded: DisplayNameCache = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded.get(&g(1)), Some("Gladius"));
    }
}
