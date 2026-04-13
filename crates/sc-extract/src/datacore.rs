//! Runtime [`Datacore`] session and serializable [`DatacoreSnapshot`].
//!
//! This module splits the old `ExtractedData` god-struct into two pieces:
//!
//! - [`DatacoreSnapshot`] — fully owned, serde-friendly bundle of every
//!   DCB-derived value produced in one parse pass (records, graph, tag tree,
//!   manufacturers, display names). No live handles, portable on disk.
//! - [`Datacore`] — live session that **owns** a [`DataCoreDatabase`] so
//!   consumers can keep running raw svarog queries after high-level parsing.
//!   Holds a [`DatacoreSnapshot`] for the cooked data.
//!
//! Constructed via [`Datacore::parse`]. See [`crate::asset_data::AssetData`]
//! for the asset-sourced companion (currently just the locale map).

use serde::{Deserialize, Serialize};

use crate::asset_data::AssetData;
use crate::assets::AssetSource;
use crate::config::DatacoreConfig;
use crate::display_names::DisplayNameCache;
use crate::error::{Error, Result};
use crate::generated::{Builder, RecordStore};
use crate::graph::ReferenceGraph;
use crate::manufacturers::ManufacturerRegistry;
use crate::tags::TagTree;
use crate::svarog_datacore::DataCoreDatabase;

/// Serializable snapshot of every DCB-derived value from one parse pass.
///
/// Produced by [`Datacore::into_snapshot`] / [`Datacore::snapshot`] and
/// embedded in [`crate::ExtractSnapshot`] for on-disk persistence.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DatacoreSnapshot {
    /// Every top-level DCB record, split by concrete Rust type.
    #[serde(default)]
    pub records: RecordStore,

    /// Cross-record reference graph (forward + reverse edges). Empty if
    /// [`DatacoreConfig::build_graph`] was false at parse time.
    #[serde(default)]
    pub graph: ReferenceGraph,

    /// Hierarchical tag tree.
    #[serde(default)]
    pub tag_tree: TagTree,

    /// Manufacturer lookup by GUID and ticker code.
    #[serde(default)]
    pub manufacturers: ManufacturerRegistry,

    /// Pre-computed localized entity display names. Populated only when
    /// [`DatacoreConfig::build_display_names`] is true *and* the
    /// accompanying [`AssetData`] had a non-empty locale.
    #[serde(default)]
    pub display_names: DisplayNameCache,
}

impl DatacoreSnapshot {
    /// Total number of DCB records held across all top-level types.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
}

/// Live datacore session: owns the parsed [`DataCoreDatabase`] plus the
/// cooked [`DatacoreSnapshot`].
///
/// The database is kept alive so consumers can run raw svarog queries
/// (via [`Datacore::db`]) after high-level parsing — a capability the old
/// `parse_inner` flow dropped on the floor.
///
/// Not serializable. To persist, call [`Datacore::into_snapshot`] and embed
/// the result in an [`crate::ExtractSnapshot`].
pub struct Datacore {
    db: DataCoreDatabase,
    snapshot: DatacoreSnapshot,
}

impl Datacore {
    /// Parse the DCB from an open [`AssetSource`] and build every index
    /// enabled by `config`. `asset_data` provides the locale map used to
    /// resolve entity display names — pass [`AssetData::default`] if you
    /// don't need localized names.
    pub fn parse(
        assets: &AssetSource,
        asset_data: &AssetData,
        config: &DatacoreConfig,
    ) -> Result<Self> {
        let start = std::time::Instant::now();

        tracing::info!("locating Game2.dcb");
        let (dcb_name, dcb_bytes) = assets
            .find_and_read(|name| name.to_ascii_lowercase().ends_with("game2.dcb"))?
            .ok_or(Error::DcbNotFound)?;
        tracing::info!(dcb_name = %dcb_name, bytes = dcb_bytes.len(), "extracted Game2.dcb");

        tracing::info!("parsing DataCore");
        let db = DataCoreDatabase::parse(&dcb_bytes).map_err(Error::DcbParse)?;

        tracing::info!("building record store");
        let records = Builder::new(&db).consume_database().finish();
        tracing::info!(records = records.len(), "record store built");

        let graph = if config.build_graph {
            tracing::info!("building reference graph");
            ReferenceGraph::from_database(&db)
        } else {
            ReferenceGraph::new()
        };

        let tag_tree = if config.build_tag_tree {
            tracing::info!("building tag tree");
            TagTree::from_database(&db)
        } else {
            TagTree::new()
        };

        let manufacturers = if config.build_manufacturers {
            tracing::info!("building manufacturer registry");
            ManufacturerRegistry::from_database(&db)
        } else {
            ManufacturerRegistry::new()
        };

        let display_names = if config.build_display_names {
            tracing::info!("building display name cache");
            DisplayNameCache::from_database(&db, &asset_data.locale)
        } else {
            DisplayNameCache::new()
        };

        let snapshot = DatacoreSnapshot {
            records,
            graph,
            tag_tree,
            manufacturers,
            display_names,
        };

        tracing::info!(
            records = snapshot.records.len(),
            elapsed_ms = start.elapsed().as_millis(),
            "datacore parse complete"
        );

        Ok(Self { db, snapshot })
    }

    /// Raw access to the live [`DataCoreDatabase`]. Use this for svarog
    /// queries the high-level API doesn't cover — the database owns the
    /// DCB bytes, so queries stay valid for the lifetime of this session.
    pub fn db(&self) -> &DataCoreDatabase {
        &self.db
    }

    /// Borrow the cooked [`DatacoreSnapshot`] without consuming the session.
    pub fn snapshot(&self) -> &DatacoreSnapshot {
        &self.snapshot
    }

    /// Consume the session and return only the snapshot. Drops the live
    /// [`DataCoreDatabase`]; use this right before embedding in an
    /// [`crate::ExtractSnapshot`] for save.
    pub fn into_snapshot(self) -> DatacoreSnapshot {
        self.snapshot
    }

    /// Delegate convenience: the [`RecordStore`] inside the snapshot.
    pub fn records(&self) -> &RecordStore {
        &self.snapshot.records
    }
}

impl std::fmt::Debug for Datacore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Datacore")
            .field("records", &self.snapshot.records.len())
            .field("graph_edges", &self.snapshot.graph.edge_count())
            .field("display_names", &self.snapshot.display_names.len())
            .finish()
    }
}
