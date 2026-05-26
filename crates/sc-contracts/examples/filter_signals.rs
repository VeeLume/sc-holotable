//! For one mission (matched by title or `--by-debug` debug name), try
//! every plausible "discriminator signal" against each alternatives
//! set, to see which can be filtered down deterministically from
//! contract-level data alone (no hardcoded rules).
//!
//! Signals tried:
//! 1. **Contract template tags** — `Contract.template` is a
//!    `MissionTemplate` (or similar) record; dump its tags + active
//!    contract settings.
//! 2. **Contract-level direct properties** — anything that looks
//!    like a tier / difficulty marker on the Contract / SubContract
//!    / Handler / Generator chain.
//! 3. **Handler debug_name tokens** — sometimes the difficulty is
//!    in the handler name (`EckhartSecurity_ShipAmbush_Nyx_VeryEasy`
//!    → token `VeryEasy`).
//! 4. **Tag intersection** — for each alternative inside a
//!    ShipOptions, intersect its tags with the union of signals from
//!    1-3. The alternative whose tags overlap most uniquely with the
//!    contract context is a candidate "this is the one that fires".
//!
//! ```bash
//! cargo run -p sc-contracts --release --example filter_signals -- "Settle a Score"
//! cargo run -p sc-contracts --release --example filter_signals -- --by-debug EckhartSecurity_ShipAmbush_Nyx_VeryEasy
//! ```

use std::collections::{BTreeSet, HashMap, HashSet};

use sc_contracts::MissionIndex;
use sc_extract::generated::{
    BaseMissionPropertyValuePtr, ContractGeneratorHandlerBasePtr as H, ContractParamOverrides,
    DataPools, Handle, MissionProperty, SpawnDescription_ShipOptions, SubContract,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid, TagTree};

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
        .ok_or("usage: filter_signals [--by-debug] <substring>")?
        .to_lowercase();

    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);
    let pools = &datacore.records().pools;
    let tree = &datacore.snapshot().tag_tree;
    let db = datacore.db();
    let locale = &asset_data.locale;

    let mut by_mission: HashMap<Guid, ParamChain> = HashMap::new();
    walk_all_handlers(&datacore, &mut by_mission);

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
        eprintln!("no match");
        return Ok(());
    }

    for c in &matches {
        println!("\n══════════════════════════════════════════════════════════════");
        println!("{} [{:?}]", c.debug_name, c.title(locale).unwrap_or("?"));
        println!("  id: {}", c.id);
        println!("  origin.source_debug_name: {}", c.origin.source_debug_name);
        println!("══════════════════════════════════════════════════════════════");

        // ── Signal 1: extract tokens from debug names ────────────────
        let handler_tokens = tokenize(&c.origin.source_debug_name);
        let contract_tokens = tokenize(&c.debug_name);
        let mut name_tokens: BTreeSet<String> = BTreeSet::new();
        name_tokens.extend(handler_tokens);
        name_tokens.extend(contract_tokens);
        println!("\n[Signal 1] Debug-name tokens (handler + contract):");
        println!("  {:?}", name_tokens);

        // ── Dump raw record field shape so we know what's available ──
        dump_record_fields(&datacore, c.id, "  raw fields:");

        // ── Signal 2: Contract/SubContract tags from raw record ──────
        let raw_tags = collect_record_tags(&datacore, c.id);
        let raw_tag_names: Vec<String> = tag_names_of(&raw_tags, tree);
        println!(
            "\n[Signal 2] Contract record tags (raw): {:?}",
            raw_tag_names
        );

        // ── Signal 3: Template tags ──────────────────────────────────
        let template_tags = c
            .origin
            .subcontract_of
            .or(Some(c.id))
            .and_then(|guid| collect_template_tags(&datacore, guid))
            .unwrap_or_default();
        let template_tag_names: Vec<String> = tag_names_of(&template_tags, tree);
        println!("\n[Signal 3] Template tags: {:?}", template_tag_names);

        // ── Signal 4: handler-level prerequisites / availability tier hints
        // (Skipped for brevity — prerequisites are dumped by contract_dump.)

        // Union of all contract-side tag GUIDs.
        let mut signal_tags: HashSet<Guid> = HashSet::new();
        signal_tags.extend(raw_tags.iter().copied());
        signal_tags.extend(template_tags.iter().copied());

        // Also project debug-name tokens into a synthetic "string signal"
        // we can match against tag NAMES (since debug-name tokens like
        // "VeryEasy" / "Hard" aren't GUIDs but DO match tag names verbatim).
        let signal_name_strs: HashSet<String> = name_tokens.iter().cloned().collect();

        // ── Walk every alternatives set and try to filter ─────────────
        let Some(chain) = by_mission.get(&c.id) else {
            println!("  <no param chain>");
            continue;
        };
        println!("\n--- Alternatives filtering attempt ---");
        let mut total = 0usize;
        let mut unique_pick = 0usize;
        let mut ambiguous = 0usize;
        let mut unfilterable = 0usize;

        let _ = db;
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

            for (gi, gh) in val.spawn_descriptions.iter().enumerate() {
                let Some(group) = gh.get(pools) else { continue };
                for (si, sh) in group.ships.iter().enumerate() {
                    let Some(so) = sh.get(pools) else { continue };
                    if so.options.len() < 2 {
                        continue;
                    }
                    total += 1;

                    let outcome = try_filter(so, pools, tree, &signal_tags, &signal_name_strs);
                    println!(
                        "\n  prop='{}' group[{gi}].ships[{si}]  options={}",
                        prop.mission_variable_name,
                        so.options.len()
                    );
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
                        let matched = matches!(outcome, FilterOutcome::Single(idx) if idx == oi);
                        let marker = if matched { "→" } else { " " };
                        println!(
                            "    {marker} opt[{oi}] c={} w={:.2} score={:>2} [{}]",
                            opt.concurrent_amount,
                            opt.weight,
                            score_option(opt, pools, tree, &signal_tags, &signal_name_strs),
                            tag_names.join(",")
                        );
                    }
                    match outcome {
                        FilterOutcome::Single(idx) => {
                            println!(
                                "    ✓ unique pick: opt[{idx}] (highest-scoring distinct match)"
                            );
                            unique_pick += 1;
                        }
                        FilterOutcome::Tied(idxs) => {
                            println!("    ≈ tied between {idxs:?} (signals don't disambiguate)");
                            ambiguous += 1;
                        }
                        FilterOutcome::NoSignal => {
                            println!("    ? no contract-side signal overlaps any alternative");
                            unfilterable += 1;
                        }
                    }
                }
            }
        }
        println!(
            "\n--- Summary for {}: {total} alternatives sets, {unique_pick} uniquely filterable, {ambiguous} tied, {unfilterable} no-signal ---",
            c.debug_name
        );
    }

    Ok(())
}

#[derive(Debug)]
enum FilterOutcome {
    Single(usize),
    Tied(Vec<usize>),
    NoSignal,
}

/// Score an option by how many of its tags overlap with contract-side
/// signals — either as GUIDs (exact tag-record match) or as names
/// (debug-name token match against tag name).
fn score_option(
    opt: &sc_extract::generated::SpawnDescription_Ship,
    pools: &DataPools,
    tree: &TagTree,
    signal_tags: &HashSet<Guid>,
    signal_names: &HashSet<String>,
) -> usize {
    let Some(tl) = opt.tags.as_ref().and_then(|h| h.get(pools)) else {
        return 0;
    };
    let mut score = 0;
    for g in &tl.tags {
        if signal_tags.contains(g) {
            score += 2; // GUID match is stronger than name match
        }
        if let Some(node) = tree.get(g)
            && signal_names.contains(&node.name)
        {
            score += 1;
        }
    }
    score
}

fn try_filter(
    so: &SpawnDescription_ShipOptions,
    pools: &DataPools,
    tree: &TagTree,
    signal_tags: &HashSet<Guid>,
    signal_names: &HashSet<String>,
) -> FilterOutcome {
    let scores: Vec<usize> = so
        .options
        .iter()
        .map(|h| {
            h.get(pools)
                .map(|opt| score_option(opt, pools, tree, signal_tags, signal_names))
                .unwrap_or(0)
        })
        .collect();
    let max = *scores.iter().max().unwrap_or(&0);
    if max == 0 {
        return FilterOutcome::NoSignal;
    }
    let winners: Vec<usize> = scores
        .iter()
        .enumerate()
        .filter(|&(_, &s)| s == max)
        .map(|(i, _)| i)
        .collect();
    if winners.len() == 1 {
        FilterOutcome::Single(winners[0])
    } else {
        FilterOutcome::Tied(winners)
    }
}

fn tokenize(name: &str) -> Vec<String> {
    // Emit both the underscore-separated chunks (which preserve compounds
    // like "VeryEasy") AND the CamelCase-split parts ("Very", "Easy"),
    // so tag names that are compounds match alongside tags that use the
    // separate parts.
    let mut out: Vec<String> = Vec::new();
    for chunk in name.split(|c: char| c == '_' || c.is_ascii_digit()) {
        if chunk.is_empty() {
            continue;
        }
        out.push(chunk.to_string()); // preserve compound
        let mut buf = String::new();
        let chars: Vec<char> = chunk.chars().collect();
        for (i, &ch) in chars.iter().enumerate() {
            if i > 0 && ch.is_ascii_uppercase() {
                let prev = chars[i - 1];
                let next = chars.get(i + 1).copied();
                if prev.is_ascii_lowercase()
                    || (prev.is_ascii_uppercase() && next.is_some_and(|n| n.is_ascii_lowercase()))
                {
                    buf.push('_');
                }
            }
            buf.push(ch);
        }
        for tok in buf.split('_') {
            if !tok.is_empty() && tok != chunk {
                out.push(tok.to_string());
            }
        }
    }
    out
}

fn collect_record_tags(datacore: &sc_extract::Datacore, guid: Guid) -> Vec<Guid> {
    let db = datacore.db();
    let Some(rec) = db.record(&guid) else {
        return Vec::new();
    };
    let inst = rec.as_instance();
    let mut out: Vec<Guid> = Vec::new();
    // Common shapes: `tags: TagList`, or `taglist: TagList`.
    for field in &["tags", "taglist", "Tags", "TagList"] {
        if let Some(tl) = inst.get_instance(field)
            && let Some(arr) = tl.get_array("tags")
        {
            for v in arr {
                if let Some(r) = v.as_record_ref() {
                    out.push(r.guid);
                }
            }
        }
    }
    out
}

fn collect_template_tags(
    datacore: &sc_extract::Datacore,
    contract_guid: Guid,
) -> Option<Vec<Guid>> {
    // Walk contract record → template field → tags.
    let db = datacore.db();
    let rec = db.record(&contract_guid)?;
    let inst = rec.as_instance();
    let tpl_ref = inst.get("template").and_then(|v| v.as_record_ref())?;
    let tpl_rec = db.record(&tpl_ref.guid)?;
    let tpl_inst = tpl_rec.as_instance();
    let mut out: Vec<Guid> = Vec::new();
    if let Some(tl) = tpl_inst.get_instance("tags")
        && let Some(arr) = tl.get_array("tags")
    {
        for v in arr {
            if let Some(r) = v.as_record_ref() {
                out.push(r.guid);
            }
        }
    }
    Some(out)
}

fn dump_record_fields(datacore: &sc_extract::Datacore, guid: Guid, label: &str) {
    let db = datacore.db();
    let Some(rec) = db.record(&guid) else {
        println!("{label} <no record for {guid}>");
        return;
    };
    let inst = rec.as_instance();
    println!("{label}");
    for prop in inst.properties() {
        let n = prop.name;
        let v = prop.value;
        let kind = match &v {
            sc_contracts::raw::Value::Class { .. } => "Class".into(),
            sc_contracts::raw::Value::ClassRef(_) => "ClassRef".into(),
            sc_contracts::raw::Value::StrongPointer(Some(_)) => "StrongPtr".into(),
            sc_contracts::raw::Value::WeakPointer(Some(_)) => "WeakPtr".into(),
            sc_contracts::raw::Value::Reference(Some(_)) => "Ref".into(),
            sc_contracts::raw::Value::String(s) => format!("String={:?}", s),
            sc_contracts::raw::Value::Bool(b) => format!("Bool={b}"),
            sc_contracts::raw::Value::Int32(i) => format!("Int32={i}"),
            sc_contracts::raw::Value::Float(f) => format!("F32={f}"),
            sc_contracts::raw::Value::Guid(g) => format!("Guid={g}"),
            sc_contracts::raw::Value::Locale(s) => format!("Locale={:?}", s),
            sc_contracts::raw::Value::Enum(s) => format!("Enum={:?}", s),
            sc_contracts::raw::Value::Array(_) => "Array".into(),
            _ => "<other>".into(),
        };
        println!("    {n}: {kind}");
    }
}

fn tag_names_of(guids: &[Guid], tree: &TagTree) -> Vec<String> {
    guids
        .iter()
        .map(|g| {
            tree.get(g)
                .map(|n| n.name.clone())
                .unwrap_or_else(|| "?".into())
        })
        .collect()
}

// ── Param-chain walker (same as options_mission_map.rs) ─────────────────

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

fn walk_all_handlers(datacore: &sc_extract::Datacore, out: &mut HashMap<Guid, ParamChain>) {
    let pools = &datacore.records().pools;
    for (_g, gh) in &datacore.records().records.multi_feature.contract_generator {
        let Some(generator) = gh.get(pools) else {
            continue;
        };
        for handler_ptr in &generator.generators {
            match handler_ptr {
                H::ContractGeneratorHandler_Legacy(h) => {
                    let Some(handler) = h.get(pools) else {
                        continue;
                    };
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
                    let Some(handler) = h.get(pools) else {
                        continue;
                    };
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
                    h.get(pools)
                        .map(|h| (h.contract_params.clone(), h.contracts.clone())),
                    pools,
                    out,
                ),
                H::ContractGeneratorHandler_LinearSeries(h) => walk_list_like(
                    h.get(pools)
                        .map(|h| (h.contract_params.clone(), h.contracts.clone())),
                    pools,
                    out,
                ),
                H::ContractGeneratorHandler_TutorialSeriesDef(h) => walk_list_like(
                    h.get(pools)
                        .map(|h| (h.contract_params.clone(), h.contracts.clone())),
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
