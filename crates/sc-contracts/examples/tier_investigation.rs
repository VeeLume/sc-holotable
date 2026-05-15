//! Phase A1 investigation — does the live DCB carry typed difficulty
//! / risk / tier data on contracts, sufficient to derive a
//! `Contract.tier` field without falling back to substring matching?
//!
//! Outputs four sections:
//!
//! 1. **`ContractResults.difficulty` coverage** — how many
//!    Contract / CareerContract / ContractLegacy records carry it,
//!    distribution of the four 1-7 scale enums.
//! 2. **Sub-contract sharing** — do parents with many sub-contracts
//!    use them as risk tiers? If yes, ContractDifficulty (which lives
//!    on the parent) can't distinguish them.
//! 3. **Tag-tree probe** — tags whose names hint at risk/tier/rank.
//!    Counts entities + contracts that carry them.
//! 4. **Bounty-family deep dive** — pulls a few contracts whose
//!    debug_name suggests a tier label (Easy/Mod/High/...) and dumps
//!    their typed fields side-by-side.
//!
//! Run:
//!
//! ```bash
//! cargo run -p sc-contracts --release --example tier_investigation
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::generated::{
    Contract, ContractDifficulty, ContractResults, EDifficultyRange_GameKnowledge,
    EDifficultyRange_MechanicalSkill, EDifficultyRange_MentalLoad, EDifficultyRange_RiskOfLoss,
    Handle,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

#[derive(Default)]
struct DiffStats {
    populated: usize,
    total: usize,
    risk_of_loss: BTreeMap<String, usize>,
    mech_skill: BTreeMap<String, usize>,
    mental_load: BTreeMap<String, usize>,
    game_knowledge: BTreeMap<String, usize>,
    distinct_profiles: HashMap<Guid, usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let pools = &datacore.records().pools;
    let snap = datacore.snapshot();
    let tag_tree = &snap.tag_tree;

    // ── Section 1: ContractResults.difficulty coverage ───────────────────
    let mut contract_stats = DiffStats::default();
    let mut career_stats = DiffStats::default();
    let mut legacy_stats = DiffStats::default();

    for c in pools.multi_feature.contract.iter().flatten() {
        contract_stats.total += 1;
        if let Some(d) = resolve_difficulty(&c.contract_results, pools) {
            tally(&mut contract_stats, d);
        }
    }
    for c in pools.contracts.career_contract.iter().flatten() {
        career_stats.total += 1;
        if let Some(d) = resolve_career_difficulty(&c.contract_results, pools) {
            tally(&mut career_stats, d);
        }
    }
    for c in pools.contracts.contract_legacy.iter().flatten() {
        legacy_stats.total += 1;
        if let Some(d) = resolve_legacy_difficulty(&c.contract_results, pools) {
            tally(&mut legacy_stats, d);
        }
    }

    println!("=== Section 1: ContractResults.difficulty coverage ===");
    print_stats("Contract       ", &contract_stats);
    print_stats("CareerContract ", &career_stats);
    print_stats("ContractLegacy ", &legacy_stats);
    println!();

    // ── Section 2: sub-contract sharing ───────────────────────────────────
    let mut sub_count_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut multi_subs_with_diff = 0usize;
    let mut multi_subs_total = 0usize;

    for c in pools.multi_feature.contract.iter().flatten() {
        let n = c.sub_contracts.len();
        *sub_count_hist.entry(n).or_default() += 1;
        if n > 1 {
            multi_subs_total += 1;
            if resolve_difficulty(&c.contract_results, pools).is_some() {
                multi_subs_with_diff += 1;
            }
        }
    }

    println!("=== Section 2: sub-contract sharing on Contract ===");
    println!("sub_contract count distribution:");
    let mut shown = 0;
    for (k, v) in &sub_count_hist {
        println!("  n={k:>3}: {v}");
        shown += 1;
        if shown > 15 {
            break;
        }
    }
    println!(
        "Contracts with >1 sub-contract: {multi_subs_total}, of which {multi_subs_with_diff} have a parent ContractDifficulty",
    );
    println!(
        "  → if parents with many subs ARE bounty families, those {multi_subs_with_diff} share ONE difficulty across all sub-contracts.",
    );
    println!();

    // ── Section 3: tag-tree probe ──────────────────────────────────────────
    println!("=== Section 3: tag-tree probe — names hinting at tier / risk / rank ===");
    let needles = [
        "risk",
        "tier",
        "rank",
        "easy",
        "moderate",
        "medium",
        "hard",
        "high",
        "low",
        "extreme",
        "verylow",
        "veryhigh",
        "difficulty",
    ];
    let mut hits: Vec<(String, usize)> = Vec::new();
    for node in tag_tree.iter() {
        let lower = node.name.to_lowercase();
        if needles.iter().any(|n| lower.contains(n)) {
            // Count how many ECDs carry this tag (rough: walk pools)
            let count = count_tag_carriers(node.guid, pools);
            if count > 0 || lower.contains("risk") || lower.contains("tier") {
                hits.push((node.name.clone(), count));
            }
        }
    }
    hits.sort_by(|a, b| b.1.cmp(&a.1));
    for (name, count) in hits.iter().take(40) {
        println!("  {name:<48} {count} carriers");
    }
    println!("  (total hits: {})", hits.len());
    println!();

    // ── Section 4: bounty-family deep dive ────────────────────────────────
    println!("=== Section 4: bounty-family deep dive ===");
    let labels = [
        "VeryEasy", "Easy", "Moderate", "Medium", "High", "VeryHigh", "Extreme", "Low",
    ];

    // Find all Contract records whose debug_name contains a tier label,
    // group by the label-stripped prefix.
    let mut family_groups: HashMap<String, Vec<&Contract>> = HashMap::new();
    for c in pools.multi_feature.contract.iter().flatten() {
        let dn = &c.debug_name;
        for label in &labels {
            if let Some(pos) = dn.find(label) {
                let prefix = &dn[..pos];
                family_groups.entry(prefix.to_string()).or_default().push(c);
                break;
            }
        }
    }

    let mut families: Vec<(&String, &Vec<&Contract>)> =
        family_groups.iter().filter(|(_, v)| v.len() >= 3).collect();
    families.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    for (prefix, members) in families.iter().take(5) {
        println!(
            "\n--- family prefix: {prefix:?} ({} members) ---",
            members.len()
        );
        for c in members.iter() {
            let d = resolve_difficulty(&c.contract_results, pools);
            print!("  {:<60}", c.debug_name);
            print!("  subs={:>2}", c.sub_contracts.len());
            match d {
                Some(diff) => {
                    print!(
                        "  RoL={:<24} MS={:<24} ML={:<24} GK={:<24}",
                        risk_label(&diff.risk_of_loss),
                        mech_label(&diff.mechanical_skill),
                        mental_label(&diff.mental_load),
                        knowledge_label(&diff.game_knowledge),
                    );
                    if let Some(p) = diff.difficulty_profile {
                        print!("  profile={p:?}");
                    }
                }
                None => print!("  (no ContractDifficulty)"),
            }
            println!();
        }
    }

    Ok(())
}

fn resolve_difficulty<'a>(
    cr: &Option<Handle<ContractResults>>,
    pools: &'a sc_extract::DataPools,
) -> Option<&'a ContractDifficulty> {
    let cr = cr.and_then(|h| h.get(pools))?;
    cr.difficulty.and_then(|h| h.get(pools))
}

fn resolve_career_difficulty<'a>(
    cr: &Option<Handle<ContractResults>>,
    pools: &'a sc_extract::DataPools,
) -> Option<&'a ContractDifficulty> {
    resolve_difficulty(cr, pools)
}

fn resolve_legacy_difficulty<'a>(
    cr: &Option<Handle<ContractResults>>,
    pools: &'a sc_extract::DataPools,
) -> Option<&'a ContractDifficulty> {
    resolve_difficulty(cr, pools)
}

fn tally(s: &mut DiffStats, d: &ContractDifficulty) {
    s.populated += 1;
    *s.risk_of_loss
        .entry(risk_label(&d.risk_of_loss))
        .or_default() += 1;
    *s.mech_skill
        .entry(mech_label(&d.mechanical_skill))
        .or_default() += 1;
    *s.mental_load
        .entry(mental_label(&d.mental_load))
        .or_default() += 1;
    *s.game_knowledge
        .entry(knowledge_label(&d.game_knowledge))
        .or_default() += 1;
    if let Some(p) = d.difficulty_profile {
        *s.distinct_profiles.entry(p).or_default() += 1;
    }
}

fn print_stats(label: &str, s: &DiffStats) {
    println!(
        "{label}: {} / {} populated ({:.1}%)",
        s.populated,
        s.total,
        if s.total > 0 {
            100.0 * s.populated as f32 / s.total as f32
        } else {
            0.0
        }
    );
    if s.populated == 0 {
        return;
    }
    print_hist("  risk_of_loss   ", &s.risk_of_loss);
    print_hist("  mech_skill     ", &s.mech_skill);
    print_hist("  mental_load    ", &s.mental_load);
    print_hist("  game_knowledge ", &s.game_knowledge);
    println!(
        "  distinct difficulty_profile refs: {}",
        s.distinct_profiles.len()
    );
}

fn print_hist(label: &str, h: &BTreeMap<String, usize>) {
    let mut entries: Vec<_> = h.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    let formatted: Vec<String> = entries
        .iter()
        .take(8)
        .map(|(k, v)| format!("{k}={v}"))
        .collect();
    println!("{label}: {}", formatted.join(", "));
}

fn risk_label(v: &EDifficultyRange_RiskOfLoss) -> String {
    format!("{v:?}")
}
fn mech_label(v: &EDifficultyRange_MechanicalSkill) -> String {
    format!("{v:?}")
}
fn mental_label(v: &EDifficultyRange_MentalLoad) -> String {
    format!("{v:?}")
}
fn knowledge_label(v: &EDifficultyRange_GameKnowledge) -> String {
    format!("{v:?}")
}

fn count_tag_carriers(tag: Guid, pools: &sc_extract::DataPools) -> usize {
    let mut count = 0;
    for ecd in pools.multi_feature.entity_class_definition.iter().flatten() {
        if ecd.tags.iter().any(|t| *t == tag) {
            count += 1;
        }
    }
    count
}
