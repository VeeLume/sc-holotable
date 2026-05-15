//! Pre-computed localization-key cache for entity records.
//!
//! Every `EntityClassDefinition` carries `Components →
//! SAttachableComponentParams.AttachDef.Localization` with three locale
//! fields: `Name`, `ShortName`, `Description`. The cache walks the chain
//! once at parse time and stores the keys, indexed by entity GUID:
//!
//! ```text
//! EntityClassDefinition.Components
//!   → SAttachableComponentParams
//!     → AttachDef
//!       → Localization
//!         → Name / ShortName / Description  (LocaleKey, raw '@'-prefixed)
//! ```
//!
//! The cache is **locale-independent** — it stores keys, not resolved
//! strings. Resolution is the call site's job, through whatever
//! [`crate::LocaleMap`] is current. See [`docs/localization.md`] for the
//! rule and rationale.
//!
//! [`docs/localization.md`]: ../../../docs/localization.md

use std::collections::HashMap;

use crate::Guid;
use crate::locale::LocaleKey;
use crate::svarog_datacore::{DataCoreDatabase, Instance, Value};

/// The three [`LocaleKey`]s every localized item exposes.
///
/// All fields keep the leading `@` that the DCB carries — the workspace
/// rule is "keys are raw, resolution happens at the call site". A `None`
/// means the underlying DCB field was empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocalizedItem {
    /// `Localization.Name` — the primary display name key.
    pub name_key: Option<LocaleKey>,
    /// `Localization.ShortName` — typically the manufacturer ticker /
    /// short ID. Empty on most entities.
    pub short_name_key: Option<LocaleKey>,
    /// `Localization.Description` — the long-form item description key.
    pub desc_key: Option<LocaleKey>,
}

/// Pre-computed localization keys for every entity record that exposes a
/// `SAttachableComponentParams.AttachDef.Localization` block.
///
/// Built once during [`crate::Datacore::parse`] and held inside the
/// snapshot. Locale-agnostic — the cache content does not depend on
/// which [`crate::LocaleMap`] the consumer eventually resolves through.
#[derive(Debug, Clone, Default)]
pub struct LocalizedItemCache {
    by_record: HashMap<Guid, LocalizedItem>,
}

impl LocalizedItemCache {
    /// Construct an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace the cached entry for a record.
    pub fn insert(&mut self, guid: Guid, item: LocalizedItem) {
        self.by_record.insert(guid, item);
    }

    /// Look up the cached entry for a record GUID.
    pub fn get(&self, guid: &Guid) -> Option<&LocalizedItem> {
        self.by_record.get(guid)
    }

    /// Convenience: just the `Name` key for a record.
    pub fn name_key(&self, guid: &Guid) -> Option<&LocaleKey> {
        self.by_record.get(guid).and_then(|i| i.name_key.as_ref())
    }

    /// Convenience: just the `Description` key for a record.
    pub fn desc_key(&self, guid: &Guid) -> Option<&LocaleKey> {
        self.by_record.get(guid).and_then(|i| i.desc_key.as_ref())
    }

    /// Convenience: just the `ShortName` key for a record.
    pub fn short_name_key(&self, guid: &Guid) -> Option<&LocaleKey> {
        self.by_record
            .get(guid)
            .and_then(|i| i.short_name_key.as_ref())
    }

    /// Number of records with cached localization keys.
    pub fn len(&self) -> usize {
        self.by_record.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_record.is_empty()
    }

    /// Iterate over every (guid, item) pair.
    pub fn iter(&self) -> impl Iterator<Item = (&Guid, &LocalizedItem)> + '_ {
        self.by_record.iter()
    }

    // ── Construction ────────────────────────────────────────────────────

    /// Walk every `EntityClassDefinition` record in the DCB and cache
    /// its `Localization` keys. Records without a
    /// `SAttachableComponentParams.AttachDef.Localization` block are
    /// omitted.
    pub fn from_database(db: &DataCoreDatabase) -> Self {
        let mut cache = Self::new();

        for record in db.records_by_type("EntityClassDefinition") {
            if let Some(item) = resolve_entity_localization(&record.as_instance(), db) {
                cache.insert(record.id(), item);
            }
        }

        cache
    }
}

/// Resolve an entity's localization keys by walking its `Components`
/// array, finding `SAttachableComponentParams`, and pulling the three
/// `LocaleKey` fields out of `AttachDef.Localization`.
///
/// Returns `None` when the entity has no `SAttachableComponentParams` or
/// the chain doesn't resolve. A returned `LocalizedItem` may still have
/// individual `None` fields if a particular DCB locale field was empty.
///
/// Exposed publicly so consumers can resolve a single entity without
/// building a whole cache (the live svarog `Instance` is required because
/// it doesn't expose its internal `db` handle to materialize array
/// elements).
pub fn resolve_entity_localization(
    inst: &Instance<'_>,
    db: &DataCoreDatabase,
) -> Option<LocalizedItem> {
    let components = inst.get_array("Components")?;

    for value in components {
        let Some(component) = value_to_instance(&value, db) else {
            continue;
        };
        if component.type_name() != Some("SAttachableComponentParams") {
            continue;
        }

        let attach_def = component.get_instance("AttachDef")?;
        let localization = attach_def.get_instance("Localization")?;

        return Some(LocalizedItem {
            name_key: locale_field(&localization, "Name"),
            short_name_key: locale_field(&localization, "ShortName"),
            desc_key: locale_field(&localization, "Description"),
        });
    }

    None
}

/// Pull a `Locale` field out of an `Instance` as a `LocaleKey`. Returns
/// `None` when the field is missing or empty (after preserving any `@`
/// prefix — keys are stored raw).
fn locale_field(inst: &Instance<'_>, name: &str) -> Option<LocaleKey> {
    let raw = inst.get_str(name)?;
    if raw.is_empty() {
        return None;
    }
    Some(LocaleKey::new(raw))
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

    #[test]
    fn cache_starts_empty() {
        let cache = LocalizedItemCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn insert_and_get_round_trip() {
        let mut cache = LocalizedItemCache::new();
        let guid = Guid::default();
        let item = LocalizedItem {
            name_key: Some(LocaleKey::new("@item_NameTest")),
            short_name_key: None,
            desc_key: Some(LocaleKey::new("@item_DescTest")),
        };
        cache.insert(guid, item.clone());

        let got = cache.get(&guid).unwrap();
        assert_eq!(got, &item);
        assert_eq!(cache.name_key(&guid).unwrap().as_str(), "@item_NameTest");
        assert_eq!(cache.desc_key(&guid).unwrap().as_str(), "@item_DescTest");
        assert!(cache.short_name_key(&guid).is_none());
    }

    #[test]
    fn missing_guid_returns_none() {
        let cache = LocalizedItemCache::new();
        assert!(cache.get(&Guid::default()).is_none());
        assert!(cache.name_key(&Guid::default()).is_none());
    }

    #[test]
    fn keys_keep_at_prefix() {
        let mut cache = LocalizedItemCache::new();
        let guid = Guid::default();
        cache.insert(
            guid,
            LocalizedItem {
                name_key: Some(LocaleKey::new("@item_NameRaw")),
                short_name_key: None,
                desc_key: None,
            },
        );
        // Stored key keeps the '@'; consumers strip at call time if needed.
        assert!(cache.name_key(&guid).unwrap().as_str().starts_with('@'));
    }
}
