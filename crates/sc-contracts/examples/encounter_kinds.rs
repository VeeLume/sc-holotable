//! Smoke test for the v2 phase 6 Encounter enum + NPC/Entity widening.
//! Counts encounters by variant and surfaces a few sample
//! NPC-with-allied-marker and Entity slots so we can verify the new
//! shape against live data.

use sc_contracts::{Encounter, MissionIndex};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);

    let mut ships = 0usize;
    let mut npcs = 0usize;
    let mut entities = 0usize;
    let mut unknown = 0usize;

    let mut npc_phases = 0usize;
    let mut npc_slots = 0usize;
    let mut npc_with_allied = 0usize;
    let mut npc_critical = 0usize;
    let mut npc_with_faction_override = 0usize;

    let mut entity_phases = 0usize;
    let mut entity_slots = 0usize;

    let mut sample_npc_allied: Vec<String> = Vec::new();
    let mut sample_entity: Vec<String> = Vec::new();

    for c in &index.contracts {
        for enc in &c.encounters {
            match enc {
                Encounter::Ships(_) => ships += 1,
                Encounter::Npcs(s) => {
                    npcs += 1;
                    npc_phases += s.phases.len();
                    for phase in &s.phases {
                        npc_slots += phase.slots.len();
                        for slot in &phase.slots {
                            if slot.mission_allied_marker {
                                npc_with_allied += 1;
                                if sample_npc_allied.len() < 6 {
                                    sample_npc_allied.push(format!(
                                        "{} → {} ({} ident_tags)",
                                        c.debug_name,
                                        s.variable_name,
                                        slot.identifier_tags.len(),
                                    ));
                                }
                            }
                            if slot.is_critical {
                                npc_critical += 1;
                            }
                            if slot.faction_override.is_some() {
                                npc_with_faction_override += 1;
                            }
                        }
                    }
                }
                Encounter::Entities(s) => {
                    entities += 1;
                    entity_phases += s.phases.len();
                    for phase in &s.phases {
                        entity_slots += phase.slots.len();
                        if sample_entity.len() < 4 && !phase.slots.is_empty() {
                            sample_entity.push(format!(
                                "{} → {} (phase \"{}\", {} slot(s))",
                                c.debug_name,
                                s.variable_name,
                                phase.name,
                                phase.slots.len(),
                            ));
                        }
                    }
                }
                Encounter::Unknown { .. } => unknown += 1,
            }
        }
    }

    println!("=== Encounter variant census ===");
    println!("missions:                  {}", index.contracts.len());
    println!("Encounter::Ships:          {ships}");
    println!("Encounter::Npcs:           {npcs}  ({npc_phases} phases, {npc_slots} slots)");
    println!("Encounter::Entities:       {entities}  ({entity_phases} phases, {entity_slots} slots)");
    println!("Encounter::Unknown:        {unknown}");
    println!();
    println!("=== NPC slot signal coverage ===");
    println!("  mission_allied_marker:   {npc_with_allied}/{npc_slots}");
    println!("  is_critical:             {npc_critical}/{npc_slots}");
    println!("  faction_override (some): {npc_with_faction_override}/{npc_slots}");
    println!();
    println!("=== Sample NPC-allied-marker spawns ===");
    for s in &sample_npc_allied {
        println!("  {s}");
    }
    println!();
    println!("=== Sample Entity encounters ===");
    for s in &sample_entity {
        println!("  {s}");
    }
    Ok(())
}
