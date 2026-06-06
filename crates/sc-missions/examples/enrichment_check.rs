//! Validate the Missions-sweep enrichment against LIVE: faction / category /
//! difficulty / chain / location-kind resolution + ServiceBeacon coverage.
//!
//! ```bash
//! cargo run -p sc-missions --release --example enrichment_check
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
use sc_missions::{HandlerKind, Missions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let index = Missions::build(&datacore);

    // ── Coverage ─────────────────────────────────────────────────────────────
    let n = index.len();
    let with_cat = index.iter().filter(|m| m.category.is_some()).count();
    let with_fac = index.iter().filter(|m| m.faction.is_some()).count();
    let with_diff = index.iter().filter(|m| m.difficulty.is_some()).count();
    let with_chain = index.iter().filter(|m| !m.grants_completion_tags.is_empty()).count();
    let with_buyin = index.iter().filter(|m| m.buy_in > 0).count();
    let with_rep = index
        .iter()
        .filter(|m| {
            m.prerequisites
                .iter()
                .any(|p| matches!(p, sc_missions::PrereqView::Reputation { .. }))
        })
        .count();
    let svc = index
        .iter()
        .filter(|m| m.origin.kind == HandlerKind::ServiceBeacon)
        .count();
    println!("missions={n}");
    println!("  category={with_cat}  faction={with_fac}  difficulty={with_diff}  grantsChainTags={with_chain}  buyIn>0={with_buyin}  repRequired={with_rep}");
    println!("  ServiceBeacon missions (newly covered)={svc}");
    println!("  registries: factions={} missionTypes={} repStandings={}",
        index.factions.len(), index.mission_types.len(), index.rep_standings.len());

    // ── Spot-check Ling Family ──────────────────────────────────────────────
    println!("\n=== Ling Family Hauling spot-check ===");
    let mut shown = 0;
    for m in index.iter() {
        let Some(t) = m.title(locale) else { continue };
        if !t.to_lowercase().contains("ling") {
            continue;
        }
        let faction = m
            .faction
            .and_then(|g| index.factions.get(&g))
            .and_then(|f| locale.resolve(&f.display_name_key))
            .unwrap_or("—");
        if faction != "Ling Family Hauling" {
            continue;
        }
        let category = m
            .category
            .and_then(|g| index.mission_types.get(&g))
            .and_then(|c| locale.resolve(&c.name_key))
            .unwrap_or("—");
        let diff = m
            .difficulty
            .map(|d| format!("[{},{},{},{}]", d.mechanical_skill, d.mental_load, d.risk_of_loss, d.game_knowledge))
            .unwrap_or_else(|| "—".into());
        let prereqs = index.prerequisite_missions(m).len();
        // location kinds across the mission's localities
        let kinds: Vec<String> = m
            .mission_span
            .iter()
            .filter_map(|g| index.localities.get(g))
            .flat_map(|l| l.locations.iter())
            .filter_map(|loc| loc.kind.as_ref().map(|k| k.as_dcb_str().to_string()))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        let bp = if m.rewards.blueprints.is_empty() { "" } else { " ⚑BP" };
        println!(
            "  [{category}] diff={diff} buyIn={} grantsTags={} prereqMissions={prereqs}{bp}\n      kinds={kinds:?} :: {t}",
            m.buy_in,
            m.grants_completion_tags.len(),
        );
        shown += 1;
        if shown >= 14 {
            break;
        }
    }

    Ok(())
}
