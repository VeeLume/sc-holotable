use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{DataPools, Guid};

use crate::classify::{classify, WeaponCategory};
use crate::damage::{self, DamageSummary};
use crate::fire_action::{self, FireActionKind};
use crate::sustain::{self, SustainKind};

/// A ship-mounted weapon with all data materialized into owned fields.
///
/// Constructed via [`ShipWeapon::try_new`] which resolves all handles from
/// the flat pool. After construction every accessor is a plain field read.
#[derive(Debug, Clone)]
pub struct ShipWeapon {
    /// Record GUID — stable across game patches.
    pub guid: Guid,
    /// DCB record name (e.g. `"GATS_BallisticGatling_S1"`).
    pub record_name: String,
    /// Weapon size (1-12).
    pub size: i32,
    /// Weapon grade (always 1 in 4.7).
    pub grade: i32,
    /// Item type enum.
    pub item_type: EItemType,
    /// Item subtype enum (`Gun`, `Rocket`, `NoseMounted`).
    pub item_sub_type: EItemSubType,
    /// Manufacturer GUID — look up via `ManufacturerRegistry::get`.
    pub manufacturer_guid: Option<Guid>,
    /// Primary fire action (from `fireActions[0]`).
    pub primary_fire_action: FireActionKind,
    /// Total number of fire actions (>1 means fire mode switching).
    pub fire_action_count: usize,
    /// Sustain model: Heat, Energy, or None.
    pub sustain: SustainKind,
    /// Per-shot damage (direct + explosion, all 6 types). `None` if ammo
    /// could not be resolved (mining lasers, dummies).
    pub damage: Option<DamageSummary>,
    /// Ammo projectile speed in m/s.
    pub ammo_speed: Option<f32>,
    /// Ammo lifetime in seconds (range ≈ speed × lifetime).
    pub ammo_lifetime: Option<f32>,
    /// Pellet count from `SProjectileLauncher` (scatter guns: 8 or 12).
    /// `None` for single-projectile weapons.
    pub pellet_count: Option<i32>,
    /// Magazine size from `SAmmoContainerComponentParams.maxAmmoCount`.
    pub magazine_size: Option<i32>,
    /// Power draw per second when active (from `ItemResourceComponentParams`).
    pub power_draw: Option<f32>,
    /// Weapon health points (from `SHealthComponentParams`).
    pub health: Option<f32>,
    /// Raw entity handle — escape hatch for consumers with `&DataPools`.
    pub entity_handle: Handle<EntityClassDefinition>,
}

impl ShipWeapon {
    /// Attempt to construct a `ShipWeapon` from an entity handle.
    ///
    /// Returns `None` if:
    /// - The entity doesn't have `SCItemWeaponComponentParams`
    /// - The entity doesn't have `SAttachableComponentParams` with an `AttachDef`
    /// - Classification doesn't match [`WeaponCategory::Ship`]
    pub fn try_new(
        handle: Handle<EntityClassDefinition>,
        guid: Guid,
        pools: &DataPools,
        ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
        ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
        record_names: &HashMap<Guid, &str>,
    ) -> Option<Self> {
        let ecd = handle.get(pools)?;

        // Must have weapon component
        let wp = damage::find_component::<SCItemWeaponComponentParams>(ecd, pools)?;

        // Must have attachable with item definition
        let attachable = damage::find_component::<SAttachableComponentParams>(ecd, pools)?;
        let item_def = attachable.attach_def.and_then(|h| h.get(pools))?;

        // Must classify as ship weapon
        if classify(&item_def.r#type, &item_def.sub_type) != Some(WeaponCategory::Ship) {
            return None;
        }

        // Fire action
        let primary_fire_action = wp
            .fire_actions
            .first()
            .map(|a| fire_action::extract_fire_action(a, pools))
            .unwrap_or(FireActionKind::Unknown);

        // Sustain
        let sustain = sustain::extract_sustain(wp, pools);

        // Ammo
        let ammo = damage::resolve_ammo(ecd, wp, pools, ecd_map, ammo_map);

        // Pellets
        let pellet_count = damage::extract_pellet_count(wp, pools);

        // Magazine
        let magazine_size = damage::extract_magazine_size(ecd, pools);

        // Power draw
        let power_draw = extract_power_draw(ecd, pools);

        // Health
        let health = damage::find_component::<SHealthComponentParams>(ecd, pools)
            .map(|h| h.health);

        // Record name
        let record_name = record_names
            .get(&guid)
            .map(|n| n.strip_prefix("EntityClassDefinition.").unwrap_or(n))
            .unwrap_or("")
            .to_string();

        Some(ShipWeapon {
            guid,
            record_name,
            size: item_def.size,
            grade: item_def.grade,
            item_type: item_def.r#type.clone(),
            item_sub_type: item_def.sub_type.clone(),
            manufacturer_guid: item_def.manufacturer,
            primary_fire_action,
            fire_action_count: wp.fire_actions.len(),
            sustain,
            damage: ammo.as_ref().map(|a| a.damage),
            ammo_speed: ammo.as_ref().map(|a| a.speed),
            ammo_lifetime: ammo.as_ref().map(|a| a.lifetime),
            pellet_count,
            magazine_size,
            power_draw,
            health,
            entity_handle: handle,
        })
    }
}

/// Extract power draw from `ItemResourceComponentParams`.
fn extract_power_draw(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<f32> {
    let irc = damage::find_component::<ItemResourceComponentParams>(ecd, pools)?;

    for state_h in &irc.states {
        let state = state_h.get(pools)?;
        for delta in &state.deltas {
            if let ItemResourceDeltaBasePtr::ItemResourceDeltaConsumption(h) = delta {
                let cons = h.get(pools)?;
                let amount = cons.consumption.and_then(|h| h.get(pools))?;
                if matches!(amount.resource, ResourceNetworkResource::Power) {
                    let val = amount.resource_amount_per_second.as_ref().and_then(|u| {
                        match u {
                            SBaseResourceUnitPtr::SStandardResourceUnit(h) => {
                                h.get(pools).map(|v| v.standard_resource_units)
                            }
                            SBaseResourceUnitPtr::SPowerSegmentResourceUnit(h) => {
                                h.get(pools).map(|v| v.units as f32)
                            }
                            _ => None,
                        }
                    });
                    if let Some(v) = val {
                        return Some(v);
                    }
                }
            }
        }
    }
    None
}
