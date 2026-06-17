//! Probe: read `HarvestableProviderComponent.preset` GUIDs out of object-container
//! files.
//!
//! This doubles as a worked demonstration of the generic
//! [`sc_extract::object_container`] reader: the example knows the *domain* fact
//! (the body→provider join is `HarvestableProviderComponent.preset`); the reader
//! supplies the *format* (socpak ZIP / CrCh `.soc` / CryXmlB / plain XML →
//! [`XmlNode`] tree). All the chunk-peeling and CryXmlB decoding lives in the
//! library; here we just `decode()` and walk the tree.
//!
//! It settles the resource-gathering "where" join (see
//! `docs/resource-gathering.md`): the link sits in a plain/CryXmlB `pivot.entxml`
//! (planet/moon bodies) or inside the binary `.soc` CrCh chunk container
//! (asteroid fields, gas clouds, lagrange childclouds) — both handled uniformly.
//!
//! Usage:
//!
//! ```bash
//! # Scan the default probe dir (target/probe-resources) for .soc/.pla/.entxml:
//! cargo run -p sc-extract --example soc_harvestable_probe
//!
//! # Scan explicit files or directories:
//! cargo run -p sc-extract --example soc_harvestable_probe -- path/to/body.soc some/dir
//!
//! # Assert a specific GUID is present somewhere (exit 1 if missing) — the
//! # validation gate. Clio: 703a18ca-7f7c-4489-a64a-cd0cd359b8fe (HPP_Stanton4b);
//! # Glaciem Ring: e9aa8f98-4c87-468f-ae03-10a96d9497e5 (HPP_Nyx_GlaciemRing):
//! cargo run -p sc-extract --example soc_harvestable_probe -- --expect e9aa8f98-4c87-468f-ae03-10a96d9497e5
//! ```

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sc_extract::object_container;

/// The all-zero GUID some segment containers carry instead of a real preset.
const NULL_GUID: &str = "00000000-0000-0000-0000-000000000000";

fn main() -> ExitCode {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut expect: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--expect" => match args.next() {
                Some(g) => expect = Some(g.to_ascii_lowercase()),
                None => {
                    eprintln!("--expect needs a GUID argument");
                    return ExitCode::from(2);
                }
            },
            other => paths.push(PathBuf::from(other)),
        }
    }

    // Default scan target: the exploration dump under target/.
    if paths.is_empty() {
        let default = default_probe_dir();
        println!("no paths given — scanning {}", default.display());
        paths.push(default);
    }

    let mut files: Vec<PathBuf> = Vec::new();
    for p in &paths {
        collect(p, &mut files);
    }
    files.sort();

    if files.is_empty() {
        eprintln!("no .soc/.pla/.entxml files found under the given paths");
        return ExitCode::from(2);
    }

    let mut all_hits: Vec<(PathBuf, String)> = Vec::new();
    let mut decode_failures = 0usize;

    for file in &files {
        match providers_in_file(file) {
            Ok(guids) if guids.is_empty() => {}
            Ok(guids) => {
                let rel = file.display();
                for g in guids {
                    let tag = if g == NULL_GUID { "  (null)" } else { "" };
                    println!("{rel}\n    preset = {g}{tag}");
                    all_hits.push((file.clone(), g));
                }
            }
            Err(e) => {
                decode_failures += 1;
                eprintln!("! {} — {e}", file.display());
            }
        }
    }

    let real = all_hits.iter().filter(|(_, g)| g != NULL_GUID).count();

    println!(
        "\nscanned {} files — {} provider component(s), {} with a real GUID, {} null{}",
        files.len(),
        all_hits.len(),
        real,
        all_hits.len() - real,
        if decode_failures > 0 {
            format!(", {decode_failures} decode failure(s)")
        } else {
            String::new()
        }
    );

    if let Some(want) = expect {
        if all_hits.iter().any(|(_, g)| *g == want) {
            println!("EXPECT OK: {want} present");
            ExitCode::SUCCESS
        } else {
            eprintln!("EXPECT FAILED: {want} not found in any scanned file");
            ExitCode::FAILURE
        }
    } else {
        ExitCode::SUCCESS
    }
}

/// `<crate>/../../target/probe-resources` relative to this example.
fn default_probe_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR = crates/sc-extract; the workspace target/ is two up.
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/probe-resources")
}

/// Recursively collect candidate object-container files.
fn collect(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_file() {
        if is_candidate(path) {
            out.push(path.to_path_buf());
        }
        return;
    }
    let Ok(entries) = std::fs::read_dir(path) else {
        eprintln!("! cannot read dir {}", path.display());
        return;
    };
    for entry in entries.flatten() {
        collect(&entry.path(), out);
    }
}

fn is_candidate(path: &Path) -> bool {
    // The provider join always lives in the original object container
    // (`.soc`/`.pla` CrCh files, or `pivot.entxml`), never in a decoded `.xml`
    // sibling — so we skip `.xml` to avoid sweeping the whole DCB record corpus.
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("soc") | Some("pla") | Some("entxml")
    )
}

/// Decode one object-container file via the generic reader and pull out every
/// `HarvestableProviderComponent.preset` GUID (lower-cased). An include-only
/// CrCh container decodes to `None` — no entity tree, so no providers.
fn providers_in_file(path: &Path) -> Result<Vec<String>, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let Some(root) = object_container::decode(&bytes).map_err(|e| e.to_string())? else {
        return Ok(Vec::new());
    };
    Ok(root
        .find_all("HarvestableProviderComponent")
        .filter_map(|n| n.attr("preset"))
        .map(str::to_ascii_lowercase)
        .collect())
}
