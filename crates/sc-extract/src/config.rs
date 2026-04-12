//! Extraction pipeline configuration.
//!
//! [`ExtractConfig`] controls which indices the extraction pipeline builds.
//! The reference graph, tag tree, manufacturer registry, display name cache,
//! and locale parsing are all independently toggleable.
//!
//! # Default
//!
//! [`ExtractConfig::standard()`] is the recommended default — it builds
//! everything except the reference graph (which is expensive and unused by
//! most consumers). Use [`ExtractConfig::all()`] if you need the graph.

/// Controls which indices the extraction pipeline builds.
///
/// Use the builder pattern for custom configurations, or one of the
/// predefined constructors for common setups.
///
/// ```
/// use sc_extract::ExtractConfig;
///
/// // Recommended default — everything except the expensive graph:
/// let config = ExtractConfig::standard();
///
/// // Full extraction including graph:
/// let config = ExtractConfig::all();
///
/// // Custom:
/// let config = ExtractConfig::builder()
///     .graph(true)
///     .locale(false)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ExtractConfig {
    /// Build the ReferenceGraph? Walks all records iteratively — ~7s on
    /// real data, adds ~15 MB to snapshot size.
    pub(crate) build_graph: bool,

    /// Build the TagTree? Walks only "Tag" records — fast (~20ms).
    pub(crate) build_tag_tree: bool,

    /// Build the ManufacturerRegistry? Walks only "SCItemManufacturer"
    /// records — fast (~5ms).
    pub(crate) build_manufacturers: bool,

    /// Build the DisplayNameCache? Walks "EntityClassDefinition" records
    /// and resolves names through the locale map — fast (~80ms), but
    /// returns 0 results if locale is empty.
    pub(crate) build_display_names: bool,

    /// Parse global.ini from the archive into a LocaleMap?
    pub(crate) build_locale: bool,
}

impl ExtractConfig {
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
            build_locale: true,
        }
    }

    /// Recommended default: everything except the reference graph.
    ///
    /// The graph is the most expensive index (~7s, ~15 MB) and most
    /// consumers don't need it. Tags, manufacturers, display names, and
    /// locale are cheap and broadly useful.
    pub fn standard() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: true,
            build_manufacturers: true,
            build_display_names: true,
            build_locale: true,
        }
    }

    /// Minimal: only the record store, no secondary indices.
    ///
    /// Fastest extraction — skips all index building. Useful when you
    /// only need typed record access via the RecordStore.
    pub fn minimal() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: false,
            build_manufacturers: false,
            build_display_names: false,
            build_locale: false,
        }
    }

    /// Returns a builder for custom configurations.
    pub fn builder() -> ExtractConfigBuilder {
        ExtractConfigBuilder::new()
    }
}

/// Builder for [`ExtractConfig`].
///
/// Starts with everything disabled. Enable what you need.
#[derive(Debug, Clone)]
pub struct ExtractConfigBuilder {
    build_graph: bool,
    build_tag_tree: bool,
    build_manufacturers: bool,
    build_display_names: bool,
    build_locale: bool,
}

impl ExtractConfigBuilder {
    fn new() -> Self {
        Self {
            build_graph: false,
            build_tag_tree: false,
            build_manufacturers: false,
            build_display_names: false,
            build_locale: false,
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

    /// Enable or disable locale parsing (global.ini).
    pub fn locale(mut self, yes: bool) -> Self {
        self.build_locale = yes;
        self
    }

    /// Build the config.
    pub fn build(self) -> ExtractConfig {
        ExtractConfig {
            build_graph: self.build_graph,
            build_tag_tree: self.build_tag_tree,
            build_manufacturers: self.build_manufacturers,
            build_display_names: self.build_display_names,
            build_locale: self.build_locale,
        }
    }
}
