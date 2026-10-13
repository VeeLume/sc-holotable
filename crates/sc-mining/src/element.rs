//! Elements — the `MineableElement` records a rock's composition lists.

use std::collections::HashMap;

use sc_extract::{Datacore, Guid, RecordCollection};
use sc_resources::Resources;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::ruleset::density;

/// One `MineableElement`: what an element contributes to a rock's
/// properties. The same element can appear more than once in a rock (at
/// different qualities); each occurrence counts on its own.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub guid: Guid,
    /// `resourceType`: the `ResourceType` the element yields.
    pub resource: Option<Guid>,
    /// `elementInstability`.
    pub instability: f32,
    /// `elementResistance`, -1..1 (negative elements lower resistance).
    pub resistance: f32,
    /// `elementOptimalWindowMidpoint`, 0..1 of the charge bar.
    pub window_midpoint: f32,
    /// `elementOptimalWindowMidpointRandomness`: the midpoint is shifted by
    /// up to this, per rock, from a seeded random.
    pub window_midpoint_randomness: f32,
    /// `elementOptimalWindowThinness`: positive values narrow the window.
    pub window_thinness: f32,
    /// `elementExplosionMultiplier`.
    pub explosion_multiplier: f32,
    /// `elementClusterFactor`: how strongly fracturing concentrates the
    /// element into few child rocks.
    pub cluster_factor: f32,
    /// Density of [`resource`](Self::resource), g/cm³ (t/m³).
    pub density: Option<f32>,
}

/// Every `MineableElement`, keyed by GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Elements {
    by_guid: HashMap<Guid, Element>,
}

impl Elements {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read every `MineableElement`; densities join through [`Resources`].
    pub fn build(datacore: &Datacore, resources: &Resources) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut by_guid = HashMap::new();
        for (&guid, handle) in &store.records.multi_feature.mineable_element {
            let Some(e) = handle.get(pools) else {
                continue;
            };
            let element = Element {
                guid,
                resource: e.resource_type,
                instability: e.element_instability,
                resistance: e.element_resistance,
                window_midpoint: e.element_optimal_window_midpoint,
                window_midpoint_randomness: e.element_optimal_window_midpoint_randomness,
                window_thinness: e.element_optimal_window_thinness,
                explosion_multiplier: e.element_explosion_multiplier,
                cluster_factor: e.element_cluster_factor,
                density: e.resource_type.and_then(|g| density(resources, &g)),
            };
            debug!(%guid, ?element, "mineable element");
            by_guid.insert(guid, element);
        }
        Self { by_guid }
    }

    /// Add or replace one element (tests, hand-built scenarios).
    pub fn insert(&mut self, element: Element) {
        self.by_guid.insert(element.guid, element);
    }
}

impl RecordCollection for Elements {
    type Item = Element;

    fn get(&self, guid: &Guid) -> Option<&Element> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Element)> + '_ {
        self.by_guid.iter()
    }
}
