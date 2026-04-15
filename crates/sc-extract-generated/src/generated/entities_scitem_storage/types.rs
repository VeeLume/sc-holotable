// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-storage`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `STrackViewOutfitInteractionSwapData`
/// Inherits from: `SAnimatedOutfitSwapData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STrackViewOutfitInteractionSwapData {
    /// `interaction` (WeakPointer)
    #[serde(default)]
    pub interaction: Option<Handle<SSharedInteractionParams>>,
    /// `selectionTags` (Class)
    #[serde(default)]
    pub selection_tags: Option<Handle<TagList>>,
}

impl Pooled for STrackViewOutfitInteractionSwapData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_storage.strack_view_outfit_interaction_swap_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_storage.strack_view_outfit_interaction_swap_data }
}

impl<'a> Extract<'a> for STrackViewOutfitInteractionSwapData {
    const TYPE_NAME: &'static str = "STrackViewOutfitInteractionSwapData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interaction: match inst.get("interaction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            selection_tags: match inst.get("selectionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityComponentOutfitHangerParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityComponentOutfitHangerParams {
    /// `equipInteraction` (WeakPointer)
    #[serde(default)]
    pub equip_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `hangInteraction` (WeakPointer)
    #[serde(default)]
    pub hang_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapAllInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_all_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `equipAllInteraction` (WeakPointer)
    #[serde(default)]
    pub equip_all_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `hangAllInteraction` (WeakPointer)
    #[serde(default)]
    pub hang_all_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `disguiseSwapAllInteraction` (WeakPointer)
    #[serde(default)]
    pub disguise_swap_all_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `itemPortTypeSubtypes` (Class (array))
    #[serde(default)]
    pub item_port_type_subtypes: Vec<Handle<TypeSubtypeParams>>,
    /// `animatedOutfitSwap` (StrongPointer)
    #[serde(default)]
    pub animated_outfit_swap: Option<SAnimatedOutfitSwapDataPtr>,
    /// `animatedOutfitHang` (StrongPointer)
    #[serde(default)]
    pub animated_outfit_hang: Option<SAnimatedOutfitSwapDataPtr>,
    /// `destroyPlayerItems` (Boolean)
    #[serde(default)]
    pub destroy_player_items: bool,
}

impl Pooled for SEntityComponentOutfitHangerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_storage.sentity_component_outfit_hanger_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_storage.sentity_component_outfit_hanger_params }
}

impl<'a> Extract<'a> for SEntityComponentOutfitHangerParams {
    const TYPE_NAME: &'static str = "SEntityComponentOutfitHangerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            equip_interaction: match inst.get("equipInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hang_interaction: match inst.get("hangInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_interaction: match inst.get("swapInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_all_interaction: match inst.get("swapAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            equip_all_interaction: match inst.get("equipAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hang_all_interaction: match inst.get("hangAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disguise_swap_all_interaction: match inst.get("disguiseSwapAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_port_type_subtypes: inst.get_array("itemPortTypeSubtypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TypeSubtypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TypeSubtypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            animated_outfit_swap: match inst.get("animatedOutfitSwap") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SAnimatedOutfitSwapDataPtr::from_ref(b, r)),
                _ => None,
            },
            animated_outfit_hang: match inst.get("animatedOutfitHang") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SAnimatedOutfitSwapDataPtr::from_ref(b, r)),
                _ => None,
            },
            destroy_player_items: inst.get_bool("destroyPlayerItems").unwrap_or_default(),
        }
    }
}

/// DCB type: `SFlightsuitHangerGroup`
/// Inherits from: `SOutfitHangerGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFlightsuitHangerGroup {
    /// `groupPorts` (WeakPointer (array))
    #[serde(default)]
    pub group_ports: Vec<Handle<SItemPortDef>>,
    /// `equipAllInteraction` (WeakPointer)
    #[serde(default)]
    pub equip_all_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapAllInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_all_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `armorPorts` (WeakPointer (array))
    #[serde(default)]
    pub armor_ports: Vec<Handle<SItemPortDef>>,
    /// `swapAllClothesInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_all_clothes_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapAllUndersuitInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_all_undersuit_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapAllHelmetInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_all_helmet_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapAllCoreInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_all_core_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapUndersuitClothesInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_undersuit_clothes_interaction: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for SFlightsuitHangerGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_storage.sflightsuit_hanger_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_storage.sflightsuit_hanger_group }
}

impl<'a> Extract<'a> for SFlightsuitHangerGroup {
    const TYPE_NAME: &'static str = "SFlightsuitHangerGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            group_ports: inst.get_array("groupPorts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            equip_all_interaction: match inst.get("equipAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_all_interaction: match inst.get("swapAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            armor_ports: inst.get_array("armorPorts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            swap_all_clothes_interaction: match inst.get("swapAllClothesInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_all_undersuit_interaction: match inst.get("swapAllUndersuitInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_all_helmet_interaction: match inst.get("swapAllHelmetInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_all_core_interaction: match inst.get("swapAllCoreInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_undersuit_clothes_interaction: match inst.get("swapUndersuitClothesInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SMannequinHangerGroup`
/// Inherits from: `SOutfitHangerGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMannequinHangerGroup {
    /// `groupPorts` (WeakPointer (array))
    #[serde(default)]
    pub group_ports: Vec<Handle<SItemPortDef>>,
    /// `equipAllInteraction` (WeakPointer)
    #[serde(default)]
    pub equip_all_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `swapAllInteraction` (WeakPointer)
    #[serde(default)]
    pub swap_all_interaction: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for SMannequinHangerGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_storage.smannequin_hanger_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_storage.smannequin_hanger_group }
}

impl<'a> Extract<'a> for SMannequinHangerGroup {
    const TYPE_NAME: &'static str = "SMannequinHangerGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            group_ports: inst.get_array("groupPorts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            equip_all_interaction: match inst.get("equipAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swap_all_interaction: match inst.get("swapAllInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

