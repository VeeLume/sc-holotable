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
use sc_items::{Items, ItemsBuilder};
use sc_locations::{Locations, LocationsBuilder};
use sc_manufacturers::{Manufacturers, ManufacturersBuilder};
use sc_resources::{Resources, ResourcesBuilder};
use sc_tags::{Tags, TagsBuilder};
use serde::{Deserialize, Serialize};

/// Every foundational cooked index, built together. See
/// [`HolotableSnapshot`] for the persistable form.
pub struct Foundations {
    pub items: Items,
    pub tags: Tags,
    pub manufacturers: Manufacturers,
    pub resources: Resources,
    pub locations: Locations,
    pub paths: RecordPaths,
}

/// Build all foundational indices in a single bundled `all_records` pass.
///
/// The `RecordPaths` member declares [`sc_extract::Interest::AllRecords`], so
/// the walk is a full pass and the four type-readers ride along for free —
/// strictly cheaper than five independent `X::build`s.
pub fn build_foundations(datacore: &Datacore) -> Foundations {
    let (items, tags, manufacturers, resources, locations, paths) =
        BundledWalk::new(datacore).run((
            ItemsBuilder::default(),
            TagsBuilder::default(),
            ManufacturersBuilder::default(),
            ResourcesBuilder::default(),
            LocationsBuilder::default(),
            RecordPathsBuilder::default(),
        ));
    Foundations {
        items,
        tags,
        manufacturers,
        resources,
        locations,
        paths,
    }
}

/// Cook-format version of [`HolotableSnapshot`]. Bump when any included
/// index's serialized layout changes.
///
/// v2 (2026-05-31): added optional `resources: Resources` field.
/// v3 (2026-06-03): added optional `locations: Locations` field.
pub const HOLOTABLE_COOK_VERSION: u32 = 3;

/// A batteries-included bundle of cooked indices, serializable for fast load.
///
/// Fields are optional so a producer can ship only what a consumer needs and a
/// consumer can tolerate a producer that omitted some. Persisted via
/// [`sc_extract::ProcessedSnapshot`]. `Items` serializes its generated
/// enums as DCB strings (see `sc_items`' serde adapter).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HolotableSnapshot {
    pub items: Option<Items>,
    pub tags: Option<Tags>,
    pub manufacturers: Option<Manufacturers>,
    pub resources: Option<Resources>,
    pub locations: Option<Locations>,
    pub paths: Option<RecordPaths>,
}

impl HolotableSnapshot {
    /// Capture the serde-clean indices from a live [`Foundations`].
    pub fn from_foundations(f: &Foundations) -> Self {
        Self {
            items: Some(f.items.clone()),
            tags: Some(f.tags.clone()),
            manufacturers: Some(f.manufacturers.clone()),
            resources: Some(f.resources.clone()),
            locations: Some(f.locations.clone()),
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
        Ok(
            ProcessedSnapshot::<HolotableSnapshot>::load(path, HOLOTABLE_COOK_VERSION)?
                .into_index(),
        )
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
            tags: Some(Tags::new()),
            manufacturers: Some(Manufacturers::new()),
            resources: Some(Resources::new()),
            paths: Some(paths),
            ..Default::default()
        };

        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("holotable.cook");
        snap.save(SnapshotMeta::default(), &p).unwrap();

        let loaded = HolotableSnapshot::load(&p).unwrap();
        assert_eq!(loaded.paths.expect("paths present").len(), 1);
        assert!(loaded.tags.is_some());
        assert!(loaded.manufacturers.is_some());
        assert!(loaded.resources.is_some());
    }
}
