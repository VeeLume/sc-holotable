//! `sc-holotable` — the umbrella prelude crate for the workspace.
//!
//! The recommended public dependency. Re-exports the typed surfaces of the
//! workspace crates behind feature flags, so a downstream consumer pins one
//! tag, names no svarog rev, and selects capabilities with features instead of
//! remembering individual sc-extract leaf flags.
//!
//! ```toml
//! sc-holotable = { git = "...", tag = "v0.8.0", features = ["missions", "weapons"] }
//! ```
//!
//! # Layout
//!
//! - Per-crate modules ([`asset`], [`items`], [`tags`], [`manufacturers`],
//!   [`weapons`], [`crafting`], [`missions`], [`install`]) — each a glob
//!   re-export of one workspace crate, gated behind its feature.
//! - [`prelude`] — the common types in one `use`.
//! - With the `foundations` feature: [`Foundations`] + [`build_foundations`]
//!   (build every cooked index in one bundled pass) and [`HolotableSnapshot`]
//!   (a batteries-included processed-snapshot bundle).
//!
//! Only the crates that exist today are wired up; the full design-doc feature
//! map (sc-vehicles, sc-loadouts, …) lands as those crates do.

#[cfg(feature = "installs")]
pub mod install {
    //! Install discovery ([`sc_discovery`]).
    pub use sc_discovery::*;
}

#[cfg(feature = "extract")]
pub mod asset {
    //! Asset + DataCore access ([`sc_extract`]): `AssetSource`, `AssetData`,
    //! `Datacore`, snapshots, `RecordPaths`, the bundled-walk API, and the
    //! svarog escape hatches.
    pub use sc_extract::*;
}

#[cfg(feature = "items")]
pub mod items {
    //! Per-entity item metadata ([`sc_items`]).
    pub use sc_items::*;
}

#[cfg(feature = "tags")]
pub mod tags {
    //! Hierarchical tag tree ([`sc_tags`]).
    pub use sc_tags::*;
}

#[cfg(feature = "manufacturers")]
pub mod manufacturers {
    //! Manufacturer registry ([`sc_manufacturers`]).
    pub use sc_manufacturers::*;
}

#[cfg(feature = "resources")]
pub mod resources {
    //! Resource catalog ([`sc_resources`]) — `ResourceType` records,
    //! refining graph, density, volatility, plus the shared
    //! `CargoQuantity` primitive used by sc-crafting.
    pub use sc_resources::*;
}

#[cfg(feature = "weapons")]
pub mod weapons {
    //! Ship / FPS weapons + missiles ([`sc_weapons`]).
    pub use sc_weapons::*;
}

#[cfg(feature = "crafting")]
pub mod crafting {
    //! Crafting blueprints ([`sc_crafting`]).
    pub use sc_crafting::*;
}

#[cfg(feature = "missions")]
pub mod missions {
    //! Missions / contracts ([`sc_missions`]).
    pub use sc_missions::*;
}

#[cfg(feature = "foundations")]
mod foundations;
#[cfg(feature = "foundations")]
pub use foundations::{Foundations, HOLOTABLE_COOK_VERSION, HolotableSnapshot, build_foundations};

/// The common types in one `use sc_holotable::prelude::*`.
pub mod prelude {
    #[cfg(feature = "extract")]
    pub use sc_extract::{
        AssetConfig, AssetData, AssetSource, Datacore, ExtractSnapshot, Guid, LocaleKey, LocaleMap,
        ProcessedSnapshot, RecordPath, RecordPaths, SnapshotMeta,
    };
    #[cfg(feature = "items")]
    pub use sc_items::{Item, Items};
    #[cfg(feature = "manufacturers")]
    pub use sc_manufacturers::{Manufacturer, Manufacturers};
    #[cfg(feature = "missions")]
    pub use sc_missions::Missions;
    #[cfg(feature = "resources")]
    pub use sc_resources::{CargoQuantity, Density, DensityUnit, Resource, Resources, Volatility};
    #[cfg(feature = "tags")]
    pub use sc_tags::Tags;
    #[cfg(feature = "weapons")]
    pub use sc_weapons::{FpsWeapon, Missile, ShipWeapon, WeaponPools, Weapons};
    #[cfg(feature = "crafting")]
    pub use sc_crafting::{
        Blueprint, Blueprints, Categories, Category, Cost, Duration, GameplayProperties,
        GameplayProperty, GlobalParams, Process, Quality, QualityDistribution,
        QualityDistributionShape, QualityLocationOverride, QualityQuantization, Recipe,
        RecipeCosts, RecipeResult, Research, ResourceCost, Tier,
    };

    #[cfg(feature = "foundations")]
    pub use crate::{Foundations, HolotableSnapshot, build_foundations};
}
