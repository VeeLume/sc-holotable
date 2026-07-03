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

#[cfg(feature = "locations")]
pub mod locations {
    //! Universe locations ([`sc_locations`]) — typed `StarMapObject` surface
    //! with class-CRC resolution and hierarchy.
    pub use sc_locations::*;
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

#[cfg(feature = "fps-weapons")]
pub mod fps_weapons {
    //! FPS-weapon base-stat sheet ([`sc_items_fps_weapons`]) — the T1
    //! per-itemtype data source for crafting product stats.
    pub use sc_items_fps_weapons::*;
}

#[cfg(feature = "armor")]
pub mod armor {
    //! Character-armor base-stat sheet ([`sc_items_armor`]) — temperature /
    //! radiation resistance + per-type damage resistance.
    pub use sc_items_armor::*;
}

#[cfg(feature = "ship-components")]
pub mod ship_components {
    //! Ship-component base-stat sheet ([`sc_items_ship_components`]) —
    //! integrity, quantum drive, shield, cooler / power-plant generation.
    pub use sc_items_ship_components::*;
}

#[cfg(feature = "ship-weapons")]
pub mod ship_weapons {
    //! Ship-weapon base-stat sheet ([`sc_items_ship_weapons`]) — integrity +
    //! gun per-shot damage / mining-laser beam DPS.
    pub use sc_items_ship_weapons::*;
}

#[cfg(feature = "crafting")]
pub mod crafting {
    //! Crafting blueprints ([`sc_crafting`]).
    pub use sc_crafting::*;
}

#[cfg(feature = "gathering")]
pub mod gathering {
    //! Resource gathering ([`sc_gathering`]) — mining / salvage / plants
    //! providers and the `StarMapObject` ↔ provider location join.
    pub use sc_gathering::*;
}

#[cfg(feature = "missions")]
pub mod missions {
    //! Missions / contracts ([`sc_missions`]).
    //!
    //! The `missions-payout` umbrella feature additionally enables
    //! sc-missions' `payout` feature — `UecCurve` + `Mission::estimate_uec`,
    //! backed by the `gamemode` sc-extract pools (GameMode.SC_Default's
    //! uecCurve) instead of hardcoded curve constants.
    pub use sc_missions::*;
}

#[cfg(feature = "foundations")]
mod foundations;
#[cfg(feature = "foundations")]
pub use foundations::{Foundations, HOLOTABLE_COOK_VERSION, HolotableSnapshot, build_foundations};

/// The common types in one `use sc_holotable::prelude::*`.
pub mod prelude {
    #[cfg(feature = "crafting")]
    pub use sc_crafting::{
        Blueprint, Blueprints, Categories, Category, CompositionInclusion, Cost, CostContext,
        Duration, GameplayProperties, GameplayProperty, GameplayPropertyModifier, GameplayStat,
        GlobalParams, ItemCost, ModifierValue, Process, ProductStat, ProductStatSource, Recipe,
        RecipeCosts, RecipeResult, Research, ResourceCost, SlotName, Tier, ValueRange,
    };
    #[cfg(feature = "extract")]
    pub use sc_extract::{
        AssetConfig, AssetData, AssetSource, CrcIndex, Datacore, ExtractSnapshot, Guid, LocaleKey,
        LocaleMap, ProcessedSnapshot, RecordCollection, RecordPath, RecordPaths, SnapshotMeta,
        class_crc,
    };
    #[cfg(feature = "gathering")]
    pub use sc_gathering::{
        Cluster, Deposit, GatherableElement, GatheringMode, Provider, ProviderGroup,
        ProviderLocations, Providers,
    };
    #[cfg(feature = "items")]
    pub use sc_items::{Item, Items};
    #[cfg(feature = "armor")]
    pub use sc_items_armor::{Armor, ArmorKind, ArmorStats, DamageResistance, ResistanceEntry};
    #[cfg(feature = "fps-weapons")]
    pub use sc_items_fps_weapons::{Damage, FpsWeaponStats, FpsWeapons};
    #[cfg(feature = "ship-components")]
    pub use sc_items_ship_components::{ShipComponentKind, ShipComponentStats, ShipComponents};
    #[cfg(feature = "ship-weapons")]
    pub use sc_items_ship_weapons::{ShipWeaponKind, ShipWeaponStats, ShipWeapons};
    #[cfg(feature = "locations")]
    pub use sc_locations::{
        Location, LocationKind, Locations, ObjectContainers, Place, Placement, PlacementId,
        Universe,
    };
    #[cfg(feature = "manufacturers")]
    pub use sc_manufacturers::{Manufacturer, Manufacturers};
    #[cfg(feature = "missions")]
    pub use sc_missions::Missions;
    #[cfg(feature = "resources")]
    pub use sc_resources::{
        CargoQuantity, Density, DensityUnit, Quality, QualityDistribution,
        QualityDistributionShape, QualityLocationOverride, QualityQuantization, Resource,
        Resources, Volatility,
    };
    #[cfg(feature = "tags")]
    pub use sc_tags::Tags;
    #[cfg(feature = "weapons")]
    pub use sc_weapons::{FpsWeapon, Missile, ShipWeapon, WeaponPools, Weapons};

    #[cfg(feature = "foundations")]
    pub use crate::{Foundations, HolotableSnapshot, build_foundations};
}
