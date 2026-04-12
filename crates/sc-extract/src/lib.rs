//! Shared game-data access for the sc-holotable workspace.
//!
//! `sc-extract` sits between raw [svarog] and the domain crates
//! (`sc-weapons`, `sc-contracts`, ...). It owns the machinery that *every*
//! domain needs, so each domain doesn't have to re-derive it.
//!
//! See `docs/sc-extract.md` in the workspace repository for the full
//! design specification and `implementing/sc-extract-*.md` for phase-by-phase
//! implementation notes.
//!
//! # Scope (growing across phases)
//!
//! Phase 2a (this phase):
//! - svarog re-exports + ergonomic `Guid` alias
//! - [`FromInstance`] trait — the bridge between raw svarog `Instance`
//!   values and owned, serialisable Rust types
//! - [`Error`] type + [`Result`] alias for the whole crate
//! - [`LocaleMap`] — parse and serialize `global.ini`
//!
//! Planned for later phases:
//! - Vehicle XML (CryXmlB) parsing
//! - `ReferenceGraph`, `TagTree`, `ManufacturerRegistry`, `DisplayNameCache`
//! - Playable-content filters
//! - `ExtractedData` envelope and snapshot save/load
//! - The `generated/` module (produced by `tools/sc-generator`)
//! - `parse_from_p4k` orchestrator
//!
//! # svarog re-exports
//!
//! sc-extract publicly re-exports `svarog-common` and `svarog-datacore`, and
//! also exposes the most commonly used types directly at the crate root for
//! ergonomic use. Consumers write:
//!
//! ```ignore
//! use sc_extract::{Guid, Value, Instance, DataCoreDatabase};
//! ```
//!
//! for the 90% case, and drop to `sc_extract::svarog_datacore::...` for the
//! rare case where they need something more obscure. sc-extract does **not**
//! redefine any of these types — they all come from svarog.
//!
//! # Relationship to sc-installs
//!
//! `sc-extract` depends on `sc-installs` for the [`parse_from_install`]
//! entry point, which accepts an [`sc_installs::Installation`] and fills
//! in `game_version` / `build_id` from its manifest. The raw
//! [`parse_from_p4k`] entry point still works with a plain `&Path` for
//! callers that don't need install discovery.

mod assets;
mod config;
mod display_names;
mod error;
mod extracted;
mod filters;
mod graph;
mod locale;
mod manufacturers;
mod tags;

/// Machine-generated DataCore schema bindings.
///
/// Re-export of the workspace-internal `sc-extract-generated` crate — see
/// `implementing/sc-generator.md` and `docs/codegen.md` for the design.
/// Every type in this module mirrors a DCB struct or enum and implements
/// [`FromInstance`] for parsing.
///
/// Split into its own crate so cold release builds only pay the LLVM cost
/// of optimizing the ~270k generated lines once, and so it can be built at
/// a lighter `opt-level` (see workspace `Cargo.toml` profile overrides).
pub use sc_extract_generated as generated;

pub use assets::AssetSource;
pub use config::{ExtractConfig, ExtractConfigBuilder};
pub use display_names::{resolve_entity_display_name, DisplayNameCache};
pub use error::{Error, Result};
pub use extracted::ExtractedData;
pub use filters::{is_playable_ship, is_playable_weapon};
pub use sc_extract_generated::{
    Builder, DataPools, Extract, RecordIndex, RecordStore,
};
pub use graph::ReferenceGraph;
pub use locale::{LocKey, LocaleMap};
pub use manufacturers::{Manufacturer, ManufacturerRegistry};
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
///
/// Every record in the DCB has a GUID that can be used to look it up via
/// the reference graph or the record store.
pub use svarog_common::CigGuid as Guid;

/// Ergonomic top-level re-exports from `svarog-datacore`.
///
/// These are the types consumers touch most often. Anything not in this
/// list is still reachable via `sc_extract::svarog_datacore::...`.
pub use svarog_datacore::{
    ArrayElementType, ArrayRef, DataCoreDatabase, DataType, Instance, InstanceRef, Query, Record,
    RecordRef, Value,
};

// ── Top-level orchestration ────────────────────────────────────────────────

use std::path::Path;

/// Open a `Data.p4k` from a discovered [`sc_installs::Installation`], parse
/// the DCB, and build indices per [`ExtractConfig::standard()`].
///
/// This is the recommended entry point. It automatically fills in
/// `game_version` and `build_id` from the installation's manifest.
///
/// ```no_run
/// let install = sc_installs::discover_primary()?;
/// let (data, assets) = sc_extract::parse_from_install(&install)?;
/// println!("parsed {} records from {}", data.records.len(), install.channel);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_from_install(
    install: &sc_installs::Installation,
) -> Result<(ExtractedData, AssetSource)> {
    parse_from_install_with(install, &ExtractConfig::standard())
}

/// Like [`parse_from_install`] but with a custom [`ExtractConfig`].
pub fn parse_from_install_with(
    install: &sc_installs::Installation,
    config: &ExtractConfig,
) -> Result<(ExtractedData, AssetSource)> {
    let version = install.short_version().to_string();
    let build_id = install.manifest.build_id.clone();

    tracing::info!(
        channel = %install.channel,
        version = %version,
        build_id = %build_id,
        "extracting from installation"
    );

    parse_inner(&install.data_p4k(), &version, &build_id, config)
}

/// Open a `Data.p4k` by path and build indices per [`ExtractConfig::standard()`].
///
/// Game version and build ID are set to `"unknown"`. Prefer
/// [`parse_from_install`] when you have an [`sc_installs::Installation`].
pub fn parse_from_p4k(p4k_path: &Path) -> Result<(ExtractedData, AssetSource)> {
    parse_with_config(p4k_path, &ExtractConfig::standard())
}

/// Like [`parse_from_p4k`] but with a custom [`ExtractConfig`].
pub fn parse_with_config(
    p4k_path: &Path,
    config: &ExtractConfig,
) -> Result<(ExtractedData, AssetSource)> {
    parse_inner(p4k_path, "unknown", "unknown", config)
}

/// Same as [`parse_from_p4k`] but drops the [`AssetSource`] before returning.
pub fn parse_snapshot_only(p4k_path: &Path) -> Result<ExtractedData> {
    parse_from_p4k(p4k_path).map(|(data, _assets)| data)
}

/// Shared implementation for all `parse_*` entry points.
fn parse_inner(
    p4k_path: &Path,
    game_version: &str,
    build_id: &str,
    config: &ExtractConfig,
) -> Result<(ExtractedData, AssetSource)> {
    let start = std::time::Instant::now();

    tracing::info!(path = %p4k_path.display(), "opening p4k for extraction");
    let assets = AssetSource::open(p4k_path)?;

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

    let locale = if config.build_locale {
        tracing::info!("parsing global.ini");
        match assets
            .find_and_read(|name| name.to_ascii_lowercase().ends_with("english/global.ini"))?
        {
            Some((_, bytes)) => LocaleMap::parse(&bytes).unwrap_or_else(|e| {
                tracing::warn!(error = %e, "failed to parse global.ini; falling back to empty locale");
                LocaleMap::default()
            }),
            None => {
                tracing::warn!("no english/global.ini found in archive; locale will be empty");
                LocaleMap::default()
            }
        }
    } else {
        LocaleMap::default()
    };

    let display_names = if config.build_display_names {
        tracing::info!("building display name cache");
        DisplayNameCache::from_database(&db, &locale)
    } else {
        DisplayNameCache::new()
    };

    let data = ExtractedData {
        schema_version: ExtractedData::SCHEMA_VERSION,
        game_version: game_version.to_string(),
        build_id: build_id.to_string(),
        extracted_at: current_iso_timestamp(),
        records,
        graph,
        tag_tree,
        manufacturers,
        display_names,
        locale,
    };

    tracing::info!(
        records = data.records.len(),
        game_version = %data.game_version,
        elapsed_ms = start.elapsed().as_millis(),
        "extraction complete"
    );

    Ok((data, assets))
}

/// Best-effort UTC timestamp for `ExtractedData.extracted_at`.
fn current_iso_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{secs}")
}
