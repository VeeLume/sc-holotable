// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `scitemcommscomponentsetup`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCItemCommsComponentSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemCommsComponentSetup {
    /// `showInContactsList` (Boolean)
    #[serde(default)]
    pub show_in_contacts_list: bool,
    /// `broadcastRange` (Single)
    #[serde(default)]
    pub broadcast_range: f32,
    /// `startIncomingCallSound` (Class)
    #[serde(default)]
    pub start_incoming_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// `stopIncomingCallSound` (Class)
    #[serde(default)]
    pub stop_incoming_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// `startOutgoingCallSound` (Class)
    #[serde(default)]
    pub start_outgoing_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// `stopOutgoingCallSound` (Class)
    #[serde(default)]
    pub stop_outgoing_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// `introTrigger` (Class)
    #[serde(default)]
    pub intro_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `outroTrigger` (Class)
    #[serde(default)]
    pub outro_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `joinTrigger` (Class)
    #[serde(default)]
    pub join_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `leaveTrigger` (Class)
    #[serde(default)]
    pub leave_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `renderParticles` (Boolean)
    #[serde(default)]
    pub render_particles: bool,
}

impl Pooled for SCItemCommsComponentSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.scitemcommscomponentsetup.scitem_comms_component_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.scitemcommscomponentsetup.scitem_comms_component_setup }
}

impl<'a> Extract<'a> for SCItemCommsComponentSetup {
    const TYPE_NAME: &'static str = "SCItemCommsComponentSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            show_in_contacts_list: inst.get_bool("showInContactsList").unwrap_or_default(),
            broadcast_range: inst.get_f32("broadcastRange").unwrap_or_default(),
            start_incoming_call_sound: match inst.get("startIncomingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_incoming_call_sound: match inst.get("stopIncomingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            start_outgoing_call_sound: match inst.get("startOutgoingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_outgoing_call_sound: match inst.get("stopOutgoingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            intro_trigger: match inst.get("introTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outro_trigger: match inst.get("outroTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            join_trigger: match inst.get("joinTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            leave_trigger: match inst.get("leaveTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            render_particles: inst.get_bool("renderParticles").unwrap_or_default(),
        }
    }
}

