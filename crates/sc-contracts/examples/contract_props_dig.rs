//! Dump every typed + raw property reachable from one mission, then
//! intersect with the per-option tags inside its alternatives to see
//! which contract-side properties (NOT debug-name tokens) could
//! deterministically pick one alternative.
//!
//! Walks:
//!   1. The Contract record (typed pool entry: fields, contract_results)
//!   2. Contract.template (raw record — usually a MissionTemplate)
//!      → ALL its properties (no field cherry-picking)
//!   3. The ContractGenerator (top-level record)
//!   4. The ContractGeneratorHandler (typed pool entry + raw walk)
//!   5. The ContractAvailability + ContractParamOverrides
//!   6. The handler-level prerequisites
//!
//! Then dumps every alternative's tag set and runs an intersection
//! against the GUIDs we collected from steps 1-6.
//!
//! ```bash
//! cargo run -p sc-contracts --release --example contract_props_dig -- "Settle a Score"
//! ```

use std::collections::{HashMap, HashSet};

use sc_contracts::MissionIndex;
use sc_extract::generated::{
    BaseMissionPropertyValuePtr, ContractGeneratorHandlerBasePtr as H, ContractParamOverrides,
    DataPools, Handle, MissionProperty, SubContract,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

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
        .ok_or("usage: contract_props_dig [--by-debug] <substring>")?
        .to_lowercase();

    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);
    let locale = &asset_data.locale;
    let pools = &datacore.records().pools;
    let tree = &datacore.snapshot().tag_tree;
    let db = datacore.db();

    // Map mission → param chain + parent contract handle.
    let mut by_mission: HashMap<Guid, MissionContext> = HashMap::new();
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
        let Some(ctx) = by_mission.get(&c.id) else {
            continue;
        };

        println!("\n══════════════════════════════════════════════════════════════");
        println!("{} [{:?}]", c.debug_name, c.title(locale).unwrap_or("?"));
        println!("══════════════════════════════════════════════════════════════");

        // Collect all reachable Guid + tag-name references from the
        // contract-side context. Anything we find here is "contract data"
        // — fair game for alternative filtering.
        let mut signal_guids: HashSet<Guid> = HashSet::new();
        let mut sources: Vec<(String, Guid)> = Vec::new(); // (where_from, guid)

        // 1) Typed Contract fields (from pool) — Contract / ContractLegacy / CareerContract
        println!("\n── (1) Contract typed fields ──");
        let mut template_guid: Option<Guid> = None;
        if let Some(c) = ctx
            .parent_contract_handle
            .as_ref()
            .and_then(|h| h.get(pools))
        {
            println!("  [Contract] template:               {:?}", c.template);
            println!(
                "  [Contract] additional_prerequisites: {} items",
                c.additional_prerequisites.len()
            );
            println!(
                "  [Contract] sub_contracts:           {} items",
                c.sub_contracts.len()
            );
            template_guid = c.template;
        }
        if let Some(c) = ctx.parent_legacy_handle.as_ref().and_then(|h| h.get(pools)) {
            println!("  [ContractLegacy] template:          {:?}", c.template);
            println!(
                "  [ContractLegacy] additional_prerequisites: {} items",
                c.additional_prerequisites.len()
            );
            println!(
                "  [ContractLegacy] sub_contracts:     {} items",
                c.sub_contracts.len()
            );
            template_guid = c.template.or(template_guid);
        }
        if let Some(c) = ctx.parent_career_handle.as_ref().and_then(|h| h.get(pools)) {
            println!("  [CareerContract] template:          {:?}", c.template);
            println!(
                "  [CareerContract] additional_prerequisites: {} items",
                c.additional_prerequisites.len()
            );
            println!(
                "  [CareerContract] sub_contracts:     {} items",
                c.sub_contracts.len()
            );
            template_guid = c.template.or(template_guid);
        }
        let sub_contract = ctx.sub_contract_handle.as_ref().and_then(|h| h.get(pools));
        if let Some(s) = sub_contract {
            println!("  subContract.id:                   {}", s.id);
            println!(
                "  subContract.additional_prerequisites: {} items",
                s.additional_prerequisites.len()
            );
            println!(
                "  subContract.property_overrides:   {} items",
                s.property_overrides.len()
            );
        }
        if let Some(t) = template_guid {
            signal_guids.insert(t);
            sources.push(("contract.template".into(), t));
        }

        // 2) Template raw record (and ALL its top-level properties)
        if let Some(tg) = template_guid {
            println!("\n── (2) Template raw fields ({tg}) ──");
            if let Some(rec) = db.record(&tg) {
                let inst = rec.as_instance();
                dump_instance_deep(&inst, "template", 0, 4, db, &mut signal_guids, &mut sources);
            } else {
                println!("  <template record not found>");
            }
        } else {
            println!("\n── (2) Template: NONE on contract ──");
        }

        // 3) ContractGenerator (top-level) raw record
        println!(
            "\n── (3) ContractGenerator record ({}) ──",
            c.origin.generator_id
        );
        if let Some(rec) = db.record(&c.origin.generator_id) {
            let inst = rec.as_instance();
            for prop in inst.properties() {
                println!("  {}: {}", prop.name, describe_value(&prop.value));
                collect_guids_from_value(
                    &prop.value,
                    db,
                    &mut signal_guids,
                    &mut sources,
                    &format!("generator.{}", prop.name),
                );
            }
        } else {
            println!("  <not found>");
        }

        // 4) Handler-level: ContractParamOverrides + ContractAvailability
        if let Some(ap) = ctx.handler_params.as_ref().and_then(|h| h.get(pools)) {
            println!("\n── (4a) handler.contractParams ──");
            println!(
                "  bool_param_overrides: {} items",
                ap.bool_param_overrides.len()
            );
            println!(
                "  int_param_overrides:  {} items",
                ap.int_param_overrides.len()
            );
            println!(
                "  property_overrides:   {} items",
                ap.property_overrides.len()
            );
        }
        if let Some(av) = ctx.handler_availability.as_ref().and_then(|h| h.get(pools)) {
            println!("\n── (4b) handler.defaultAvailability ──");
            println!("  once_only:                       {}", av.once_only);
            println!(
                "  available_in_prison:             {}",
                av.available_in_prison
            );
            println!(
                "  hide_in_mobi_glas:               {}",
                av.hide_in_mobi_glas
            );
            println!(
                "  has_personal_cooldown:           {}",
                av.has_personal_cooldown
            );
            println!(
                "  personal_cooldown_time:          {}",
                av.personal_cooldown_time
            );
            println!(
                "  abandoned_cooldown_time:         {}",
                av.abandoned_cooldown_time
            );
            println!(
                "  prerequisites:                   {} items",
                av.prerequisites.len()
            );
        }

        // 5) ── now run the filter check ─────────────────────────────────
        let mut name_signals: HashSet<String> = HashSet::new();
        for g in &signal_guids {
            if let Some(node) = tree.get(g) {
                name_signals.insert(node.name.clone());
            }
        }
        println!("\n── (5) signal summary ──");
        println!(
            "  total contract-side GUIDs collected: {}",
            signal_guids.len()
        );
        println!(
            "  of which resolve to tag names:        {}",
            name_signals.len()
        );
        println!("  tag-name signals: {:?}", name_signals);

        println!("\n── (6) Alternatives vs. signals ──");
        if let Some(chain) = by_mission.get(&c.id) {
            for ph in chain.props_iter(pools) {
                let Some(prop) = ph.get(pools) else { continue };
                let Some(ptr) = prop.value.as_ref() else {
                    continue;
                };
                let BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(vh) =
                    ptr
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
                        println!(
                            "\n  prop='{}' group[{gi}].ships[{si}] options={}",
                            prop.mission_variable_name,
                            so.options.len()
                        );
                        for (oi, oh) in so.options.iter().enumerate() {
                            let Some(opt) = oh.get(pools) else { continue };
                            let tag_guids: HashSet<Guid> = opt
                                .tags
                                .as_ref()
                                .and_then(|h| h.get(pools))
                                .map(|tl| tl.tags.iter().copied().collect())
                                .unwrap_or_default();
                            let tag_names: Vec<String> = tag_guids
                                .iter()
                                .map(|g| {
                                    tree.get(g)
                                        .map(|n| n.name.clone())
                                        .unwrap_or_else(|| "?".into())
                                })
                                .collect();
                            let guid_hits: Vec<Guid> = tag_guids
                                .iter()
                                .filter(|g| signal_guids.contains(g))
                                .copied()
                                .collect();
                            let name_hits: Vec<&String> = tag_names
                                .iter()
                                .filter(|n| name_signals.contains(n.as_str()))
                                .collect();
                            println!(
                                "    opt[{oi}] c={} w={:.2} guid_hits={} name_hits={}  tags=[{}]",
                                opt.concurrent_amount,
                                opt.weight,
                                guid_hits.len(),
                                name_hits.len(),
                                tag_names.join(",")
                            );
                            for h in guid_hits {
                                let src = sources
                                    .iter()
                                    .find(|(_, g)| *g == h)
                                    .map(|(s, _)| s.as_str())
                                    .unwrap_or("?");
                                let tn = tree.get(&h).map(|n| n.name.as_str()).unwrap_or("?");
                                println!("      ✓ GUID hit: {tn}  (from {src})");
                            }
                            for n in name_hits {
                                println!(
                                    "      ~ name hit: {n}  (no GUID match — name happens to equal a signal tag's name)"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn describe_value(v: &sc_contracts::raw::Value<'_>) -> String {
    use sc_contracts::raw::Value as V;
    match v {
        V::Bool(b) => format!("Bool={b}"),
        V::Int8(i) => format!("I8={i}"),
        V::Int16(i) => format!("I16={i}"),
        V::Int32(i) => format!("I32={i}"),
        V::Int64(i) => format!("I64={i}"),
        V::UInt8(i) => format!("U8={i}"),
        V::UInt16(i) => format!("U16={i}"),
        V::UInt32(i) => format!("U32={i}"),
        V::UInt64(i) => format!("U64={i}"),
        V::Float(f) => format!("F32={f}"),
        V::Double(d) => format!("F64={d}"),
        V::String(s) => format!("String={s:?}"),
        V::Locale(s) => format!("Locale={s:?}"),
        V::Enum(s) => format!("Enum={s:?}"),
        V::Guid(g) => format!("Guid={g}"),
        V::Class { struct_index, .. } => format!("Class[struct={struct_index}]"),
        V::ClassRef(r) => format!(
            "ClassRef[struct={},instance={}]",
            r.struct_index, r.instance_index
        ),
        V::StrongPointer(Some(r)) => format!(
            "StrongPtr[struct={},instance={}]",
            r.struct_index, r.instance_index
        ),
        V::WeakPointer(Some(r)) => format!(
            "WeakPtr[struct={},instance={}]",
            r.struct_index, r.instance_index
        ),
        V::Reference(Some(r)) => format!("Ref→{}", r.guid),
        V::Array(_) => "Array".into(),
        _ => "<null/other>".into(),
    }
}

fn dump_instance_deep(
    inst: &sc_contracts::raw::Instance<'_>,
    label: &str,
    depth: usize,
    max_depth: usize,
    db: &sc_contracts::raw::DataCoreDatabase,
    signal_guids: &mut HashSet<Guid>,
    sources: &mut Vec<(String, Guid)>,
) {
    use sc_contracts::raw::Value as V;
    let indent = "  ".repeat(depth);
    for prop in inst.properties() {
        let v = describe_value(&prop.value);
        println!("{indent}{label}.{}: {v}", prop.name);
        let sub_label = format!("{label}.{}", prop.name);
        collect_guids_from_value(&prop.value, db, signal_guids, sources, &sub_label);
        if depth + 1 > max_depth {
            continue;
        }
        // Recurse into inline Class, ClassRef, AND StrongPointer/WeakPointer.
        match &prop.value {
            V::Class { struct_index, data } => {
                let nested = sc_contracts::raw::Instance::from_inline_data(db, *struct_index, data);
                dump_instance_deep(
                    &nested,
                    &sub_label,
                    depth + 1,
                    max_depth,
                    db,
                    signal_guids,
                    sources,
                );
            }
            V::ClassRef(r) => {
                let nested = db.instance(r.struct_index, r.instance_index);
                dump_instance_deep(
                    &nested,
                    &sub_label,
                    depth + 1,
                    max_depth,
                    db,
                    signal_guids,
                    sources,
                );
            }
            V::StrongPointer(Some(r)) | V::WeakPointer(Some(r)) => {
                let nested = db.instance(r.struct_index, r.instance_index);
                let sname = db
                    .struct_name(r.struct_index.try_into().unwrap_or(0))
                    .unwrap_or("?");
                println!("{indent}  ↳ {sub_label} (StrongPtr → {sname})");
                dump_instance_deep(
                    &nested,
                    &sub_label,
                    depth + 1,
                    max_depth,
                    db,
                    signal_guids,
                    sources,
                );
            }
            V::Array(_) => {
                if let Some(arr) = inst.get_array(prop.name) {
                    for (i, av) in arr.enumerate() {
                        match av {
                            V::Class { struct_index, data } => {
                                let nested = sc_contracts::raw::Instance::from_inline_data(
                                    db,
                                    struct_index,
                                    data,
                                );
                                dump_instance_deep(
                                    &nested,
                                    &format!("{sub_label}[{i}]"),
                                    depth + 1,
                                    max_depth,
                                    db,
                                    signal_guids,
                                    sources,
                                );
                            }
                            V::ClassRef(r) => {
                                let nested = db.instance(r.struct_index, r.instance_index);
                                dump_instance_deep(
                                    &nested,
                                    &format!("{sub_label}[{i}]"),
                                    depth + 1,
                                    max_depth,
                                    db,
                                    signal_guids,
                                    sources,
                                );
                            }
                            V::Reference(Some(r)) => {
                                signal_guids.insert(r.guid);
                                sources.push((format!("{sub_label}[{i}]"), r.guid));
                            }
                            V::Guid(g) => {
                                signal_guids.insert(g);
                                sources.push((format!("{sub_label}[{i}]"), g));
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn collect_guids_from_value(
    v: &sc_contracts::raw::Value<'_>,
    _db: &sc_contracts::raw::DataCoreDatabase,
    out: &mut HashSet<Guid>,
    sources: &mut Vec<(String, Guid)>,
    label: &str,
) {
    use sc_contracts::raw::Value as V;
    match v {
        V::Guid(g) => {
            out.insert(*g);
            sources.push((label.into(), *g));
        }
        V::Reference(Some(r)) => {
            out.insert(r.guid);
            sources.push((label.into(), r.guid));
        }
        _ => {}
    }
}

// ── Param-chain walker (shared shape with previous examples) ────────────

#[derive(Default, Clone)]
struct MissionContext {
    handler_params: Option<Handle<ContractParamOverrides>>,
    handler_availability: Option<Handle<sc_extract::generated::ContractAvailability>>,
    contract_params: Option<Handle<ContractParamOverrides>>,
    sub_contract_handle: Option<Handle<SubContract>>,
    parent_contract_handle: Option<Handle<sc_extract::generated::Contract>>,
    parent_legacy_handle: Option<Handle<sc_extract::generated::ContractLegacy>>,
    parent_career_handle: Option<Handle<sc_extract::generated::CareerContract>>,
}
impl MissionContext {
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
        if let Some(h) = self.sub_contract_handle.as_ref()
            && let Some(s) = h.get(pools)
        {
            out.extend(s.property_overrides.iter());
        }
        out
    }
}

fn walk_all_handlers(datacore: &Datacore, out: &mut HashMap<Guid, MissionContext>) {
    let pools = &datacore.records().pools;
    for (_g, gh) in &datacore.records().records.multi_feature.contract_generator {
        let Some(generator) = gh.get(pools) else {
            continue;
        };
        for handler_ptr in &generator.generators {
            match handler_ptr {
                H::ContractGeneratorHandler_Legacy(_) | H::ContractGeneratorHandler_Career(_) => {
                    // handled in second pass below
                }
                H::ContractGeneratorHandler_List(h) => take_contracts(
                    h.get(pools).map(|h| {
                        (
                            h.contract_params.clone(),
                            h.default_availability.clone(),
                            h.contracts.clone(),
                        )
                    }),
                    pools,
                    out,
                ),
                H::ContractGeneratorHandler_LinearSeries(h) => take_contracts(
                    h.get(pools).map(|h| {
                        (
                            h.contract_params.clone(),
                            h.default_availability.clone(),
                            h.contracts.clone(),
                        )
                    }),
                    pools,
                    out,
                ),
                H::ContractGeneratorHandler_TutorialSeriesDef(h) => take_contracts(
                    h.get(pools).map(|h| {
                        (
                            h.contract_params.clone(),
                            h.default_availability.clone(),
                            h.contracts.clone(),
                        )
                    }),
                    pools,
                    out,
                ),
                _ => {}
            }
        }
    }
    // Walk Legacy / Career via different code path since they have different Contract types.
    for (_g, gh) in &datacore.records().records.multi_feature.contract_generator {
        let Some(generator) = gh.get(pools) else {
            continue;
        };
        for handler_ptr in &generator.generators {
            if let H::ContractGeneratorHandler_Legacy(h) = handler_ptr
                && let Some(handler) = h.get(pools)
            {
                let hp = handler.contract_params.clone();
                let ha = handler.default_availability.clone();
                for ch in &handler.legacy_contracts {
                    let Some(c) = ch.get(pools) else { continue };
                    out.entry(c.id).or_insert(MissionContext {
                        handler_params: hp.clone(),
                        handler_availability: ha.clone(),
                        contract_params: c.param_overrides.clone(),
                        sub_contract_handle: None,
                        parent_contract_handle: None,
                        parent_legacy_handle: Some(ch.clone()),
                        parent_career_handle: None,
                    });
                    for sh in &c.sub_contracts {
                        let Some(s) = sh.get(pools) else { continue };
                        out.entry(s.id).or_insert(MissionContext {
                            handler_params: hp.clone(),
                            handler_availability: ha.clone(),
                            contract_params: c.param_overrides.clone(),
                            sub_contract_handle: Some(sh.clone()),
                            parent_contract_handle: None,
                            parent_legacy_handle: Some(ch.clone()),
                            parent_career_handle: None,
                        });
                    }
                }
            }
            if let H::ContractGeneratorHandler_Career(h) = handler_ptr
                && let Some(handler) = h.get(pools)
            {
                let hp = handler.contract_params.clone();
                let ha = handler.default_availability.clone();
                for ch in &handler.contracts {
                    let Some(c) = ch.get(pools) else { continue };
                    out.entry(c.id).or_insert(MissionContext {
                        handler_params: hp.clone(),
                        handler_availability: ha.clone(),
                        contract_params: c.param_overrides.clone(),
                        sub_contract_handle: None,
                        parent_contract_handle: None,
                        parent_legacy_handle: None,
                        parent_career_handle: Some(ch.clone()),
                    });
                    for sh in &c.sub_contracts {
                        let Some(s) = sh.get(pools) else { continue };
                        out.entry(s.id).or_insert(MissionContext {
                            handler_params: hp.clone(),
                            handler_availability: ha.clone(),
                            contract_params: c.param_overrides.clone(),
                            sub_contract_handle: Some(sh.clone()),
                            parent_contract_handle: None,
                            parent_legacy_handle: None,
                            parent_career_handle: Some(ch.clone()),
                        });
                    }
                }
            }
        }
    }
}

fn take_contracts(
    handler: Option<(
        Option<Handle<ContractParamOverrides>>,
        Option<Handle<sc_extract::generated::ContractAvailability>>,
        Vec<Handle<sc_extract::generated::Contract>>,
    )>,
    pools: &DataPools,
    out: &mut HashMap<Guid, MissionContext>,
) {
    let Some((hp, ha, contracts)) = handler else {
        return;
    };
    for ch in &contracts {
        let Some(c) = ch.get(pools) else { continue };
        out.entry(c.id).or_insert(MissionContext {
            handler_params: hp.clone(),
            handler_availability: ha.clone(),
            contract_params: c.param_overrides.clone(),
            sub_contract_handle: None,
            parent_contract_handle: Some(ch.clone()),
            parent_legacy_handle: None,
            parent_career_handle: None,
        });
        for sh in &c.sub_contracts {
            let Some(s) = sh.get(pools) else { continue };
            out.entry(s.id).or_insert(MissionContext {
                handler_params: hp.clone(),
                handler_availability: ha.clone(),
                contract_params: c.param_overrides.clone(),
                sub_contract_handle: Some(sh.clone()),
                parent_contract_handle: Some(ch.clone()),
                parent_legacy_handle: None,
                parent_career_handle: None,
            });
        }
    }
}
