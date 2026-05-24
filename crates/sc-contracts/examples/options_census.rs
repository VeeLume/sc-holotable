//! Census every `SpawnDescription_ShipOptions` in the DCB that has
//! more than one `option`. For each one, compute the tag-diff across
//! siblings and the spread of `concurrent_amount` / `weight`. Answers:
//!
//! - What fraction of ShipOptions are "alternatives" (options > 1) vs
//!   "degenerate concurrent" (options == 1)?
//! - When alternatives exist, what tags discriminate them?
//!   - `HumanPilotNN` only? → fix can specialise as "difficulty tiers"
//!   - Other axes too? → renderer has to stay generic ("one of N picks")
//! - Are weights ever non-uniform? (Would change "max concurrent" to
//!   "weighted-mode concurrent" in the count rule.)
//! - Are concurrent_amount values ever the same across siblings? (Then
//!   alternatives really are pure ship-pool picks, not tier scaling.)
//!
//! ```bash
//! cargo run -p sc-contracts --release --example options_census
//! ```

use std::collections::{BTreeMap, BTreeSet, HashSet};

use sc_contracts::MissionIndex;
use sc_extract::generated::{DataPools, SpawnDescription_ShipOptions};
use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid, TagTree,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let _index = MissionIndex::build(&datacore);
    let pools = &datacore.records().pools;
    let tree = &datacore.snapshot().tag_tree;

    let mut total = 0usize;
    let mut degenerate = 0usize; // options.len() == 1
    let mut size_hist: BTreeMap<usize, usize> = BTreeMap::new();

    // For alternatives (options > 1):
    let mut weights_uniform = 0usize;
    let mut weights_nonuniform = 0usize;
    let mut concurrents_uniform = 0usize;
    let mut concurrents_varied = 0usize;
    let mut concurrent_distinct_hist: BTreeMap<usize, usize> = BTreeMap::new();

    // Tag-diff classifier — which tag families discriminate the siblings.
    let mut discriminator_hist: BTreeMap<String, usize> = BTreeMap::new();
    let mut samples_by_discriminator: BTreeMap<String, Vec<String>> = BTreeMap::new();
    const SAMPLE_LIMIT: usize = 3;

    for so in pools.multi_feature.spawn_description_ship_options.iter().flatten() {
        total += 1;
        let n = so.options.len();
        *size_hist.entry(n).or_default() += 1;
        if n <= 1 {
            degenerate += 1;
            continue;
        }

        // Resolve options for analysis.
        let opts: Vec<&_> = so
            .options
            .iter()
            .filter_map(|h| h.get(pools))
            .collect();
        if opts.len() < 2 {
            continue;
        }

        // Weight uniformity.
        let first_w = opts[0].weight;
        if opts.iter().all(|o| (o.weight - first_w).abs() < 1e-4) {
            weights_uniform += 1;
        } else {
            weights_nonuniform += 1;
        }

        // Concurrent uniformity + spread.
        let concs: BTreeSet<i32> = opts.iter().map(|o| o.concurrent_amount).collect();
        if concs.len() == 1 {
            concurrents_uniform += 1;
        } else {
            concurrents_varied += 1;
        }
        *concurrent_distinct_hist.entry(concs.len()).or_default() += 1;

        // Tag families that vary across siblings.
        let discriminator = classify_discriminator(so, pools, tree);
        *discriminator_hist
            .entry(discriminator.clone())
            .or_default() += 1;

        // Keep a few samples per discriminator class for hand inspection.
        let bucket = samples_by_discriminator
            .entry(discriminator.clone())
            .or_default();
        if bucket.len() < SAMPLE_LIMIT {
            bucket.push(describe_sample(so, pools, tree));
        }
    }

    // ── Report ────────────────────────────────────────────────────────────
    let alternatives = total - degenerate;
    println!("\n=== ShipOptions census ({} total instances) ===", total);
    println!("  options == 1 (degenerate concurrent slot): {degenerate} ({:.1}%)", pct(degenerate, total));
    println!("  options >= 2 (alternatives — engine picks one): {alternatives} ({:.1}%)", pct(alternatives, total));

    println!("\n--- size histogram ---");
    for (n, count) in &size_hist {
        println!("  options={n:>2}: {count}");
    }

    if alternatives > 0 {
        println!("\n=== Alternatives sub-census (n={alternatives}) ===");
        println!("  weight uniform:    {weights_uniform} ({:.1}%)", pct(weights_uniform, alternatives));
        println!("  weight non-uniform: {weights_nonuniform} ({:.1}%)", pct(weights_nonuniform, alternatives));
        println!();
        println!("  concurrent_amount all equal: {concurrents_uniform} ({:.1}%)", pct(concurrents_uniform, alternatives));
        println!("  concurrent_amount varied:    {concurrents_varied} ({:.1}%)", pct(concurrents_varied, alternatives));
        println!();
        println!("  distinct concurrent values per ShipOptions:");
        for (k, v) in &concurrent_distinct_hist {
            println!("    {k}: {v}");
        }

        println!("\n--- discriminator classes (which tag families differ across siblings) ---");
        let mut sorted: Vec<_> = discriminator_hist.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (disc, count) in &sorted {
            println!("  [{count:>5}]  {disc}");
        }

        println!("\n--- samples per discriminator class (up to {SAMPLE_LIMIT} each) ---");
        let mut sample_keys: Vec<_> = samples_by_discriminator.keys().collect();
        sample_keys.sort_by_key(|k| std::cmp::Reverse(discriminator_hist.get(*k).copied().unwrap_or(0)));
        for key in sample_keys {
            println!("\n  «{key}»");
            for s in &samples_by_discriminator[key] {
                println!("    {s}");
            }
        }
    }

    Ok(())
}

fn pct(n: usize, d: usize) -> f64 {
    if d == 0 { 0.0 } else { n as f64 * 100.0 / d as f64 }
}

/// Compact classification of *which tag families* vary across the
/// siblings inside a single `ShipOptions`. Buckets every tag from
/// every sibling's `tags` list, then for each bucket reports whether
/// the values are identical or varying across siblings.
fn classify_discriminator(
    so: &SpawnDescription_ShipOptions,
    pools: &DataPools,
    tree: &TagTree,
) -> String {
    // Collect (sibling_idx → Set<tag_guid>) for the positive tag list of each option.
    let mut sib_tagsets: Vec<HashSet<Guid>> = Vec::new();
    for oh in &so.options {
        let Some(opt) = oh.get(pools) else { continue };
        let set: HashSet<Guid> = opt
            .tags
            .as_ref()
            .and_then(|h| h.get(pools))
            .map(|tl| tl.tags.iter().copied().collect())
            .unwrap_or_default();
        sib_tagsets.push(set);
    }
    if sib_tagsets.len() < 2 {
        return "<single sibling>".to_string();
    }

    // Union of all tag guids. The discriminator is the set of tags
    // that appear in some but not all siblings.
    let mut all_tags: HashSet<Guid> = HashSet::new();
    for s in &sib_tagsets {
        all_tags.extend(s.iter().copied());
    }

    let mut discriminator_families: BTreeSet<String> = BTreeSet::new();
    for guid in &all_tags {
        let in_all = sib_tagsets.iter().all(|s| s.contains(guid));
        if in_all {
            continue;
        }
        let name = tree.get(guid).map(|n| n.name.as_str()).unwrap_or("?");
        discriminator_families.insert(family_of(name));
    }

    if discriminator_families.is_empty() {
        // Siblings carry identical tags — they must differ on something
        // else (concurrent_amount only? weight only?).
        return "<tags identical — alternatives differ on concurrent/weight only>".to_string();
    }

    discriminator_families
        .into_iter()
        .collect::<Vec<_>>()
        .join(" + ")
}

/// Bucket a tag name into a coarse family for the discriminator
/// report. The intent is to spot "HumanPilotNN" / "AISkill_NN" /
/// "Difficulty_NN" patterns as a single family rather than three
/// distinct tags, so the census reads cleanly.
fn family_of(name: &str) -> String {
    let trimmed = name.trim();
    if let Some(rest) = trimmed.strip_prefix("HumanPilot") {
        if rest.chars().all(|c| c.is_ascii_digit()) {
            return "HumanPilotNN".to_string();
        }
    }
    if let Some(rest) = trimmed.strip_prefix("AISkill_") {
        if rest.chars().all(|c| c.is_ascii_digit()) {
            return "AISkill_NN".to_string();
        }
    }
    if let Some(rest) = trimmed.strip_prefix("AISkill") {
        if rest.chars().all(|c| c.is_ascii_digit()) {
            return "AISkillNN".to_string();
        }
    }
    // VeryEasy / Easy / Medium / Hard / VeryHard
    if matches!(trimmed, "VeryEasy" | "Easy" | "Medium" | "Hard" | "VeryHard") {
        return "DifficultyTier".to_string();
    }
    // Faction-ish heuristic: well-known faction tag names.
    if matches!(
        trimmed,
        "Criminal" | "Outlaw" | "Vanduul" | "UEE_Navy" | "Civilian" | "Pirate" | "XenoThreat" | "Headhunters" | "Nine Tails" | "Dusters"
    ) {
        return "Faction".to_string();
    }
    // Ship-name-ish: contains an underscore or starts uppercase — keep verbatim.
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
            "opt[{oi}] c={} w={:.2} tags=[{}]",
            opt.concurrent_amount,
            opt.weight,
            tag_names.join(",")
        ));
    }
    parts.join("  ‖  ")
}
