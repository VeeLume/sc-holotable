//! Run `merge_expansions` against the live DCB and print the summary
//! + a few illustrative groups. Validates the ~1,497 SCMDB target.
//!
//! Run:
//! ```bash
//! cargo run -p sc-contracts --release --example merge_sample
//! cargo run -p sc-contracts --release --example merge_sample -- --filter TarPits
//! ```
#![allow(non_snake_case)]

use std::time::Instant;

use sc_contracts::{
    BlueprintPoolRegistry, Contract, HandlerKind, LocalityRegistry, LocationRegistry, RewardAmount,
    RewardCurrencyCatalog, ShipRegistry, expand_all, find_bp_conflicts, merge_expansions,
    merge_stats,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let filter: Option<String> = args
        .iter()
        .position(|a| a == "--filter")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_lowercase());

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

    let ships = ShipRegistry::build(&datacore);
    let blueprints = BlueprintPoolRegistry::build(&datacore, &asset_data.locale);
    let currency = RewardCurrencyCatalog::build(&datacore);
    let locations = LocationRegistry::build(&datacore, &asset_data.locale);
    let localities = LocalityRegistry::build(&datacore, &locations);

    let t0 = Instant::now();
    let expansions = expand_all(
        &datacore,
        &asset_data.locale,
        &ships,
        &blueprints,
        &currency,
        &localities,
    );
    let expand_secs = t0.elapsed().as_secs_f64();
    let expand_count = expansions.len();

    let t1 = Instant::now();
    let contracts = merge_expansions(expansions);
    let merge_secs = t1.elapsed().as_secs_f64();

    println!();
    println!("expand_all:     {expand_count} expansions in {expand_secs:.2}s");
    println!(
        "merge:          {} contracts in {merge_secs:.2}s",
        contracts.len(),
    );
    println!(
        "collapse ratio: {:.1}× ({} expansions per contract average)",
        expand_count as f64 / contracts.len().max(1) as f64,
        expand_count / contracts.len().max(1),
    );
    println!(
        "SCMDB target:   ~1,497 — {} from target",
        delta_against_target(contracts.len(), 1497)
    );

    let stats = merge_stats(&contracts);
    println!();
    println!("─── Merge stats ─────────────────────────────────");
    println!("  Total contracts:          {}", stats.total_contracts);
    println!("  Total variations:         {}", stats.total_variations);
    println!("  Largest variation group:  {}", stats.largest_variation);
    println!(
        "  Contracts with title siblings: {}  ({:.1}%)",
        stats.contracts_with_title_siblings,
        100.0 * stats.contracts_with_title_siblings as f64 / stats.total_contracts.max(1) as f64,
    );
    println!();
    println!("  Variation count histogram (contracts with N variations):");
    for (n, c) in &stats.variation_count_histogram {
        println!("    {n:>3} var(s)  ×  {c:>5} contract(s)");
    }
    println!();
    println!("  Sibling count histogram (title+desc shared with N others):");
    for (n, c) in &stats.sibling_count_histogram {
        println!("    {n:>3} sib(s)  ×  {c:>5} contract(s)");
    }

    // Handler-kind breakdown.
    let mut by_kind: std::collections::BTreeMap<HandlerKind, usize> =
        std::collections::BTreeMap::new();
    for c in &contracts {
        *by_kind.entry(c.handler_kind).or_default() += 1;
    }
    println!();
    println!("  By handler kind:");
    for (kind, count) in &by_kind {
        println!("    {kind:?} — {count}");
    }

    // Reward content breakdown.
    let with_bp = contracts
        .iter()
        .filter(|c| c.blueprint_reward.is_some())
        .count();
    let with_scrip = contracts
        .iter()
        .filter(|c| !c.reward_scrip.is_empty())
        .count();
    let with_rep = contracts
        .iter()
        .filter(|c| !c.reward_rep.is_empty())
        .count();
    let with_items = contracts
        .iter()
        .filter(|c| !c.reward_items.is_empty())
        .count();
    let uec_calc = contracts
        .iter()
        .filter(|c| matches!(c.reward_uec, RewardAmount::Calculated))
        .count();
    println!();
    println!("  Reward content:");
    println!("    With blueprint:   {with_bp}");
    println!("    With scrip:       {with_scrip}");
    println!("    With rep:         {with_rep}");
    println!("    With items:       {with_items}");
    println!("    Calculated UEC:   {uec_calc}");

    // Filtered rows.
    println!();
    if let Some(f) = filter {
        let matching: Vec<&Contract> = contracts
            .iter()
            .filter(|c| {
                c.debug_name.to_lowercase().contains(&f)
                    || c.handler_debug_name.to_lowercase().contains(&f)
                    || c.title
                        .as_deref()
                        .map(|t| t.to_lowercase().contains(&f))
                        .unwrap_or(false)
            })
            .collect();
        println!(
            "─── Filter '{f}': {} of {} contracts ─────────────",
            matching.len(),
            contracts.len()
        );
        for c in matching.iter().take(25) {
            print_contract(c, &localities);
        }
    } else {
        println!("─── Sample contracts ──────────────────────────");
        println!();
        println!("  [Unique-title] (no siblings):");
        for c in contracts
            .iter()
            .filter(|c| c.title_siblings.is_empty())
            .take(3)
        {
            print_contract(c, &localities);
        }
        println!();
        println!("  [Shared-title] (one+ siblings):");
        for c in contracts
            .iter()
            .filter(|c| !c.title_siblings.is_empty())
            .take(3)
        {
            print_contract(c, &localities);
        }

        // Blueprint conflict summary.
        let conflicts = find_bp_conflicts(&contracts);
        println!();
        println!(
            "─── Blueprint conflicts: {} groups, {} with mixed presence ──",
            conflicts.len(),
            conflicts.iter().filter(|g| g.has_mixed_presence).count()
        );

        // Mission-span summary — contracts touching multiple systems
        // usually explain why title_siblings carry different rewards.
        let multi_system = contracts
            .iter()
            .filter(|c| {
                let mut syss = std::collections::BTreeSet::new();
                for g in &c.mission_span {
                    if let Some(v) = localities.get(g) {
                        for s in &v.systems {
                            syss.insert(s.clone());
                        }
                    }
                }
                syss.len() > 1
            })
            .count();
        let with_span = contracts
            .iter()
            .filter(|c| !c.mission_span.is_empty())
            .count();
        println!();
        println!(
            "─── Mission-span: {with_span} contracts have span ({} locality record(s) total); {multi_system} cross multiple systems",
            contracts
                .iter()
                .map(|c| c.mission_span.len())
                .sum::<usize>(),
        );
    }

    Ok(())
}

fn print_contract(c: &Contract, localities: &sc_contracts::LocalityRegistry) {
    let title = c.title.as_deref().unwrap_or("<no title>");
    let desc_short: String = c
        .description
        .as_deref()
        .unwrap_or("")
        .chars()
        .take(80)
        .collect();
    let desc_tail = if c
        .description
        .as_deref()
        .map(|d| d.chars().count() > 80)
        .unwrap_or(false)
    {
        "…"
    } else {
        ""
    };
    let subst = if c.has_runtime_substitution { " *" } else { "" };
    println!();
    let sibling_marker = if c.title_siblings.is_empty() {
        String::new()
    } else {
        format!(" (+{} sib)", c.title_siblings.len())
    };
    println!(
        "  [{kind:?}]{sib} {hd} / {cd}",
        kind = c.handler_kind,
        sib = sibling_marker,
        hd = c.handler_debug_name,
        cd = c.debug_name,
    );
    println!("      title:    {title}{subst}");
    if !desc_short.is_empty() {
        println!("      desc:     {desc_short}{desc_tail}");
    }
    println!(
        "      variants: {} ({})",
        c.variations.len(),
        c.variations
            .iter()
            .map(|v| match v.origin {
                sc_contracts::ContractOrigin::Contract => "C",
                sc_contracts::ContractOrigin::ContractLegacy => "L",
                sc_contracts::ContractOrigin::CareerContract => "K",
                sc_contracts::ContractOrigin::SubContract { .. } => "S",
            })
            .collect::<String>(),
    );
    // Which variations have extra prereqs?
    let with_extras = c
        .variations
        .iter()
        .filter(|v| !v.extra_prerequisites.is_empty())
        .count();
    if with_extras > 0 {
        println!(
            "      var-ext:  {with_extras} variation(s) add unique prereqs (locations / gates)"
        );
    }

    // Rewards + enc + cd summary.
    let mut bits = Vec::new();
    match c.reward_uec {
        RewardAmount::None => {}
        RewardAmount::Calculated => bits.push("UEC: calc".to_string()),
        RewardAmount::Fixed(n) => bits.push(format!("UEC: {n}")),
    }
    for s in &c.reward_scrip {
        let name = if s.display_name.is_empty() {
            &s.record_name
        } else {
            &s.display_name
        };
        bits.push(format!("{name}×{}", s.amount));
    }
    if !c.reward_rep.is_empty() {
        let total: Vec<String> = c
            .reward_rep
            .iter()
            .map(|r| match r.amount {
                Some(n) => n.to_string(),
                None => "calc".into(),
            })
            .collect();
        bits.push(format!("rep: [{}]", total.join(",")));
    }
    if let Some(bp) = &c.blueprint_reward {
        bits.push(format!("BP×{} ({:.0}%)", bp.items.len(), bp.chance * 100.0));
    }
    if !bits.is_empty() {
        println!("      rewards:  {}", bits.join("; "));
    }
    if !c.mission_span.is_empty() {
        let parts: Vec<String> = c
            .mission_span
            .iter()
            .filter_map(|g| localities.get(g))
            .map(|v| {
                let tag = if v.name.is_empty() {
                    "?".to_string()
                } else {
                    v.name.clone()
                };
                if v.region_label.is_empty() {
                    tag
                } else {
                    format!("{tag} ({})", v.region_label)
                }
            })
            .collect();
        let shown: Vec<String> = parts.iter().take(3).cloned().collect();
        let more = if parts.len() > 3 {
            format!("; … +{}", parts.len() - 3)
        } else {
            String::new()
        };
        println!("      span:     {}{more}", shown.join("; "));
    }
    if !c.encounters.is_empty() {
        let waves: usize = c.encounters.iter().map(|g| g.waves.len()).sum();
        let sample: Vec<String> = c
            .encounters
            .iter()
            .flat_map(|g| g.waves.iter())
            .flat_map(|w| w.slots.iter())
            .flat_map(|s| s.candidates.iter())
            .map(|cand| cand.display_name.clone())
            .filter(|n| !n.is_empty())
            .take(5)
            .collect();
        println!(
            "      enc:      {groups} group(s), {waves} wave(s){ships}",
            groups = c.encounters.len(),
            ships = if sample.is_empty() {
                String::new()
            } else {
                format!(" — sample: {}", sample.join(", "))
            },
        );
    }
}

fn delta_against_target(n: usize, target: usize) -> String {
    let d = n as i64 - target as i64;
    if d == 0 {
        "exactly on target".into()
    } else if d > 0 {
        format!(
            "+{d} over target ({:+.1}%)",
            (d as f64) * 100.0 / target as f64
        )
    } else {
        format!(
            "{d} under target ({:+.1}%)",
            (d as f64) * 100.0 / target as f64
        )
    }
}
