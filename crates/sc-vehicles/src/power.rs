//! Power plant output and an item's power ratio, as the client computes
//! them (LIVE 4.10.197).

use crate::{PowerRange, ResourceState, Signature};

/// What one of `online` power plants makes when they share the network:
/// with more than one online, each makes `round(units / online)` (halves
/// away from zero) plus `(online − 1) × modifier`, where `modifier` is
/// [`NetworkParams::plant_modifier`](crate::NetworkParams::plant_modifier).
///
/// Client: `FUN_1428b3fe0` (the plant's Power generation). It applies only
/// to whole segments; crafting quality (`GPP_ItemResource_PowerGeneration`)
/// scales `units` first. Two Sedulity plants (19) make 12 each, 24 in all,
/// as spviewer shows for the Constellation Mk V Centaurus.
pub fn plant_output(units: i32, online: i32, modifier: i32) -> i32 {
    if online <= 1 {
        return units;
    }
    let share = units as f32 / online as f32;
    share.round() as i32 + (online - 1) * modifier
}

/// The ratio an item's signatures scale with, from `ratio` (supplied over
/// requested for a consumer; for a power plant, the load on it over its
/// output, which exceeds 1 when the ship asks for more than the plants
/// make) and the state's `units` (segments requested or generated).
///
/// Client: `CItemResourceComponent::UpdatePowerRatio` (`FUN_1429a5950`).
/// The ratio is multiplied by the `modifier` of the last registered
/// [`PowerRange`] whose `start` is at most `round(units × ratio)` (halves
/// away from zero); 1 when none is.
pub fn signature_ratio(ranges: &[PowerRange], units: i32, ratio: f32) -> f32 {
    let segments = (units as f32 * ratio).round() as i32;
    let modifier = ranges
        .iter()
        .rev()
        .find(|r| r.registered && r.start <= segments)
        .map_or(1.0, |r| r.modifier);
    ratio * modifier
}

/// The EM signature the item settles at: the state's nominal EM times
/// [`signature_ratio`] (the live value decays toward it at the entry's
/// `decay_rate`).
pub fn em_signature(state: &ResourceState, units: i32, ratio: f32) -> f32 {
    state.em.map_or(0.0, |Signature { nominal, .. }| {
        nominal * signature_ratio(&state.power_ranges, units, ratio)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plant_split() {
        // Centaurus: 2 × Sedulity (19, size 2 → modifier 2).
        assert_eq!(plant_output(19, 2, 2), 12);
        assert_eq!(plant_output(19, 1, 2), 19);
        // Halves round away from zero: 21 / 2 = 10.5 → 11.
        assert_eq!(plant_output(21, 2, 2), 13);
        assert_eq!(plant_output(20, 3, 1), 9);
    }

    #[test]
    fn range_modifier() {
        let ranges = [
            PowerRange {
                start: 0,
                modifier: 0.7,
                registered: false,
            },
            PowerRange {
                start: 1,
                modifier: 0.85,
                registered: true,
            },
            PowerRange {
                start: 4,
                modifier: 1.0,
                registered: true,
            },
        ];
        // Snowfall cooler, 4 segments: full power sits in the top band.
        assert_eq!(signature_ratio(&ranges, 4, 1.0), 1.0);
        // 3 of 4 → 3 segments, the 0.85 band.
        assert!((signature_ratio(&ranges, 4, 0.75) - 0.6375).abs() < 1e-6);
        // Below every registered band: unmodified.
        assert_eq!(signature_ratio(&ranges, 4, 0.0), 0.0);
        assert_eq!(signature_ratio(&[], 4, 0.5), 0.5);
    }
}
