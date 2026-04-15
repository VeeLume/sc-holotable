// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-others`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `NavSplineAnchorPointComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavSplineAnchorPointComponentParams {
}

impl Pooled for NavSplineAnchorPointComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.nav_spline_anchor_point_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.nav_spline_anchor_point_component_params }
}

impl<'a> Extract<'a> for NavSplineAnchorPointComponentParams {
    const TYPE_NAME: &'static str = "NavSplineAnchorPointComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GroupEntityParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEntityParams {
    /// `syncToClients` (Boolean)
    #[serde(default)]
    pub sync_to_clients: bool,
}

impl Pooled for GroupEntityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.group_entity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.group_entity_params }
}

impl<'a> Extract<'a> for GroupEntityParams {
    const TYPE_NAME: &'static str = "GroupEntityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sync_to_clients: inst.get_bool("syncToClients").unwrap_or_default(),
        }
    }
}

/// DCB type: `LoudspeakerComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoudspeakerComponentParams {
    /// `minRadius` (Single)
    #[serde(default)]
    pub min_radius: f32,
    /// `maxRadius` (Single)
    #[serde(default)]
    pub max_radius: f32,
}

impl Pooled for LoudspeakerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.loudspeaker_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.loudspeaker_component_params }
}

impl<'a> Extract<'a> for LoudspeakerComponentParams {
    const TYPE_NAME: &'static str = "LoudspeakerComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_radius: inst.get_f32("minRadius").unwrap_or_default(),
            max_radius: inst.get_f32("maxRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `RestrictedAreaSplineParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedAreaSplineParams {
    /// `captureRadius` (Single)
    #[serde(default)]
    pub capture_radius: f32,
    /// `autopilotMessage` (Locale)
    #[serde(default)]
    pub autopilot_message: LocaleKey,
}

impl Pooled for RestrictedAreaSplineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.restricted_area_spline_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.restricted_area_spline_params }
}

impl<'a> Extract<'a> for RestrictedAreaSplineParams {
    const TYPE_NAME: &'static str = "RestrictedAreaSplineParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            capture_radius: inst.get_f32("captureRadius").unwrap_or_default(),
            autopilot_message: inst.get_str("autopilotMessage").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingSplineVisualParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingSplineVisualParams {
    /// `distanceBetweenNodes` (Single)
    #[serde(default)]
    pub distance_between_nodes: f32,
    /// `borderDimensions` (Class)
    #[serde(default)]
    pub border_dimensions: Option<Handle<Vec2>>,
    /// `distanceMinimum` (Single)
    #[serde(default)]
    pub distance_minimum: f32,
    /// `distanceMaximum` (Single)
    #[serde(default)]
    pub distance_maximum: f32,
    /// `scaleMaximum` (Single)
    #[serde(default)]
    pub scale_maximum: f32,
    /// `markerDistanceFromEnds` (Single)
    #[serde(default)]
    pub marker_distance_from_ends: f32,
    /// `markerRadius` (Single)
    #[serde(default)]
    pub marker_radius: f32,
    /// `markerMoveRadius` (Single)
    #[serde(default)]
    pub marker_move_radius: f32,
}

impl Pooled for LandingSplineVisualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.landing_spline_visual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.landing_spline_visual_params }
}

impl<'a> Extract<'a> for LandingSplineVisualParams {
    const TYPE_NAME: &'static str = "LandingSplineVisualParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distance_between_nodes: inst.get_f32("distanceBetweenNodes").unwrap_or_default(),
            border_dimensions: match inst.get("borderDimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distance_minimum: inst.get_f32("distanceMinimum").unwrap_or_default(),
            distance_maximum: inst.get_f32("distanceMaximum").unwrap_or_default(),
            scale_maximum: inst.get_f32("scaleMaximum").unwrap_or_default(),
            marker_distance_from_ends: inst.get_f32("markerDistanceFromEnds").unwrap_or_default(),
            marker_radius: inst.get_f32("markerRadius").unwrap_or_default(),
            marker_move_radius: inst.get_f32("markerMoveRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityComponentWaterDisturbance_NoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentWaterDisturbance_NoiseParams {
    /// `lacunarity` (Single)
    #[serde(default)]
    pub lacunarity: f32,
    /// `persistence` (Single)
    #[serde(default)]
    pub persistence: f32,
    /// `amplitude` (Class)
    #[serde(default)]
    pub amplitude: Option<Handle<Vec2>>,
}

impl Pooled for EntityComponentWaterDisturbance_NoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.entity_component_water_disturbance_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.entity_component_water_disturbance_noise_params }
}

impl<'a> Extract<'a> for EntityComponentWaterDisturbance_NoiseParams {
    const TYPE_NAME: &'static str = "EntityComponentWaterDisturbance_NoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lacunarity: inst.get_f32("lacunarity").unwrap_or_default(),
            persistence: inst.get_f32("persistence").unwrap_or_default(),
            amplitude: match inst.get("amplitude") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityComponentWaterDisturbanceParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentWaterDisturbanceParams {
    /// `style` (EnumChoice)
    #[serde(default)]
    pub style: DisturbanceStyle,
    /// `distributionArea` (Class)
    #[serde(default)]
    pub distribution_area: Option<Handle<Vec2>>,
    /// `hitSize` (Single)
    #[serde(default)]
    pub hit_size: f32,
    /// `depth` (Single)
    #[serde(default)]
    pub depth: f32,
    /// `pressure` (Single)
    #[serde(default)]
    pub pressure: f32,
    /// `foamAmount` (Single)
    #[serde(default)]
    pub foam_amount: f32,
    /// `frequency` (Class)
    #[serde(default)]
    pub frequency: Option<Handle<Vec2>>,
    /// `noiseParams` (Class)
    #[serde(default)]
    pub noise_params: Option<Handle<EntityComponentWaterDisturbance_NoiseParams>>,
}

impl Pooled for EntityComponentWaterDisturbanceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.entity_component_water_disturbance_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.entity_component_water_disturbance_params }
}

impl<'a> Extract<'a> for EntityComponentWaterDisturbanceParams {
    const TYPE_NAME: &'static str = "EntityComponentWaterDisturbanceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            style: DisturbanceStyle::from_dcb_str(inst.get_str("style").unwrap_or("")),
            distribution_area: match inst.get("distributionArea") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hit_size: inst.get_f32("hitSize").unwrap_or_default(),
            depth: inst.get_f32("depth").unwrap_or_default(),
            pressure: inst.get_f32("pressure").unwrap_or_default(),
            foam_amount: inst.get_f32("foamAmount").unwrap_or_default(),
            frequency: match inst.get("frequency") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            noise_params: match inst.get("noiseParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityComponentWaterDisturbance_NoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityComponentWaterImpactTestParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentWaterImpactTestParams {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: TestType,
    /// `distributionArea` (Class)
    #[serde(default)]
    pub distribution_area: Option<Handle<Vec2>>,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
}

impl Pooled for EntityComponentWaterImpactTestParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.entity_component_water_impact_test_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.entity_component_water_impact_test_params }
}

impl<'a> Extract<'a> for EntityComponentWaterImpactTestParams {
    const TYPE_NAME: &'static str = "EntityComponentWaterImpactTestParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: TestType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            distribution_area: match inst.get("distributionArea") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitNavSplineDataParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitNavSplineDataParams {
}

impl Pooled for TransitNavSplineDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_others.transit_nav_spline_data_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_others.transit_nav_spline_data_params }
}

impl<'a> Extract<'a> for TransitNavSplineDataParams {
    const TYPE_NAME: &'static str = "TransitNavSplineDataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

