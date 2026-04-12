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

/// DCB type: `TacticScoringProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticScoringProfile {
    /// DCB field: `tactics` (StrongPointer (array))
    #[serde(default)]
    pub tactics: Vec<Handle<CommonTacticScores>>,
}

impl Pooled for TacticScoringProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tactic_scoring_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tactic_scoring_profile }
}

impl<'a> Extract<'a> for TacticScoringProfile {
    const TYPE_NAME: &'static str = "TacticScoringProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tactics: inst.get_array("tactics")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommonTacticScores>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommonTacticScores>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TacticPlayerDistance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticPlayerDistance {
    /// DCB field: `shortDistanceTreshold` (Single)
    #[serde(default)]
    pub short_distance_treshold: f32,
    /// DCB field: `mediumDistanceTreshhold` (Single)
    #[serde(default)]
    pub medium_distance_treshhold: f32,
}

impl Pooled for TacticPlayerDistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tactic_player_distance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tactic_player_distance }
}

impl<'a> Extract<'a> for TacticPlayerDistance {
    const TYPE_NAME: &'static str = "TacticPlayerDistance";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            short_distance_treshold: inst.get_f32("shortDistanceTreshold").unwrap_or_default(),
            medium_distance_treshhold: inst.get_f32("mediumDistanceTreshhold").unwrap_or_default(),
        }
    }
}

/// DCB type: `TagTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagTrigger {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `startTrigger` (Class)
    #[serde(default)]
    pub start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `stopTrigger` (Class)
    #[serde(default)]
    pub stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `boneName` (String)
    #[serde(default)]
    pub bone_name: String,
}

impl Pooled for TagTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tag_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tag_trigger }
}

impl<'a> Extract<'a> for TagTrigger {
    const TYPE_NAME: &'static str = "TagTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            start_trigger: match inst.get("startTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_trigger: match inst.get("stopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bone_name: inst.get_str("boneName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `TargetTrackingAutoZoomDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetTrackingAutoZoomDef {
    /// DCB field: `zoomByDistance` (Class)
    #[serde(default)]
    pub zoom_by_distance: Option<Handle<BezierCurve>>,
    /// DCB field: `referenceDistance` (Single)
    #[serde(default)]
    pub reference_distance: f32,
    /// DCB field: `zoomAngleMin` (Single)
    #[serde(default)]
    pub zoom_angle_min: f32,
    /// DCB field: `zoomAngleMax` (Single)
    #[serde(default)]
    pub zoom_angle_max: f32,
    /// DCB field: `zoomLerpSpeedIn` (Single)
    #[serde(default)]
    pub zoom_lerp_speed_in: f32,
    /// DCB field: `zoomLerpSpeedOut` (Single)
    #[serde(default)]
    pub zoom_lerp_speed_out: f32,
}

impl Pooled for TargetTrackingAutoZoomDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.target_tracking_auto_zoom_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.target_tracking_auto_zoom_def }
}

impl<'a> Extract<'a> for TargetTrackingAutoZoomDef {
    const TYPE_NAME: &'static str = "TargetTrackingAutoZoomDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zoom_by_distance: match inst.get("zoomByDistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reference_distance: inst.get_f32("referenceDistance").unwrap_or_default(),
            zoom_angle_min: inst.get_f32("zoomAngleMin").unwrap_or_default(),
            zoom_angle_max: inst.get_f32("zoomAngleMax").unwrap_or_default(),
            zoom_lerp_speed_in: inst.get_f32("zoomLerpSpeedIn").unwrap_or_default(),
            zoom_lerp_speed_out: inst.get_f32("zoomLerpSpeedOut").unwrap_or_default(),
        }
    }
}

/// DCB type: `TaggedSubHarvestableConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedSubHarvestableConfig {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `tagList` (StrongPointer)
    #[serde(default)]
    pub tag_list: Option<Handle<HarvestableTagListBase>>,
    /// DCB field: `subConfig` (StrongPointer)
    #[serde(default)]
    pub sub_config: Option<Handle<SubHarvestableConfigSingleBase>>,
}

impl Pooled for TaggedSubHarvestableConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tagged_sub_harvestable_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tagged_sub_harvestable_config }
}

impl<'a> Extract<'a> for TaggedSubHarvestableConfig {
    const TYPE_NAME: &'static str = "TaggedSubHarvestableConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tag_list: match inst.get("tagList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HarvestableTagListBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HarvestableTagListBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sub_config: match inst.get("subConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SubHarvestableConfigSingleBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SubHarvestableConfigSingleBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TagToAudioRtpcList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagToAudioRtpcList {
    /// DCB field: `TagsToRtpcs` (Class (array))
    #[serde(default)]
    pub tags_to_rtpcs: Vec<Handle<TagToAudioRtpc>>,
}

impl Pooled for TagToAudioRtpcList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tag_to_audio_rtpc_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tag_to_audio_rtpc_list }
}

impl<'a> Extract<'a> for TagToAudioRtpcList {
    const TYPE_NAME: &'static str = "TagToAudioRtpcList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags_to_rtpcs: inst.get_array("TagsToRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagToAudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagToAudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagToAudioRtpc`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagToAudioRtpc {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `rtpcName` (String)
    #[serde(default)]
    pub rtpc_name: String,
    /// DCB field: `rtpcValue` (Single)
    #[serde(default)]
    pub rtpc_value: f32,
}

impl Pooled for TagToAudioRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tag_to_audio_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tag_to_audio_rtpc }
}

impl<'a> Extract<'a> for TagToAudioRtpc {
    const TYPE_NAME: &'static str = "TagToAudioRtpc";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rtpc_name: inst.get_str("rtpcName").map(String::from).unwrap_or_default(),
            rtpc_value: inst.get_f32("rtpcValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `TagSearchTerm`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSearchTerm {
    /// DCB field: `positiveTags` (Reference (array))
    #[serde(default)]
    pub positive_tags: Vec<CigGuid>,
    /// DCB field: `negativeTags` (Reference (array))
    #[serde(default)]
    pub negative_tags: Vec<CigGuid>,
}

impl Pooled for TagSearchTerm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tag_search_term }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tag_search_term }
}

impl<'a> Extract<'a> for TagSearchTerm {
    const TYPE_NAME: &'static str = "TagSearchTerm";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            positive_tags: inst.get_array("positiveTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            negative_tags: inst.get_array("negativeTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TacticalQuery`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalQuery {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `options` (StrongPointer (array))
    #[serde(default)]
    pub options: Vec<Handle<TQSOption>>,
}

impl Pooled for TacticalQuery {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tactical_query }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tactical_query }
}

impl<'a> Extract<'a> for TacticalQuery {
    const TYPE_NAME: &'static str = "TacticalQuery";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            options: inst.get_array("options")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSOption>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TQSOption>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// DCB field: `legacyGUID` (UInt32)
    #[serde(default)]
    pub legacy_guid: u32,
    /// DCB field: `tagName` (String)
    #[serde(default)]
    pub tag_name: String,
    /// DCB field: `children` (Reference (array))
    #[serde(default)]
    pub children: Vec<CigGuid>,
}

impl Pooled for Tag {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tag }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tag }
}

impl<'a> Extract<'a> for Tag {
    const TYPE_NAME: &'static str = "Tag";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            legacy_guid: inst.get_u32("legacyGUID").unwrap_or_default(),
            tag_name: inst.get_str("tagName").map(String::from).unwrap_or_default(),
            children: inst.get_array("children")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDatabase {
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for TagDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tag_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tag_database }
}

impl<'a> Extract<'a> for TagDatabase {
    const TYPE_NAME: &'static str = "TagDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagsDNFTerm`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsDNFTerm {
    /// DCB field: `positiveTags` (Reference (array))
    #[serde(default)]
    pub positive_tags: Vec<CigGuid>,
    /// DCB field: `negativeTags` (Reference (array))
    #[serde(default)]
    pub negative_tags: Vec<CigGuid>,
}

impl Pooled for TagsDNFTerm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tags_dnfterm }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tags_dnfterm }
}

impl<'a> Extract<'a> for TagsDNFTerm {
    const TYPE_NAME: &'static str = "TagsDNFTerm";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            positive_tags: inst.get_array("positiveTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            negative_tags: inst.get_array("negativeTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagsDNF`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsDNF {
    /// DCB field: `terms` (Class (array))
    #[serde(default)]
    pub terms: Vec<Handle<TagsDNFTerm>>,
}

impl Pooled for TagsDNF {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tags_dnf }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tags_dnf }
}

impl<'a> Extract<'a> for TagsDNF {
    const TYPE_NAME: &'static str = "TagsDNF";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            terms: inst.get_array("terms")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagsDNFTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagsDNFTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagList {
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for TagList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.tag_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.tag_list }
}

impl<'a> Extract<'a> for TagList {
    const TYPE_NAME: &'static str = "TagList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TakeDownMaxDistances`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeDownMaxDistances {
    /// DCB field: `maxDistanceFront` (Single)
    #[serde(default)]
    pub max_distance_front: f32,
    /// DCB field: `maxDistanceBack` (Single)
    #[serde(default)]
    pub max_distance_back: f32,
    /// DCB field: `maxDistanceRight` (Single)
    #[serde(default)]
    pub max_distance_right: f32,
    /// DCB field: `maxDistanceLeft` (Single)
    #[serde(default)]
    pub max_distance_left: f32,
}

impl Pooled for TakeDownMaxDistances {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.take_down_max_distances }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.take_down_max_distances }
}

impl<'a> Extract<'a> for TakeDownMaxDistances {
    const TYPE_NAME: &'static str = "TakeDownMaxDistances";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_distance_front: inst.get_f32("maxDistanceFront").unwrap_or_default(),
            max_distance_back: inst.get_f32("maxDistanceBack").unwrap_or_default(),
            max_distance_right: inst.get_f32("maxDistanceRight").unwrap_or_default(),
            max_distance_left: inst.get_f32("maxDistanceLeft").unwrap_or_default(),
        }
    }
}

/// DCB type: `TakeDownParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeDownParams {
    /// DCB field: `attackerSkeletonFilter` (Class)
    #[serde(default)]
    pub attacker_skeleton_filter: Option<Handle<ActorStateSkeletonFilter>>,
    /// DCB field: `maxDistances` (Class)
    #[serde(default)]
    pub max_distances: Option<Handle<TakeDownMaxDistances>>,
    /// DCB field: `angleRange` (Single)
    #[serde(default)]
    pub angle_range: f32,
    /// DCB field: `surpriseDelay` (Single)
    #[serde(default)]
    pub surprise_delay: f32,
    /// DCB field: `maxHeightDiffUp` (Single)
    #[serde(default)]
    pub max_height_diff_up: f32,
    /// DCB field: `maxHeightDiffDown` (Single)
    #[serde(default)]
    pub max_height_diff_down: f32,
    /// DCB field: `victimStance` (EnumChoice)
    #[serde(default)]
    pub victim_stance: String,
    /// DCB field: `attackerQuadrant` (EnumChoice)
    #[serde(default)]
    pub attacker_quadrant: String,
    /// DCB field: `isTakeDownAlwaysLethal` (Boolean)
    #[serde(default)]
    pub is_take_down_always_lethal: bool,
    /// DCB field: `interruptOnHitReaction` (Boolean)
    #[serde(default)]
    pub interrupt_on_hit_reaction: bool,
    /// DCB field: `isDodgeable` (Boolean)
    #[serde(default)]
    pub is_dodgeable: bool,
    /// DCB field: `dodgeWeaponRequiredTag` (Reference)
    #[serde(default)]
    pub dodge_weapon_required_tag: Option<CigGuid>,
    /// DCB field: `animSpeedupOnDodge` (Single)
    #[serde(default)]
    pub anim_speedup_on_dodge: f32,
    /// DCB field: `maxSpeedForRangeBoost` (Single)
    #[serde(default)]
    pub max_speed_for_range_boost: f32,
    /// DCB field: `minSpeedForRangeBoost` (Single)
    #[serde(default)]
    pub min_speed_for_range_boost: f32,
    /// DCB field: `RangeBoostForSpeed` (Single)
    #[serde(default)]
    pub range_boost_for_speed: f32,
}

impl Pooled for TakeDownParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.take_down_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.take_down_params }
}

impl<'a> Extract<'a> for TakeDownParams {
    const TYPE_NAME: &'static str = "TakeDownParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attacker_skeleton_filter: match inst.get("attackerSkeletonFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_distances: match inst.get("maxDistances") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TakeDownMaxDistances>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TakeDownMaxDistances>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angle_range: inst.get_f32("angleRange").unwrap_or_default(),
            surprise_delay: inst.get_f32("surpriseDelay").unwrap_or_default(),
            max_height_diff_up: inst.get_f32("maxHeightDiffUp").unwrap_or_default(),
            max_height_diff_down: inst.get_f32("maxHeightDiffDown").unwrap_or_default(),
            victim_stance: inst.get_str("victimStance").map(String::from).unwrap_or_default(),
            attacker_quadrant: inst.get_str("attackerQuadrant").map(String::from).unwrap_or_default(),
            is_take_down_always_lethal: inst.get_bool("isTakeDownAlwaysLethal").unwrap_or_default(),
            interrupt_on_hit_reaction: inst.get_bool("interruptOnHitReaction").unwrap_or_default(),
            is_dodgeable: inst.get_bool("isDodgeable").unwrap_or_default(),
            dodge_weapon_required_tag: inst.get("dodgeWeaponRequiredTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            anim_speedup_on_dodge: inst.get_f32("animSpeedupOnDodge").unwrap_or_default(),
            max_speed_for_range_boost: inst.get_f32("maxSpeedForRangeBoost").unwrap_or_default(),
            min_speed_for_range_boost: inst.get_f32("minSpeedForRangeBoost").unwrap_or_default(),
            range_boost_for_speed: inst.get_f32("RangeBoostForSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `TakeDownConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeDownConfig {
    /// DCB field: `takeDownParams` (Class (array))
    #[serde(default)]
    pub take_down_params: Vec<Handle<TakeDownParams>>,
    /// DCB field: `isTakedownImmune` (Boolean)
    #[serde(default)]
    pub is_takedown_immune: bool,
    /// DCB field: `takedownStimulusRange` (Single)
    #[serde(default)]
    pub takedown_stimulus_range: f32,
    /// DCB field: `QTEConfig_Knife` (Reference)
    #[serde(default)]
    pub qteconfig_knife: Option<CigGuid>,
    /// DCB field: `QTEConfig_Unarmed` (Reference)
    #[serde(default)]
    pub qteconfig_unarmed: Option<CigGuid>,
    /// DCB field: `QTEConfig_AI` (Reference)
    #[serde(default)]
    pub qteconfig_ai: Option<CigGuid>,
}

impl Pooled for TakeDownConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ta.take_down_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ta.take_down_config }
}

impl<'a> Extract<'a> for TakeDownConfig {
    const TYPE_NAME: &'static str = "TakeDownConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            take_down_params: inst.get_array("takeDownParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TakeDownParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TakeDownParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            is_takedown_immune: inst.get_bool("isTakedownImmune").unwrap_or_default(),
            takedown_stimulus_range: inst.get_f32("takedownStimulusRange").unwrap_or_default(),
            qteconfig_knife: inst.get("QTEConfig_Knife").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            qteconfig_unarmed: inst.get("QTEConfig_Unarmed").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            qteconfig_ai: inst.get("QTEConfig_AI").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

