//! Validate the *extracted* aUEC estimator against known SCMDB payouts.
//!
//! Builds [`UecCurve`] from the game's `GameMode.SC_Default.uecCurve` (no longer
//! a hardcoded formula) and applies it to a set of missions the maintainer
//! looked up on SCMDB, printing estimate vs known. The set spans difficulty
//! 2–7 and should resolve every row to 0.0% after rounding; any regression
//! after a data/extraction change shows up here.
//!
//! Requires the `payout` feature (for `UecCurve`):
//!
//! ```bash
//! cargo run -p sc-missions --release --features payout --example uec_validate
//! ```

use std::collections::BTreeSet;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
use sc_missions::{Missions, RewardAmount, UecCurve};

/// Known SCMDB payouts the maintainer provided (title substring → aUEC).
/// Several titles have system/rep variants with distinct payouts; we print
/// every matching expansion so the spread is visible.
const SAMPLES: &[(&str, &[i32])] = &[
    // ── Levels 2–5, standard profile (wsum 1.0) — the calibration set ────────
    ("A Chance to Impress", &[26_000]),
    ("Les Arlington", &[35_750]),
    ("Eliminate Annoyance", &[33_750]),
    ("Reduce Overpopulation", &[30_500]),
    ("Security Contractor Evaluation", &[15_750]),
    ("Access Engineering Files", &[37_500]),
    ("Updated Energy Anomaly Data", &[41_500]),
    ("Updated Security Data", &[62_000]),
    ("Updated Seismic Data", &[82_750]),
    ("Updated Power Usage Data", &[103_500]),
    ("Onyx Personnel Files", &[21_500]),
    ("Intensive Eradication Effort", &[112_250]),
    ("Alliance Aid: Hauler Hunters", &[84_000]),
    // ── Levels 6–7 — top-of-curve confirmation (r holds, no taper) ───────────
    ("Advanced Tracker License Certification", &[91_000]),
    ("Master Tracker License Certification", &[141_750]),
    ("Obtain Irradiated Valakkar Pearls", &[695_250]), // L6, high-sum profile 1.391
    ("Retrieve Additional Smuggler Intel", &[1_370_500]),
    ("Tactical Strike Group Needed", &[2_878_250]),
    // ── Known anomaly (NOT a formula error): the variant we extract is +22%
    //    off SCMDB — an event-mission / title-collision mismatch, kept here to
    //    flag it. See all matching variants in the output.
    ("Eliminate XenoThreat Enforcer", &[260_750]),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let index = Missions::build(&datacore);

    let curve = UecCurve::build(&datacore)
        .ok_or("GameMode.SC_Default uecCurve not found (gamemode pools empty?)")?;
    println!(
        "extracted GameMode.SC_Default.uecCurve: i={} k={} m={}\n",
        curve.i, curve.k, curve.m
    );

    println!("=== aUEC estimator vs known SCMDB payouts ===\n");
    for (needle, known) in SAMPLES {
        // Dedup expansions sharing the same (weighted, time) — they collapse to
        // one displayed payout in Hearth, so showing them once matches the UI.
        let mut seen: BTreeSet<(u32, u32)> = BTreeSet::new();
        let mut rows: Vec<(f32, f32, [u8; 4], Option<i32>)> = Vec::new();
        for m in index.values() {
            let Some(title) = m.title(locale) else {
                continue;
            };
            if !title.contains(needle) {
                continue;
            }
            if !matches!(m.rewards.uec, RewardAmount::Calculated) {
                continue;
            }
            let Some(d) = m.difficulty else { continue };
            let Some(w) = d.weights else { continue };
            let levels = [
                d.mechanical_skill,
                d.mental_load,
                d.risk_of_loss,
                d.game_knowledge,
            ];
            let weighted = levels[0] as f32 * w[0]
                + levels[1] as f32 * w[1]
                + levels[2] as f32 * w[2]
                + levels[3] as f32 * w[3];
            let t = m.time_to_complete;
            let key = ((weighted * 1000.0) as u32, (t * 10.0) as u32);
            if !seen.insert(key) {
                continue;
            }
            rows.push((weighted, t, levels, curve.estimate(weighted, t)));
        }
        // Match each known value to its nearest estimate for a % error readout.
        println!("{needle}  (known: {known:?})");
        if rows.is_empty() {
            println!("    — no Calculated+difficulty expansion found\n");
            continue;
        }
        for (weighted, t, levels, est) in &rows {
            let est_s = est.map(|e| e.to_string()).unwrap_or_else(|| "—".into());
            let err = est
                .map(|e| {
                    let nearest = known
                        .iter()
                        .min_by_key(|k| (**k - e).unsigned_abs())
                        .copied()
                        .unwrap_or(0);
                    let pct = (e - nearest) as f32 / nearest as f32 * 100.0;
                    format!("nearest known {nearest} ({pct:+.1}%)")
                })
                .unwrap_or_default();
            println!(
                "    levels={levels:?} weighted={weighted:.3} time={t:.1}  ->  est {est_s}  [{err}]"
            );
        }
        println!();
    }

    Ok(())
}
