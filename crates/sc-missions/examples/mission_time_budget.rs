//! How `Mission::time_to_complete` is spread over the missions, and what it
//! means for the payout.
//!
//! The engine pays a `Calculated` contract `rate(difficulty) · minutes / 60`
//! ([`UecCurve`], validated against SCMDB in `uec_validate`), so the minutes
//! are the time the payout assumes the mission takes. This prints how many
//! missions carry a budget, the spread of the budgets, and sample titles per
//! budget so they can be checked against play.
//!
//! ```bash
//! cargo run -p sc-missions --release --features payout --example mission_time_budget
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
use sc_missions::{Missions, RewardAmount, UecCurve};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let missions = Missions::build(&datacore);
    let curve = UecCurve::build(&datacore).ok_or("no uecCurve")?;

    let total = missions.len();
    let mut zero = 0usize;
    let mut calculated_with = 0usize;
    let mut calculated_without = 0usize;
    // minutes -> (count, sample titles, hourly rates seen)
    let mut buckets: BTreeMap<u32, (usize, Vec<String>, Vec<i32>)> = BTreeMap::new();

    for m in missions.values() {
        let minutes = m.time_to_complete;
        let calculated = matches!(m.rewards.uec, RewardAmount::Calculated);
        if minutes <= 0.0 {
            zero += 1;
            if calculated {
                calculated_without += 1;
            }
            continue;
        }
        if calculated {
            calculated_with += 1;
        }
        let entry = buckets.entry(minutes.round() as u32).or_default();
        entry.0 += 1;
        if entry.1.len() < 4
            && let Some(title) = m.title(locale)
            && !entry.1.iter().any(|t| t == title)
        {
            entry.1.push(title.to_string());
        }
        if calculated
            && let Some(w) = m.difficulty.as_ref().and_then(|d| {
                d.weights.map(|w| {
                    w[0] * d.mechanical_skill as f32
                        + w[1] * d.mental_load as f32
                        + w[2] * d.risk_of_loss as f32
                        + w[3] * d.game_knowledge as f32
                })
            })
            && let Some(per_hour) = curve.estimate(w, 60.0)
        {
            entry.2.push(per_hour);
        }
    }

    println!("missions: {total}");
    println!("  without a budget (0): {zero} ({calculated_without} of them Calculated pay)");
    println!(
        "  with a budget: {} ({calculated_with} of them Calculated pay)",
        total - zero
    );
    println!("\nminutes  count  aUEC/h (min..max)  samples");
    for (minutes, (count, titles, rates)) in &buckets {
        let rate = match (rates.iter().min(), rates.iter().max()) {
            (Some(lo), Some(hi)) => format!("{lo}..{hi}"),
            _ => "-".to_string(),
        };
        println!(
            "{minutes:>7}  {count:>5}  {rate:>17}  {}",
            titles.join(" | ")
        );
    }
    Ok(())
}
