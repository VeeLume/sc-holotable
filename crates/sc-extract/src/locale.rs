//! Parse and serialize Star Citizen `global.ini` localization files.
//!
//! `global.ini` is a flat key=value file shipped inside `Data.p4k` at
//! `Data/Localization/<lang>/global.ini`. The source files are **UTF-16 LE
//! with BOM**. Localization keys are referenced from DCB records with an
//! `@` prefix (e.g. `"@item_NameGATS_Ballistic_S4"`) that must be stripped
//! before lookup.
//!
//! This module provides [`LocaleMap`] for parsing, lookup, mutation, and
//! round-trip serialization, plus a [`LocaleKey`] newtype for type-safe
//! localization references.

use std::collections::HashMap;
use std::path::Path;

use encoding_rs::UTF_16LE;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Re-exported from `sc-extract-generated` so that every generated struct
/// field of DCB type `Locale` uses the same newtype the hand-written
/// lookup API accepts. See the generated crate for the type definition.
pub use sc_extract_generated::LocaleKey;

/// An in-memory representation of a parsed `global.ini` localization file.
///
/// Construct via [`LocaleMap::parse`] or [`LocaleMap::parse_file`], look up
/// values via [`LocaleMap::get`] (raw key) or [`LocaleMap::resolve`]
/// (handles `@` prefix), mutate via [`LocaleMap::set`] /
/// [`LocaleMap::remove`], and serialise via [`LocaleMap::serialize`].
///
/// # Serialisation format
///
/// [`LocaleMap::serialize`] produces **UTF-8 with BOM**. Star Citizen
/// accepts both UTF-16 LE (the format the game ships) and UTF-8 with BOM
/// (what sc-langpatch writes) for override files; UTF-8 is easier to
/// inspect and patch. The output is deterministic (keys sorted
/// alphabetically) so snapshots diff cleanly.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LocaleMap {
    entries: HashMap<String, String>,
}

impl LocaleMap {
    /// Create an empty `LocaleMap`.
    pub fn new() -> Self {
        Self::default()
    }

    // ── Parsing ─────────────────────────────────────────────────────────

    /// Parse a `global.ini` from raw bytes.
    ///
    /// Handles the UTF-16 LE BOM (optional — if present, it's stripped
    /// before decoding). Lines are split on `\n` / `\r\n`. Blank lines and
    /// comment lines (beginning with `#` or `;`) are skipped. Each
    /// remaining line must contain exactly one `=` separator; the first
    /// `=` is the split point so values may contain further `=` characters.
    ///
    /// Returns an error if the bytes cannot be decoded as UTF-16 LE, or
    /// if any non-blank, non-comment line lacks an `=` separator.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        // Strip the BOM if present (FFFE in little-endian).
        let data = if bytes.starts_with(&[0xFF, 0xFE]) {
            &bytes[2..]
        } else {
            bytes
        };

        let (text, _encoding, had_errors) = UTF_16LE.decode(data);
        if had_errors {
            return Err(Error::LocaleDecodeFailed);
        }

        let mut entries = HashMap::new();
        for (index, raw_line) in text.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(Error::LocaleMalformedLine {
                    line_number: index + 1,
                    line: raw_line.to_string(),
                });
            };
            entries.insert(key.trim().to_string(), value.to_string());
        }

        Ok(Self { entries })
    }

    /// Read a `global.ini` from the given path and parse it.
    ///
    /// Accepts either the UTF-16 LE files shipped in `Data.p4k` or the
    /// UTF-8 BOM files `sc-langpatch` writes as overrides. The caller is
    /// expected to know which encoding their file is in; [`LocaleMap::parse`]
    /// handles the UTF-16 LE case, and UTF-8 BOM files can go through
    /// [`LocaleMap::parse_utf8_bom`] instead.
    pub fn parse_file(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path).map_err(|source| Error::LocaleIoError {
            path: path.to_path_buf(),
            source,
        })?;
        Self::parse(&bytes)
    }

    /// Parse a `global.ini` from raw bytes that use **UTF-8 with BOM**
    /// rather than the game's native UTF-16 LE. This is the encoding
    /// `sc-langpatch` writes its override files in.
    pub fn parse_utf8_bom(bytes: &[u8]) -> Result<Self> {
        // Strip UTF-8 BOM if present (EF BB BF).
        let data = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            &bytes[3..]
        } else {
            bytes
        };

        let text = std::str::from_utf8(data).map_err(|_| Error::LocaleDecodeFailed)?;

        let mut entries = HashMap::new();
        for (index, raw_line) in text.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(Error::LocaleMalformedLine {
                    line_number: index + 1,
                    line: raw_line.to_string(),
                });
            };
            entries.insert(key.trim().to_string(), value.to_string());
        }

        Ok(Self { entries })
    }

    // ── Lookup ──────────────────────────────────────────────────────────

    /// Look up a raw localization key (without leading `@`).
    ///
    /// Accepts either a `&str` or a [`LocaleKey`] reference — anything that
    /// implements [`AsRef<str>`]. Pass a `LocaleKey` through
    /// [`LocaleKey::stripped`] first if it might contain the `@` prefix, or
    /// use [`LocaleMap::resolve`] which strips it for you.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        self.entries.get(key.as_ref()).map(String::as_str)
    }

    /// Resolve a localization key that may have an `@` prefix.
    ///
    /// `"@item_NameGATS_Ballistic_S4"` and `"item_NameGATS_Ballistic_S4"`
    /// both look up the same entry. Accepts either `&str` or `&LocaleKey`.
    pub fn resolve(&self, loc_key: impl AsRef<str>) -> Option<&str> {
        let s = loc_key.as_ref();
        self.get(s.strip_prefix('@').unwrap_or(s))
    }

    /// True if this map contains the given key (raw, no `@` handling).
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.entries.contains_key(key.as_ref())
    }

    /// Number of entries in the map.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True if the map has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate over every (key, value) pair. Order is unspecified.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.entries.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    // ── Mutation ────────────────────────────────────────────────────────

    /// Insert or replace an entry. Returns the previous value if any.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> Option<String> {
        self.entries.insert(key.into(), value.into())
    }

    /// Remove an entry. Returns the removed value if any.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.entries.remove(key)
    }

    // ── Serialization ───────────────────────────────────────────────────

    /// Serialize to UTF-8 bytes with a leading BOM.
    ///
    /// This is the format `sc-langpatch` writes its override files in.
    /// Output is deterministic: keys are sorted alphabetically, line
    /// endings are `\r\n`, and a UTF-8 BOM (`EF BB BF`) is prepended.
    pub fn serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        // UTF-8 BOM
        output.extend_from_slice(&[0xEF, 0xBB, 0xBF]);

        let mut keys: Vec<&String> = self.entries.keys().collect();
        keys.sort();

        for key in keys {
            let value = &self.entries[key];
            output.extend_from_slice(key.as_bytes());
            output.push(b'=');
            output.extend_from_slice(value.as_bytes());
            output.extend_from_slice(b"\r\n");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a UTF-16 LE byte buffer with BOM from a string, suitable for
    /// passing to `LocaleMap::parse`. Used by the test suite to avoid
    /// committing binary fixtures.
    fn utf16_le_with_bom(text: &str) -> Vec<u8> {
        let mut bytes = vec![0xFF, 0xFE];
        for codepoint in text.encode_utf16() {
            bytes.extend_from_slice(&codepoint.to_le_bytes());
        }
        bytes
    }

    fn utf8_with_bom(text: &str) -> Vec<u8> {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(text.as_bytes());
        bytes
    }

    // ── LocaleKey tests ────────────────────────────────────────────────────

    #[test]
    fn lockey_stripped_removes_at_prefix() {
        let k = LocaleKey::new("@item_NameGATS");
        assert_eq!(k.stripped(), "item_NameGATS");
        assert_eq!(k.as_str(), "@item_NameGATS");
    }

    #[test]
    fn lockey_stripped_without_prefix() {
        let k = LocaleKey::new("item_NameGATS");
        assert_eq!(k.stripped(), "item_NameGATS");
    }

    #[test]
    fn lockey_from_str() {
        let k: LocaleKey = "foo".into();
        assert_eq!(k.as_str(), "foo");
    }

    // ── Parse tests (UTF-16 LE) ─────────────────────────────────────────

    #[test]
    fn parse_simple_utf16_le() {
        let ini = "item_NameGATS=GATS Ballistic\r\nitem_NameKLWE=KLWE Laser\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("item_NameGATS"), Some("GATS Ballistic"));
        assert_eq!(map.get("item_NameKLWE"), Some("KLWE Laser"));
    }

    #[test]
    fn parse_without_bom_still_works() {
        let ini = "key=value\r\n";
        let mut bytes = Vec::new();
        for codepoint in ini.encode_utf16() {
            bytes.extend_from_slice(&codepoint.to_le_bytes());
        }
        let map = LocaleMap::parse(&bytes).unwrap();
        assert_eq!(map.get("key"), Some("value"));
    }

    #[test]
    fn parse_skips_blank_and_comment_lines() {
        let ini = "# a comment\r\n\r\n; another comment\r\nkey=value\r\n\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key"), Some("value"));
    }

    #[test]
    fn parse_handles_values_containing_equals() {
        // Values with '=' in them should survive — split_once only takes
        // the first '=' as the separator.
        let ini = "formula=a=b+c\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        assert_eq!(map.get("formula"), Some("a=b+c"));
    }

    #[test]
    fn parse_trims_key_whitespace() {
        let ini = "  key  =value\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        assert_eq!(map.get("key"), Some("value"));
    }

    #[test]
    fn parse_empty_input_is_empty_map() {
        let map = LocaleMap::parse(&utf16_le_with_bom("")).unwrap();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn parse_malformed_line_errors() {
        let ini = "good_key=good_value\r\nthis line has no equals\r\n";
        let err = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap_err();
        match err {
            Error::LocaleMalformedLine { line_number, line } => {
                assert_eq!(line_number, 2);
                assert_eq!(line, "this line has no equals");
            }
            other => panic!("expected LocaleMalformedLine, got {other:?}"),
        }
    }

    // ── Parse tests (UTF-8 with BOM) ────────────────────────────────────

    #[test]
    fn parse_utf8_bom_simple() {
        let ini = "key1=value1\r\nkey2=value2\r\n";
        let map = LocaleMap::parse_utf8_bom(&utf8_with_bom(ini)).unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("key1"), Some("value1"));
        assert_eq!(map.get("key2"), Some("value2"));
    }

    #[test]
    fn parse_utf8_bom_without_bom() {
        // parse_utf8_bom should still work on UTF-8 data without a BOM,
        // since the BOM is optional in UTF-8.
        let map = LocaleMap::parse_utf8_bom(b"key=value\r\n").unwrap();
        assert_eq!(map.get("key"), Some("value"));
    }

    // ── Lookup tests ────────────────────────────────────────────────────

    #[test]
    fn resolve_handles_at_prefix() {
        let ini = "item_Foo=Foo Display\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        assert_eq!(map.resolve("@item_Foo"), Some("Foo Display"));
        assert_eq!(map.resolve("item_Foo"), Some("Foo Display"));
    }

    #[test]
    fn resolve_accepts_lockey_newtype() {
        let ini = "item_Foo=Foo Display\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        let key = LocaleKey::new("@item_Foo");
        assert_eq!(map.resolve(&key), Some("Foo Display"));
        assert_eq!(map.get(key.stripped()), Some("Foo Display"));
    }

    #[test]
    fn resolve_missing_key_is_none() {
        let map = LocaleMap::new();
        assert_eq!(map.resolve("@nothing"), None);
    }

    #[test]
    fn contains_key_works() {
        let ini = "a=1\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        assert!(map.contains_key("a"));
        assert!(!map.contains_key("b"));
    }

    #[test]
    fn iter_yields_all_entries() {
        let ini = "a=1\r\nb=2\r\n";
        let map = LocaleMap::parse(&utf16_le_with_bom(ini)).unwrap();
        let mut pairs: Vec<(&str, &str)> = map.iter().collect();
        pairs.sort();
        assert_eq!(pairs, vec![("a", "1"), ("b", "2")]);
    }

    // ── Mutation tests ──────────────────────────────────────────────────

    #[test]
    fn set_inserts_new_and_replaces_existing() {
        let mut map = LocaleMap::new();
        assert_eq!(map.set("k", "v1"), None);
        assert_eq!(map.get("k"), Some("v1"));
        assert_eq!(map.set("k", "v2"), Some("v1".to_string()));
        assert_eq!(map.get("k"), Some("v2"));
    }

    #[test]
    fn remove_returns_previous_value() {
        let mut map = LocaleMap::new();
        map.set("k", "v");
        assert_eq!(map.remove("k"), Some("v".to_string()));
        assert_eq!(map.remove("k"), None);
    }

    // ── Serialization tests ─────────────────────────────────────────────

    #[test]
    fn serialize_produces_utf8_bom() {
        let mut map = LocaleMap::new();
        map.set("a", "1");
        map.set("b", "2");
        let bytes = map.serialize();
        assert_eq!(&bytes[..3], &[0xEF, 0xBB, 0xBF], "missing UTF-8 BOM");
    }

    #[test]
    fn serialize_sorts_keys_for_determinism() {
        let mut map = LocaleMap::new();
        map.set("z", "26");
        map.set("a", "1");
        map.set("m", "13");
        let bytes = map.serialize();
        let text = std::str::from_utf8(&bytes[3..]).unwrap();
        assert_eq!(text, "a=1\r\nm=13\r\nz=26\r\n");
    }

    #[test]
    fn serialize_round_trip_via_utf8_bom() {
        let mut original = LocaleMap::new();
        original.set("item_A", "Alpha");
        original.set("item_B", "Beta");
        original.set("item_C", "Charlie");

        let bytes = original.serialize();
        let parsed = LocaleMap::parse_utf8_bom(&bytes).unwrap();

        assert_eq!(parsed, original);
    }

    #[test]
    fn serialize_preserves_values_with_equals() {
        let mut map = LocaleMap::new();
        map.set("formula", "a=b+c");
        let bytes = map.serialize();
        let parsed = LocaleMap::parse_utf8_bom(&bytes).unwrap();
        assert_eq!(parsed.get("formula"), Some("a=b+c"));
    }

    // ── File I/O test ───────────────────────────────────────────────────

    #[test]
    fn parse_file_reads_from_disk() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("global.ini");
        let ini = "item_Foo=bar\r\n";
        std::fs::write(&path, utf16_le_with_bom(ini)).unwrap();

        let map = LocaleMap::parse_file(&path).unwrap();
        assert_eq!(map.get("item_Foo"), Some("bar"));
    }

    #[test]
    fn parse_file_missing_file_errors() {
        let temp = tempfile::tempdir().unwrap();
        let missing = temp.path().join("does_not_exist.ini");
        let err = LocaleMap::parse_file(&missing).unwrap_err();
        assert!(matches!(err, Error::LocaleIoError { .. }));
    }
}
