// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `NotificationDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDef {
    /// DCB field: `canvasOverride` (Reference)
    #[serde(default)]
    pub canvas_override: Option<CigGuid>,
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `body` (Locale)
    #[serde(default)]
    pub body: String,
    /// DCB field: `buttonAcceptText` (Locale)
    #[serde(default)]
    pub button_accept_text: String,
    /// DCB field: `buttonDeclineText` (Locale)
    #[serde(default)]
    pub button_decline_text: String,
    /// DCB field: `appLink` (Class)
    #[serde(default)]
    pub app_link: Option<Handle<SMobiGlasAppLink>>,
    /// DCB field: `notificationType` (EnumChoice)
    #[serde(default)]
    pub notification_type: String,
    /// DCB field: `audioEvent` (String)
    #[serde(default)]
    pub audio_event: String,
    /// DCB field: `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// DCB field: `displayValue` (Int32)
    #[serde(default)]
    pub display_value: i32,
    /// DCB field: `screenTime` (Single)
    #[serde(default)]
    pub screen_time: f32,
    /// DCB field: `buttonAcceptDisplay` (Boolean)
    #[serde(default)]
    pub button_accept_display: bool,
    /// DCB field: `buttonDeclineDisplay` (Boolean)
    #[serde(default)]
    pub button_decline_display: bool,
    /// DCB field: `combineDisplayValue` (Boolean)
    #[serde(default)]
    pub combine_display_value: bool,
    /// DCB field: `displayAcceptDeclineControlHint` (Boolean)
    #[serde(default)]
    pub display_accept_decline_control_hint: bool,
    /// DCB field: `hideIcon` (Boolean)
    #[serde(default)]
    pub hide_icon: bool,
    /// DCB field: `isHostility` (Boolean)
    #[serde(default)]
    pub is_hostility: bool,
    /// DCB field: `isLowPriorityNotification` (Boolean)
    #[serde(default)]
    pub is_low_priority_notification: bool,
    /// DCB field: `isObjective` (Boolean)
    #[serde(default)]
    pub is_objective: bool,
    /// DCB field: `isTutorial` (Boolean)
    #[serde(default)]
    pub is_tutorial: bool,
    /// DCB field: `removeThisUniqueID` (Boolean)
    #[serde(default)]
    pub remove_this_unique_id: bool,
    /// DCB field: `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// DCB field: `suppressible` (Boolean)
    #[serde(default)]
    pub suppressible: bool,
}

impl Pooled for NotificationDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_no.notification_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_no.notification_def }
}

impl<'a> Extract<'a> for NotificationDef {
    const TYPE_NAME: &'static str = "NotificationDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            canvas_override: inst.get("canvasOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            body: inst.get_str("body").map(String::from).unwrap_or_default(),
            button_accept_text: inst.get_str("buttonAcceptText").map(String::from).unwrap_or_default(),
            button_decline_text: inst.get_str("buttonDeclineText").map(String::from).unwrap_or_default(),
            app_link: match inst.get("appLink") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMobiGlasAppLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMobiGlasAppLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notification_type: inst.get_str("notificationType").map(String::from).unwrap_or_default(),
            audio_event: inst.get_str("audioEvent").map(String::from).unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            display_value: inst.get_i32("displayValue").unwrap_or_default(),
            screen_time: inst.get_f32("screenTime").unwrap_or_default(),
            button_accept_display: inst.get_bool("buttonAcceptDisplay").unwrap_or_default(),
            button_decline_display: inst.get_bool("buttonDeclineDisplay").unwrap_or_default(),
            combine_display_value: inst.get_bool("combineDisplayValue").unwrap_or_default(),
            display_accept_decline_control_hint: inst.get_bool("displayAcceptDeclineControlHint").unwrap_or_default(),
            hide_icon: inst.get_bool("hideIcon").unwrap_or_default(),
            is_hostility: inst.get_bool("isHostility").unwrap_or_default(),
            is_low_priority_notification: inst.get_bool("isLowPriorityNotification").unwrap_or_default(),
            is_objective: inst.get_bool("isObjective").unwrap_or_default(),
            is_tutorial: inst.get_bool("isTutorial").unwrap_or_default(),
            remove_this_unique_id: inst.get_bool("removeThisUniqueID").unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            suppressible: inst.get_bool("suppressible").unwrap_or_default(),
        }
    }
}

