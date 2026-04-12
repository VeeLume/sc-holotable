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

/// DCB type: `Faction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `defaultReaction` (EnumChoice)
    #[serde(default)]
    pub default_reaction: String,
    /// DCB field: `alliedFactions` (Reference (array))
    #[serde(default)]
    pub allied_factions: Vec<CigGuid>,
    /// DCB field: `enemyFactions` (Reference (array))
    #[serde(default)]
    pub enemy_factions: Vec<CigGuid>,
    /// DCB field: `friendlyFireBehaviorOverrides` (Class (array))
    #[serde(default)]
    pub friendly_fire_behavior_overrides: Vec<Handle<FriendlyFireReactionOverride>>,
    /// DCB field: `factionType` (EnumChoice)
    #[serde(default)]
    pub faction_type: String,
    /// DCB field: `ableToArrest` (Boolean)
    #[serde(default)]
    pub able_to_arrest: bool,
    /// DCB field: `policesLawfulTrespass` (Boolean)
    #[serde(default)]
    pub polices_lawful_trespass: bool,
    /// DCB field: `policesCriminality` (Boolean)
    #[serde(default)]
    pub polices_criminality: bool,
    /// DCB field: `noLegalRights` (Boolean)
    #[serde(default)]
    pub no_legal_rights: bool,
    /// DCB field: `factionReputationRef` (Reference)
    #[serde(default)]
    pub faction_reputation_ref: Option<CigGuid>,
}

impl Pooled for Faction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fa.faction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fa.faction }
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

/// DCB type: `FactionPalettes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionPalettes {
    /// DCB field: `Palettes` (Reference (array))
    #[serde(default)]
    pub palettes: Vec<CigGuid>,
}

impl Pooled for FactionPalettes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fa.faction_palettes }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fa.faction_palettes }
}

impl<'a> Extract<'a> for FactionPalettes {
    const TYPE_NAME: &'static str = "FactionPalettes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            palettes: inst.get_array("Palettes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FactionPalette`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionPalette {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `tintingActive` (Boolean)
    #[serde(default)]
    pub tinting_active: bool,
    /// DCB field: `ExteriorMaterialOverride` (Class)
    #[serde(default)]
    pub exterior_material_override: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `InteriorMaterialOverride` (Class)
    #[serde(default)]
    pub interior_material_override: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `BrandingMaterialOverride` (Class)
    #[serde(default)]
    pub branding_material_override: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `ExteriorPrimaryColor` (Class)
    #[serde(default)]
    pub exterior_primary_color: Option<Handle<SRGB8>>,
    /// DCB field: `ExteriorSecondaryColor` (Class)
    #[serde(default)]
    pub exterior_secondary_color: Option<Handle<SRGB8>>,
    /// DCB field: `ExteriorTertiaryColor` (Class)
    #[serde(default)]
    pub exterior_tertiary_color: Option<Handle<SRGB8>>,
    /// DCB field: `ExteriorGraphicsColor` (Class)
    #[serde(default)]
    pub exterior_graphics_color: Option<Handle<SRGB8>>,
    /// DCB field: `InteriorPrimaryColor` (Class)
    #[serde(default)]
    pub interior_primary_color: Option<Handle<SRGB8>>,
    /// DCB field: `InteriorSecondaryColor` (Class)
    #[serde(default)]
    pub interior_secondary_color: Option<Handle<SRGB8>>,
    /// DCB field: `InteriorTertiaryColor` (Class)
    #[serde(default)]
    pub interior_tertiary_color: Option<Handle<SRGB8>>,
    /// DCB field: `InteriorGraphicsColor` (Class)
    #[serde(default)]
    pub interior_graphics_color: Option<Handle<SRGB8>>,
}

impl Pooled for FactionPalette {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fa.faction_palette }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fa.faction_palette }
}

impl<'a> Extract<'a> for FactionPalette {
    const TYPE_NAME: &'static str = "FactionPalette";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tinting_active: inst.get_bool("tintingActive").unwrap_or_default(),
            exterior_material_override: match inst.get("ExteriorMaterialOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            interior_material_override: match inst.get("InteriorMaterialOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            branding_material_override: match inst.get("BrandingMaterialOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exterior_primary_color: match inst.get("ExteriorPrimaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exterior_secondary_color: match inst.get("ExteriorSecondaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exterior_tertiary_color: match inst.get("ExteriorTertiaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exterior_graphics_color: match inst.get("ExteriorGraphicsColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            interior_primary_color: match inst.get("InteriorPrimaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            interior_secondary_color: match inst.get("InteriorSecondaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            interior_tertiary_color: match inst.get("InteriorTertiaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            interior_graphics_color: match inst.get("InteriorGraphicsColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FactionReputation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionReputation {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `GEID` (UInt64)
    #[serde(default)]
    pub geid: u64,
    /// DCB field: `logo` (String)
    #[serde(default)]
    pub logo: String,
    /// DCB field: `hideInDelpihApp` (Boolean)
    #[serde(default)]
    pub hide_in_delpih_app: bool,
    /// DCB field: `isNPC` (Boolean)
    #[serde(default)]
    pub is_npc: bool,
    /// DCB field: `reputationContextPropertiesUI` (Reference)
    #[serde(default)]
    pub reputation_context_properties_ui: Option<CigGuid>,
    /// DCB field: `allies` (Reference (array))
    #[serde(default)]
    pub allies: Vec<CigGuid>,
    /// DCB field: `enemies` (Reference (array))
    #[serde(default)]
    pub enemies: Vec<CigGuid>,
    /// DCB field: `hostilityParams` (Class)
    #[serde(default)]
    pub hostility_params: Option<Handle<RelationStandingParams>>,
    /// DCB field: `alliedParams` (Class)
    #[serde(default)]
    pub allied_params: Option<Handle<RelationStandingParams>>,
    /// DCB field: `propertiesBB` (Class (array))
    #[serde(default)]
    pub properties_bb: Vec<Handle<SReputationContextBBPropertyParams>>,
    /// DCB field: `perkReputationRewardList` (Reference (array))
    #[serde(default)]
    pub perk_reputation_reward_list: Vec<CigGuid>,
    /// DCB field: `sandboxTriggers` (StrongPointer (array))
    #[serde(default)]
    pub sandbox_triggers: Vec<Handle<SandboxTriggerBaseDef>>,
}

impl Pooled for FactionReputation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fa.faction_reputation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fa.faction_reputation }
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

/// DCB type: `FactionRelationship`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionRelationship {
    /// DCB field: `faction` (Reference)
    #[serde(default)]
    pub faction: Option<CigGuid>,
    /// DCB field: `reactionType` (EnumChoice)
    #[serde(default)]
    pub reaction_type: String,
}

impl Pooled for FactionRelationship {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fa.faction_relationship }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fa.faction_relationship }
}

impl<'a> Extract<'a> for FactionRelationship {
    const TYPE_NAME: &'static str = "FactionRelationship";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            faction: inst.get("faction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reaction_type: inst.get_str("reactionType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `Faction_LEGACY`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction_LEGACY {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `factionIcon` (String)
    #[serde(default)]
    pub faction_icon: String,
    /// DCB field: `gameToken` (String)
    #[serde(default)]
    pub game_token: String,
    /// DCB field: `defaultReaction` (EnumChoice)
    #[serde(default)]
    pub default_reaction: String,
    /// DCB field: `friendlyFireBehaviorOverrides` (Class (array))
    #[serde(default)]
    pub friendly_fire_behavior_overrides: Vec<Handle<FriendlyFireReactionOverride>>,
    /// DCB field: `factionRelationships` (Class (array))
    #[serde(default)]
    pub faction_relationships: Vec<Handle<FactionRelationship>>,
    /// DCB field: `disguiseManufacturerTags` (Class)
    #[serde(default)]
    pub disguise_manufacturer_tags: Option<Handle<TagsDNF>>,
}

impl Pooled for Faction_LEGACY {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fa.faction_legacy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fa.faction_legacy }
}

impl<'a> Extract<'a> for Faction_LEGACY {
    const TYPE_NAME: &'static str = "Faction_LEGACY";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            faction_icon: inst.get_str("factionIcon").map(String::from).unwrap_or_default(),
            game_token: inst.get_str("gameToken").map(String::from).unwrap_or_default(),
            default_reaction: inst.get_str("defaultReaction").map(String::from).unwrap_or_default(),
            friendly_fire_behavior_overrides: inst.get_array("friendlyFireBehaviorOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FriendlyFireReactionOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FriendlyFireReactionOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            faction_relationships: inst.get_array("factionRelationships")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FactionRelationship>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FactionRelationship>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            disguise_manufacturer_tags: match inst.get("disguiseManufacturerTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

