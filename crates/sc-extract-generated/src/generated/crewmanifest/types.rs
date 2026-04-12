// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `crewmanifest`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CrewManifest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewManifest {
    /// `descriptionTags` (Class)
    #[serde(default)]
    pub description_tags: Option<Handle<TagList>>,
    /// `crew` (Class (array))
    #[serde(default)]
    pub crew: Vec<Handle<CrewData>>,
}

impl Pooled for CrewManifest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crewmanifest.crew_manifest }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crewmanifest.crew_manifest }
}

impl<'a> Extract<'a> for CrewManifest {
    const TYPE_NAME: &'static str = "CrewManifest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description_tags: match inst.get("descriptionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            crew: inst.get_array("crew")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CrewData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CrewData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CrewData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewData {
    /// `entityClassDefinition` (Reference)
    #[serde(default)]
    pub entity_class_definition: Option<CigGuid>,
    /// `outfitTags` (Reference (array))
    #[serde(default)]
    pub outfit_tags: Vec<CigGuid>,
    /// `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// `dnfTags` (Class)
    #[serde(default)]
    pub dnf_tags: Option<Handle<TagsDNF>>,
    /// `archetype` (Reference)
    #[serde(default)]
    pub archetype: Option<CigGuid>,
}

impl Pooled for CrewData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.crewmanifest.crew_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.crewmanifest.crew_data }
}

impl<'a> Extract<'a> for CrewData {
    const TYPE_NAME: &'static str = "CrewData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_class_definition: inst.get("entityClassDefinition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            outfit_tags: inst.get_array("outfitTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            dnf_tags: match inst.get("dnfTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            archetype: inst.get("archetype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

