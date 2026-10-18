//! Item-port traversing modifiers: items that change other items on the
//! vehicle (the Constellation Mk V's engineering buff multiplies its guns'
//! capacitors by 9).

use std::collections::HashMap;

use sc_extract::generated::{
    ActivationMethod, BaseItemModifierParamsPtr, DataForgeComponentParamsPtr, EItemType,
    EMasterMode,
};
use sc_extract::{DataPools, Datacore, Guid, RecordCollection};
use sc_weapons::WeaponPower;
use serde::{Deserialize, Serialize};

use crate::network::item_type_serde;
use crate::{Allocation, Mounted, PowerNetwork, Vehicles, network_items};

/// One `ItemportTraversingModifiersParams` of an item's
/// `EntityComponentAttachableModifierParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemportModifier {
    /// `targetItemType`: the items it changes.
    #[serde(with = "item_type_serde")]
    pub target: EItemType,
    /// `activationMethod` is `ActivateOnAttach`: it holds while the item
    /// is mounted (otherwise it needs an interaction).
    pub on_attach: bool,
    /// `weaponStats.regenModifier` of its weapon modifiers.
    pub regen: Option<RegenModifier>,
}

/// `SRegenConsumerModifier`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RegenModifier {
    pub power_ratio_multiplier: f32,
    pub max_ammo_load_multiplier: f32,
    pub max_regen_multiplier: f32,
}

/// Every entity's [`ItemportModifier`]s, keyed by `EntityClassDefinition`
/// GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemportModifiers {
    by_guid: HashMap<Guid, Vec<ItemportModifier>>,
}

impl ItemportModifiers {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut by_guid = HashMap::new();
        for (&guid, &handle) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let found: Vec<ItemportModifier> = ecd
                .components
                .iter()
                .filter_map(|c| match c {
                    DataForgeComponentParamsPtr::EntityComponentAttachableModifierParams(h) => {
                        h.get(pools)
                    }
                    _ => None,
                })
                .flat_map(|m| {
                    let on_attach = m.activation_method == ActivationMethod::ActivateOnAttach;
                    m.modifiers
                        .iter()
                        .filter_map(move |p| traversing(p, on_attach, pools))
                })
                .collect();
            if !found.is_empty() {
                by_guid.insert(guid, found);
            }
        }
        Self { by_guid }
    }
}

impl RecordCollection for ItemportModifiers {
    type Item = Vec<ItemportModifier>;

    fn get(&self, guid: &Guid) -> Option<&Vec<ItemportModifier>> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Vec<ItemportModifier>)> + '_ {
        self.by_guid.iter()
    }
}

fn traversing(
    p: &BaseItemModifierParamsPtr,
    on_attach: bool,
    pools: &DataPools,
) -> Option<ItemportModifier> {
    let BaseItemModifierParamsPtr::ItemportTraversingModifiersParams(h) = p else {
        return None;
    };
    let t = h.get(pools)?;
    let regen = t.modifiers.iter().find_map(|m| match m {
        BaseItemModifierParamsPtr::ItemWeaponModifiersParams(w) => {
            let r = w
                .get(pools)?
                .weapon_modifier?
                .get(pools)?
                .weapon_stats?
                .get(pools)?
                .regen_modifier?
                .get(pools)?;
            Some(RegenModifier {
                power_ratio_multiplier: r.power_ratio_multiplier,
                max_ammo_load_multiplier: r.max_ammo_load_multiplier,
                max_regen_multiplier: r.max_regen_per_sec_multiplier,
            })
        }
        _ => None,
    });
    Some(ItemportModifier {
        target: t.target_item_type.clone(),
        on_attach,
        regen,
    })
}

/// Each gun's [`WeaponPower`] under `allocation` in `mode`: its power ratio
/// ([`PowerNetwork::ratios`]) and the regen modifiers of the attach-active
/// traversing modifiers on `loadout`'s own network that target guns
/// (multiplied together). Pairs each gun's index in
/// [`PowerNetwork::consumers`] with its power.
///
/// That a modifier reaches every gun on the vehicle (turret guns too) is
/// inferred; the client's traversal was not read.
pub fn weapon_power(
    network: &PowerNetwork,
    allocation: &Allocation,
    mode: &EMasterMode,
    loadout: &[Mounted],
    vehicles: &Vehicles,
    modifiers: &ItemportModifiers,
) -> Vec<(usize, WeaponPower)> {
    let mut regen = RegenModifier {
        power_ratio_multiplier: 1.0,
        max_ammo_load_multiplier: 1.0,
        max_regen_multiplier: 1.0,
    };
    for (_, item) in network_items(loadout, vehicles) {
        for m in modifiers.get(&item).into_iter().flatten() {
            if let (true, EItemType::WeaponGun, Some(r)) = (m.on_attach, &m.target, m.regen) {
                regen.power_ratio_multiplier *= r.power_ratio_multiplier;
                regen.max_ammo_load_multiplier *= r.max_ammo_load_multiplier;
                regen.max_regen_multiplier *= r.max_regen_multiplier;
            }
        }
    }
    let ratios = network.ratios(allocation, mode);
    network
        .consumers
        .iter()
        .enumerate()
        .filter(|(_, c)| c.item_type == EItemType::WeaponGun)
        .map(|(i, _)| {
            (
                i,
                WeaponPower {
                    ratio: ratios[i],
                    power_ratio_multiplier: regen.power_ratio_multiplier,
                    max_ammo_load_multiplier: regen.max_ammo_load_multiplier,
                    max_regen_multiplier: regen.max_regen_multiplier,
                },
            )
        })
        .collect()
}
