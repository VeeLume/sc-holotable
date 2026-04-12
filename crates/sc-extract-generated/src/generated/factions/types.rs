// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `factions`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Faction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `defaultReaction` (EnumChoice)
    #[serde(default)]
    pub default_reaction: String,
    /// `alliedFactions` (Reference (array))
    #[serde(default)]
    pub allied_factions: Vec<CigGuid>,
    /// `enemyFactions` (Reference (array))
    #[serde(default)]
    pub enemy_factions: Vec<CigGuid>,
    /// `friendlyFireBehaviorOverrides` (Class (array))
    #[serde(default)]
    pub friendly_fire_behavior_overrides: Vec<Handle<FriendlyFireReactionOverride>>,
    /// `factionType` (EnumChoice)
    #[serde(default)]
    pub faction_type: String,
    /// `ableToArrest` (Boolean)
    #[serde(default)]
    pub able_to_arrest: bool,
    /// `policesLawfulTrespass` (Boolean)
    #[serde(default)]
    pub polices_lawful_trespass: bool,
    /// `policesCriminality` (Boolean)
    #[serde(default)]
    pub polices_criminality: bool,
    /// `noLegalRights` (Boolean)
    #[serde(default)]
    pub no_legal_rights: bool,
    /// `factionReputationRef` (Reference)
    #[serde(default)]
    pub faction_reputation_ref: Option<CigGuid>,
}

impl Pooled for Faction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.factions.faction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.factions.faction }
}

impl<'a> Extract<'a> for Faction {
    const TYPE_NAME: &'static str = "Faction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            default_reaction: inst.get_str("defaultReaction").map(String::from).unwrap_or_default(),
            allied_factions: inst.get_array("alliedFactions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            enemy_factions: inst.get_array("enemyFactions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            friendly_fire_behavior_overrides: inst.get_array("friendlyFireBehaviorOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FriendlyFireReactionOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FriendlyFireReactionOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            faction_type: inst.get_str("factionType").map(String::from).unwrap_or_default(),
            able_to_arrest: inst.get_bool("ableToArrest").unwrap_or_default(),
            polices_lawful_trespass: inst.get_bool("policesLawfulTrespass").unwrap_or_default(),
            polices_criminality: inst.get_bool("policesCriminality").unwrap_or_default(),
            no_legal_rights: inst.get_bool("noLegalRights").unwrap_or_default(),
            faction_reputation_ref: inst.get("factionReputationRef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `FactionReputation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionReputation {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `GEID` (UInt64)
    #[serde(default)]
    pub geid: u64,
    /// `logo` (String)
    #[serde(default)]
    pub logo: String,
    /// `hideInDelpihApp` (Boolean)
    #[serde(default)]
    pub hide_in_delpih_app: bool,
    /// `isNPC` (Boolean)
    #[serde(default)]
    pub is_npc: bool,
    /// `reputationContextPropertiesUI` (Reference)
    #[serde(default)]
    pub reputation_context_properties_ui: Option<CigGuid>,
    /// `allies` (Reference (array))
    #[serde(default)]
    pub allies: Vec<CigGuid>,
    /// `enemies` (Reference (array))
    #[serde(default)]
    pub enemies: Vec<CigGuid>,
    /// `hostilityParams` (Class)
    #[serde(default)]
    pub hostility_params: Option<Handle<RelationStandingParams>>,
    /// `alliedParams` (Class)
    #[serde(default)]
    pub allied_params: Option<Handle<RelationStandingParams>>,
    /// `propertiesBB` (Class (array))
    #[serde(default)]
    pub properties_bb: Vec<Handle<SReputationContextBBPropertyParams>>,
    /// `perkReputationRewardList` (Reference (array))
    #[serde(default)]
    pub perk_reputation_reward_list: Vec<CigGuid>,
    /// `sandboxTriggers` (StrongPointer (array))
    #[serde(default)]
    pub sandbox_triggers: Vec<Handle<SandboxTriggerBaseDef>>,
}

impl Pooled for FactionReputation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.factions.faction_reputation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.factions.faction_reputation }
}

impl<'a> Extract<'a> for FactionReputation {
    const TYPE_NAME: &'static str = "FactionReputation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            geid: inst.get("GEID").and_then(|v| v.as_u64()).unwrap_or_default(),
            logo: inst.get_str("logo").map(String::from).unwrap_or_default(),
            hide_in_delpih_app: inst.get_bool("hideInDelpihApp").unwrap_or_default(),
            is_npc: inst.get_bool("isNPC").unwrap_or_default(),
            reputation_context_properties_ui: inst.get("reputationContextPropertiesUI").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            allies: inst.get_array("allies")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            enemies: inst.get_array("enemies")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            hostility_params: match inst.get("hostilityParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RelationStandingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RelationStandingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allied_params: match inst.get("alliedParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RelationStandingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RelationStandingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            properties_bb: inst.get_array("propertiesBB")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            perk_reputation_reward_list: inst.get_array("perkReputationRewardList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            sandbox_triggers: inst.get_array("sandboxTriggers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SandboxTriggerBaseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SandboxTriggerBaseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RelationMarkerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationMarkerParams {
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
}

impl Pooled for RelationMarkerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.factions.relation_marker_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.factions.relation_marker_params }
}

impl<'a> Extract<'a> for RelationMarkerParams {
    const TYPE_NAME: &'static str = "RelationMarkerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RelationStandingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationStandingParams {
    /// `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// `markerParams` (Class)
    #[serde(default)]
    pub marker_params: Option<Handle<RelationMarkerParams>>,
}

impl Pooled for RelationStandingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.factions.relation_standing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.factions.relation_standing_params }
}

impl<'a> Extract<'a> for RelationStandingParams {
    const TYPE_NAME: &'static str = "RelationStandingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope: inst.get("scope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            standing: inst.get("standing").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            marker_params: match inst.get("markerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RelationMarkerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RelationMarkerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SandboxTriggerBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTriggerBaseDef {
}

impl Pooled for SandboxTriggerBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.factions.sandbox_trigger_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.factions.sandbox_trigger_base_def }
}

impl<'a> Extract<'a> for SandboxTriggerBaseDef {
    const TYPE_NAME: &'static str = "SandboxTriggerBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

