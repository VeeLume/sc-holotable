//! Runtime [`Datacore`] session.
//!
//! [`Datacore`] is a live parse-run session that **owns** a
//! [`DataCoreDatabase`] (so consumers can keep running raw svarog queries
//! after high-level parsing) alongside the cooked [`RecordStore`] produced
//! in one eager pass over every top-level DCB record.
//!
//! Not serialized: persistence happens through [`crate::ExtractSnapshot`],
//! which archives the raw DCB bytes and re-parses on load. Constructed via
//! [`Datacore::parse`]. See [`crate::asset_data::AssetData`] for the
//! asset-sourced companion (currently just the locale map).

use crate::Guid;
use crate::asset_data::AssetData;
use crate::assets::AssetSource;
use crate::error::{Error, Result};
use crate::generated::{Builder, Handle, RecordLookup, RecordStore};
use crate::svarog_datacore::DataCoreDatabase;

/// Live datacore session: owns the parsed [`DataCoreDatabase`] plus the
/// cooked [`RecordStore`].
///
/// The database is kept alive so consumers can run raw svarog queries
/// (via [`Datacore::db`]) after high-level parsing.
///
/// Not persisted directly. Snapshot files archive the raw DCB bytes
/// (via [`crate::ExtractSnapshot::capture`]) and re-materialize a live
/// [`Datacore`] on load (via [`crate::ExtractSnapshot::hydrate`]).
pub struct Datacore {
    db: DataCoreDatabase,
    records: RecordStore,
}

impl Datacore {
    /// Parse the DCB from an open [`AssetSource`] into the record store.
    ///
    /// Cooked domain indices (items, tags, manufacturers, reference graph)
    /// are no longer built here — each is an explicit `build(&datacore)` in
    /// its owning crate (`sc-items`, `sc-tags`, `sc-manufacturers`, or
    /// [`ReferenceGraph::from_database`]). `asset_data` is retained for API
    /// symmetry; DCB-derived data is locale-independent.
    pub fn parse(assets: &AssetSource, asset_data: &AssetData) -> Result<Self> {
        let _ = asset_data; // reserved for future asset-derived indices
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

        tracing::info!(
            records = records.len(),
            elapsed_ms = start.elapsed().as_millis(),
            "datacore parse complete"
        );

        Ok(Self { db, records })
    }

    /// Raw access to the live [`DataCoreDatabase`]. Use this for svarog
    /// queries the high-level API doesn't cover — the database owns the
    /// DCB bytes, so queries stay valid for the lifetime of this session.
    pub fn db(&self) -> &DataCoreDatabase {
        &self.db
    }

    /// Borrow the cooked [`RecordStore`] — every top-level DCB record, split
    /// by concrete Rust type.
    pub fn records(&self) -> &RecordStore {
        &self.records
    }

    /// Resolve a record `Reference` GUID back onto the typed surface.
    ///
    /// Generated structs emit `DataType::Reference` fields as bare
    /// `Option<Guid>` (cross-record pointers aren't followed into the
    /// typed pool graph). This turns such a GUID into a typed `&T`, so a
    /// consumer can stay on the typed API across reference hops instead of
    /// dropping to [`Self::db`] and walking the raw instance by string
    /// field name.
    ///
    /// `T` must be a *seeded record type* — the [`RecordLookup`] bound is a
    /// compile-time guarantee that `T` is GUID-addressable. Returns `None`
    /// if no record of type `T` carries `guid` (wrong type, dangling
    /// reference, or a feature-gated-away record).
    pub fn resolve<T: RecordLookup>(&self, guid: &Guid) -> Option<&T> {
        let store = self.records();
        T::lookup(&store.records, guid)?.get(&store.pools)
    }

    /// Like [`Self::resolve`] but returns the [`Handle`] rather than the
    /// borrowed value — for when the handle must outlive a `pools` borrow
    /// or be stored.
    pub fn resolve_handle<T: RecordLookup>(&self, guid: &Guid) -> Option<Handle<T>> {
        T::lookup(&self.records().records, guid)
    }
}

impl std::fmt::Debug for Datacore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Datacore")
            .field("records", &self.records.len())
            .finish()
    }
}
