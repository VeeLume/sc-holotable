//! What a regen changed, read from the generated files before and after,
//! and the PR body that reports it.

use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

/// A summary of one state of `crates/sc-extract-generated/src/generated`.
pub struct Bindings {
    /// Every record type the generator emitted a binding for.
    record_types: BTreeSet<String>,
    structs: usize,
    enums: usize,
    core: usize,
    multi_feature: usize,
    dormant: usize,
    leaf: usize,
    leaf_features: usize,
}

impl Bindings {
    pub fn read(generated: &Path) -> Result<Self, String> {
        let record_store = read(&generated.join("record_store.rs"))?;
        let metadata = read(&generated.join("metadata.rs"))?;
        let mut bindings = Self {
            record_types: known_record_types(&record_store),
            structs: metadata_count(&metadata, "STRUCT_COUNT"),
            enums: metadata_count(&metadata, "ENUM_COUNT"),
            core: 0,
            multi_feature: 0,
            dormant: 0,
            leaf: 0,
            leaf_features: 0,
        };
        let entries = std::fs::read_dir(generated)
            .map_err(|e| format!("reading {}: {e}", generated.display()))?;
        for entry in entries.flatten() {
            let types = entry.path().join("types.rs");
            if !types.is_file() {
                continue;
            }
            let count = read(&types)?
                .lines()
                .filter(|l| l.starts_with("pub struct "))
                .count();
            match entry.file_name().to_string_lossy().as_ref() {
                "core" => bindings.core = count,
                "multi_feature" => bindings.multi_feature = count,
                "dormant" => bindings.dormant = count,
                _ => {
                    bindings.leaf += count;
                    bindings.leaf_features += 1;
                }
            }
        }
        Ok(bindings)
    }
}

/// One check run against the regenerated bindings.
pub struct Check {
    pub name: String,
    pub passed: bool,
    /// The last lines of its combined output.
    pub tail: String,
}

/// The game build a regen ran against.
pub struct BuildInfo<'a> {
    pub label: &'a str,
    pub channel: &'a str,
    pub changelist: u64,
    pub previous_tag: Option<&'a str>,
}

pub fn pr_body(build: &BuildInfo, before: &Bindings, after: &Bindings, checks: &[Check]) -> String {
    let mut out = String::new();
    let previous = build
        .previous_tag
        .map(|t| format!("`{t}`"))
        .unwrap_or_else(|| "none".into());
    let _ = writeln!(
        out,
        "Bindings regenerated from **{}** ({}, changelist {}). Previous tag: {previous}.\n",
        build.label, build.channel, build.changelist
    );

    out.push_str("| | before | after |\n|---|---:|---:|\n");
    let rows = [
        ("DCB structs", before.structs, after.structs),
        ("DCB enums", before.enums, after.enums),
        (
            "Record types with bindings",
            before.record_types.len(),
            after.record_types.len(),
        ),
        ("`core/` types", before.core, after.core),
        (
            "`multi_feature/` types",
            before.multi_feature,
            after.multi_feature,
        ),
        ("`dormant/` types", before.dormant, after.dormant),
        ("Leaf-feature types", before.leaf, after.leaf),
        ("Leaf features", before.leaf_features, after.leaf_features),
    ];
    for (name, b, a) in rows {
        let _ = writeln!(out, "| {name} | {b} | {a} |");
    }
    out.push_str(
        "\nClassifier drift shows as `core/` ballooning or `dormant/` collapsing \
         (CLAUDE.md, *Generated code and feature gating*).\n\n",
    );

    let added: Vec<&String> = after
        .record_types
        .difference(&before.record_types)
        .collect();
    let removed: Vec<&String> = before
        .record_types
        .difference(&after.record_types)
        .collect();
    let _ = writeln!(
        out,
        "**Record types added ({}):** {}",
        added.len(),
        names(&added)
    );
    let _ = writeln!(
        out,
        "\n**Record types removed ({}):** {}",
        removed.len(),
        names(&removed)
    );
    if removed.is_empty() {
        out.push_str(
            "\nNo record type was removed. Field- and enum-level changes are only \
             in the diff.\n",
        );
    } else {
        out.push_str(
            "\nRecord types were removed: generated surface shrank, so this release \
             is `0.X.0`.\n",
        );
    }

    out.push_str("\n### Checks\n\n");
    for check in checks {
        let mark = if check.passed { "pass" } else { "**FAIL**" };
        let _ = writeln!(out, "- {mark}: `{}`", check.name);
    }
    // A passing check's tail is a list of test binaries; only failures say anything.
    for check in checks.iter().filter(|c| !c.passed) {
        let _ = writeln!(
            out,
            "\n<details><summary>{} output (tail)</summary>\n\n```text\n{}\n```\n</details>",
            check.name,
            check.tail.trim_end()
        );
    }
    out.push_str("\nOpened by `sc-regen`.\n");
    out
}

fn names(list: &[&String]) -> String {
    if list.is_empty() {
        return "none".into();
    }
    list.iter()
        .map(|n| format!("`{n}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))
}

/// The entries of `KNOWN_RECORD_TYPE_NAMES` in the generated
/// `seed_database`: one quoted name per line.
fn known_record_types(record_store: &str) -> BTreeSet<String> {
    record_store
        .lines()
        .skip_while(|l| !l.contains("const KNOWN_RECORD_TYPE_NAMES"))
        .skip(1)
        .take_while(|l| !l.trim_start().starts_with("];"))
        .filter_map(|l| {
            l.trim()
                .strip_prefix('"')
                .and_then(|l| l.strip_suffix("\","))
                .map(str::to_string)
        })
        .collect()
}

/// `pub const <NAME>: usize = <n>;` from the generated `metadata.rs`.
fn metadata_count(metadata: &str, name: &str) -> usize {
    let prefix = format!("pub const {name}: usize = ");
    metadata
        .lines()
        .find_map(|l| l.strip_prefix(&prefix)?.strip_suffix(';')?.parse().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_known_record_types_and_counts() {
        let store = "        const KNOWN_RECORD_TYPE_NAMES: &[&str] = &[\n            \"A\",\n            \"B\",\n        ];\n";
        let names = known_record_types(store);
        assert_eq!(names.into_iter().collect::<Vec<_>>(), ["A", "B"]);
        let metadata =
            "pub const STRUCT_COUNT: usize = 6696;\npub const ENUM_COUNT: usize = 774;\n";
        assert_eq!(metadata_count(metadata, "STRUCT_COUNT"), 6696);
        assert_eq!(metadata_count(metadata, "ENUM_COUNT"), 774);
    }
}
