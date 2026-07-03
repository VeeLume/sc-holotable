//! [`Universe`] — the join of the two location structures.
//!
//! - the **DCB StarMap** ([`Locations`]) — the logical hierarchy of named places
//!   (system → star → planet → moon → landing zone → sub-POI), and
//! - the **socpak placement graph** ([`ObjectContainers`]) — the physical
//!   entities placed in the world (positions, containment, mission CRCs).
//!
//! They are separate structures (different sources, different keys) linked by
//! `placement.starmapRecord → StarMapObject` and by the mission DataSet CRC.
//! [`Universe`] merges them and exposes a [`Place`] — one node seen through
//! **both** facets, either of which may be absent (a logical-only place has no
//! physical build yet; a physical-only place is a nameless mission objective).
//!
//! **Direct vs. derived.** Accessors return the *direct* facet — the object's own
//! `name_key`, its own `position()`. Fallbacks (nearest-named-ancestor name,
//! parent-body position) are separate, opt-in helpers ([`Place::display_name`],
//! [`Place::anchor_position`]); they never replace the direct attribute.

use sc_extract::{Guid, LocaleKey, LocaleMap};

use crate::object_containers::{ObjectContainers, Orbit, Placement, PlacementId};
use crate::{Location, LocationKind, Locations, RecordCollection};

/// The joined universe: DCB StarMap + socpak placement graph.
pub struct Universe {
    star_map: Locations,
    containers: ObjectContainers,
}

impl Universe {
    /// Join a logical [`Locations`] with a physical [`ObjectContainers`].
    pub fn join(star_map: Locations, containers: ObjectContainers) -> Self {
        Self {
            star_map,
            containers,
        }
    }

    /// The logical structure (DCB StarMap).
    pub fn star_map(&self) -> &Locations {
        &self.star_map
    }
    /// The physical structure (socpak placement graph).
    pub fn containers(&self) -> &ObjectContainers {
        &self.containers
    }

    // ── point lookups → a Place ──────────────────────────────────────────

    /// By `StarMapObject` GUID.
    pub fn by_guid(&self, guid: Guid) -> Option<Place<'_>> {
        let location = self.star_map.get(&guid);
        let placement = self.best_placement_for(guid);
        (location.is_some() || placement.is_some()).then_some(Place {
            universe: self,
            location,
            placement,
        })
    }

    /// By EntityGraph wire CRC (`class_crc` of the GUID — the gRPC `subject_id`).
    pub fn by_class_crc(&self, crc: u32) -> Option<Place<'_>> {
        self.by_guid(self.star_map.guid_by_crc(crc)?)
    }

    /// By mission DataSet CRC (a gRPC mission dropoff/pickup id). The `Place`'s
    /// logical facet is the leaf's own `StarMapObject` (often `None` for an
    /// unnamed objective — use [`Place::display_name`] for the shown name).
    pub fn by_mission_crc(&self, crc: u32) -> Option<Place<'_>> {
        let pid = self.containers.by_mission_crc(crc)?;
        let location = self
            .containers
            .get(pid)
            .starmap_guid()
            .and_then(|g| self.star_map.get(&g));
        Some(Place {
            universe: self,
            location,
            placement: Some(pid),
        })
    }

    /// Places whose resolved name contains `needle` (case-insensitive). Linear.
    pub fn by_name(&self, locale: &LocaleMap, needle: &str) -> Vec<Place<'_>> {
        let needle = needle.to_ascii_lowercase();
        self.star_map
            .iter()
            .filter(|(_, l)| {
                l.display_name(locale)
                    .map(|n| n.to_ascii_lowercase().contains(&needle))
                    .unwrap_or(false)
            })
            .filter_map(|(g, _)| self.by_guid(*g))
            .collect()
    }

    // ── logical StarMap graph (as Places) ────────────────────────────────

    /// Roots of the logical hierarchy (systems / stars — no parent).
    pub fn roots(&self) -> Vec<Place<'_>> {
        self.star_map
            .iter()
            .filter(|(_, l)| l.parent.is_none())
            .filter_map(|(g, _)| self.by_guid(*g))
            .collect()
    }
    /// The logical parent of a place, if any.
    pub fn parent(&self, place: &Place<'_>) -> Option<Place<'_>> {
        self.by_guid(self.star_map.parent_of(&place.location?.guid)?.guid)
    }
    /// The direct logical children of a place.
    pub fn children(&self, place: &Place<'_>) -> Vec<Place<'_>> {
        let Some(loc) = place.location else {
            return Vec::new();
        };
        self.star_map
            .children_of(&loc.guid)
            .filter_map(|c| self.by_guid(c.guid))
            .collect()
    }
    /// The logical ancestor chain, nearest first.
    pub fn ancestors(&self, place: &Place<'_>) -> Vec<Place<'_>> {
        let Some(loc) = place.location else {
            return Vec::new();
        };
        self.star_map
            .ancestors(&loc.guid)
            .filter_map(|a| self.by_guid(a.guid))
            .collect()
    }

    /// Prefer a *positioned* placement for a star-map object; else any.
    fn best_placement_for(&self, guid: Guid) -> Option<PlacementId> {
        let ids = self.containers.placements_of(guid);
        ids.iter()
            .copied()
            .find(|id| self.containers.get(*id).position().is_some())
            .or_else(|| ids.first().copied())
    }
}

/// One universe node, seen through both facets. Either may be absent.
pub struct Place<'u> {
    universe: &'u Universe,
    location: Option<&'u Location>,
    placement: Option<PlacementId>,
}

impl<'u> Place<'u> {
    // ── the source records (escape hatch to the raw structures) ──────────

    /// The logical record (DCB `StarMapObject`), or `None` for a physical-only place.
    pub fn location(&self) -> Option<&'u Location> {
        self.location
    }
    /// The physical record (socpak placement), or `None` for a logical-only place.
    pub fn placement(&self) -> Option<&'u Placement> {
        self.placement.map(|id| self.universe.containers.get(id))
    }

    // ── DIRECT facets ────────────────────────────────────────────────────

    pub fn guid(&self) -> Option<Guid> {
        self.location
            .map(|l| l.guid)
            .or_else(|| self.placement().and_then(|p| p.starmap_guid()))
    }
    /// The place's own localized-name key — the raw [`LocaleKey`], resolution is
    /// separate ([`Self::name`]). `None` when the place has no star-map name.
    pub fn name_key(&self) -> Option<&'u LocaleKey> {
        self.location.and_then(|l| l.name_key.as_ref())
    }
    pub fn kind(&self) -> Option<&'u LocationKind> {
        self.location.map(|l| &l.kind)
    }
    /// The placement's own `Pos` — **direct**, never parent-substituted.
    pub fn position(&self) -> Option<[f64; 3]> {
        self.placement().and_then(|p| p.position())
    }
    pub fn orbit(&self) -> Option<Orbit> {
        self.placement().and_then(|p| p.orbit())
    }
    pub fn socpak(&self) -> Option<&'u str> {
        self.placement().map(|p| p.socpak())
    }
    /// Raw socpak entity `Name` — the identity for a place with no star-map name.
    pub fn entity_name(&self) -> Option<&'u str> {
        self.placement()
            .map(|p| p.entity_name())
            .filter(|s| !s.is_empty())
    }

    // ── DERIVED helpers (opt-in; never replace the direct facets) ─────────

    /// Resolve the place's **own** name key through `locale` (no fallback).
    pub fn name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.location.and_then(|l| l.display_name(locale))
    }

    /// The **shown** name: own name → nearest *named* ancestor (physical
    /// containment chain if placed, else logical hierarchy). Does not synthesize
    /// from the raw entity name — use [`Self::entity_name`] for that.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        if let Some(n) = self.name(locale) {
            return Some(n);
        }
        // Physical chain: how the game names an unnamed sub-location.
        if let Some(pid) = self.placement {
            for aid in self
                .universe
                .containers
                .chain_of(pid)
                .into_iter()
                .rev()
                .skip(1)
            {
                if let Some(g) = self.universe.containers.get(aid).starmap_guid()
                    && let Some(n) = self
                        .universe
                        .star_map
                        .get(&g)
                        .and_then(|l| l.display_name(locale))
                {
                    return Some(n);
                }
            }
        }
        // Logical hierarchy fallback.
        if let Some(l) = self.location {
            for a in self.universe.star_map.ancestors(&l.guid) {
                if let Some(n) = a.display_name(locale) {
                    return Some(n);
                }
            }
        }
        None
    }

    /// **System-global** position of this place's placement (sums the containment
    /// chain; ignores per-level rotation). The coordinate to use for ordering
    /// stops by distance. `None` for a logical-only place — use
    /// [`Self::anchor_position`] to fall back to a positioned ancestor.
    pub fn global_position(&self) -> Option<[f64; 3]> {
        self.placement
            .and_then(|id| self.universe.containers.global_position(id))
    }

    /// Position with parent fallback: own (global) position → the first *logical*
    /// ancestor (parent body) that has a global position. Good enough for ordering
    /// by which body a stop sits at, even when the stop itself isn't placed.
    pub fn anchor_position(&self) -> Option<[f64; 3]> {
        if let Some(p) = self.global_position() {
            return Some(p);
        }
        let l = self.location?;
        self.universe.star_map.ancestors(&l.guid).find_map(|a| {
            self.universe
                .by_guid(a.guid)
                .and_then(|p| p.global_position())
        })
    }
}
