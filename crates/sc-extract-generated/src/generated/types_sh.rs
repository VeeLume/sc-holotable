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

/// DCB type: `SharedTacticParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTacticParams {
    /// DCB field: `distanceToTargetThreshold` (Class)
    #[serde(default)]
    pub distance_to_target_threshold: Option<Handle<TacticPlayerDistance>>,
}

impl Pooled for SharedTacticParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shared_tactic_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shared_tactic_params }
}

impl<'a> Extract<'a> for SharedTacticParams {
    const TYPE_NAME: &'static str = "SharedTacticParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distance_to_target_threshold: match inst.get("distanceToTargetThreshold") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TacticPlayerDistance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TacticPlayerDistance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ShootingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShootingParams {
    /// DCB field: `shortCombatDistanceShootingThreshold` (Single)
    #[serde(default)]
    pub short_combat_distance_shooting_threshold: f32,
    /// DCB field: `dogfightCloseCombatDistanceThreshold` (Single)
    #[serde(default)]
    pub dogfight_close_combat_distance_threshold: f32,
    /// DCB field: `undisciplinedTriggerFingerMultiplier` (Single)
    #[serde(default)]
    pub undisciplined_trigger_finger_multiplier: f32,
    /// DCB field: `triggerDisciplinedMultiplier` (Single)
    #[serde(default)]
    pub trigger_disciplined_multiplier: f32,
}

impl Pooled for ShootingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shooting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shooting_params }
}

impl<'a> Extract<'a> for ShootingParams {
    const TYPE_NAME: &'static str = "ShootingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            short_combat_distance_shooting_threshold: inst.get_f32("shortCombatDistanceShootingThreshold").unwrap_or_default(),
            dogfight_close_combat_distance_threshold: inst.get_f32("dogfightCloseCombatDistanceThreshold").unwrap_or_default(),
            undisciplined_trigger_finger_multiplier: inst.get_f32("undisciplinedTriggerFingerMultiplier").unwrap_or_default(),
            trigger_disciplined_multiplier: inst.get_f32("triggerDisciplinedMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SHandsRecoilCurveNoiseModifer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHandsRecoilCurveNoiseModifer {
    /// DCB field: `xNoiseModifier` (Single)
    #[serde(default)]
    pub x_noise_modifier: f32,
    /// DCB field: `yNoiseModifier` (Single)
    #[serde(default)]
    pub y_noise_modifier: f32,
    /// DCB field: `zNoiseModifier` (Single)
    #[serde(default)]
    pub z_noise_modifier: f32,
}

impl Pooled for SHandsRecoilCurveNoiseModifer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shands_recoil_curve_noise_modifer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shands_recoil_curve_noise_modifer }
}

impl<'a> Extract<'a> for SHandsRecoilCurveNoiseModifer {
    const TYPE_NAME: &'static str = "SHandsRecoilCurveNoiseModifer";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x_noise_modifier: inst.get_f32("xNoiseModifier").unwrap_or_default(),
            y_noise_modifier: inst.get_f32("yNoiseModifier").unwrap_or_default(),
            z_noise_modifier: inst.get_f32("zNoiseModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SHeadRecoilNoiseModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHeadRecoilNoiseModifier {
    /// DCB field: `offsetModifier` (Class)
    #[serde(default)]
    pub offset_modifier: Option<Handle<Vec3>>,
    /// DCB field: `noiseModifier` (Class)
    #[serde(default)]
    pub noise_modifier: Option<Handle<Vec3>>,
}

impl Pooled for SHeadRecoilNoiseModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shead_recoil_noise_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shead_recoil_noise_modifier }
}

impl<'a> Extract<'a> for SHeadRecoilNoiseModifier {
    const TYPE_NAME: &'static str = "SHeadRecoilNoiseModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset_modifier: match inst.get("offsetModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_modifier: match inst.get("noiseModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ShockwaveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShockwaveParams {
    /// DCB field: `primitiveType` (EnumChoice)
    #[serde(default)]
    pub primitive_type: String,
    /// DCB field: `axis` (Class)
    #[serde(default)]
    pub axis: Option<Handle<Vec3>>,
    /// DCB field: `halfHeight` (Single)
    #[serde(default)]
    pub half_height: f32,
    /// DCB field: `detonationSpeed` (Single)
    #[serde(default)]
    pub detonation_speed: f32,
    /// DCB field: `peakPressure` (Single)
    #[serde(default)]
    pub peak_pressure: f32,
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// DCB field: `durationScale` (Single)
    #[serde(default)]
    pub duration_scale: f32,
    /// DCB field: `density` (Single)
    #[serde(default)]
    pub density: f32,
    /// DCB field: `resistance` (Single)
    #[serde(default)]
    pub resistance: f32,
}

impl Pooled for ShockwaveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shockwave_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shockwave_params }
}

impl<'a> Extract<'a> for ShockwaveParams {
    const TYPE_NAME: &'static str = "ShockwaveParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            primitive_type: inst.get_str("primitiveType").map(String::from).unwrap_or_default(),
            axis: match inst.get("axis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            half_height: inst.get_f32("halfHeight").unwrap_or_default(),
            detonation_speed: inst.get_f32("detonationSpeed").unwrap_or_default(),
            peak_pressure: inst.get_f32("peakPressure").unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
            duration_scale: inst.get_f32("durationScale").unwrap_or_default(),
            density: inst.get_f32("density").unwrap_or_default(),
            resistance: inst.get_f32("resistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SHealingBeamBoneEntryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHealingBeamBoneEntryParams {
    /// DCB field: `boneName` (String)
    #[serde(default)]
    pub bone_name: String,
    /// DCB field: `boneOffset` (Class)
    #[serde(default)]
    pub bone_offset: Option<Handle<Vec3>>,
    /// DCB field: `cardOffset` (Class)
    #[serde(default)]
    pub card_offset: Option<Handle<Vec3>>,
}

impl Pooled for SHealingBeamBoneEntryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shealing_beam_bone_entry_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shealing_beam_bone_entry_params }
}

impl<'a> Extract<'a> for SHealingBeamBoneEntryParams {
    const TYPE_NAME: &'static str = "SHealingBeamBoneEntryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bone_name: inst.get_str("boneName").map(String::from).unwrap_or_default(),
            bone_offset: match inst.get("boneOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            card_offset: match inst.get("cardOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHealingBeamBodyPartHighlightingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHealingBeamBodyPartHighlightingParams {
    /// DCB field: `characterAttachmentName` (String)
    #[serde(default)]
    pub character_attachment_name: String,
    /// DCB field: `zonesToShow` (EnumChoice (array))
    #[serde(default)]
    pub zones_to_show: Vec<String>,
}

impl Pooled for SHealingBeamBodyPartHighlightingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shealing_beam_body_part_highlighting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shealing_beam_body_part_highlighting_params }
}

impl<'a> Extract<'a> for SHealingBeamBodyPartHighlightingParams {
    const TYPE_NAME: &'static str = "SHealingBeamBodyPartHighlightingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            character_attachment_name: inst.get_str("characterAttachmentName").map(String::from).unwrap_or_default(),
            zones_to_show: inst.get_array("zonesToShow")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHealingBeamBodyPartParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHealingBeamBodyPartParams {
    /// DCB field: `bodyPart` (Reference)
    #[serde(default)]
    pub body_part: Option<CigGuid>,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `boneEntry` (Class)
    #[serde(default)]
    pub bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// DCB field: `highlightParams` (Class)
    #[serde(default)]
    pub highlight_params: Option<Handle<SHealingBeamBodyPartHighlightingParams>>,
}

impl Pooled for SHealingBeamBodyPartParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shealing_beam_body_part_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shealing_beam_body_part_params }
}

impl<'a> Extract<'a> for SHealingBeamBodyPartParams {
    const TYPE_NAME: &'static str = "SHealingBeamBodyPartParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            body_part: inst.get("bodyPart").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            bone_entry: match inst.get("boneEntry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHealingBeamBodyPartHighlightingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHealingBeamBodyPartHighlightingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHostilityRules`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHostilityRules {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `useReputationHostility` (Boolean)
    #[serde(default)]
    pub use_reputation_hostility: bool,
    /// DCB field: `usePersonalHostility` (Boolean)
    #[serde(default)]
    pub use_personal_hostility: bool,
    /// DCB field: `useFactionHostility` (Boolean)
    #[serde(default)]
    pub use_faction_hostility: bool,
    /// DCB field: `policesLawfulTrespass` (Boolean)
    #[serde(default)]
    pub polices_lawful_trespass: bool,
    /// DCB field: `hostileToCriminals` (Boolean)
    #[serde(default)]
    pub hostile_to_criminals: bool,
    /// DCB field: `hostileToStolenVehicles` (Boolean)
    #[serde(default)]
    pub hostile_to_stolen_vehicles: bool,
    /// DCB field: `factionOverride` (Reference)
    #[serde(default)]
    pub faction_override: Option<CigGuid>,
    /// DCB field: `factionToOverride` (Reference)
    #[serde(default)]
    pub faction_to_override: Option<CigGuid>,
}

impl Pooled for SHostilityRules {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shostility_rules }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shostility_rules }
}

impl<'a> Extract<'a> for SHostilityRules {
    const TYPE_NAME: &'static str = "SHostilityRules";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            use_reputation_hostility: inst.get_bool("useReputationHostility").unwrap_or_default(),
            use_personal_hostility: inst.get_bool("usePersonalHostility").unwrap_or_default(),
            use_faction_hostility: inst.get_bool("useFactionHostility").unwrap_or_default(),
            polices_lawful_trespass: inst.get_bool("policesLawfulTrespass").unwrap_or_default(),
            hostile_to_criminals: inst.get_bool("hostileToCriminals").unwrap_or_default(),
            hostile_to_stolen_vehicles: inst.get_bool("hostileToStolenVehicles").unwrap_or_default(),
            faction_override: inst.get("factionOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            faction_to_override: inst.get("factionToOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ShipInsurancePolicyRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipInsurancePolicyRecord {
    /// DCB field: `uninsuredItemTypes` (Class (array))
    #[serde(default)]
    pub uninsured_item_types: Vec<Handle<SUninsuredItem>>,
}

impl Pooled for ShipInsurancePolicyRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.ship_insurance_policy_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.ship_insurance_policy_record }
}

impl<'a> Extract<'a> for ShipInsurancePolicyRecord {
    const TYPE_NAME: &'static str = "ShipInsurancePolicyRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            uninsured_item_types: inst.get_array("uninsuredItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SUninsuredItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SUninsuredItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHighlightBehaviorNodeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHighlightBehaviorNodeParams {
}

impl Pooled for SHighlightBehaviorNodeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shighlight_behavior_node_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shighlight_behavior_node_params }
}

impl<'a> Extract<'a> for SHighlightBehaviorNodeParams {
    const TYPE_NAME: &'static str = "SHighlightBehaviorNodeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SHighlightBehaviorNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHighlightBehaviorNode {
    /// DCB field: `HighlightBehaviorNodeParams` (StrongPointer)
    #[serde(default)]
    pub highlight_behavior_node_params: Option<Handle<SHighlightBehaviorNodeParams>>,
    /// DCB field: `nextNodes` (Class (array))
    #[serde(default)]
    pub next_nodes: Vec<Handle<SHighlightBehaviorNode>>,
}

impl Pooled for SHighlightBehaviorNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shighlight_behavior_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shighlight_behavior_node }
}

impl<'a> Extract<'a> for SHighlightBehaviorNode {
    const TYPE_NAME: &'static str = "SHighlightBehaviorNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            highlight_behavior_node_params: match inst.get("HighlightBehaviorNodeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHighlightBehaviorNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHighlightBehaviorNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            next_nodes: inst.get_array("nextNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHighlightBehaviorNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHighlightBehaviorNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHudTapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHudTapeParams {
    /// DCB field: `range` (Single)
    #[serde(default)]
    pub range: f32,
    /// DCB field: `mainTickIncrement` (Single)
    #[serde(default)]
    pub main_tick_increment: f32,
    /// DCB field: `subTicks` (Int32)
    #[serde(default)]
    pub sub_ticks: i32,
}

impl Pooled for SHudTapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shud_tape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shud_tape_params }
}

impl<'a> Extract<'a> for SHudTapeParams {
    const TYPE_NAME: &'static str = "SHudTapeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            range: inst.get_f32("range").unwrap_or_default(),
            main_tick_increment: inst.get_f32("mainTickIncrement").unwrap_or_default(),
            sub_ticks: inst.get_i32("subTicks").unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipComputerDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipComputerDefinition {
    /// DCB field: `communicationConfig` (Reference)
    #[serde(default)]
    pub communication_config: Option<CigGuid>,
    /// DCB field: `preBootUpTime` (Single)
    #[serde(default)]
    pub pre_boot_up_time: f32,
    /// DCB field: `bootUpTime` (Single)
    #[serde(default)]
    pub boot_up_time: f32,
    /// DCB field: `timeSinceLastHitMarkerSfx` (Class)
    #[serde(default)]
    pub time_since_last_hit_marker_sfx: Option<Handle<AudioRtpc>>,
}

impl Pooled for ShipComputerDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.ship_computer_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.ship_computer_definition }
}

impl<'a> Extract<'a> for ShipComputerDefinition {
    const TYPE_NAME: &'static str = "ShipComputerDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            communication_config: inst.get("communicationConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pre_boot_up_time: inst.get_f32("preBootUpTime").unwrap_or_default(),
            boot_up_time: inst.get_f32("bootUpTime").unwrap_or_default(),
            time_since_last_hit_marker_sfx: match inst.get("timeSinceLastHitMarkerSfx") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHelmetLinkedState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHelmetLinkedState {
    /// DCB field: `stateMachine` (EnumChoice)
    #[serde(default)]
    pub state_machine: String,
    /// DCB field: `stateToEnter` (EnumChoice)
    #[serde(default)]
    pub state_to_enter: String,
}

impl Pooled for SHelmetLinkedState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shelmet_linked_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shelmet_linked_state }
}

impl<'a> Extract<'a> for SHelmetLinkedState {
    const TYPE_NAME: &'static str = "SHelmetLinkedState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_machine: inst.get_str("stateMachine").map(String::from).unwrap_or_default(),
            state_to_enter: inst.get_str("stateToEnter").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SHelmetStateBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHelmetStateBaseParams {
    /// DCB field: `state` (EnumChoice)
    #[serde(default)]
    pub state: String,
    /// DCB field: `nextState` (EnumChoice)
    #[serde(default)]
    pub next_state: String,
    /// DCB field: `linkedStates` (Class (array))
    #[serde(default)]
    pub linked_states: Vec<Handle<SHelmetLinkedState>>,
}

impl Pooled for SHelmetStateBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shelmet_state_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shelmet_state_base_params }
}

impl<'a> Extract<'a> for SHelmetStateBaseParams {
    const TYPE_NAME: &'static str = "SHelmetStateBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: inst.get_str("state").map(String::from).unwrap_or_default(),
            next_state: inst.get_str("nextState").map(String::from).unwrap_or_default(),
            linked_states: inst.get_array("linkedStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHelmetLinkedState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHelmetLinkedState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHelmetStateMachineParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHelmetStateMachineParams {
    /// DCB field: `stateMachine` (EnumChoice)
    #[serde(default)]
    pub state_machine: String,
    /// DCB field: `states` (StrongPointer (array))
    #[serde(default)]
    pub states: Vec<Handle<SHelmetStateBaseParams>>,
    /// DCB field: `startState` (EnumChoice)
    #[serde(default)]
    pub start_state: String,
}

impl Pooled for SHelmetStateMachineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shelmet_state_machine_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shelmet_state_machine_params }
}

impl<'a> Extract<'a> for SHelmetStateMachineParams {
    const TYPE_NAME: &'static str = "SHelmetStateMachineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_machine: inst.get_str("stateMachine").map(String::from).unwrap_or_default(),
            states: inst.get_array("states")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHelmetStateBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHelmetStateBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            start_state: inst.get_str("startState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ShopInteractionData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopInteractionData {
    /// DCB field: `quickBuyConditionList` (Class)
    #[serde(default)]
    pub quick_buy_condition_list: Option<Handle<InteractionConditionList>>,
    /// DCB field: `quickBuyInteractionText` (Locale)
    #[serde(default)]
    pub quick_buy_interaction_text: String,
    /// DCB field: `quickBuyPriceStringToken` (String)
    #[serde(default)]
    pub quick_buy_price_string_token: String,
    /// DCB field: `moreInfoInteractionText` (Locale)
    #[serde(default)]
    pub more_info_interaction_text: String,
}

impl Pooled for ShopInteractionData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shop_interaction_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shop_interaction_data }
}

impl<'a> Extract<'a> for ShopInteractionData {
    const TYPE_NAME: &'static str = "ShopInteractionData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            quick_buy_condition_list: match inst.get("quickBuyConditionList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InteractionConditionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InteractionConditionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            quick_buy_interaction_text: inst.get_str("quickBuyInteractionText").map(String::from).unwrap_or_default(),
            quick_buy_price_string_token: inst.get_str("quickBuyPriceStringToken").map(String::from).unwrap_or_default(),
            more_info_interaction_text: inst.get_str("moreInfoInteractionText").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ShopFranchise`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopFranchise {
    /// DCB field: `localizedName` (Locale)
    #[serde(default)]
    pub localized_name: String,
}

impl Pooled for ShopFranchise {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shop_franchise }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shop_franchise }
}

impl<'a> Extract<'a> for ShopFranchise {
    const TYPE_NAME: &'static str = "ShopFranchise";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            localized_name: inst.get_str("localizedName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ShieldTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldTypeParams {
    /// DCB field: `impactEffect` (Class)
    #[serde(default)]
    pub impact_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `minHealthColor` (Class)
    #[serde(default)]
    pub min_health_color: Option<Handle<SRGB8>>,
    /// DCB field: `maxHealthColor` (Class)
    #[serde(default)]
    pub max_health_color: Option<Handle<SRGB8>>,
    /// DCB field: `maxDamageStrength` (Single)
    #[serde(default)]
    pub max_damage_strength: f32,
    /// DCB field: `sizeScaleMultiplier` (Single)
    #[serde(default)]
    pub size_scale_multiplier: f32,
    /// DCB field: `sizeScale1stPerson` (Single)
    #[serde(default)]
    pub size_scale1st_person: f32,
    /// DCB field: `alphaScaleDefault` (Single)
    #[serde(default)]
    pub alpha_scale_default: f32,
    /// DCB field: `alphaScale1stPerson` (Single)
    #[serde(default)]
    pub alpha_scale1st_person: f32,
    /// DCB field: `maxHitImpact` (Int32)
    #[serde(default)]
    pub max_hit_impact: i32,
}

impl Pooled for ShieldTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shield_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shield_type_params }
}

impl<'a> Extract<'a> for ShieldTypeParams {
    const TYPE_NAME: &'static str = "ShieldTypeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            impact_effect: match inst.get("impactEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_health_color: match inst.get("minHealthColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_health_color: match inst.get("maxHealthColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_damage_strength: inst.get_f32("maxDamageStrength").unwrap_or_default(),
            size_scale_multiplier: inst.get_f32("sizeScaleMultiplier").unwrap_or_default(),
            size_scale1st_person: inst.get_f32("sizeScale1stPerson").unwrap_or_default(),
            alpha_scale_default: inst.get_f32("alphaScaleDefault").unwrap_or_default(),
            alpha_scale1st_person: inst.get_f32("alphaScale1stPerson").unwrap_or_default(),
            max_hit_impact: inst.get_i32("maxHitImpact").unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipComputerPresetList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipComputerPresetList {
    /// DCB field: `presets` (Reference (array))
    #[serde(default)]
    pub presets: Vec<CigGuid>,
}

impl Pooled for ShipComputerPresetList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.ship_computer_preset_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.ship_computer_preset_list }
}

impl<'a> Extract<'a> for ShipComputerPresetList {
    const TYPE_NAME: &'static str = "ShipComputerPresetList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            presets: inst.get_array("presets")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipComputerPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipComputerPreset {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `base` (Reference)
    #[serde(default)]
    pub base: Option<CigGuid>,
    /// DCB field: `displayText` (Locale)
    #[serde(default)]
    pub display_text: String,
    /// DCB field: `responses` (Reference (array))
    #[serde(default)]
    pub responses: Vec<CigGuid>,
}

impl Pooled for ShipComputerPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.ship_computer_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.ship_computer_preset }
}

impl<'a> Extract<'a> for ShipComputerPreset {
    const TYPE_NAME: &'static str = "ShipComputerPreset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            base: inst.get("base").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_text: inst.get_str("displayText").map(String::from).unwrap_or_default(),
            responses: inst.get_array("responses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHandsRecoilCurveNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHandsRecoilCurveNoiseParams {
    /// DCB field: `xNoise` (Single)
    #[serde(default)]
    pub x_noise: f32,
    /// DCB field: `canInvertXCurve` (Boolean)
    #[serde(default)]
    pub can_invert_xcurve: bool,
    /// DCB field: `yNoise` (Single)
    #[serde(default)]
    pub y_noise: f32,
    /// DCB field: `canInvertYCurve` (Boolean)
    #[serde(default)]
    pub can_invert_ycurve: bool,
    /// DCB field: `zNoise` (Single)
    #[serde(default)]
    pub z_noise: f32,
    /// DCB field: `canInvertZCurve` (Boolean)
    #[serde(default)]
    pub can_invert_zcurve: bool,
}

impl Pooled for SHandsRecoilCurveNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shands_recoil_curve_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shands_recoil_curve_noise_params }
}

impl<'a> Extract<'a> for SHandsRecoilCurveNoiseParams {
    const TYPE_NAME: &'static str = "SHandsRecoilCurveNoiseParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x_noise: inst.get_f32("xNoise").unwrap_or_default(),
            can_invert_xcurve: inst.get_bool("canInvertXCurve").unwrap_or_default(),
            y_noise: inst.get_f32("yNoise").unwrap_or_default(),
            can_invert_ycurve: inst.get_bool("canInvertYCurve").unwrap_or_default(),
            z_noise: inst.get_f32("zNoise").unwrap_or_default(),
            can_invert_zcurve: inst.get_bool("canInvertZCurve").unwrap_or_default(),
        }
    }
}

/// DCB type: `SHandsRecoilTimeModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHandsRecoilTimeModifier {
    /// DCB field: `recoilModifierTime` (Single)
    #[serde(default)]
    pub recoil_modifier_time: f32,
    /// DCB field: `positionMaxValueCurves` (Class)
    #[serde(default)]
    pub position_max_value_curves: Option<Handle<SXYZCurves>>,
    /// DCB field: `rotationMaxValueCurves` (Class)
    #[serde(default)]
    pub rotation_max_value_curves: Option<Handle<SXYZCurves>>,
}

impl Pooled for SHandsRecoilTimeModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shands_recoil_time_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shands_recoil_time_modifier }
}

impl<'a> Extract<'a> for SHandsRecoilTimeModifier {
    const TYPE_NAME: &'static str = "SHandsRecoilTimeModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recoil_modifier_time: inst.get_f32("recoilModifierTime").unwrap_or_default(),
            position_max_value_curves: match inst.get("positionMaxValueCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_max_value_curves: match inst.get("rotationMaxValueCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHeadRecoilNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHeadRecoilNoiseParams {
    /// DCB field: `xNoise` (Single)
    #[serde(default)]
    pub x_noise: f32,
    /// DCB field: `canXInvert` (Boolean)
    #[serde(default)]
    pub can_xinvert: bool,
    /// DCB field: `yNoise` (Single)
    #[serde(default)]
    pub y_noise: f32,
    /// DCB field: `canYInvert` (Boolean)
    #[serde(default)]
    pub can_yinvert: bool,
    /// DCB field: `zNoise` (Single)
    #[serde(default)]
    pub z_noise: f32,
    /// DCB field: `canZInvert` (Boolean)
    #[serde(default)]
    pub can_zinvert: bool,
}

impl Pooled for SHeadRecoilNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sh.shead_recoil_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sh.shead_recoil_noise_params }
}

impl<'a> Extract<'a> for SHeadRecoilNoiseParams {
    const TYPE_NAME: &'static str = "SHeadRecoilNoiseParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x_noise: inst.get_f32("xNoise").unwrap_or_default(),
            can_xinvert: inst.get_bool("canXInvert").unwrap_or_default(),
            y_noise: inst.get_f32("yNoise").unwrap_or_default(),
            can_yinvert: inst.get_bool("canYInvert").unwrap_or_default(),
            z_noise: inst.get_f32("zNoise").unwrap_or_default(),
            can_zinvert: inst.get_bool("canZInvert").unwrap_or_default(),
        }
    }
}

