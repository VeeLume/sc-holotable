//! What each mission's completion-tag grant adds, next to the chain gates
//! that ask for those tags.
//!
//! A `CompletedContractTags` gate's `required_count` is tag points, not runs:
//! a run adds the grant's `count`. This prints, for a few chains, every tag a
//! mission grants with its count, and every gate requiring that tag with its
//! `required_count`, so runs = `required_count / count` can be read off. It
//! also reports corpus-wide shape: count values, tags granted twice by one
//! mission, and missions carrying several chain gates.
//!
//! ```bash
//! cargo run -p sc-missions --release --example completion_tag_grants
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};
use sc_missions::{Mission, Missions, PrereqView, RecordCollection};

/// Debug-name prefixes to spot-check (display scoping only, never a join).
const SPOT: &[&str] = &[
    "FoxwellEnforcement_ShipAmbush_Pyro",
    "CleanAir_Killship_Medium",
    "CleanAir_RecoverCargo_VIP",
    "CleanAir_ResourceGathering_Research_MolinaMold",
];

fn gates(m: &Mission) -> impl Iterator<Item = (&[Guid], i32)> + '_ {
    m.prerequisites.iter().filter_map(|p| match p {
        PrereqView::CompletedContractTags {
            required_tags,
            required_count,
            ..
        } => Some((required_tags.as_slice(), *required_count)),
        _ => None,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let index = Missions::build(&datacore);
    let tag_name = |g: &Guid| -> String {
        let p = index.tag_tree.path(g);
        if p.is_empty() {
            g.to_string()
        } else {
            p.join(".")
        }
    };

    // Gates by required tag: tag -> [(gated mission, required_count)].
    let mut gated_by: HashMap<Guid, Vec<(&Mission, i32)>> = HashMap::new();
    for m in index.values() {
        for (tags, n) in gates(m) {
            for t in tags {
                gated_by.entry(*t).or_default().push((m, n));
            }
        }
    }

    // ── Corpus shape ─────────────────────────────────────────────────────────
    let mut counts: BTreeMap<i32, usize> = BTreeMap::new();
    let mut twice = Vec::new();
    for m in index.values() {
        let mut per_tag: HashMap<Guid, Vec<i32>> = HashMap::new();
        for g in &m.grants_completion_tags {
            *counts.entry(g.count).or_default() += 1;
            per_tag.entry(g.tag).or_default().push(g.count);
        }
        for (t, cs) in per_tag {
            if cs.len() > 1 {
                twice.push((m.debug_name.clone(), tag_name(&t), cs));
            }
        }
    }
    let granting = index
        .values()
        .filter(|m| !m.grants_completion_tags.is_empty())
        .count();
    println!("missions={} granting={granting}", index.len());
    println!("grant count values (value: grants): {counts:?}");
    println!(
        "same tag granted more than once by one mission: {}",
        twice.len()
    );
    for (d, t, cs) in twice.iter().take(10) {
        println!("  {d}  {t}  counts={cs:?}");
    }
    let mut multi_gate: Vec<&Mission> = index.values().filter(|m| gates(m).count() > 1).collect();
    multi_gate.sort_by_key(|m| (std::cmp::Reverse(gates(m).count()), m.debug_name.clone()));
    multi_gate.dedup_by(|a, b| a.debug_name == b.debug_name);
    let multi_tag = index
        .values()
        .flat_map(gates)
        .filter(|(tags, _)| tags.len() > 1)
        .count();
    println!(
        "missions with >1 chain gate: {}   gates listing >1 tag: {multi_tag}",
        multi_gate.len()
    );

    // ── Spot-check chains ────────────────────────────────────────────────────
    for prefix in SPOT {
        println!("\n=== {prefix}* ===");
        let mut ms: Vec<&Mission> = index
            .values()
            .filter(|m| m.debug_name.starts_with(prefix))
            .collect();
        // One expansion per debug name with the same grants is plenty.
        ms.sort_by(|a, b| a.debug_name.cmp(&b.debug_name));
        ms.dedup_by(|a, b| {
            a.debug_name == b.debug_name && a.grants_completion_tags == b.grants_completion_tags
        });
        for m in ms {
            println!(
                "{}  \"{}\"",
                m.debug_name,
                m.title(locale).unwrap_or_default()
            );
            for g in &m.grants_completion_tags {
                println!("  grants {} +{}", tag_name(&g.tag), g.count);
                let mut lines: Vec<(String, i32)> = gated_by
                    .get(&g.tag)
                    .map(Vec::as_slice)
                    .unwrap_or(&[])
                    .iter()
                    .map(|(gm, n)| {
                        let title = gm.title(locale).unwrap_or_default();
                        (format!("{} \"{title}\"", gm.debug_name), *n)
                    })
                    .collect();
                lines.sort();
                lines.dedup();
                for (gate, n) in lines {
                    let runs = if g.count > 0 {
                        format!("{:.1}", f64::from(n) / f64::from(g.count))
                    } else {
                        "-".into()
                    };
                    println!("    gate {gate} required_count={n} -> runs={runs}");
                }
            }
        }
    }

    // ── Missions with several chain gates ────────────────────────────────────
    println!("\n=== missions with the most chain gates ===");
    let red = multi_gate
        .iter()
        .filter(|m| m.debug_name == "FoxwellEnforcement_ShipAmbush_Pyro_Super");
    for m in red.chain(multi_gate.iter().take(6)) {
        println!(
            "{}  \"{}\"",
            m.debug_name,
            m.title(locale).unwrap_or_default()
        );
        for (tags, n) in gates(m) {
            let names: Vec<String> = tags.iter().map(tag_name).collect();
            println!("  needs {n} of {names:?}");
        }
    }
    Ok(())
}
