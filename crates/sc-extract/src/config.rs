//! Extraction pipeline configuration.
//!
//! [`DatacoreConfig`] controls which DCB-derived indices the datacore
//! pipeline builds. The reference graph, tag tree, manufacturer registry,
//! and display name cache are all independently toggleable.
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

    /// Build the DisplayNameCache? Walks "EntityClassDefinition" records
    /// and resolves names against the [`crate::AssetData::locale`] passed
    /// into [`crate::Datacore::parse`] — fast (~80ms), but returns 0
    /// results if the locale is empty.
    pub build_display_names: bool,
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
            build_display_names: true,
        }
    }

    /// Recommended default: everything except the reference graph.
    ///
    /// The graph is the most expensive index (~7s, ~15 MB) and most
    /// consumers don't need it. Tags, manufacturers, and display names
    /// are cheap and broadly useful.
    pub fn standard() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: true,
            build_manufacturers: true,
            build_display_names: true,
        }
    }

    /// Minimal: only the record store, no secondary indices.
    pub fn minimal() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: false,
            build_manufacturers: false,
            build_display_names: false,
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
    build_display_names: bool,
}

impl DatacoreConfigBuilder {
    fn new() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: false,
            build_manufacturers: false,
            build_display_names: false,
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

    /// Enable or disable the display name cache.
    pub fn display_names(mut self, yes: bool) -> Self {
        self.build_display_names = yes;
        self
    }

    /// Build the config.
    pub fn build(self) -> DatacoreConfig {
        DatacoreConfig {
            build_graph: self.build_graph,
            build_tag_tree: self.build_tag_tree,
            build_manufacturers: self.build_manufacturers,
            build_display_names: self.build_display_names,
        }
    }
}
