//! What a regen changed, read from the generated files before and after,
//! and the PR body that reports it.

use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

/// A summary of one state of `crates/sc-extract-generated/src/generated`.
pub struct Bindings {
    /// Every record type the generator emitted a binding for.
    record_types: BTreeSet<String>,
    /// Types the newest covered build no longer has, or has no record of:
    /// the generator marks them `#[deprecated]`. Once the build that still
    /// had them leaves the window, their removal is expected.
    retired: BTreeSet<String>,
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
            retired: BTreeSet::new(),
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
            let types = read(&types)?;
            let count = types
                .lines()
                .filter(|l| l.starts_with("pub struct "))
                .count();
            bindings.retired.extend(deprecated_structs(&types));
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

/// One game build a regen ran against.
pub struct GameBuild<'a> {
    pub label: &'a str,
    pub channel: String,
    pub changelist: u64,
}

/// The game builds a regen ran against, oldest first.
pub struct BuildInfo<'a> {
    pub builds: Vec<GameBuild<'a>>,
    pub previous_tag: Option<&'a str>,
}

/// `GENERATED_BUILDS` from the generated `metadata.rs`: the labels of the
/// builds the bindings cover, oldest first. Empty for bindings from a
/// generator that predates it.
pub fn generated_builds(generated: &Path) -> Result<Vec<String>, String> {
    Ok(generated_builds_in(&read(&generated.join("metadata.rs"))?))
}

fn generated_builds_in(metadata: &str) -> Vec<String> {
    // rustfmt may wrap the array over several lines.
    let Some(start) = metadata.find("pub const GENERATED_BUILDS") else {
        return Vec::new();
    };
    let rest = &metadata[start..];
    let rest = &rest[rest.find("= &[").map_or(rest.len(), |i| i + 4)..];
    let array = &rest[..rest.find(']').unwrap_or(rest.len())];
    array
        .split(',')
        .filter_map(|s| s.trim().strip_prefix('"').and_then(|s| s.strip_suffix('"')))
        .map(str::to_string)
        .collect()
}

/// Why a regen must wait for a human instead of merging itself; empty when
/// it may merge once GitHub CI passes. Removed record types make the release
/// `0.X.0` and can break consumers, unless the previous bindings already
/// marked them deprecated (a build that had them left the window; the
/// workspace tests still catch a crate that used them). Drifting buckets
/// mean the classifier needs a look (CLAUDE.md, *Generated code and
/// feature gating*).
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
    let unexpected: Vec<&String> = removed_record_types(before, after)
        .into_iter()
        .filter(|t| !before.retired.contains(*t))
        .collect();
    if !unexpected.is_empty() {
        blockers.push(format!("record types removed: {}", names(&unexpected)));
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
    let builds: Vec<String> = build
        .builds
        .iter()
        .map(|b| {
            format!(
                "**{}** ({}, changelist {})",
                b.label, b.channel, b.changelist
            )
        })
        .collect();
    let _ = writeln!(
        out,
        "Bindings regenerated from {}. Previous tag: {previous}.\n",
        builds.join(" and ")
    );
    if build.builds.len() > 1 {
        out.push_str(
            "The bindings are the union of these builds, so one release reads all of \
             them. What the newest build dropped is `#[deprecated]`.\n\n",
        );
    }

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
        let expected: Vec<&String> = removed
            .iter()
            .copied()
            .filter(|t| before.retired.contains(*t))
            .collect();
        if !expected.is_empty() {
            let _ = writeln!(
                out,
                "\nAlready deprecated before this regen (expected): {}",
                names(&expected)
            );
        }
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

/// The structs a generated `types.rs` marks `#[deprecated]` as a whole. A
/// struct-level attribute sits at column 0, among the other attributes
/// (`#[cfg(...)]`, possibly over several lines) right above `pub struct`;
/// a field-level one is indented.
fn deprecated_structs(types: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut pending = false;
    for line in types.lines() {
        if line.starts_with("#[deprecated") {
            pending = true;
        } else if let Some(rest) = line.strip_prefix("pub struct ") {
            if pending {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                found.push(name);
            }
            pending = false;
        } else if line.is_empty() || line.starts_with("pub ") || line.starts_with("impl") {
            pending = false;
        }
    }
    found
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

    #[test]
    fn reads_generated_builds_on_one_line_or_wrapped() {
        let one = "pub const GENERATED_BUILDS: &[&str] = &[\"4.10.3-live.1\", \"5.0.0-ptu.2\"];\n";
        assert_eq!(generated_builds_in(one), ["4.10.3-live.1", "5.0.0-ptu.2"]);
        let wrapped = "/// x\npub const GENERATED_BUILDS: &[&str] = &[\n    \"a.1\",\n    \"b.2\",\n];\npub const OTHER: &[&str] = &[\"c\"];\n";
        assert_eq!(generated_builds_in(wrapped), ["a.1", "b.2"]);
        assert!(generated_builds_in("pub const STRUCT_COUNT: usize = 1;\n").is_empty());
    }

    #[test]
    fn finds_struct_level_deprecations_only() {
        let types = "\
/// doc
#[deprecated(note = \"absent from 5.0.0-ptu.2\")]
pub struct Gone {}

#[deprecated(note = \"no record in 5.0.0-ptu.2 uses this type\")]
#[cfg(any(
    feature = \"a\",
    feature = \"b\"
))]
pub struct Unused {
    pub x: bool,
}

pub struct Kept {
    #[deprecated(note = \"absent from 5.0.0-ptu.2\")]
    pub old_field: bool,
}
";
        assert_eq!(deprecated_structs(types), ["Gone", "Unused"]);
    }

    #[test]
    fn a_deprecated_record_type_may_leave() {
        let pass = [Check {
            name: "t".into(),
            passed: true,
            tail: String::new(),
        }];
        let mut before = bindings(&["A", "B", "C"], 100, 100);
        before.retired.insert("B".into());
        let after = bindings(&["A"], 100, 100);
        assert_eq!(
            auto_merge_blockers(true, &before, &after, &pass),
            ["record types removed: `C`"]
        );
    }

    fn bindings(record_types: &[&str], core: usize, dormant: usize) -> Bindings {
        Bindings {
            record_types: record_types.iter().map(|s| s.to_string()).collect(),
            retired: BTreeSet::new(),
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
