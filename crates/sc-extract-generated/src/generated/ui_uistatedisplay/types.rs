// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-uistatedisplay`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `UIStateDisplay_Threshold`
pub struct UIStateDisplay_Threshold {
    /// `displayName` (Locale)
    pub display_name: LocaleKey,
    /// `timelineLabel` (String)
    pub timeline_label: String,
    /// `minThresholdValue` (Single)
    pub min_threshold_value: f32,
    /// `stateColor` (EnumChoice)
    pub state_color: HUDPalleteEntry,
}

impl Pooled for UIStateDisplay_Threshold {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uistatedisplay.uistate_display_threshold }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uistatedisplay.uistate_display_threshold }
}

impl<'a> Extract<'a> for UIStateDisplay_Threshold {
    const TYPE_NAME: &'static str = "UIStateDisplay_Threshold";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(LocaleKey::from).unwrap_or_default(),
            timeline_label: inst.get_str("timelineLabel").map(String::from).unwrap_or_default(),
            min_threshold_value: inst.get_f32("minThresholdValue").unwrap_or_default(),
            state_color: HUDPalleteEntry::from_dcb_str(inst.get_str("stateColor").unwrap_or("")),
        }
    }
}

/// DCB type: `UIStateDisplay`
pub struct UIStateDisplay {
    /// `thresholds` (Class (array))
    pub thresholds: Vec<Handle<UIStateDisplay_Threshold>>,
}

impl Pooled for UIStateDisplay {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uistatedisplay.uistate_display }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uistatedisplay.uistate_display }
}

impl<'a> Extract<'a> for UIStateDisplay {
    const TYPE_NAME: &'static str = "UIStateDisplay";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            thresholds: inst.get_array("thresholds")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIStateDisplay_Threshold>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<UIStateDisplay_Threshold>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

