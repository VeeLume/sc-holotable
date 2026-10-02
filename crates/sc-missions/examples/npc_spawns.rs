//! Prove the NPC spawn counts and character tags on [`NpcSlot`] against
//! live data: how many slots carry `AutoSpawnSettings`, whether the counts
//! agree with the designer-written `x N` in phase names, and where the
//! character tags sit in the tag tree (the subtrees a consumer classifies
//! archetype and faction by).
//!
//! ```bash
//! cargo run -p sc-missions --release --example npc_spawns
//! ```
use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, RecordCollection};
use sc_missions::{Encounter, Missions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let missions = Missions::build(&datacore);
    let tree = &missions.tag_tree;

    let (mut slots, mut with_counts) = (0usize, 0usize);
    let (mut named, mut agree) = (0usize, 0usize);
    let mut disagreements: BTreeMap<String, usize> = BTreeMap::new();
    let mut tag_paths: BTreeMap<String, usize> = BTreeMap::new();
    // (archetype or class, factions) → NPCs spawned, summed over max_spawns.
    let mut classified: BTreeMap<String, usize> = BTreeMap::new();
    for (_, m) in missions.iter() {
        for e in &m.encounters {
            let Encounter::Npcs(n) = e else { continue };
            for phase in &n.phases {
                let name_count = count_in_name(&phase.name);
                for opt in phase.all_options() {
                    slots += 1;
                    let Some(c) = opt.spawn_counts else { continue };
                    with_counts += 1;
                    if let Some(want) = name_count {
                        named += 1;
                        if c.max_spawns == want {
                            agree += 1;
                        } else {
                            *disagreements
                                .entry(format!(
                                    "{:<44} spawns {} (at once {})",
                                    phase.name, c.max_spawns, c.max_concurrent
                                ))
                                .or_default() += 1;
                        }
                    }
                    for (guid, _) in opt.character_tags.iter() {
                        *tag_paths.entry(tree.path(guid).join(" > ")).or_default() += 1;
                    }
                    // Archetype when tagged, else the NPC class.
                    let mut kind: Vec<&str> = opt.character_tags.archetypes(tree).collect();
                    if kind.is_empty() {
                        kind = opt.character_tags.npc_classes(tree).collect();
                    }
                    let factions: Vec<&str> = opt.character_tags.factions(tree).collect();
                    *classified
                        .entry(format!("{kind:?} {factions:?}"))
                        .or_default() += c.max_spawns as usize;
                }
            }
        }
    }

    println!("NPC slots across all missions: {slots}");
    println!("  with spawn counts:            {with_counts}");
    println!("  phase name carries `x N`:     {named}");
    println!("  max_spawns == N:              {agree}");
    println!("\nDisagreements (phase name vs counts):");
    for (k, n) in &disagreements {
        println!("  {n:>5}  {k}");
    }
    println!("\nCharacter tag paths:");
    let mut paths: Vec<_> = tag_paths.into_iter().collect();
    paths.sort_by_key(|e| std::cmp::Reverse(e.1));
    for (p, n) in paths.iter().take(60) {
        println!("  {n:>6}  {p}");
    }
    println!("\nNPCs spawned by (archetype or class, factions):");
    let mut classified: Vec<_> = classified.into_iter().collect();
    classified.sort_by_key(|e| std::cmp::Reverse(e.1));
    for (k, n) in classified.iter().take(30) {
        println!("  {n:>6}  {k}");
    }
    let unclassified: usize = classified
        .iter()
        .filter(|(k, _)| k.starts_with("[]"))
        .map(|(_, n)| n)
        .sum();
    println!("  NPCs with neither archetype nor class: {unclassified}");
    Ok(())
}

/// The `N` in a designer phase name like `Soldier x 3` / `CQC x 3 -Target`.
fn count_in_name(name: &str) -> Option<i32> {
    let (_, rest) = name.split_once(" x ")?;
    let digits: String = rest
        .trim_start()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}
