//! Diagnose why some contracts come back with an empty `Mission.cargo`: for the
//! given contract_definition_ids, dump the matched mission's cargo + its
//! sub-contract children's cargo, to see whether the manifest lives on a child.
//!
//! ```bash
//! cargo run -p sc-missions --release --example cargo_lookup -- <guid> [<guid> ...]
//! ```
use std::collections::HashMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Guid};
use sc_missions::{Mission, Missions};

fn dump(m: &Mission, indent: &str) {
    println!(
        "{indent}{}  cargo={}  sub_of={:?}",
        m.debug_name,
        m.cargo.len(),
        m.origin.subcontract_of.map(|g| g.to_string())
    );
    for l in &m.cargo {
        println!(
            "{indent}    leg: resource={:?} scu {}–{} box {}",
            l.resource, l.min_scu, l.max_scu, l.max_box
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let missions = Missions::build(&datacore);

    let by_id: HashMap<Guid, &Mission> = missions.contracts.iter().map(|m| (m.id, m)).collect();
    // children keyed by parent
    let mut children: HashMap<Guid, Vec<&Mission>> = HashMap::new();
    for m in &missions.contracts {
        if let Some(p) = m.origin.subcontract_of {
            children.entry(p).or_default().push(m);
        }
    }

    let targets: Vec<String> = std::env::args().skip(1).collect();
    for t in &targets {
        let Ok(g) = t.parse::<Guid>() else {
            eprintln!("bad guid {t}");
            continue;
        };
        println!("\n=== contract_def {t} ===");
        match by_id.get(&g) {
            Some(m) => {
                dump(m, "  ");
                if let Some(kids) = children.get(&g) {
                    println!("  children ({}):", kids.len());
                    for k in kids {
                        dump(k, "    ");
                    }
                }
            }
            None => println!("  (not found by id — contract_def != Mission.id?)"),
        }
        // also: any mission whose debug_name shares the contract stem, with cargo
        if let Some(m) = by_id.get(&g) {
            let stem = m
                .debug_name
                .split('_')
                .take(3)
                .collect::<Vec<_>>()
                .join("_");
            let kin: Vec<&Mission> = missions
                .contracts
                .iter()
                .filter(|x| x.debug_name.starts_with(&stem) && !x.cargo.is_empty())
                .collect();
            println!("  kin sharing stem '{stem}*' WITH cargo: {}", kin.len());
            for k in kin.iter().take(4) {
                dump(k, "    ~ ");
            }
        }
    }
    Ok(())
}
