//! The quality an element of a freshly spawned rock gets.

use std::collections::BTreeMap;

use sc_extract::{Guid, RecordCollection};
use sc_locations::Locations;
use sc_resources::{
    DistributionRef, LocationOverrideRef, Quality, QualityDistributionShape, QuantizationBand,
    QuantizationRef, Resources,
};
use serde::{Deserialize, Serialize};

use crate::Elements;
use crate::sim::Rng;

/// What a quality roll needs: the resources (each element's quality wiring),
/// the shared quality records, and where the rock is.
#[derive(Debug, Clone, Copy)]
pub struct QualityRoll<'a> {
    pub elements: &'a Elements,
    pub resources: &'a Resources,
    pub catalog: &'a Quality,
    /// The rock's location ids, most specific first (the client walks them
    /// in order and uses the first that has an override). The ids an override
    /// lists are location records (`StarMapObject`s); which ones a spawned
    /// rock reports is runtime, so pass the body / system the rock is at.
    pub locations: &'a [Guid],
}

impl QualityRoll<'_> {
    /// The location ids for a rock at `place` (a `StarMapObject`), most
    /// specific first: the place, its `parent` chain, then its solar system
    /// ([`Locations::system_of`]). Pass them as
    /// [`locations`](Self::locations).
    ///
    /// The client (4.10) takes them from its universe hierarchy
    /// (`CUniverseHierarchyShardGraph::GetLocationIds`): the rock's node, then
    /// every node above it. That hierarchy hangs each MegaMap system's
    /// containers under a node for the system's `SolarSystemRecord`, which is
    /// how Pyro's system-wide overrides reach its rocks; the `parent` chain
    /// alone stops at the star. The system node is read from the client's
    /// location-debug builder, not from the runtime graph builder itself.
    pub fn locations_at(locations: &Locations, place: &Guid) -> Vec<Guid> {
        let mut ids: Vec<Guid> = std::iter::once(*place)
            .chain(locations.ancestors(place).map(|l| l.guid))
            .collect();
        if let Some(system) = locations.system_of(place)
            && !ids.contains(&system.guid)
        {
            ids.push(system.guid);
        }
        ids
    }

    /// Where `element`'s distribution comes from at these locations: the
    /// location whose override applies, or the resource's default. Places
    /// with the same source for every element a rock holds give the same
    /// quality odds, so a caller can group a provider's places by it (the
    /// groups can be finer than the odds: the Keeger belt's 30 `Rockcracker`
    /// clusters are 30 override locations with one distribution). `None`
    /// when the element's resource has no quality.
    ///
    /// ```no_run
    /// # fn group(roll: &sc_mining::QualityRoll<'_>, ti: &sc_extract::Guid) {
    /// match roll.source(ti) {
    ///     Some(sc_mining::QualitySource::Override(at)) => println!("override at {at}"),
    ///     Some(sc_mining::QualitySource::Default) => println!("default odds"),
    ///     None => println!("no quality"),
    /// }
    /// # }
    /// ```
    pub fn source(&self, element: &Guid) -> Option<QualitySource> {
        let resource = self.resources.get(&self.elements.get(element)?.resource?)?;
        let wiring = resource.quality.as_ref()?;
        Some(match self.overriding(wiring.location_override.as_ref()) {
            Some((at, _)) => QualitySource::Override(at),
            None => QualitySource::Default,
        })
    }

    /// One element entry's quality, as the client rolls it at spawn (LIVE
    /// 4.10): `Q` from the resource's distribution (the first matching
    /// location override, else the default); a normal one is truncated to
    /// `[min - 0.5, max + 0.5]` by inverse CDF and rounded; then `q =
    /// clamp(int(Q * qualityScale), 0, 1000)`, mapped by the quantization band
    /// containing it (when its mapped value lies in the band). `None` when the
    /// element's resource has no quality.
    pub(crate) fn roll(&self, element: &Guid, quality_scale: f32, rng: &mut Rng) -> Option<u16> {
        let (shape, bands) = self.wiring(element)?;
        let q = sample(shape, rng)?;
        Some(finish(q, quality_scale, bands))
    }

    /// The exact odds of each quality the roll gives `element` in a part with
    /// this `quality_scale` (a [`Part`](crate::Part)'s): the same steps as
    /// the roll, with each `Q`'s probability taken from the truncated
    /// normal's CDF instead of a draw. Exact up to the client's own
    /// approximations of the normal CDF and its inverse, which do not
    /// quite invert each other (a cumulative odd differs from the draws' by
    /// under 0.002). `None`
    /// when the element's resource has no quality or its distribution is
    /// not a normal one.
    ///
    /// ```no_run
    /// # fn odds(roll: &sc_mining::QualityRoll<'_>, titanium: &sc_extract::Guid) {
    /// if let Some(odds) = roll.odds(titanium, 1.0) {
    ///     println!("Q700+ in {:.1} % of rolls", odds.at_least(700) * 100.0);
    /// }
    /// # }
    /// ```
    pub fn odds(&self, element: &Guid, quality_scale: f32) -> Option<QualityOdds> {
        let (shape, bands) = self.wiring(element)?;
        let mut steps: BTreeMap<u16, f64> = BTreeMap::new();
        for (q, p) in distribution(shape)? {
            *steps.entry(finish(q, quality_scale, bands)).or_default() += p;
        }
        Some(QualityOdds {
            steps: steps.into_iter().filter(|&(_, p)| p > 0.0).collect(),
        })
    }

    /// The element's distribution (location override applied) and
    /// quantization bands.
    fn wiring(
        &self,
        element: &Guid,
    ) -> Option<(&QualityDistributionShape, Option<&[QuantizationBand]>)> {
        let resource = self.resources.get(&self.elements.get(element)?.resource?)?;
        let wiring = resource.quality.as_ref()?;
        let shape = self.shape(
            wiring.location_override.as_ref(),
            wiring.distribution.as_ref(),
        )?;
        let bands = match &wiring.quantization {
            Some(QuantizationRef::Inline(bands)) => Some(bands.as_slice()),
            Some(QuantizationRef::Record(g)) => {
                self.catalog.quantization(g).map(|r| r.bands.as_slice())
            }
            _ => None,
        };
        Some((shape, bands))
    }

    fn shape<'b>(
        &'b self,
        location_override: Option<&'b LocationOverrideRef>,
        default: Option<&'b DistributionRef>,
    ) -> Option<&'b QualityDistributionShape> {
        let overridden = self.overriding(location_override).map(|(_, d)| d);
        match overridden.or(default)? {
            DistributionRef::Inline(shape) => Some(shape),
            DistributionRef::Record(g) => self.catalog.distribution(g)?.shape.as_ref(),
            DistributionRef::Other { .. } => None,
        }
    }
}

impl<'a> QualityRoll<'a> {
    /// The first of [`locations`](Self::locations) an override lists with a
    /// distribution, and that distribution.
    fn overriding<'b>(
        &self,
        location_override: Option<&'b LocationOverrideRef>,
    ) -> Option<(Guid, &'b DistributionRef)>
    where
        'a: 'b,
    {
        let entries = match location_override? {
            LocationOverrideRef::Inline(entries) => entries.as_slice(),
            LocationOverrideRef::Record(g) => self.catalog.location_override(g)?.entries.as_slice(),
            LocationOverrideRef::Other { .. } => return None,
        };
        self.locations.iter().find_map(|loc| {
            entries
                .iter()
                .find(|e| e.location == Some(*loc))
                .and_then(|e| e.distribution.as_ref())
                .map(|d| (*loc, d))
        })
    }
}

/// Where an element's quality distribution comes from (see
/// [`QualityRoll::source`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualitySource {
    /// The resource's own distribution.
    Default,
    /// The override listed for this location (a `StarMapObject` GUID).
    Override(Guid),
}

/// `Q` from a distribution. The client's `CraftingQualityDistributionUniform`
/// (`round(lerp(min - 0.5, max + 0.5, u))`) has no records in 4.10 and
/// arrives as `Other`.
fn sample(shape: &QualityDistributionShape, rng: &mut Rng) -> Option<i32> {
    match *shape {
        QualityDistributionShape::Normal {
            min,
            max,
            mean,
            stddev,
        } => {
            let (lo, hi) = (min as f64, max as f64);
            let (mu, sigma) = (f64::from(mean), f64::from(stddev).max(1e-9));
            let a = phi((lo - 0.5 - mu) / sigma);
            let b = phi((hi + 0.5 - mu) / sigma);
            let x = phi_inverse(a + f64::from(rng.unit()) * (b - a)) * sigma + mu;
            Some(x.clamp(lo, hi).round() as i32)
        }
        QualityDistributionShape::Other { .. } => None,
    }
}

/// What the client does with a drawn `Q`: scale it, truncate, clamp to
/// `0..=1000`, then quantize.
fn finish(q: i32, quality_scale: f32, bands: Option<&[QuantizationBand]>) -> u16 {
    let q = ((q as f32 * quality_scale) as i32).clamp(0, 1000);
    bands.map_or(q, |b| quantize(b, q)) as u16
}

/// Every `Q` [`sample`] can return with its probability. A draw lands on
/// `k` when the truncated normal falls in `[k - 0.5, k + 0.5)`; the
/// truncation bounds are `min - 0.5` and `max + 0.5`, so the steps sum to 1.
fn distribution(shape: &QualityDistributionShape) -> Option<Vec<(i32, f64)>> {
    match *shape {
        QualityDistributionShape::Normal {
            min,
            max,
            mean,
            stddev,
        } => {
            let (mu, sigma) = (f64::from(mean), f64::from(stddev).max(1e-9));
            let cdf = |x: f64| phi((x - mu) / sigma);
            let width = cdf(f64::from(max) + 0.5) - cdf(f64::from(min) - 0.5);
            if width <= 1e-12 {
                // All the mass is beyond one end; the draw clamps to it.
                let end = if mu < f64::from(min) { min } else { max };
                return Some(vec![(end, 1.0)]);
            }
            Some(
                (min..=max)
                    .map(|k| {
                        let k_f = f64::from(k);
                        (k, (cdf(k_f + 0.5) - cdf(k_f - 0.5)) / width)
                    })
                    .collect(),
            )
        }
        QualityDistributionShape::Other { .. } => None,
    }
}

/// The odds of each quality an element's roll gives (see
/// [`QualityRoll::odds`]).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityOdds {
    steps: Vec<(u16, f64)>,
}

impl QualityOdds {
    /// Each quality the roll can give with its probability, lowest first;
    /// the probabilities sum to 1.
    pub fn steps(&self) -> &[(u16, f64)] {
        &self.steps
    }

    /// The chance of a quality of `quality` or better.
    pub fn at_least(&self, quality: u16) -> f64 {
        self.steps
            .iter()
            .filter(|(q, _)| *q >= quality)
            // A fold from +0.0: an empty f64 sum is -0.0.
            .fold(0.0, |sum, (_, p)| sum + p)
            .min(1.0)
    }

    /// The expected quality.
    pub fn mean(&self) -> f64 {
        self.steps
            .iter()
            .fold(0.0, |sum, &(q, p)| sum + f64::from(q) * p)
    }
}

/// The first band containing `q` maps it, if the mapped value lies in that
/// band.
fn quantize(bands: &[QuantizationBand], q: i32) -> i32 {
    bands
        .iter()
        .find(|b| (b.start..=b.end).contains(&q))
        .filter(|b| (b.start..=b.end).contains(&b.mapped_value))
        .map_or(q, |b| b.mapped_value)
}

/// The normal CDF as the client approximates it (Abramowitz–Stegun 26.2.19).
fn phi(x: f64) -> f64 {
    if x < 0.0 {
        return 1.0 - phi(-x);
    }
    if x >= 5.25 {
        return 1.0;
    }
    let p = 1.0
        + x * (0.049_867_347
            + x * (0.021_141_006
                + x * (0.003_277_626_4 + x * (3.800_36e-5 + x * (4.889_06e-5 + x * 5.383e-6)))));
    1.0 - 0.5 / p.powi(16)
}

/// The inverse normal CDF as the client approximates it (Voutier 2010).
fn phi_inverse(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }
    if (0.025..=0.975).contains(&p) {
        let q = p - 0.5;
        let r = q * q;
        return q
            * (1.365_020_2
                + (0.151_015_5 - 0.530_357_24 * r) / ((r - 0.760_732_5) * r + 0.132_089_63));
    }
    if p > 0.975 {
        return -phi_inverse(1.0 - p);
    }
    let s = (1.0 / (p * p)).ln().sqrt();
    (4.120_411_4 * s + 16.682_322) / ((s + 8.759_693) * s + 7.173_787_6) + 0.029_814_187
        - 1.000_182_5 * s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phi_and_inverse() {
        assert!((phi(0.0) - 0.5).abs() < 1e-6);
        assert!((phi(1.96) - 0.975).abs() < 1e-3);
        for p in [0.01, 0.2, 0.5, 0.8, 0.99] {
            assert!((phi(phi_inverse(p)) - p).abs() < 2e-3, "{p}");
        }
    }

    #[test]
    fn normal_stays_in_range() {
        let shape = QualityDistributionShape::Normal {
            min: 300,
            max: 700,
            mean: 500.0,
            stddev: 150.0,
        };
        let mut rng = Rng::new(3);
        let qs: Vec<i32> = (0..2000).filter_map(|_| sample(&shape, &mut rng)).collect();
        assert!(qs.iter().all(|q| (300..=700).contains(q)));
        let mean = qs.iter().sum::<i32>() as f32 / qs.len() as f32;
        assert!((mean - 500.0).abs() < 10.0, "{mean}");
    }

    /// The exact steps against 2 000 000 draws through the client's inverse
    /// CDF: every cumulative odd within 0.002.
    #[test]
    fn distribution_matches_draws() {
        for shape in [
            QualityDistributionShape::Normal {
                min: 300,
                max: 700,
                mean: 500.0,
                stddev: 150.0,
            },
            QualityDistributionShape::Normal {
                min: 1,
                max: 1000,
                mean: 250.0,
                stddev: 120.0,
            },
        ] {
            let exact = distribution(&shape).unwrap();
            let total: f64 = exact.iter().map(|(_, p)| p).sum();
            assert!((total - 1.0).abs() < 1e-9, "{total}");
            let runs = 2_000_000;
            let mut rng = Rng::new(7);
            let mut drawn: BTreeMap<i32, u32> = BTreeMap::new();
            for _ in 0..runs {
                *drawn.entry(sample(&shape, &mut rng).unwrap()).or_default() += 1;
            }
            let (mut want, mut got) = (0.0, 0.0);
            for (k, p) in &exact {
                want += p;
                got += f64::from(drawn.get(k).copied().unwrap_or(0)) / f64::from(runs);
                assert!((want - got).abs() < 2e-3, "at {k}: {want} vs {got}");
            }
        }
    }

    #[test]
    fn odds_follow_scale_and_bands() {
        let shape = QualityDistributionShape::Normal {
            min: 400,
            max: 600,
            mean: 500.0,
            stddev: 50.0,
        };
        let bands = [QuantizationBand {
            start: 0,
            end: 499,
            mapped_value: 250,
        }];
        let mut steps: BTreeMap<u16, f64> = BTreeMap::new();
        for (q, p) in distribution(&shape).unwrap() {
            *steps.entry(finish(q, 2.0, Some(&bands))).or_default() += p;
        }
        // Scaled by 2 every Q lands at 800..=1000 (clamped); no band holds it.
        assert!(steps.keys().all(|q| (800..=1000).contains(q)));
        assert!(steps[&1000] > 0.49, "{}", steps[&1000]);
        let odds = QualityOdds {
            steps: steps.into_iter().collect(),
        };
        assert!((odds.at_least(0) - 1.0).abs() < 1e-9);
        assert!((odds.at_least(1000) - odds.steps().last().unwrap().1).abs() < 1e-12);
        // Below the band's end everything maps to 250.
        let mut low: BTreeMap<u16, f64> = BTreeMap::new();
        for (q, p) in distribution(&shape).unwrap() {
            *low.entry(finish(q, 0.5, Some(&bands))).or_default() += p;
        }
        assert_eq!(low.len(), 1);
        assert!((low[&250] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn mass_beyond_the_range_clamps_to_its_end() {
        let shape = QualityDistributionShape::Normal {
            min: 0,
            max: 100,
            mean: 900.0,
            stddev: 1.0,
        };
        assert_eq!(distribution(&shape).unwrap(), vec![(100, 1.0)]);
    }

    #[test]
    fn quantization_bands() {
        let bands = [
            QuantizationBand {
                start: 0,
                end: 99,
                mapped_value: 50,
            },
            QuantizationBand {
                start: 100,
                end: 199,
                mapped_value: 500,
            },
        ];
        assert_eq!(quantize(&bands, 10), 50);
        assert_eq!(quantize(&bands, 150), 150); // mapped value outside its band
        assert_eq!(quantize(&bands, 300), 300);
    }
}
