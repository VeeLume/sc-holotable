//! Coordination build-context + bundled processed snapshot.
//!
//! [`build_foundations`] runs every foundational cooked index in **one**
//! `db.all_records()` pass via the bundled-walk API, instead of four separate
//! `X::build` calls. [`HolotableSnapshot`] is the serializable bundle of those
//! indices for fast load (it rides in a [`sc_extract::ProcessedSnapshot`]).

use std::path::Path;

use sc_extract::{
    BundledWalk, Datacore, ProcessedSnapshot, RecordPaths, RecordPathsBuilder, Result, SnapshotMeta,
};
use sc_items::{ItemCache, ItemCacheBuilder};
use sc_manufacturers::{ManufacturerRegistry, ManufacturerRegistryBuilder};
use sc_tags::{TagTree, TagTreeBuilder};
use serde::{Deserialize, Serialize};

/// Every foundational cooked index, built together. The live, in-memory bundle
/// (holds [`ItemCache`], which is not yet serde-capable — see
/// [`HolotableSnapshot`] for the persistable subset).
pub struct Foundations {
    pub items: ItemCache,
    pub tags: TagTree,
    pub manufacturers: ManufacturerRegistry,
    pub paths: RecordPaths,
}

/// Build all foundational indices in a single bundled `all_records` pass.
///
/// The `RecordPaths` member declares [`sc_extract::Interest::AllRecords`], so
/// the walk is a full pass and the three type-readers ride along for free —
/// strictly cheaper than four independent `X::build`s.
pub fn build_foundations(datacore: &Datacore) -> Foundations {
    let (items, tags, manufacturers, paths) = BundledWalk::new(datacore).run((
        ItemCacheBuilder::default(),
        TagTreeBuilder::default(),
        ManufacturerRegistryBuilder::default(),
        RecordPathsBuilder::default(),
    ));
    Foundations {
        items,
        tags,
        manufacturers,
        paths,
    }
}

/// Cook-format version of [`HolotableSnapshot`]. Bump when any included
/// index's serialized layout changes.
pub const HOLOTABLE_COOK_VERSION: u32 = 1;

/// A batteries-included bundle of cooked indices, serializable for fast load.
///
/// Fields are optional so a producer can ship only what a consumer needs and a
/// consumer can tolerate a producer that omitted some. Holds only the
/// serde-clean indices today; [`ItemCache`] joins once its serde adapter lands
/// (regen-gated — see the project note). Persisted via
/// [`sc_extract::ProcessedSnapshot`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HolotableSnapshot {
    pub tags: Option<TagTree>,
    pub manufacturers: Option<ManufacturerRegistry>,
    pub paths: Option<RecordPaths>,
}

impl HolotableSnapshot {
    /// Capture the serde-clean indices from a live [`Foundations`].
    pub fn from_foundations(f: &Foundations) -> Self {
        Self {
            tags: Some(f.tags.clone()),
            manufacturers: Some(f.manufacturers.clone()),
            paths: Some(f.paths.clone()),
        }
    }

    /// Serialize and write to `path` (zstd + msgpack, version-guarded).
    pub fn save(&self, meta: SnapshotMeta, path: &Path) -> Result<()> {
        ProcessedSnapshot::new(meta, HOLOTABLE_COOK_VERSION, self.clone()).save(path)
    }

    /// Load from `path`, rejecting a stale cook version (caller falls back to a
    /// raw hydrate or a fresh [`build_foundations`]).
    pub fn load(path: &Path) -> Result<Self> {
        Ok(ProcessedSnapshot::<HolotableSnapshot>::load(path, HOLOTABLE_COOK_VERSION)?.into_index())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_extract::{Guid, RecordPath};

    #[test]
    fn holotable_snapshot_round_trip() {
        let mut paths = RecordPaths::new();
        paths.insert(RecordPath {
            guid: Guid::from_bytes([1; 16]),
            name: "X".into(),
            struct_index: 0,
            is_main: true,
            path: "libs/foundry/records/a/b.xml".into(),
        });
        let snap = HolotableSnapshot {
            tags: Some(TagTree::new()),
            manufacturers: Some(ManufacturerRegistry::new()),
            paths: Some(paths),
        };

        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("holotable.cook");
        snap.save(SnapshotMeta::default(), &p).unwrap();

        let loaded = HolotableSnapshot::load(&p).unwrap();
        assert_eq!(loaded.paths.expect("paths present").len(), 1);
        assert!(loaded.tags.is_some());
        assert!(loaded.manufacturers.is_some());
    }
}
