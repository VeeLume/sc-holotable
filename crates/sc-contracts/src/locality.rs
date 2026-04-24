//! Locality / mission-span resolution.
//!
//! Contracts can gate availability on a `MissionLocality` record via
//! `ContractPrerequisite_Locality.locality_available`. Each locality
//! fans out to a `Vec<CigGuid>` of [`StarMapObject`] records that list
//! where the mission is offered (planets, moons, stations, asteroid
//! clusters, …).
//!
//! This module turns those raw GUIDs into:
//!
//! - [`LocationRef`] — one star-map object with `(system, body)`
//!   classified through its parent chain.
//! - [`LocalityView`] — one `MissionLocality` resolved to a list of
//!   [`LocationRef`] and the distinct systems it touches.
//! - [`LocationRegistry`] / [`LocalityRegistry`] — walk-once lookups
//!   used by [`crate::expand_all`] to populate
//!   [`crate::ExpandedContract::mission_span`].
//!
//! Parent-chain traversal is the data-driven part (workspace principle
//! §5): `StarMapObject.parent` walked up to the root yields the
//! *system root* for free. Record-name parsing is used only for human
//! display labels (`Pyro III (Bloom)`, `Hurston`) where no typed
//! alternative exists in the DCB.
//!
//! # Why it matters
//!
//! Regional blueprint-reward conflicts (Adagio Pyro Region A/B vs
//! C/D) are invisible on a contract's rewards alone — two contracts
//! with the same title and description can ship different
//! [`crate::BlueprintReward`]s. `mission_span` surfaces the region /
//! system the contract covers, so [`crate::find_bp_conflicts`]
//! consumers can show *why* the variants diverge and annotate the
//! relevant region at display time.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use sc_extract::generated::{MissionLocality, StarMapObject};
use sc_extract::{Datacore, Guid, LocaleMap};

// ── Types ──────────────────────────────────────────────────────────────────

/// Broad star-system grouping used by [`LocationRef`]. Derived from
/// the root-most `StarMapObject` parent's record-name prefix —
/// `Stanton` / `Pyro` / `Nyx`. New SC systems (future patches) fall
/// through to `Other(name)` with the raw record name preserved so
/// consumers can still render them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SystemKey {
    Stanton,
    Pyro,
    Nyx,
    /// Unknown system root — carries the raw `StarMapObject` name
    /// (`StarMapObject.` prefix stripped). Covers both "CIG added a
    /// system we don't recognise" and "this location has no parent
    /// chain we could walk" cases.
    Other(String),
}

impl SystemKey {
    fn from_root_name(name: &str) -> Self {
        let stem = name.strip_prefix("StarMapObject.").unwrap_or(name);
        // Roots in the live DCB show up in three shapes:
        //   - System-name-only (`Stanton`, `Nyx`)
        //   - `<System>Star` (`PyroStar`)
        //   - `<System>SolarSystem` (`NyxSolarSystem`)
        // Normalise by stripping the common suffixes before
        // matching — same enum for all three forms.
        let normalised = stem
            .strip_suffix("SolarSystem")
            .or_else(|| stem.strip_suffix("Star"))
            .unwrap_or(stem);
        match normalised {
            "Stanton" => SystemKey::Stanton,
            "Pyro" => SystemKey::Pyro,
            "Nyx" => SystemKey::Nyx,
            _ => SystemKey::Other(stem.to_string()),
        }
    }

    /// Human-friendly name for display. `Other` returns its raw name.
    pub fn display(&self) -> &str {
        match self {
            SystemKey::Stanton => "Stanton",
            SystemKey::Pyro => "Pyro",
            SystemKey::Nyx => "Nyx",
            SystemKey::Other(s) => s.as_str(),
        }
    }
}

/// One resolved `StarMapObject`. Names are stripped of the
/// `StarMapObject.` prefix for display; the `guid` remains the
/// canonical identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationRef {
    pub guid: Guid,
    /// Record name with `StarMapObject.` prefix stripped
    /// (`Pyro3_L1`, `RR_P6_LEO_CLINIC`, `Stanton4b`, …). Empty if the
    /// DCB record lacks a name (shouldn't happen on clean data).
    pub record_name: String,
    /// Localized player-facing name from `StarMapObject.name`
    /// (`Sirus`, `Bloom`, `Hurston`, `New Babbage`, …). Empty when
    /// the locale map has no entry (asteroid clusters, nav points,
    /// and other CIG-internal records that never appear on a player
    /// HUD tend to have empty names).
    pub display_name: String,
    /// System derived from the parent-chain root.
    pub system: SystemKey,
    /// Body-level ancestor's stripped record name, when one exists
    /// between this location and the system root. `None` for
    /// system-level locations (the star itself) and for locations
    /// whose parent chain is one hop (direct children of root).
    ///
    /// Example: `StarMapObject.Pyro3_L1` (Pyro III Lagrange 1) has
    /// `body = Some("Pyro3")`, because its parent `Pyro3` sits
    /// directly below the `Pyro` root.
    pub body: Option<String>,
    /// Localized name of the body ancestor (`Bloom`, `Hurston`, …).
    /// Populated when `body` is `Some` and the body's `name` locale
    /// key resolved. Empty otherwise.
    pub body_display_name: String,
}

impl LocationRef {
    /// Convenience: `"Pyro / Pyro3"` or `"Stanton"` — a short tag
    /// suitable for badges and census output.
    pub fn short_tag(&self) -> String {
        match &self.body {
            Some(b) => format!("{} / {}", self.system.display(), b),
            None => self.system.display().to_string(),
        }
    }

    /// Human-friendly tag preferring localized body and system
    /// names: `"Pyro / Bloom"`, `"Stanton / Hurston"`, or
    /// `"Stanton"` for system-level locations. Falls back to
    /// `short_tag` when localized names aren't available.
    pub fn human_tag(&self) -> String {
        if self.body.is_some() && !self.body_display_name.is_empty() {
            format!("{} / {}", self.system.display(), self.body_display_name)
        } else {
            self.short_tag()
        }
    }
}

/// One resolved `MissionLocality`.
#[derive(Debug, Clone)]
pub struct LocalityView {
    pub guid: Guid,
    /// Record name with `MissionLocality.` prefix stripped. Empty if
    /// the DCB record has no name. Useful as a stable identifier
    /// (`RegionA`, `RegionC`) — see [`region_label`][Self::region_label]
    /// for a player-facing summary.
    pub name: String,
    /// Every location this locality points at, resolved through
    /// [`LocationRegistry`]. GUIDs the registry couldn't resolve are
    /// dropped silently (feature-gated types or DCB breakage are the
    /// only causes — the default crate build has `contracts`
    /// feature so all observed data resolves).
    pub locations: Vec<LocationRef>,
    /// Distinct systems covered by `locations`. Populated during
    /// registry build so consumers annotating Contracts don't have
    /// to recompute the set per render.
    pub systems: BTreeSet<SystemKey>,
    /// Player-facing one-line summary of what bodies this locality
    /// spans, built from the localized body names of `locations`.
    ///
    /// Shape (in rough preference order):
    /// - `"Pyro: Sirus, Monox, Bloom"` — multiple bodies in one system
    /// - `"Pyro: Bloom"` — single body in one system
    /// - `"Pyro (system-wide)"` — only system-level locations (no body ancestor)
    /// - `"Stanton + Pyro"` — crosses multiple systems; no body detail
    /// - `""` — empty locality (shouldn't happen on clean DCB data)
    ///
    /// Built once during registry construction so downstream
    /// rendering is a string clone.
    pub region_label: String,
}

impl LocalityView {
    /// True when the locality touches more than one system —
    /// the Pyro cross-system salvage span that motivates this model.
    pub fn spans_multiple_systems(&self) -> bool {
        self.systems.len() > 1
    }
}

// ── Registries ─────────────────────────────────────────────────────────────

/// Resolves every `StarMapObject` GUID the DCB exposes into a
/// [`LocationRef`] with its parent chain walked to classify the
/// system + body.
#[derive(Debug, Clone, Default)]
pub struct LocationRegistry {
    by_guid: HashMap<Guid, LocationRef>,
}

impl LocationRegistry {
    /// Walk every `StarMapObject` record in the DCB, resolve its
    /// parent chain, and cache a [`LocationRef`].
    ///
    /// Parent-chain traversal is iterative with a cycle guard
    /// (should never happen on real data, but the DCB is untrusted
    /// input). Unresolvable parents terminate the walk — the
    /// object's own name still ends up as both the location name
    /// and (if it's the top it could find) the system key.
    ///
    /// `locale` resolves `StarMapObject.name` into the localized
    /// display string (`"Sirus"`, `"Hurston"`) cached on
    /// [`LocationRef::display_name`]. Pass the same `LocaleMap`
    /// already in use for blueprint / description resolution.
    pub fn build(datacore: &Datacore, locale: &LocaleMap) -> Self {
        let pools = &datacore.records().pools;
        let records = &datacore.records().records;
        let db = datacore.db();

        // Pre-resolve every StarMapObject's display_name via locale,
        // keyed by record GUID. Used twice: once for each location's
        // own display_name, once for its body ancestor's display_name
        // after parent-chain walking.
        let mut display_by_guid: HashMap<Guid, String> =
            HashMap::with_capacity(records.multi_feature.star_map_object.len());
        for (guid, handle) in &records.multi_feature.star_map_object {
            if let Some(obj) = handle.get(pools) {
                let name_key = obj.name.stripped();
                let display = if name_key.is_empty() {
                    String::new()
                } else {
                    locale.get(name_key).unwrap_or("").to_string()
                };
                display_by_guid.insert(*guid, display);
            }
        }

        let mut by_guid: HashMap<Guid, LocationRef> =
            HashMap::with_capacity(records.multi_feature.star_map_object.len());

        for (guid, handle) in &records.multi_feature.star_map_object {
            let Some(obj) = handle.get(pools) else { continue };

            let name = record_name_stripped(db, guid, "StarMapObject.");
            let display_name = display_by_guid.get(guid).cloned().unwrap_or_default();

            // Walk parent chain. `root_name` is the top-most
            // ancestor's record name. `body_guid` is the node
            // *directly below* the root, when there's at least one
            // intermediate step between `guid` and the root.
            let (root_name, body_guid) =
                walk_parents(guid, obj, &records.multi_feature.star_map_object, pools, db);
            let body_name = body_guid
                .map(|g| record_name_stripped(db, &g, "StarMapObject."))
                .filter(|s| !s.is_empty());
            let body_display_name = body_guid
                .and_then(|g| display_by_guid.get(&g).cloned())
                .unwrap_or_default();

            by_guid.insert(
                *guid,
                LocationRef {
                    guid: *guid,
                    record_name: name,
                    display_name,
                    system: SystemKey::from_root_name(&root_name),
                    body: body_name,
                    body_display_name,
                },
            );
        }

        Self { by_guid }
    }

    /// Number of resolved `StarMapObject` records.
    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }

    /// Look up a location by its `StarMapObject` GUID.
    pub fn get(&self, guid: &Guid) -> Option<&LocationRef> {
        self.by_guid.get(guid)
    }

    pub fn iter(&self) -> impl Iterator<Item = &LocationRef> + '_ {
        self.by_guid.values()
    }
}

/// Resolves `MissionLocality` GUIDs into [`LocalityView`]s with each
/// referenced location already classified via [`LocationRegistry`].
#[derive(Debug, Clone, Default)]
pub struct LocalityRegistry {
    by_guid: HashMap<Guid, LocalityView>,
    /// Running count of locality entries whose `available_locations`
    /// array held a GUID the `LocationRegistry` couldn't resolve.
    /// Exposed for diagnostics; typically zero on clean DCBs.
    unresolved_location_refs: usize,
}

impl LocalityRegistry {
    /// Build from a pre-built [`LocationRegistry`].
    pub fn build(datacore: &Datacore, locations: &LocationRegistry) -> Self {
        let pools = &datacore.records().pools;
        let records = &datacore.records().records;
        let db = datacore.db();

        let mut by_guid: HashMap<Guid, LocalityView> =
            HashMap::with_capacity(records.multi_feature.mission_locality.len());
        let mut unresolved_location_refs = 0usize;

        for (guid, handle) in &records.multi_feature.mission_locality {
            let Some(locality): Option<&MissionLocality> = handle.get(pools) else {
                continue;
            };

            let name = record_name_stripped(db, guid, "MissionLocality.");

            let mut resolved_locations = Vec::with_capacity(locality.available_locations.len());
            let mut systems: BTreeSet<SystemKey> = BTreeSet::new();
            for loc_guid in &locality.available_locations {
                if let Some(loc) = locations.get(loc_guid) {
                    systems.insert(loc.system.clone());
                    resolved_locations.push(loc.clone());
                } else {
                    unresolved_location_refs += 1;
                }
            }
            // Stable output: sort by (system, body, record_name).
            resolved_locations.sort_by(|a, b| {
                a.system
                    .cmp(&b.system)
                    .then_with(|| a.body.cmp(&b.body))
                    .then_with(|| a.record_name.cmp(&b.record_name))
            });

            let region_label = build_region_label(&resolved_locations, &systems);

            by_guid.insert(
                *guid,
                LocalityView {
                    guid: *guid,
                    name,
                    locations: resolved_locations,
                    systems,
                    region_label,
                },
            );
        }

        Self {
            by_guid,
            unresolved_location_refs,
        }
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }

    pub fn get(&self, guid: &Guid) -> Option<&LocalityView> {
        self.by_guid.get(guid)
    }

    pub fn iter(&self) -> impl Iterator<Item = &LocalityView> + '_ {
        self.by_guid.values()
    }

    /// Number of location GUIDs across all localities that the
    /// [`LocationRegistry`] couldn't resolve. Non-zero means either
    /// the DCB has a dangling reference, or a `StarMapObject`
    /// subtype is behind a feature gate we don't have enabled.
    pub fn unresolved_location_refs(&self) -> usize {
        self.unresolved_location_refs
    }
}

// ── Internals ──────────────────────────────────────────────────────────────

/// Summarise a locality's locations into a player-facing label.
/// Aggregates by (system, body) and prefers localized body names —
/// falls back to the record-name stem (`Pyro3`) when the display
/// name is empty (asteroid clusters, unnamed nav points).
///
/// Grouping rules:
/// - Multiple systems → `"Stanton + Pyro"`, no per-body detail.
///   Cross-system localities are rare enough that the per-system
///   body list would be noise.
/// - Single system, at least one body resolved → `"Pyro: Sirus, Bloom, Monox"`.
///   Bodies sorted alphabetically; record-name fallbacks are
///   deduplicated against display names (so `Pyro3` doesn't appear
///   alongside `Bloom` when they're the same body).
/// - Single system, only system-level locations → `"Pyro (system-wide)"`.
///   These localities list stations / asteroid clusters without a
///   planet ancestor.
/// - Empty locality → `""`. Shouldn't occur in clean DCB data.
fn build_region_label(locations: &[LocationRef], systems: &BTreeSet<SystemKey>) -> String {
    if locations.is_empty() {
        return String::new();
    }

    if systems.len() > 1 {
        let joined: Vec<&str> = systems.iter().map(SystemKey::display).collect();
        return joined.join(" + ");
    }

    // Single-system path.
    let Some(sys) = systems.iter().next() else {
        return String::new();
    };

    // Deduplicate bodies. Prefer the localized display name when
    // present; fall back to the record-name stem. Track both forms
    // together so one body never appears twice under different
    // names.
    let mut bodies: BTreeMap<String, String> = BTreeMap::new();
    let mut system_level_count = 0usize;
    for loc in locations {
        match (&loc.body, loc.body_display_name.is_empty()) {
            (Some(body_rec), false) => {
                bodies
                    .entry(body_rec.clone())
                    .or_insert_with(|| loc.body_display_name.clone());
            }
            (Some(body_rec), true) => {
                bodies
                    .entry(body_rec.clone())
                    .or_insert_with(|| body_rec.clone());
            }
            (None, _) => {
                system_level_count += 1;
            }
        }
    }

    if bodies.is_empty() {
        // All locations are stations / asteroids with no planet
        // ancestor — mark as system-wide so the label still conveys
        // scale.
        let _ = system_level_count;
        return format!("{} (system-wide)", sys.display());
    }

    // Sort body labels for stable output.
    let mut body_labels: Vec<String> = bodies.into_values().collect();
    body_labels.sort();
    body_labels.dedup();

    // Cap at 5 bodies with a "+N more" suffix so a locality covering
    // half the system stays on a single line. Player-facing callers
    // with space for the full list can walk `locations` directly.
    const MAX_BODIES: usize = 5;
    if body_labels.len() > MAX_BODIES {
        let extra = body_labels.len() - MAX_BODIES;
        let head: Vec<String> = body_labels.into_iter().take(MAX_BODIES).collect();
        format!("{}: {}, +{} more", sys.display(), head.join(", "), extra)
    } else {
        format!("{}: {}", sys.display(), body_labels.join(", "))
    }
}

/// Read a root record's name via svarog and strip a known prefix.
/// Returns the empty string when the record has no name.
fn record_name_stripped(
    db: &sc_extract::svarog_datacore::DataCoreDatabase,
    guid: &Guid,
    prefix: &str,
) -> String {
    let Some(record) = db.record(guid) else {
        return String::new();
    };
    let Some(name) = record.name() else {
        return String::new();
    };
    name.strip_prefix(prefix).unwrap_or(name).to_string()
}

/// Walk the parent chain of a StarMapObject up to the root. Returns
/// `(root_record_name, body_guid_when_distinct)`. The body GUID is
/// surfaced only when there's at least one intermediate ancestor
/// between the starting node and the root — callers can then look up
/// the body's record name and localized display name separately.
fn walk_parents(
    start_guid: &Guid,
    start_obj: &StarMapObject,
    by_guid: &HashMap<Guid, sc_extract::generated::Handle<StarMapObject>>,
    pools: &sc_extract::generated::DataPools,
    db: &sc_extract::svarog_datacore::DataCoreDatabase,
) -> (String, Option<Guid>) {
    use std::collections::HashSet;

    let mut visited: HashSet<Guid> = HashSet::new();
    visited.insert(*start_guid);

    // chain[0] = start, chain[1] = parent, … chain[n] = root.
    let mut chain: Vec<(Guid, &StarMapObject)> = vec![(*start_guid, start_obj)];
    let mut cursor_parent = start_obj.parent;

    while let Some(pg) = cursor_parent {
        if !visited.insert(pg) {
            break; // cycle guard
        }
        let Some(handle) = by_guid.get(&pg) else {
            break;
        };
        let Some(parent_obj) = handle.get(pools) else {
            break;
        };
        chain.push((pg, parent_obj));
        cursor_parent = parent_obj.parent;
    }

    let (root_guid, _root_obj) = *chain.last().unwrap();
    let root_name = record_name_stripped(db, &root_guid, "StarMapObject.");

    let body_guid = if chain.len() >= 3 {
        Some(chain[chain.len() - 2].0)
    } else {
        None
    };

    (root_name, body_guid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_key_classifies_known_roots() {
        assert_eq!(
            SystemKey::from_root_name("StarMapObject.Stanton"),
            SystemKey::Stanton
        );
        assert_eq!(SystemKey::from_root_name("Pyro"), SystemKey::Pyro);
        assert_eq!(SystemKey::from_root_name("Nyx"), SystemKey::Nyx);
        // Real DCB: Pyro's root StarMapObject is named `PyroStar`.
        assert_eq!(
            SystemKey::from_root_name("StarMapObject.PyroStar"),
            SystemKey::Pyro
        );
        // Real DCB: Nyx's root can appear as `NyxSolarSystem`.
        assert_eq!(
            SystemKey::from_root_name("StarMapObject.NyxSolarSystem"),
            SystemKey::Nyx
        );
        assert_eq!(
            SystemKey::from_root_name("StarMapObject.FutureSystem"),
            SystemKey::Other("FutureSystem".to_string())
        );
    }

    #[test]
    fn empty_registries_are_empty() {
        assert!(LocationRegistry::default().is_empty());
        assert!(LocalityRegistry::default().is_empty());
    }

    #[test]
    fn location_short_tag_formats() {
        let loc = LocationRef {
            guid: Guid::default(),
            record_name: "Pyro3_L1".to_string(),
            display_name: String::new(),
            system: SystemKey::Pyro,
            body: Some("Pyro3".to_string()),
            body_display_name: "Bloom".to_string(),
        };
        assert_eq!(loc.short_tag(), "Pyro / Pyro3");
        assert_eq!(loc.human_tag(), "Pyro / Bloom");

        let top = LocationRef {
            guid: Guid::default(),
            record_name: "Stanton".to_string(),
            display_name: "Stanton".to_string(),
            system: SystemKey::Stanton,
            body: None,
            body_display_name: String::new(),
        };
        assert_eq!(top.short_tag(), "Stanton");
        assert_eq!(top.human_tag(), "Stanton");
    }

    fn mk_loc(body: Option<(&str, &str)>, system: SystemKey) -> LocationRef {
        LocationRef {
            guid: Guid::default(),
            record_name: "x".to_string(),
            display_name: String::new(),
            system,
            body: body.map(|(rec, _)| rec.to_string()),
            body_display_name: body.map(|(_, disp)| disp.to_string()).unwrap_or_default(),
        }
    }

    #[test]
    fn region_label_single_system_with_bodies_lists_them() {
        let locs = vec![
            mk_loc(Some(("Pyro1", "Sirus")), SystemKey::Pyro),
            mk_loc(Some(("Pyro2", "Monox")), SystemKey::Pyro),
            mk_loc(Some(("Pyro3", "Bloom")), SystemKey::Pyro),
            // Duplicate: same body, different child — must dedupe.
            mk_loc(Some(("Pyro3", "Bloom")), SystemKey::Pyro),
        ];
        let mut systems = BTreeSet::new();
        systems.insert(SystemKey::Pyro);
        assert_eq!(build_region_label(&locs, &systems), "Pyro: Bloom, Monox, Sirus");
    }

    #[test]
    fn region_label_cross_system_suppresses_bodies() {
        let locs = vec![
            mk_loc(Some(("Stanton1", "Hurston")), SystemKey::Stanton),
            mk_loc(Some(("Pyro3", "Bloom")), SystemKey::Pyro),
        ];
        let mut systems = BTreeSet::new();
        systems.insert(SystemKey::Stanton);
        systems.insert(SystemKey::Pyro);
        assert_eq!(build_region_label(&locs, &systems), "Stanton + Pyro");
    }

    #[test]
    fn region_label_caps_long_body_list() {
        let bodies = [
            ("A", "Alpha"),
            ("B", "Beta"),
            ("C", "Ceti"),
            ("D", "Delta"),
            ("E", "Echo"),
            ("F", "Foxtrot"),
            ("G", "Gamma"),
        ];
        let locs: Vec<LocationRef> = bodies
            .iter()
            .map(|(rec, disp)| mk_loc(Some((rec, disp)), SystemKey::Pyro))
            .collect();
        let mut systems = BTreeSet::new();
        systems.insert(SystemKey::Pyro);
        let label = build_region_label(&locs, &systems);
        // Sorted alphabetically: Alpha, Beta, Ceti, Delta, Echo shown;
        // Foxtrot + Gamma hidden behind the "+2 more" marker.
        assert_eq!(label, "Pyro: Alpha, Beta, Ceti, Delta, Echo, +2 more");
    }

    #[test]
    fn region_label_system_only_marks_system_wide() {
        let locs = vec![
            mk_loc(None, SystemKey::Pyro),
            mk_loc(None, SystemKey::Pyro),
        ];
        let mut systems = BTreeSet::new();
        systems.insert(SystemKey::Pyro);
        assert_eq!(build_region_label(&locs, &systems), "Pyro (system-wide)");
    }
}
