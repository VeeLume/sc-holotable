// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `resourcetypedatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ResourceTypeProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeProperties {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for ResourceTypeProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.resourcetypedatabase.resource_type_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.resourcetypedatabase.resource_type_properties }
}

impl<'a> Extract<'a> for ResourceTypeProperties {
    const TYPE_NAME: &'static str = "ResourceTypeProperties";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ResourceTypeDensityType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeDensityType {
}

impl Pooled for ResourceTypeDensityType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.resourcetypedatabase.resource_type_density_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.resourcetypedatabase.resource_type_density_type }
}

impl<'a> Extract<'a> for ResourceTypeDensityType {
    const TYPE_NAME: &'static str = "ResourceTypeDensityType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ResourceType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceType {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `densityType` (StrongPointer)
    #[serde(default)]
    pub density_type: Option<Handle<ResourceTypeDensityType>>,
    /// `defaultThumbnailPath` (String)
    #[serde(default)]
    pub default_thumbnail_path: String,
    /// `defaultThumbnailPathSVG` (String)
    #[serde(default)]
    pub default_thumbnail_path_svg: String,
    /// `rttThumbnailEntityClass` (Reference)
    #[serde(default)]
    pub rtt_thumbnail_entity_class: Option<CigGuid>,
    /// `properties` (StrongPointer (array))
    #[serde(default)]
    pub properties: Vec<Handle<ResourceTypeProperties>>,
    /// `refinedVersion` (Reference)
    #[serde(default)]
    pub refined_version: Option<CigGuid>,
    /// `validateDefaultCargoBox` (Boolean)
    #[serde(default)]
    pub validate_default_cargo_box: bool,
    /// `defaultCargoContainers` (StrongPointer)
    #[serde(default)]
    pub default_cargo_containers: Option<Handle<SResourceTypeDefaultCargoContainers>>,
}

impl Pooled for ResourceType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.resourcetypedatabase.resource_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.resourcetypedatabase.resource_type }
}

impl<'a> Extract<'a> for ResourceType {
    const TYPE_NAME: &'static str = "ResourceType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            density_type: match inst.get("densityType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ResourceTypeDensityType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ResourceTypeDensityType>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_thumbnail_path: inst.get_str("defaultThumbnailPath").map(String::from).unwrap_or_default(),
            default_thumbnail_path_svg: inst.get_str("defaultThumbnailPathSVG").map(String::from).unwrap_or_default(),
            rtt_thumbnail_entity_class: inst.get("rttThumbnailEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ResourceTypeProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ResourceTypeProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            refined_version: inst.get("refinedVersion").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            validate_default_cargo_box: inst.get_bool("validateDefaultCargoBox").unwrap_or_default(),
            default_cargo_containers: match inst.get("defaultCargoContainers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SResourceTypeDefaultCargoContainers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SResourceTypeDefaultCargoContainers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ResourceTypeGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeGroup {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `defaultThumbnailPath` (String)
    #[serde(default)]
    pub default_thumbnail_path: String,
    /// `defaultThumbnailPathSVG` (String)
    #[serde(default)]
    pub default_thumbnail_path_svg: String,
    /// `groups` (Reference (array))
    #[serde(default)]
    pub groups: Vec<CigGuid>,
    /// `resources` (Reference (array))
    #[serde(default)]
    pub resources: Vec<CigGuid>,
}

impl Pooled for ResourceTypeGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.resourcetypedatabase.resource_type_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.resourcetypedatabase.resource_type_group }
}

impl<'a> Extract<'a> for ResourceTypeGroup {
    const TYPE_NAME: &'static str = "ResourceTypeGroup";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            default_thumbnail_path: inst.get_str("defaultThumbnailPath").map(String::from).unwrap_or_default(),
            default_thumbnail_path_svg: inst.get_str("defaultThumbnailPathSVG").map(String::from).unwrap_or_default(),
            groups: inst.get_array("groups")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            resources: inst.get_array("resources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ResourceTypeDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeDatabase {
    /// `groups` (Reference (array))
    #[serde(default)]
    pub groups: Vec<CigGuid>,
}

impl Pooled for ResourceTypeDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.resourcetypedatabase.resource_type_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.resourcetypedatabase.resource_type_database }
}

impl<'a> Extract<'a> for ResourceTypeDatabase {
    const TYPE_NAME: &'static str = "ResourceTypeDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            groups: inst.get_array("groups")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

