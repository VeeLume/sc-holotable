//! A uniform read surface over the workspace's GUID-keyed record collections.
//!
//! Every cooked index in the workspace — `sc_items::Items`, `sc_tags::Tags`,
//! `sc_locations::Locations`, `sc_resources::Resources`, … — is a map from a
//! record [`Guid`] to a curated wrapper. They historically each exposed their
//! own ad-hoc accessor names (`iter` vs `all` vs `providers`, `get(&Guid)` vs
//! `get(Guid)`), which a consumer had to relearn per crate.
//!
//! [`RecordCollection`] is the single contract they all satisfy. Implementing it
//! turns a wrong `get` signature or a missing `iter` into a compile error rather
//! than a code-review nit, and lets generic code treat any collection uniformly.
//!
//! The canonical iteration follows `std` map semantics:
//! [`iter`](RecordCollection::iter) yields `(&Guid, &Item)` pairs;
//! [`values`](RecordCollection::values) yields `&Item`.
//!
//! The surface is **trait-only**: implementors do *not* mirror these methods as
//! inherent copies (that duplication is exactly the drift this trait removes), so
//! bring the trait into scope to call them, as with `std::io::Write`. Every
//! collection crate re-exports the trait next to its collection, and
//! `sc_holotable::prelude` includes it.
//!
//! What stays inherent on a collection: its constructors (`new` == `Default`,
//! and `build` — always that verb), secondary-key lookups (`by_<key>` returning
//! `Option<&Item>` for a single hit or `&[Guid]` for many), and `by_crc` /
//! `guid_by_crc` where class-CRC resolution is meaningful. A collection never
//! exposes its backing map as a public field.

use crate::Guid;

/// A read-only, GUID-keyed collection of curated record wrappers.
///
/// This is a *read* contract only — construction stays in each crate's inherent
/// `build`, because the input type is role-specific: foundational record-index
/// crates take `&RecordStore`, Datacore-consuming domain crates take `&Datacore`
/// plus the indices they need by reference, never both.
/// Secondary-key and class-CRC lookups also stay inherent; the trait covers the
/// universal GUID surface every collection shares.
pub trait RecordCollection {
    /// The curated wrapper stored per record GUID.
    type Item;

    /// Look up the entry for a record GUID. Always borrows the key.
    fn get(&self, guid: &Guid) -> Option<&Self::Item>;

    /// Number of entries. O(1).
    fn len(&self) -> usize;

    /// Iterate `(guid, item)` pairs (`std` map semantics). Order is unspecified.
    fn iter(&self) -> impl Iterator<Item = (&Guid, &Self::Item)> + '_;

    /// Whether the collection is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether a record GUID is present.
    fn contains(&self, guid: &Guid) -> bool {
        self.get(guid).is_some()
    }

    /// Iterate the wrapper values, dropping the keys.
    fn values(&self) -> impl Iterator<Item = &Self::Item> + '_ {
        self.iter().map(|(_, v)| v)
    }

    /// Iterate the record GUIDs, dropping the values.
    fn guids(&self) -> impl Iterator<Item = &Guid> + '_ {
        self.iter().map(|(k, _)| k)
    }
}
