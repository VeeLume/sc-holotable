//! Per-mission census: walk every Mission's param chain
//! (handler.contractParams + contract.paramOverrides +
//! sub_contract.property_overrides), find every `ShipOptions` with
//! options >= 2, classify the discriminator, and sample 3 missions
//! per class. Also measures content-sharing across missions.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example options_mission_map
//! ```

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use sc_contracts::MissionIndex;
use sc_extract::generated::{
    BaseMissionPropertyValuePtr, ContractGeneratorHandlerBasePtr as H, ContractParamOverrides,
    DataPools, Handle, MissionProperty, SpawnDescription_ShipOptions, SubContract,
};
use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid, LocaleMap, TagTree,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);
    let pools = &datacore.records().pools;
    let tree = &datacore.snapshot().tag_tree;
    let locale = &asset_data.locale;

    // Pre-compute the param chain for every mission once.
    let mut by_mission: HashMap<Guid, ParamChain> = HashMap::new();
    walk_all_handlers(&datacore, &mut by_mission);

    // For each mission, walk its param chain → ShipOptions.
    // Bucket per discriminator. Track content-signature sharing.
    type DiscriminatorKey = String;
    type ContentSignature = String;

    let mut bucket_samples: BTreeMap<DiscriminatorKey, Vec<MissionSample>> = BTreeMap::new();
    let mut bucket_counts: BTreeMap<DiscriminatorKey, usize> = BTreeMap::new();
    let mut signature_to_missions: BTreeMap<ContentSignature, BTreeSet<String>> = BTreeMap::new();
    let mut signature_discriminator: BTreeMap<ContentSignature, DiscriminatorKey> = BTreeMap::new();

    const SAMPLE_LIMIT: usize = 5;

    for c in &index.contracts {
        let Some(chain) = by_mission.get(&c.id) else {
            continue;
        };
        let mission_label = format!(
            "{}  [title={:?}]",
            c.debug_name,
            c.title(locale).unwrap_or("?")
        );

        let mut visit_shipopts = |so: &SpawnDescription_ShipOptions, varname: &str| {
            let n = so.options.len();
            if n < 2 {
                return;
            }
            let disc = classify_discriminator(so, pools, tree);
            let sig = content_signature(so, pools, tree);

            *bucket_counts.entry(disc.clone()).or_default() += 1;
            let bucket = bucket_samples.entry(disc.clone()).or_default();
            if bucket.iter().all(|s| s.mission_label != mission_label) && bucket.len() < SAMPLE_LIMIT
            {
                bucket.push(MissionSample {
                    mission_label: mission_label.clone(),
                    var_name: varname.to_string(),
                    summary: describe_sample(so, pools, tree),
                });
            }

            signature_discriminator
                .entry(sig.clone())
                .or_insert_with(|| disc.clone());
            signature_to_missions
                .entry(sig)
                .or_default()
                .insert(mission_label.clone());
        };

        for ph in chain.props_iter(pools) {
            let Some(prop) = ph.get(pools) else { continue };
            let Some(ptr) = prop.value.as_ref() else {
                continue;
            };
            let BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(vh) = ptr
            else {
                continue;
            };
            let Some(val) = vh.get(pools) else { continue };
            for gh in &val.spawn_descriptions {
                let Some(group) = gh.get(pools) else { continue };
                for sh in &group.ships {
                    let Some(so) = sh.get(pools) else { continue };
                    visit_shipopts(so, &prop.mission_variable_name);
                }
            }
        }
    }

    // ── Per-discriminator sample report ─────────────────────────────────
    println!("\n=== Per-discriminator mission samples (up to {SAMPLE_LIMIT} each) ===\n");
    let mut sorted: Vec<_> = bucket_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (disc, count) in &sorted {
        println!("──────────────────────────────────────────────");
        println!("«{disc}»   ({count} ShipOptions instances across all missions)");
        if let Some(samples) = bucket_samples.get(disc.as_str()) {
            for s in samples {
                println!("  • {}", s.mission_label);
                println!("      var='{}'", s.var_name);
                println!("      {}", s.summary);
            }
        }
        println!();
    }

    // ── Content sharing report ──────────────────────────────────────────
    let mut shared: Vec<(&String, &BTreeSet<String>)> = signature_to_missions
        .iter()
        .filter(|(_, ms)| ms.len() > 1)
        .collect();
    shared.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let total_sigs = signature_to_missions.len();
    let shared_sigs = shared.len();
    println!("\n=== Content sharing of ShipOptions across missions ===");
    println!("  total distinct ShipOptions content signatures: {total_sigs}");
    println!(
        "  signatures used by >1 mission: {shared_sigs} ({:.1}%)",
        if total_sigs == 0 {
            0.0
        } else {
            shared_sigs as f64 * 100.0 / total_sigs as f64
        }
    );
    println!("\n--- top 15 most-shared signatures ---");
    for (sig, missions) in shared.iter().take(15) {
        let disc = signature_discriminator
            .get(sig.as_str())
            .map(|s| s.as_str())
            .unwrap_or("?");
        println!("\n  shared by {} missions  «{disc}»", missions.len());
        // Show signature trimmed
        let sig_trim = if sig.len() > 200 {
            format!("{}…", &sig[..200])
        } else {
            sig.to_string()
        };
        println!("    sig: {sig_trim}");
        for m in missions.iter().take(8) {
            println!("    - {m}");
        }
        if missions.len() > 8 {
            println!("    … +{} more", missions.len() - 8);
        }
    }

    // ── Distortion deep-dive ────────────────────────────────────────────
    println!("\n=== Distortion-variant census ===");
    let mut distortion_missions: BTreeSet<String> = BTreeSet::new();
    for (sig, missions) in &signature_to_missions {
        if sig.contains("Distortion") {
            for m in missions {
                distortion_missions.insert(m.clone());
            }
        }
    }
    println!(
        "  missions whose alternatives include a Distortion-tagged option: {}",
        distortion_missions.len()
    );
    for m in distortion_missions.iter().take(20) {
        println!("    - {m}");
    }

    Ok(())
}

struct MissionSample {
    mission_label: String,
    var_name: String,
    summary: String,
}

#[derive(Default, Clone)]
struct ParamChain {
    handler_params: Option<Handle<ContractParamOverrides>>,
    contract_params: Option<Handle<ContractParamOverrides>>,
    sub_contract: Option<Handle<SubContract>>,
}

impl ParamChain {
    fn props_iter<'a>(&'a self, pools: &'a DataPools) -> Vec<&'a Handle<MissionProperty>> {
        let mut out: Vec<&Handle<MissionProperty>> = Vec::new();
        if let Some(h) = self.handler_params.as_ref()
            && let Some(p) = h.get(pools)
        {
            out.extend(p.property_overrides.iter());
        }
        if let Some(h) = self.contract_params.as_ref()
            && let Some(p) = h.get(pools)
        {
            out.extend(p.property_overrides.iter());
        }
        if let Some(h) = self.sub_contract.as_ref()
            && let Some(s) = h.get(pools)
        {
            out.extend(s.property_overrides.iter());
        }
        out
    }
}

/// Walk every ContractGenerator and populate `out` with the param
/// chain for every mission id we encounter.
fn walk_all_handlers(datacore: &Datacore, out: &mut HashMap<Guid, ParamChain>) {
    let pools = &datacore.records().pools;
    for (_g, gh) in &datacore.records().records.multi_feature.contract_generator {
        let Some(generator) = gh.get(pools) else {
            continue;
        };
        for handler_ptr in &generator.generators {
            match handler_ptr {
                H::ContractGeneratorHandler_Legacy(h) => {
                    let Some(handler) = h.get(pools) else { continue };
                    let hp = handler.contract_params.clone();
                    for ch in &handler.legacy_contracts {
                        let Some(c) = ch.get(pools) else { continue };
                        out.entry(c.id).or_insert(ParamChain {
                            handler_params: hp.clone(),
                            contract_params: c.param_overrides.clone(),
                            sub_contract: None,
                        });
                        for sh in &c.sub_contracts {
                            let Some(s) = sh.get(pools) else { continue };
                            out.entry(s.id).or_insert(ParamChain {
                                handler_params: hp.clone(),
                                contract_params: c.param_overrides.clone(),
                                sub_contract: Some(sh.clone()),
                            });
                        }
                    }
                }
                H::ContractGeneratorHandler_Career(h) => {
                    let Some(handler) = h.get(pools) else { continue };
                    let hp = handler.contract_params.clone();
                    for ch in &handler.contracts {
                        let Some(c) = ch.get(pools) else { continue };
                        out.entry(c.id).or_insert(ParamChain {
                            handler_params: hp.clone(),
                            contract_params: c.param_overrides.clone(),
                            sub_contract: None,
                        });
                        for sh in &c.sub_contracts {
                            let Some(s) = sh.get(pools) else { continue };
                            out.entry(s.id).or_insert(ParamChain {
                                handler_params: hp.clone(),
                                contract_params: c.param_overrides.clone(),
                                sub_contract: Some(sh.clone()),
                            });
                        }
                    }
                }
                H::ContractGeneratorHandler_List(h) => walk_list_like(
                    h.get(pools).map(|h| (h.contract_params.clone(), h.contracts.clone())),
                    pools,
                    out,
                ),
                H::ContractGeneratorHandler_LinearSeries(h) => walk_list_like(
                    h.get(pools).map(|h| (h.contract_params.clone(), h.contracts.clone())),
                    pools,
                    out,
                ),
                H::ContractGeneratorHandler_TutorialSeriesDef(h) => walk_list_like(
                    h.get(pools).map(|h| (h.contract_params.clone(), h.contracts.clone())),
                    pools,
                    out,
                ),
                _ => {}
            }
        }
    }
}

fn walk_list_like(
    handler: Option<(
        Option<Handle<ContractParamOverrides>>,
        Vec<Handle<sc_extract::generated::Contract>>,
    )>,
    pools: &DataPools,
    out: &mut HashMap<Guid, ParamChain>,
) {
    let Some((hp, contracts)) = handler else {
        return;
    };
    for ch in &contracts {
        let Some(c) = ch.get(pools) else { continue };
        out.entry(c.id).or_insert(ParamChain {
            handler_params: hp.clone(),
            contract_params: c.param_overrides.clone(),
            sub_contract: None,
        });
        for sh in &c.sub_contracts {
            let Some(s) = sh.get(pools) else { continue };
            out.entry(s.id).or_insert(ParamChain {
                handler_params: hp.clone(),
                contract_params: c.param_overrides.clone(),
                sub_contract: Some(sh.clone()),
            });
        }
    }
}

fn classify_discriminator(
    so: &SpawnDescription_ShipOptions,
    pools: &DataPools,
    tree: &TagTree,
) -> String {
    let mut sib: Vec<HashSet<Guid>> = Vec::new();
    for oh in &so.options {
        let Some(opt) = oh.get(pools) else { continue };
        let set: HashSet<Guid> = opt
            .tags
            .as_ref()
            .and_then(|h| h.get(pools))
            .map(|tl| tl.tags.iter().copied().collect())
            .unwrap_or_default();
        sib.push(set);
    }
    if sib.len() < 2 {
        return "<single>".to_string();
    }
    let mut all_tags: HashSet<Guid> = HashSet::new();
    for s in &sib {
        all_tags.extend(s.iter().copied());
    }
    let mut fams: BTreeSet<String> = BTreeSet::new();
    for g in &all_tags {
        if sib.iter().all(|s| s.contains(g)) {
            continue;
        }
        let name = tree.get(g).map(|n| n.name.as_str()).unwrap_or("?");
        fams.insert(family_of(name));
    }
    if fams.is_empty() {
        return "<tags identical>".to_string();
    }
    fams.into_iter().collect::<Vec<_>>().join(" + ")
}

fn family_of(name: &str) -> String {
    let t = name.trim();
    if let Some(r) = t.strip_prefix("HumanPilot") {
        if r.chars().all(|c| c.is_ascii_digit()) {
            return "HumanPilotNN".to_string();
        }
    }
    if matches!(t, "VeryEasy" | "Easy" | "Medium" | "Hard" | "VeryHard") {
        return "DifficultyTier".to_string();
    }
    name.to_string()
}

fn describe_sample(
    so: &SpawnDescription_ShipOptions,
    pools: &DataPools,
    tree: &TagTree,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (oi, oh) in so.options.iter().enumerate() {
        let Some(opt) = oh.get(pools) else { continue };
        let tag_names: Vec<&str> = opt
            .tags
            .as_ref()
            .and_then(|h| h.get(pools))
            .map(|tl| {
                tl.tags
                    .iter()
                    .map(|g| tree.get(g).map(|n| n.name.as_str()).unwrap_or("?"))
                    .collect()
            })
            .unwrap_or_default();
        parts.push(format!(
            "opt[{oi}] c={} w={:.2} [{}]",
            opt.concurrent_amount,
            opt.weight,
            tag_names.join(",")
        ));
    }
    parts.join("  ‖  ")
}

/// Stable content fingerprint for a `ShipOptions` — used to detect
/// when multiple missions reference the same alternatives template
/// (whether via raw DCB reference or by independent inlining).
fn content_signature(
    so: &SpawnDescription_ShipOptions,
    pools: &DataPools,
    tree: &TagTree,
) -> String {
    let mut entries: Vec<String> = Vec::new();
    for oh in &so.options {
        let Some(opt) = oh.get(pools) else { continue };
        let mut tags: Vec<&str> = opt
            .tags
            .as_ref()
            .and_then(|h| h.get(pools))
            .map(|tl| {
                tl.tags
                    .iter()
                    .map(|g| tree.get(g).map(|n| n.name.as_str()).unwrap_or("?"))
                    .collect()
            })
            .unwrap_or_default();
        tags.sort();
        entries.push(format!(
            "c={};w={:.2};tags=[{}]",
            opt.concurrent_amount,
            opt.weight,
            tags.join(",")
        ));
    }
    // Order-sensitive — option order is part of the template identity.
    entries.join(" ‖ ")
}
