// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `weaponarmodifiers`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `WeaponARModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponARModifier {
    /// `arMuzzleFlashEffect` (Class)
    #[serde(default)]
    pub ar_muzzle_flash_effect: Option<Handle<GlobalResourceParticle>>,
    /// `arMFXImpact` (String)
    #[serde(default)]
    pub ar_mfximpact: String,
    /// `arTriggerTag` (Reference)
    #[serde(default)]
    pub ar_trigger_tag: Option<CigGuid>,
    /// `weaponModifier` (Class)
    #[serde(default)]
    pub weapon_modifier: Option<Handle<SWeaponModifierParams>>,
}

impl Pooled for WeaponARModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponarmodifiers.weapon_armodifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponarmodifiers.weapon_armodifier }
}

impl<'a> Extract<'a> for WeaponARModifier {
    const TYPE_NAME: &'static str = "WeaponARModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ar_muzzle_flash_effect: match inst.get("arMuzzleFlashEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ar_mfximpact: inst.get_str("arMFXImpact").map(String::from).unwrap_or_default(),
            ar_trigger_tag: inst.get("arTriggerTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weapon_modifier: match inst.get("weaponModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponModifierParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponModifierParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

