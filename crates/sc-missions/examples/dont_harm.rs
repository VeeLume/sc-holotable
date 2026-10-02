//! Census of the `DontHarm*` mission flags (the crimestat signal) as
//! resolved by [`Mission::integer_properties`] across all override layers,
//! next to who the mission actually spawns (allied-marked / hostile NPCs).
//! Optional title filters print matching missions in detail.
//!
//! ```bash
//! cargo run -p sc-missions --release --example dont_harm -- "Reprisal" "Secure Site"
//! ```
use std::collections::{BTreeMap, BTreeSet};

use sc_extract::{AssetConfig, AssetData, AssetSource, RecordCollection};
use sc_missions::{Encounter, Mission, Missions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filters: Vec<String> = std::env::args().skip(1).collect();
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let missions = Missions::build(&datacore);
    let tree = &missions.tag_tree;

    // (flags, allied NPC slots > 0, hostile NPC slots > 0) → missions
    let mut census: BTreeMap<String, usize> = BTreeMap::new();
    let mut printed: BTreeSet<String> = BTreeSet::new();
    for (_, m) in missions.iter() {
        let flags = dont_harm(m);
        let (allied, hostile) = npc_sides(m);
        *census
            .entry(format!(
                "{flags:<48} allied={:<5} hostile={}",
                allied > 0,
                hostile > 0
            ))
            .or_default() += 1;

        let title = missions.title_text(m, locale).unwrap_or_default();
        // One detail print per generator contract (expansions repeat it).
        if filters.iter().any(|f| title.contains(f.as_str()))
            && printed.insert(m.debug_name.clone())
        {
            println!("== {title}  [{}]", m.debug_name);
            println!("   flags: {flags}");
            for e in &m.encounters {
                let Encounter::Npcs(n) = e else { continue };
                for phase in &n.phases {
                    for o in phase.all_options() {
                        let factions: Vec<&str> = o.character_tags.factions(tree).collect();
                        println!(
                            "   {:<32} {:<26} allied={} x{}",
                            n.variable_name,
                            factions.join("+"),
                            o.mission_allied_marker,
                            o.spawn_counts.map_or(1, |c| c.max_spawns)
                        );
                    }
                }
            }
        }
    }

    println!("\nDontHarm flags vs NPC sides (all missions):");
    for (k, n) in &census {
        println!("  {n:>5}  {k}");
    }
    Ok(())
}

/// `DontHarm*` variables and their option sets, e.g. `DontHarmCivs_BP=[1]`.
fn dont_harm(m: &Mission) -> String {
    let parts: Vec<String> = m
        .integer_properties
        .iter()
        .filter(|(k, _)| k.contains("DontHarm"))
        .map(|(k, v)| format!("{k}={v:?}"))
        .collect();
    if parts.is_empty() {
        "-".into()
    } else {
        parts.join(" ")
    }
}

/// (allied-marked, unmarked) NPC slot counts.
fn npc_sides(m: &Mission) -> (usize, usize) {
    let mut sides = (0, 0);
    for e in &m.encounters {
        let Encounter::Npcs(n) = e else { continue };
        for o in n.phases.iter().flat_map(|p| p.all_options()) {
            if o.mission_allied_marker {
                sides.0 += 1;
            } else {
                sides.1 += 1;
            }
        }
    }
    sides
}
