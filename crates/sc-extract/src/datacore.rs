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
use crate::generated::metadata::{GENERATED_CHANGELIST, GENERATED_GAME_BRANCH};
use crate::generated::{Builder, Handle, RecordLookup, RecordStore, UnknownRecordType};
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
    unknown_record_types: Vec<UnknownRecordType>,
}

/// Stack size for the parse worker thread. The materialize walk recurses per
/// record-nesting depth; deeply-nested records (e.g. `GameMode.SC_Default`,
/// pulled in by the `gamemode` feature) overflow the default ~1 MB stack.
const PARSE_STACK_SIZE: usize = 64 * 1024 * 1024;

impl Datacore {
    /// Parse the DCB from an open [`AssetSource`] into the record store.
    ///
    /// Cooked domain indices (items, tags, manufacturers, reference graph)
    /// are no longer built here — each is an explicit `build(&datacore)` in
    /// its owning crate (`sc-items`, `sc-tags`, `sc-manufacturers`, or
    /// [`crate::ReferenceGraph::from_database`]). `asset_data` is retained for API
    /// symmetry; DCB-derived data is locale-independent.
    pub fn parse(assets: &AssetSource, asset_data: &AssetData) -> Result<Self> {
        let _ = asset_data; // reserved for future asset-derived indices
        let start = std::time::Instant::now();

        tracing::info!("locating Game2.dcb");
        let (dcb_name, dcb_bytes) = assets
            .find_and_read(|name| name.to_ascii_lowercase().ends_with("game2.dcb"))?
            .ok_or(Error::DcbNotFound)?;
        tracing::info!(dcb_name = %dcb_name, bytes = dcb_bytes.len(), "extracted Game2.dcb");

        // The materialize walk recurses per record-nesting depth; deeply-nested
        // records (e.g. `GameMode.SC_Default`, pulled in by the `gamemode`
        // feature) overflow the default ~1 MB thread stack. Run it on a large
        // stack so every consumer — and the snapshot `hydrate` path — is
        // protected without each spawning its own big-stack thread.
        std::thread::Builder::new()
            .stack_size(PARSE_STACK_SIZE)
            .spawn(move || -> Result<Self> {
                tracing::info!("parsing DataCore");
                let db = DataCoreDatabase::parse(&dcb_bytes).map_err(Error::DcbParse)?;

                tracing::info!("building record store");
                let builder = Builder::new(&db).consume_database();
                let unknown_record_types = builder.unknown_record_types().to_vec();
                let records = builder.finish();
                tracing::info!(records = records.len(), "record store built");
                log_binding_drift(&unknown_record_types);

                tracing::info!(
                    records = records.len(),
                    elapsed_ms = start.elapsed().as_millis(),
                    "datacore parse complete"
                );
                Ok(Self {
                    db,
                    records,
                    unknown_record_types,
                })
            })
            .expect("spawn datacore parse thread")
            .join()
            .expect("datacore parse thread panicked")
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

    /// Record types in this DCB that the generated bindings have never
    /// seen, sorted by name. Their records were skipped, so every index
    /// built from this session is silently missing them.
    ///
    /// Empty when the bindings match the game build. Non-empty means the
    /// bindings are stale: [`Self::parse`] already logged a `warn!` naming
    /// the build they came from. Debug builds panic during parsing instead,
    /// so this is only ever non-empty in release builds.
    ///
    /// ```no_run
    /// # fn demo(datacore: &sc_extract::Datacore) {
    /// for t in datacore.unknown_record_types() {
    ///     println!("{}: {} records skipped", t.name, t.records);
    /// }
    /// # }
    /// ```
    pub fn unknown_record_types(&self) -> &[UnknownRecordType] {
        &self.unknown_record_types
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

/// Name the game build the bindings came from on every parse, and warn when
/// the DCB has record types they don't know.
fn log_binding_drift(unknown: &[UnknownRecordType]) {
    let bindings_changelist = GENERATED_CHANGELIST.unwrap_or("-");
    if unknown.is_empty() {
        tracing::info!(
            bindings_branch = GENERATED_GAME_BRANCH,
            bindings_changelist,
            "generated bindings know every record type in this DCB"
        );
        return;
    }
    let names: Vec<&str> = unknown.iter().map(|t| t.name.as_str()).collect();
    tracing::warn!(
        bindings_branch = GENERATED_GAME_BRANCH,
        bindings_changelist,
        unknown_types = unknown.len(),
        skipped_records = unknown.iter().map(|t| t.records).sum::<usize>(),
        ?names,
        "generated bindings are stale for this game build: DCB record types they don't know \
         were skipped; regenerate the bindings"
    );
}

impl std::fmt::Debug for Datacore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Datacore")
            .field("records", &self.records.len())
            .finish()
    }
}
