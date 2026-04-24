//! Find mission titles where siblings disagree on blueprint rewards.
//!
//! Uses the top-level [`ContractIndex`] + [`find_bp_conflicts`] —
//! the canonical pattern for sc-langpatch-style annotation. For
//! every title where siblings ship different blueprint pools,
//! renders the pools + the resolved `mission_span` so the reason
//! for the fork (inner Pyro vs outer Pyro, Stanton vs Pyro, etc.)
//! is visible on the row.
//!
//! Run:
//! ```bash
//! cargo run -p sc-contracts --release --example bp_title_conflicts
//! cargo run -p sc-contracts --release --example bp_title_conflicts -- --pyro-only
//! ```

use sc_contracts::{find_bp_conflicts, Contract, ContractIndex, LocalityRegistry};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

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

    let index = ContractIndex::build(&datacore, &asset_data.locale);
    println!("ContractIndex: {} merged contracts\n", index.len());

    let mut conflicts = find_bp_conflicts(&index.contracts);
    if pyro_only {
        conflicts.retain(|g| {
            g.members.iter().any(|m| {
                let Some(c) = index.get(m.contract_id) else { return false };
                c.debug_name.to_lowercase().contains("pyro")
                    || c.handler_debug_name.to_lowercase().contains("pyro")
            })
        });
    }

    println!("═══════════════════════════════════════════════════════════");
    println!(
        "  {} blueprint conflict group(s){}",
        conflicts.len(),
        if pyro_only { " (Pyro-touching only)" } else { "" }
    );
    println!(
        "  {} with mixed BP presence (some variants have BP, some don't)",
        conflicts.iter().filter(|g| g.has_mixed_presence).count()
    );
    println!("═══════════════════════════════════════════════════════════");

    for (idx, group) in conflicts.iter().take(15).enumerate() {
        println!();
        println!("── #{} title='{}'", idx + 1, truncate(&group.title, 70));
        if let Some(d) = &group.description {
            println!("   desc='{}'", truncate(d, 70));
        }
        println!(
            "   {} siblings across {} distinct BP pool(s){}",
            group.members.len(),
            group.distinct_pool_count,
            if group.has_mixed_presence { " — MIXED PRESENCE" } else { "" }
        );
        for m in &group.members {
            let Some(c) = index.get(m.contract_id) else { continue };
            let bp_label = match (&m.pool_name, m.item_count) {
                (Some(pool), n) if n > 0 => {
                    let sample = sample_items(c);
                    format!("  BP pool='{pool}' ({n} items): {sample}")
                }
                _ => "  (no blueprint)".to_string(),
            };
            println!("      [{:?}] {}", m.handler_kind, m.debug_name);
            println!("        {bp_label}");
            let span = render_span(c, &index.localities);
            if !span.is_empty() {
                println!("        span: {span}");
            }
        }
    }

    if conflicts.len() > 15 {
        println!();
        println!("  … and {} more groups (limited to 15)", conflicts.len() - 15);
    }

    Ok(())
}

/// Render the contract's mission_span — each locality resolved to
/// its `region_label` (e.g., `"Pyro: Bloom, Rat's Nest"`), prefixed
/// with the locality's stable record name (e.g., `"RegionB"`).
fn render_span(c: &Contract, localities: &LocalityRegistry) -> String {
    c.mission_span
        .iter()
        .filter_map(|g| localities.get(g))
        .map(|view| {
            let tag = if view.name.is_empty() {
                "?".to_string()
            } else {
                view.name.clone()
            };
            if view.region_label.is_empty() {
                tag
            } else {
                format!("{tag} ({})", view.region_label)
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// First 4 BP item display names for an at-a-glance preview.
fn sample_items(c: &Contract) -> String {
    let Some(bp) = &c.blueprint_reward else { return "—".into() };
    let names: Vec<&str> = bp
        .items
        .iter()
        .filter_map(|it| {
            if it.display_name.is_empty() {
                None
            } else {
                Some(it.display_name.as_str())
            }
        })
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
