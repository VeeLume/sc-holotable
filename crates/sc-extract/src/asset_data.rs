//! Serializable asset-sourced data — currently just the parsed locale map.
//!
//! [`AssetData`] is the asset-side counterpart to [`crate::DatacoreSnapshot`]:
//! it bundles everything that comes from non-DCB files inside the P4K into
//! one owned, serde-friendly struct. Today that's just a [`LocaleMap`]
//! parsed from `global.ini`, but the shape is designed so future cached
//! asset-derived values (keybindings, vehicle XML summaries, etc.) can be
//! added without changing the surrounding API.
//!
//! Built by [`AssetData::extract`] from a live [`AssetSource`]. The live
//! archive handle is kept separate so callers who only need locale can
//! snapshot this struct and drop the archive.

use serde::{Deserialize, Serialize};

use crate::assets::AssetSource;
use crate::error::Result;
use crate::locale::LocaleMap;

/// Controls which asset-sourced values [`AssetData::extract`] builds.
#[derive(Debug, Clone)]
pub struct AssetConfig {
    /// Parse `global.ini` from the archive into a [`LocaleMap`].
    pub build_locale: bool,
}

impl AssetConfig {
    /// Everything: locale on. The recommended default for consumers that
    /// want display names to be populated in a downstream [`crate::Datacore`].
    pub fn standard() -> Self {
        Self { build_locale: true }
    }

    /// Nothing — produces an empty [`AssetData`].
    pub fn minimal() -> Self {
        Self {
            build_locale: false,
        }
    }
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self::standard()
    }
}

/// Owned, serializable bundle of asset-sourced values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetData {
    /// Parsed `global.ini` localization table. Empty if
    /// [`AssetConfig::build_locale`] was false or the file was missing.
    #[serde(default)]
    pub locale: LocaleMap,
}

impl AssetData {
    /// Read and parse every asset file enabled by `config` from the
    /// archive.
    pub fn extract(assets: &AssetSource, config: &AssetConfig) -> Result<Self> {
        let locale: LocaleMap = if config.build_locale {
            tracing::info!("parsing global.ini");
            match assets
                .find_and_read(|name| name.to_ascii_lowercase().ends_with("english\\global.ini"))?
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

        Ok(Self { locale })
    }
}
