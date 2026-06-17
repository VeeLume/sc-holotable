//! aUEC payout estimation from the engine's contract-reward curve.
//!
//! `GameMode.SC_Default → subsumptionMissionModule.uecCurve` (a `RewardScale`).
//! Requires the `payout` feature, which pulls the `gamemode` sc-extract feature
//! so the `SSubsumptionMission` / `RewardScale` pools are populated.

use sc_extract::Datacore;

use crate::{Mission, RewardAmount};

/// The engine contract-reward curve, read from the game's `GameMode.SC_Default`
/// config. For a `Calculated`-reward mission the payout is
///
/// ```text
/// aUEC = round₂₅₀( i · exp( k · (weighted − m) ) · minutes / 60 )
/// ```
///
/// where `weighted` is the difficulty-weighted skill sum. SC 4.8 ships
/// `i = 1, k = 0.303, m = −37`; validated to 0 aUEC across 20 SCMDB payouts
/// (difficulty 2–7) — see `examples/uec_validate.rs`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UecCurve {
    pub i: f32,
    pub k: f32,
    pub m: f32,
}

impl UecCurve {
    /// Extract the live curve from a parsed [`Datacore`]. The `SC_Default`
    /// game mode's subsumption-mission module — the one with
    /// `useNewMissionSystem = true` — carries the authoritative `uecCurve`.
    /// `None` if the `gamemode` pools are empty or no curve is present.
    pub fn build(datacore: &Datacore) -> Option<Self> {
        let pools = &datacore.records().pools;
        let scale = pools
            .multi_feature
            .ssubsumption_mission
            .iter()
            .flatten()
            .find(|s| s.use_new_mission_system && s.uec_curve.is_some())?
            .uec_curve
            .as_ref()?
            .get(pools)?;
        Some(Self {
            i: scale.i,
            k: scale.k,
            m: scale.m,
        })
    }

    /// Estimate the aUEC payout for a difficulty-weighted skill sum and a
    /// time-to-complete in minutes. `None` for non-positive inputs.
    pub fn estimate(&self, weighted: f32, time_minutes: f32) -> Option<i32> {
        if weighted <= 0.0 || time_minutes <= 0.0 {
            return None;
        }
        let raw = self.i * (self.k * (weighted - self.m)).exp() * (time_minutes / 60.0);
        // Engine quantization: round raw to whole aUEC, then snap to nearest 250.
        let whole = (raw + 0.5).floor();
        Some(((whole * (1.0 / 250.0) + 0.5) as i32) * 250)
    }
}

impl Mission {
    /// Estimate the engine-`Calculated` aUEC reward via the extracted
    /// [`UecCurve`]. `None` unless the reward is `Calculated` and the mission
    /// has a difficulty profile with weights.
    pub fn estimate_uec(&self, curve: &UecCurve) -> Option<i32> {
        if !matches!(self.rewards.uec, RewardAmount::Calculated) {
            return None;
        }
        let d = self.difficulty?;
        let w = d.weights?;
        let weighted = d.mechanical_skill as f32 * w[0]
            + d.mental_load as f32 * w[1]
            + d.risk_of_loss as f32 * w[2]
            + d.game_knowledge as f32 * w[3];
        curve.estimate(weighted, self.time_to_complete)
    }
}
