//! Spike: can we build our own `objContainers`-equivalent (celestial-body +
//! station positions) straight from the live install, so a cargo-route advisor
//! can order drop-offs by distance without vendoring anyone's static JSON?
//!
//! What it does:
//!   1. Discover the primary install, parse the DataCore, build `Locations`
//!      (StarMapObject → human name), load the locale.
//!   2. Walk every `*system.socpak` for `OrbitingObjectContainer` placements —
//!      the same bridge entities `LocationContainers::cook` harvests — but also
//!      pull the *positional* half the v1 binding defers: `Pos`, `OrbitalRadius`,
//!      `OrbitalAngle`, `parentGUID`.
//!   3. Resolve each placement's `starmapRecord` → location name + kind, so we
//!      see how many mission-relevant places resolve with real coordinates.
//!   4. Demonstrate the payoff: a pairwise-distance "oracle" over the positioned
//!      places (nearest-neighbour from a seed) — the core a route advisor needs.
//!
//! Because the exact on-entity XML shape of the positional fields isn't
//! documented, the first few entities are dumped verbatim (`--dump`) so we can
//! confirm field names/format against live data instead of guessing.
//!
//! Usage:
//!   cargo run -p sc-locations --example objcontainer_positions --release
//!   cargo run -p sc-locations --example objcontainer_positions --release -- --dump
//!   cargo run -p sc-locations --example objcontainer_positions --release -- <p4k>
//!   cargo run -p sc-locations --example objcontainer_positions --release -- --grep tressler

use std::collections::HashSet;

use sc_extract::object_container::{Socpak, XmlNode, decode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, RecordCollection};
use sc_locations::Locations;

/// One placed object container with whatever positional data we could pull.
#[allow(dead_code)] // orbital_angle harvested for completeness; not read in this spike.
struct Placement {
    starmap: Guid,
    socpak: String,
    pos: Option<[f64; 3]>,
    orbital_radius: Option<f64>,
    orbital_angle: Option<f64>,
    parent_guid: Option<String>,
    /// Resolved human name (via Locations + locale), if any.
    name: Option<String>,
    kind: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let dump = args.iter().any(|a| a == "--dump");
    let diag = args.iter().any(|a| a == "--diag");
    let recurse = args.iter().any(|a| a == "--recurse");
    let grep = args
        .iter()
        .position(|a| a == "--grep")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_ascii_lowercase());
    let explicit_p4k = args
        .iter()
        .find(|a| !a.starts_with('-') && Some(*a) != grep.as_ref().map(|_| &args[0]))
        .filter(|a| a.ends_with(".p4k"));

    // ── open assets ──────────────────────────────────────────────────────
    let assets = if let Some(p4k) = explicit_p4k {
        println!("-> opening {p4k}");
        AssetSource::open(std::path::Path::new(p4k))?
    } else {
        let install = sc_discovery::discover_primary()?;
        println!(
            "-> found {} v{} at {}",
            install.channel,
            install.short_version(),
            install.root.display()
        );
        AssetSource::from_install(&install)?
    };

    // ── locale + datacore + locations ────────────────────────────────────
    println!("-> extracting locale");
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    println!("-> parsing datacore");
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let locations = Locations::build(store);
    println!(
        "-> {} StarMapObject locations, {} locale entries",
        locations.len(),
        asset_data.locale.len()
    );

    // ── walk the system socpaks ──────────────────────────────────────────
    let socpaks: Vec<String> = assets
        .find(|name| name.to_ascii_lowercase().ends_with("system.socpak"))
        .map(|e| e.name.to_string())
        .collect();
    println!("-> {} *system.socpak: {:?}\n", socpaks.len(), socpaks);

    if diag {
        diagnose(&assets, &socpaks)?;
        return Ok(());
    }

    let mut placements: Vec<Placement> = Vec::new();
    let mut dumped = 0usize;

    for socpak in &socpaks {
        let bytes = assets.read(socpak)?;
        let mut pak = match Socpak::open(bytes) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("  skip {socpak}: {e}");
                continue;
            }
        };
        for i in 0..pak.len() {
            let Some(member) = pak.name(i) else { continue };
            if !member.to_ascii_lowercase().ends_with(".entxml") {
                continue;
            }
            let Ok(bytes) = pak.read(i) else { continue };
            let Ok(Some(root)) = decode(&bytes) else {
                continue;
            };

            for entity in root
                .find_all("Entity")
                .filter(|e| e.attr("EntityClass") == Some("OrbitingObjectContainer"))
            {
                let container = entity
                    .find_all("EntityComponentObjectContainer")
                    .next()
                    .and_then(|c| c.attr("objectContainer"));
                // 4.8: metadata component renamed SNavPointObjectMetadataParams
                // → SObjectMetadataParams (this is why v1 harvested 0).
                let starmap = entity
                    .find_all("SObjectMetadataParams")
                    .next()
                    .and_then(|c| c.attr("starmapRecord"));

                let (Some(container), Some(starmap)) = (container, starmap) else {
                    continue;
                };
                let starmap = strip_braces(starmap);
                if starmap.is_empty() || starmap.starts_with("00000000-") {
                    continue;
                }
                let Ok(guid) = starmap.parse::<Guid>() else {
                    continue;
                };

                // First few entities: dump the raw shape so we can confirm
                // where the positional fields actually live.
                if dump && dumped < 4 {
                    println!("──── raw OrbitingObjectContainer #{dumped} ({member}) ────");
                    dump_positional(entity, 0);
                    println!();
                    dumped += 1;
                }

                let loc = locations.get(&guid);
                placements.push(Placement {
                    starmap: guid,
                    socpak: normalize(container),
                    pos: find_vec3(entity, "pos"),
                    orbital_radius: find_f64(entity, "orbitalradius"),
                    orbital_angle: find_f64(entity, "orbitalangle"),
                    parent_guid: find_attr(entity, "parentguid").map(str::to_string),
                    name: loc.and_then(|l| l.display_name(&asset_data.locale).map(str::to_string)),
                    kind: loc.map(|l| format!("{:?}", l.kind)).unwrap_or_default(),
                });
            }
        }
    }

    report(&placements, grep.as_deref());

    if recurse {
        recurse_probe(&assets, &asset_data, &locations, &placements)?;
    }
    if args.iter().any(|a| a == "--parents") {
        parent_fallback_probe(&asset_data, &locations, &placements);
    }
    Ok(())
}

/// Prove the ordering fallback: a mission station we *can't* position directly
/// (Everus Harbor, Port Tressler, …) inherits its parent **body's** position
/// via the StarMapObject hierarchy — enough to order deliveries by which
/// planet/moon they sit at.
fn parent_fallback_probe(asset_data: &AssetData, locations: &Locations, positioned: &[Placement]) {
    use std::collections::HashMap;
    // guid → position, from the system-level harvest.
    let pos_by_guid: HashMap<Guid, [f64; 3]> = positioned
        .iter()
        .filter_map(|p| p.pos.map(|v| (p.starmap, v)))
        .collect();

    // name (lowercased) → guid, over ALL locations.
    let mut guid_by_name: HashMap<String, Guid> = HashMap::new();
    for (guid, loc) in locations.iter() {
        if let Some(n) = loc.display_name(&asset_data.locale) {
            guid_by_name.insert(n.to_ascii_lowercase(), *guid);
        }
    }

    let targets = [
        "Everus Harbor",
        "Baijini Point",
        "Port Tressler",
        "Seraphim Station",
        "Port Olisar",
        "Grim HEX",
        "CRU-L1 Ambitious Dream Station",
        "Checkmate",
        "Ruin Station",
        "Orbituary",
    ];

    println!("\n═══ PARENT-BODY FALLBACK (position mission stations by their body) ═══");
    for t in targets {
        let Some(guid) = guid_by_name.get(&t.to_ascii_lowercase()) else {
            println!("  {t:<32} : not a StarMapObject location");
            continue;
        };
        // Self first, then ancestors, until we hit a positioned body.
        let mut chain = vec![*guid];
        chain.extend(locations.ancestors(guid).map(|l| l.guid));
        let hit = chain.iter().find_map(|g| {
            pos_by_guid.get(g).map(|p| {
                let name = locations
                    .get(g)
                    .and_then(|l| l.display_name(&asset_data.locale))
                    .unwrap_or("?");
                (name.to_string(), *p)
            })
        });
        match hit {
            Some((body, [x, y, z])) => {
                println!("  {t:<32} → {body:<14} [{x:.0}, {y:.0}, {z:.0}]")
            }
            None => println!("  {t:<32} : found, but no positioned ancestor"),
        }
    }
}

/// One level deeper: open each *body* socpak referenced by a top-level
/// placement and harvest the nested `OrbitingObjectContainer`s (the reststops /
/// harbors / distribution centres that live *inside* a planet/moon OC). Proves
/// whether the mission stations the cargo planner cares about are recoverable.
fn recurse_probe(
    assets: &AssetSource,
    asset_data: &AssetData,
    locations: &Locations,
    top: &[Placement],
) -> Result<(), Box<dyn std::error::Error>> {
    // Unique body socpaks referenced by top-level placements.
    let mut bodies: Vec<String> = top.iter().map(|p| p.socpak.clone()).collect();
    bodies.sort();
    bodies.dedup();

    let mut nested: Vec<(String, String, Option<[f64; 3]>)> = Vec::new(); // (name, kind, pos)
    let mut opened = 0usize;

    for body in &bodies {
        let base = body.rsplit('/').next().unwrap_or(body).to_string();
        // Resolve the real archive entry by basename (paths differ in case/prefix).
        let Some(entry) = assets
            .find(|n| n.to_ascii_lowercase().ends_with(&base))
            .map(|e| e.name.to_string())
            .next()
        else {
            continue;
        };
        let Ok(bytes) = assets.read(&entry) else {
            continue;
        };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        opened += 1;
        for i in 0..pak.len() {
            let Some(member) = pak.name(i) else { continue };
            if !member.to_ascii_lowercase().ends_with(".entxml") {
                continue;
            }
            let Ok(bytes) = pak.read(i) else { continue };
            let Ok(Some(root)) = decode(&bytes) else {
                continue;
            };
            for entity in root
                .find_all("Entity")
                .filter(|e| e.attr("EntityClass") == Some("OrbitingObjectContainer"))
            {
                let Some(starmap) = entity
                    .find_all("SObjectMetadataParams")
                    .next()
                    .and_then(|c| c.attr("starmapRecord"))
                else {
                    continue;
                };
                let starmap = strip_braces(starmap);
                if starmap.is_empty() || starmap.starts_with("00000000-") {
                    continue;
                }
                let Ok(guid) = starmap.parse::<Guid>() else {
                    continue;
                };
                let Some(loc) = locations.get(&guid) else {
                    continue;
                };
                let Some(name) = loc.display_name(&asset_data.locale) else {
                    continue;
                };
                nested.push((
                    name.to_string(),
                    format!("{:?}", loc.kind),
                    find_vec3(entity, "pos"),
                ));
            }
        }
    }

    nested.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    nested.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    println!("\n═══ RECURSION PROBE (one level into body OCs) ═══");
    println!("  body socpaks opened   : {opened}");
    println!("  nested named places   : {}", nested.len());
    println!();
    for (name, kind, pos) in &nested {
        let pos = match pos {
            Some([x, y, z]) => format!("[{x:.0}, {y:.0}, {z:.0}]"),
            None => "<no pos>".to_string(),
        };
        println!("  {name:<40} {kind:<20} {pos}");
    }
    Ok(())
}

/// Profile the real 4.8 socpak structure: member extensions, entity classes,
/// and which attribute keys carry positional / starmap data. Run once to learn
/// the shape after a format change.
fn diagnose(assets: &AssetSource, socpaks: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::BTreeMap;

    // Focus on the first (stanton) socpak — representative and biggest.
    let Some(socpak) = socpaks.first() else {
        return Ok(());
    };
    println!("═══ DIAGNOSE {socpak} ═══");
    let bytes = assets.read(socpak)?;
    let mut pak = Socpak::open(bytes)?;

    // 1. Member extension histogram.
    let mut ext: BTreeMap<String, usize> = BTreeMap::new();
    let mut members: Vec<(usize, String)> = Vec::new();
    for i in 0..pak.len() {
        let Some(name) = pak.name(i) else { continue };
        let e = name.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
        *ext.entry(e).or_default() += 1;
        members.push((i, name));
    }
    println!("  members: {} total", members.len());
    for (e, n) in &ext {
        println!("    .{e:<10} {n}");
    }

    // 2. Decode every member; aggregate entity classes + attr keys.
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();
    let mut poskeys: BTreeMap<String, usize> = BTreeMap::new();
    let mut starmap_carriers: BTreeMap<String, usize> = BTreeMap::new();
    let mut sample_shown = 0usize;

    for (i, name) in &members {
        let Ok(bytes) = pak.read(*i) else { continue };
        let Ok(Some(root)) = decode(&bytes) else {
            continue;
        };
        for node in root.descendants() {
            if node.tag == "Entity" {
                if let Some(cls) = node.attr("EntityClass") {
                    *classes.entry(cls.to_string()).or_default() += 1;
                }
            }
            for (k, v) in &node.attrs {
                let kl = k.to_ascii_lowercase();
                if kl.contains("pos") || kl.contains("orbit") {
                    *poskeys.entry(format!("{}::{k}", node.tag)).or_default() += 1;
                }
                if kl.contains("starmap") && !v.is_empty() && !v.starts_with("00000000-") {
                    *starmap_carriers.entry(node.tag.clone()).or_default() += 1;
                    // Show a couple of full entities that carry a starmapRecord.
                    if sample_shown < 3 {
                        println!("\n  ── entity carrying starmapRecord (member {name}) ──");
                        dump_positional(&root, 0);
                        sample_shown += 1;
                    }
                }
            }
        }
    }

    println!("\n  EntityClass histogram (top 25):");
    let mut cls: Vec<(&String, &usize)> = classes.iter().collect();
    cls.sort_by(|a, b| b.1.cmp(a.1));
    for (c, n) in cls.iter().take(25) {
        println!("    {n:>6}  {c}");
    }

    println!("\n  attr keys containing pos/orbit (tag::key → count):");
    let mut pk: Vec<(&String, &usize)> = poskeys.iter().collect();
    pk.sort_by(|a, b| b.1.cmp(a.1));
    for (k, n) in pk.iter().take(30) {
        println!("    {n:>6}  {k}");
    }

    println!("\n  tags carrying a non-null starmapRecord:");
    for (t, n) in &starmap_carriers {
        println!("    {n:>6}  {t}");
    }
    Ok(())
}

/// Recursively print any node/attr that looks positional — used once to learn
/// the real field shape from live data.
fn dump_positional(node: &XmlNode, depth: usize) {
    let pad = "  ".repeat(depth);
    let hot: Vec<&(String, String)> = node
        .attrs
        .iter()
        .filter(|(k, _)| {
            let k = k.to_ascii_lowercase();
            k.contains("pos")
                || k.contains("orbit")
                || k.contains("parent")
                || k.contains("rot")
                || k == "name"
                || k == "guid"
        })
        .collect();
    if !hot.is_empty()
        || node.tag.contains("Metadata")
        || node.tag.contains("Orbit")
        || node.tag == "Child"
    {
        println!("{pad}<{}>", node.tag);
        for (k, v) in hot {
            let v = if v.len() > 80 { &v[..80] } else { v.as_str() };
            println!("{pad}    {k} = {v}");
        }
    }
    for child in &node.children {
        dump_positional(child, depth + 1);
    }
}

/// First descendant attribute whose lowercased key matches exactly.
fn find_attr<'a>(node: &'a XmlNode, key_lc: &str) -> Option<&'a str> {
    node.descendants().find_map(|n| {
        n.attrs
            .iter()
            .find(|(k, _)| k.to_ascii_lowercase() == key_lc)
            .map(|(_, v)| v.as_str())
    })
}

fn find_f64(node: &XmlNode, key_lc: &str) -> Option<f64> {
    find_attr(node, key_lc).and_then(|v| v.trim().parse().ok())
}

/// Parse a `Pos`-style attribute. Handles `"x,y,z"` and space-separated forms.
fn find_vec3(node: &XmlNode, key_lc: &str) -> Option<[f64; 3]> {
    let raw = find_attr(node, key_lc)?;
    let parts: Vec<f64> = raw
        .split(|c| c == ',' || c == ' ')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    (parts.len() >= 3).then(|| [parts[0], parts[1], parts[2]])
}

/// 4.8 GUIDs come brace-wrapped (`{6A035D6B-…}`); strip them for `Guid::parse`.
fn strip_braces(s: &str) -> &str {
    s.trim().trim_start_matches('{').trim_end_matches('}')
}

fn normalize(raw: &str) -> String {
    let s = raw.replace('\\', "/").to_ascii_lowercase();
    s.strip_prefix("data/").unwrap_or(&s).to_string()
}

fn report(placements: &[Placement], grep: Option<&str>) {
    let total = placements.len();
    let named = placements.iter().filter(|p| p.name.is_some()).count();
    let positioned = placements.iter().filter(|p| p.pos.is_some()).count();
    let named_and_positioned = placements
        .iter()
        .filter(|p| p.name.is_some() && p.pos.is_some())
        .count();

    println!("═══ SUMMARY ═══");
    println!("  placements found      : {total}");
    println!("  resolved to a name    : {named}");
    println!("  have a Pos vector     : {positioned}");
    println!("  named AND positioned  : {named_and_positioned}");
    println!();

    // Distinct kinds seen (so we know if stations show up at system level).
    let kinds: HashSet<&str> = placements.iter().map(|p| p.kind.as_str()).collect();
    println!("  kinds seen            : {kinds:?}");
    println!();

    println!("═══ PLACEMENTS (name — kind — Pos — orbitalRadius) ═══");
    let mut rows: Vec<&Placement> = placements
        .iter()
        .filter(|p| match grep {
            Some(g) => p
                .name
                .as_deref()
                .map(|n| n.to_ascii_lowercase().contains(g))
                .unwrap_or(false),
            None => true,
        })
        .collect();
    rows.sort_by(|a, b| a.name.cmp(&b.name));

    for p in &rows {
        let name = p.name.as_deref().unwrap_or("<unresolved>");
        let pos = match p.pos {
            Some([x, y, z]) => format!("[{x:.0}, {y:.0}, {z:.0}]"),
            None => "<no pos>".to_string(),
        };
        let orb = p
            .orbital_radius
            .map(|r| format!("r={r:.0}"))
            .unwrap_or_default();
        println!(
            "  {name:<38} {:<16} {pos:<40} {orb}  ({})",
            p.kind,
            short_socpak(&p.socpak)
        );
        if p.name.is_none() {
            println!("      starmap={}  parent={:?}", p.starmap, p.parent_guid);
        }
    }
    println!();

    // ── distance oracle demo ────────────────────────────────────────────
    let positioned: Vec<&Placement> = placements
        .iter()
        .filter(|p| p.pos.is_some() && p.name.is_some())
        .collect();
    if positioned.len() >= 2 {
        let seed = positioned[0];
        let sp = seed.pos.unwrap();
        let mut by_dist: Vec<(&Placement, f64)> = positioned
            .iter()
            .skip(1)
            .map(|p| {
                let q = p.pos.unwrap();
                let d = ((sp[0] - q[0]).powi(2) + (sp[1] - q[1]).powi(2) + (sp[2] - q[2]).powi(2))
                    .sqrt();
                (*p, d)
            })
            .collect();
        by_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        println!(
            "═══ DISTANCE ORACLE — nearest to {:?} ═══",
            seed.name.as_deref().unwrap_or("?")
        );
        for (p, d) in by_dist.iter().take(12) {
            println!(
                "  {:>12.0} km   {}",
                d / 1000.0,
                p.name.as_deref().unwrap_or("?")
            );
        }
    }
}

fn short_socpak(s: &str) -> String {
    s.rsplit('/').next().unwrap_or(s).to_string()
}
