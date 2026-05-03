//! Extraction pipeline configuration.
//!
//! [`DatacoreConfig`] controls which DCB-derived indices the datacore
//! pipeline builds. The reference graph, tag tree, manufacturer registry,
//! and localized-item cache are all independently toggleable.
//!
//! Locale parsing lives on [`crate::AssetConfig`] — it's an asset-sourced
//! value, not DCB-derived.
//!
//! # Default
//!
//! [`DatacoreConfig::standard()`] is the recommended default — it builds
//! everything except the reference graph (which is expensive and unused by
//! most consumers). Use [`DatacoreConfig::all()`] if you need the graph.

/// Controls which DCB-derived indices [`crate::Datacore::parse`] builds.
///
/// Use one of the predefined constructors or the builder pattern.
///
/// ```
/// use sc_extract::DatacoreConfig;
///
/// // Recommended default — everything except the expensive graph:
/// let config = DatacoreConfig::standard();
///
/// // Full: includes the reference graph:
/// let config = DatacoreConfig::all();
///
/// // Custom:
/// let config = DatacoreConfig::builder()
///     .graph(true)
///     .manufacturers(false)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct DatacoreConfig {
    /// Build the ReferenceGraph? Walks all records iteratively — ~7s on
    /// real data, adds ~15 MB to snapshot size.
    pub build_graph: bool,

    /// Build the TagTree? Walks only "Tag" records — fast (~20ms).
    pub build_tag_tree: bool,

    /// Build the ManufacturerRegistry? Walks only "SCItemManufacturer"
    /// records — fast (~5ms).
    pub build_manufacturers: bool,

    /// Build the LocalizedItemCache? Walks "EntityClassDefinition"
    /// records and stores the `Localization.{Name, ShortName,
    /// Description}` keys per entity GUID — locale-independent (no
    /// `LocaleMap` consulted at parse time). Fast (~80ms).
    pub build_localized_items: bool,
}

impl DatacoreConfig {
    /// Build everything — all indices including the reference graph.
    ///
    /// Use this when you need the full graph for dependency analysis.
    /// Be aware the graph adds ~7s parse time and ~15 MB snapshot size.
    pub fn all() -> Self {
        Self {
            build_graph: true,
            build_tag_tree: true,
            build_manufacturers: true,
            build_localized_items: true,
        }
    }

    /// Recommended default: everything except the reference graph.
    ///
    /// The graph is the most expensive index (~7s, ~15 MB) and most
    /// consumers don't need it. Tags, manufacturers, and localized
    /// items are cheap and broadly useful.
    pub fn standard() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: true,
            build_manufacturers: true,
            build_localized_items: true,
        }
    }

    /// Minimal: only the record store, no secondary indices.
    pub fn minimal() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: false,
            build_manufacturers: false,
            build_localized_items: false,
        }
    }

    /// Returns a builder for custom configurations.
    pub fn builder() -> DatacoreConfigBuilder {
        DatacoreConfigBuilder::new()
    }
}

impl Default for DatacoreConfig {
    fn default() -> Self {
        Self::standard()
    }
}

/// Builder for [`DatacoreConfig`].
///
/// Starts with everything disabled. Enable what you need.
#[derive(Debug, Clone)]
pub struct DatacoreConfigBuilder {
    build_graph: bool,
    build_tag_tree: bool,
    build_manufacturers: bool,
    build_localized_items: bool,
}

impl DatacoreConfigBuilder {
    fn new() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: false,
            build_manufacturers: false,
            build_localized_items: false,
        }
    }

    /// Enable or disable the reference graph.
    pub fn graph(mut self, yes: bool) -> Self {
        self.build_graph = yes;
        self
    }

    /// Enable or disable the tag tree.
    pub fn tag_tree(mut self, yes: bool) -> Self {
        self.build_tag_tree = yes;
        self
    }

    /// Enable or disable the manufacturer registry.
    pub fn manufacturers(mut self, yes: bool) -> Self {
        self.build_manufacturers = yes;
        self
    }

    /// Enable or disable the localized-item cache.
    pub fn localized_items(mut self, yes: bool) -> Self {
        self.build_localized_items = yes;
        self
    }

    /// Build the config.
    pub fn build(self) -> DatacoreConfig {
        DatacoreConfig {
            build_graph: self.build_graph,
            build_tag_tree: self.build_tag_tree,
            build_manufacturers: self.build_manufacturers,
            build_localized_items: self.build_localized_items,
        }
    }
}
