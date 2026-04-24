//! Dump power draw from ItemResourceComponentParams on weapons.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example dump_power_draw
//! ```

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{AssetConfig, AssetData, AssetSource, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let install = sc_installs::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;

    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    println!(
        "Name\tDisplay\tS\tSustain\tState\tResource\tAmount/s\tMinFraction\tPwrLow\tPwrMed\tPwrHigh"
    );

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names
            .get(&guid)
            .copied()
            .unwrap_or("?")
            .replace("EntityClassDefinition.", "");

        if !name.starts_with(|c: char| c.is_uppercase()) {
            continue;
        }
        if name.contains("CML") || name.contains("Mining") || name.contains("Salvage") {
            continue;
        }

        let has_weapon = ecd.components.iter().any(|c| {
            matches!(
                c,
                DataForgeComponentParamsPtr::SCItemWeaponComponentParams(_)
            )
        });
        if !has_weapon {
            continue;
        }

        let base = dedup_name(&name);
        if seen.contains(&base) {
            continue;
        }
        seen.insert(base);

        let display = snap.display_names.get(&guid).unwrap_or("").to_string();
        let item_def = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SAttachableComponentParams(h) => h
                .get(pools)
                .and_then(|a| a.attach_def.and_then(|h| h.get(pools))),
            _ => None,
        });
        let size = item_def.map(|d| d.size).unwrap_or(0);

        let wp = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        });
        let has_heat = wp
            .and_then(|w| w.connection_params.and_then(|h| h.get(pools)))
            .and_then(|c| c.simplified_heat_params)
            .and_then(|h| h.get(pools))
            .is_some();
        let has_regen = wp
            .and_then(|w| w.weapon_regen_consumer_params.and_then(|h| h.get(pools)))
            .is_some();
        let sustain = match (has_heat, has_regen) {
            (true, false) => "HEAT",
            (false, true) => "ENERGY",
            (true, true) => "BOTH",
            (false, false) => "none",
        };

        // Find ItemResourceComponentParams
        let irc = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::ItemResourceComponentParams(h) => h.get(pools),
            _ => None,
        });
        let Some(irc) = irc else {
            continue;
        };

        for state_h in &irc.states {
            let Some(state) = state_h.get(pools) else {
                continue;
            };

            // Power ranges
            let pwr_str = state
                .power_ranges
                .and_then(|h| h.get(pools))
                .map(|pr| {
                    let low = pr
                        .low
                        .and_then(|h| h.get(pools))
                        .map(|r| format!("{}:{:.2}", r.start, r.modifier))
                        .unwrap_or_default();
                    let med = pr
                        .medium
                        .and_then(|h| h.get(pools))
                        .map(|r| format!("{}:{:.2}", r.start, r.modifier))
                        .unwrap_or_default();
                    let high = pr
                        .high
                        .and_then(|h| h.get(pools))
                        .map(|r| format!("{}:{:.2}", r.start, r.modifier))
                        .unwrap_or_default();
                    (low, med, high)
                })
                .unwrap_or_default();

            for delta in &state.deltas {
                if let ItemResourceDeltaBasePtr::ItemResourceDeltaConsumption(h) = delta {
                    let Some(cons) = h.get(pools) else { continue };
                    let Some(amount) = cons.consumption.and_then(|h| h.get(pools)) else {
                        continue;
                    };

                    let resource = format!("{:?}", amount.resource);
                    let amount_val = amount
                        .resource_amount_per_second
                        .as_ref()
                        .map(|u| {
                            match u {
                                SBaseResourceUnitPtr::SStandardResourceUnit(h) => h
                                    .get(pools)
                                    .map(|v| v.standard_resource_units)
                                    .unwrap_or(0.0),
                                SBaseResourceUnitPtr::SPowerSegmentResourceUnit(h) => {
                                    h.get(pools).map(|v| v.units as f32).unwrap_or(0.0)
                                }
                                _ => -1.0, // unknown unit type
                            }
                        })
                        .unwrap_or(0.0);

                    println!(
                        "{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{:.2}\t{}\t{}\t{}",
                        name,
                        display,
                        size,
                        sustain,
                        state.name,
                        resource,
                        amount_val,
                        cons.minimum_consumption_fraction,
                        pwr_str.0,
                        pwr_str.1,
                        pwr_str.2
                    );
                }
            }
        }
    }

    Ok(())
}

fn dedup_name(name: &str) -> String {
    if name.starts_with(|c: char| c.is_uppercase()) {
        return name.to_string();
    }
    if let Some(pos) = name.find("_01_") {
        return name[..pos + 3].to_string();
    }
    if let Some(pos) = name.find("_02_") {
        return name[..pos + 3].to_string();
    }
    name.to_string()
}
