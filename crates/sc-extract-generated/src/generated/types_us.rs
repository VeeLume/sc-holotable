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

/// DCB type: `UserRTPC`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRTPC {
    /// DCB field: `userRTPCName` (String)
    #[serde(default)]
    pub user_rtpcname: String,
    /// DCB field: `userRTPCValue` (Single)
    #[serde(default)]
    pub user_rtpcvalue: f32,
    /// DCB field: `userRTPCResetValue` (Single)
    #[serde(default)]
    pub user_rtpcreset_value: f32,
}

impl Pooled for UserRTPC {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_us.user_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_us.user_rtpc }
}

impl<'a> Extract<'a> for UserRTPC {
    const TYPE_NAME: &'static str = "UserRTPC";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            user_rtpcname: inst.get_str("userRTPCName").map(String::from).unwrap_or_default(),
            user_rtpcvalue: inst.get_f32("userRTPCValue").unwrap_or_default(),
            user_rtpcreset_value: inst.get_f32("userRTPCResetValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `UsableArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsableArchetype {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `useChannels` (Reference (array))
    #[serde(default)]
    pub use_channels: Vec<CigGuid>,
}

impl Pooled for UsableArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_us.usable_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_us.usable_archetype }
}

impl<'a> Extract<'a> for UsableArchetype {
    const TYPE_NAME: &'static str = "UsableArchetype";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            use_channels: inst.get_array("useChannels")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UseChannelArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseChannelArchetype {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
    /// DCB field: `onItemPortOccupiedFragmentTag` (String)
    #[serde(default)]
    pub on_item_port_occupied_fragment_tag: String,
}

impl Pooled for UseChannelArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_us.use_channel_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_us.use_channel_archetype }
}

impl<'a> Extract<'a> for UseChannelArchetype {
    const TYPE_NAME: &'static str = "UseChannelArchetype";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
            on_item_port_occupied_fragment_tag: inst.get_str("onItemPortOccupiedFragmentTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `UsableArchetypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsableArchetypes {
    /// DCB field: `archetypes` (Reference (array))
    #[serde(default)]
    pub archetypes: Vec<CigGuid>,
}

impl Pooled for UsableArchetypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_us.usable_archetypes }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_us.usable_archetypes }
}

impl<'a> Extract<'a> for UsableArchetypes {
    const TYPE_NAME: &'static str = "UsableArchetypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            archetypes: inst.get_array("archetypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

