//! Typed surface over Star Citizen `StarMapObject` records — the universe's
//! stable *places*: solar systems, planets, moons, landing zones, rest stops,
//! stations, outposts, mining claims.
//!
//! A [`Location`] is the curated view of one `StarMapObject`: localized name,
//! typed [`LocationKind`] category, and hierarchy parent. [`Locations`] indexes
//! every location by GUID **and by class-CRC**, so an EntityGraph gRPC
//! `subject_id` (a [`sc_extract::class_crc`] of the record GUID) resolves
//! straight to a typed location. See `docs/sc-locations.md` for the design spec.
//!
//! # Why this is its own crate
//!
//! `StarMapObject` records are not items, so they don't resolve through
//! `sc-items`'s catalog. The generic [`sc_extract::CrcIndex`] resolves any
//! record CRC to a GUID + raw record name; `sc-locations` is the typed upgrade
//! for the location domain (localized name, category enum, hierarchy), the same
//! way `sc-items` is the typed upgrade for items.
//!
//! # Sharing
//!
//! [`Locations::build`] returns an **owned** index. Build it once and pass
//! `&Locations` to consumers; the walk touches every `StarMapObject`.

use std::collections::{HashMap, HashSet};

use sc_extract::generated::{
    ERespawnLocationType, NavPointIconEnum, RecordLookup, StarMapObject, StarMapObjectType,
};
use sc_extract::{Guid, LocaleKey, LocaleMap, RecordStore, class_crc};
use serde::{Deserialize, Serialize};
use tracing::warn;

// Re-export the canonical accessor trait (get / iter / len / values) so consumers
// can bring it into scope alongside the collection.
pub use sc_extract::RecordCollection;

mod object_containers;
pub use object_containers::{
    ObjectContainers, Orbit, Placement, PlacementId, normalize_socpak_path,
};
mod universe;
pub use universe::{Place, Universe};

/// Typed location category, resolved from a `StarMapObjectType.name`. The known
/// variants are the 21 type records observed in live DCB data; a value the
/// generator hasn't seen (a future patch's type) falls through to
/// [`LocationKind::Unrecognized`] for forward-compat. Mirrors the generated
/// enums' `from_dcb_str` / `as_dcb_str` round-trip.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum LocationKind {
    Anomaly,
    Asteroid,
    AsteroidValidQt,
    CardinalPoint,
    JumpPoint,
    LandingZone,
    Manmade,
    ManmadeJumpPoint,
    ManmadeVisibleOnInteraction,
    Moon,
    NavPoint,
    Outpost,
    OutpostInvalidQt,
    Planet,
    PointOfInterest,
    QuantumTracePoint,
    /// Squadron-42 moon variant.
    S42Moon,
    /// Squadron-42 planet variant.
    S42Planet,
    SolarSystem,
    Star,
    YouAreHere,
    /// A type string not in the known set (forward-compat). Carries the raw
    /// `StarMapObjectType.name`.
    Unrecognized(String),
}

impl LocationKind {
    /// Map a raw `StarMapObjectType.name` to a typed kind.
    pub fn from_dcb_str(s: &str) -> Self {
        match s {
            "Anomaly" => Self::Anomaly,
            "Asteroid" => Self::Asteroid,
            "Asteroid_ValidQT" => Self::AsteroidValidQt,
            "CardinalPoint" => Self::CardinalPoint,
            "JumpPoint" => Self::JumpPoint,
            "LandingZone" => Self::LandingZone,
            "Manmade" => Self::Manmade,
            "ManmadeJumpPoint" => Self::ManmadeJumpPoint,
            "Manmade_VisibleOnInteraction" => Self::ManmadeVisibleOnInteraction,
            "Moon" => Self::Moon,
            "NavPoint" => Self::NavPoint,
            "Outpost" => Self::Outpost,
            "Outpost_InvalidQT" => Self::OutpostInvalidQt,
            "Planet" => Self::Planet,
            "PointOfInterest" => Self::PointOfInterest,
            "QuantumTracePoint" => Self::QuantumTracePoint,
            "S42_Moon" => Self::S42Moon,
            "S42_Planet" => Self::S42Planet,
            "SolarSystem" => Self::SolarSystem,
            "Star" => Self::Star,
            "YouAreHere" => Self::YouAreHere,
            other => Self::Unrecognized(other.to_string()),
        }
    }

    /// The raw `StarMapObjectType.name` this kind round-trips to.
    pub fn as_dcb_str(&self) -> &str {
        match self {
            Self::Anomaly => "Anomaly",
            Self::Asteroid => "Asteroid",
            Self::AsteroidValidQt => "Asteroid_ValidQT",
            Self::CardinalPoint => "CardinalPoint",
            Self::JumpPoint => "JumpPoint",
            Self::LandingZone => "LandingZone",
            Self::Manmade => "Manmade",
            Self::ManmadeJumpPoint => "ManmadeJumpPoint",
            Self::ManmadeVisibleOnInteraction => "Manmade_VisibleOnInteraction",
            Self::Moon => "Moon",
            Self::NavPoint => "NavPoint",
            Self::Outpost => "Outpost",
            Self::OutpostInvalidQt => "Outpost_InvalidQT",
            Self::Planet => "Planet",
            Self::PointOfInterest => "PointOfInterest",
            Self::QuantumTracePoint => "QuantumTracePoint",
            Self::S42Moon => "S42_Moon",
            Self::S42Planet => "S42_Planet",
            Self::SolarSystem => "SolarSystem",
            Self::Star => "Star",
            Self::YouAreHere => "YouAreHere",
            Self::Unrecognized(s) => s,
        }
    }
}

/// serde adapters for the category + the two generated identity enums. Generated
/// enums carry no serde of their own (the generated crate stays serde-free), and
/// [`LocationKind`] follows the same convention, so each is stored as its DCB
/// string via the `as_dcb_str` / `from_dcb_str` round-trip.
mod enum_serde {
    use super::{ERespawnLocationType, LocationKind, NavPointIconEnum};
    use serde::{Deserialize, Deserializer, Serializer};

    pub mod location_kind {
        use super::*;
        pub fn serialize<S: Serializer>(v: &LocationKind, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<LocationKind, D::Error> {
            Ok(LocationKind::from_dcb_str(&String::deserialize(d)?))
        }
    }

    pub mod nav_icon {
        use super::*;
        pub fn serialize<S: Serializer>(v: &NavPointIconEnum, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<NavPointIconEnum, D::Error> {
            Ok(NavPointIconEnum::from_dcb_str(&String::deserialize(d)?))
        }
    }

    pub mod respawn {
        use super::*;
        pub fn serialize<S: Serializer>(v: &ERespawnLocationType, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(
            d: D,
        ) -> Result<ERespawnLocationType, D::Error> {
            Ok(ERespawnLocationType::from_dcb_str(&String::deserialize(d)?))
        }
    }
}

/// One universe location, materialized from a `StarMapObject` record.
///
/// The [`LocaleKey`]s keep the leading `@` the DCB carries — keys are raw,
/// resolution happens at the call site (see [`Location::display_name`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// The record GUID. `class_crc(guid)` is the EntityGraph wire `subject_id`.
    pub guid: Guid,
    /// `name` — primary display-name key (e.g. `@ui_pregame_port_Levski_name`).
    pub name_key: Option<LocaleKey>,
    /// `description` — long-form description key.
    pub desc_key: Option<LocaleKey>,
    /// `callout1..3` — short callout-line keys, empties dropped.
    pub callouts: Vec<LocaleKey>,
    /// Typed category, resolved from `type → StarMapObjectType.name`.
    #[serde(with = "enum_serde::location_kind")]
    pub kind: LocationKind,
    /// `navIcon` — typed map-icon class (identification aid).
    #[serde(with = "enum_serde::nav_icon")]
    pub nav_icon: NavPointIconEnum,
    /// `respawnLocationType` — respawn classification.
    #[serde(with = "enum_serde::respawn")]
    pub respawn: ERespawnLocationType,
    /// `parent` — hierarchy parent (another [`Location`]), if any.
    pub parent: Option<Guid>,
    /// `jurisdiction` — law-system record GUID (a bare reference, not followed
    /// onto the typed surface here).
    pub jurisdiction: Option<Guid>,
}

impl Location {
    /// Resolve the display name (`name` key) through a [`LocaleMap`]. Returns
    /// `None` when there's no key or it resolves to empty text.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        let key = self.name_key.as_ref()?;
        let name = locale.resolve(key)?;
        (!name.is_empty()).then_some(name)
    }
}

/// Every location, indexed by GUID and by class-CRC, with parent→children
/// adjacency for hierarchy traversal. Build once via [`Locations::build`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "LocationsRepr", into = "LocationsRepr")]
pub struct Locations {
    by_guid: HashMap<Guid, Location>,
    /// Derived: `class_crc → guid`. Rebuilt, not serialized.
    by_crc: HashMap<u32, Guid>,
    /// Derived: `parent guid → child guids` (sorted). Rebuilt, not serialized.
    children: HashMap<Guid, Vec<Guid>>,
}

/// Serialization shadow: only `by_guid` is persisted; the derived `by_crc` and
/// `children` indices are recomputed on the way back in.
#[derive(Serialize, Deserialize)]
struct LocationsRepr {
    by_guid: HashMap<Guid, Location>,
}

impl From<LocationsRepr> for Locations {
    fn from(repr: LocationsRepr) -> Self {
        let mut locations = Locations {
            by_guid: repr.by_guid,
            by_crc: HashMap::new(),
            children: HashMap::new(),
        };
        locations.rebuild_indices();
        locations
    }
}

impl From<Locations> for LocationsRepr {
    fn from(locations: Locations) -> Self {
        LocationsRepr {
            by_guid: locations.by_guid,
        }
    }
}

impl Locations {
    /// Empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build from a collection of [`Location`]s (keyed by their `guid`),
    /// recomputing the derived CRC and hierarchy indices. The primary entry
    /// points ([`build`](Self::build) / [`LocationsBuilder`]) delegate here.
    pub fn from_locations(locations: impl IntoIterator<Item = Location>) -> Self {
        let by_guid = locations.into_iter().map(|l| (l.guid, l)).collect();
        let mut index = Locations {
            by_guid,
            by_crc: HashMap::new(),
            children: HashMap::new(),
        };
        index.rebuild_indices();
        index
    }

    /// Build the index from a parsed [`RecordStore`] by walking the typed
    /// `StarMapObject` pool. Returns an owned index — build once, share
    /// `&Locations`.
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let locations = store
            .records
            .multi_feature
            .star_map_object
            .iter()
            .filter_map(|(&guid, &handle)| {
                handle.get(pools).map(|obj| location_for(guid, obj, store))
            });
        Self::from_locations(locations)
    }

    /// Recompute `by_crc` and `children` from `by_guid`.
    fn rebuild_indices(&mut self) {
        let mut by_crc = HashMap::with_capacity(self.by_guid.len());
        let mut children: HashMap<Guid, Vec<Guid>> = HashMap::new();
        for (&guid, loc) in &self.by_guid {
            let crc = class_crc(&guid);
            if let Some(prev) = by_crc.insert(crc, guid)
                && prev != guid
            {
                warn!(crc, %prev, new = %guid, "class_crc collision: Locations by_crc overwritten");
            }
            if let Some(parent) = loc.parent {
                children.entry(parent).or_default().push(guid);
            }
        }
        // Deterministic child order (the source HashMap iteration is not).
        for kids in children.values_mut() {
            kids.sort_by_key(|g| g.to_string());
        }
        self.by_crc = by_crc;
        self.children = children;
    }

    /// Resolve an EntityGraph wire CRC back to its location. The typed
    /// counterpart to [`sc_extract::CrcIndex`] — returns `&Location`.
    pub fn by_crc(&self, crc: u32) -> Option<&Location> {
        self.by_guid.get(self.by_crc.get(&crc)?)
    }

    /// Resolve an EntityGraph wire CRC back to its record GUID.
    pub fn guid_by_crc(&self, crc: u32) -> Option<Guid> {
        self.by_crc.get(&crc).copied()
    }

    /// The typed category of a location.
    pub fn kind(&self, guid: &Guid) -> Option<&LocationKind> {
        self.by_guid.get(guid).map(|l| &l.kind)
    }

    /// The hierarchy parent of a location, if any.
    pub fn parent_of(&self, guid: &Guid) -> Option<&Location> {
        let parent = self.by_guid.get(guid)?.parent?;
        self.by_guid.get(&parent)
    }

    /// The direct children of a location (places whose `parent` is `guid`),
    /// in deterministic order.
    pub fn children_of(&self, guid: &Guid) -> impl Iterator<Item = &Location> + '_ {
        self.children
            .get(guid)
            .into_iter()
            .flatten()
            .filter_map(move |g| self.by_guid.get(g))
    }

    /// The parent chain of a location, nearest first, up to the system root.
    /// Guards against a malformed `parent` cycle (caps the walk at the first
    /// repeat).
    pub fn ancestors(&self, guid: &Guid) -> impl Iterator<Item = &Location> + '_ {
        let mut current = self.by_guid.get(guid).and_then(|l| l.parent);
        let mut seen: HashSet<Guid> = HashSet::new();
        std::iter::from_fn(move || {
            let g = current?;
            if !seen.insert(g) {
                return None; // cycle guard
            }
            let loc = self.by_guid.get(&g)?;
            current = loc.parent;
            Some(loc)
        })
    }
}

impl sc_extract::RecordCollection for Locations {
    type Item = Location;

    fn get(&self, guid: &Guid) -> Option<&Location> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Location)> + '_ {
        self.by_guid.iter()
    }
}

/// Materialize the [`Location`] for one `StarMapObject`, resolving its category
/// through the `type` reference. Shared by [`Locations::build`] and
/// [`LocationsBuilder`].
fn location_for(guid: Guid, obj: &StarMapObject, store: &RecordStore) -> Location {
    let pools = &store.pools;
    let kind = obj
        .r#type
        .and_then(|g| StarMapObjectType::lookup(&store.records, &g))
        .and_then(|h| h.get(pools))
        .map(|t| LocationKind::from_dcb_str(&t.name))
        .unwrap_or_else(|| LocationKind::Unrecognized(String::new()));
    let callouts = [&obj.callout1, &obj.callout2, &obj.callout3]
        .into_iter()
        .filter_map(non_empty)
        .collect();
    Location {
        guid,
        name_key: non_empty(&obj.name),
        desc_key: non_empty(&obj.description),
        callouts,
        kind,
        nav_icon: obj.nav_icon.clone(),
        respawn: obj.respawn_location_type.clone(),
        parent: obj.parent,
        jurisdiction: obj.jurisdiction,
    }
}

/// [`sc_extract::RecordVisitor`] that builds a [`Locations`] in a bundled walk.
/// Declares interest in `StarMapObject` records. Equivalent to
/// [`Locations::build`] but fusible with other visitors in one pass.
#[derive(Default)]
pub struct LocationsBuilder {
    locations: Vec<Location>,
}

impl sc_extract::RecordVisitor for LocationsBuilder {
    type Output = Locations;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["StarMapObject"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let Some(handle) = StarMapObject::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(obj) = handle.get(&store.pools) else {
            return;
        };
        self.locations.push(location_for(item.guid, obj, store));
    }

    fn finish(self) -> Locations {
        Locations::from_locations(self.locations)
    }
}

/// Clone a [`LocaleKey`] only if it isn't the empty string.
fn non_empty(key: &LocaleKey) -> Option<LocaleKey> {
    (!key.as_str().is_empty()).then(|| key.clone())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn loc(guid: Guid, kind: LocationKind, parent: Option<Guid>) -> Location {
        Location {
            guid,
            name_key: None,
            desc_key: None,
            callouts: Vec::new(),
            kind,
            nav_icon: NavPointIconEnum::from_dcb_str("LandingZone"),
            respawn: ERespawnLocationType::from_dcb_str("Other"),
            parent,
            jurisdiction: None,
        }
    }

    #[test]
    fn location_kind_round_trips_known_and_unknown() {
        for s in [
            "Anomaly",
            "Asteroid_ValidQT",
            "LandingZone",
            "Manmade_VisibleOnInteraction",
            "Outpost_InvalidQT",
            "S42_Moon",
            "SolarSystem",
            "YouAreHere",
        ] {
            assert_eq!(LocationKind::from_dcb_str(s).as_dcb_str(), s);
        }
        // A future type the generator hasn't seen.
        let future = LocationKind::from_dcb_str("FutureKind");
        assert_eq!(future, LocationKind::Unrecognized("FutureKind".into()));
        assert_eq!(future.as_dcb_str(), "FutureKind");
    }

    #[test]
    fn crc_index_resolves_levski() {
        // The real Nyx_Levski StarMapObject RecordId → its EntityGraph
        // subject_id. Levski's live StarMapObjectType is `Manmade` (a station in
        // the Delamar asteroid), not LandingZone — its nav icon is LandingZone.
        let levski = Guid::from_str("468d4102-a210-47b5-8bc3-084f791a173c").unwrap();
        let mut l = loc(levski, LocationKind::Manmade, None);
        l.name_key = Some(LocaleKey::new("@ui_pregame_port_Levski_name"));
        let locs = Locations::from_locations([l]);

        assert_eq!(locs.guid_by_crc(3_723_364_946), Some(levski));
        assert_eq!(
            locs.by_crc(3_723_364_946).map(|l| l.kind.clone()),
            Some(LocationKind::Manmade)
        );
        assert_eq!(locs.kind(&levski), Some(&LocationKind::Manmade));
        // An unknown CRC resolves to nothing.
        assert_eq!(locs.guid_by_crc(1), None);
    }

    #[test]
    fn hierarchy_parent_children_ancestors() {
        let sys = Guid::from_bytes([1; 16]);
        let planet = Guid::from_bytes([2; 16]);
        let lz = Guid::from_bytes([3; 16]);
        let locs = Locations::from_locations([
            loc(sys, LocationKind::SolarSystem, None),
            loc(planet, LocationKind::Planet, Some(sys)),
            loc(lz, LocationKind::LandingZone, Some(planet)),
        ]);

        assert_eq!(locs.parent_of(&lz).map(|l| l.guid), Some(planet));
        assert_eq!(locs.parent_of(&sys), None);
        let kids: Vec<Guid> = locs.children_of(&sys).map(|l| l.guid).collect();
        assert_eq!(kids, vec![planet]);
        let chain: Vec<Guid> = locs.ancestors(&lz).map(|l| l.guid).collect();
        assert_eq!(chain, vec![planet, sys]);
    }

    #[test]
    fn ancestors_guards_against_cycle() {
        // a → b → a (malformed). The walk must terminate, not loop.
        let a = Guid::from_bytes([10; 16]);
        let b = Guid::from_bytes([11; 16]);
        let locs = Locations::from_locations([
            loc(a, LocationKind::Outpost, Some(b)),
            loc(b, LocationKind::Outpost, Some(a)),
        ]);
        let chain: Vec<Guid> = locs.ancestors(&a).map(|l| l.guid).collect();
        assert_eq!(chain, vec![b, a]); // stops at the first repeat
    }

    #[test]
    fn serde_round_trip_rebuilds_indices() {
        let sys = Guid::from_bytes([1; 16]);
        let lz = Guid::from_bytes([3; 16]);
        let locs = Locations::from_locations([
            loc(sys, LocationKind::SolarSystem, None),
            loc(lz, LocationKind::LandingZone, Some(sys)),
        ]);

        let json = serde_json::to_string(&locs).unwrap();
        // Derived indices are not persisted.
        assert!(!json.contains("by_crc"));
        assert!(!json.contains("children"));
        let decoded: Locations = serde_json::from_str(&json).unwrap();

        // ...but they're rebuilt on load.
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded.guid_by_crc(class_crc(&lz)), Some(lz));
        assert_eq!(
            decoded
                .children_of(&sys)
                .map(|l| l.guid)
                .collect::<Vec<_>>(),
            vec![lz]
        );
    }
}
