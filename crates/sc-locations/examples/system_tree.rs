//! Render the universe's *logical* structure as a tree — the `StarMapObject`
//! parent hierarchy from [`Locations`] — optionally annotated with whether we
//! harvested a **socpak position** for each node (`--pos`).
//!
//! The tree comes from the DCB (fast, ~28s). `--pos` additionally does the full
//! `objectcontainers/*.socpak` walk (~2min) and marks each node `●` (positioned)
//! or `○` (no position harvested) — so the coverage gaps (Crusader, ArcCorp, …)
//! are visible *in context* in the tree.
//!
//! Usage:
//!   cargo run -p sc-locations --example system_tree --release
//!   cargo run -p sc-locations --example system_tree --release -- stanton
//!   cargo run -p sc-locations --example system_tree --release -- stanton --pos

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use sc_extract::object_container::{Socpak, decode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid};
use sc_locations::{Location, Locations, RecordCollection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let missing = args.iter().any(|a| a == "--missing");
    let want_pos = args.iter().any(|a| a == "--pos") || missing;
    let no_ast = args.iter().any(|a| a == "--no-asteroids");
    let filter = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(|s| s.to_ascii_lowercase());

    let install = sc_discovery::discover_primary()?;
    println!("-> {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let ad = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &ad)?;
    let locs = Locations::build(datacore.records());

    // Optional: harvest socpak positions (guid → world Pos).
    let positions: HashMap<Guid, [f64; 3]> = if want_pos {
        harvest_positions(&assets)
    } else {
        HashMap::new()
    };

    // --missing: list every location we DON'T have a harvested position for,
    // grouped by kind. Iterates all locations (not just tree-reachable).
    if missing {
        report_missing(&locs, &ad, &positions, no_ast);
        return Ok(());
    }

    // Roots = locations with no parent (the Star of each system + parentless
    // SolarSystem nodes). Sort for stable output.
    let mut roots: Vec<&Location> = locs
        .iter()
        .map(|(_, l)| l)
        .filter(|l| l.parent.is_none())
        .collect();
    roots.sort_by_key(|l| l.display_name(&ad.locale).unwrap_or("").to_string());

    let mut counts = Counts::default();
    for root in roots {
        // System filter: match the root's own name or skip.
        if let Some(f) = &filter {
            let n = root
                .display_name(&ad.locale)
                .unwrap_or("")
                .to_ascii_lowercase();
            if !n.contains(f) {
                continue;
            }
        }
        print_node(
            root,
            0,
            &locs,
            &ad,
            &positions,
            want_pos,
            no_ast,
            &mut counts,
            &mut HashSet::new(),
        );
    }

    println!("\n─── {} nodes shown", counts.total);
    if want_pos {
        println!(
            "─── {}/{} positioned ({} missing)",
            counts.positioned,
            counts.total,
            counts.total - counts.positioned
        );
        // Which kinds are worst-covered.
        let mut kinds: Vec<(&String, &(u32, u32))> = counts.by_kind.iter().collect();
        kinds.sort_by_key(|(k, _)| k.to_string());
        println!("─── per-kind positioned/total:");
        for (k, (p, t)) in kinds {
            println!("      {p:>4}/{t:<4}  {k}");
        }
    }
    Ok(())
}

/// List every location without a harvested position, grouped by kind.
fn report_missing(
    locs: &Locations,
    ad: &AssetData,
    positions: &HashMap<Guid, [f64; 3]>,
    no_ast: bool,
) {
    // kind → (missing entries, total count)
    let mut by_kind: HashMap<String, (Vec<(String, String)>, u32)> = HashMap::new();
    for (guid, loc) in locs.iter() {
        let kind = format!("{:?}", loc.kind);
        if no_ast && kind.contains("Asteroid") {
            continue;
        }
        let slot = by_kind.entry(kind).or_default();
        slot.1 += 1;
        if positions.contains_key(guid) {
            continue;
        }
        let name = loc
            .display_name(&ad.locale)
            .map(str::to_string)
            .unwrap_or_else(|| "<unnamed>".to_string());
        let parent = locs
            .parent_of(guid)
            .and_then(|p| p.display_name(&ad.locale))
            .unwrap_or("?")
            .to_string();
        slot.0.push((name, parent));
    }

    #[allow(clippy::type_complexity)]
    let mut kinds: Vec<(&String, &(Vec<(String, String)>, u32))> = by_kind.iter().collect();
    // Worst-covered kinds first (by missing count).
    kinds.sort_by_key(|(_, (m, _))| std::cmp::Reverse(m.len()));

    let total_missing: usize = by_kind.values().map(|(m, _)| m.len()).sum();
    println!("═══ MISSING POSITIONS — {total_missing} locations ═══\n");
    for (kind, (miss, total)) in &kinds {
        if miss.is_empty() {
            continue;
        }
        println!("── {kind}: {}/{} missing", miss.len(), total);
        let mut entries = (*miss).clone();
        entries.sort();
        for (name, parent) in entries.iter().take(60) {
            println!("     {name:<44} (under {parent})");
        }
        if entries.len() > 60 {
            println!("     … +{} more", entries.len() - 60);
        }
        println!();
    }
}

#[derive(Default)]
struct Counts {
    total: u32,
    positioned: u32,
    by_kind: HashMap<String, (u32, u32)>,
}

#[allow(clippy::too_many_arguments)]
fn print_node(
    loc: &Location,
    depth: usize,
    locs: &Locations,
    ad: &AssetData,
    positions: &HashMap<Guid, [f64; 3]>,
    want_pos: bool,
    no_ast: bool,
    counts: &mut Counts,
    seen: &mut HashSet<Guid>,
) {
    // Skip asteroid clutter (Lagrange fields + mining claims) when asked.
    if no_ast && format!("{:?}", loc.kind).contains("Asteroid") {
        return;
    }
    if !seen.insert(loc.guid) {
        return; // cycle guard
    }
    counts.total += 1;
    let kind = format!("{:?}", loc.kind);
    let name = loc
        .display_name(&ad.locale)
        .map(str::to_string)
        .unwrap_or_else(|| "<unnamed>".to_string());

    let marker = if want_pos {
        let e = counts.by_kind.entry(kind.clone()).or_default();
        e.1 += 1;
        if positions.contains_key(&loc.guid) {
            counts.positioned += 1;
            e.0 += 1;
            "● "
        } else {
            "○ "
        }
    } else {
        ""
    };

    println!("{}{marker}{name}  [{kind}]", "  ".repeat(depth));

    // Children, sorted by kind then name so bodies group above surface spots.
    let mut kids: Vec<&Location> = locs.children_of(&loc.guid).collect();
    kids.sort_by_key(|l| {
        (
            format!("{:?}", l.kind),
            l.display_name(&ad.locale).unwrap_or("").to_string(),
        )
    });
    for kid in kids {
        print_node(
            kid,
            depth + 1,
            locs,
            ad,
            positions,
            want_pos,
            no_ast,
            counts,
            seen,
        );
    }
}

/// Full socpak walk → guid → world Pos (entity-level `Pos` + `SObjectMetadataParams`
/// starmapRecord). Same harvest as `object_container_bench`, keyed by GUID.
fn harvest_positions(assets: &AssetSource) -> HashMap<Guid, [f64; 3]> {
    let t = Instant::now();
    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak") && l.contains("objectcontainers")
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("-> harvesting positions from {} socpaks…", socpaks.len());

    let mut out: HashMap<Guid, [f64; 3]> = HashMap::new();
    for sp in &socpaks {
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        // The nav-graph <Child> nodes (bodies/L-points) live in `.xml` members,
        // but only in the 3 system socpaks — decode those there, skip elsewhere
        // (decoding every socpak's `.xml` would explode parse time).
        let allow_xml = sp
            .to_ascii_lowercase()
            .replace('\\', "/")
            .contains("/system/");
        for m in 0..pak.len() {
            let mm = pak.name(m).unwrap_or_default().to_ascii_lowercase();
            let ok = mm.ends_with(".soc")
                || mm.ends_with(".pla")
                || mm.ends_with(".entxml")
                || (allow_xml && mm.ends_with(".xml"));
            if !ok {
                continue;
            }
            let Ok(b) = pak.read(m) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            // (a) Entity-level placements: <Entity Pos=…> + SObjectMetadataParams.
            for e in root.find_all("Entity") {
                let Some(pos) = e.attr("Pos").and_then(vec3) else {
                    continue;
                };
                // Search ALL SObjectMetadataParams (not just .next()) — the body's
                // own starmapRecord may sit on a later one; the first is often an
                // empty/child marker. This was the bug that hid Crusader/ArcCorp.
                let Some(sm) = e
                    .find_all("SObjectMetadataParams")
                    .find_map(|p| p.attr("starmapRecord"))
                    .filter(|s| !s.starts_with("00000000-"))
                else {
                    continue;
                };
                if let Ok(g) = sm.trim().trim_matches(['{', '}']).parse::<Guid>() {
                    out.entry(g).or_insert(pos);
                }
            }
            // (b) Nav-graph <Child> nodes (NOT nested under <Entity>): each a
            // body / Lagrange point / station carrying its own pos + starmapRecord.
            // This is where Crusader / ArcCorp / most moons live.
            for ch in root.find_all("Child") {
                let Some(pos) = ch.attr("pos").or_else(|| ch.attr("Pos")).and_then(vec3) else {
                    continue;
                };
                let Some(sm) = ch
                    .attr("starmapRecord")
                    .filter(|s| !s.starts_with("00000000-"))
                else {
                    continue;
                };
                if let Ok(g) = sm.trim().trim_matches(['{', '}']).parse::<Guid>() {
                    out.entry(g).or_insert(pos);
                }
            }
        }
    }
    eprintln!(
        "-> {} positioned guids in {:.1}s",
        out.len(),
        t.elapsed().as_secs_f64()
    );
    out
}

fn vec3(raw: &str) -> Option<[f64; 3]> {
    let p: Vec<f64> = raw
        .split([',', ' '])
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    (p.len() >= 3).then(|| [p[0], p[1], p[2]])
}
