//! Find mission titles where pool members disagree on blueprint rewards.
//!
//! Phase 4 of the v2 redesign: replaces the old `find_bp_conflicts`
//! helper with the precomputed [`MissionPools`] + divergence helpers
//! on [`MissionIndex`]. The same answer falls out of one filter over
//! `index.pools.title_key`.
//!
//! Run:
//! ```bash
//! cargo run -p sc-contracts --release --example bp_title_conflicts
//! cargo run -p sc-contracts --release --example bp_title_conflicts -- --pyro-only
//! ```

use std::collections::HashSet;

use sc_contracts::{MissionIndex, Mission, LocalityRegistry};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let pyro_only = args.iter().any(|a| a == "--pyro-only");

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

    let index = MissionIndex::build(&datacore);
    println!("MissionIndex: {} contract expansions\n", index.len());

    // Collect title-key pools where blueprint rewards diverge across
    // members. The two divergence axes:
    //   blueprint_mixed              some have BP, others don't
    //   !blueprint_pool_consistent   different BP pool GUIDs across members
    let mut groups: Vec<(&str, &[Guid])> = index
        .pools
        .title_key
        .iter()
        .filter(|(_, ids)| {
            ids.len() > 1
                && (index.blueprint_mixed(ids) || !index.blueprint_pool_consistent(ids))
        })
        .map(|(key, ids)| (key.as_str(), ids.as_slice()))
        .collect();

    if pyro_only {
        groups.retain(|(_, ids)| {
            index.iter_pool(ids).any(|c| {
                c.debug_name.to_lowercase().contains("pyro")
                    || c.origin.source_debug_name.to_lowercase().contains("pyro")
            })
        });
    }
    // Deterministic order: most-divergent first (by member count), then alpha.
    groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(b.0)));

    let mixed_count = groups
        .iter()
        .filter(|(_, ids)| index.blueprint_mixed(ids))
        .count();

    println!("═══════════════════════════════════════════════════════════");
    println!(
        "  {} title-key pool(s) with blueprint divergence{}",
        groups.len(),
        if pyro_only {
            " (Pyro-touching only)"
        } else {
            ""
        }
    );
    println!(
        "  {} with mixed BP presence (some members have BP, some don't)",
        mixed_count
    );
    println!("═══════════════════════════════════════════════════════════");

    for (idx, (_key, ids)) in groups.iter().take(15).enumerate() {
        println!();
        let head = index.get(ids[0]).expect("pool member id resolves");
        let title = head.title(&asset_data.locale).unwrap_or(&head.debug_name);
        println!("── #{} title='{}'", idx + 1, truncate(title, 70));
        if let Some(d) = head.description(&asset_data.locale) {
            println!("   desc='{}'", truncate(d, 70));
        }
        let distinct_pools: HashSet<Guid> = index
            .iter_pool(ids)
            .filter_map(|c| c.rewards.blueprint.as_ref().map(|bp| bp.pool_guid))
            .collect();
        let mixed = if index.blueprint_mixed(ids) {
            " — MIXED PRESENCE"
        } else {
            ""
        };
        println!(
            "   {} members across {} distinct BP pool(s){mixed}",
            ids.len(),
            distinct_pools.len(),
        );
        for c in index.iter_pool(ids) {
            let bp_label = match c.rewards.blueprint.as_ref() {
                Some(bp) if !bp.items.is_empty() => {
                    let sample = sample_items(c, &asset_data.locale, &datacore.snapshot().localized_items);
                    format!("  BP pool='{}' ({} items): {sample}", bp.pool_name, bp.items.len())
                }
                _ => "  (no blueprint)".to_string(),
            };
            println!("      [{:?}] {}", c.origin.kind, c.debug_name);
            println!("        {bp_label}");
            let span = render_span(c, &index.localities, &asset_data.locale);
            if !span.is_empty() {
                println!("        span: {span}");
            }
        }
    }

    if groups.len() > 15 {
        println!();
        println!("  … and {} more groups (limited to 15)", groups.len() - 15);
    }

    Ok(())
}

/// Render the contract's mission_span — each locality resolved to
/// its `region_label` (e.g., `"Pyro: Bloom, Rat's Nest"`), prefixed
/// with the locality's stable record name (e.g., `"RegionB"`).
fn render_span(
    c: &Mission,
    localities: &LocalityRegistry,
    locale: &sc_extract::LocaleMap,
) -> String {
    c.mission_span
        .iter()
        .filter_map(|g| localities.get(g))
        .map(|view| {
            let tag = if view.name.is_empty() {
                "?".to_string()
            } else {
                view.name.clone()
            };
            let label = view.region_label(locale);
            if label.is_empty() {
                tag
            } else {
                format!("{tag} ({label})")
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// First 4 BP item display names for an at-a-glance preview.
fn sample_items(
    c: &Mission,
    locale: &sc_extract::LocaleMap,
    cache: &sc_extract::LocalizedItemCache,
) -> String {
    let Some(bp) = &c.rewards.blueprint else {
        return "—".into();
    };
    let names: Vec<&str> = bp
        .items
        .iter()
        .filter_map(|it| it.display_name(cache, locale))
        .take(4)
        .collect();
    if names.is_empty() {
        "—".into()
    } else if bp.items.len() > 4 {
        format!("{}, … +{} more", names.join(", "), bp.items.len() - 4)
    } else {
        names.join(", ")
    }
}

fn truncate(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        s.to_string()
    } else {
        let head: String = chars[..max].iter().collect();
        format!("{head}…")
    }
}
