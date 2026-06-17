//! Build the provider spine from the loose probe DCB and print one provider.
//!
//! Validates Tier 1 against the trace numbers — Clio (`HPP_Stanton4b`): three
//! groups with mode-shares 13.8% / 28.7% / 57.5%, ship elements Ice/Copper 40%,
//! Taranite 18%, Quantainium 2%.
//!
//! ```bash
//! cargo run -p sc-gathering --example dump_provider                  # Clio
//! cargo run -p sc-gathering --example dump_provider -- <provider-guid>
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordPaths};
use sc_gathering::Gathering;

const DCB: &str = "target/probe-resources/dcbfile/Data/Game2.dcb";
const CLIO: &str = "703a18ca-7f7c-4489-a64a-cd0cd359b8fe"; // HPP_Stanton4b

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = BTreeMap::new();
    files.insert("Data/Game2.dcb".to_string(), std::fs::read(DCB)?);
    let assets = AssetSource::from_snapshot(files, "probe");
    let asset_data = AssetData::extract(&assets, &AssetConfig::minimal())?;
    let dc = Datacore::parse(&assets, &asset_data)?;
    let paths = RecordPaths::build(&dc);
    let gathering = Gathering::build(dc.records(), &paths);
    eprintln!("providers: {}", gathering.len());

    let guid: Guid = std::env::args()
        .nth(1)
        .unwrap_or_else(|| CLIO.to_string())
        .parse()?;
    let Some(p) = gathering.provider(&guid) else {
        eprintln!("no provider for {guid}");
        std::process::exit(1);
    };

    println!("provider {guid} — {} groups", p.groups.len());
    for grp in &p.groups {
        println!(
            "  [{}]  mode={:?}  group_prob={}  mode_share={:.1}%  ({} elements)",
            grp.name,
            grp.mode,
            grp.group_probability,
            grp.mode_share * 100.0,
            grp.elements.len()
        );
        for e in &grp.elements {
            let cluster = e
                .cluster
                .as_ref()
                .map(|c| {
                    format!(
                        "{}%·{}-{}",
                        c.probability_of_clustering,
                        c.min_size(),
                        c.max_size()
                    )
                })
                .unwrap_or_default();
            let deposit = e
                .deposit
                .as_ref()
                .map(|d| {
                    let res = d.parts.iter().filter(|p| p.resource.is_some()).count();
                    format!("{:?} ({}/{} parts w/ resource)", d.name, res, d.parts.len())
                })
                .unwrap_or_else(|| "<no deposit>".to_string());
            let signal = e
                .signal
                .map(|s| format!("sig {:.3}", s / 1000.0))
                .unwrap_or_else(|| "sig —".to_string());
            println!(
                "      rel={:<6} share={:>5.1}%  {cluster:<12}  {signal:<11}  {deposit}",
                e.relative_probability,
                e.share * 100.0
            );
        }
    }
    Ok(())
}
