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

/// Why a regen must wait for a human instead of merging itself; empty when
/// it may merge once GitHub CI passes. Removed record types make the release
/// `0.X.0` and can break consumers; drifting buckets mean the classifier
/// needs a look (CLAUDE.md, *Generated code and feature gating*).
pub fn auto_merge_blockers(
    live: bool,
    before: &Bindings,
    after: &Bindings,
    checks: &[Check],
) -> Vec<String> {
    let mut blockers = Vec::new();
    if !live {
        blockers.push("not a LIVE or hotfix build".to_string());
    }
    for check in checks.iter().filter(|c| !c.passed) {
        blockers.push(format!("`{}` failed", check.name));
    }
    let removed = removed_record_types(before, after);
    if !removed.is_empty() {
        blockers.push(format!("record types removed: {}", names(&removed)));
    }
    // More than 10% growth in `core/`, or more than 25% shrinkage in `dormant/`.
    if after.core * 10 > before.core * 11 {
        blockers.push(format!(
            "`core/` grew from {} to {} types",
            before.core, after.core
        ));
    }
    if after.dormant * 4 < before.dormant * 3 {
        blockers.push(format!(
            "`dormant/` shrank from {} to {} types",
            before.dormant, after.dormant
        ));
    }
    blockers
}

fn removed_record_types<'a>(before: &'a Bindings, after: &'a Bindings) -> Vec<&'a String> {
    before
        .record_types
        .difference(&after.record_types)
        .collect()
}

pub fn pr_body(
    build: &BuildInfo,
    before: &Bindings,
    after: &Bindings,
    checks: &[Check],
    blockers: &[String],
) -> String {
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
    let removed = removed_record_types(before, after);
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
    out.push_str("\n### Merge\n\n");
    if blockers.is_empty() {
        out.push_str(
            "`sc-regen` merges this once GitHub CI passes; the release workflow then \
             tags `datacore/*` and cuts the `sc-holotable/v*` release. Remove the \
             `auto-merge` label to stop it.\n",
        );
    } else {
        out.push_str("Left for review:\n\n");
        for blocker in blockers {
            let _ = writeln!(out, "- {blocker}");
        }
        out.push_str("\nMerging it by hand still tags `datacore/*` and cuts the release.\n");
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

    fn bindings(record_types: &[&str], core: usize, dormant: usize) -> Bindings {
        Bindings {
            record_types: record_types.iter().map(|s| s.to_string()).collect(),
            structs: 0,
            enums: 0,
            core,
            multi_feature: 0,
            dormant,
            leaf: 0,
            leaf_features: 0,
        }
    }

    #[test]
    fn auto_merge_only_for_a_clean_live_regen() {
        let before = bindings(&["A", "B"], 100, 100);
        let pass = [Check {
            name: "t".into(),
            passed: true,
            tail: String::new(),
        }];
        let fail = [Check {
            name: "t".into(),
            passed: false,
            tail: String::new(),
        }];

        // Added types and modest bucket movement are fine.
        let grown = bindings(&["A", "B", "C"], 110, 75);
        assert!(auto_merge_blockers(true, &before, &grown, &pass).is_empty());

        assert_eq!(auto_merge_blockers(false, &before, &grown, &pass).len(), 1);
        assert_eq!(auto_merge_blockers(true, &before, &grown, &fail).len(), 1);
        let removed = bindings(&["A"], 100, 100);
        assert_eq!(
            auto_merge_blockers(true, &before, &removed, &pass),
            ["record types removed: `B`"]
        );
        let drifted = bindings(&["A", "B"], 111, 74);
        assert_eq!(auto_merge_blockers(true, &before, &drifted, &pass).len(), 2);
    }
}
