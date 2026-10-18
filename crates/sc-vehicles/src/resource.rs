//! Every item's place on the ship's resource network: what it makes, uses
//! and converts per state, and the signatures that go with it.

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EMasterMode, Handle, ItemResourceComponentParams,
    ItemResourceDeltaAmount, ItemResourceDeltaBasePtr, ItemResourceDynamicAmountBasePtr,
    ItemResourceSignatureEntry, ItemResourceState, ResourceNetworkResource, ResourceRangeParams,
    SBaseResourceUnitPtr,
};
use sc_extract::{DataPools, Datacore, Guid, RecordCollection};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// An entity's `ItemResourceComponentParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceProfile {
    /// The states it can be in (`Online`, a quantum drive's `Travelling`,
    /// …), in data order.
    pub states: Vec<ResourceState>,
    /// `defaultPriority`.
    pub default_priority: i32,
    /// `powerPlantOverride.additiveModifier`: replaces the network's
    /// per-size modifier when several power plants share the output (see
    /// [`NetworkParams::plant_modifier`](crate::NetworkParams::plant_modifier)).
    pub plant_modifier: Option<i32>,
    /// `ItemControlComponentParams.turnedOnByDefault` (true without the
    /// component).
    #[serde(default = "yes")]
    pub on_by_default: bool,
    /// `ItemControlComponentParams.masterModeExclusionsOverride`: the master
    /// modes it is switched off in, replacing the network's table for its
    /// type ([`NetworkParams::mode_exclusions`](crate::NetworkParams::mode_exclusions)).
    #[serde(default, with = "modes_serde")]
    pub mode_exclusions: Option<Vec<EMasterMode>>,
}

fn yes() -> bool {
    true
}

impl ResourceProfile {
    /// The state named `name` (case-insensitive).
    pub fn state(&self, name: &str) -> Option<&ResourceState> {
        self.states
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))
    }

    /// The `Online` state, where a calculator reads an item's normal draw.
    pub fn online(&self) -> Option<&ResourceState> {
        self.state("Online")
    }
}

/// One `ItemResourceState`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceState {
    pub name: String,
    pub deltas: Vec<ResourceDelta>,
    /// `signatureParams.EMSignature`.
    pub em: Option<Signature>,
    /// `signatureParams.IRSignature`.
    pub ir: Option<Signature>,
    /// `powerRanges.{low, medium, high}`, in that order.
    pub power_ranges: Vec<PowerRange>,
}

impl ResourceState {
    /// What the state makes of `resource` per second: a generation, or the
    /// output of a conversion.
    pub fn generation(&self, resource: &ResourceNetworkResource) -> Option<&ResourceDelta> {
        self.deltas.iter().find(|d| {
            d.generates
                .as_ref()
                .is_some_and(|a| &a.resource == resource)
        })
    }

    /// What the state uses of `resource` per second: a consumption, or the
    /// input of a conversion.
    pub fn consumption(&self, resource: &ResourceNetworkResource) -> Option<&ResourceDelta> {
        self.deltas
            .iter()
            .find(|d| d.consumes.as_ref().is_some_and(|a| &a.resource == resource))
    }
}

/// One delta of a state: an `ItemResourceDelta*`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDelta {
    pub kind: DeltaKind,
    /// What it takes from the network (consumption, conversion input,
    /// storage charge).
    pub consumes: Option<ResourceAmount>,
    /// What it gives the network (generation, conversion output, storage
    /// discharge).
    pub generates: Option<ResourceAmount>,
    /// `minimumConsumptionFraction`: the share of `consumes` below which
    /// the item does not run (0 for kinds without one).
    pub minimum_fraction: f32,
    /// `dynamicAmountOverride`: the amount is computed at run time and the
    /// listed one is a placeholder.
    pub dynamic: Option<DynamicAmount>,
}

/// The kind of [`ResourceDelta`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeltaKind {
    Generation,
    Consumption,
    Conversion,
    Storage,
    Other { type_name: String },
}

/// `ItemResourceDeltaAmount`: a resource and its rate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceAmount {
    #[serde(with = "resource_serde")]
    pub resource: ResourceNetworkResource,
    pub per_second: ResourceUnits,
}

/// A resource rate in the unit the data gives it in.
///
/// The client holds every amount as integer micro-units: a power segment
/// and a standard unit are 1,000,000 each, a centi unit 10,000.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ResourceUnits {
    /// `SPowerSegmentResourceUnit.units`: whole power segments (pips).
    Segments(i32),
    /// `SStandardResourceUnit.standardResourceUnits`.
    Standard(f32),
    /// `SCentiResourceUnit.centiResourceUnits`.
    Centi(i32),
    /// `SMicroResourceUnit.microResourceUnits`.
    Micro(i32),
}

impl ResourceUnits {
    /// The rate in standard units (= segments).
    pub fn standard(self) -> f64 {
        match self {
            Self::Segments(n) => f64::from(n),
            Self::Standard(v) => f64::from(v),
            Self::Centi(n) => f64::from(n) / 100.0,
            Self::Micro(n) => f64::from(n) / 1_000_000.0,
        }
    }
}

/// An `ItemResourceDynamicAmount*` override.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicAmount {
    Coolant,
    ThrusterFuel,
    QuantumTravelFuel,
    JumpTravelFuel,
    FuelNozzleFuel,
    Other { type_name: String },
}

/// `ItemResourceSignatureEntry`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    /// `nominalSignature`: the value at a power ratio of 1.
    pub nominal: f32,
    /// `decayRate`: how fast the live value follows the ideal one.
    pub decay_rate: f32,
    /// `functionalityCurve.points`: maps the ratio before it scales the
    /// nominal value (empty: linear).
    #[serde(default)]
    pub curve: Vec<[f32; 2]>,
}

/// `ResourceRangeParams`: a band of the item's segment count with its
/// signature modifier.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PowerRange {
    /// `start`: the first segment count in the band.
    pub start: i32,
    pub modifier: f32,
    /// `registerRange`: an unregistered band is skipped.
    pub registered: bool,
}

/// Every entity's [`ResourceProfile`], keyed by `EntityClassDefinition`
/// GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceProfiles {
    by_guid: HashMap<Guid, ResourceProfile>,
}

impl ResourceProfiles {
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
            let Some(params) = ecd.components.iter().find_map(|c| match c {
                DataForgeComponentParamsPtr::ItemResourceComponentParams(h) => h.get(pools),
                _ => None,
            }) else {
                continue;
            };
            let mut profile = profile(params, pools);
            if let Some(control) = ecd.components.iter().find_map(|c| match c {
                DataForgeComponentParamsPtr::ItemControlComponentParams(h) => h.get(pools),
                _ => None,
            }) {
                profile.on_by_default = control.turned_on_by_default;
                profile.mode_exclusions = control
                    .master_mode_exclusions_override
                    .and_then(|h| h.get(pools))
                    .map(|o| o.master_mode_exclusions.clone());
            }
            debug!(%guid, states = profile.states.len(), "resource profile");
            by_guid.insert(guid, profile);
        }
        Self { by_guid }
    }
}

impl RecordCollection for ResourceProfiles {
    type Item = ResourceProfile;

    fn get(&self, guid: &Guid) -> Option<&ResourceProfile> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &ResourceProfile)> + '_ {
        self.by_guid.iter()
    }
}

fn profile(p: &ItemResourceComponentParams, pools: &DataPools) -> ResourceProfile {
    ResourceProfile {
        states: p.states.iter().filter_map(|s| state(*s, pools)).collect(),
        default_priority: p.default_priority,
        plant_modifier: p
            .power_plant_override
            .and_then(|h| h.get(pools))
            .map(|m| m.additive_modifier),
        on_by_default: true,
        mode_exclusions: None,
    }
}

fn state(h: Handle<ItemResourceState>, pools: &DataPools) -> Option<ResourceState> {
    let s = h.get(pools)?;
    let signatures = s.signature_params.and_then(|h| h.get(pools));
    let signature = |e: Option<Handle<ItemResourceSignatureEntry>>| {
        e.and_then(|h| h.get(pools)).map(|e| Signature {
            nominal: e.nominal_signature,
            decay_rate: e.decay_rate,
            curve: e
                .functionality_curve
                .and_then(|c| c.get(pools))
                .map(|c| {
                    c.points
                        .iter()
                        .filter_map(|p| p.get(pools))
                        .map(|p| [p.x, p.y])
                        .collect()
                })
                .unwrap_or_default(),
        })
    };
    let range = |r: Option<Handle<ResourceRangeParams>>| {
        r.and_then(|h| h.get(pools)).map(|r| PowerRange {
            start: r.start,
            modifier: r.modifier,
            registered: r.register_range,
        })
    };
    let power_ranges = s
        .power_ranges
        .and_then(|h| h.get(pools))
        .map(|r| {
            [r.low, r.medium, r.high]
                .into_iter()
                .filter_map(range)
                .collect()
        })
        .unwrap_or_default();
    Some(ResourceState {
        name: s.name.clone(),
        deltas: s.deltas.iter().filter_map(|d| delta(d, pools)).collect(),
        em: signatures.and_then(|g| signature(g.emsignature)),
        ir: signatures.and_then(|g| signature(g.irsignature)),
        power_ranges,
    })
}

fn delta(d: &ItemResourceDeltaBasePtr, pools: &DataPools) -> Option<ResourceDelta> {
    let amount = |a: Option<Handle<ItemResourceDeltaAmount>>| amount(a, pools);
    let dynamic = |o: &Option<ItemResourceDynamicAmountBasePtr>| o.as_ref().map(dynamic_amount);
    Some(match d {
        ItemResourceDeltaBasePtr::ItemResourceDeltaGeneration(h) => {
            let g = h.get(pools)?;
            ResourceDelta {
                kind: DeltaKind::Generation,
                consumes: None,
                generates: amount(g.generation),
                minimum_fraction: 0.0,
                dynamic: dynamic(&g.dynamic_amount_override),
            }
        }
        ItemResourceDeltaBasePtr::ItemResourceDeltaConsumption(h) => {
            let c = h.get(pools)?;
            ResourceDelta {
                kind: DeltaKind::Consumption,
                consumes: amount(c.consumption),
                generates: None,
                minimum_fraction: c.minimum_consumption_fraction,
                dynamic: dynamic(&c.dynamic_amount_override),
            }
        }
        ItemResourceDeltaBasePtr::ItemResourceDeltaConversion(h) => {
            let c = h.get(pools)?;
            ResourceDelta {
                kind: DeltaKind::Conversion,
                consumes: amount(c.consumption),
                generates: amount(c.generation),
                minimum_fraction: c.minimum_consumption_fraction,
                dynamic: dynamic(&c.dynamic_amount_override),
            }
        }
        ItemResourceDeltaBasePtr::ItemResourceDeltaStorage(h) => {
            let s = h.get(pools)?;
            ResourceDelta {
                kind: DeltaKind::Storage,
                consumes: amount(s.consumption),
                generates: amount(s.generation),
                minimum_fraction: 0.0,
                dynamic: None,
            }
        }
        other => ResourceDelta {
            kind: DeltaKind::Other {
                type_name: delta_type_name(other).to_string(),
            },
            consumes: None,
            generates: None,
            minimum_fraction: 0.0,
            dynamic: None,
        },
    })
}

fn delta_type_name(d: &ItemResourceDeltaBasePtr) -> &'static str {
    match d {
        ItemResourceDeltaBasePtr::ItemResourceDeltaNetworkReflection(_) => {
            "ItemResourceDeltaNetworkReflection"
        }
        ItemResourceDeltaBasePtr::ItemResourceDeltaBase(_) => "ItemResourceDeltaBase",
        _ => "unknown",
    }
}

fn amount(a: Option<Handle<ItemResourceDeltaAmount>>, pools: &DataPools) -> Option<ResourceAmount> {
    let a = a?.get(pools)?;
    let per_second = match a.resource_amount_per_second.as_ref()? {
        SBaseResourceUnitPtr::SPowerSegmentResourceUnit(h) => {
            ResourceUnits::Segments(h.get(pools)?.units)
        }
        SBaseResourceUnitPtr::SStandardResourceUnit(h) => {
            ResourceUnits::Standard(h.get(pools)?.standard_resource_units)
        }
        SBaseResourceUnitPtr::SCentiResourceUnit(h) => {
            ResourceUnits::Centi(h.get(pools)?.centi_resource_units)
        }
        SBaseResourceUnitPtr::SMicroResourceUnit(h) => {
            ResourceUnits::Micro(h.get(pools)?.micro_resource_units)
        }
        _ => return None,
    };
    Some(ResourceAmount {
        resource: a.resource.clone(),
        per_second,
    })
}

fn dynamic_amount(d: &ItemResourceDynamicAmountBasePtr) -> DynamicAmount {
    use ItemResourceDynamicAmountBasePtr as D;
    match d {
        D::ItemResourceDynamicAmountCoolant(_) => DynamicAmount::Coolant,
        D::ItemResourceDynamicAmountThrusterFuel(_) => DynamicAmount::ThrusterFuel,
        D::ItemResourceDynamicAmountQuantumTravelFuel(_) => DynamicAmount::QuantumTravelFuel,
        D::ItemResourceDynamicAmountJumpTravelFuel(_) => DynamicAmount::JumpTravelFuel,
        D::ItemResourceDynamicAmountFuelNozzleFuel(_) => DynamicAmount::FuelNozzleFuel,
        // Room life support (`entities-roomsystem`, not a ship item) and
        // dormant types.
        _ => DynamicAmount::Other {
            type_name: "ItemResourceDynamicAmount".to_string(),
        },
    }
}

/// `serde` adapter for the generated `ResourceNetworkResource`.
pub(crate) mod resource_serde {
    use sc_extract::generated::ResourceNetworkResource;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &ResourceNetworkResource, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<ResourceNetworkResource, D::Error> {
        Ok(ResourceNetworkResource::from_dcb_str(&String::deserialize(
            d,
        )?))
    }
}

/// `serde` adapter for an optional list of the generated `EMasterMode`.
pub(crate) mod modes_serde {
    use sc_extract::generated::EMasterMode;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<Vec<EMasterMode>>, s: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(v) => s.collect_seq(v.iter().map(EMasterMode::as_dcb_str)),
            None => s.serialize_none(),
        }
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<Vec<EMasterMode>>, D::Error> {
        Ok(Option::<Vec<String>>::deserialize(d)?
            .map(|v| v.iter().map(|s| EMasterMode::from_dcb_str(s)).collect()))
    }
}

/// `serde` adapter for a list of the generated `EMasterMode`.
pub(crate) mod modes_list_serde {
    use sc_extract::generated::EMasterMode;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &[EMasterMode], s: S) -> Result<S::Ok, S::Error> {
        s.collect_seq(v.iter().map(EMasterMode::as_dcb_str))
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<EMasterMode>, D::Error> {
        Ok(Vec::<String>::deserialize(d)?
            .iter()
            .map(|s| EMasterMode::from_dcb_str(s))
            .collect())
    }
}
