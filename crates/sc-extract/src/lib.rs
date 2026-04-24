//! Shared game-data access for the sc-holotable workspace.
//!
//! `sc-extract` sits between raw [svarog] and the domain crates. It owns
//! the machinery that *every* domain needs, so each domain doesn't have
//! to re-derive it.
//!
//! See `docs/sc-extract.md` for the full design spec and
//! `implementing/sc-extract-*.md` for phase-by-phase notes.
//!
//! # API shape
//!
//! The crate exposes three staged building blocks that map to the four
//! consumer patterns:
//!
//! ```no_run
//! // 1. install only — use sc-installs, don't touch sc-extract
//! let install = sc_installs::discover_primary()?;
//!
//! // 2. install + assets — open the archive, read files directly
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let bytes = assets.read("data/some/file.xml")?;
//!
//! // 3. install + assets + datacore — full pipeline
//! let asset_data = sc_extract::AssetData::extract(
//!     &assets,
//!     &sc_extract::AssetConfig::standard(),
//! )?;
//! let datacore = sc_extract::Datacore::parse(
//!     &assets,
//!     &asset_data,
//!     &sc_extract::DatacoreConfig::standard(),
//! )?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! [`Datacore`] owns the live [`DataCoreDatabase`], so raw svarog queries
//! remain available via [`Datacore::db`] after high-level parsing.
//!
//! Persist a parse run with [`ExtractSnapshot`]: call
//! [`ExtractSnapshot::capture`] against a live [`AssetSource`] to archive
//! the raw `game2.dcb` + `global.ini` bytes under descriptive
//! [`SnapshotMeta`], then [`ExtractSnapshot::save`] to write the zstd-
//! compressed file. Load back with [`ExtractSnapshot::load`] (cheap, no
//! DCB parse) and [`ExtractSnapshot::hydrate`] (~15–25s re-parse) to
//! rebuild a live [`Datacore`] + [`AssetData`]. Historical snapshots are
//! forward/backward compatible across generator regenerations for the
//! same DCB, because the parse-time binding is deferred to load time.
//!
//! # svarog re-exports
//!
//! sc-extract publicly re-exports `svarog-common`, `svarog-datacore`, and
//! `svarog-p4k` in full as escape hatches, and cherry-picks the most
//! commonly used types at the crate root for ergonomic use:
//!
//! ```ignore
//! use sc_extract::{Guid, Value, Instance, DataCoreDatabase};
//! ```

mod asset_data;
mod assets;
mod config;
mod datacore;
mod display_names;
mod error;
mod filters;
mod graph;
mod locale;
mod manufacturers;
mod snapshot;
mod tags;

/// Machine-generated DataCore schema bindings — workspace-internal escape
/// hatch. Every type in this module mirrors a DCB struct or enum and
/// implements [`Extract`] for parsing from svarog `Instance` values.
pub use sc_extract_generated as generated;

pub use asset_data::{AssetConfig, AssetData};
pub use assets::AssetSource;
pub use config::{DatacoreConfig, DatacoreConfigBuilder};
pub use datacore::{Datacore, DatacoreSnapshot};
pub use display_names::{DisplayNameCache, resolve_entity_display_name};
pub use error::{Error, Result};
pub use filters::{is_playable_ship, is_playable_weapon};
pub use graph::ReferenceGraph;
pub use locale::{LocaleKey, LocaleMap};
pub use manufacturers::{Manufacturer, ManufacturerRegistry};
pub use sc_extract_generated::{Builder, DataPools, Extract, RecordIndex, RecordStore};
pub use snapshot::{ExtractSnapshot, SnapshotCaptureConfig, SnapshotMeta};
pub use tags::{TagNode, TagTree};

// ── svarog re-exports ──────────────────────────────────────────────────────

/// Access to the full [`svarog-common`] namespace (escape hatch).
pub use svarog_common;

/// Access to the full [`svarog-datacore`] namespace (escape hatch).
pub use svarog_datacore;

/// Access to the full [`svarog-p4k`] namespace (escape hatch). Most consumers
/// should go through [`AssetSource`] instead.
pub use svarog_p4k;

/// Star Citizen's custom 16-byte GUID type (re-exported from `svarog-common`).
pub use svarog_common::CigGuid as Guid;

/// Ergonomic top-level re-exports from `svarog-datacore`. Anything not in
/// this list is still reachable via `sc_extract::svarog_datacore::...`.
pub use svarog_datacore::{
    ArrayElementType, ArrayRef, DataCoreDatabase, DataType, Instance, InstanceRef, Query, Record,
    RecordRef, Value,
};

// ── Helpers ────────────────────────────────────────────────────────────────

/// Current UTC timestamp in RFC 3339 / ISO 8601 format for
/// [`SnapshotMeta::extracted_at`].
pub fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Convenience: build a [`SnapshotMeta`] from a discovered installation and
/// the current time.
pub fn snapshot_meta_from_install(install: &sc_installs::Installation) -> SnapshotMeta {
    SnapshotMeta {
        schema_version: ExtractSnapshot::SCHEMA_VERSION,
        game_version: install.short_version().to_string(),
        build_id: install.manifest.build_id.clone(),
        extracted_at: current_timestamp(),
    }
}
