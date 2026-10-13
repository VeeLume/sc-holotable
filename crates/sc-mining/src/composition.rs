//! Compositions — the `MineableComposition` records a rock's elements are
//! rolled from.

use std::collections::{BTreeSet, HashMap};

use sc_extract::{Datacore, Guid, LocaleKey, RecordCollection};
use serde::{Deserialize, Serialize};

use crate::sim::Rng;
use crate::{Entry, QualityRoll, Quantiles};

/// One `MineableComposition`: the parts a rock of this kind can contain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Composition {
    pub guid: Guid,
    /// `depositName`, the scanner's name for the rock.
    pub deposit_name: LocaleKey,
    /// `minimumDistinctElements`: parts forced in when the roll yields fewer.
    pub minimum_distinct_elements: u32,
    pub parts: Vec<Part>,
}

/// One `MineableCompositionPart`. The same element can be listed in more
/// than one part, so a rock can hold it twice (at two qualities).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Part {
    pub element: Option<Guid>,
    /// Percent of the rock's volume.
    pub min_percentage: f32,
    pub max_percentage: f32,
    /// Chance the part is in a rock at all.
    pub probability: f32,
    /// The share is `min + (max - min) * u^curveExponent`, so exponents above
    /// 1 favour the low end.
    pub curve_exponent: f32,
    pub quality_scale: f32,
}

impl Composition {
    /// Roll one rock's composition, as the client does at spawn (LIVE 4.10):
    /// each part in order is in with its `probability`, at
    /// `min + (max - min) * u^curveExponent` percent, and dropped if it would
    /// take the total over 100 %; then, while fewer than `minimumDistinctElements` parts are in,
    /// one more is picked among the rest weighted by `probability`, stopping
    /// at the first that does not fit. Not normalised: the rest is inert.
    ///
    /// One draw: `seed` picks it (same seed, same rock). It is not the game's
    /// seed: the client seeds each rock from its spawn location id, which
    /// exists only at runtime, so no tool can predict a particular rock; for
    /// the odds use [`odds`](Self::odds). Entries carry no quality; see
    /// [`roll_with_quality`](Self::roll_with_quality).
    pub fn roll(&self, seed: u64) -> Vec<Entry> {
        self.roll_parts(&mut Rng::new(seed))
            .into_iter()
            .map(|(entry, _)| entry)
            .collect()
    }

    /// [`roll`](Self::roll), then each entry's quality from the same stream
    /// (one independent roll per entry, scaled by its part's `qualityScale`;
    /// see [`QualityRoll`]).
    pub fn roll_with_quality(&self, seed: u64, quality: &QualityRoll<'_>) -> Vec<Entry> {
        let mut rng = Rng::new(seed);
        let mut entries = self.roll_parts(&mut rng);
        for (entry, part) in &mut entries {
            entry.quality = quality.roll(&entry.element, self.parts[*part].quality_scale, &mut rng);
        }
        entries.into_iter().map(|(entry, _)| entry).collect()
    }

    /// Each part's odds over `runs` rolls: how often it is in a rock and the
    /// share it has when it is.
    pub fn odds(&self, runs: u32) -> Vec<PartOdds> {
        let mut shares: Vec<Vec<f32>> = vec![Vec::new(); self.parts.len()];
        for seed in 0..u64::from(runs) {
            for (entry, part) in self.roll_parts(&mut Rng::new(seed)) {
                shares[part].push(entry.fraction);
            }
        }
        let runs = runs.max(1) as f32;
        self.parts
            .iter()
            .zip(shares)
            .map(|(part, s)| PartOdds {
                element: part.element,
                quality_scale: part.quality_scale,
                chance: s.len() as f32 / runs,
                mean_share: if s.is_empty() {
                    0.0
                } else {
                    s.iter().sum::<f32>() / s.len() as f32
                },
                share: Quantiles::of(s),
            })
            .collect()
    }

    /// The rolled entries with the index of the part each came from.
    fn roll_parts(&self, rng: &mut Rng) -> Vec<(Entry, usize)> {
        let mut total = 0.0f32;
        let mut chosen = BTreeSet::new();
        let mut entries = Vec::new();
        let take = |i: usize, rng: &mut Rng, total: &mut f32, entries: &mut Vec<(Entry, usize)>| {
            let part = &self.parts[i];
            let pct = part.min_percentage
                + (part.max_percentage - part.min_percentage)
                    * rng.unit().powf(part.curve_exponent);
            if pct + *total > 100.0 {
                return false;
            }
            *total += pct;
            if let Some(element) = part.element {
                entries.push((
                    Entry {
                        element,
                        fraction: pct / 100.0,
                        quality: None,
                    },
                    i,
                ));
            }
            true
        };
        for (i, part) in self.parts.iter().enumerate() {
            if rng.unit() <= part.probability && take(i, rng, &mut total, &mut entries) {
                chosen.insert(i);
            }
        }
        while chosen.len() < self.minimum_distinct_elements as usize {
            let open: Vec<usize> = (0..self.parts.len())
                .filter(|i| !chosen.contains(i))
                .collect();
            let weight: f32 = open.iter().map(|&i| self.parts[i].probability).sum();
            if weight <= 0.0 {
                break;
            }
            let target = rng.unit() * weight;
            let mut cumulative = 0.0;
            let Some(&pick) = open.iter().find(|&&i| {
                cumulative += self.parts[i].probability;
                target < cumulative
            }) else {
                break;
            };
            if !take(pick, rng, &mut total, &mut entries) {
                break;
            }
            chosen.insert(pick);
        }
        entries
    }
}

/// One part's odds (see [`Composition::odds`]).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PartOdds {
    pub element: Option<Guid>,
    pub quality_scale: f32,
    /// Chance the part is in a rock.
    pub chance: f32,
    /// Its share of the rock's volume (0..1) when it is in.
    pub share: Quantiles,
    pub mean_share: f32,
}

/// Every `MineableComposition`, keyed by GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Compositions {
    by_guid: HashMap<Guid, Composition>,
}

impl Compositions {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut by_guid = HashMap::new();
        for (&guid, handle) in &store.records.multi_feature.mineable_composition {
            let Some(c) = handle.get(pools) else {
                continue;
            };
            let parts = c
                .composition_array
                .iter()
                .filter_map(|h| h.get(pools))
                .map(|p| Part {
                    element: p.mineable_element,
                    min_percentage: p.min_percentage,
                    max_percentage: p.max_percentage,
                    probability: p.probability,
                    curve_exponent: p.curve_exponent,
                    quality_scale: p.quality_scale,
                })
                .collect();
            by_guid.insert(
                guid,
                Composition {
                    guid,
                    deposit_name: c.deposit_name.clone(),
                    minimum_distinct_elements: c.minimum_distinct_elements.max(0) as u32,
                    parts,
                },
            );
        }
        Self { by_guid }
    }
}

impl RecordCollection for Compositions {
    type Item = Composition;

    fn get(&self, guid: &Guid) -> Option<&Composition> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Composition)> + '_ {
        self.by_guid.iter()
    }
}
