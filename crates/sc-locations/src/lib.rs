//! Typed surface over Star Citizen `StarMapObject` records — the universe's
//! stable *places*: solar systems, planets, moons, landing zones, rest stops,
//! stations, outposts, mining claims.
//!
//! A [`Location`] is the curated view of one `StarMapObject`: localized name,
//! typed [`LocationKind`] category, and hierarchy parent. [`Locations`] indexes
//! every location by GUID **and by class-CRC**, so an EntityGraph gRPC
//! `subject_id` (a [`sc_extract::class_crc`] of the record GUID) resolves
//! straight to a typed location.
//!
//! Two more layers build on it. [`ObjectContainers`] is the *physical* side: one
//! tolerant walk over every `.socpak`, yielding the placement graph (positions,
//! containment, mission CRCs, which socpak realizes which `StarMapObject`).
//! [`Universe`] joins the two into [`Place`]s that expose both facets, either of
//! which may be absent.
//!
//! The category is the `StarMapObjectType` the record references, **not** its
//! nav icon — Levski is `Manmade`, not `LandingZone`. There is no dedicated
//! Lagrange or asteroid-cluster kind: those are `Asteroid` / `AsteroidValidQt`,
//! jump points are `Anomaly`, stations are `Manmade`. The hierarchy is rooted at
//! the `Star`; the `SolarSystem` record is a parentless, childless sibling,
//! which [`Locations::system_of`] joins a place to through the system's
//! `SSolarSystem` record.
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
    ERespawnLocationType, NavPointIconEnum, RecordLookup, SSolarSystem, StarMapObject,
    StarMapObjectType,
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
mod waypoints;
pub use waypoints::{Waypoint, Waypoints};
mod bodies;
pub use bodies::{Bodies, Body, OM_OFFSET_M};
mod jump_links;
pub use jump_links::{JumpLink, JumpLinks};

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
    /// `quantumTravelData`: where quantum travel to it ends and what it
    /// blocks.
    #[serde(default)]
    pub quantum: Option<QuantumTravel>,
}

/// `StarMapObject.quantumTravelData`, m. Hurston: arrival 1,050,000,
/// obstruction 1,008,900.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QuantumTravel {
    /// `arrivalRadius`: quantum travel drops out this far from the centre.
    pub arrival_radius: f32,
    /// `obstructionRadius`: a route passing closer is blocked.
    pub obstruction_radius: f32,
    /// `adoptionRadius`. On a planet with moons it encloses them (Hurston
    /// 128,354,900), so it is a parent zone, not the body's spin frame.
    pub adoption_radius: f32,
    /// `arrivalPointDetectionOffset`. Varies per body (Hurston 10,000,
    /// Crusader 200,000); not the 10 km in [`Body::om_radius`].
    #[serde(default)]
    pub arrival_point_detection_offset: f32,
    /// `subPointRadiusMultiplier`: 1.5 on almost every object (Crusader
    /// 1.512, stars 7.5 and 25). Its use is unknown: it does not scale the
    /// orbital markers.
    #[serde(default)]
    pub sub_point_radius_multiplier: f32,
}

// `QuantumTravel` holds floats; equality on a location is still exact.
impl Eq for QuantumTravel {}

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
    /// The `SSolarSystem` records: (`DefaultLocation`, `SolarSystemRecord`).
    systems: Vec<SystemLink>,
    /// Derived: `class_crc → guid`. Rebuilt, not serialized.
    by_crc: HashMap<u32, Guid>,
    /// Derived: `parent guid → child guids` (sorted). Rebuilt, not serialized.
    children: HashMap<Guid, Vec<Guid>>,
    /// Derived: hierarchy root (the star) → solar-system location. Rebuilt,
    /// not serialized.
    system_by_root: HashMap<Guid, Guid>,
}

/// One `SSolarSystem` record, as far as the hierarchy needs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct SystemLink {
    /// `DefaultLocation`: a place in the system, whose chain gives its star.
    default_location: Guid,
    /// `SolarSystemRecord`: the system's own `StarMapObject`.
    system: Guid,
}

/// Serialization shadow: `by_guid` and the system links are persisted; the
/// derived indices are recomputed on the way back in.
#[derive(Serialize, Deserialize)]
struct LocationsRepr {
    by_guid: HashMap<Guid, Location>,
    #[serde(default)]
    systems: Vec<SystemLink>,
}

impl From<LocationsRepr> for Locations {
    fn from(repr: LocationsRepr) -> Self {
        Locations::assemble(repr.by_guid, repr.systems)
    }
}

impl From<Locations> for LocationsRepr {
    fn from(locations: Locations) -> Self {
        LocationsRepr {
            by_guid: locations.by_guid,
            systems: locations.systems,
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
    /// No solar-system links: [`system_of`](Self::system_of) is `None`.
    pub fn from_locations(locations: impl IntoIterator<Item = Location>) -> Self {
        let by_guid = locations.into_iter().map(|l| (l.guid, l)).collect();
        Self::assemble(by_guid, Vec::new())
    }

    fn assemble(by_guid: HashMap<Guid, Location>, systems: Vec<SystemLink>) -> Self {
        let mut index = Locations {
            by_guid,
            systems,
            by_crc: HashMap::new(),
            children: HashMap::new(),
            system_by_root: HashMap::new(),
        };
        index.rebuild_indices();
        index
    }

    /// Build the index from a parsed [`RecordStore`] by walking the typed
    /// `StarMapObject` pool, plus the `SSolarSystem` records for
    /// [`system_of`](Self::system_of). Returns an owned index — build once,
    /// share `&Locations`.
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let by_guid = store
            .records
            .multi_feature
            .star_map_object
            .iter()
            .filter_map(|(&guid, &handle)| {
                handle
                    .get(pools)
                    .map(|obj| (guid, location_for(guid, obj, store)))
            })
            .collect();
        let systems = store
            .records
            .multi_feature
            .ssolar_system
            .values()
            .filter_map(|h| system_link(h.get(pools)?))
            .collect();
        Self::assemble(by_guid, systems)
    }

    /// The solar system a place is in: the `StarMapObject` that system's
    /// `SSolarSystem.SolarSystemRecord` names (`PyroSolarSystem`). The
    /// hierarchy does not reach it (it has no children); the join is the
    /// star: a place belongs to the system whose `DefaultLocation` has the
    /// same root. `None` for a place under no system's star, and for the
    /// system location itself.
    ///
    /// ```no_run
    /// # fn system(locations: &sc_locations::Locations, place: &sc_extract::Guid) {
    /// if let Some(system) = locations.system_of(place) {
    ///     println!("in {}", system.guid);
    /// }
    /// # }
    /// ```
    pub fn system_of(&self, guid: &Guid) -> Option<&Location> {
        let root = self.ancestors(guid).last().map_or(*guid, |l| l.guid);
        self.by_guid.get(self.system_by_root.get(&root)?)
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
        let mut system_by_root = HashMap::new();
        for link in &self.systems {
            if !self.by_guid.contains_key(&link.default_location) {
                continue;
            }
            let root = self
                .ancestors(&link.default_location)
                .last()
                .map_or(link.default_location, |l| l.guid);
            if let Some(prev) = system_by_root.insert(root, link.system)
                && prev != link.system
            {
                warn!(%root, %prev, new = %link.system, "two solar systems share a star");
            }
        }
        self.system_by_root = system_by_root;
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
        quantum: obj
            .quantum_travel_data
            .and_then(|h| h.get(pools))
            .map(|q| QuantumTravel {
                arrival_radius: q.arrival_radius,
                obstruction_radius: q.obstruction_radius,
                adoption_radius: q.adoption_radius,
                arrival_point_detection_offset: q.arrival_point_detection_offset,
                sub_point_radius_multiplier: q.sub_point_radius_multiplier,
            }),
    }
}

/// One `SSolarSystem` record's link, when it names both ends.
fn system_link(ss: &SSolarSystem) -> Option<SystemLink> {
    Some(SystemLink {
        default_location: ss.default_location?,
        system: ss.solar_system_record?,
    })
}

/// [`sc_extract::RecordVisitor`] that builds a [`Locations`] in a bundled walk.
/// Declares interest in `StarMapObject` and `SSolarSystem` records.
/// Equivalent to [`Locations::build`] but fusible with other visitors in one
/// pass.
#[derive(Default)]
pub struct LocationsBuilder {
    locations: Vec<Location>,
    systems: Vec<SystemLink>,
}

impl sc_extract::RecordVisitor for LocationsBuilder {
    type Output = Locations;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["StarMapObject", "SSolarSystem"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        if let Some(handle) = SSolarSystem::lookup(&store.records, &item.guid) {
            if let Some(link) = handle.get(&store.pools).and_then(system_link) {
                self.systems.push(link);
            }
            return;
        }
        let Some(handle) = StarMapObject::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(obj) = handle.get(&store.pools) else {
            return;
        };
        self.locations.push(location_for(item.guid, obj, store));
    }

    fn finish(self) -> Locations {
        let by_guid = self.locations.into_iter().map(|l| (l.guid, l)).collect();
        Locations::assemble(by_guid, self.systems)
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
            quantum: None,
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

    /// A place joins the system whose `DefaultLocation` shares its star; the
    /// links survive a serde round trip.
    #[test]
    fn system_of_joins_through_the_star() {
        let g = |b| Guid::from_bytes([b; 16]);
        let (star, planet, lz, system) = (g(20), g(21), g(22), g(23));
        let (other_star, other_moon) = (g(30), g(31));
        let by_guid = [
            loc(star, LocationKind::Star, None),
            loc(planet, LocationKind::Planet, Some(star)),
            loc(lz, LocationKind::LandingZone, Some(planet)),
            loc(system, LocationKind::SolarSystem, None),
            loc(other_star, LocationKind::Star, None),
            loc(other_moon, LocationKind::Moon, Some(other_star)),
        ]
        .into_iter()
        .map(|l| (l.guid, l))
        .collect();
        let locs = Locations::assemble(
            by_guid,
            vec![SystemLink {
                default_location: lz,
                system,
            }],
        );
        for place in [lz, planet, star] {
            assert_eq!(locs.system_of(&place).map(|l| l.guid), Some(system));
        }
        assert_eq!(locs.system_of(&other_moon), None);
        assert_eq!(locs.system_of(&system), None);
        assert_eq!(
            Locations::from_locations(locs.values().cloned()).system_of(&lz),
            None
        );
        let back: Locations = serde_json::from_str(&serde_json::to_string(&locs).unwrap()).unwrap();
        assert_eq!(back.system_of(&planet).map(|l| l.guid), Some(system));
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
