//! The ship resource network's global parameters and a vehicle's power
//! pools.

use sc_extract::Datacore;
use sc_extract::generated::{EItemType, EMasterMode, PowerPoolPtr, ResourceNetworkPowerPoolSetup};
use sc_extract::{DataPools, generated::Handle};
use serde::{Deserialize, Serialize};

/// `ItemResourceNetworkGlobal`: the constants every ship's network runs on.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkParams {
    /// `powerParams.modifiers[].additiveModifier`, indexed by power plant
    /// size.
    pub plant_modifiers: Vec<i32>,
    /// `maxPowerToCoolantRatio`.
    pub max_power_to_coolant_ratio: f32,
    /// `thrusterCoolantMultiplier`.
    pub thruster_coolant_multiplier: f32,
    /// `powerBaseConversionRate`.
    pub power_base_conversion_rate: f32,
    /// `defaultPowerDistributionParams`: how power is shared out when the
    /// ship powers on.
    pub distribution: DistributionDefaults,
    /// `MasterModeExclusionGlobalParams.exclusions`: the master modes each
    /// item type is switched off in (LIVE 4.10: shields in Navigation,
    /// quantum drives in SCM). An item's own override replaces its type's
    /// entry ([`ResourceProfile::mode_exclusions`](crate::ResourceProfile::mode_exclusions)).
    #[serde(with = "exclusions_serde")]
    pub mode_exclusions: Vec<(EItemType, Vec<EMasterMode>)>,
    /// The `EItemType` values in their DCB definition order: the client
    /// keys power pools by that ordinal.
    pub item_type_order: Vec<String>,
}

/// `ItemResourceNetworkDefaultPowerDistributionParams`. The client turns
/// each into an `item_resource.*` console variable with this default.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DistributionDefaults {
    /// `powerDistPoolsFillMin`: pools first get their members' minimum.
    pub pools_fill_min: bool,
    /// `powerDistCoolerMin`: the share of its request a cooler is topped up
    /// to.
    pub cooler_min: f32,
    /// `powerDistPoolsProportion`: the share of its size a pool gets.
    pub pools_proportion: f32,
    /// `poolDefaultWeapons`: replaces `pools_proportion` for the weapon pool.
    pub weapons: f32,
    /// `poolDefaultEngines`: … for the engine (flight controller) pool.
    pub engines: f32,
    /// `poolDefaultShields`: … for the shield pool.
    pub shields: f32,
}

impl NetworkParams {
    /// The `ItemResourceNetworkGlobal` record (one per build); `None` when
    /// the bindings lack it.
    pub fn build(datacore: &Datacore) -> Option<Self> {
        let store = datacore.records();
        let pools = &store.pools;
        let g = store
            .records
            .multi_feature
            .item_resource_network_global
            .values()
            .find_map(|h| h.get(pools))?;
        let plant_modifiers = g
            .power_params
            .and_then(|h| h.get(pools))
            .map(|p| {
                p.modifiers
                    .iter()
                    .map(|m| m.get(pools).map_or(0, |m| m.additive_modifier))
                    .collect()
            })
            .unwrap_or_default();
        let d = g
            .default_power_distribution_params
            .and_then(|h| h.get(pools));
        let mode_exclusions = store
            .records
            .entities_scitem_mastermodeexclusionglobalparams
            .master_mode_exclusion_global_params
            .values()
            .filter_map(|h| h.get(pools))
            .flat_map(|g| &g.exclusions)
            .filter_map(|e| e.get(pools))
            .map(|e| (e.item_type.clone(), e.master_mode_exclusions.clone()))
            .collect();
        // The enum's own definition order: the generated enum carries no
        // ordinal, and the client sorts pools by it.
        let db = datacore.db();
        let item_type_order = db
            .enum_definitions()
            .iter()
            .enumerate()
            .find(|(i, _)| db.enum_name(*i) == Some("EItemType"))
            .map(|(_, e)| db.enum_options(e).into_iter().map(str::to_string).collect())
            .unwrap_or_default();
        Some(Self {
            plant_modifiers,
            max_power_to_coolant_ratio: g.max_power_to_coolant_ratio,
            thruster_coolant_multiplier: g.thruster_coolant_multiplier,
            power_base_conversion_rate: g.power_base_conversion_rate,
            distribution: DistributionDefaults {
                pools_fill_min: d.is_some_and(|d| d.power_dist_pools_fill_min),
                cooler_min: d.map_or(0.0, |d| d.power_dist_cooler_min),
                pools_proportion: d.map_or(0.0, |d| d.power_dist_pools_proportion),
                weapons: d.map_or(0.0, |d| d.pool_default_weapons),
                engines: d.map_or(0.0, |d| d.pool_default_engines),
                shields: d.map_or(0.0, |d| d.pool_default_shields),
            },
            mode_exclusions,
            item_type_order,
        })
    }

    /// Whether an item of `item_type` (with its own `override_modes`) is
    /// switched off in `mode`.
    pub fn excluded(
        &self,
        item_type: &EItemType,
        override_modes: Option<&[EMasterMode]>,
        mode: &EMasterMode,
    ) -> bool {
        let modes = override_modes.or_else(|| {
            self.mode_exclusions
                .iter()
                .find(|(t, _)| t == item_type)
                .map(|(_, m)| m.as_slice())
        });
        modes.is_some_and(|m| m.contains(mode))
    }

    /// `item_type`'s ordinal in [`item_type_order`](Self::item_type_order)
    /// (last when unknown).
    pub fn item_type_rank(&self, item_type: &EItemType) -> usize {
        self.item_type_order
            .iter()
            .position(|t| t == item_type.as_dcb_str())
            .unwrap_or(usize::MAX)
    }

    /// What each extra online power plant adds to every plant's share: the
    /// plant's own `powerPlantOverride` if it has one, else the modifier for
    /// its size (0 past the table).
    pub fn plant_modifier(&self, size: i32, own: Option<i32>) -> i32 {
        own.unwrap_or_else(|| {
            usize::try_from(size)
                .ok()
                .and_then(|s| self.plant_modifiers.get(s))
                .copied()
                .unwrap_or(0)
        })
    }
}

/// One entry of `SItemPortContainerComponentParams.resourceNetworkPowerPools`:
/// the items of one type draw on a shared budget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerPool {
    #[serde(with = "item_type_serde")]
    pub item_type: EItemType,
    pub size: PoolSize,
}

/// How a [`PowerPool`]'s budget is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolSize {
    /// `FixedPowerPool.poolSize` segments, whatever its members request.
    Fixed(i32),
    /// `DynamicPowerPool`: sized by its members; `maxItemCount` caps how
    /// many (`None`: no cap).
    Dynamic { max_items: Option<i32> },
    /// A plain `PowerPool`.
    Plain,
}

pub(crate) fn power_pools(
    setup: Option<Handle<ResourceNetworkPowerPoolSetup>>,
    pools: &DataPools,
) -> Vec<PowerPool> {
    let Some(setup) = setup.and_then(|h| h.get(pools)) else {
        return Vec::new();
    };
    setup
        .item_pools
        .iter()
        .filter_map(|p| match p {
            PowerPoolPtr::FixedPowerPool(h) => h.get(pools).map(|f| PowerPool {
                item_type: f.item_type.clone(),
                size: PoolSize::Fixed(f.pool_size),
            }),
            PowerPoolPtr::DynamicPowerPool(h) => h.get(pools).map(|d| PowerPool {
                item_type: d.item_type.clone(),
                size: PoolSize::Dynamic {
                    max_items: (d.max_item_count > 0).then_some(d.max_item_count),
                },
            }),
            PowerPoolPtr::PowerPool(h) => h.get(pools).map(|p| PowerPool {
                item_type: p.item_type.clone(),
                size: PoolSize::Plain,
            }),
            _ => None,
        })
        .collect()
}

/// `serde` adapter for the generated `EItemType`.
pub(crate) mod item_type_serde {
    use sc_extract::generated::EItemType;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &EItemType, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemType, D::Error> {
        Ok(EItemType::from_dcb_str(&String::deserialize(d)?))
    }
}

/// `serde` adapter for the per-type exclusion table.
mod exclusions_serde {
    use sc_extract::generated::{EItemType, EMasterMode};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        v: &[(EItemType, Vec<EMasterMode>)],
        s: S,
    ) -> Result<S::Ok, S::Error> {
        s.collect_seq(v.iter().map(|(t, m)| {
            (
                t.as_dcb_str(),
                m.iter().map(EMasterMode::as_dcb_str).collect::<Vec<_>>(),
            )
        }))
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Vec<(EItemType, Vec<EMasterMode>)>, D::Error> {
        Ok(Vec::<(String, Vec<String>)>::deserialize(d)?
            .into_iter()
            .map(|(t, m)| {
                (
                    EItemType::from_dcb_str(&t),
                    m.iter().map(|s| EMasterMode::from_dcb_str(s)).collect(),
                )
            })
            .collect())
    }
}
