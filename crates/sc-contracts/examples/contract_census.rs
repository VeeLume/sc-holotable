//! Contract census — calibration diagnostic for sc-contracts.
//!
//! Walks every `ContractGenerator` root record in the live DCB and prints
//! the structural statistics that drive the crate's design decisions:
//!
//! - handler-kind histogram (6 kinds expected + service-beacon + unknowns)
//! - contract-subclass counts (Contract / ContractLegacy / CareerContract)
//! - sub-contract counts
//! - prerequisite-kind histogram (polymorphic dispatch)
//! - reward-kind histogram (polymorphic dispatch)
//! - `ContractResult_Item.entity_class` top-N — for reward-currency catalog
//! - `MissionProperty.mission_variable_name` top-N — for encounter classifier
//! - title-key collision histogram — for merge-rule calibration
//!
//! Run:
//!
//! ```bash
//! cargo run -p sc-contracts --release --example contract_census
//! cargo run -p sc-contracts --release --example contract_census -- --top 40
//! ```
//!
//! See `docs/sc-contracts.md` §"Census and calibration".
#![allow(non_snake_case)]

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use sc_contracts::{BlueprintPoolRegistry, RewardCurrencyCatalog, ShipRegistry};
use sc_extract::generated::{
    CareerContract, Contract, ContractGeneratorHandlerBasePtr, ContractLegacy,
    ContractParamOverrides, ContractPrerequisiteBasePtr, ContractResultBasePtr,
    ContractStringParamType, DataPools, Handle, SubContract, TagList,
};
use sc_extract::svarog_datacore::DataCoreDatabase;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid, LocaleKey};

// ── Accumulator ─────────────────────────────────────────────────────────────

#[derive(Default)]
struct Census {
    // Generators / handlers / expansions
    total_generators: usize,
    total_handlers: usize,
    total_expansions: usize, // concrete (handler, contract, sub_contract?) triples

    // Handler kind histogram
    handler_kinds: BTreeMap<&'static str, usize>,
    handler_unknown: BTreeMap<String, usize>, // db struct_name → count

    // Contract subclass counts (per concrete pool visited through handlers)
    contract_kinds: BTreeMap<&'static str, usize>,
    subcontracts: usize,

    // Intro contracts (Career handler only)
    intro_contracts: usize,

    // Prereq / reward kind histograms
    prereq_kinds: BTreeMap<&'static str, usize>,
    prereq_unknown: BTreeMap<String, usize>,
    reward_kinds: BTreeMap<&'static str, usize>,
    reward_unknown: BTreeMap<String, usize>,

    // ContractResult_Item.entity_class top-N: guid → (occurrences, sum_amount)
    item_entity_class: HashMap<Guid, (usize, i64)>,

    // MissionProperty.mission_variable_name top-N
    mission_variable_names: HashMap<String, usize>,

    // Title key collisions (title → occurrences across expansions)
    title_counts: HashMap<String, usize>,
    missing_titles: usize,
    missing_descriptions: usize,

    // Availability inheritance stats — how many handlers declare their own
    // defaultAvailability (vs. relying on defaults all the way through).
    availability_on_handler: usize,

    // Ship-registry coverage — for every SpawnDescription_Ship option in the
    // graph, does `ShipRegistry::resolve_spawn` return ≥1 candidate?
    spawn_options_total: usize,
    spawn_options_empty_positive: usize,
    spawn_options_resolved: usize,
    spawn_options_unresolved: usize,
    // Size histogram of the ship registry.
    ship_sizes: BTreeMap<i32, usize>,
}

impl Census {
    fn bump_handler(&mut self, kind: &'static str) {
        *self.handler_kinds.entry(kind).or_default() += 1;
        self.total_handlers += 1;
    }

    fn bump_handler_unknown(&mut self, db: &DataCoreDatabase, struct_index: u32) {
        let name = db
            .struct_name(struct_index as usize)
            .unwrap_or("?")
            .to_string();
        *self.handler_unknown.entry(name).or_default() += 1;
        self.total_handlers += 1;
    }

    fn bump_contract(&mut self, kind: &'static str) {
        *self.contract_kinds.entry(kind).or_default() += 1;
    }

    fn bump_prereq(&mut self, kind: &'static str) {
        *self.prereq_kinds.entry(kind).or_default() += 1;
    }

    fn bump_prereq_unknown(&mut self, db: &DataCoreDatabase, struct_index: u32) {
        let name = db
            .struct_name(struct_index as usize)
            .unwrap_or("?")
            .to_string();
        *self.prereq_unknown.entry(name).or_default() += 1;
    }

    fn bump_reward(&mut self, kind: &'static str) {
        *self.reward_kinds.entry(kind).or_default() += 1;
    }

    fn bump_reward_unknown(&mut self, db: &DataCoreDatabase, struct_index: u32) {
        let name = db
            .struct_name(struct_index as usize)
            .unwrap_or("?")
            .to_string();
        *self.reward_unknown.entry(name).or_default() += 1;
    }
}

// ── Entry ───────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    // Command-line: --top N for top-N list sizes (default 20).
    let args: Vec<String> = std::env::args().skip(1).collect();
    let top_n: usize = args
        .iter()
        .position(|a| a == "--top")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    let t0 = Instant::now();
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
    let parse_secs = t0.elapsed().as_secs_f64();
    println!("Parsed datacore in {parse_secs:.1}s\n");

    let db = datacore.db();
    let pools = &datacore.records().pools;

    let mut c = Census::default();

    // Walk every ContractGenerator root record. We iterate the pool directly:
    // root records land in the same pool as nested ones, so non-None slots
    // cover both. (For these types all entries are generators, since nothing
    // else points at ContractGenerator.)
    for gen_slot in pools.multi_feature.contract_generator.iter().flatten() {
        c.total_generators += 1;
        for handler_ptr in &gen_slot.generators {
            walk_handler(&mut c, db, pools, handler_ptr);
        }
    }

    // Mission-property sweep — visit every materialized MissionProperty.
    // Includes those reachable from propertyOverrides in contracts, sub-
    // contracts, handler contractParams, and templates.
    for prop in pools.multi_feature.mission_property.iter().flatten() {
        if !prop.mission_variable_name.is_empty() {
            *c.mission_variable_names
                .entry(prop.mission_variable_name.clone())
                .or_default() += 1;
        }
    }

    // Ship-registry build + coverage sweep.
    let ship_start = Instant::now();
    let ships = ShipRegistry::build(&datacore);
    let ship_build_secs = ship_start.elapsed().as_secs_f64();
    println!(
        "Built ShipRegistry in {ship_build_secs:.2}s — {} entities, {} spawn-referenced tags",
        ships.len(),
        ships.spawn_referenced_tags().len()
    );
    for ship in ships.iter() {
        *c.ship_sizes.entry(ship.size).or_default() += 1;
    }
    run_spawn_coverage(&mut c, pools, &ships);

    // Blueprint pool registry — depends on LocaleMap from AssetData.
    let bp_start = Instant::now();
    let blueprints = BlueprintPoolRegistry::build(&datacore, &asset_data.locale);
    println!(
        "Built BlueprintPoolRegistry in {:.2}s — {} pools, {} unresolved records, {} missing locale keys",
        bp_start.elapsed().as_secs_f64(),
        blueprints.len(),
        blueprints.unresolved_blueprint_records(),
        blueprints.missing_locale_names(),
    );

    // Reward currency catalog.
    let cur_start = Instant::now();
    let currencies = RewardCurrencyCatalog::build(&datacore);
    println!(
        "Built RewardCurrencyCatalog in {:.2}s — {} currency entities",
        cur_start.elapsed().as_secs_f64(),
        currencies.len(),
    );

    println!();
    print_report(&c, db, top_n, &ships);
    print_blueprint_registry(&blueprints, top_n);
    print_currency_catalog(&c, &currencies);
    diagnose_unresolved_spawns(pools, &ships, &datacore, 5);
    Ok(())
}

fn print_currency_catalog(c: &Census, catalog: &RewardCurrencyCatalog) {
    println!("─── Reward currency catalog ───────────────────────");
    println!("  currency entities: {}", catalog.len());
    println!();
    // For each currency, show record name + display name + how often
    // it appears in the contract-reward census (so we see scrip rewards
    // dominating and ship one-offs not being flagged as currency).
    let mut rows: Vec<(&CurrencyInfo, usize)> = catalog
        .iter()
        .map(|info| {
            let count = c
                .item_entity_class
                .get(&info.entity_guid)
                .map(|(n, _)| *n)
                .unwrap_or(0);
            (info, count)
        })
        .collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));
    println!("  {:>6}  {:<30}  {}", "count", "display", "record");
    for (info, count) in &rows {
        let display = if info.display_name.is_empty() {
            "<none>"
        } else {
            info.display_name.as_str()
        };
        println!("  {count:>6}  {display:<30}  {}", info.record_name);
    }
    // Cross-check: how many distinct ContractResult_Item entity_classes
    // are NOT in the catalog? Those are real items (ship unlocks,
    // collector weapons, etc.).
    let total_distinct = c.item_entity_class.len();
    let currency_count = rows.iter().filter(|(_, n)| *n > 0).count();
    println!();
    println!("  ContractResult_Item distinct entity_classes: {total_distinct}");
    println!("    of which currency:  {currency_count}");
    println!(
        "    of which item:      {}",
        total_distinct - currency_count
    );
    println!();
}

use sc_contracts::CurrencyInfo;

fn print_blueprint_registry(registry: &BlueprintPoolRegistry, top_n: usize) {
    println!("─── Blueprint pools ────────────────────────────────");
    println!("  pools:                      {}", registry.len());
    println!(
        "  unresolved blueprint refs:  {}",
        registry.unresolved_blueprint_records()
    );
    println!(
        "  missing locale keys:        {}",
        registry.missing_locale_names()
    );

    // Size distribution.
    let mut size_hist: BTreeMap<usize, usize> = BTreeMap::new();
    for pool in registry.iter() {
        *size_hist.entry(pool.items.len()).or_default() += 1;
    }
    println!();
    println!("  pool-size histogram (items-per-pool → count):");
    for (size, count) in &size_hist {
        println!("    {size:>3} items  ×  {count:>3} pool(s)");
    }

    // Top-N pools by item count, with the first few items shown.
    println!();
    println!("  Top-{top_n} pools by item count:");
    let mut pools: Vec<_> = registry.iter().collect();
    pools.sort_by(|a, b| b.items.len().cmp(&a.items.len()).then(a.name.cmp(&b.name)));
    for (i, pool) in pools.iter().take(top_n).enumerate() {
        let resolved: usize = pool
            .items
            .iter()
            .filter(|i| !i.display_name.is_empty())
            .count();
        println!(
            "   {i:>3}. [{items} items, {resolved} resolved]  {name}",
            i = i + 1,
            items = pool.items.len(),
            name = pool.name,
        );
        let shown: Vec<&str> = pool
            .items
            .iter()
            .filter_map(|it| {
                if it.display_name.is_empty() {
                    None
                } else {
                    Some(it.display_name.as_str())
                }
            })
            .take(5)
            .collect();
        if !shown.is_empty() {
            println!("           {}", shown.join(", "));
        }
    }
    println!();
}

/// Walk every `SpawnDescription_Ship` option in the pools and check that
/// `ShipRegistry::resolve_spawn` produces ≥1 candidate. Populates the
/// coverage counters on `Census`.
fn run_spawn_coverage(c: &mut Census, pools: &DataPools, ships: &ShipRegistry) {
    for option in pools.multi_feature.spawn_description_ship.iter().flatten() {
        c.spawn_options_total += 1;

        let positive = tags_as_set(pools, option.tags.as_ref());
        let negative = tags_as_set(pools, option.negative_tags.as_ref());

        if positive.is_empty() {
            c.spawn_options_empty_positive += 1;
            continue;
        }

        let candidates = ships.resolve_spawn(&positive, &negative);
        if candidates.is_empty() {
            c.spawn_options_unresolved += 1;
        } else {
            c.spawn_options_resolved += 1;
        }
    }
}

/// Diagnostic — for the first few unresolved spawn queries, print the
/// positive / negative tag names and whether *any* ship entity in the
/// registry matches a *subset* of the positive tags (so we can see
/// which tag specifically breaks the match).
fn diagnose_unresolved_spawns(
    pools: &DataPools,
    ships: &ShipRegistry,
    datacore: &Datacore,
    limit: usize,
) {
    use std::collections::HashSet;
    let tags = &datacore.snapshot().tag_tree;
    let mut shown = 0usize;

    println!("─── Diagnostic: first {limit} unresolved spawn queries ─────────");
    for option in pools.multi_feature.spawn_description_ship.iter().flatten() {
        if shown >= limit {
            break;
        }
        let positive = tags_as_set(pools, option.tags.as_ref());
        let negative = tags_as_set(pools, option.negative_tags.as_ref());
        if positive.is_empty() {
            continue;
        }
        if !ships.resolve_spawn(&positive, &negative).is_empty() {
            continue;
        }

        shown += 1;
        let pos_names: Vec<String> = positive
            .iter()
            .map(|g| {
                tags.get(g)
                    .map(|n| n.name.clone())
                    .unwrap_or_else(|| format!("<{g}>"))
            })
            .collect();
        let neg_names: Vec<String> = negative
            .iter()
            .map(|g| {
                tags.get(g)
                    .map(|n| n.name.clone())
                    .unwrap_or_else(|| format!("<{g}>"))
            })
            .collect();
        println!(
            "\n  #{shown} positive={{{}}} negative={{{}}}",
            pos_names.join(", "),
            neg_names.join(", ")
        );

        // For each positive tag, how many ships carry it?
        for (name, guid) in pos_names.iter().zip(positive.iter()) {
            let count = ships.iter().filter(|s| s.tags.contains(guid)).count();
            println!("    {name:<40} carried by {count} ship entities");
        }

        // "Best match" — which ship shares the most positive tags?
        let (best_ship, best_match) = ships
            .iter()
            .map(|s| {
                let matched = positive.iter().filter(|t| s.tags.contains(*t)).count();
                (s, matched)
            })
            .max_by_key(|(_, m)| *m)
            .map(|(s, m)| (Some(s), m))
            .unwrap_or((None, 0));
        if let Some(ship) = best_ship {
            let missing: Vec<&str> = pos_names
                .iter()
                .zip(positive.iter())
                .filter(|(_, g)| !ship.tags.contains(g))
                .map(|(n, _)| n.as_str())
                .collect();
            println!(
                "    Best match: {} ({}/{} tags). Missing: {{{}}}",
                if ship.display_name.is_empty() {
                    format!("<{}>", ship.entity_guid)
                } else {
                    ship.display_name.clone()
                },
                best_match,
                positive.len(),
                missing.join(", ")
            );
        }

        // Search the FULL EntityClassDefinition pool (not just ships) —
        // are there entities that satisfy all positive tags but got
        // excluded from the ship registry?
        let mut full_matches = 0usize;
        let mut first_full: Option<String> = None;
        for (guid, handle) in &datacore
            .records()
            .records
            .multi_feature
            .entity_class_definition
        {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let entity_tag_set: HashSet<Guid> = ecd.tags.iter().copied().collect();
            if positive.iter().all(|t| entity_tag_set.contains(t))
                && negative.iter().all(|t| !entity_tag_set.contains(t))
            {
                full_matches += 1;
                if first_full.is_none() {
                    let name = datacore
                        .snapshot()
                        .display_names
                        .get(guid)
                        .unwrap_or("<no display>")
                        .to_string();
                    let rec_name = datacore
                        .db()
                        .record(guid)
                        .and_then(|r| r.name().map(|s| s.to_string()))
                        .unwrap_or_else(|| "?".to_string());
                    first_full = Some(format!("{name} [{rec_name}]"));
                }
            }
        }
        if full_matches > 0 {
            println!(
                "    FULL-DCB match: {full_matches} EntityClassDefinitions satisfy this query. First: {}",
                first_full.unwrap()
            );
        } else {
            println!(
                "    FULL-DCB match: 0 — no EntityClassDefinition satisfies this query (impossible tag combo or tags live elsewhere)."
            );
        }
    }
    println!();
}

fn tags_as_set(pools: &DataPools, h: Option<&Handle<TagList>>) -> std::collections::HashSet<Guid> {
    let Some(h) = h else {
        return std::collections::HashSet::new();
    };
    let Some(list) = h.get(pools) else {
        return std::collections::HashSet::new();
    };
    list.tags.iter().copied().collect()
}

// ── Handler dispatch ────────────────────────────────────────────────────────

fn walk_handler(
    c: &mut Census,
    db: &DataCoreDatabase,
    pools: &DataPools,
    ptr: &ContractGeneratorHandlerBasePtr,
) {
    use ContractGeneratorHandlerBasePtr as H;

    match ptr {
        H::ContractGeneratorHandler_Legacy(h) => {
            c.bump_handler("Legacy");
            let Some(handler) = h.get(pools) else { return };
            if handler.default_availability.is_some() {
                c.availability_on_handler += 1;
            }
            for contract_h in &handler.legacy_contracts {
                if let Some(contract) = contract_h.get(pools) {
                    c.bump_contract("ContractLegacy");
                    walk_contract_legacy(c, db, pools, contract);
                }
            }
        }
        H::ContractGeneratorHandler_Career(h) => {
            c.bump_handler("Career");
            let Some(handler) = h.get(pools) else { return };
            if handler.default_availability.is_some() {
                c.availability_on_handler += 1;
            }
            for contract_h in &handler.contracts {
                if let Some(contract) = contract_h.get(pools) {
                    c.bump_contract("CareerContract");
                    walk_career_contract(c, db, pools, contract);
                }
            }
            for contract_h in &handler.intro_contracts {
                if let Some(contract) = contract_h.get(pools) {
                    c.bump_contract("Contract (intro)");
                    c.intro_contracts += 1;
                    walk_contract(c, db, pools, contract);
                }
            }
        }
        H::ContractGeneratorHandler_List(h) => {
            c.bump_handler("List");
            let Some(handler) = h.get(pools) else { return };
            if handler.default_availability.is_some() {
                c.availability_on_handler += 1;
            }
            for contract_h in &handler.contracts {
                if let Some(contract) = contract_h.get(pools) {
                    c.bump_contract("Contract");
                    walk_contract(c, db, pools, contract);
                }
            }
        }
        H::ContractGeneratorHandler_LinearSeries(h) => {
            c.bump_handler("LinearSeries");
            let Some(handler) = h.get(pools) else { return };
            if handler.default_availability.is_some() {
                c.availability_on_handler += 1;
            }
            for contract_h in &handler.contracts {
                if let Some(contract) = contract_h.get(pools) {
                    c.bump_contract("Contract");
                    walk_contract(c, db, pools, contract);
                }
            }
        }
        H::ContractGeneratorHandler_TutorialSeriesDef(h) => {
            c.bump_handler("Tutorial");
            let Some(handler) = h.get(pools) else { return };
            if handler.default_availability.is_some() {
                c.availability_on_handler += 1;
            }
            for contract_h in &handler.contracts {
                if let Some(contract) = contract_h.get(pools) {
                    c.bump_contract("Contract");
                    walk_contract(c, db, pools, contract);
                }
            }
        }
        H::ContractGeneratorHandler_PVPBountyDef(h) => {
            c.bump_handler("PVPBounty");
            if h.get(pools).is_some() {
                // PVP bounty handler has a more complex internal structure;
                // for census purposes we count it but don't expand contracts.
                // Full walk lives in the crate-proper expansion logic.
            }
        }
        H::ContractGeneratorHandler_ServiceBeacon(h) => {
            c.bump_handler("ServiceBeacon");
            let _ = h.get(pools);
        }
        H::ContractGeneratorHandlerBase(_) => {
            // Bare base with no subclass shouldn't occur in real data but is
            // a valid runtime variant.
            c.bump_handler("_Base");
        }
        H::Unknown {
            struct_index,
            instance_index: _,
        } => {
            c.bump_handler_unknown(db, *struct_index);
        }
    }
}

// ── Contract walkers ────────────────────────────────────────────────────────

fn walk_contract(c: &mut Census, db: &DataCoreDatabase, pools: &DataPools, contract: &Contract) {
    c.total_expansions += 1;
    walk_param_overrides(
        c,
        pools,
        contract.param_overrides.as_ref(),
        /*level=*/ 1,
    );
    walk_prereqs(c, db, &contract.additional_prerequisites);
    walk_rewards(c, db, pools, contract.contract_results.as_ref());

    for sub_h in &contract.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            walk_subcontract(c, db, pools, sub);
        }
    }
}

fn walk_contract_legacy(
    c: &mut Census,
    db: &DataCoreDatabase,
    pools: &DataPools,
    contract: &ContractLegacy,
) {
    c.total_expansions += 1;
    walk_param_overrides(c, pools, contract.param_overrides.as_ref(), 1);
    walk_prereqs(c, db, &contract.additional_prerequisites);
    walk_rewards(c, db, pools, contract.contract_results.as_ref());

    for sub_h in &contract.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            walk_subcontract(c, db, pools, sub);
        }
    }
}

fn walk_career_contract(
    c: &mut Census,
    db: &DataCoreDatabase,
    pools: &DataPools,
    contract: &CareerContract,
) {
    c.total_expansions += 1;
    walk_param_overrides(c, pools, contract.param_overrides.as_ref(), 1);
    walk_prereqs(c, db, &contract.additional_prerequisites);
    walk_rewards(c, db, pools, contract.contract_results.as_ref());

    for sub_h in &contract.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            walk_subcontract(c, db, pools, sub);
        }
    }
}

fn walk_subcontract(c: &mut Census, db: &DataCoreDatabase, pools: &DataPools, sub: &SubContract) {
    c.subcontracts += 1;
    c.total_expansions += 1;
    // SubContract flattens ContractParamOverrides fields inline — walk its
    // string param overrides directly for title tracking.
    walk_subcontract_params(c, pools, sub);
    walk_prereqs(c, db, &sub.additional_prerequisites);
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn walk_param_overrides(
    c: &mut Census,
    pools: &DataPools,
    po: Option<&Handle<ContractParamOverrides>>,
    level: u8,
) {
    let Some(h) = po else {
        if level == 1 {
            c.missing_titles += 1;
        }
        return;
    };
    let Some(po) = h.get(pools) else { return };

    let (title, desc) = extract_title_desc(pools, &po.string_param_overrides);
    if let Some(title) = title {
        *c.title_counts.entry(title).or_default() += 1;
    } else if level == 1 {
        c.missing_titles += 1;
    }
    if desc.is_none() && level == 1 {
        c.missing_descriptions += 1;
    }
}

fn walk_subcontract_params(c: &mut Census, pools: &DataPools, sub: &SubContract) {
    let (title, _desc) = extract_title_desc(pools, &sub.string_param_overrides);
    if let Some(title) = title {
        *c.title_counts.entry(title).or_default() += 1;
    }
}

/// Extract the title and description locale keys (as their raw string form)
/// from a list of ContractStringParam handles.
fn extract_title_desc(
    pools: &DataPools,
    handles: &[Handle<sc_extract::generated::ContractStringParam>],
) -> (Option<String>, Option<String>) {
    let mut title = None;
    let mut desc = None;
    for h in handles {
        let Some(param) = h.get(pools) else { continue };
        let key = locale_key_as_str(&param.value);
        match &param.param {
            ContractStringParamType::Title => {
                if !key.is_empty() && title.is_none() {
                    title = Some(key.to_string());
                }
            }
            ContractStringParamType::Description => {
                if !key.is_empty() && desc.is_none() {
                    desc = Some(key.to_string());
                }
            }
            _ => {}
        }
    }
    (title, desc)
}

/// Use the stripped form — titles in global.ini are keyed without the
/// leading `@` prefix, so collision detection should match that shape.
fn locale_key_as_str(k: &LocaleKey) -> &str {
    k.stripped()
}

fn walk_prereqs(c: &mut Census, db: &DataCoreDatabase, prereqs: &[ContractPrerequisiteBasePtr]) {
    use ContractPrerequisiteBasePtr as P;
    for p in prereqs {
        match p {
            P::ContractPrerequisiteBase(_) => c.bump_prereq("_Base"),
            P::ContractPrerequisite_Locality(_) => c.bump_prereq("Locality"),
            P::ContractPrerequisite_Location(_) => c.bump_prereq("Location"),
            P::ContractPrerequisite_LocationProperty(_) => c.bump_prereq("LocationProperty"),
            P::ContractPrerequisite_CrimeStat(_) => c.bump_prereq("CrimeStat"),
            P::ContractPrerequisite_Reputation(_) => c.bump_prereq("Reputation"),
            P::ContractPrerequisite_CompletedContractTags(_) => {
                c.bump_prereq("CompletedContractTags")
            }
            P::Unknown {
                struct_index,
                instance_index: _,
            } => c.bump_prereq_unknown(db, *struct_index),
        }
    }
}

fn walk_rewards(
    c: &mut Census,
    db: &DataCoreDatabase,
    pools: &DataPools,
    results: Option<&Handle<sc_extract::generated::ContractResults>>,
) {
    let Some(h) = results else { return };
    let Some(results) = h.get(pools) else { return };

    use ContractResultBasePtr as R;
    for r in &results.contract_results {
        match r {
            R::ContractResultBase(_) => c.bump_reward("_Base"),
            R::ContractResult_Reward(_) => c.bump_reward("Reward"),
            R::ContractResult_CalculatedReward(_) => c.bump_reward("CalculatedReward"),
            R::ContractResult_LegacyReputation(_) => c.bump_reward("LegacyReputation"),
            R::ContractResult_CalculatedReputation(_) => c.bump_reward("CalculatedReputation"),
            R::ContractResult_JournalEntry(_) => c.bump_reward("JournalEntry"),
            R::ContractResult_RefundBuyIn(_) => c.bump_reward("RefundBuyIn"),
            R::ContractResult_CompletionTags(_) => c.bump_reward("CompletionTags"),
            R::ContractResult_BadgeAward(_) => c.bump_reward("BadgeAward"),
            R::ContractResult_Item(ih) => {
                c.bump_reward("Item");
                if let Some(item) = ih.get(pools) {
                    if let Some(ec) = item.entity_class {
                        let entry = c.item_entity_class.entry(ec).or_insert((0, 0));
                        entry.0 += 1;
                        entry.1 += item.amount as i64;
                    }
                }
            }
            R::ContractResult_CompletionBounty(_) => c.bump_reward("CompletionBounty"),
            R::ContractResult_ItemsWeighting(_) => c.bump_reward("ItemsWeighting"),
            R::ContractResult_ScenarioProgress(_) => c.bump_reward("ScenarioProgress"),
            R::BlueprintRewards(_) => c.bump_reward("BlueprintRewards"),
            R::Unknown {
                struct_index,
                instance_index: _,
            } => c.bump_reward_unknown(db, *struct_index),
        }
    }
}

// ── Report ──────────────────────────────────────────────────────────────────

fn print_report(c: &Census, db: &DataCoreDatabase, top_n: usize, ships: &ShipRegistry) {
    println!("═══════════════════════════════════════════════════");
    println!("                 CONTRACT CENSUS");
    println!("═══════════════════════════════════════════════════\n");

    println!("Generators:          {}", c.total_generators);
    println!("Handlers:            {}", c.total_handlers);
    println!(
        "Expansions:          {}  (handler × contract × sub?)",
        c.total_expansions
    );
    println!("Sub-contracts:       {}", c.subcontracts);
    println!("Intro contracts:     {}", c.intro_contracts);
    println!();

    print_histogram("Handler kinds", &c.handler_kinds);
    if !c.handler_unknown.is_empty() {
        println!("  Unknown handler kinds:");
        let mut items: Vec<_> = c.handler_unknown.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1));
        for (n, k) in items {
            println!("    {n:<40} {k}");
        }
        println!();
    }

    print_histogram("Contract subclasses", &c.contract_kinds);
    print_histogram("Prereq kinds", &c.prereq_kinds);
    if !c.prereq_unknown.is_empty() {
        println!("  Unknown prereq kinds:");
        let mut items: Vec<_> = c.prereq_unknown.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1));
        for (n, k) in items {
            println!("    {n:<40} {k}");
        }
        println!();
    }

    print_histogram("Reward kinds", &c.reward_kinds);
    if !c.reward_unknown.is_empty() {
        println!("  Unknown reward kinds:");
        let mut items: Vec<_> = c.reward_unknown.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1));
        for (n, k) in items {
            println!("    {n:<40} {k}");
        }
        println!();
    }

    print_entity_class_top_n(c, db, top_n);
    print_mission_variable_top_n(c, top_n);
    print_title_collisions(c, top_n);
    print_ship_registry(c, ships);

    println!("─── Coverage signals (non-zero = lint) ────────────");
    println!("Missing titles:          {}", c.missing_titles);
    println!("Missing descriptions:    {}", c.missing_descriptions);
    println!(
        "Unknown handler kinds:   {}",
        c.handler_unknown.values().sum::<usize>()
    );
    println!(
        "Unknown prereq kinds:    {}",
        c.prereq_unknown.values().sum::<usize>()
    );
    println!(
        "Unknown reward kinds:    {}",
        c.reward_unknown.values().sum::<usize>()
    );
    println!(
        "Unresolved spawn queries:{} of {} ({:.1}%)",
        c.spawn_options_unresolved,
        c.spawn_options_total,
        if c.spawn_options_total > 0 {
            100.0 * c.spawn_options_unresolved as f64 / c.spawn_options_total as f64
        } else {
            0.0
        }
    );
}

fn print_ship_registry(c: &Census, ships: &ShipRegistry) {
    println!("─── Ship registry ─────────────────────────────");
    println!("  entities:                {}", ships.len());
    println!(
        "  spawn-referenced tags:   {}",
        ships.spawn_referenced_tags().len()
    );
    println!(
        "  ship-relevant tags:      {}",
        ships.ship_relevant_tags().len()
    );
    println!();
    println!("  size  count");
    for (size, count) in &c.ship_sizes {
        println!("  {size:>4}  {count:>5}");
    }
    println!();
    println!("  Spawn-query coverage:");
    println!("    total options:         {}", c.spawn_options_total);
    println!(
        "    empty positive set:    {}  (no constraint — skipped)",
        c.spawn_options_empty_positive
    );
    println!("    resolved (≥1 cand):    {}", c.spawn_options_resolved);
    println!(
        "    unresolved (0 cand):   {}  ← coverage invariant",
        c.spawn_options_unresolved
    );
    println!();
}

fn print_histogram(label: &str, m: &BTreeMap<&'static str, usize>) {
    println!("─── {label} ──────────────────────────────");
    let total: usize = m.values().sum();
    let mut items: Vec<_> = m.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));
    for (k, v) in items {
        let pct = if total > 0 {
            100.0 * (*v as f64) / (total as f64)
        } else {
            0.0
        };
        println!("  {k:<30} {v:>6}  ({pct:>5.1}%)");
    }
    println!("  {:30} {total:>6}", "total");
    println!();
}

fn print_entity_class_top_n(c: &Census, db: &DataCoreDatabase, top_n: usize) {
    println!("─── ContractResult_Item.entity_class top-{top_n} (for currency catalog) ──");
    let mut items: Vec<(&Guid, usize, i64)> = c
        .item_entity_class
        .iter()
        .map(|(g, (n, a))| (g, *n, *a))
        .collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "  {:<38} {:>6} {:>14} {}",
        "entity_class guid", "count", "sum amount", "record name"
    );
    for (g, n, sum) in items.iter().take(top_n) {
        let name = db
            .record(g)
            .and_then(|r| r.name().map(|s| s.to_string()))
            .unwrap_or_else(|| "?".to_string());
        println!("  {:<38} {:>6} {:>14} {name}", g.to_string(), n, sum);
    }
    println!(
        "  ({} distinct entity_classes total)\n",
        c.item_entity_class.len()
    );
}

fn print_mission_variable_top_n(c: &Census, top_n: usize) {
    println!("─── MissionProperty.mission_variable_name top-{top_n} (for encounter classifier) ──");
    let mut items: Vec<_> = c.mission_variable_names.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));
    println!("  {:<48} {:>6}", "mission_variable_name", "count");
    for (k, v) in items.iter().take(top_n) {
        println!("  {k:<48} {v:>6}");
    }
    println!(
        "  ({} distinct variable names total)\n",
        c.mission_variable_names.len()
    );
}

fn print_title_collisions(c: &Census, top_n: usize) {
    println!("─── Title collisions (expansions sharing a title_key) ──");
    // Distribution: N contracts with K expansions each.
    let mut buckets: BTreeMap<usize, usize> = BTreeMap::new();
    for count in c.title_counts.values() {
        *buckets.entry(*count).or_default() += 1;
    }
    println!("  expansions_per_title   titles_in_bucket");
    for (k, v) in &buckets {
        println!("  {k:>20}   {v:>18}");
    }
    println!();
    // Top colliders.
    println!("  Top-{top_n} most-repeated titles:");
    let mut items: Vec<_> = c.title_counts.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));
    println!("  {:<52} {:>6}", "title_key", "count");
    for (k, v) in items.iter().take(top_n) {
        println!("  {k:<52} {v:>6}");
    }
    let distinct_titles = c.title_counts.len();
    let total_title_bearing = c.title_counts.values().sum::<usize>();
    println!(
        "  ({distinct_titles} distinct titles across {total_title_bearing} title-bearing expansions)\n"
    );
}
