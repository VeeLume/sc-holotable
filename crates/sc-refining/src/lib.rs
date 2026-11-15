//! Refining — a domain crate.
//!
//! - [`RefiningProcesses`]: the `RefiningProcess` records, the methods a
//!   refinery offers: a [`RefiningSpeed`] and a [`RefiningQuality`] with
//!   their names.
//! - [`Refineries`]: the entity classes that refine (those carrying an
//!   `EntityComponentRefineryParams`, a component with no parameters).
//! - [`RefineryPlacements`]: where object containers place them and the
//!   star-map places those belong to (live p4k).
//!
//! What refining turns a resource into is
//! `sc_resources::Resource::refined_version`.
//!
//! # Not in the data (LIVE 4.10)
//!
//! How much refined material a refinery makes from an amount of ore. The
//! types that would hold it (`CraftingProcess_Refining` and
//! `CraftingProcessSpecificRecipeData_Refining`, with a raw and a refined
//! quantity) have no records, and the processes carry only their speed and
//! quality labels; so no yield is modelled.
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
//! use sc_refining::{Refineries, RefineryPlacements, RefiningProcesses};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let processes = RefiningProcesses::build(&datacore);
//! let refineries = Refineries::build(&datacore);
//! let placements = RefineryPlacements::cook(&assets, &refineries)?;
//! println!(
//!     "{} processes, refineries at {} places",
//!     processes.len(),
//!     placements.places().count()
//! );
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod placement;

use std::collections::HashMap;

use sc_extract::generated::DataForgeComponentParamsPtr;
pub use sc_extract::generated::{RefiningQuality, RefiningSpeed};
use sc_extract::{Datacore, Guid, LocaleKey};
use serde::{Deserialize, Serialize};
use tracing::info;

pub use placement::{RefineryPlacement, RefineryPlacements};
/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collections.
pub use sc_extract::RecordCollection;

/// One `RefiningProcess` record: a refining method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefiningProcess {
    pub guid: Guid,
    /// `refiningSpeed`.
    #[serde(with = "speed_serde")]
    pub speed: RefiningSpeed,
    /// `refiningQuality`.
    #[serde(with = "quality_serde")]
    pub quality: RefiningQuality,
    /// `processName`.
    pub name: LocaleKey,
    /// `processDetail`.
    pub detail: LocaleKey,
}

/// Every `RefiningProcess` record, keyed by GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RefiningProcesses {
    by_guid: HashMap<Guid, RefiningProcess>,
}

impl RefiningProcesses {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut by_guid = HashMap::new();
        for (guid, h) in &store.records.refiningprocess.refining_process {
            let Some(p) = h.get(pools) else { continue };
            by_guid.insert(
                *guid,
                RefiningProcess {
                    guid: *guid,
                    speed: p.refining_speed.clone(),
                    quality: p.refining_quality.clone(),
                    name: p.process_name.clone(),
                    detail: p.process_detail.clone(),
                },
            );
        }
        info!(processes = by_guid.len(), "refining processes built");
        Self { by_guid }
    }
}

impl RecordCollection for RefiningProcesses {
    type Item = RefiningProcess;

    fn get(&self, guid: &Guid) -> Option<&RefiningProcess> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &RefiningProcess)> + '_ {
        self.by_guid.iter()
    }
}

/// An entity class that refines: it carries an
/// `EntityComponentRefineryParams`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refinery {
    /// The `EntityClassDefinition` GUID.
    pub guid: Guid,
}

/// Every refinery entity class, keyed by its `EntityClassDefinition` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Refineries {
    by_guid: HashMap<Guid, Refinery>,
}

impl Refineries {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut by_guid = HashMap::new();
        for (guid, h) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = h.get(pools) else { continue };
            if ecd.components.iter().any(|c| {
                matches!(
                    c,
                    DataForgeComponentParamsPtr::EntityComponentRefineryParams(_)
                )
            }) {
                by_guid.insert(*guid, Refinery { guid: *guid });
            }
        }
        info!(refineries = by_guid.len(), "refinery classes built");
        Self { by_guid }
    }

    /// Add a refinery class (for tests and hand-built collections).
    pub fn insert(&mut self, refinery: Refinery) {
        self.by_guid.insert(refinery.guid, refinery);
    }
}

impl RecordCollection for Refineries {
    type Item = Refinery;

    fn get(&self, guid: &Guid) -> Option<&Refinery> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Refinery)> + '_ {
        self.by_guid.iter()
    }
}

/// `serde` adapter: the DCB string.
mod speed_serde {
    use sc_extract::generated::RefiningSpeed;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &RefiningSpeed, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<RefiningSpeed, D::Error> {
        Ok(RefiningSpeed::from_dcb_str(&String::deserialize(d)?))
    }
}

/// `serde` adapter: the DCB string.
mod quality_serde {
    use sc_extract::generated::RefiningQuality;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &RefiningQuality, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<RefiningQuality, D::Error> {
        Ok(RefiningQuality::from_dcb_str(&String::deserialize(d)?))
    }
}
