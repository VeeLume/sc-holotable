// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-basebuilding`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `BlueprintCategoryAvailability_Whitelist`
/// Inherits from: `BlueprintCategoryAvailability_Base_NonRef`
pub struct BlueprintCategoryAvailability_Whitelist {
    /// `categories` (Reference (array))
    pub categories: Vec<CigGuid>,
}

impl Pooled for BlueprintCategoryAvailability_Whitelist {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_basebuilding
            .blueprint_category_availability_whitelist
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_basebuilding
            .blueprint_category_availability_whitelist
    }
}

impl<'a> Extract<'a> for BlueprintCategoryAvailability_Whitelist {
    const TYPE_NAME: &'static str = "BlueprintCategoryAvailability_Whitelist";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst
                .get_array("categories")
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

/// DCB type: `CrafterDoorStateEvent`
/// Inherits from: `EventDispatcher`
pub struct CrafterDoorStateEvent {
    /// `doorState` (EnumChoice)
    pub door_state: ECraftingMachineDoorState,
}

impl Pooled for CrafterDoorStateEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_basebuilding.crafter_door_state_event
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_basebuilding.crafter_door_state_event
    }
}

impl<'a> Extract<'a> for CrafterDoorStateEvent {
    const TYPE_NAME: &'static str = "CrafterDoorStateEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            door_state: ECraftingMachineDoorState::from_dcb_str(
                inst.get_str("doorState").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `CraftingQueueCoreParams`
pub struct CraftingQueueCoreParams {
    /// `debugName` (String)
    pub debug_name: String,
    /// `blueprintCategoryAvailability` (StrongPointer)
    pub blueprint_category_availability: Option<BlueprintCategoryAvailability_BasePtr>,
    /// `processTypeAvailability` (Boolean (array))
    pub process_type_availability: Vec<bool>,
    /// `maxJobsInProgress` (Int32)
    pub max_jobs_in_progress: i32,
    /// `maxJobsWaiting` (Int32)
    pub max_jobs_waiting: i32,
}

impl Pooled for CraftingQueueCoreParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_basebuilding.crafting_queue_core_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_basebuilding.crafting_queue_core_params
    }
}

impl<'a> Extract<'a> for CraftingQueueCoreParams {
    const TYPE_NAME: &'static str = "CraftingQueueCoreParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            blueprint_category_availability: match inst.get("blueprintCategoryAvailability") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BlueprintCategoryAvailability_BasePtr::from_ref(b, r))
                }
                _ => None,
            },
            process_type_availability: inst
                .get_array("processTypeAvailability")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            max_jobs_in_progress: inst.get_i32("maxJobsInProgress").unwrap_or_default(),
            max_jobs_waiting: inst.get_i32("maxJobsWaiting").unwrap_or_default(),
        }
    }
}

/// DCB type: `CrafterInteractionParams`
pub struct CrafterInteractionParams {
    /// `openDoorInteraction` (WeakPointer)
    pub open_door_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `closeDoorInteraction` (WeakPointer)
    pub close_door_interaction: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for CrafterInteractionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_basebuilding.crafter_interaction_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_basebuilding.crafter_interaction_params
    }
}

impl<'a> Extract<'a> for CrafterInteractionParams {
    const TYPE_NAME: &'static str = "CrafterInteractionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            open_door_interaction: match inst.get("openDoorInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            close_door_interaction: match inst.get("closeDoorInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `CrafterComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct CrafterComponentParams {
    /// `queues` (Class (array))
    pub queues: Vec<Handle<CraftingQueueCoreParams>>,
    /// `maxJobsInHistory` (Int32)
    pub max_jobs_in_history: i32,
    /// `interactionParams` (Class)
    pub interaction_params: Option<Handle<CrafterInteractionParams>>,
}

impl Pooled for CrafterComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_basebuilding.crafter_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_basebuilding.crafter_component_params
    }
}

impl<'a> Extract<'a> for CrafterComponentParams {
    const TYPE_NAME: &'static str = "CrafterComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            queues: inst
                .get_array("queues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<CraftingQueueCoreParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<CraftingQueueCoreParams>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            max_jobs_in_history: inst.get_i32("maxJobsInHistory").unwrap_or_default(),
            interaction_params: match inst.get("interactionParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CrafterInteractionParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `CrafterPagedUIListParams`
pub struct CrafterPagedUIListParams {
    /// `numPagedElementsHorizontally` (Single)
    pub num_paged_elements_horizontally: f32,
    /// `numPagedElementsVertically` (Single)
    pub num_paged_elements_vertically: f32,
    /// `categoryHeaderVerticalSizeAsFractionOfElement` (Single)
    pub category_header_vertical_size_as_fraction_of_element: f32,
    /// `subCategoryHeaderVerticalSizeAsFractionOfElement` (Single)
    pub sub_category_header_vertical_size_as_fraction_of_element: f32,
}

impl Pooled for CrafterPagedUIListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_basebuilding.crafter_paged_uilist_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_basebuilding.crafter_paged_uilist_params
    }
}

impl<'a> Extract<'a> for CrafterPagedUIListParams {
    const TYPE_NAME: &'static str = "CrafterPagedUIListParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            num_paged_elements_horizontally: inst
                .get_f32("numPagedElementsHorizontally")
                .unwrap_or_default(),
            num_paged_elements_vertically: inst
                .get_f32("numPagedElementsVertically")
                .unwrap_or_default(),
            category_header_vertical_size_as_fraction_of_element: inst
                .get_f32("categoryHeaderVerticalSizeAsFractionOfElement")
                .unwrap_or_default(),
            sub_category_header_vertical_size_as_fraction_of_element: inst
                .get_f32("subCategoryHeaderVerticalSizeAsFractionOfElement")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CrafterUIProviderComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct CrafterUIProviderComponentParams {
    /// `blueprintPagedListParams` (Class)
    pub blueprint_paged_list_params: Option<Handle<CrafterPagedUIListParams>>,
    /// `materialsPagedListParams` (Class)
    pub materials_paged_list_params: Option<Handle<CrafterPagedUIListParams>>,
}

impl Pooled for CrafterUIProviderComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_basebuilding
            .crafter_uiprovider_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_basebuilding
            .crafter_uiprovider_component_params
    }
}

impl<'a> Extract<'a> for CrafterUIProviderComponentParams {
    const TYPE_NAME: &'static str = "CrafterUIProviderComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blueprint_paged_list_params: match inst.get("blueprintPagedListParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CrafterPagedUIListParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            materials_paged_list_params: match inst.get("materialsPagedListParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CrafterPagedUIListParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `CrafterStateModifier`
/// Inherits from: `SStateModifier`
pub struct CrafterStateModifier {
    /// `lockedState` (WeakPointer)
    pub locked_state: Option<Handle<SInteractionState>>,
    /// `unlockedState` (WeakPointer)
    pub unlocked_state: Option<Handle<SInteractionState>>,
    /// `startProgressState` (WeakPointer)
    pub start_progress_state: Option<Handle<SInteractionState>>,
    /// `finishProgressState` (WeakPointer)
    pub finish_progress_state: Option<Handle<SInteractionState>>,
}

impl Pooled for CrafterStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_basebuilding.crafter_state_modifier
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_basebuilding.crafter_state_modifier
    }
}

impl<'a> Extract<'a> for CrafterStateModifier {
    const TYPE_NAME: &'static str = "CrafterStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            locked_state: match inst.get("lockedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            unlocked_state: match inst.get("unlockedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            start_progress_state: match inst.get("startProgressState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            finish_progress_state: match inst.get("finishProgressState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}
