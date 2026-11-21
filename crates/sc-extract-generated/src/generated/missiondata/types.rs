// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missiondata`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `MissionLocationClasses`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionLocationClasses {
    /// `locationClasses` (Reference (array))
    pub location_classes: Vec<CigGuid>,
}

impl Pooled for MissionLocationClasses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_location_classes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_location_classes
    }
}

impl<'a> Extract<'a> for MissionLocationClasses {
    const TYPE_NAME: &'static str = "MissionLocationClasses";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_classes: inst
                .get_array("locationClasses")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionLocationSizes`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionLocationSizes {
    /// `locationSizes` (Reference (array))
    pub location_sizes: Vec<CigGuid>,
}

impl Pooled for MissionLocationSizes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_location_sizes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_location_sizes
    }
}

impl<'a> Extract<'a> for MissionLocationSizes {
    const TYPE_NAME: &'static str = "MissionLocationSizes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_sizes: inst
                .get_array("locationSizes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionLocationFunctions`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionLocationFunctions {
    /// `locationFunctions` (Reference (array))
    pub location_functions: Vec<CigGuid>,
}

impl Pooled for MissionLocationFunctions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_location_functions
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_location_functions
    }
}

impl<'a> Extract<'a> for MissionLocationFunctions {
    const TYPE_NAME: &'static str = "MissionLocationFunctions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_functions: inst
                .get_array("locationFunctions")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractScope_Item`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct ContractScope_Item {
    /// `displayName` (Locale)
    pub display_name: LocaleKey,
    /// `icon` (String)
    pub icon: String,
}

impl Pooled for ContractScope_Item {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.contract_scope_item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.contract_scope_item
    }
}

impl<'a> Extract<'a> for ContractScope_Item {
    const TYPE_NAME: &'static str = "ContractScope_Item";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst
                .get_str("displayName")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractScopeGlobalParams`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct ContractScopeGlobalParams {
    /// `location` (StrongPointer)
    pub location: Option<Handle<ContractScope_Item>>,
    /// `cluster` (StrongPointer)
    pub cluster: Option<Handle<ContractScope_Item>>,
    /// `sector` (StrongPointer)
    pub sector: Option<Handle<ContractScope_Item>>,
    /// `planetaryBody` (StrongPointer)
    pub planetary_body: Option<Handle<ContractScope_Item>>,
    /// `planetarySystem` (StrongPointer)
    pub planetary_system: Option<Handle<ContractScope_Item>>,
    /// `solarSystem` (StrongPointer)
    pub solar_system: Option<Handle<ContractScope_Item>>,
    /// `universe` (StrongPointer)
    pub universe: Option<Handle<ContractScope_Item>>,
}

impl Pooled for ContractScopeGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.contract_scope_global_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.contract_scope_global_params
    }
}

impl<'a> Extract<'a> for ContractScopeGlobalParams {
    const TYPE_NAME: &'static str = "ContractScopeGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location: match inst.get("location") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractScope_Item>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            cluster: match inst.get("cluster") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractScope_Item>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            sector: match inst.get("sector") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractScope_Item>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            planetary_body: match inst.get("planetaryBody") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractScope_Item>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            planetary_system: match inst.get("planetarySystem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractScope_Item>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            solar_system: match inst.get("solarSystem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractScope_Item>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            universe: match inst.get("universe") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractScope_Item>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityClusterId`
pub struct EntityClusterId {}

impl Pooled for EntityClusterId {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.entity_cluster_id
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.entity_cluster_id
    }
}

impl<'a> Extract<'a> for EntityClusterId {
    const TYPE_NAME: &'static str = "EntityClusterId";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `EntityClusterMember`
pub struct EntityClusterMember {
    /// `variables` (StrongPointer (array))
    pub variables: Vec<MissionVariableBasePtr>,
}

impl Pooled for EntityClusterMember {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.entity_cluster_member
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.entity_cluster_member
    }
}

impl<'a> Extract<'a> for EntityClusterMember {
    const TYPE_NAME: &'static str = "EntityClusterMember";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variables: inst
                .get_array("variables")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(MissionVariableBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionVariableBoolean`
/// Inherits from: `MissionVariableBase`
pub struct MissionVariableBoolean {
    /// `name` (String)
    pub name: String,
    /// `description` (String)
    pub description: String,
    /// `value` (Boolean)
    pub value: bool,
}

impl Pooled for MissionVariableBoolean {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_variable_boolean
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_variable_boolean
    }
}

impl<'a> Extract<'a> for MissionVariableBoolean {
    const TYPE_NAME: &'static str = "MissionVariableBoolean";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst
                .get_str("description")
                .map(String::from)
                .unwrap_or_default(),
            value: inst.get_bool("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionVariableInteger`
/// Inherits from: `MissionVariableBase`
pub struct MissionVariableInteger {
    /// `name` (String)
    pub name: String,
    /// `description` (String)
    pub description: String,
    /// `value` (Int32)
    pub value: i32,
}

impl Pooled for MissionVariableInteger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_variable_integer
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_variable_integer
    }
}

impl<'a> Extract<'a> for MissionVariableInteger {
    const TYPE_NAME: &'static str = "MissionVariableInteger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst
                .get_str("description")
                .map(String::from)
                .unwrap_or_default(),
            value: inst.get_i32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `ModuleDeclarationType_Location`
/// Inherits from: `ModuleDeclarationType_Base`
pub struct ModuleDeclarationType_Location {
    /// `isPersistent` (Boolean)
    pub is_persistent: bool,
}

impl Pooled for ModuleDeclarationType_Location {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.module_declaration_type_location
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.module_declaration_type_location
    }
}

impl<'a> Extract<'a> for ModuleDeclarationType_Location {
    const TYPE_NAME: &'static str = "ModuleDeclarationType_Location";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_persistent: inst.get_bool("isPersistent").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionInitParamBoolean`
/// Inherits from: `AbstractMissionInitParam`
pub struct MissionInitParamBoolean {
    /// `name` (String)
    pub name: String,
    /// `value` (Boolean)
    pub value: bool,
}

impl Pooled for MissionInitParamBoolean {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_init_param_boolean
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_init_param_boolean
    }
}

impl<'a> Extract<'a> for MissionInitParamBoolean {
    const TYPE_NAME: &'static str = "MissionInitParamBoolean";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            value: inst.get_bool("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionInitParamInteger`
/// Inherits from: `AbstractMissionInitParam`
pub struct MissionInitParamInteger {
    /// `name` (String)
    pub name: String,
    /// `value` (Int32)
    pub value: i32,
}

impl Pooled for MissionInitParamInteger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_init_param_integer
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_init_param_integer
    }
}

impl<'a> Extract<'a> for MissionInitParamInteger {
    const TYPE_NAME: &'static str = "MissionInitParamInteger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            value: inst.get_i32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionInitParamTag`
/// Inherits from: `AbstractMissionInitParam`
pub struct MissionInitParamTag {
    /// `name` (String)
    pub name: String,
    /// `value` (Class)
    pub value: Option<Handle<TagSearchTerm>>,
}

impl Pooled for MissionInitParamTag {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_init_param_tag
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_init_param_tag
    }
}

impl<'a> Extract<'a> for MissionInitParamTag {
    const TYPE_NAME: &'static str = "MissionInitParamTag";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagSearchTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionInitParamFloat`
/// Inherits from: `AbstractMissionInitParam`
pub struct MissionInitParamFloat {
    /// `name` (String)
    pub name: String,
    /// `value` (Single)
    pub value: f32,
}

impl Pooled for MissionInitParamFloat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_init_param_float
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_init_param_float
    }
}

impl<'a> Extract<'a> for MissionInitParamFloat {
    const TYPE_NAME: &'static str = "MissionInitParamFloat";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `SpawnDescriptionEntry`
pub struct SpawnDescriptionEntry {
    /// `description` (String)
    pub description: String,
    /// `spawnGroup` (StrongPointer)
    pub spawn_group: Option<BaseMissionPropertyValuePtr>,
}

impl Pooled for SpawnDescriptionEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.spawn_description_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.spawn_description_entry
    }
}

impl<'a> Extract<'a> for SpawnDescriptionEntry {
    const TYPE_NAME: &'static str = "SpawnDescriptionEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst
                .get_str("description")
                .map(String::from)
                .unwrap_or_default(),
            spawn_group: match inst.get("spawnGroup") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BaseMissionPropertyValuePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SpawnDescriptions`
pub struct SpawnDescriptions {
    /// `spawnDescriptions` (Class (array))
    pub spawn_descriptions: Vec<Handle<SpawnDescriptionEntry>>,
}

impl Pooled for SpawnDescriptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.spawn_descriptions
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.spawn_descriptions
    }
}

impl<'a> Extract<'a> for SpawnDescriptions {
    const TYPE_NAME: &'static str = "SpawnDescriptions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spawn_descriptions: inst
                .get_array("spawnDescriptions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SpawnDescriptionEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SpawnDescriptionEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
