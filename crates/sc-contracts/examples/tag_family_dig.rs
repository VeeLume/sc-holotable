//! Dump TagTree paths for a hand-picked set of "discriminator" tags so
//! we can see whether the tree's own hierarchy gives us a natural
//! axis-classification (scaling vs flavor) without hardcoding.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example tag_family_dig
//! ```

use sc_contracts::MissionIndex;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let _index = MissionIndex::build(&datacore);
    let tree = &datacore.snapshot().tag_tree;

    // Tags that appeared as alternatives discriminators in our census.
    // For each, show:
    //   - the full path in the tag tree (e.g., AI / Skills / HumanPilot30)
    //   - the root family (first segment)
    let probes = [
        "HumanPilot10",
        "HumanPilot30",
        "HumanPilot50",
        "HumanPilot70",
        "HumanPilot90",
        "VeryEasy",
        "Easy",
        "Medium",
        "Hard",
        "VeryHard",
        "Super",
        "Distortion",
        "CombatShip",
        "LargeCombatShip",
        "HeavyInterceptor",
        "MediumInterceptor",
        "Defenders",
        "Target",
        "Bounty",
        "Criminal",
        "Vanduul",
        "Civilians",
        "XenoThreat",
        "Ninetails",
        "Scythe",
        "Blade",
        "Avenger_Titan",
        "135c",
        "315p",
        "Cutlass_Black",
        "Freelancer_MIS",
        "RAFT",
        "Mantis",
        "Scorpius",
        "Scorpius_Antares",
        "Caterpillar",
        "StarLifter_C2",
        "Reclaimer",
        "Starfarer",
        "Full Cargo",
        "Half Cargo",
        "Scraps Cargo",
        "LowValue",
        "MediumValue",
        "HighValue",
        "Mixed",
        "PoweredOff",
        "EngineOff",
        "PU_Human_Criminal_Light_wTurret",
        "ArriveViaQT",
        "DefendShip",
        "General",
    ];

    println!("\n=== Tag-tree paths for census discriminators ===");
    println!("{:>32}  {:>14}  full path", "tag", "root family");
    println!("{}", "─".repeat(80));
    for name in probes {
        let guids = tree.by_name(name);
        if guids.is_empty() {
            println!("{name:>32}  {:>14}  <not in tree>", "—");
            continue;
        }
        for g in guids {
            let path = tree.path(g);
            let root = path.first().copied().unwrap_or("?");
            println!("{name:>32}  {root:>14}  {}", path.join(" / "));
        }
    }

    Ok(())
}
