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

/// DCB type: `MissionLocationTemplate`
pub struct MissionLocationTemplate {
    /// `locationData` (Class)
    pub location_data: Option<Handle<MissionLocationData>>,
}

impl Pooled for MissionLocationTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.mission_location_template
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.mission_location_template
    }
}

impl<'a> Extract<'a> for MissionLocationTemplate {
    const TYPE_NAME: &'static str = "MissionLocationTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_data: match inst.get("locationData") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MissionLocationData>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
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

/// DCB type: `LocationResourceSlot`
pub struct LocationResourceSlot {
    /// `resourceSlotTag` (Reference)
    pub resource_slot_tag: Option<CigGuid>,
    /// `autoConsumeIfPlayerAtLocation` (Boolean)
    pub auto_consume_if_player_at_location: bool,
}

impl Pooled for LocationResourceSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.location_resource_slot
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.location_resource_slot
    }
}

impl<'a> Extract<'a> for LocationResourceSlot {
    const TYPE_NAME: &'static str = "LocationResourceSlot";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource_slot_tag: inst
                .get("resourceSlotTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            auto_consume_if_player_at_location: inst
                .get_bool("autoConsumeIfPlayerAtLocation")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationEntityType_StaticEntity`
/// Inherits from: `LocationEntityType_Base`
pub struct LocationEntityType_StaticEntity {}

impl Pooled for LocationEntityType_StaticEntity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.location_entity_type_static_entity
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.location_entity_type_static_entity
    }
}

impl<'a> Extract<'a> for LocationEntityType_StaticEntity {
    const TYPE_NAME: &'static str = "LocationEntityType_StaticEntity";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `ModuleLocationEntities`
pub struct ModuleLocationEntities {
    /// `debugName` (String)
    pub debug_name: String,
    /// `optional` (Boolean)
    pub optional: bool,
}

impl Pooled for ModuleLocationEntities {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.module_location_entities
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.module_location_entities
    }
}

impl<'a> Extract<'a> for ModuleLocationEntities {
    const TYPE_NAME: &'static str = "ModuleLocationEntities";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            optional: inst.get_bool("optional").unwrap_or_default(),
        }
    }
}

/// DCB type: `ModuleLocationEntities_Static`
/// Inherits from: `ModuleLocationEntities`
pub struct ModuleLocationEntities_Static {
    /// `debugName` (String)
    pub debug_name: String,
    /// `optional` (Boolean)
    pub optional: bool,
    /// `entityDeclarations` (Reference (array))
    pub entity_declarations: Vec<CigGuid>,
}

impl Pooled for ModuleLocationEntities_Static {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.module_location_entities_static
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.module_location_entities_static
    }
}

impl<'a> Extract<'a> for ModuleLocationEntities_Static {
    const TYPE_NAME: &'static str = "ModuleLocationEntities_Static";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            optional: inst.get_bool("optional").unwrap_or_default(),
            entity_declarations: inst
                .get_array("entityDeclarations")
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

/// DCB type: `ModuleDeclarationType_Mission`
/// Inherits from: `ModuleDeclarationType_Base`
pub struct ModuleDeclarationType_Mission {
    /// `locationEntities` (StrongPointer (array))
    pub location_entities: Vec<ModuleLocationEntitiesPtr>,
    /// `requiredLocationModules` (Reference (array))
    pub required_location_modules: Vec<CigGuid>,
    /// `requiredResourceSlot` (Reference)
    pub required_resource_slot: Option<CigGuid>,
}

impl Pooled for ModuleDeclarationType_Mission {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.module_declaration_type_mission
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.module_declaration_type_mission
    }
}

impl<'a> Extract<'a> for ModuleDeclarationType_Mission {
    const TYPE_NAME: &'static str = "ModuleDeclarationType_Mission";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_entities: inst
                .get_array("locationEntities")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(ModuleLocationEntitiesPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            required_location_modules: inst
                .get_array("requiredLocationModules")
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
            required_resource_slot: inst
                .get("requiredResourceSlot")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
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

/// DCB type: `ModuleDeclaration`
pub struct ModuleDeclaration {
    /// `module` (String)
    pub module: String,
    /// `moduleType` (StrongPointer)
    pub module_type: Option<ModuleDeclarationType_BasePtr>,
    /// `properties` (Class (array))
    pub properties: Vec<Handle<MissionProperty>>,
}

impl Pooled for ModuleDeclaration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missiondata.module_declaration
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missiondata.module_declaration
    }
}

impl<'a> Extract<'a> for ModuleDeclaration {
    const TYPE_NAME: &'static str = "ModuleDeclaration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            module: inst.get_str("module").map(String::from).unwrap_or_default(),
            module_type: match inst.get("moduleType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ModuleDeclarationType_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
            properties: inst
                .get_array("properties")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<MissionProperty>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<MissionProperty>(
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
