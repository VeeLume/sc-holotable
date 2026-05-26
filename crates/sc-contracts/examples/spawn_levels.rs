//! Verify the three-level nesting (`ShipGroup` → `ShipOptions` →
//! `SpawnDescription_Ship`) by walking the typed pools for one or
//! more missions matched by title substring (default) or debug-name
//! substring (`--by-debug`).
//!
//! ```bash
//! cargo run -p sc-contracts --release --example spawn_levels -- "Settle a Score"
//! cargo run -p sc-contracts --release --example spawn_levels -- --by-debug Gilly
//! ```
//!
//! For each match prints, per `MissionProperty` with a ship spawn:
//!
//!   ▸ Group "Wave1"   (every ShipOptions below fires concurrently)
//!       · ShipOptions[0]  options=N  ← engine picks 1 of N by weight
//!           - Ship[0]  concurrent=…  weight=…  candidates=[…]
//!
//! If every ShipOptions has only 1 inner option, the flat slot list ==
//! true spawn count. If any has >1, then summing concurrents across
//! those inner options is wrong (engine picks one).

use std::collections::HashSet;

use sc_contracts::MissionIndex;
use sc_extract::generated::{
    BaseMissionPropertyValuePtr, ContractParamOverrides, DataPools, Handle, MissionProperty,
    SpawnDescription_Ship, SpawnDescription_ShipGroup, SpawnDescription_ShipOptions, SubContract,
};
use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid, LocaleMap,
    LocalizedItemCache,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut by_debug = false;
    args.retain(|a| {
        if a == "--by-debug" {
            by_debug = true;
            false
        } else {
            true
        }
    });
    let needle = args
        .into_iter()
        .next()
        .ok_or("usage: spawn_levels [--by-debug] <substring>")?
        .to_lowercase();

    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);
    let cache = &datacore.snapshot().localized_items;
    let locale = &asset_data.locale;
    let ships = &index.ships;
    let pools = &datacore.records().pools;

    let mut matches: Vec<&_> = Vec::new();
    for c in &index.contracts {
        let hay = if by_debug {
            c.debug_name.to_lowercase()
        } else {
            c.title(locale).unwrap_or("").to_lowercase()
        };
        if hay.contains(&needle) {
            matches.push(c);
        }
    }
    if matches.is_empty() {
        eprintln!("no contract matched '{needle}'");
        return Ok(());
    }

    println!("=== {} match(es) ===", matches.len());

    // To dump from the typed pools we need each mission's
    // ContractParamOverrides + (optionally) its SubContract. The
    // Mission expansion doesn't currently expose these handles, so
    // re-walk the ContractGenerator graph and resolve them via
    // mission id.
    let mut dumped = HashSet::<Guid>::new();
    for c in &matches {
        if !dumped.insert(c.id) {
            continue;
        }
        println!();
        println!("══════════════════════════════════════════════════════════════");
        println!(
            "{}   [title={:?}]",
            c.debug_name,
            c.title(locale).unwrap_or("?")
        );
        println!("  id: {}", c.id);
        println!("  subcontract_of: {:?}", c.origin.subcontract_of);
        println!("══════════════════════════════════════════════════════════════");

        let found = find_mission_param_chain(c.id, &datacore);
        if !found.any() {
            println!("  <no params/subcontract chain found via generator walk>");
            continue;
        }

        // Walk handler.contractParams first (base), then
        // contract.paramOverrides, then sub_contract.property_overrides.
        if let Some(h) = found.handler_params.as_ref() {
            walk_property_overrides(
                "handler.contractParams",
                pools,
                h.get(pools)
                    .map(|p| p.property_overrides.as_slice())
                    .unwrap_or(&[]),
                cache,
                locale,
                ships,
                &index.tag_tree,
            );
        }
        if let Some(h) = found.contract_params.as_ref() {
            walk_property_overrides(
                "contract.paramOverrides",
                pools,
                h.get(pools)
                    .map(|p| p.property_overrides.as_slice())
                    .unwrap_or(&[]),
                cache,
                locale,
                ships,
                &index.tag_tree,
            );
        }
        if let Some(sub) = found.sub_contract.as_ref()
            && let Some(s) = sub.get(pools)
        {
            walk_property_overrides(
                "sub_contract.property_overrides",
                pools,
                s.property_overrides.as_slice(),
                cache,
                locale,
                ships,
                &index.tag_tree,
            );
        }
    }

    Ok(())
}

#[derive(Default)]
struct ParamChain {
    handler_params: Option<Handle<ContractParamOverrides>>,
    contract_params: Option<Handle<ContractParamOverrides>>,
    sub_contract: Option<Handle<SubContract>>,
}
impl ParamChain {
    fn any(&self) -> bool {
        self.handler_params.is_some()
            || self.contract_params.is_some()
            || self.sub_contract.is_some()
    }
}

/// Re-walk the ContractGenerator graph to find which handler / contract
/// / sub-contract a mission id came from, returning the relevant
/// ContractParamOverrides handles + the SubContract handle (if any).
fn find_mission_param_chain(mission_id: Guid, datacore: &Datacore) -> ParamChain {
    use sc_extract::generated::ContractGeneratorHandlerBasePtr as H;

    let pools = &datacore.records().pools;
    for (_gen_guid, gen_handle) in &datacore.records().records.multi_feature.contract_generator {
        let Some(generator) = gen_handle.get(pools) else {
            continue;
        };
        for handler_ptr in &generator.generators {
            // Each variant has the same shape; we only care about the
            // ones we walk for missions.
            let (handler_params, contracts_legacy, contracts, sub_contracts_intro): (
                Option<Handle<ContractParamOverrides>>,
                Vec<sc_extract::generated::Handle<sc_extract::generated::ContractLegacy>>,
                Vec<sc_extract::generated::Handle<sc_extract::generated::Contract>>,
                Vec<sc_extract::generated::Handle<sc_extract::generated::CareerContract>>,
            ) = match handler_ptr {
                H::ContractGeneratorHandler_Legacy(h) => {
                    let Some(handler) = h.get(pools) else {
                        continue;
                    };
                    (
                        handler.contract_params.clone(),
                        handler.legacy_contracts.clone(),
                        Vec::new(),
                        Vec::new(),
                    )
                }
                H::ContractGeneratorHandler_Career(h) => {
                    let Some(handler) = h.get(pools) else {
                        continue;
                    };
                    (
                        handler.contract_params.clone(),
                        Vec::new(),
                        Vec::new(),
                        handler.contracts.clone(),
                    )
                }
                H::ContractGeneratorHandler_List(h) => {
                    let Some(handler) = h.get(pools) else {
                        continue;
                    };
                    (
                        handler.contract_params.clone(),
                        Vec::new(),
                        handler.contracts.clone(),
                        Vec::new(),
                    )
                }
                H::ContractGeneratorHandler_LinearSeries(h) => {
                    let Some(handler) = h.get(pools) else {
                        continue;
                    };
                    (
                        handler.contract_params.clone(),
                        Vec::new(),
                        handler.contracts.clone(),
                        Vec::new(),
                    )
                }
                H::ContractGeneratorHandler_TutorialSeriesDef(h) => {
                    let Some(handler) = h.get(pools) else {
                        continue;
                    };
                    (
                        handler.contract_params.clone(),
                        Vec::new(),
                        handler.contracts.clone(),
                        Vec::new(),
                    )
                }
                _ => continue,
            };

            for ch in &contracts_legacy {
                let Some(c) = ch.get(pools) else { continue };
                if c.id == mission_id {
                    return ParamChain {
                        handler_params,
                        contract_params: c.param_overrides.clone(),
                        sub_contract: None,
                    };
                }
                for sh in &c.sub_contracts {
                    let Some(s) = sh.get(pools) else { continue };
                    if s.id == mission_id {
                        return ParamChain {
                            handler_params,
                            contract_params: c.param_overrides.clone(),
                            sub_contract: Some(sh.clone()),
                        };
                    }
                }
            }
            for ch in &contracts {
                let Some(c) = ch.get(pools) else { continue };
                if c.id == mission_id {
                    return ParamChain {
                        handler_params,
                        contract_params: c.param_overrides.clone(),
                        sub_contract: None,
                    };
                }
                for sh in &c.sub_contracts {
                    let Some(s) = sh.get(pools) else { continue };
                    if s.id == mission_id {
                        return ParamChain {
                            handler_params,
                            contract_params: c.param_overrides.clone(),
                            sub_contract: Some(sh.clone()),
                        };
                    }
                }
            }
            for ch in &sub_contracts_intro {
                let Some(c) = ch.get(pools) else { continue };
                if c.id == mission_id {
                    return ParamChain {
                        handler_params,
                        contract_params: c.param_overrides.clone(),
                        sub_contract: None,
                    };
                }
                for sh in &c.sub_contracts {
                    let Some(s) = sh.get(pools) else { continue };
                    if s.id == mission_id {
                        return ParamChain {
                            handler_params,
                            contract_params: c.param_overrides.clone(),
                            sub_contract: Some(sh.clone()),
                        };
                    }
                }
            }
        }
    }
    ParamChain::default()
}

fn walk_property_overrides(
    label: &str,
    pools: &DataPools,
    props: &[Handle<MissionProperty>],
    cache: &LocalizedItemCache,
    locale: &LocaleMap,
    ships: &sc_contracts::ShipRegistry,
    tree: &sc_extract::TagTree,
) {
    let mut any_ship = false;
    for (pi, ph) in props.iter().enumerate() {
        let Some(prop) = ph.get(pools) else {
            continue;
        };
        let Some(ptr) = prop.value.as_ref() else {
            continue;
        };
        let BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(vh) = ptr
        else {
            continue;
        };
        let Some(val) = vh.get(pools) else { continue };

        if !any_ship {
            println!("\n[{label}]");
            any_ship = true;
        }
        println!(
            "  propertyOverride[{pi}] var='{}' ext='{}'  (ShipSpawnDescriptions)",
            prop.mission_variable_name, prop.extended_text_token
        );

        for (gi, gh) in val.spawn_descriptions.iter().enumerate() {
            let Some(group) = gh.get(pools) else { continue };
            print_group(gi, group, pools, cache, locale, ships, tree);
        }
    }
}

fn print_group(
    gi: usize,
    group: &SpawnDescription_ShipGroup,
    pools: &DataPools,
    cache: &LocalizedItemCache,
    locale: &LocaleMap,
    ships: &sc_contracts::ShipRegistry,
    tree: &sc_extract::TagTree,
) {
    let ships_count = group.ships.len();
    println!(
        "    ▸ Group[{gi}] Name='{}'  ShipOptions={ships_count}  (each ShipOptions fires concurrently)",
        group.name
    );
    for (si, sh) in group.ships.iter().enumerate() {
        let Some(so) = sh.get(pools) else { continue };
        print_options(si, so, pools, cache, locale, ships, tree);
    }
}

fn print_options(
    si: usize,
    so: &SpawnDescription_ShipOptions,
    pools: &DataPools,
    cache: &LocalizedItemCache,
    locale: &LocaleMap,
    ships: &sc_contracts::ShipRegistry,
    tree: &sc_extract::TagTree,
) {
    let opt_count = so.options.len();
    let marker = if opt_count > 1 {
        format!("← engine picks 1 of {opt_count} by weight (ALTERNATIVES)")
    } else {
        "← single option (concurrent only)".to_string()
    };
    println!("        · ShipOptions[{si}]  options={opt_count}  {marker}");
    let weight_sum: f32 = so
        .options
        .iter()
        .filter_map(|oh| oh.get(pools))
        .map(|o| o.weight)
        .sum();
    for (oi, oh) in so.options.iter().enumerate() {
        let Some(opt) = oh.get(pools) else { continue };
        print_ship(oi, opt, weight_sum, pools, cache, locale, ships, tree);
    }
}

fn print_ship(
    oi: usize,
    opt: &SpawnDescription_Ship,
    weight_sum: f32,
    pools: &DataPools,
    cache: &LocalizedItemCache,
    locale: &LocaleMap,
    ships: &sc_contracts::ShipRegistry,
    tree: &sc_extract::TagTree,
) {
    let pos = opt
        .tags
        .as_ref()
        .and_then(|h| h.get(pools))
        .map(|tl| tl.tags.iter().copied().collect::<HashSet<Guid>>())
        .unwrap_or_default();
    let neg = opt
        .negative_tags
        .as_ref()
        .and_then(|h| h.get(pools))
        .map(|tl| tl.tags.iter().copied().collect::<HashSet<Guid>>())
        .unwrap_or_default();
    let cand = ships.resolve_spawn(&pos, &neg);
    let mut names: Vec<String> = cand
        .iter()
        .filter_map(|c| {
            ships
                .display_name(&c.entity_guid, cache, locale)
                .map(String::from)
        })
        .collect();
    names.sort();
    names.dedup();
    let cand_short = if names.is_empty() {
        "<none>".to_string()
    } else if names.len() <= 6 {
        names.join(", ")
    } else {
        format!("{} … +{}", names[..6].join(", "), names.len() - 6)
    };
    let pct = if weight_sum > 0.0 {
        opt.weight / weight_sum * 100.0
    } else {
        0.0
    };
    println!(
        "            - Ship[{oi}]  concurrent={}  weight={:.2} ({:.0}%)  candidates={}  [{cand_short}]",
        opt.concurrent_amount,
        opt.weight,
        pct,
        names.len()
    );
    // Dump tag names so we can see what discriminates the alternatives.
    let tag_names: Vec<String> = opt
        .tags
        .as_ref()
        .and_then(|h| h.get(pools))
        .map(|tl| {
            tl.tags
                .iter()
                .map(|g| {
                    tree.get(g)
                        .map(|n| n.name.as_str())
                        .unwrap_or("<?>")
                        .to_string()
                })
                .collect()
        })
        .unwrap_or_default();
    if !tag_names.is_empty() {
        println!("                tags: {}", tag_names.join(", "));
    }
}
