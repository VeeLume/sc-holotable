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

/// DCB type: `Deg3`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deg3 {
    /// DCB field: `x` (Single)
    #[serde(default)]
    pub x: f32,
    /// DCB field: `y` (Single)
    #[serde(default)]
    pub y: f32,
    /// DCB field: `z` (Single)
    #[serde(default)]
    pub z: f32,
}

impl Pooled for Deg3 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.deg3 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.deg3 }
}

impl<'a> Extract<'a> for Deg3 {
    const TYPE_NAME: &'static str = "Deg3";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x: inst.get_f32("x").unwrap_or_default(),
            y: inst.get_f32("y").unwrap_or_default(),
            z: inst.get_f32("z").unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultBlueprintSelection_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultBlueprintSelection_Base {
}

impl Pooled for DefaultBlueprintSelection_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_blueprint_selection_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_blueprint_selection_base }
}

impl<'a> Extract<'a> for DefaultBlueprintSelection_Base {
    const TYPE_NAME: &'static str = "DefaultBlueprintSelection_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionDef {
}

impl Pooled for DefaultActionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_action_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_action_def }
}

impl<'a> Extract<'a> for DefaultActionDef {
    const TYPE_NAME: &'static str = "DefaultActionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsParams {
    /// DCB field: `states` (StrongPointer (array))
    #[serde(default)]
    pub states: Vec<Handle<DefaultActionsEntityState>>,
    /// DCB field: `defaultActions` (StrongPointer)
    #[serde(default)]
    pub default_actions: Option<Handle<DefaultActionDef>>,
}

impl Pooled for DefaultActionsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_actions_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_actions_params }
}

impl<'a> Extract<'a> for DefaultActionsParams {
    const TYPE_NAME: &'static str = "DefaultActionsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            states: inst.get_array("states")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsEntityState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsEntityState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_actions: match inst.get("defaultActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DefaultActionDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DefaultActionDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultActions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActions {
    /// DCB field: `defaultActionsPerState` (Class (array))
    #[serde(default)]
    pub default_actions_per_state: Vec<Handle<DefaultActionsParams>>,
}

impl Pooled for DefaultActions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_actions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_actions }
}

impl<'a> Extract<'a> for DefaultActions {
    const TYPE_NAME: &'static str = "DefaultActions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_actions_per_state: inst.get_array("defaultActionsPerState")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionDescriptionOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionDescriptionOverride {
    /// DCB field: `action` (String)
    #[serde(default)]
    pub action: String,
    /// DCB field: `actionsDescription` (Locale)
    #[serde(default)]
    pub actions_description: String,
}

impl Pooled for DefaultActionDescriptionOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_action_description_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_action_description_override }
}

impl<'a> Extract<'a> for DefaultActionDescriptionOverride {
    const TYPE_NAME: &'static str = "DefaultActionDescriptionOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            action: inst.get_str("action").map(String::from).unwrap_or_default(),
            actions_description: inst.get_str("actionsDescription").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntry {
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<DefaultActionsEntityEntryCondition>>,
    /// DCB field: `defaultActions` (Class)
    #[serde(default)]
    pub default_actions: Option<Handle<DefaultActions>>,
}

impl Pooled for DefaultActionsEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_actions_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_actions_entry }
}

impl<'a> Extract<'a> for DefaultActionsEntry {
    const TYPE_NAME: &'static str = "DefaultActionsEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsEntityEntryCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsEntityEntryCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_actions: match inst.get("defaultActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DefaultActions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DefaultActions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntityEntryCondition {
}

impl Pooled for DefaultActionsEntityEntryCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_actions_entity_entry_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_actions_entity_entry_condition }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionsEntityState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntityState {
}

impl Pooled for DefaultActionsEntityState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_actions_entity_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_actions_entity_state }
}

impl<'a> Extract<'a> for DefaultActionsEntityState {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultEntitlementRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultEntitlementRecord {
    /// DCB field: `Entitlements` (EnumChoice (array))
    #[serde(default)]
    pub entitlements: Vec<String>,
    /// DCB field: `subscribersOnly` (Boolean)
    #[serde(default)]
    pub subscribers_only: bool,
}

impl Pooled for DefaultEntitlementRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_entitlement_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_entitlement_record }
}

impl<'a> Extract<'a> for DefaultEntitlementRecord {
    const TYPE_NAME: &'static str = "DefaultEntitlementRecord";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entitlements: inst.get_array("Entitlements")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            subscribers_only: inst.get_bool("subscribersOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultPlayerLoadoutEntitlementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPlayerLoadoutEntitlementParams {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `LoadoutId` (UInt32)
    #[serde(default)]
    pub loadout_id: u32,
    /// DCB field: `Entitlement` (EnumChoice)
    #[serde(default)]
    pub entitlement: String,
    /// DCB field: `Loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for DefaultPlayerLoadoutEntitlementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_player_loadout_entitlement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_player_loadout_entitlement_params }
}

impl<'a> Extract<'a> for DefaultPlayerLoadoutEntitlementParams {
    const TYPE_NAME: &'static str = "DefaultPlayerLoadoutEntitlementParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            loadout_id: inst.get_u32("LoadoutId").unwrap_or_default(),
            entitlement: inst.get_str("Entitlement").map(String::from).unwrap_or_default(),
            loadout: match inst.get("Loadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultPlayerLoadoutEntitlementRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPlayerLoadoutEntitlementRecord {
    /// DCB field: `Loadouts` (Class (array))
    #[serde(default)]
    pub loadouts: Vec<Handle<DefaultPlayerLoadoutEntitlementParams>>,
}

impl Pooled for DefaultPlayerLoadoutEntitlementRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.default_player_loadout_entitlement_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.default_player_loadout_entitlement_record }
}

impl<'a> Extract<'a> for DefaultPlayerLoadoutEntitlementRecord {
    const TYPE_NAME: &'static str = "DefaultPlayerLoadoutEntitlementRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadouts: inst.get_array("Loadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultPlayerLoadoutEntitlementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultPlayerLoadoutEntitlementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DematerializeAnimation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DematerializeAnimation {
    /// DCB field: `dissolveStartTime` (Single)
    #[serde(default)]
    pub dissolve_start_time: f32,
    /// DCB field: `dissolveDuration` (Single)
    #[serde(default)]
    pub dissolve_duration: f32,
    /// DCB field: `headEffect` (Class)
    #[serde(default)]
    pub head_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `armEffect` (Class)
    #[serde(default)]
    pub arm_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `legEffect` (Class)
    #[serde(default)]
    pub leg_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `torsoEffect` (Class)
    #[serde(default)]
    pub torso_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `debugEffect` (Class)
    #[serde(default)]
    pub debug_effect: Option<Handle<GlobalResourceParticle>>,
}

impl Pooled for DematerializeAnimation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.dematerialize_animation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.dematerialize_animation }
}

impl<'a> Extract<'a> for DematerializeAnimation {
    const TYPE_NAME: &'static str = "DematerializeAnimation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            dissolve_start_time: inst.get_f32("dissolveStartTime").unwrap_or_default(),
            dissolve_duration: inst.get_f32("dissolveDuration").unwrap_or_default(),
            head_effect: match inst.get("headEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arm_effect: match inst.get("armEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            leg_effect: match inst.get("legEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            torso_effect: match inst.get("torsoEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            debug_effect: match inst.get("debugEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DevOwnerType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevOwnerType_Base {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for DevOwnerType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.dev_owner_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.dev_owner_type_base }
}

impl<'a> Extract<'a> for DevOwnerType_Base {
    const TYPE_NAME: &'static str = "DevOwnerType_Base";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `DevOwner`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevOwner {
    /// DCB field: `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<DevOwnerType_Base>>,
}

impl Pooled for DevOwner {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.dev_owner }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.dev_owner }
}

impl<'a> Extract<'a> for DevOwner {
    const TYPE_NAME: &'static str = "DevOwner";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: match inst.get("type") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DevOwnerType_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DevOwnerType_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DeltaSignatureSensitivityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSignatureSensitivityParams {
    /// DCB field: `sensitivity` (Single)
    #[serde(default)]
    pub sensitivity: f32,
    /// DCB field: `pierce` (Single)
    #[serde(default)]
    pub pierce: f32,
}

impl Pooled for DeltaSignatureSensitivityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.delta_signature_sensitivity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.delta_signature_sensitivity_params }
}

impl<'a> Extract<'a> for DeltaSignatureSensitivityParams {
    const TYPE_NAME: &'static str = "DeltaSignatureSensitivityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sensitivity: inst.get_f32("sensitivity").unwrap_or_default(),
            pierce: inst.get_f32("pierce").unwrap_or_default(),
        }
    }
}

/// DCB type: `DeltaSignatureSpikeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSignatureSpikeParams {
    /// DCB field: `operationType` (EnumChoice)
    #[serde(default)]
    pub operation_type: String,
    /// DCB field: `spikeValue` (Single)
    #[serde(default)]
    pub spike_value: f32,
}

impl Pooled for DeltaSignatureSpikeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.delta_signature_spike_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.delta_signature_spike_params }
}

impl<'a> Extract<'a> for DeltaSignatureSpikeParams {
    const TYPE_NAME: &'static str = "DeltaSignatureSpikeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            operation_type: inst.get_str("operationType").map(String::from).unwrap_or_default(),
            spike_value: inst.get_f32("spikeValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `DebugLoadoutKit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLoadoutKit {
    /// DCB field: `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
    /// DCB field: `loadoutKit` (Reference)
    #[serde(default)]
    pub loadout_kit: Option<CigGuid>,
}

impl Pooled for DebugLoadoutKit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_de.debug_loadout_kit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_de.debug_loadout_kit }
}

impl<'a> Extract<'a> for DebugLoadoutKit {
    const TYPE_NAME: &'static str = "DebugLoadoutKit";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            loadout_kit: inst.get("loadoutKit").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

