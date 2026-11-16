//! Law-system jurisdictions over the DCB's `Jurisdiction` records: what each
//! one prohibits and which substances it controls.
//!
//! A jurisdiction (UEE, Hurston Dynamics, Klescher Rehabilitation, Rough &
//! Ready, …) lives in `libs/foundry/records/lawsystem/jurisdictions/`.
//! Places point at theirs through `sc_locations::Location::jurisdiction`;
//! mission contracts override it per quest.
//!
//! # What a jurisdiction outlaws
//!
//! Two lists, each authored twice: once as resources, once as the commodity
//! entity classes those resources ship as.
//!
//! - **Contraband:** [`Jurisdiction::prohibited_resources`] (`ResourceType`
//!   GUIDs, which join `sc_resources::Resources`) and
//!   [`Jurisdiction::prohibited_goods`] (`EntityClassDefinition` GUIDs, which
//!   join `sc_items::Items`).
//! - **Controlled substances:** [`Jurisdiction::controlled_substances`],
//!   classes of drugs with a possession limit each. A class is an inline
//!   element of the jurisdiction, not a record: it has no name or GUID, and
//!   its position is not an identity (the same slot holds different drugs in
//!   different jurisdictions).
//!
//! Every reference stays a GUID; resolve it through the collection of the
//! record it names. `examples/jurisdictions.rs` asserts on live data that
//! every resource reference resolves in `Resources`.
//!
//! # Not modelled
//!
//! - **Inheritance.** [`Jurisdiction::parent`] and
//!   [`Jurisdiction::respects_parent_laws`] are passed through raw. Some
//!   jurisdictions with empty lists respect a parent that prohibits goods.
//!   Whether the parent's lists apply there in game is unverified, so there
//!   is no "effective law" helper.
//! - Infractions, infraction sets, fines, early payment, impounding, the
//!   journal entry and the logo.
//!
//! # Example
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
//! use sc_law::{Jurisdictions, RecordCollection};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//!
//! let jurisdictions = Jurisdictions::build(datacore.records());
//! for j in jurisdictions.values() {
//!     let drugs: usize = j.controlled_substances.iter().map(|c| c.resources.len()).sum();
//!     println!(
//!         "{:?}: {} contraband, {drugs} controlled",
//!         j.name_key.as_ref().and_then(|k| data.locale.resolve(k)),
//!         j.prohibited_resources.len(),
//!     );
//! }
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;

use sc_extract::generated::{Jurisdiction as DcbJurisdiction, RecordLookup};
use sc_extract::{DataPools, Guid, LocaleKey, RecordStore};
use serde::{Deserialize, Serialize};

// Re-export the canonical accessor trait (get / iter / len / values) so consumers
// can bring it into scope alongside the collection.
pub use sc_extract::RecordCollection;

/// One law-system jurisdiction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// The record GUID. `sc_locations::Location::jurisdiction` points here.
    pub guid: Guid,
    /// `name`: display-name key, raw with its leading `@`. `None` when empty.
    pub name_key: Option<LocaleKey>,
    /// `parentJurisdiction` (the UEE, for the Stanton corporate
    /// jurisdictions). Passed through raw; see the crate docs on inheritance.
    pub parent: Option<Guid>,
    /// `respectsParentJurisdictionLaws`. Passed through raw.
    pub respects_parent_laws: bool,
    /// `isPrison`.
    pub is_prison: bool,
    /// `maxStolenGoodsPossessionSCU`.
    pub max_stolen_goods_scu: f32,
    /// `prohibitedResources`: contraband, as `ResourceType` GUIDs.
    pub prohibited_resources: Vec<Guid>,
    /// `prohibitedGoods`: the same contraband as commodity entity classes
    /// (`EntityClassDefinition` GUIDs).
    pub prohibited_goods: Vec<Guid>,
    /// `controlledSubstanceClasses`, in record order.
    pub controlled_substances: Vec<ControlledSubstanceClass>,
}

/// One class of controlled substances inside a [`Jurisdiction`]. Inline, so
/// it has no GUID or name of its own.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlledSubstanceClass {
    /// `resources`: the substances, as `ResourceType` GUIDs.
    pub resources: Vec<Guid>,
    /// `commodities`: the same substances as commodity entity classes
    /// (`EntityClassDefinition` GUIDs).
    pub commodities: Vec<Guid>,
    /// `maxPossessionSCU`: how much of the class may be carried.
    pub max_possession_scu: f32,
}

/// Every `Jurisdiction` record, by GUID. Build once, share by reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Jurisdictions {
    by_guid: HashMap<Guid, Jurisdiction>,
}

impl Jurisdictions {
    /// Construct an empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build from a parsed [`RecordStore`] (typed `Jurisdiction` pool).
    pub fn build(store: &RecordStore) -> Self {
        let mut out = Self::new();
        for (&guid, &handle) in &store.records.multi_feature.jurisdiction {
            let Some(j) = handle.get(&store.pools) else {
                continue;
            };
            out.insert(jurisdiction_for(guid, j, &store.pools));
        }
        tracing::info!(jurisdictions = out.len(), "built jurisdictions");
        out
    }

    /// Insert or replace a jurisdiction.
    pub fn insert(&mut self, jurisdiction: Jurisdiction) {
        self.by_guid.insert(jurisdiction.guid, jurisdiction);
    }
}

impl sc_extract::RecordCollection for Jurisdictions {
    type Item = Jurisdiction;

    fn get(&self, guid: &Guid) -> Option<&Jurisdiction> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Jurisdiction)> + '_ {
        self.by_guid.iter()
    }
}

/// Project one typed `Jurisdiction`. Shared by [`Jurisdictions::build`] and
/// [`JurisdictionsBuilder`].
fn jurisdiction_for(guid: Guid, j: &DcbJurisdiction, pools: &DataPools) -> Jurisdiction {
    tracing::debug!(%guid, "jurisdiction");
    let controlled_substances = j
        .controlled_substance_classes
        .iter()
        .filter_map(|h| h.get(pools))
        .map(|c| ControlledSubstanceClass {
            resources: c.resources.clone(),
            commodities: c.commodities.clone(),
            max_possession_scu: c.max_possession_scu,
        })
        .collect();
    Jurisdiction {
        guid,
        name_key: (!j.name.as_str().is_empty()).then(|| j.name.clone()),
        parent: j.parent_jurisdiction,
        respects_parent_laws: j.respects_parent_jurisdiction_laws,
        is_prison: j.is_prison,
        max_stolen_goods_scu: j.max_stolen_goods_possession_scu,
        prohibited_resources: j.prohibited_resources.clone(),
        prohibited_goods: j.prohibited_goods.clone(),
        controlled_substances,
    }
}

/// [`sc_extract::RecordVisitor`] that builds [`Jurisdictions`] in a bundled
/// walk. Equivalent to [`Jurisdictions::build`] but fusible with other
/// visitors.
#[derive(Default)]
pub struct JurisdictionsBuilder {
    inner: Jurisdictions,
}

impl sc_extract::RecordVisitor for JurisdictionsBuilder {
    type Output = Jurisdictions;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["Jurisdiction"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let Some(handle) = DcbJurisdiction::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(j) = handle.get(&store.pools) else {
            return;
        };
        self.inner
            .insert(jurisdiction_for(item.guid, j, &store.pools));
    }

    fn finish(self) -> Jurisdictions {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(byte: u8) -> Guid {
        Guid::from_bytes([byte; 16])
    }

    fn make(guid: Guid, parent: Option<Guid>) -> Jurisdiction {
        Jurisdiction {
            guid,
            name_key: Some(LocaleKey::new("@Jurisdictions_Name_001")),
            parent,
            respects_parent_laws: parent.is_some(),
            is_prison: false,
            max_stolen_goods_scu: 1.0,
            prohibited_resources: vec![g(10)],
            prohibited_goods: vec![g(20)],
            controlled_substances: vec![ControlledSubstanceClass {
                resources: vec![g(11)],
                commodities: vec![g(21)],
                max_possession_scu: 0.125,
            }],
        }
    }

    #[test]
    fn new_is_empty() {
        let js = Jurisdictions::new();
        assert!(js.is_empty());
        assert_eq!(js.len(), 0);
    }

    #[test]
    fn insert_and_lookup() {
        let mut js = Jurisdictions::new();
        js.insert(make(g(1), None));
        js.insert(make(g(2), Some(g(1))));
        assert_eq!(js.len(), 2);
        assert_eq!(js.get(&g(2)).and_then(|j| j.parent), Some(g(1)));
        assert!(js.get(&g(3)).is_none());
    }

    #[test]
    fn serde_round_trip() {
        let mut js = Jurisdictions::new();
        js.insert(make(g(1), None));
        let json = serde_json::to_string(&js).unwrap();
        let back: Jurisdictions = serde_json::from_str(&json).unwrap();
        assert_eq!(back.get(&g(1)), js.get(&g(1)));
    }
}
