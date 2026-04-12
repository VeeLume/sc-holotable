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

/// DCB type: `SkillDefinitions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinitions {
    /// DCB field: `descriptionTags` (Class)
    #[serde(default)]
    pub description_tags: Option<Handle<TagList>>,
    /// DCB field: `traits` (Class)
    #[serde(default)]
    pub traits: Option<Handle<TagList>>,
    /// DCB field: `skills` (Class (array))
    #[serde(default)]
    pub skills: Vec<Handle<Skill>>,
}

impl Pooled for SkillDefinitions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sk.skill_definitions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sk.skill_definitions }
}

impl<'a> Extract<'a> for SkillDefinitions {
    const TYPE_NAME: &'static str = "SkillDefinitions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description_tags: match inst.get("descriptionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            traits: match inst.get("traits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            skills: inst.get_array("skills")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Skill>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Skill>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Skill`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// DCB field: `isSkill` (Boolean)
    #[serde(default)]
    pub is_skill: bool,
    /// DCB field: `optional` (Boolean)
    #[serde(default)]
    pub optional: bool,
    /// DCB field: `skillType` (EnumChoice)
    #[serde(default)]
    pub skill_type: String,
    /// DCB field: `skillLevel` (Single)
    #[serde(default)]
    pub skill_level: f32,
    /// DCB field: `skillTag` (Reference)
    #[serde(default)]
    pub skill_tag: Option<CigGuid>,
    /// DCB field: `categoryName` (String)
    #[serde(default)]
    pub category_name: String,
    /// DCB field: `skills` (Class (array))
    #[serde(default)]
    pub skills: Vec<Handle<Skill>>,
}

impl Pooled for Skill {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sk.skill }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sk.skill }
}

impl<'a> Extract<'a> for Skill {
    const TYPE_NAME: &'static str = "Skill";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            is_skill: inst.get_bool("isSkill").unwrap_or_default(),
            optional: inst.get_bool("optional").unwrap_or_default(),
            skill_type: inst.get_str("skillType").map(String::from).unwrap_or_default(),
            skill_level: inst.get_f32("skillLevel").unwrap_or_default(),
            skill_tag: inst.get("skillTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            category_name: inst.get_str("categoryName").map(String::from).unwrap_or_default(),
            skills: inst.get_array("skills")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Skill>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Skill>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SkinInteractableTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInteractableTemplate {
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `InteractionPoints` (Class (array))
    #[serde(default)]
    pub interaction_points: Vec<Handle<SInteractionPointParams>>,
}

impl Pooled for SkinInteractableTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sk.skin_interactable_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sk.skin_interactable_template }
}

impl<'a> Extract<'a> for SkinInteractableTemplate {
    const TYPE_NAME: &'static str = "SkinInteractableTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            interaction_points: inst.get_array("InteractionPoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SInteractionPointParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SInteractionPointParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SkinInteractableTemplates`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInteractableTemplates {
    /// DCB field: `Templates` (Class (array))
    #[serde(default)]
    pub templates: Vec<Handle<SkinInteractableTemplate>>,
}

impl Pooled for SkinInteractableTemplates {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sk.skin_interactable_templates }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sk.skin_interactable_templates }
}

impl<'a> Extract<'a> for SkinInteractableTemplates {
    const TYPE_NAME: &'static str = "SkinInteractableTemplates";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            templates: inst.get_array("Templates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SkinInteractableTemplate>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SkinInteractableTemplate>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

