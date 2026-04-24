//! Audit ship pools resolved from contract spawn queries.
//!
//! Two diagnostics, selectable by flag:
//!
//! 1. `--target-only` — list contracts with spawn options whose only
//!    positive tags are pure role markers (`Target`, `Defenders`, …)
//!    that no ship entity carries. These are the 2.6% the spec flags
//!    as genuinely unresolvable from the DCB alone (they merge with
//!    runtime location-AI-spawn tags in the game engine).
//!
//! 2. `--contains <name>` (repeatable) — list contracts whose resolved
//!    ship pool contains at least one entity whose display name or
//!    record name contains `<name>` (case-insensitive). Default set:
//!    idris, polaris, perseus.
//!
//! Run:
//!
//! ```bash
//! cargo run -p sc-contracts --release --example ship_pool_audit
//! cargo run -p sc-contracts --release --example ship_pool_audit -- --target-only
//! cargo run -p sc-contracts --release --example ship_pool_audit -- \
//!     --contains idris --contains polaris --contains perseus
//! ```
#![allow(non_snake_case)]

use std::collections::{BTreeMap, HashSet};

use sc_contracts::{
    resolve_contract_text, ContractAnchor, ShipCandidate, ShipRegistry, SpawnContext,
};
use sc_extract::generated::{
    BaseMissionPropertyValuePtr, CareerContract, Contract, ContractGeneratorHandlerBasePtr,
    ContractLegacy, ContractParamOverrides, ContractStringParamType, DataPools, Handle,
    MissionProperty, SpawnDescription_Ship, SubContract, Tag, TagList,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

// ── CLI / filters ───────────────────────────────────────────────────────────

#[derive(Default)]
struct Args {
    target_only: bool,
    contains: Vec<String>,
    limit: usize,
    dump: Option<String>,
    describe: Option<String>,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut args = Args {
        target_only: false,
        contains: Vec::new(),
        limit: 30,
        dump: None,
        describe: None,
    };
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--target-only" => args.target_only = true,
            "--contains" => {
                if let Some(v) = raw.get(i + 1) {
                    args.contains.push(v.to_lowercase());
                    i += 1;
                }
            }
            "--limit" => {
                if let Some(v) = raw.get(i + 1).and_then(|s| s.parse().ok()) {
                    args.limit = v;
                    i += 1;
                }
            }
            "--dump" => {
                if let Some(v) = raw.get(i + 1) {
                    args.dump = Some(v.clone());
                    i += 1;
                }
            }
            "--describe" => {
                if let Some(v) = raw.get(i + 1) {
                    args.describe = Some(v.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    // Default set if no flag supplied.
    if args.dump.is_none()
        && args.describe.is_none()
        && !args.target_only
        && args.contains.is_empty()
    {
        args.contains = vec!["idris".into(), "polaris".into(), "perseus".into()];
    }
    args
}

// ── Visit context ───────────────────────────────────────────────────────────

#[derive(Clone)]
struct Ctx {
    handler_debug: String,
    handler_kind: &'static str,
    contract_title: String,
    contract_debug: String,
    /// Human-readable origin (handler vs contract vs sub_contract).
    origin: &'static str,
}

struct Option_<'p> {
    ctx: Ctx,
    opt: &'p SpawnDescription_Ship,
    group_name: String,
}

// ── Entry ───────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let args = parse_args();

    let install = sc_installs::discover_primary()?;
    println!(
        "Found {} v{} at {}",
        install.channel,
        install.short_version(),
        install.root.display()
    );
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let pools = &datacore.records().pools;
    let ships = ShipRegistry::build(&datacore);
    println!("ShipRegistry: {} entities\n", ships.len());

    // Collect every spawn option with its contract context.
    let mut options: Vec<Option_<'_>> = Vec::new();
    for gen_slot in pools.multi_feature.contract_generator.iter().flatten() {
        for handler_ptr in &gen_slot.generators {
            walk_handler(pools, handler_ptr, &mut options);
        }
    }
    println!("Collected {} spawn options from contract graph.\n", options.len());

    if args.target_only {
        print_target_only(&options, &ships, pools, &datacore, args.limit);
    }

    for needle in &args.contains {
        print_ship_existence(needle, &ships, &datacore);
        print_contains(&options, &ships, pools, needle, args.limit);
    }

    if let Some(name) = &args.dump {
        dump_contract(&options, &ships, pools, &datacore, name);
        explain_contract(&options, pools, &datacore, name);
    }

    if let Some(pattern) = &args.describe {
        describe_contracts(&options, &ships, pools, &datacore, &asset_data, pattern, args.limit);
        search_locale_for_pattern(&asset_data, pattern, 20);
    }

    Ok(())
}

/// Search the locale map for keys whose stripped name contains
/// `pattern` (case-insensitive) AND whose value looks like a mission
/// title — specifically, values containing `[%` or starting with `[`.
/// Surfaces the `[SHIP] Clean Up` templated strings directly.
fn search_locale_for_pattern(asset_data: &AssetData, pattern: &str, limit: usize) {
    let needle = pattern.to_lowercase();
    let mut matches: Vec<(&str, &str)> = Vec::new();
    for (k, v) in asset_data.locale.iter() {
        if !k.to_lowercase().contains(&needle) {
            continue;
        }
        matches.push((k, v));
    }
    matches.sort();

    println!("\n═══════════════════════════════════════════════════");
    println!("  Locale keys matching '{pattern}' (first {limit})");
    println!("═══════════════════════════════════════════════════");
    println!("  {} match(es) total", matches.len());
    for (k, v) in matches.iter().take(limit) {
        let trimmed = if v.len() > 160 {
            format!("{}…", &v[..160])
        } else {
            v.to_string()
        };
        let trimmed = trimmed.replace("\\n", " / ");
        println!("  {k}");
        println!("      {trimmed}");
    }
    if matches.len() > limit {
        println!("  … +{} more", matches.len() - limit);
    }
}

/// For every contract whose handler / debug_name / title matches `pattern`,
/// print the resolved title + description text alongside the spawn pool
/// and candidate ships per wave. Lets us eyeball text-vs-pool
/// relationships (e.g. "description mentions Caterpillar but pool has
/// Starfarer").
fn describe_contracts(
    options: &[Option_<'_>],
    ships: &ShipRegistry,
    pools: &DataPools,
    datacore: &Datacore,
    asset_data: &AssetData,
    pattern: &str,
    limit: usize,
) {
    use std::collections::{BTreeMap, HashSet};
    let needle = pattern.to_lowercase();
    let tree = &datacore.snapshot().tag_tree;

    // Group options by contract — one block per contract, multiple waves inside.
    let mut by_contract: BTreeMap<(String, String), Vec<&Option_<'_>>> = BTreeMap::new();
    for o in options {
        let matches = o.ctx.handler_debug.to_lowercase().contains(&needle)
            || o.ctx.contract_debug.to_lowercase().contains(&needle)
            || o.ctx.contract_title.to_lowercase().contains(&needle);
        if !matches {
            continue;
        }
        by_contract
            .entry((o.ctx.handler_debug.clone(), o.ctx.contract_debug.clone()))
            .or_default()
            .push(o);
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("  Describe: contracts matching '{pattern}'");
    println!("═══════════════════════════════════════════════════");
    println!("  {} contract(s) matched\n", by_contract.len());

    for (i, ((hd, cd), opts)) in by_contract.iter().take(limit).enumerate() {
        // Resolve title + description via the full inheritance chain
        // (sub-contract → contract → handler → template).
        let resolved = resolve_text_for_contract(pools, datacore, &asset_data.locale, hd, cd);
        let title_text = resolved
            .title
            .clone()
            .unwrap_or_else(|| "<no title>".to_string());
        let desc_text = resolved.description.clone().unwrap_or_default();
        let has_subst = resolved.has_runtime_substitution();

        let cd_show = if cd.is_empty() { "<handler-level>" } else { cd.as_str() };
        println!("{}", "─".repeat(72));
        println!(
            "{i:>3}. handler='{hd}' contract='{cd_show}' kind={kind}",
            i = i + 1,
            kind = opts[0].ctx.handler_kind,
        );
        let subst_marker = if has_subst { "  [runtime substitution]" } else { "" };
        println!("     title:       {title_text}{subst_marker}");
        if !desc_text.is_empty() {
            let display = desc_text.replace("\\n", "\n                  ");
            println!("     description: {display}");
        } else {
            println!("     description: <none>");
        }
        println!();

        // Group waves within this contract.
        let mut waves: BTreeMap<(String, &'static str), Vec<&Option_<'_>>> = BTreeMap::new();
        for o in opts {
            waves.entry((o.group_name.clone(), o.ctx.origin)).or_default().push(o);
        }
        for ((wave_name, origin), ws) in &waves {
            let wave_show = if wave_name.is_empty() {
                "<no name>"
            } else {
                wave_name.as_str()
            };
            println!("     wave={wave_show}  origin={origin}  options={}", ws.len());
            for o in ws {
                let pos = tags_set(pools, o.opt.tags.as_ref());
                let neg = tags_set(pools, o.opt.negative_tags.as_ref());
                let candidates = ships.resolve_spawn(&pos, &neg);
                let ctx = SpawnContext::classify(tree, &pos);
                let mut names: Vec<&str> = candidates.iter().map(|c| c.display_name.as_str()).collect();
                names.sort();
                names.dedup();

                let ship_str = if names.is_empty() {
                    "<none>".to_string()
                } else if names.len() > 10 {
                    format!("{}, … (+{} more)", names[..10].join(", "), names.len() - 10)
                } else {
                    names.join(", ")
                };
                print!("       concurrent×{c:<2} → ships: {ship_str}", c = o.opt.concurrent_amount);

                // Compact context on same line.
                let mut tail = Vec::new();
                if let Some(s) = ctx.ai_skill {
                    tail.push(format!("skill={}", s));
                }
                if ctx.ace_pilot {
                    tail.push("Ace".into());
                }
                if !ctx.factions.is_empty() {
                    tail.push(format!("factions={}", ctx.factions.join("+")));
                }
                if !ctx.cargo.is_empty() {
                    tail.push(format!("cargo={}", ctx.cargo.join("+")));
                }
                if !tail.is_empty() {
                    println!("  [{}]", tail.join(" "));
                } else {
                    println!();
                }

                // Diagnostic: do any ship names in the description text
                // match the candidate ship model names?
                let mentioned = mentions_ship_in_text(&desc_text, tree, &pos);
                if !mentioned.is_empty() {
                    let in_pool: HashSet<String> =
                        names.iter().map(|n| n.to_lowercase()).collect();
                    let missing: Vec<&str> = mentioned
                        .iter()
                        .filter(|m| !in_pool.iter().any(|p| p.contains(&m.to_lowercase())))
                        .map(String::as_str)
                        .collect();
                    if !missing.is_empty() {
                        println!(
                            "         ⚠ description mentions {} but no candidate matches that name",
                            missing.join(", ")
                        );
                    }
                }
            }
        }
        println!();
    }
}

/// Resolve title + description for a contract by debug_name, walking
/// the full inheritance chain via [`sc_contracts::resolve_contract_text`].
/// Also returns the handler's `contractParams` if the contract lives
/// under a handler we know about (needed for Level 3 inheritance).
fn resolve_text_for_contract(
    pools: &DataPools,
    datacore: &Datacore,
    locale: &sc_extract::LocaleMap,
    handler_debug: &str,
    contract_debug: &str,
) -> sc_contracts::ResolvedText {
    // Find the contract anchor by debug_name across the three pools.
    let anchor = find_contract_anchor(pools, contract_debug);
    let Some(anchor) = anchor else {
        return sc_contracts::ResolvedText::default();
    };

    // Find the handler's contractParams — match by handler debug_name.
    let handler_params = find_handler_contract_params(pools, handler_debug);

    resolve_contract_text(None, anchor, handler_params.as_ref(), datacore, locale)
}

fn find_contract_anchor<'p>(
    pools: &'p DataPools,
    debug_name: &str,
) -> Option<ContractAnchor<'p>> {
    if debug_name.is_empty() {
        return None;
    }
    for c in pools.multi_feature.contract.iter().flatten() {
        if c.debug_name == debug_name {
            return Some(ContractAnchor::Contract(c));
        }
    }
    for c in pools.contracts.contract_legacy.iter().flatten() {
        if c.debug_name == debug_name {
            return Some(ContractAnchor::ContractLegacy(c));
        }
    }
    for c in pools.contracts.career_contract.iter().flatten() {
        if c.debug_name == debug_name {
            return Some(ContractAnchor::CareerContract(c));
        }
    }
    None
}

fn find_handler_contract_params(
    pools: &DataPools,
    handler_debug: &str,
) -> Option<sc_extract::generated::Handle<sc_extract::generated::ContractParamOverrides>> {
    if handler_debug.is_empty() {
        return None;
    }
    for h in pools.contracts.contract_generator_handler_legacy.iter().flatten() {
        if h.debug_name == handler_debug {
            return h.contract_params.clone();
        }
    }
    for h in pools.contracts.contract_generator_handler_career.iter().flatten() {
        if h.debug_name == handler_debug {
            return h.contract_params.clone();
        }
    }
    for h in pools.contracts.contract_generator_handler_list.iter().flatten() {
        if h.debug_name == handler_debug {
            return h.contract_params.clone();
        }
    }
    for h in pools.contracts.contract_generator_handler_linear_series.iter().flatten() {
        if h.debug_name == handler_debug {
            return h.contract_params.clone();
        }
    }
    for h in pools.contracts.contract_generator_handler_tutorial_series_def.iter().flatten() {
        if h.debug_name == handler_debug {
            return h.contract_params.clone();
        }
    }
    None
}

/// Legacy helper — kept for reference from earlier diagnostic.
#[allow(dead_code)]
fn resolve_template_title(pools: &DataPools, datacore: &Datacore, o: &Option_<'_>) -> String {
    // We need the contract's template guid. The audit walker doesn't
    // carry it through — re-read it via svarog on the contract_debug.
    // Pragmatic shortcut: search the generated Contract pools for one
    // whose debug_name matches, then follow its template field.
    let cd = &o.ctx.contract_debug;
    if cd.is_empty() {
        return String::new();
    }

    let template_guid = find_template_for_contract(pools, cd);
    let Some(guid) = template_guid else {
        return String::new();
    };

    // Walk the template record via svarog raw layer.
    let db = datacore.db();
    let Some(record) = db.record(&guid) else {
        return String::new();
    };
    let inst = record.as_instance();

    // Template has a `stringParamOverrides` (same shape as
    // ContractParamOverrides). Scan for `Title` param.
    let Some(overrides) = inst.get_array("stringParamOverrides") else {
        return String::new();
    };
    use sc_extract::svarog_datacore::Value;
    for v in overrides {
        let (struct_index, data) = match v {
            Value::Class { struct_index, data } => (struct_index, data),
            _ => continue,
        };
        let param = sc_extract::svarog_datacore::Instance::from_inline_data(db, struct_index, data);
        let param_name = match param.get("param") {
            Some(Value::Enum(e)) => e.to_string(),
            Some(Value::String(s)) => s.to_string(),
            _ => continue,
        };
        if param_name == "Title" {
            if let Some(Value::String(s)) = param.get("value") {
                return s.strip_prefix('@').unwrap_or(s).to_string();
            }
            if let Some(Value::Locale(s)) = param.get("value") {
                return s.strip_prefix('@').unwrap_or(s).to_string();
            }
        }
    }
    String::new()
}

fn find_template_for_contract(pools: &DataPools, debug_name: &str) -> Option<Guid> {
    // Search all three contract pools for one whose debug_name matches.
    for c in pools.multi_feature.contract.iter().flatten() {
        if c.debug_name == debug_name {
            return c.template;
        }
    }
    for c in pools.contracts.contract_legacy.iter().flatten() {
        if c.debug_name == debug_name {
            return c.template;
        }
    }
    for c in pools.contracts.career_contract.iter().flatten() {
        if c.debug_name == debug_name {
            return c.template;
        }
    }
    None
}

/// Check whether the description text mentions any ship-model tag name.
/// Returns the list of model tag names that appear in the text.
fn mentions_ship_in_text(desc: &str, tree: &sc_extract::TagTree, tags: &std::collections::HashSet<Guid>) -> Vec<String> {
    if desc.is_empty() {
        return Vec::new();
    }
    let desc_lower = desc.to_lowercase();
    let mut mentions = Vec::new();
    for guid in tags {
        let Some(node) = tree.get(guid) else { continue };
        let path = tree.path(guid);
        // Only check ship-tree tags — those are ship-model names.
        if path.first() != Some(&"Ship") {
            continue;
        }
        let n = node.name.to_lowercase();
        // Strip underscores so `Cat_Caterpillar` matches "caterpillar".
        let normalised = n.replace('_', " ");
        if desc_lower.contains(&normalised) || desc_lower.contains(&n) {
            mentions.push(node.name.clone());
        }
    }
    mentions
}

/// Walk the `children` graph upward from a tag to find its root ancestor.
/// Returns the root tag's name, or `None` if the tag isn't in the tag pool.
fn find_tag_root(datacore: &Datacore, start: Guid) -> Option<String> {
    use std::collections::HashMap;
    let pools = &datacore.records().pools;
    let records = &datacore.records().records.multi_feature.tag;

    // Compute parent_of (cached per-call — ok for diagnostic).
    let mut by_guid: HashMap<Guid, &Tag> = HashMap::new();
    for (guid, handle) in records {
        if let Some(tag) = handle.get(pools) {
            by_guid.insert(*guid, tag);
        }
    }
    let mut parent_of: HashMap<Guid, Guid> = HashMap::new();
    for (guid, tag) in &by_guid {
        for child in &tag.children {
            parent_of.insert(*child, *guid);
        }
    }

    let mut current = start;
    let mut guard = 0;
    while let Some(p) = parent_of.get(&current) {
        current = *p;
        guard += 1;
        if guard > 50 {
            return None;
        }
    }
    by_guid.get(&current).map(|t| t.tag_name.clone())
}

/// Walk every `Tag` record and print the top-level hierarchy — tags with
/// children, grouped by how many descendants they dominate. This tells
/// us whether the DCB has a ship-tag subtree we can use to classify
/// tags as ship-selective vs faction/context.
#[allow(dead_code)]
fn dump_tag_hierarchy(datacore: &Datacore) {
    use std::collections::{HashMap, HashSet};
    let pools = &datacore.records().pools;
    let tag_pool = &pools.multi_feature.tag;

    // Map guid → tag handle position for cross-lookup.
    let records = &datacore.records().records.multi_feature.tag;
    let mut by_guid: HashMap<Guid, &Tag> = HashMap::new();
    for (guid, handle) in records {
        if let Some(tag) = handle.get(pools) {
            by_guid.insert(*guid, tag);
        }
    }

    // Compute parent links from children arrays.
    let mut parent_of: HashMap<Guid, Guid> = HashMap::new();
    for (guid, tag) in &by_guid {
        for child_guid in &tag.children {
            parent_of.insert(*child_guid, *guid);
        }
    }

    // Find roots (tags with no parent).
    let mut roots: Vec<Guid> = by_guid
        .keys()
        .filter(|g| !parent_of.contains_key(g))
        .copied()
        .collect();
    roots.sort_by_key(|g| by_guid.get(g).map(|t| t.tag_name.clone()).unwrap_or_default());

    // Count total descendants for each root (transitive closure).
    fn count_descendants(
        g: &Guid,
        by_guid: &HashMap<Guid, &Tag>,
        seen: &mut HashSet<Guid>,
    ) -> usize {
        if !seen.insert(*g) {
            return 0;
        }
        let Some(tag) = by_guid.get(g) else { return 1 };
        let mut total = 1;
        for c in &tag.children {
            total += count_descendants(c, by_guid, seen);
        }
        total
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("  Tag hierarchy — top-level roots with descendants");
    println!("═══════════════════════════════════════════════════");
    println!("  Total Tag records: {} (pool size: {})", by_guid.len(), tag_pool.len());
    println!("  Roots (no parent): {}\n", roots.len());

    let mut root_sizes: Vec<(Guid, String, usize)> = roots
        .iter()
        .map(|g| {
            let name = by_guid
                .get(g)
                .map(|t| t.tag_name.clone())
                .unwrap_or_default();
            let mut seen = HashSet::new();
            let size = count_descendants(g, &by_guid, &mut seen);
            (*g, name, size)
        })
        .collect();
    root_sizes.sort_by(|a, b| b.2.cmp(&a.2));

    for (guid, name, size) in root_sizes.iter().take(20) {
        println!("  {size:>6}  {name}");
        if ["AI", "Missions", "Ship"].contains(&name.as_str()) {
            if let Some(root_tag) = by_guid.get(guid) {
                let mut child_names: Vec<(String, usize)> = root_tag
                    .children
                    .iter()
                    .filter_map(|c| {
                        by_guid.get(c).map(|t| {
                            let mut seen = HashSet::new();
                            (t.tag_name.clone(), count_descendants(c, &by_guid, &mut seen))
                        })
                    })
                    .collect();
                child_names.sort_by(|a, b| b.1.cmp(&a.1));
                for (n, c) in child_names.iter().take(10) {
                    println!("           ▸ {c:>5}  {n}");
                }
            }
        }
    }
    println!();
}

/// For each matching wave, break down per-tag: how many ship-pool entities
/// carry it, how many full-DCB EntityClassDefinitions carry it, and
/// whether it's in the data-derived `spawn_referenced_tags`.
///
/// Also lists entities matching the LEAST-common positive tag across the
/// full DCB — that's usually the ship-model selector, and the right
/// candidate set.
fn explain_contract(
    options: &[Option_<'_>],
    pools: &DataPools,
    datacore: &Datacore,
    name: &str,
) {
    use std::collections::HashMap;
    let needle = name.to_lowercase();
    let tree = &datacore.snapshot().tag_tree;
    let db = datacore.db();

    // Pre-compute per-tag carrier counts across the full EntityClassDefinition pool.
    let mut full_carrier_count: HashMap<Guid, usize> = HashMap::new();
    let mut full_carriers: HashMap<Guid, Vec<Guid>> = HashMap::new();
    for (guid, handle) in &datacore.records().records.multi_feature.entity_class_definition {
        let Some(ecd) = handle.get(pools) else { continue };
        for t in &ecd.tags {
            *full_carrier_count.entry(*t).or_default() += 1;
            full_carriers.entry(*t).or_default().push(*guid);
        }
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("  Tag-by-tag breakdown for '{name}'");
    println!("═══════════════════════════════════════════════════");

    let mut seen: std::collections::HashSet<(String, String, String)> =
        std::collections::HashSet::new();
    for o in options {
        if !o.ctx.handler_debug.to_lowercase().contains(&needle)
            && !o.ctx.contract_debug.to_lowercase().contains(&needle)
            && !o.ctx.contract_title.to_lowercase().contains(&needle)
        {
            continue;
        }
        let key = (
            o.ctx.contract_debug.clone(),
            o.group_name.clone(),
            o.ctx.origin.to_string(),
        );
        if !seen.insert(key) {
            continue;
        }

        let pos = tags_set(pools, o.opt.tags.as_ref());
        if pos.is_empty() {
            continue;
        }
        println!(
            "\n  {} · wave='{}' · origin={}",
            if o.ctx.contract_debug.is_empty() {
                "<handler-level>"
            } else {
                o.ctx.contract_debug.as_str()
            },
            o.group_name,
            o.ctx.origin,
        );
        println!("    Positive tags:");
        let mut rows: Vec<(String, usize, Guid)> = pos
            .iter()
            .map(|g| {
                let name = tree
                    .get(g)
                    .map(|n| n.name.clone())
                    .unwrap_or_else(|| format!("<{g}>"));
                let count = full_carrier_count.get(g).copied().unwrap_or(0);
                (name, count, *g)
            })
            .collect();
            rows.sort_by_key(|(_, c, _)| *c);
        for (name, count, guid) in &rows {
            let root = find_tag_root(datacore, *guid);
            let root_str = root.unwrap_or_else(|| "<no root found>".to_string());
            println!("      {name:<32} count={count:>5}  root: {root_str}");
        }

        // Show the first three entities carrying the most-selective tag.
        if let Some((tag_name, count, guid)) = rows.first() {
            if let Some(list) = full_carriers.get(guid) {
                let examples: Vec<String> = list
                    .iter()
                    .take(5)
                    .map(|g| {
                        db.record(g)
                            .and_then(|r| r.name().map(|s| s.to_string()))
                            .unwrap_or_default()
                    })
                    .collect();
                println!(
                    "    Most-selective tag: '{tag_name}' ({count} carriers). First few: {}",
                    examples.join(", ")
                );
            }
        }
    }
    println!();
}

/// Print every spawn option attributed to any context whose
/// handler_debug or contract_debug contains `name` (case-insensitive),
/// grouped by (handler, contract, origin, wave). Shows raw tag sets
/// and resolved candidate ships.
fn dump_contract(
    options: &[Option_<'_>],
    ships: &ShipRegistry,
    pools: &DataPools,
    datacore: &Datacore,
    name: &str,
) {
    let needle = name.to_lowercase();
    let tree = &datacore.snapshot().tag_tree;

    println!("═══════════════════════════════════════════════════");
    println!("  Dump of spawn options for '{name}'");
    println!("═══════════════════════════════════════════════════");

    // Group by (handler, contract_debug, origin, wave_name)
    let mut groups: BTreeMap<
        (String, String, &'static str, String),
        Vec<&Option_<'_>>,
    > = BTreeMap::new();
    for o in options {
        if !o.ctx.handler_debug.to_lowercase().contains(&needle)
            && !o.ctx.contract_debug.to_lowercase().contains(&needle)
            && !o.ctx.contract_title.to_lowercase().contains(&needle)
        {
            continue;
        }
        let key = (
            o.ctx.handler_debug.clone(),
            o.ctx.contract_debug.clone(),
            o.ctx.origin,
            o.group_name.clone(),
        );
        groups.entry(key).or_default().push(o);
    }

    if groups.is_empty() {
        println!("  No spawn options match '{name}' in handler/contract/title.\n");
        return;
    }

    for ((hd, cd, origin, wave), opts) in &groups {
        let cd_show = if cd.is_empty() { "<handler-level>" } else { cd.as_str() };
        println!(
            "\n── handler='{hd}' contract='{cd_show}' origin={origin} wave='{wave}' ({n} option(s))",
            n = opts.len()
        );
        for (i, o) in opts.iter().enumerate() {
            let pos = tags_set(pools, o.opt.tags.as_ref());
            let neg = tags_set(pools, o.opt.negative_tags.as_ref());
            let candidates = ships.resolve_spawn(&pos, &neg);
            let ctx = SpawnContext::classify(tree, &pos);
            let mut names: Vec<&str> = candidates.iter().map(|c| c.display_name.as_str()).collect();
            names.sort();
            names.dedup();
            println!(
                "    #{i} concurrent={conc} weight={w:.2}",
                i = i + 1,
                conc = o.opt.concurrent_amount,
                w = o.opt.weight,
            );
            println!(
                "       → {n} ship(s): {}",
                if names.len() > 12 {
                    format!("{}, … (+{} more)", names[..12].join(", "), names.len() - 12)
                } else if names.is_empty() {
                    "<none>".to_string()
                } else {
                    names.join(", ")
                },
                n = names.len()
            );
            if !ctx.is_empty() {
                print_spawn_context(&ctx, "       ");
            }
            if !neg.is_empty() {
                let neg_names: Vec<String> = neg
                    .iter()
                    .map(|g| {
                        tree.get(g)
                            .map(|n| n.name.clone())
                            .unwrap_or_else(|| format!("<{g}>"))
                    })
                    .collect();
                let mut s = neg_names;
                s.sort();
                println!("       exclude: {{{}}}", s.join(", "));
            }
        }
    }
    println!();
}

fn print_spawn_context(ctx: &SpawnContext, indent: &str) {
    if let Some(level) = ctx.ai_skill {
        print!("{indent}ai_skill={level}");
        if ctx.ace_pilot {
            print!(" (Ace)");
        }
        println!();
    } else if ctx.ace_pilot {
        println!("{indent}ai_skill=Ace");
    }
    if !ctx.factions.is_empty() {
        println!("{indent}factions: {}", ctx.factions.join(", "));
    }
    if !ctx.cargo.is_empty() {
        println!("{indent}cargo:    {}", ctx.cargo.join(", "));
    }
    if !ctx.mission_tags.is_empty() {
        println!("{indent}mission:  {}", ctx.mission_tags.join(", "));
    }
    if !ctx.ai_traits.is_empty() {
        println!("{indent}traits:   {}", ctx.ai_traits.join(", "));
    }
    if !ctx.directives.is_empty() {
        println!("{indent}runtime:  {}", ctx.directives.join(", "));
    }
    if !ctx.other.is_empty() {
        println!("{indent}other:    {}", ctx.other.join(", "));
    }
}

/// Distinguish "ship doesn't exist in the DCB" from "ship exists but isn't
/// referenced by any contract's spawn query". Prints every
/// `EntityClassDefinition` whose display name or record name contains the
/// needle, marked with whether it's in the ship registry.
fn print_ship_existence(needle: &str, ships: &ShipRegistry, datacore: &Datacore) {
    let pools = &datacore.records().pools;
    let display_names = &datacore.snapshot().display_names;
    let db = datacore.db();
    let in_pool: HashSet<Guid> = ships.iter().map(|s| s.entity_guid).collect();

    let mut matches: Vec<(Guid, String, String, bool)> = Vec::new();
    for (guid, handle) in &datacore.records().records.multi_feature.entity_class_definition {
        let Some(_ecd) = handle.get(pools) else { continue };
        let display = display_names.get(guid).unwrap_or("").to_string();
        let record = db
            .record(guid)
            .and_then(|r| r.name().map(|s| s.to_string()))
            .unwrap_or_default();
        if display.to_lowercase().contains(needle) || record.to_lowercase().contains(needle) {
            matches.push((*guid, display, record, in_pool.contains(guid)));
        }
    }
    matches.sort_by(|a, b| b.3.cmp(&a.3).then_with(|| a.1.cmp(&b.1)));

    println!(
        "\n  DCB existence check for \"{needle}\": {} entity(ies), {} in ship registry",
        matches.len(),
        matches.iter().filter(|(_, _, _, in_pool)| *in_pool).count()
    );
    for (_, display, record, in_pool) in matches.iter().take(15) {
        let mark = if *in_pool { "[POOL]" } else { "[     ]" };
        let display = if display.is_empty() { "<no display>" } else { display };
        println!("    {mark} {display:<40} [{record}]");
    }
    if matches.len() > 15 {
        println!("    … +{} more", matches.len() - 15);
    }
}

// ── Handler dispatch ────────────────────────────────────────────────────────

fn walk_handler<'p>(
    pools: &'p DataPools,
    ptr: &ContractGeneratorHandlerBasePtr,
    out: &mut Vec<Option_<'p>>,
) {
    use ContractGeneratorHandlerBasePtr as H;
    match ptr {
        H::ContractGeneratorHandler_Legacy(h) => {
            let Some(handler) = h.get(pools) else { return };
            let kind = "Legacy";
            // Handler-level param overrides apply to all contracts.
            walk_handler_params(pools, &handler.debug_name, kind, handler.contract_params.as_ref(), out);
            for contract_h in &handler.legacy_contracts {
                if let Some(c) = contract_h.get(pools) {
                    walk_legacy(pools, &handler.debug_name, kind, c, out);
                }
            }
        }
        H::ContractGeneratorHandler_Career(h) => {
            let Some(handler) = h.get(pools) else { return };
            let kind = "Career";
            walk_handler_params(pools, &handler.debug_name, kind, handler.contract_params.as_ref(), out);
            for contract_h in &handler.contracts {
                if let Some(c) = contract_h.get(pools) {
                    walk_career(pools, &handler.debug_name, kind, c, out);
                }
            }
            for contract_h in &handler.intro_contracts {
                if let Some(c) = contract_h.get(pools) {
                    walk_contract(pools, &handler.debug_name, kind, c, out);
                }
            }
        }
        H::ContractGeneratorHandler_List(h) => {
            let Some(handler) = h.get(pools) else { return };
            let kind = "List";
            walk_handler_params(pools, &handler.debug_name, kind, handler.contract_params.as_ref(), out);
            for contract_h in &handler.contracts {
                if let Some(c) = contract_h.get(pools) {
                    walk_contract(pools, &handler.debug_name, kind, c, out);
                }
            }
        }
        H::ContractGeneratorHandler_LinearSeries(h) => {
            let Some(handler) = h.get(pools) else { return };
            let kind = "LinearSeries";
            walk_handler_params(pools, &handler.debug_name, kind, handler.contract_params.as_ref(), out);
            for contract_h in &handler.contracts {
                if let Some(c) = contract_h.get(pools) {
                    walk_contract(pools, &handler.debug_name, kind, c, out);
                }
            }
        }
        H::ContractGeneratorHandler_TutorialSeriesDef(h) => {
            let Some(handler) = h.get(pools) else { return };
            let kind = "Tutorial";
            walk_handler_params(pools, &handler.debug_name, kind, handler.contract_params.as_ref(), out);
            for contract_h in &handler.contracts {
                if let Some(c) = contract_h.get(pools) {
                    walk_contract(pools, &handler.debug_name, kind, c, out);
                }
            }
        }
        _ => {}
    }
}

fn walk_handler_params<'p>(
    pools: &'p DataPools,
    handler_debug: &str,
    kind: &'static str,
    params: Option<&Handle<ContractParamOverrides>>,
    out: &mut Vec<Option_<'p>>,
) {
    let ctx = Ctx {
        handler_debug: handler_debug.to_string(),
        handler_kind: kind,
        contract_title: String::new(),
        contract_debug: String::new(),
        origin: "handler",
    };
    walk_param_overrides(pools, params, &ctx, out);
}

fn walk_contract<'p>(
    pools: &'p DataPools,
    handler_debug: &str,
    kind: &'static str,
    contract: &Contract,
    out: &mut Vec<Option_<'p>>,
) {
    let title = title_from_overrides(pools, contract.param_overrides.as_ref());
    let ctx = Ctx {
        handler_debug: handler_debug.to_string(),
        handler_kind: kind,
        contract_title: title,
        contract_debug: contract.debug_name.clone(),
        origin: "contract",
    };
    walk_param_overrides(pools, contract.param_overrides.as_ref(), &ctx, out);
    for sub_h in &contract.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            walk_subcontract(pools, &ctx, sub, out);
        }
    }
}

fn walk_career<'p>(
    pools: &'p DataPools,
    handler_debug: &str,
    kind: &'static str,
    contract: &CareerContract,
    out: &mut Vec<Option_<'p>>,
) {
    let title = title_from_overrides(pools, contract.param_overrides.as_ref());
    let ctx = Ctx {
        handler_debug: handler_debug.to_string(),
        handler_kind: kind,
        contract_title: title,
        contract_debug: contract.debug_name.clone(),
        origin: "contract",
    };
    walk_param_overrides(pools, contract.param_overrides.as_ref(), &ctx, out);
    for sub_h in &contract.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            walk_subcontract(pools, &ctx, sub, out);
        }
    }
}

fn walk_legacy<'p>(
    pools: &'p DataPools,
    handler_debug: &str,
    kind: &'static str,
    contract: &ContractLegacy,
    out: &mut Vec<Option_<'p>>,
) {
    let title = title_from_overrides(pools, contract.param_overrides.as_ref());
    let ctx = Ctx {
        handler_debug: handler_debug.to_string(),
        handler_kind: kind,
        contract_title: title,
        contract_debug: contract.debug_name.clone(),
        origin: "contract",
    };
    walk_param_overrides(pools, contract.param_overrides.as_ref(), &ctx, out);
    for sub_h in &contract.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            walk_subcontract(pools, &ctx, sub, out);
        }
    }
}

fn walk_subcontract<'p>(
    pools: &'p DataPools,
    parent: &Ctx,
    sub: &SubContract,
    out: &mut Vec<Option_<'p>>,
) {
    // SubContract flattens ContractParamOverrides fields inline.
    let title = {
        let mut t = String::new();
        for h in &sub.string_param_overrides {
            if let Some(param) = h.get(pools) {
                if matches!(param.param, ContractStringParamType::Title) && !param.value.is_empty()
                {
                    t = param.value.stripped().to_string();
                    break;
                }
            }
        }
        if t.is_empty() {
            parent.contract_title.clone()
        } else {
            t
        }
    };

    let ctx = Ctx {
        handler_debug: parent.handler_debug.clone(),
        handler_kind: parent.handler_kind,
        contract_title: title,
        contract_debug: parent.contract_debug.clone(),
        origin: "sub_contract",
    };
    // Property-overrides on the sub-contract.
    for prop_h in &sub.property_overrides {
        walk_property(pools, prop_h, &ctx, out);
    }
}

fn walk_param_overrides<'p>(
    pools: &'p DataPools,
    params: Option<&Handle<ContractParamOverrides>>,
    ctx: &Ctx,
    out: &mut Vec<Option_<'p>>,
) {
    let Some(h) = params else { return };
    let Some(po) = h.get(pools) else { return };
    for prop_h in &po.property_overrides {
        walk_property(pools, prop_h, ctx, out);
    }
}

fn walk_property<'p>(
    pools: &'p DataPools,
    prop_h: &Handle<MissionProperty>,
    ctx: &Ctx,
    out: &mut Vec<Option_<'p>>,
) {
    let Some(prop) = prop_h.get(pools) else { return };
    let Some(value_ptr) = prop.value.as_ref() else { return };
    let BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(val_h) = value_ptr
    else {
        return;
    };
    let Some(val) = val_h.get(pools) else { return };
    for group_h in &val.spawn_descriptions {
        let Some(group) = group_h.get(pools) else { continue };
        let group_name = group.name.clone();
        for options_h in &group.ships {
            let Some(options) = options_h.get(pools) else { continue };
            for option_h in &options.options {
                if let Some(opt) = option_h.get(pools) {
                    out.push(Option_ {
                        ctx: ctx.clone(),
                        opt,
                        group_name: group_name.clone(),
                    });
                }
            }
        }
    }
}

fn title_from_overrides(pools: &DataPools, h: Option<&Handle<ContractParamOverrides>>) -> String {
    let Some(h) = h else { return String::new() };
    let Some(po) = h.get(pools) else { return String::new() };
    for p in &po.string_param_overrides {
        if let Some(param) = p.get(pools) {
            if matches!(param.param, ContractStringParamType::Title) && !param.value.is_empty() {
                return param.value.stripped().to_string();
            }
        }
    }
    String::new()
}

// ── Diagnostics ────────────────────────────────────────────────────────────

fn tags_set(pools: &DataPools, h: Option<&Handle<TagList>>) -> HashSet<Guid> {
    let Some(h) = h else { return HashSet::new() };
    let Some(list) = h.get(pools) else { return HashSet::new() };
    list.tags.iter().copied().collect()
}

fn print_target_only(
    options: &[Option_<'_>],
    ships: &ShipRegistry,
    pools: &DataPools,
    datacore: &Datacore,
    limit: usize,
) {
    println!("═══════════════════════════════════════════════════");
    println!("  Spawn options with only role-marker positive tags");
    println!("═══════════════════════════════════════════════════");
    println!(
        "  Criteria: after filtering positive tags through ship_relevant_tags,"
    );
    println!("  nothing remains → the query selects by runtime location context.\n");

    let tree = &datacore.snapshot().tag_tree;
    let ship_rel = ships.ship_relevant_tags();

    // Group by (contract_debug, tag_signature) so we don't print the same
    // contract-and-tag-combo 6× for multiple waves.
    let mut grouped: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    let mut examples: BTreeMap<(String, String, String), (Ctx, Vec<String>, Vec<String>, String)> =
        BTreeMap::new();

    for o in options {
        let pos = tags_set(pools, o.opt.tags.as_ref());
        let neg = tags_set(pools, o.opt.negative_tags.as_ref());
        if pos.is_empty() {
            continue;
        }
        let filtered: HashSet<Guid> = pos.iter().filter(|t| ship_rel.contains(*t)).copied().collect();
        if !filtered.is_empty() {
            continue; // resolvable — skip
        }

        let pos_names: Vec<String> = pos
            .iter()
            .map(|g| tree.get(g).map(|n| n.name.clone()).unwrap_or_else(|| format!("<{g}>")))
            .collect();
        let neg_names: Vec<String> = neg
            .iter()
            .map(|g| tree.get(g).map(|n| n.name.clone()).unwrap_or_else(|| format!("<{g}>")))
            .collect();
        let mut pos_sorted = pos_names.clone();
        pos_sorted.sort();
        let mut neg_sorted = neg_names.clone();
        neg_sorted.sort();
        let tag_sig = format!("+{:?}/-{:?}", pos_sorted, neg_sorted);

        let key = (
            o.ctx.contract_debug.clone(),
            o.ctx.contract_title.clone(),
            tag_sig,
        );
        *grouped.entry(key.clone()).or_default() += 1;
        examples
            .entry(key)
            .or_insert_with(|| (o.ctx.clone(), pos_sorted, neg_sorted, o.group_name.clone()));
    }

    println!("Distinct (contract, tag-signature) pairs: {}", grouped.len());
    println!("Showing first {}:\n", limit.min(grouped.len()));

    // Sort by occurrence count descending, then stably.
    let mut items: Vec<_> = grouped.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    for (i, (key, count)) in items.iter().take(limit).enumerate() {
        let (ctx, pos, neg, group) = &examples[*key];
        println!(
            "{:>3}. [{kind}] handler='{hd}' contract='{cd}' title='{title}' origin={origin}",
            i + 1,
            kind = ctx.handler_kind,
            hd = ctx.handler_debug,
            cd = ctx.contract_debug,
            title = ctx.contract_title,
            origin = ctx.origin,
        );
        println!(
            "     wave='{group}'  positive={{{}}}  negative={{{}}}  × {count}",
            pos.join(", "),
            neg.join(", "),
        );
    }
    println!();
}

fn print_contains(
    options: &[Option_<'_>],
    ships: &ShipRegistry,
    pools: &DataPools,
    needle: &str,
    limit: usize,
) {
    println!("═══════════════════════════════════════════════════");
    println!("  Spawn pools containing ship name: \"{needle}\"");
    println!("═══════════════════════════════════════════════════");

    // Group by contract to produce one row per contract with the matched
    // ships aggregated.
    let mut per_contract: BTreeMap<String, (Ctx, HashSet<String>, usize)> = BTreeMap::new();

    for o in options {
        let pos = tags_set(pools, o.opt.tags.as_ref());
        let neg = tags_set(pools, o.opt.negative_tags.as_ref());
        let cands: Vec<ShipCandidate> = ships.resolve_spawn(&pos, &neg);
        let matched: Vec<&ShipCandidate> = cands
            .iter()
            .filter(|c| c.display_name.to_lowercase().contains(needle))
            .collect();
        if matched.is_empty() {
            continue;
        }
        let key = format!(
            "{}/{}",
            o.ctx.handler_debug,
            if o.ctx.contract_debug.is_empty() {
                "handler-level"
            } else {
                &o.ctx.contract_debug
            }
        );
        let entry = per_contract.entry(key).or_insert_with(|| (o.ctx.clone(), HashSet::new(), 0));
        for m in matched {
            entry.1.insert(m.display_name.clone());
        }
        entry.2 += 1;
    }

    println!("Distinct contracts with a match: {}", per_contract.len());
    println!("Showing first {}:\n", limit.min(per_contract.len()));

    let mut items: Vec<_> = per_contract.iter().collect();
    items.sort_by(|a, b| b.1 .2.cmp(&a.1 .2).then_with(|| a.0.cmp(b.0)));

    for (i, (_key, (ctx, ships, query_count))) in items.iter().take(limit).enumerate() {
        let mut ship_list: Vec<&str> = ships.iter().map(String::as_str).collect();
        ship_list.sort();
        println!(
            "{:>3}. [{kind}] handler='{hd}' contract='{cd}' title='{title}' origin={origin}",
            i + 1,
            kind = ctx.handler_kind,
            hd = ctx.handler_debug,
            cd = ctx.contract_debug,
            title = ctx.contract_title,
            origin = ctx.origin,
        );
        println!(
            "     matched ships ({query_count} query slot(s)): {}",
            ship_list.join(", ")
        );
    }
    println!();
}
