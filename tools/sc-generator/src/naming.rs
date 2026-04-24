//! Identifier sanitization — snake_case conversion and Rust keyword escaping.

/// Rust keywords that must be escaped when used as identifiers.
/// These can be used as raw identifiers (`r#keyword`).
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "static",
    "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try", "union",
];

/// Rust keywords that CANNOT be used as raw identifiers.
/// These have to be renamed (we append an underscore).
const NON_RAW_KEYWORDS: &[&str] = &["self", "Self", "super", "extern", "crate"];

/// Convert a camelCase / PascalCase identifier to snake_case.
pub fn to_snake_case(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    let mut prev_lower = false;
    let mut prev_digit = false;

    for c in name.chars() {
        if c.is_ascii_uppercase() {
            if (prev_lower || prev_digit) && !out.is_empty() {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
            prev_lower = false;
            prev_digit = false;
        } else if c.is_ascii_lowercase() {
            out.push(c);
            prev_lower = true;
            prev_digit = false;
        } else if c.is_ascii_digit() {
            out.push(c);
            prev_digit = true;
            prev_lower = false;
        } else if c == '_' || c == '-' {
            if !out.ends_with('_') && !out.is_empty() {
                out.push('_');
            }
            prev_lower = false;
            prev_digit = false;
        }
        // Ignore other characters (spaces, punctuation).
    }

    // Trim trailing underscore if the input ended with a separator.
    while out.ends_with('_') {
        out.pop();
    }

    out
}

/// Sanitize a DCB field name into a valid Rust field identifier.
/// Applies snake_case conversion and escapes Rust keywords.
pub fn sanitize_field_name(name: &str) -> String {
    let snake = to_snake_case(name);
    if snake.is_empty() {
        return "_empty".to_string();
    }

    // Ensure it starts with a letter or underscore.
    let first_ok = snake
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic() || c == '_')
        .unwrap_or(false);
    let snake = if first_ok { snake } else { format!("_{snake}") };

    if NON_RAW_KEYWORDS.contains(&snake.as_str()) {
        format!("{snake}_")
    } else if RUST_KEYWORDS.contains(&snake.as_str()) {
        format!("r#{snake}")
    } else {
        snake
    }
}

/// Sanitize a DCB struct type name into a valid Rust type identifier.
/// Struct names in the DCB are usually already valid PascalCase identifiers;
/// this mostly handles edge cases (dots, hyphens, invalid starts). It also
/// renames any name that collides with a Rust keyword that cannot be used
/// as a raw type identifier (e.g., `Self`) by appending an underscore.
pub fn sanitize_struct_name(name: &str) -> String {
    let collapsed = sanitize_identifier(name);
    // Type names can't use raw identifiers for the keywords `self`, `Self`,
    // `super`, `extern`, `crate`. Suffix with `_` instead. Also handle the
    // regular keywords since `r#Weapon` etc. isn't typical for types.
    if NON_RAW_KEYWORDS.contains(&collapsed.as_str()) || RUST_KEYWORDS.contains(&collapsed.as_str())
    {
        format!("{collapsed}_")
    } else {
        collapsed
    }
}

/// Sanitize a DCB enum value name into a valid Rust enum variant identifier.
/// Same rules as struct names — enum variants share the type-position
/// restrictions on raw identifiers.
pub fn sanitize_variant_name(name: &str) -> String {
    sanitize_struct_name(name)
}

/// Shared character sanitization for struct and variant names. Replaces
/// non-alphanumeric characters with underscores, ensures the result starts
/// with a letter/underscore, and collapses consecutive underscores.
fn sanitize_identifier(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    let first_ok = out
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic() || c == '_')
        .unwrap_or(false);
    if !first_ok {
        out = format!("_{out}");
    }

    if out.is_empty() {
        out = "_Unknown".to_string();
    }

    let mut collapsed = String::with_capacity(out.len());
    let mut last_underscore = false;
    for c in out.chars() {
        if c == '_' {
            if !last_underscore {
                collapsed.push(c);
            }
            last_underscore = true;
        } else {
            collapsed.push(c);
            last_underscore = false;
        }
    }

    collapsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_conversions() {
        assert_eq!(to_snake_case("fireRate"), "fire_rate");
        // Consecutive uppercase letters are kept together (no separator between
        // them), so acronyms like "SC" in "SCItem" collapse as a unit. This is
        // a deliberate simplification — the generator cares about valid /
        // unique identifiers, not about perfect camelCase↔snake_case round-trip.
        assert_eq!(
            to_snake_case("SCItemWeaponComponentParams"),
            "scitem_weapon_component_params"
        );
        assert_eq!(to_snake_case("damage_multiplier"), "damage_multiplier");
        assert_eq!(to_snake_case("maxAmmoLoad"), "max_ammo_load");
        assert_eq!(
            to_snake_case("simplified_heatParams"),
            "simplified_heat_params"
        );
        // Digits attach to the preceding run.
        assert_eq!(to_snake_case("HTTPRequest2"), "httprequest2");
    }

    #[test]
    fn field_name_sanitization() {
        assert_eq!(sanitize_field_name("fireRate"), "fire_rate");
        assert_eq!(sanitize_field_name("type"), "r#type");
        assert_eq!(sanitize_field_name("ref"), "r#ref");
        assert_eq!(sanitize_field_name("match"), "r#match");
        assert_eq!(sanitize_field_name("self"), "self_");
        assert_eq!(sanitize_field_name(""), "_empty");
    }

    #[test]
    fn struct_name_sanitization() {
        assert_eq!(
            sanitize_struct_name("SCItemWeaponComponentParams"),
            "SCItemWeaponComponentParams"
        );
        assert_eq!(sanitize_struct_name("Foo.Bar"), "Foo_Bar");
        assert_eq!(sanitize_struct_name("Foo-Bar"), "Foo_Bar");
        assert_eq!(sanitize_struct_name("123Name"), "_123Name");
        // Consecutive non-alpha become single underscore.
        assert_eq!(sanitize_struct_name("Foo__Bar"), "Foo_Bar");
    }
}
