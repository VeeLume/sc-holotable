//! Typed wrapper for Star Citizen localization key references.
//!
//! DCB records reference localized strings by string ID (e.g.
//! `"@item_NameGATS_Ballistic_S4"`). In the wire format these are
//! [`DataType::Locale`](svarog_datacore::DataType::Locale) fields — svarog
//! distinguishes them from plain `String`, but the generator previously
//! erased the distinction. [`LocaleKey`] is the type-safe newtype that the
//! generator emits for every Locale field so callers can say "this is a
//! localization reference" at compile time.
//!
//! The actual lookup table lives in `sc_extract::LocaleMap`, which
//! accepts `&LocaleKey` via its `AsRef<str>` bound. This crate can't depend
//! on `sc-extract` (that would be a dep cycle — `sc-extract` re-exports
//! everything here), so the newtype itself lives at this layer and
//! `sc-extract` re-exports it from its `locale` module.

use serde::{Deserialize, Serialize};

/// A localization key referenced from DCB records.
///
/// In the DCB, localization references appear as strings like
/// `"@item_NameGATS_Ballistic_S4"`. The leading `@` is a convention
/// marker used by the game engine but is **not** part of the actual key
/// in `global.ini`. `LocaleMap::resolve` handles the `@` prefix
/// transparently; `LocaleMap::get` requires the caller to pass the key
/// without the prefix — use [`LocaleKey::stripped`] for that.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocaleKey(pub String);

impl LocaleKey {
    /// Create a new [`LocaleKey`] from anything that can be turned into a `String`.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// The raw key string, including any leading `@` prefix.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The key with the leading `@` stripped, if present.
    /// Suitable for passing to `LocaleMap::get`.
    pub fn stripped(&self) -> &str {
        self.0.strip_prefix('@').unwrap_or(&self.0)
    }

    /// True if the underlying string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for LocaleKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for LocaleKey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for LocaleKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
