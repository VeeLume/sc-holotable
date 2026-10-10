//! Census of event ladders ([`sc_missions::Events`]) and the points each
//! event contract awards toward them (`MissionRewards::event_points`). One
//! line per generator contract (expansions repeat it).
//!
//! On 4.10 LIVE: RSI Discovery Month (`Iasi_ScenarioProgress`) has an
//! overall ladder (4500 / 9900 / 24000 / 30000) and three category ladders
//! (Transport, Collection, Defense); 31 contracts award points, each to the
//! overall ladder and one category ladder with the same count. 3 split the
//! overall points across the party (Quantanium mining, medium and large
//! salvage); category points never split.
//!
//! ```bash
//! cargo run -p sc-missions --release --example event_points
//! ```
use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, RecordCollection};
use sc_missions::{Event, LadderKind, Missions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let missions = Missions::build(&datacore);
    let events = &missions.events;

    let ladder_name = |event: &Event, kind: LadderKind| -> String {
        event
            .ladders
            .iter()
            .find(|l| l.kind == kind)
            .and_then(|l| locale.resolve(&l.text_key))
            .map_or_else(|| format!("{kind:?}"), str::to_string)
    };

    // event record name → (event, debug name → line)
    let mut by_event: BTreeMap<String, (Option<&Event>, BTreeMap<String, String>)> =
        BTreeMap::new();
    for (_, m) in missions.iter() {
        for ep in &m.rewards.event_points {
            let event = ep.event.and_then(|g| events.get(&g));
            let split = if ep.split_for_party { "split" } else { "" };
            let ladders: Vec<String> = ep
                .ladders
                .iter()
                .map(|lp| {
                    let name = event.map_or_else(
                        || lp.tag.to_string(),
                        |e| ladder_name(e, LadderKind::Category(lp.tag)),
                    );
                    format!("{name} {}", lp.points)
                })
                .collect();
            let title = missions.title_text(m, locale).unwrap_or_default();
            let key = event.map_or_else(|| "(no event)".into(), |e| e.record_name.clone());
            by_event
                .entry(key)
                .or_insert((event, BTreeMap::new()))
                .1
                .insert(
                    m.debug_name.clone(),
                    format!(
                        "{:>7} {split:<5} | {:<18} {:<48} {title}",
                        ep.points,
                        ladders.join(", "),
                        m.debug_name
                    ),
                );
        }
    }

    for (name, (event, contracts)) in &by_event {
        println!("\n== {name} — {} contracts", contracts.len());
        if let Some(e) = event {
            for l in &e.ladders {
                let tiers: Vec<String> = l.tiers.iter().map(|t| t.min_points.to_string()).collect();
                println!(
                    "   ladder {:<12} tiers {}",
                    ladder_name(e, l.kind),
                    tiers.join(" / ")
                );
            }
        }
        println!("   overall split | category ladders");
        for line in contracts.values() {
            println!("   {line}");
        }
    }

    // Events with category ladders: every entry should feed exactly one,
    // with the same count as its overall points.
    let unmatched = missions
        .iter()
        .flat_map(|(_, m)| &m.rewards.event_points)
        .filter(|ep| {
            let has_categories = ep.event.and_then(|g| events.get(&g)).is_some_and(|e| {
                e.ladders
                    .iter()
                    .any(|l| matches!(l.kind, LadderKind::Category(_)))
            });
            has_categories && (ep.ladders.len() != 1 || ep.ladders[0].points != ep.points)
        })
        .count();
    println!("\nentries not feeding exactly one equal category ladder: {unmatched}");
    println!("events in the catalog: {}", events.len());
    Ok(())
}
