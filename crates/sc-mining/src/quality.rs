//! The quality an element of a freshly spawned rock gets.

use sc_extract::{Guid, RecordCollection};
use sc_resources::{
    DistributionRef, LocationOverrideRef, Quality, QualityDistributionShape, QuantizationBand,
    QuantizationRef, Resources,
};

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
    /// One element entry's quality, as the client rolls it at spawn (LIVE
    /// 4.10): `Q` from the resource's distribution (the first matching
    /// location override, else the default); a normal one is truncated to
    /// `[min - 0.5, max + 0.5]` by inverse CDF and rounded; then `q =
    /// clamp(int(Q * qualityScale), 0, 1000)`, mapped by the quantization band
    /// containing it (when its mapped value lies in the band). `None` when the
    /// element's resource has no quality.
    pub(crate) fn roll(&self, element: &Guid, quality_scale: f32, rng: &mut Rng) -> Option<u16> {
        let resource = self.resources.get(&self.elements.get(element)?.resource?)?;
        let wiring = resource.quality.as_ref()?;
        let shape = self.shape(
            wiring.location_override.as_ref(),
            wiring.distribution.as_ref(),
        )?;
        let q = sample(shape, rng)?;
        let q = ((q as f32 * quality_scale) as i32).clamp(0, 1000);
        let q = match &wiring.quantization {
            Some(QuantizationRef::Inline(bands)) => quantize(bands, q),
            Some(QuantizationRef::Record(g)) => self
                .catalog
                .quantization(g)
                .map_or(q, |r| quantize(&r.bands, q)),
            _ => q,
        };
        Some(q as u16)
    }

    fn shape<'b>(
        &'b self,
        location_override: Option<&'b LocationOverrideRef>,
        default: Option<&'b DistributionRef>,
    ) -> Option<&'b QualityDistributionShape> {
        let entries = match location_override {
            Some(LocationOverrideRef::Inline(entries)) => Some(entries.as_slice()),
            Some(LocationOverrideRef::Record(g)) => self
                .catalog
                .location_override(g)
                .map(|r| r.entries.as_slice()),
            _ => None,
        };
        let overridden = entries.and_then(|entries| {
            self.locations.iter().find_map(|loc| {
                entries
                    .iter()
                    .find(|e| e.location == Some(*loc))
                    .and_then(|e| e.distribution.as_ref())
            })
        });
        match overridden.or(default)? {
            DistributionRef::Inline(shape) => Some(shape),
            DistributionRef::Record(g) => self.catalog.distribution(g)?.shape.as_ref(),
            DistributionRef::Other { .. } => None,
        }
    }
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
