//! Validate **Part B**: `Mission.cargo` (the hauling manifest) is populated by
//! `Missions::build` — per-leg commodity + min/max SCU + max box, joinable to a
//! live gRPC contract by `contract_definition_id == Mission.id`.
//!
//! ```bash
//! cargo run -p sc-missions --release --example cargo_field_check
//! ```
use sc_extract::{AssetConfig, AssetData, AssetSource, LocaleMap};
use sc_missions::Missions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locale: &LocaleMap = &asset_data.locale;
    let resources = sc_resources::Resources::build(datacore.records());

    let index = Missions::build(&datacore);

    let (mut with_cargo, mut total_legs, mut mixed, mut shown) = (0usize, 0usize, 0usize, 0usize);
    for m in &index.contracts {
        if m.cargo.is_empty() {
            continue;
        }
        with_cargo += 1;
        total_legs += m.cargo.len();
        let commodities: std::collections::BTreeSet<&str> = m
            .cargo
            .iter()
            .filter_map(|l| l.commodity_name(&resources, locale))
            .collect();
        if commodities.len() > 1 {
            mixed += 1;
        }
        if shown < 12 {
            shown += 1;
            eprintln!(
                "{}  ({} leg{})",
                m.debug_name,
                m.cargo.len(),
                if m.cargo.len() == 1 { "" } else { "s" }
            );
            for l in &m.cargo {
                eprintln!(
                    "    {:<24} SCU {}–{}  box {}",
                    l.commodity_name(&resources, locale)
                        .unwrap_or("<unresolved>"),
                    l.min_scu,
                    l.max_scu,
                    l.max_box
                );
            }
        }
    }
    eprintln!(
        "\n{with_cargo}/{} missions carry a cargo manifest ({total_legs} legs, {mixed} MIXED)",
        index.contracts.len()
    );
    Ok(())
}
