//! What a fractured rock's pieces contain.

use sc_extract::{Guid, RecordCollection};
use serde::{Deserialize, Serialize};

use crate::sim::Rng;
use crate::{Combined, Elements, Entry, Rock, Ruleset, Shape, Spawn};

/// One piece's odds after a fracture (see [`Spawn::fracture_odds`]).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PieceOdds {
    /// m³.
    pub volume: f32,
    /// Whether it can be collected directly.
    pub collectable: bool,
    /// Per parent entry, in the parent's order.
    pub entries: Vec<EntryOdds>,
    /// Chance the piece comes out (almost: under 1 %) pure inert.
    pub pure_inert: f32,
}

/// How much of one parent entry a piece gets.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EntryOdds {
    pub element: Guid,
    pub quality: Option<u16>,
    /// Chance the piece gets any of it.
    pub chance: f32,
    /// Mean share of the piece's volume.
    pub mean_share: f32,
}

/// Entries below this share are dropped from a piece (the client's 1e-4 %).
const MIN_FRACTION: f32 = 1e-6;

impl Spawn {
    /// Each piece's odds over `runs` fractures (see [`fracture`](Self::fracture)).
    pub fn fracture_odds(
        &self,
        ruleset: &Ruleset,
        elements: &Elements,
        cluster: Combined,
        runs: u32,
    ) -> Vec<PieceOdds> {
        let mut odds: Vec<PieceOdds> = self
            .shape
            .pieces
            .iter()
            .map(|p| PieceOdds {
                volume: p.volume,
                collectable: ruleset.is_absorbable(p.volume),
                entries: self
                    .rock
                    .entries
                    .iter()
                    .map(|e| EntryOdds {
                        element: e.element,
                        quality: e.quality,
                        chance: 0.0,
                        mean_share: 0.0,
                    })
                    .collect(),
                pure_inert: 0.0,
            })
            .collect();
        let step = 1.0 / runs.max(1) as f32;
        for seed in 0..u64::from(runs) {
            let pieces = self.fracture(ruleset, elements, cluster, seed);
            for (o, piece) in odds.iter_mut().zip(&pieces) {
                let mut filled = 0.0;
                for (k, parent) in self.rock.entries.iter().enumerate() {
                    let share: f32 = piece
                        .rock
                        .entries
                        .iter()
                        .filter(|e| e.element == parent.element && e.quality == parent.quality)
                        .map(|e| e.fraction)
                        .sum();
                    filled += share;
                    if share > 0.0 {
                        o.entries[k].chance += step;
                    }
                    o.entries[k].mean_share += share * step;
                }
                if filled < 0.01 {
                    o.pure_inert += step;
                }
            }
        }
        odds
    }

    /// The rocks this one breaks into: one per piece of its [`Shape`], each
    /// with the piece's own pieces and a share of the parent's elements.
    ///
    /// Client code (LIVE 4.10, `SetChildCompositions` / `RedistributeElement`):
    /// every piece starts from the parent's composition; then each element
    /// entry, in order, is poured anew. Each piece draws `w = u - t` (0 when
    /// `u < t`), `t = max(0, clusterFactor * factor + add)` from the
    /// element's cluster factor and the setup's cluster modifier
    /// ([`ModifiedRock::cluster_factor`](crate::ModifiedRock::cluster_factor));
    /// the element's total goes out by share `volume * w`, largest share
    /// first, capped at what each piece has room for, the overflow to pieces
    /// with room. With no weight at all, one random piece takes everything.
    /// Each element's volume is conserved; inert is what each piece has
    /// left; qualities stay with their entries. The client draws from a
    /// process-wide random stream, so a given fracture cannot be predicted,
    /// only its odds: `seed` picks one possible outcome; see
    /// [`fracture_odds`](Self::fracture_odds). An overcharged fracture (fewer
    /// pieces) is not modelled.
    pub fn fracture(
        &self,
        ruleset: &Ruleset,
        elements: &Elements,
        cluster: Combined,
        seed: u64,
    ) -> Vec<Spawn> {
        let pieces = &self.shape.pieces;
        if pieces.is_empty() {
            return Vec::new();
        }
        let volumes: Vec<f32> = pieces.iter().map(|p| p.volume).collect();
        let total_volume: f32 = volumes.iter().sum();
        if total_volume <= 0.0 {
            return Vec::new();
        }
        let mut rng = Rng::new(seed);
        let mut used = vec![0.0f32; pieces.len()];
        let mut shares: Vec<Vec<f32>> =
            vec![Vec::with_capacity(self.rock.entries.len()); pieces.len()];
        for entry in &self.rock.entries {
            let cluster_factor = elements
                .get(&entry.element)
                .map_or(0.0, |e| e.cluster_factor);
            let threshold = cluster.apply(cluster_factor).max(0.0);
            let amount_total = entry.fraction * total_volume;
            let mut weights: Vec<f32> = volumes
                .iter()
                .map(|v| {
                    let u = rng.unit();
                    let w = if u >= threshold { u - threshold } else { 0.0 };
                    v / total_volume * w
                })
                .collect();
            let mut sum: f32 = weights.iter().sum();
            if sum <= 0.0 {
                let pick = ((rng.unit() * pieces.len() as f32) as usize).min(pieces.len() - 1);
                weights[pick] = 1.0;
                sum = 1.0;
            }
            let mut order: Vec<usize> = (0..pieces.len()).collect();
            order.sort_by(|&a, &b| weights[b].total_cmp(&weights[a]));
            let mut amount = vec![0.0f32; pieces.len()];
            let mut leftover = 0.0f32;
            for &c in &order {
                let want = weights[c] / sum * amount_total;
                let give = want.min(volumes[c] - used[c]).max(0.0);
                amount[c] = give;
                used[c] += give;
                leftover += want - give;
            }
            for &c in &order {
                let give = leftover.min(volumes[c] - used[c]).max(0.0);
                amount[c] += give;
                used[c] += give;
                leftover -= give;
            }
            for (c, share) in shares.iter_mut().enumerate() {
                share.push(amount[c] / volumes[c]);
            }
        }

        pieces
            .iter()
            .zip(&shares)
            .filter_map(|(piece, share)| {
                let mut filled = 0.0f32;
                let entries: Vec<Entry> = self
                    .rock
                    .entries
                    .iter()
                    .zip(share)
                    .filter(|&(_, &f)| f >= MIN_FRACTION)
                    .map(|(e, &f)| {
                        let fraction = f.min(1.0 - filled).max(0.0);
                        filled += fraction;
                        Entry { fraction, ..*e }
                    })
                    .collect();
                let rock = Rock::from_volume(ruleset, elements, &entries, piece.volume)?;
                Some(Spawn {
                    scale: self.scale,
                    shape: Shape {
                        path: self.shape.path.clone(),
                        volume: piece.volume,
                        pieces: piece.pieces.clone(),
                    },
                    rock,
                })
            })
            .collect()
    }
}
