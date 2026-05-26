//! Smoke-test [`Mission::combat_class`] and [`Mission::ship_count_range`]
//! against real LIVE DCB data. Verifies the helpers report the expected
//! values for known missions (Settle a Score etc.).
//!
//! ```bash
//! cargo run -p sc-contracts --release --example mission_helpers_smoke
//! ```

use sc_contracts::MissionIndex;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);
    let locale = &asset_data.locale;

    // Targeted samples — verify against the known-good values we
    // derived in the investigation phase.
    let probes = [
        "EckhartSecurity_ShipAmbush_Nyx_VeryEasy", // Settle a Score (Criminal)
        "EckhartSecurity_ShipAmbush_Nyx_VeryEasy_V", // Settle a Score (Vanduul)
        "EckhartSecurity_ShipAmbush_Nyx_Easy",     // Time to Teach a Lesson
        "EckhartSecurity_ShipAmbush_Nyx_VeryHard", // Dangerous Ambush
        "InterSec_Bounty_Nyx_Hard",                // Distortion variant
        "GillysPilotSchool_Mission05",             // pure concurrent (no alternatives)
        "BountyHuntersGuild_Certification_Easy",   // mixed Easy + VeryEasy alt
    ];

    println!();
    println!(
        "{:>50}  {:>14}  {:>11}",
        "debug_name", "combat_class", "ships(range)"
    );
    println!("{}", "─".repeat(80));

    for needle in probes {
        let mut hits = 0;
        for c in &index.contracts {
            if c.debug_name != needle {
                continue;
            }
            hits += 1;
            let cc = c.combat_class().unwrap_or("—");
            let (lo, hi) = c.ship_count_range();
            let range = if lo == hi {
                format!("{lo}")
            } else {
                format!("{lo}-{hi}")
            };
            println!(
                "{:>50}  {:>14}  {:>11}    title={:?}",
                needle,
                cc,
                range,
                c.title(locale).unwrap_or("?")
            );
        }
        if hits == 0 {
            println!("{:>50}  {:>14}  {:>11}    <no match>", needle, "—", "—");
        }
    }

    Ok(())
}
