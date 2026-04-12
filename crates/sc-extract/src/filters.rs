//! Playable-content predicates.
//!
//! Not every record in the DCB is user-visible content. Weapons that are
//! placeholders, AI-only ship variants, debris, and internal "LowPoly"
//! versions all ship inside `Data.p4k` but should be hidden from user-
//! facing UIs. These predicates encode the rules documented in bulkhead's
//! `docs/damage-system.md`.
//!
//! The predicates are **pure functions** taking pre-extracted strings so
//! they can be unit-tested without a real DCB. Callers extract the
//! relevant fields from svarog `Instance` values (via `get_str`,
//! `get_i32`, etc.) and pass them in. See the docs for
//! `is_playable_weapon` and `is_playable_ship` for the concrete field
//! lists.
//!
//! **Consumers decide whether to filter.** These functions return
//! predicates; nothing in sc-extract silently drops records.

/// Is this entity a user-visible ship weapon or FPS weapon?
///
/// Rules (from bulkhead's damage-system docs):
///
/// - **Include** if `type == "WeaponGun"` AND `sub_type` in `{"Gun", "Rocket"}`,
///   OR `type == "WeaponPersonal"` AND `sub_type != "Gadget"`.
/// - **Hide** if no localization (`display_name` is `None` or empty).
/// - **Hide** if display name is `"PLACEHOLDER"` (case-insensitive).
/// - **Hide** if `size == Some(0)`.
/// - **Hide** if `class_name` contains any of `_LowPoly`, `_Dummy`,
///   `_FakeHologram` (case-insensitive).
/// - **Hide** if `class_name` starts with `vlk_` (case-insensitive).
///
/// # Arguments
///
/// - `type_name` — the record's `AttachDef.Type` (e.g. `"WeaponGun"`)
/// - `sub_type` — the record's `AttachDef.SubType` (e.g. `"Gun"`)
/// - `display_name` — the resolved display name from the locale map, if any
/// - `class_name` — the DCB record's class name (e.g. `"GATS_BallisticGatling_S1"`)
/// - `size` — the `AttachDef.Size` field, if extracted
pub fn is_playable_weapon(
    type_name: &str,
    sub_type: &str,
    display_name: Option<&str>,
    class_name: &str,
    size: Option<i32>,
) -> bool {
    // Must match one of the weapon type/subtype combinations.
    let type_ok = match type_name {
        "WeaponGun" => matches!(sub_type, "Gun" | "Rocket"),
        "WeaponPersonal" => sub_type != "Gadget",
        _ => false,
    };
    if !type_ok {
        return false;
    }

    if !has_valid_display_name(display_name) {
        return false;
    }

    if size == Some(0) {
        return false;
    }

    if has_internal_class_pattern(class_name) {
        return false;
    }

    true
}

/// Is this entity a user-visible ship?
///
/// Rules (from bulkhead's damage-system docs):
///
/// - **Include** if `has_vehicle_component` is `true`.
/// - **Hide** if no localization (`display_name` is `None` or empty or placeholder).
/// - **Hide** if `class_name` contains any of `_AI_`, `Unmanned_`,
///   `_Template` (case-sensitive, these are actual class-name fragments).
/// - **Hide** if `class_name` contains `Debris`, `Wreck`, or `Derelict`
///   (case-insensitive).
/// - **Hide** if `class_name` contains any of `_BIS`, `_Showdown`, `_FW_`,
///   `_Mission_` (event variants).
///
/// # Arguments
///
/// - `class_name` — the DCB record's class name (e.g. `"AEGS_Gladius"`)
/// - `display_name` — the resolved display name from the locale map, if any
/// - `has_vehicle_component` — true if the entity has a
///   `VehicleComponentParams` in its components array
pub fn is_playable_ship(
    class_name: &str,
    display_name: Option<&str>,
    has_vehicle_component: bool,
) -> bool {
    if !has_vehicle_component {
        return false;
    }

    if !has_valid_display_name(display_name) {
        return false;
    }

    // AI and template variants (case-sensitive; these are exact substrings
    // bulkhead's filtering rules look for).
    if class_name.contains("_AI_")
        || class_name.contains("Unmanned_")
        || class_name.contains("_Template")
    {
        return false;
    }

    // Debris / derelicts (case-insensitive).
    let lower = class_name.to_lowercase();
    if lower.contains("debris") || lower.contains("wreck") || lower.contains("derelict") {
        return false;
    }

    // Event variants.
    if class_name.contains("_BIS")
        || class_name.contains("_Showdown")
        || class_name.contains("_FW_")
        || class_name.contains("_Mission_")
    {
        return false;
    }

    true
}

/// True if a display name is present, non-empty, and not a placeholder.
/// Used by multiple filter predicates — the rule is the same everywhere.
fn has_valid_display_name(display_name: Option<&str>) -> bool {
    match display_name {
        None => false,
        Some(name) => {
            let trimmed = name.trim();
            !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("placeholder")
        }
    }
}

/// True if the class name looks like an internal / development record
/// (LowPoly, Dummy, FakeHologram, vlk_ prefix).
fn has_internal_class_pattern(class_name: &str) -> bool {
    let lower = class_name.to_lowercase();
    if lower.contains("_lowpoly") || lower.contains("_dummy") || lower.contains("_fakehologram") {
        return true;
    }
    if lower.starts_with("vlk_") {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_playable_weapon ─────────────────────────────────────────────

    #[test]
    fn weapon_weapon_gun_gun_is_playable() {
        assert!(is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("CF-117 Bulldog"),
            "GATS_BallisticGatling_S1",
            Some(1),
        ));
    }

    #[test]
    fn weapon_weapon_gun_rocket_is_playable() {
        assert!(is_playable_weapon(
            "WeaponGun",
            "Rocket",
            Some("Thunderbolt"),
            "ANVL_Rocket_S3",
            Some(3),
        ));
    }

    #[test]
    fn weapon_weapon_personal_non_gadget_is_playable() {
        assert!(is_playable_weapon(
            "WeaponPersonal",
            "Rifle",
            Some("P4-AR"),
            "KBAR_P4AR",
            None,
        ));
    }

    #[test]
    fn weapon_weapon_gun_turret_is_not_playable() {
        // Turret is not in the allowed sub_type list for WeaponGun.
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Turret",
            Some("Some Turret"),
            "KLWE_Turret_S5",
            Some(5),
        ));
    }

    #[test]
    fn weapon_weapon_personal_gadget_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponPersonal",
            "Gadget",
            Some("Multi-Tool"),
            "GAD_Multitool",
            None,
        ));
    }

    #[test]
    fn weapon_unknown_type_is_not_playable() {
        assert!(!is_playable_weapon(
            "Something",
            "Else",
            Some("Whatever"),
            "wut_1",
            None,
        ));
    }

    #[test]
    fn weapon_missing_display_name_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            None,
            "GATS_BallisticGatling_S1",
            Some(1),
        ));
    }

    #[test]
    fn weapon_empty_display_name_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some(""),
            "GATS_BallisticGatling_S1",
            Some(1),
        ));
    }

    #[test]
    fn weapon_placeholder_display_name_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("PLACEHOLDER"),
            "WTEST_S1",
            Some(1),
        ));
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("placeholder"),
            "WTEST_S1",
            Some(1),
        ));
    }

    #[test]
    fn weapon_size_zero_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("Zero Size"),
            "ZERO_S0",
            Some(0),
        ));
    }

    #[test]
    fn weapon_size_none_is_allowed() {
        // If size isn't extracted, don't filter on it.
        assert!(is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("No Size Extracted"),
            "GATS_S1",
            None,
        ));
    }

    #[test]
    fn weapon_lowpoly_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("Low Poly Cannon"),
            "GATS_Cannon_S1_LowPoly",
            Some(1),
        ));
    }

    #[test]
    fn weapon_dummy_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("Dummy"),
            "GATS_Cannon_S1_Dummy",
            Some(1),
        ));
    }

    #[test]
    fn weapon_fakehologram_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("Holo"),
            "GATS_FakeHologram_S1",
            Some(1),
        ));
    }

    #[test]
    fn weapon_vlk_prefix_is_not_playable() {
        assert!(!is_playable_weapon(
            "WeaponGun",
            "Gun",
            Some("Vanduul Cannon"),
            "vlk_cannon_s1",
            Some(1),
        ));
    }

    // ── is_playable_ship ───────────────────────────────────────────────

    #[test]
    fn ship_with_vehicle_component_and_name_is_playable() {
        assert!(is_playable_ship("AEGS_Gladius", Some("Gladius"), true));
    }

    #[test]
    fn ship_without_vehicle_component_is_not_playable() {
        assert!(!is_playable_ship("AEGS_Gladius", Some("Gladius"), false));
    }

    #[test]
    fn ship_missing_display_name_is_not_playable() {
        assert!(!is_playable_ship("AEGS_Gladius", None, true));
    }

    #[test]
    fn ship_ai_variant_is_not_playable() {
        assert!(!is_playable_ship(
            "AEGS_Gladius_AI_Advocacy",
            Some("Gladius"),
            true,
        ));
    }

    #[test]
    fn ship_unmanned_is_not_playable() {
        assert!(!is_playable_ship(
            "AEGS_Avenger_Unmanned_Salvage",
            Some("Avenger"),
            true,
        ));
    }

    #[test]
    fn ship_template_is_not_playable() {
        assert!(!is_playable_ship(
            "AEGS_Gladius_Template",
            Some("Gladius"),
            true,
        ));
    }

    #[test]
    fn ship_debris_is_not_playable() {
        assert!(!is_playable_ship(
            "AEGS_Gladius_Debris",
            Some("Debris"),
            true,
        ));
    }

    #[test]
    fn ship_wreck_is_not_playable() {
        assert!(!is_playable_ship("Ship_Wreck_01", Some("Wreck"), true));
    }

    #[test]
    fn ship_bis_event_variant_is_not_playable() {
        assert!(!is_playable_ship(
            "AEGS_Eclipse_BIS2950",
            Some("Eclipse"),
            true,
        ));
    }

    #[test]
    fn ship_showdown_is_not_playable() {
        assert!(!is_playable_ship(
            "AEGS_Gladius_Showdown",
            Some("Gladius"),
            true,
        ));
    }
}
