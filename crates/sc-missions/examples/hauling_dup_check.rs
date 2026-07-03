//! Validation dig: flag missions whose `cargo` contains the exact same leg twice
//! (same resource + min/max SCU + box). Cross-layer double-counting in
//! `resolve_hauling_legs` (template + override both carrying `HaulingOrders`)
//! would show up here as 2N legs on an N-endpoint mission.
//!
//! Finding on SC 4.8 LIVE (2026-07-03): 282 of 1,137 cargo missions carry
//! identical legs, but every one matches its endpoint count exactly —
//! `SingleToMultiN` / `MultiNToSingle` missions carry N per-endpoint entries
//! via `CombinedDataSetEntries` (legs=N, never 2N), and no non-Multi mission
//! duplicates at all. So identical legs are structural, not double-counted.
//! Re-run after a regen; a `SingleToMultiN` mission with more than N legs is
//! the regression alarm.
use sc_extract::{AssetConfig, AssetData, AssetSource, RecordCollection};
use sc_missions::Missions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let missions = Missions::build(&datacore);

    let mut with_cargo = 0usize;
    let mut dup_missions = 0usize;
    for (guid, m) in missions.iter() {
        if m.cargo.is_empty() {
            continue;
        }
        with_cargo += 1;
        let mut seen = std::collections::HashSet::new();
        let mut dups = Vec::new();
        for leg in &m.cargo {
            let key = (
                leg.resource,
                leg.min_scu.to_bits(),
                leg.max_scu.to_bits(),
                leg.max_box.to_bits(),
            );
            if !seen.insert(key) {
                dups.push(leg);
            }
        }
        if !dups.is_empty() {
            dup_missions += 1;
            println!(
                "DUP {guid} {:?} legs={} dups={}",
                m.debug_name,
                m.cargo.len(),
                dups.len()
            );
        }
    }
    println!("\nmissions with cargo: {with_cargo}");
    println!("missions with duplicate identical legs: {dup_missions}");
    Ok(())
}
