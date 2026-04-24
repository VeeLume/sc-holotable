//! Quick dump of P4-AR fire actions to check if dual spread is fire modes or ADS.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example dump_p4ar
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

    // Find weapons with >1 fire action and dump their action details
    println!("=== WEAPONS WITH MULTIPLE FIRE ACTIONS ===\n");

    let mut multi_action: Vec<(&str, &str, Vec<String>)> = Vec::new();

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names.get(&guid).copied().unwrap_or("?");
        let display = snap.display_names.get(&guid).unwrap_or("");

        let Some(wp) = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        }) else {
            continue;
        };

        if wp.fire_actions.len() <= 1 {
            continue;
        }

        // Skip skins — only base models
        let short = name.replace("EntityClassDefinition.", "");
        if short.contains("_tint")
            || short.contains("_black0")
            || short.contains("_white0")
            || short.contains("_store0")
            || short.contains("_collector0")
            || short.contains("_gold0")
            || short.contains("_green0")
            || short.contains("_red0")
            || short.contains("_blue0")
            || short.contains("_tan0")
            || short.contains("_Luminalia")
            || short.contains("_lumi0")
            || short.contains("_firerats")
            || short.contains("_contestedzone")
            || short.contains("_xenothreat")
            || short.contains("_iae")
            || short.contains("_imp0")
            || short.contains("_cen0")
            || short.contains("_mat0")
            || short.contains("_shin0")
            || short.contains("_camo0")
            || short.contains("_chromic")
            || short.contains("_engraved")
            || short.contains("_arctic")
            || short.contains("_purple0")
            || short.contains("_pink")
            || short.contains("_grey0")
            || short.contains("_yellow0")
            || short.contains("_orange")
            || short.contains("_reward")
            || short.contains("_acid0")
            || short.contains("_sunset0")
            || short.contains("_digi0")
            || short.contains("_civilian_")
            || short.contains("_urban0")
            || short.contains("_tow")
        {
            continue;
        }

        let mut actions = Vec::new();
        for (i, action) in wp.fire_actions.iter().enumerate() {
            let desc = match action {
                SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
                    if let Some(r) = h.get(pools) {
                        let lp = r.launch_params.as_ref().and_then(|lp| match lp {
                            SLauncherBasePtr::SProjectileLauncher(h) => h.get(pools),
                            _ => None,
                        });
                        let spread = lp.and_then(|pl| pl.spread_params.and_then(|h| h.get(pools)));
                        format!(
                            "[{}] FireRapid name=\"{}\" rpm={} hps={:.2} spin={:.2}/{:.2} spread={}/{}",
                            i,
                            r.name,
                            r.fire_rate,
                            r.heat_per_shot,
                            r.spin_up_time,
                            r.spin_down_time,
                            spread
                                .map(|s| format!("{:.2}", s.min))
                                .unwrap_or("-".into()),
                            spread
                                .map(|s| format!("{:.2}", s.max))
                                .unwrap_or("-".into()),
                        )
                    } else {
                        format!("[{}] FireRapid (unresolved)", i)
                    }
                }
                SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
                    if let Some(s) = h.get(pools) {
                        let lp = s.launch_params.as_ref().and_then(|lp| match lp {
                            SLauncherBasePtr::SProjectileLauncher(h) => h.get(pools),
                            _ => None,
                        });
                        let spread = lp.and_then(|pl| pl.spread_params.and_then(|h| h.get(pools)));
                        format!(
                            "[{}] FireSingle name=\"{}\" rpm={} hps={:.2} launch={} spread={}/{}",
                            i,
                            s.name,
                            s.fire_rate,
                            s.heat_per_shot,
                            s.launch_params.is_some(),
                            spread
                                .map(|s| format!("{:.2}", s.min))
                                .unwrap_or("-".into()),
                            spread
                                .map(|s| format!("{:.2}", s.max))
                                .unwrap_or("-".into()),
                        )
                    } else {
                        format!("[{}] FireSingle (unresolved)", i)
                    }
                }
                SWeaponActionParamsPtr::SWeaponActionFireBeamParams(_) => {
                    format!("[{}] FireBeam", i)
                }
                SWeaponActionParamsPtr::SWeaponActionSequenceParams(_) => {
                    format!("[{}] Sequence", i)
                }
                SWeaponActionParamsPtr::SWeaponActionFireBurstParams(h) => {
                    if let Some(b) = h.get(pools) {
                        format!(
                            "[{}] FireBurst name=\"{}\" rpm={} shotCount={}",
                            i, b.name, b.fire_rate, b.shot_count
                        )
                    } else {
                        format!("[{}] FireBurst (unresolved)", i)
                    }
                }
                SWeaponActionParamsPtr::SWeaponActionDynamicConditionParams(h) => {
                    if let Some(dc) = h.get(pools) {
                        let default_type = dc
                            .default_weapon_action
                            .as_ref()
                            .map(|a| match a {
                                SWeaponActionParamsPtr::SWeaponActionFireRapidParams(_) => {
                                    "FireRapid"
                                }
                                SWeaponActionParamsPtr::SWeaponActionFireBeamParams(_) => {
                                    "FireBeam"
                                }
                                SWeaponActionParamsPtr::SWeaponActionFireSingleParams(_) => {
                                    "FireSingle"
                                }
                                _ => "other",
                            })
                            .unwrap_or("none");
                        format!(
                            "[{}] DynamicCondition name=\"{}\" default={} conditions={}",
                            i,
                            dc.name,
                            default_type,
                            dc.conditional_weapon_actions.len()
                        )
                    } else {
                        format!("[{}] DynamicCondition (unresolved)", i)
                    }
                }
                SWeaponActionParamsPtr::SWeaponActionParallelParams(_) => {
                    format!("[{}] Parallel", i)
                }
                #[cfg(any(feature = "entities-scitem-ships", feature = "entities-scitem-weapons"))]
                SWeaponActionParamsPtr::SWeaponActionFireHealingBeamParams(_) => {
                    format!("[{}] HealingBeam", i)
                }
                _ => format!("[{}] Other", i),
            };
            actions.push(desc);
        }

        multi_action.push((name, display, actions));
    }

    multi_action.sort_by(|a, b| a.0.cmp(b.0));

    for (name, display, actions) in &multi_action {
        let short = name.replace("EntityClassDefinition.", "");
        println!("{} ({})", short, display);
        for a in actions {
            println!("  {}", a);
        }
        println!();
    }

    println!(
        "Total weapons with >1 fire action (base models): {}",
        multi_action.len()
    );

    Ok(())
}
