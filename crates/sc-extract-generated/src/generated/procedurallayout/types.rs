// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `procedurallayout`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `FactionPalettes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionPalettes {
    /// `Palettes` (Reference (array))
    #[serde(default)]
    pub palettes: Vec<CigGuid>,
}

impl Pooled for FactionPalettes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.faction_palettes }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.faction_palettes }
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
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `tintingActive` (Boolean)
    #[serde(default)]
    pub tinting_active: bool,
    /// `ExteriorMaterialOverride` (Class)
    #[serde(default)]
    pub exterior_material_override: Option<Handle<GlobalResourceMaterial>>,
    /// `InteriorMaterialOverride` (Class)
    #[serde(default)]
    pub interior_material_override: Option<Handle<GlobalResourceMaterial>>,
    /// `BrandingMaterialOverride` (Class)
    #[serde(default)]
    pub branding_material_override: Option<Handle<GlobalResourceMaterial>>,
    /// `ExteriorPrimaryColor` (Class)
    #[serde(default)]
    pub exterior_primary_color: Option<Handle<SRGB8>>,
    /// `ExteriorSecondaryColor` (Class)
    #[serde(default)]
    pub exterior_secondary_color: Option<Handle<SRGB8>>,
    /// `ExteriorTertiaryColor` (Class)
    #[serde(default)]
    pub exterior_tertiary_color: Option<Handle<SRGB8>>,
    /// `ExteriorGraphicsColor` (Class)
    #[serde(default)]
    pub exterior_graphics_color: Option<Handle<SRGB8>>,
    /// `InteriorPrimaryColor` (Class)
    #[serde(default)]
    pub interior_primary_color: Option<Handle<SRGB8>>,
    /// `InteriorSecondaryColor` (Class)
    #[serde(default)]
    pub interior_secondary_color: Option<Handle<SRGB8>>,
    /// `InteriorTertiaryColor` (Class)
    #[serde(default)]
    pub interior_tertiary_color: Option<Handle<SRGB8>>,
    /// `InteriorGraphicsColor` (Class)
    #[serde(default)]
    pub interior_graphics_color: Option<Handle<SRGB8>>,
}

impl Pooled for FactionPalette {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.faction_palette }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.faction_palette }
}

impl<'a> Extract<'a> for FactionPalette {
    const TYPE_NAME: &'static str = "FactionPalette";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tinting_active: inst.get_bool("tintingActive").unwrap_or_default(),
            exterior_material_override: match inst.get("ExteriorMaterialOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            interior_material_override: match inst.get("InteriorMaterialOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            branding_material_override: match inst.get("BrandingMaterialOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exterior_primary_color: match inst.get("ExteriorPrimaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exterior_secondary_color: match inst.get("ExteriorSecondaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exterior_tertiary_color: match inst.get("ExteriorTertiaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exterior_graphics_color: match inst.get("ExteriorGraphicsColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            interior_primary_color: match inst.get("InteriorPrimaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            interior_secondary_color: match inst.get("InteriorSecondaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            interior_tertiary_color: match inst.get("InteriorTertiaryColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            interior_graphics_color: match inst.get("InteriorGraphicsColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralLayoutNode_Start`
/// Inherits from: `ProceduralLayoutNode_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutNode_Start {
    /// `next` (WeakPointer)
    #[serde(default)]
    pub next: Option<ProceduralLayoutNode_BasePtr>,
}

impl Pooled for ProceduralLayoutNode_Start {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.procedural_layout_node_start }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.procedural_layout_node_start }
}

impl<'a> Extract<'a> for ProceduralLayoutNode_Start {
    const TYPE_NAME: &'static str = "ProceduralLayoutNode_Start";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            next: match inst.get("next") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ProceduralLayoutNode_BasePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralLayoutNode_ElementProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutNode_ElementProperties {
    /// `ElementTags` (Reference (array))
    #[serde(default)]
    pub element_tags: Vec<CigGuid>,
    /// `ElementThemes` (Reference (array))
    #[serde(default)]
    pub element_themes: Vec<CigGuid>,
}

impl Pooled for ProceduralLayoutNode_ElementProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.procedural_layout_node_element_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.procedural_layout_node_element_properties }
}

impl<'a> Extract<'a> for ProceduralLayoutNode_ElementProperties {
    const TYPE_NAME: &'static str = "ProceduralLayoutNode_ElementProperties";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            element_tags: inst.get_array("ElementTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            element_themes: inst.get_array("ElementThemes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLayoutGraphNode_Element`
/// Inherits from: `ProceduralLayoutGraphNode_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutGraphNode_Element {
    /// `MinDistance` (Int32)
    #[serde(default)]
    pub min_distance: i32,
    /// `MaxDistance` (Int32)
    #[serde(default)]
    pub max_distance: i32,
    /// `SpecificRoutingElementsTags` (Class (array))
    #[serde(default)]
    pub specific_routing_elements_tags: Vec<Handle<TagList>>,
    /// `ChanceOfDoor` (Single)
    #[serde(default)]
    pub chance_of_door: f32,
    /// `ChanceOfGeneration` (Single)
    #[serde(default)]
    pub chance_of_generation: f32,
    /// `Mandatory` (Boolean)
    #[serde(default)]
    pub mandatory: bool,
    /// `LayerSuffix` (String)
    #[serde(default)]
    pub layer_suffix: String,
    /// `TintPalettePath` (String)
    #[serde(default)]
    pub tint_palette_path: String,
    /// `outputLinks` (WeakPointer (array))
    #[serde(default)]
    pub output_links: Vec<ProceduralLayoutNode_BasePtr>,
    /// `ElementProperties` (Class)
    #[serde(default)]
    pub element_properties: Option<Handle<ProceduralLayoutNode_ElementProperties>>,
}

impl Pooled for ProceduralLayoutGraphNode_Element {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.procedural_layout_graph_node_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.procedural_layout_graph_node_element }
}

impl<'a> Extract<'a> for ProceduralLayoutGraphNode_Element {
    const TYPE_NAME: &'static str = "ProceduralLayoutGraphNode_Element";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_distance: inst.get_i32("MinDistance").unwrap_or_default(),
            max_distance: inst.get_i32("MaxDistance").unwrap_or_default(),
            specific_routing_elements_tags: inst.get_array("SpecificRoutingElementsTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            chance_of_door: inst.get_f32("ChanceOfDoor").unwrap_or_default(),
            chance_of_generation: inst.get_f32("ChanceOfGeneration").unwrap_or_default(),
            mandatory: inst.get_bool("Mandatory").unwrap_or_default(),
            layer_suffix: inst.get_str("LayerSuffix").map(String::from).unwrap_or_default(),
            tint_palette_path: inst.get_str("TintPalettePath").map(String::from).unwrap_or_default(),
            output_links: inst.get_array("outputLinks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(ProceduralLayoutNode_BasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            element_properties: match inst.get("ElementProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralLayoutNode_ElementProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralLayoutGraphNode_MultiElement`
/// Inherits from: `ProceduralLayoutGraphNode_Element`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutGraphNode_MultiElement {
    /// `MinDistance` (Int32)
    #[serde(default)]
    pub min_distance: i32,
    /// `MaxDistance` (Int32)
    #[serde(default)]
    pub max_distance: i32,
    /// `SpecificRoutingElementsTags` (Class (array))
    #[serde(default)]
    pub specific_routing_elements_tags: Vec<Handle<TagList>>,
    /// `ChanceOfDoor` (Single)
    #[serde(default)]
    pub chance_of_door: f32,
    /// `ChanceOfGeneration` (Single)
    #[serde(default)]
    pub chance_of_generation: f32,
    /// `Mandatory` (Boolean)
    #[serde(default)]
    pub mandatory: bool,
    /// `LayerSuffix` (String)
    #[serde(default)]
    pub layer_suffix: String,
    /// `TintPalettePath` (String)
    #[serde(default)]
    pub tint_palette_path: String,
    /// `outputLinks` (WeakPointer (array))
    #[serde(default)]
    pub output_links: Vec<ProceduralLayoutNode_BasePtr>,
    /// `ElementProperties` (Class)
    #[serde(default)]
    pub element_properties: Option<Handle<ProceduralLayoutNode_ElementProperties>>,
    /// `MinElementsToGenerate` (Int32)
    #[serde(default)]
    pub min_elements_to_generate: i32,
    /// `MaxElementsToGenerate` (Int32)
    #[serde(default)]
    pub max_elements_to_generate: i32,
    /// `GenerateUniqueElements` (Boolean)
    #[serde(default)]
    pub generate_unique_elements: bool,
}

impl Pooled for ProceduralLayoutGraphNode_MultiElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.procedural_layout_graph_node_multi_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.procedural_layout_graph_node_multi_element }
}

impl<'a> Extract<'a> for ProceduralLayoutGraphNode_MultiElement {
    const TYPE_NAME: &'static str = "ProceduralLayoutGraphNode_MultiElement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_distance: inst.get_i32("MinDistance").unwrap_or_default(),
            max_distance: inst.get_i32("MaxDistance").unwrap_or_default(),
            specific_routing_elements_tags: inst.get_array("SpecificRoutingElementsTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            chance_of_door: inst.get_f32("ChanceOfDoor").unwrap_or_default(),
            chance_of_generation: inst.get_f32("ChanceOfGeneration").unwrap_or_default(),
            mandatory: inst.get_bool("Mandatory").unwrap_or_default(),
            layer_suffix: inst.get_str("LayerSuffix").map(String::from).unwrap_or_default(),
            tint_palette_path: inst.get_str("TintPalettePath").map(String::from).unwrap_or_default(),
            output_links: inst.get_array("outputLinks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(ProceduralLayoutNode_BasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            element_properties: match inst.get("ElementProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralLayoutNode_ElementProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            min_elements_to_generate: inst.get_i32("MinElementsToGenerate").unwrap_or_default(),
            max_elements_to_generate: inst.get_i32("MaxElementsToGenerate").unwrap_or_default(),
            generate_unique_elements: inst.get_bool("GenerateUniqueElements").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLayout_SupplementaryElementTagsOptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayout_SupplementaryElementTagsOptions {
    /// `SupplementaryElementTags` (Reference (array))
    #[serde(default)]
    pub supplementary_element_tags: Vec<CigGuid>,
    /// `MaxElementsToGenerate` (Int32)
    #[serde(default)]
    pub max_elements_to_generate: i32,
}

impl Pooled for ProceduralLayout_SupplementaryElementTagsOptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.procedural_layout_supplementary_element_tags_options }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.procedural_layout_supplementary_element_tags_options }
}

impl<'a> Extract<'a> for ProceduralLayout_SupplementaryElementTagsOptions {
    const TYPE_NAME: &'static str = "ProceduralLayout_SupplementaryElementTagsOptions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            supplementary_element_tags: inst.get_array("SupplementaryElementTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            max_elements_to_generate: inst.get_i32("MaxElementsToGenerate").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLayout_TagFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayout_TagFilter {
    /// `GraphNodeTagsToFilter` (Class (array))
    #[serde(default)]
    pub graph_node_tags_to_filter: Vec<Handle<TagList>>,
    /// `TagFilteringMode` (EnumChoice)
    #[serde(default)]
    pub tag_filtering_mode: ProceduralLayout_TagFilteringMode,
    /// `SupplementaryElementTagsList` (Class (array))
    #[serde(default)]
    pub supplementary_element_tags_list: Vec<Handle<ProceduralLayout_SupplementaryElementTagsOptions>>,
}

impl Pooled for ProceduralLayout_TagFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.procedural_layout_tag_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.procedural_layout_tag_filter }
}

impl<'a> Extract<'a> for ProceduralLayout_TagFilter {
    const TYPE_NAME: &'static str = "ProceduralLayout_TagFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            graph_node_tags_to_filter: inst.get_array("GraphNodeTagsToFilter")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            tag_filtering_mode: ProceduralLayout_TagFilteringMode::from_dcb_str(inst.get_str("TagFilteringMode").unwrap_or("")),
            supplementary_element_tags_list: inst.get_array("SupplementaryElementTagsList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLayout_SupplementaryElementTagsOptions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ProceduralLayout_SupplementaryElementTagsOptions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLayoutGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutGraph {
    /// `ElementsLibraries` (String (array))
    #[serde(default)]
    pub elements_libraries: Vec<String>,
    /// `RoutingElementsLibraries` (String (array))
    #[serde(default)]
    pub routing_elements_libraries: Vec<String>,
    /// `ConnectorsLibraries` (String (array))
    #[serde(default)]
    pub connectors_libraries: Vec<String>,
    /// `CapsLibraries` (String (array))
    #[serde(default)]
    pub caps_libraries: Vec<String>,
    /// `SecondaryElementsLibraries` (String (array))
    #[serde(default)]
    pub secondary_elements_libraries: Vec<String>,
    /// `DefaultRoutingElementsTags` (Class (array))
    #[serde(default)]
    pub default_routing_elements_tags: Vec<Handle<TagList>>,
    /// `GlobalTagFiltering` (Class (array))
    #[serde(default)]
    pub global_tag_filtering: Vec<Handle<ProceduralLayout_TagFilter>>,
    /// `GlobalAddOnElementsTags` (Class (array))
    #[serde(default)]
    pub global_add_on_elements_tags: Vec<Handle<TagList>>,
    /// `GlobalAddOnElementsGenerationChance` (Single)
    #[serde(default)]
    pub global_add_on_elements_generation_chance: f32,
    /// `TintPalettePath` (String)
    #[serde(default)]
    pub tint_palette_path: String,
    /// `Nodes` (StrongPointer (array))
    #[serde(default)]
    pub nodes: Vec<ProceduralLayoutNode_BasePtr>,
}

impl Pooled for ProceduralLayoutGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.procedurallayout.procedural_layout_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.procedurallayout.procedural_layout_graph }
}

impl<'a> Extract<'a> for ProceduralLayoutGraph {
    const TYPE_NAME: &'static str = "ProceduralLayoutGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            elements_libraries: inst.get_array("ElementsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            routing_elements_libraries: inst.get_array("RoutingElementsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            connectors_libraries: inst.get_array("ConnectorsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            caps_libraries: inst.get_array("CapsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            secondary_elements_libraries: inst.get_array("SecondaryElementsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            default_routing_elements_tags: inst.get_array("DefaultRoutingElementsTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_tag_filtering: inst.get_array("GlobalTagFiltering")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLayout_TagFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ProceduralLayout_TagFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_add_on_elements_tags: inst.get_array("GlobalAddOnElementsTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_add_on_elements_generation_chance: inst.get_f32("GlobalAddOnElementsGenerationChance").unwrap_or_default(),
            tint_palette_path: inst.get_str("TintPalettePath").map(String::from).unwrap_or_default(),
            nodes: inst.get_array("Nodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(ProceduralLayoutNode_BasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

