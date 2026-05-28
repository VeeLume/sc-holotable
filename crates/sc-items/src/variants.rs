//! Entity-name variant heuristics.
//!
//! CIG names item/ship entities with regular variant suffixes on top of a
//! base record name. Census over live DCB (SC 4.8) found the prevalent
//! families: cosmetic `_<color/material><NN>` (478 names, e.g.
//! `behr_rifle_ballistic_01_black02`), ship-AI `_pu_ai*` (395, e.g.
//! `aegs_avenger_stalker_pu_ai_civ_lowfuel`), plus salvage/unmanned
//! spawn variants.
//!
//! These are **string heuristics** for grouping/dedup — not a structural
//! base-record resolver (a full `BaseRecord + Variants` resolver is
//! deferred until a consumer needs it). Use them to fold obvious variants
//! in catalogs/UIs; for authoritative base identity prefer structural keys
//! (e.g. blueprint record GUID) where available.

/// True if the name carries a `_pu_ai*` AI-spawn variant marker.
pub fn is_ai_variant(name: &str) -> bool {
    name.to_ascii_lowercase().contains("_pu_ai")
}

/// True if the last `_`-segment looks like a cosmetic skin/material
/// variant: an alphabetic run followed by ≥2 trailing digits
/// (`black02`, `tan01`, `mat03`).
pub fn is_cosmetic_variant(name: &str) -> bool {
    name.rsplit('_').next().map(is_color_digit).unwrap_or(false)
}

/// Best-effort base record name: strips a trailing `_pu_ai…` AI-variant
/// suffix if present. Returns the input unchanged otherwise. (Cosmetic
/// `_<color><NN>` suffixes are *not* stripped here — they can be real
/// distinct SKUs; callers that want to fold them check
/// [`is_cosmetic_variant`] explicitly.)
pub fn ai_base_name(name: &str) -> &str {
    match name.to_ascii_lowercase().find("_pu_ai") {
        // Byte index is valid in the original string: lowercasing ASCII
        // preserves byte positions.
        Some(idx) => &name[..idx],
        None => name,
    }
}

/// Segment = letters then ≥2 digits (`black02`). ASCII-only.
fn is_color_digit(seg: &str) -> bool {
    let digits = seg.chars().rev().take_while(|c| c.is_ascii_digit()).count();
    let alpha = seg.len().saturating_sub(digits);
    digits >= 2 && alpha >= 2 && seg[..alpha].chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_variant_detection() {
        assert!(is_ai_variant("aegs_avenger_stalker_pu_ai_civ_lowfuel"));
        assert!(!is_ai_variant("behr_rifle_ballistic_01"));
    }

    #[test]
    fn ai_base_strips_suffix() {
        assert_eq!(
            ai_base_name("aegs_avenger_stalker_pu_ai_civ_lowfuel"),
            "aegs_avenger_stalker"
        );
        assert_eq!(
            ai_base_name("behr_rifle_ballistic_01"),
            "behr_rifle_ballistic_01"
        );
    }

    #[test]
    fn cosmetic_variant_detection() {
        assert!(is_cosmetic_variant("behr_rifle_ballistic_01_black02"));
        assert!(is_cosmetic_variant("foo_tan01"));
        assert!(!is_cosmetic_variant("behr_rifle_ballistic_01")); // "01" alone: no alpha run
        assert!(!is_cosmetic_variant("foo_mag")); // no digits
    }
}
