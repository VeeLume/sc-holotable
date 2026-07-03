//! Position investigation probe. Two questions in one datacore parse + a light
//! (system-socpak-only) walk:
//!
//!   1. **starmap.space GUID-join coverage** — how many of the community POI
//!      GUIDs resolve to our `StarMapObject` `Locations`, by kind + system. Tells
//!      us whether the GUID join is a viable oracle for a surface-POI error
//!      measurement.
//!   2. **Rotation-attribute sweep** — do body (planet/moon/star) placement
//!      entities carry ANY rotation / quaternion / angular attribute, or is
//!      planet spin simply absent from our data? Sweeps every attr key on the 3
//!      `*system.socpak` entities for rot/quat/ang, and dumps the full attribute
//!      set of the planet/moon entities.
//!
//! ```bash
//! cargo run -p sc-locations --release --example position_audit -- <starmap_pois.json>
//! ```

use std::collections::{BTreeMap, BTreeSet};

use sc_extract::object_container::{Socpak, XmlNode, decode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, RecordCollection};
use sc_locations::{LocationKind, Locations};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("warn").init();

    let json_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "starmap_pois.json".to_string());

    let install = sc_discovery::discover_primary()?;
    println!("-> {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let ad = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &ad)?;
    let locs = Locations::build(datacore.records());
    println!("-> {} StarMapObject locations\n", locs.len());

    coverage(&json_path, &locs, &ad, &datacore)?;
    rotation_sweep(&assets, &locs)?;
    Ok(())
}

// ── 1. starmap.space GUID-join coverage ─────────────────────────────────────

fn coverage(
    json_path: &str,
    locs: &Locations,
    ad: &AssetData,
    datacore: &sc_extract::Datacore,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = match std::fs::read(json_path) {
        Ok(b) => b,
        Err(e) => {
            println!("!! could not read {json_path}: {e} — skipping coverage");
            return Ok(());
        }
    };
    let pois: serde_json::Value = serde_json::from_slice(&bytes)?;
    let arr = pois.as_array().cloned().unwrap_or_default();
    println!(
        "═══ starmap.space GUID-join coverage ({} POIs) ═══",
        arr.len()
    );

    let mut with_guid = 0usize;
    let mut matched = 0usize;
    let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_system_unmatched: BTreeMap<String, usize> = BTreeMap::new();
    let mut unmatched_types: BTreeMap<String, usize> = BTreeMap::new();
    let mut unmatched_samples: Vec<String> = Vec::new();
    let mut matched_samples: Vec<String> = Vec::new();

    for p in &arr {
        let guid_str = p.get("GUID").and_then(|v| v.as_str()).unwrap_or("");
        if guid_str.is_empty() {
            continue;
        }
        with_guid += 1;
        let sys = p.get("System").and_then(|v| v.as_str()).unwrap_or("?");
        let name = p.get("PoiName").and_then(|v| v.as_str()).unwrap_or("?");
        let Ok(guid) = guid_str.parse::<Guid>() else {
            *by_system_unmatched
                .entry(format!("{sys} (bad-guid)"))
                .or_default() += 1;
            continue;
        };
        match locs.get(&guid) {
            Some(l) => {
                matched += 1;
                *by_kind.entry(format!("{:?}", l.kind)).or_default() += 1;
                if matched_samples.len() < 8 {
                    let disp = l.display_name(&ad.locale).unwrap_or("<no name>");
                    matched_samples.push(format!("  {name:<30} → {:?}  \"{disp}\"", l.kind));
                }
            }
            None => {
                *by_system_unmatched.entry(sys.to_string()).or_default() += 1;
                // What IS this GUID in the raw DCB, if anything?
                let ty = datacore
                    .db()
                    .record(&guid)
                    .and_then(|r| r.type_name().map(str::to_string))
                    .unwrap_or_else(|| "<not a DCB record>".to_string());
                *unmatched_types.entry(ty).or_default() += 1;
                if unmatched_samples.len() < 8 {
                    unmatched_samples.push(format!("  {name:<30} guid={guid_str}"));
                }
            }
        }
    }

    println!("  POIs with a GUID        : {with_guid}");
    println!(
        "  resolve to a Location   : {matched}  ({:.0}%)",
        100.0 * matched as f64 / with_guid.max(1) as f64
    );
    println!("  matched by kind         : {by_kind:?}");
    println!("  unmatched by system     : {by_system_unmatched:?}");
    println!("  unmatched GUID → DCB record type:");
    for (t, n) in &unmatched_types {
        println!("    {n:>6}  {t}");
    }
    println!("  — matched samples —");
    for s in &matched_samples {
        println!("{s}");
    }
    println!("  — unmatched samples —");
    for s in &unmatched_samples {
        println!("{s}");
    }
    println!();
    Ok(())
}

// ── 2. rotation-attribute sweep on body entities ────────────────────────────

fn rotation_sweep(
    assets: &AssetSource,
    locs: &Locations,
) -> Result<(), Box<dyn std::error::Error>> {
    let socpaks: Vec<String> = assets
        .find(|n| {
            n.to_ascii_lowercase()
                .replace('\\', "/")
                .ends_with("system.socpak")
        })
        .map(|e| e.name.to_string())
        .collect();
    println!(
        "═══ rotation-attribute sweep ({} *system.socpak) ═══",
        socpaks.len()
    );

    // Every attr key (tag::key) across all entities matching rot/quat/ang.
    let mut rot_keys: BTreeMap<String, usize> = BTreeMap::new();
    // Full attribute dumps for body entities (planet/moon/star).
    let mut body_dumps: Vec<String> = Vec::new();
    let body_kinds = [
        LocationKind::Planet,
        LocationKind::Moon,
        LocationKind::Star,
        LocationKind::S42Planet,
        LocationKind::S42Moon,
    ];

    for sp in &socpaks {
        let allow_xml = sp
            .to_ascii_lowercase()
            .replace('\\', "/")
            .contains("/system/");
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        for m in 0..pak.len() {
            let member = pak.name(m).unwrap_or_default().to_ascii_lowercase();
            let ok = member.ends_with(".soc")
                || member.ends_with(".pla")
                || member.ends_with(".entxml")
                || (allow_xml && member.ends_with(".xml"));
            if !ok {
                continue;
            }
            let Ok(b) = pak.read(m) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };

            for e in root.find_all("Entity") {
                // rotation-key histogram over the whole entity subtree
                for n in e.descendants() {
                    for (k, _) in &n.attrs {
                        let kl = k.to_ascii_lowercase();
                        if kl.contains("rot") || kl.contains("quat") || kl.contains("ang") {
                            *rot_keys.entry(format!("{}::{k}", n.tag)).or_default() += 1;
                        }
                    }
                }
                // is this a body? resolve starmapRecord → kind
                let sr = e
                    .descendants()
                    .find_map(|n| n.attr("starmapRecord"))
                    .map(|s| s.trim().trim_matches(['{', '}']).to_string());
                let is_body = sr
                    .as_deref()
                    .and_then(|s| s.parse::<Guid>().ok())
                    .and_then(|g| locs.get(&g))
                    .map(|l| body_kinds.contains(&l.kind))
                    .unwrap_or(false);
                if is_body && body_dumps.len() < 6 {
                    body_dumps.push(dump_entity_attrs(e));
                }
            }
        }
    }

    println!("  attr keys matching rot/quat/ang (tag::key → count):");
    if rot_keys.is_empty() {
        println!("    <NONE — no rotation/quaternion/angular attribute on any system-OC entity>");
    } else {
        for (k, n) in &rot_keys {
            println!("    {n:>6}  {k}");
        }
    }
    println!("\n  body (planet/moon/star) entity attribute dumps:");
    for d in &body_dumps {
        println!("{d}");
    }
    Ok(())
}

/// Dump the Entity node's own attrs + the tag/attrs of every component child,
/// so we see the full transform surface (Pos, RelativePos, Rot?, orbit, …).
fn dump_entity_attrs(e: &XmlNode) -> String {
    let mut out = String::new();
    let name = e.attr("Name").unwrap_or("");
    let class = e.attr("EntityClass").unwrap_or("");
    out.push_str(&format!("  ── Entity [{class}] Name={name}\n"));
    // Entity-level attrs (skip the two long GUIDs for brevity)
    let mut ekeys: Vec<&(String, String)> = e
        .attrs
        .iter()
        .filter(|(k, _)| k != "EntityClassGUID")
        .collect();
    ekeys.sort_by(|a, b| a.0.cmp(&b.0));
    for (k, v) in ekeys {
        let v = if v.len() > 70 { &v[..70] } else { v.as_str() };
        out.push_str(&format!("       @{k} = {v}\n"));
    }
    // Component children: tag + any attr keys they carry
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for n in e.descendants() {
        if n.tag == "Entity" {
            continue;
        }
        let keys: Vec<&str> = n.attrs.iter().map(|(k, _)| k.as_str()).collect();
        if keys.is_empty() {
            continue;
        }
        let line = format!("       <{}> [{}]", n.tag, keys.join(", "));
        if seen.insert(line.clone()) {
            out.push_str(&line);
            out.push('\n');
        }
    }
    out
}
