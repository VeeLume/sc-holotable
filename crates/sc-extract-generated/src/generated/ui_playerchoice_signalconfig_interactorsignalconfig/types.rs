// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-playerchoice_signalconfig_interactorsignalconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PlayerChoice_InteractionModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_InteractionModifier {
    /// `highlightFactor` (Single)
    #[serde(default)]
    pub highlight_factor: f32,
    /// `rangeFactor` (Single)
    #[serde(default)]
    pub range_factor: f32,
}

impl Pooled for PlayerChoice_InteractionModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_signalconfig_interactorsignalconfig.player_choice_interaction_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_signalconfig_interactorsignalconfig.player_choice_interaction_modifier }
}

impl<'a> Extract<'a> for PlayerChoice_InteractionModifier {
    const TYPE_NAME: &'static str = "PlayerChoice_InteractionModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            highlight_factor: inst.get_f32("highlightFactor").unwrap_or_default(),
            range_factor: inst.get_f32("rangeFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_SignalConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_SignalConfig {
    /// `zoneQueryFrequency` (Single)
    #[serde(default)]
    pub zone_query_frequency: f32,
    /// `distantIndicatorZoneQueryFrequency` (Single)
    #[serde(default)]
    pub distant_indicator_zone_query_frequency: f32,
    /// `baseColor` (Class)
    #[serde(default)]
    pub base_color: Option<Handle<RGBA>>,
    /// `baseEffectWidth` (Single)
    #[serde(default)]
    pub base_effect_width: f32,
    /// `viewMaxDistance` (Single)
    #[serde(default)]
    pub view_max_distance: f32,
    /// `viewMaxDistanceIM` (Single)
    #[serde(default)]
    pub view_max_distance_im: f32,
    /// `viewMaxDistantIndicatorDist` (Single)
    #[serde(default)]
    pub view_max_distant_indicator_dist: f32,
    /// `viewFadeDistance` (Single)
    #[serde(default)]
    pub view_fade_distance: f32,
    /// `viewFadeDistanceIM` (Single)
    #[serde(default)]
    pub view_fade_distance_im: f32,
    /// `maxVisibilityTraces` (Byte)
    #[serde(default)]
    pub max_visibility_traces: u32,
    /// `minVisibilityWait` (Byte)
    #[serde(default)]
    pub min_visibility_wait: u32,
    /// `useTwoColors` (Boolean)
    #[serde(default)]
    pub use_two_colors: bool,
    /// `secondaryColor` (Class)
    #[serde(default)]
    pub secondary_color: Option<Handle<RGBA>>,
    /// `secondaryEffectWidth` (Single)
    #[serde(default)]
    pub secondary_effect_width: f32,
    /// `maxDistanceFromCursorHighlight` (Single)
    #[serde(default)]
    pub max_distance_from_cursor_highlight: f32,
    /// `maxDistanceFromCursorHighlightIM` (Single)
    #[serde(default)]
    pub max_distance_from_cursor_highlight_im: f32,
    /// `showSignalForInspectedItems` (Boolean)
    #[serde(default)]
    pub show_signal_for_inspected_items: bool,
    /// `useHipDistance` (Boolean)
    #[serde(default)]
    pub use_hip_distance: bool,
    /// `interactionModifiers` (Class)
    #[serde(default)]
    pub interaction_modifiers: Option<Handle<PlayerChoice_InteractionModifier>>,
    /// `dashboardSwitchHighlightRadius` (Single)
    #[serde(default)]
    pub dashboard_switch_highlight_radius: f32,
    /// `dashboardSwitchHighlightCentreSize` (Single)
    #[serde(default)]
    pub dashboard_switch_highlight_centre_size: f32,
    /// `dashboardSwitchItemType` (EnumChoice)
    #[serde(default)]
    pub dashboard_switch_item_type: String,
}

impl Pooled for PlayerChoice_SignalConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_signalconfig_interactorsignalconfig.player_choice_signal_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_signalconfig_interactorsignalconfig.player_choice_signal_config }
}

impl<'a> Extract<'a> for PlayerChoice_SignalConfig {
    const TYPE_NAME: &'static str = "PlayerChoice_SignalConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zone_query_frequency: inst.get_f32("zoneQueryFrequency").unwrap_or_default(),
            distant_indicator_zone_query_frequency: inst.get_f32("distantIndicatorZoneQueryFrequency").unwrap_or_default(),
            base_color: match inst.get("baseColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            base_effect_width: inst.get_f32("baseEffectWidth").unwrap_or_default(),
            view_max_distance: inst.get_f32("viewMaxDistance").unwrap_or_default(),
            view_max_distance_im: inst.get_f32("viewMaxDistanceIM").unwrap_or_default(),
            view_max_distant_indicator_dist: inst.get_f32("viewMaxDistantIndicatorDist").unwrap_or_default(),
            view_fade_distance: inst.get_f32("viewFadeDistance").unwrap_or_default(),
            view_fade_distance_im: inst.get_f32("viewFadeDistanceIM").unwrap_or_default(),
            max_visibility_traces: inst.get_u32("maxVisibilityTraces").unwrap_or_default(),
            min_visibility_wait: inst.get_u32("minVisibilityWait").unwrap_or_default(),
            use_two_colors: inst.get_bool("useTwoColors").unwrap_or_default(),
            secondary_color: match inst.get("secondaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            secondary_effect_width: inst.get_f32("secondaryEffectWidth").unwrap_or_default(),
            max_distance_from_cursor_highlight: inst.get_f32("maxDistanceFromCursorHighlight").unwrap_or_default(),
            max_distance_from_cursor_highlight_im: inst.get_f32("maxDistanceFromCursorHighlightIM").unwrap_or_default(),
            show_signal_for_inspected_items: inst.get_bool("showSignalForInspectedItems").unwrap_or_default(),
            use_hip_distance: inst.get_bool("useHipDistance").unwrap_or_default(),
            interaction_modifiers: match inst.get("interactionModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerChoice_InteractionModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerChoice_InteractionModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dashboard_switch_highlight_radius: inst.get_f32("dashboardSwitchHighlightRadius").unwrap_or_default(),
            dashboard_switch_highlight_centre_size: inst.get_f32("dashboardSwitchHighlightCentreSize").unwrap_or_default(),
            dashboard_switch_item_type: inst.get_str("dashboardSwitchItemType").map(String::from).unwrap_or_default(),
        }
    }
}

