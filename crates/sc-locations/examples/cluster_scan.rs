//! Scan socpaks for the per-location identity fields: entityClusterId,
//! entityClusterMember, actionArea, locationActionArea, starmapRecord, and
//! LocationOwner. Reports DISTINCT non-zero values per matched socpak so we can
//! see which (if any) carry a stable per-station id.
use std::collections::BTreeSet;

use sc_extract::AssetSource;
use sc_extract::object_container::{Socpak, decode};

const ZERO: &str = "00000000-0000-0000-0000-000000000000";

fn is_entity_file(n: &str) -> bool {
    let n = n.to_ascii_lowercase();
    n.ends_with(".soc") || n.ends_with(".pla") || n.ends_with(".entxml")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let needle = args.first().expect("needle").to_lowercase();
    let max_paks: usize = args
        .iter()
        .find_map(|a| a.strip_prefix("--max=").and_then(|s| s.parse().ok()))
        .unwrap_or(2);

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;

    let entries: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase();
            l.ends_with(".socpak") && l.contains(&needle)
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("matched {} socpaks for {needle:?}", entries.len());

    for path in entries.iter().take(max_paks) {
        let bytes = assets.read(path)?;
        let mut pak = match Socpak::open(bytes) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("open {path}: {e}");
                continue;
            }
        };
        let mut cluster_id: BTreeSet<String> = BTreeSet::new();
        let mut cluster_member: BTreeSet<String> = BTreeSet::new();
        let mut action_area: BTreeSet<String> = BTreeSet::new();
        let mut loc_action_area: BTreeSet<String> = BTreeSet::new();
        let mut starmap: BTreeSet<String> = BTreeSet::new();
        let mut loc_owner: BTreeSet<String> = BTreeSet::new();
        let mut tmpl: BTreeSet<String> = BTreeSet::new();
        let mut n_meta = 0usize;

        for i in 0..pak.len() {
            let name = pak.name(i).unwrap_or_default();
            if !is_entity_file(&name) {
                continue;
            }
            let Ok(b) = pak.read(i) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            for n in root.descendants() {
                let push = |set: &mut BTreeSet<String>, v: Option<&str>| {
                    if let Some(v) = v
                        && v != ZERO
                        && !v.is_empty()
                    {
                        set.insert(v.to_string());
                    }
                };
                match n.tag.as_str() {
                    "SObjectMetadataParams" | "missionLocation" => {
                        n_meta += 1;
                        push(&mut cluster_id, n.attr("entityClusterId"));
                        push(&mut action_area, n.attr("actionArea"));
                        push(&mut loc_action_area, n.attr("locationActionArea"));
                        push(&mut starmap, n.attr("starmapRecord"));
                        push(&mut tmpl, n.attr("template"));
                    }
                    "additionalData" => {
                        push(&mut cluster_member, n.attr("entityClusterMember"));
                    }
                    "EntityComponentBaseBuilding" => {
                        push(&mut loc_owner, n.attr("LocationOwner"));
                    }
                    _ => {}
                }
            }
        }
        println!("\n==== {path}  (meta nodes: {n_meta}) ====");
        let show = |label: &str, s: &BTreeSet<String>| {
            println!("  {label}: {} distinct", s.len());
            for v in s.iter().take(12) {
                println!("      {v}");
            }
        };
        show("entityClusterId (non-zero)", &cluster_id);
        show("entityClusterMember (non-zero)", &cluster_member);
        show("actionArea (non-zero)", &action_area);
        show("locationActionArea (non-zero)", &loc_action_area);
        show("LocationOwner (non-zero)", &loc_owner);
        show("starmapRecord", &starmap);
        show("template (MissionLocationTemplate)", &tmpl);
    }
    Ok(())
}
