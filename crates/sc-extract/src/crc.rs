//! Class CRC — the u32 Star Citizen's EntityGraph gRPC service puts on the
//! wire to identify records (item classes, resource types, …) instead of the
//! full 16-byte GUID. Seen on the wire as `class_guid_crc`, `guid_hash_crc`,
//! and the resource-descriptor `resource_id`.
//!
//! The CRC is computable from a GUID alone, so resolution belongs at the
//! foundation that owns every record GUID. [`class_crc`] is the forward
//! direction; [`CrcIndex`] is the reverse, built over the complete record set
//! (via [`RecordPaths`] or a live [`Datacore`]).

use std::collections::HashMap;

use tracing::warn;

use crate::{Datacore, Guid, RecordPaths};

/// Class CRC of a record GUID: `crc32c(guid.as_bytes())` — CRC32C (Castagnoli)
/// over the GUID's 16 bytes in [`Guid::as_bytes`] storage order (svarog's
/// internal layout, **not** the dashed UUID string and **not** a
/// big/little-endian reordering of it).
///
/// Reverse-engineered and verified byte-exact against live EntityGraph
/// `(guid, crc)` pairs. Use [`CrcIndex`] for the reverse lookup.
pub fn class_crc(guid: &Guid) -> u32 {
    svarog_common::crc::hash_bytes(guid.as_bytes())
}

/// Reverse index: EntityGraph wire [`class_crc`] → record GUID, over the whole
/// DCB record set.
///
/// Derived data — **not serialized**. The GUID universe it indexes already
/// lives in a [`RecordPaths`] (which *is* snapshotted), so a consumer that
/// needs CRC resolution rebuilds the index on demand with
/// [`CrcIndex::from_paths`] rather than paying for it in every snapshot.
///
/// # Coverage
///
/// Exhaustive for **records** — every main record and sub-record reachable via
/// [`RecordPaths`] / `db.all_records()`. A CRC that resolves to `None` was
/// computed over a GUID that is not a DCB record (an instance-level or
/// synthesized GUID); that is the genuine raw-only frontier, not a gap in the
/// index.
#[derive(Debug, Clone, Default)]
pub struct CrcIndex {
    by_crc: HashMap<u32, Guid>,
}

impl CrcIndex {
    /// Empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build from any GUID source. The primary entry points
    /// ([`from_paths`](Self::from_paths) / [`from_datacore`](Self::from_datacore))
    /// delegate here.
    pub fn from_guids(guids: impl IntoIterator<Item = Guid>) -> Self {
        let mut index = Self::new();
        for guid in guids {
            index.insert(guid);
        }
        index
    }

    /// Build from a [`RecordPaths`] — the recommended path. `RecordPaths`
    /// already enumerates every DCB record GUID and is part of the standard
    /// snapshot, so this needs no extra DCB walk.
    pub fn from_paths(paths: &RecordPaths) -> Self {
        Self::from_guids(paths.iter().map(|r| r.guid))
    }

    /// Build directly from a live [`Datacore`] by walking `db.all_records()`.
    /// Use when a `RecordPaths` isn't on hand.
    pub fn from_datacore(datacore: &Datacore) -> Self {
        Self::from_guids(datacore.db().all_records().map(|r| r.id()))
    }

    /// Index one GUID under its [`class_crc`]. Logs (but tolerates) a CRC
    /// collision: CRC32 can in principle collide across the large record set,
    /// though none is observed in live data — the last writer wins.
    pub fn insert(&mut self, guid: Guid) {
        let crc = class_crc(&guid);
        if let Some(prev) = self.by_crc.insert(crc, guid)
            && prev != guid
        {
            warn!(crc, %prev, new = %guid, "class_crc collision: CrcIndex entry overwritten");
        }
    }

    /// Resolve an EntityGraph wire CRC back to its record GUID.
    pub fn guid(&self, crc: u32) -> Option<Guid> {
        self.by_crc.get(&crc).copied()
    }

    /// Whether any record hashes to `crc`.
    pub fn contains(&self, crc: u32) -> bool {
        self.by_crc.contains_key(&crc)
    }

    /// Number of indexed CRCs (distinct after collisions collapse).
    pub fn len(&self) -> usize {
        self.by_crc.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_crc.is_empty()
    }

    /// Iterate `(crc, guid)` pairs. Order unspecified.
    pub fn iter(&self) -> impl Iterator<Item = (u32, Guid)> + '_ {
        self.by_crc.iter().map(|(&crc, &guid)| (crc, guid))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn class_crc_matches_live_ground_truth() {
        // Verified byte-exact against a live EntityGraph (guid, crc) pair: the
        // CRC is crc32c over CigGuid storage-order bytes.
        let guid = Guid::from_str("bba17984-86e7-4002-aab7-f33f1279fe1f").unwrap();
        assert_eq!(class_crc(&guid), 1_038_868_829);

        // Same hash bridge across record domains: this is the RecordId of
        // `StarMapObject.Nyx_Levski` (a location, not an item), and the CRC is
        // the location `subject_id` seen on the EntityGraph wire. Records of
        // every type share one CRC space, so one CrcIndex resolves all of them.
        let levski = Guid::from_str("468d4102-a210-47b5-8bc3-084f791a173c").unwrap();
        assert_eq!(class_crc(&levski), 3_723_364_946);
    }

    #[test]
    fn index_round_trips_crc_to_guid() {
        let a = Guid::from_str("bba17984-86e7-4002-aab7-f33f1279fe1f").unwrap();
        let b = Guid::from_bytes([9; 16]);
        let index = CrcIndex::from_guids([a, b]);

        assert_eq!(index.len(), 2);
        assert_eq!(index.guid(class_crc(&a)), Some(a));
        assert_eq!(index.guid(class_crc(&b)), Some(b));
        // A CRC no record hashes to resolves to nothing.
        assert_eq!(index.guid(class_crc(&a).wrapping_add(1)), None);
        assert!(!index.contains(class_crc(&a).wrapping_add(1)));
    }
}
