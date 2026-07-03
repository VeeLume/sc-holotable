//! The object-container **placement graph** — one walk over the socpaks, from
//! which the location-domain socpak products are projected.
//!
//! The object containers (`.socpak`) place every body / station / outpost /
//! mission-location as an `<Entity>` in a containment graph. Three things the
//! rest of the crate needs are all **projections of this one graph**:
//!
//! - mission DataSet `crc32(EntityCryGUID chain)` → station name
//!   ([`crate::Universe::by_mission_crc`]),
//! - `StarMapObject` ↔ realizing socpak ([`Self::realized_socpaks`] /
//!   [`Self::locations_in`] / [`Self::placements_of`]),
//! - `StarMapObject` → 3D position for route planning ([`crate::Place::position`] /
//!   [`crate::Place::global_position`]).
//!
//! Previously each was a separate scan of the same files. This module is the
//! single tolerant walk; see `docs/object-containers.md` for the design.
//!
//! ## The tolerant-harvest contract (correctness — do not regress)
//!
//! **The placement is primary; identity is layered.** We index every placement
//! entity (`EntityCryGUID` + `Pos`); `starmapRecord`, a mission template, and the
//! entity `Name` are *optional* facets. Real objects exist with none of them
//! beyond a position (unnamed outposts, mission-objective points, easter eggs) —
//! so we **never gate on `starmapRecord`**, accept **any** `EntityClass`, and
//! search `starmapRecord` on **any** descendant node (`SObjectMetadataParams`,
//! `<Elem>`, `<Child>`), never just the first. Live 4.8 renamed the pre-4.8
//! `SNavPointObjectMetadataParams` → `SObjectMetadataParams`; the tag-agnostic
//! search means we don't care.

use std::collections::{HashMap, HashSet};

use sc_extract::object_container::{Socpak, XmlNode, decode};
use sc_extract::{AssetSource, Guid, Result};
use serde::{Deserialize, Serialize};
use tracing::warn;

// ── GUID canonicalization + CRC (shared by the projections) ─────────────────

/// Canonical 16-byte GUID storage key (rendering-independent).
pub(crate) type Key = [u8; 16];
/// CryGUID display byte order: `[b7 b6 b5 b4]-[b3 b2]-[b1 b0]-[b15 b14]-[b13..b8]`.
const MAP: [usize; 16] = [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8];

fn hex16(s: &str) -> Option<[u8; 16]> {
    let h: Vec<u8> = s.bytes().filter(u8::is_ascii_hexdigit).collect();
    if h.len() != 32 {
        return None;
    }
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[i] = ((h[i * 2] as char).to_digit(16)? * 16 + (h[i * 2 + 1] as char).to_digit(16)?) as u8;
    }
    Some(b)
}
/// `EntityCryGUID` (CryGUID display order) → storage key.
pub(crate) fn cry_key(s: &str) -> Option<Key> {
    let db = hex16(s)?;
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[MAP[i]] = db[i];
    }
    Some(b)
}
/// `parentGUID` (standard .NET order) → storage key.
pub(crate) fn std_key(s: &str) -> Option<Key> {
    let d = hex16(s)?;
    Some([
        d[3], d[2], d[1], d[0], d[5], d[4], d[7], d[6], d[8], d[9], d[10], d[11], d[12], d[13],
        d[14], d[15],
    ])
}
/// Standard CRC-32 (IEEE / zlib): init+xorout `0xFFFFFFFF`, reflected poly.
pub(crate) fn crc32_ieee(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = (c >> 1) ^ (0xEDB8_8320 & (c & 1).wrapping_neg());
        }
    }
    !c
}
/// First non-empty value of `attr` on `e` or any descendant.
pub(crate) fn first_attr<'a>(e: &'a XmlNode, attr: &str) -> Option<&'a str> {
    e.descendants()
        .find_map(|n| n.attr(attr))
        .filter(|s| !s.is_empty())
}
/// Normalize a socpak / objectContainer path: `\`→`/`, lowercase, strip `data/`.
///
/// This is the key form [`ObjectContainers`] uses everywhere a socpak path is a
/// map key (`realized_socpaks` / `locations_in` / `Placement::socpak`). Live
/// data mixes `objectcontainers/…`, `ObjectContainers/PU/…`, and
/// `Data\ObjectContainers\…` renderings of the same path — consumers joining on
/// socpak paths must normalize through this same function.
pub fn normalize_socpak_path(raw: &str) -> String {
    let s = raw.replace('\\', "/").to_ascii_lowercase();
    s.strip_prefix("data/").unwrap_or(&s).to_string()
}
/// Parse a `Pos`-style attribute (`"x,y,z"`, comma/space separated).
fn vec3(raw: &str) -> Option<[f64; 3]> {
    let p: Vec<f64> = raw
        .split([',', ' '])
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    (p.len() >= 3).then(|| [p[0], p[1], p[2]])
}

// ── The graph ───────────────────────────────────────────────────────────────

/// Orbital parameters carried by a placement's `EntityComponentOrbit`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Orbit {
    pub radius: f64,
    pub angle: f64,
}

/// Opaque handle to a [`Placement`] within an [`ObjectContainers`] graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlacementId(pub(crate) usize);

/// One placed object-container entity — the primary record. Identity facets
/// (`starmap`, `mission_leaf`, `name`) are optional; a placement is valid with a
/// position alone. Fields are private; read via the accessors below.
#[derive(Serialize, Deserialize)]
pub struct Placement {
    /// `EntityCryGUID` storage key (graph identity).
    pub(crate) key: Key,
    /// `EntityCryGUID` as the lowercase dashed string — the CRC chain uses this.
    pub(crate) disp: String,
    /// Placement socpak (normalized).
    pub(crate) socpak: String,
    /// `parentGUID` storage key (the entity-graph orbital parent), if any.
    pub(crate) parent: Option<Key>,
    /// `objectContainer` this entity realizes (normalized socpak), if any.
    pub(crate) nests: Option<String>,
    /// `starmapRecord` (raw string, brace/`.NET` form kept), if any.
    pub(crate) starmap: Option<String>,
    /// `starmapRecord` parsed to a GUID (index/accessor convenience), if valid.
    pub(crate) starmap_guid: Option<Guid>,
    /// Entity `Name` attribute (raw identity fallback).
    pub(crate) name: String,
    /// Entity `Pos` — system-global for system-OC placements, body-local for
    /// sub-body placements.
    pub(crate) pos: Option<[f64; 3]>,
    pub(crate) orbit: Option<Orbit>,
    /// A chain **leaf**: has a `starmapRecord` or a mission template.
    pub(crate) is_leaf: bool,
}

impl Placement {
    /// The `StarMapObject` this placement realizes, if it names one.
    pub fn starmap_guid(&self) -> Option<Guid> {
        self.starmap_guid
    }
    /// Entity `Pos` — **direct**, as authored (system-global for system-OC
    /// placements, body-local for sub-body ones). No parent substitution.
    pub fn position(&self) -> Option<[f64; 3]> {
        self.pos
    }
    pub fn orbit(&self) -> Option<Orbit> {
        self.orbit
    }
    /// The placement socpak (normalized).
    pub fn socpak(&self) -> &str {
        &self.socpak
    }
    /// Raw socpak entity `Name` — the identity for a place with no star-map name.
    pub fn entity_name(&self) -> &str {
        &self.name
    }
    /// True when this placement has a `starmapRecord` or a mission template — a
    /// containment-chain leaf (has a mission DataSet CRC).
    pub fn is_mission_leaf(&self) -> bool {
        self.is_leaf
    }
}

/// The cooked placement graph. The [`crate::Universe`] and the place↔socpak /
/// position indices are projected from it.
/// Only `placements` is serialized; the indices are rebuilt on load (via a
/// [`ObjectContainersRepr`] shadow), so a persisted graph re-`join`s into a
/// [`crate::Universe`] cheaply without re-walking the socpaks.
#[derive(Serialize, Deserialize)]
#[serde(from = "ObjectContainersRepr")]
pub struct ObjectContainers {
    pub(crate) placements: Vec<Placement>,
    /// `(socpak, cry) → placement` — anchors the walk per-placement (CryGUIDs are
    /// reused across cloned stations, so bare-cry lookup is ambiguous).
    #[serde(skip)]
    pub(crate) by_sp_key: HashMap<(String, Key), usize>,
    /// `cry → placements` — cross-socpak fallback for parent resolution.
    #[serde(skip)]
    pub(crate) by_key: HashMap<Key, Vec<usize>>,
    /// `nested socpak → the OOC placement that includes it` (socpak-nesting edge).
    #[serde(skip)]
    pub(crate) nest_owner: HashMap<String, usize>,
    /// `StarMapObject GUID → placements realizing it` (1:many). Built post-harvest.
    #[serde(skip)]
    by_starmap: HashMap<Guid, Vec<PlacementId>>,
    /// `mission DataSet CRC → leaf placement`. Built post-harvest.
    #[serde(skip)]
    by_mission_crc: HashMap<u32, PlacementId>,
    /// `realized socpak (objectContainer) → StarMapObjects it backs` (1:many —
    /// reused asteroid-base templates). Built post-harvest. The `place ↔ socpak`
    /// bridge (formerly `LocationContainers`).
    #[serde(skip)]
    by_realized_socpak: HashMap<String, Vec<Guid>>,
}

/// Serialization shadow: only `placements` is persisted; indices are recomputed
/// on the way back in via [`ObjectContainers::rebuild_indices`].
#[derive(Serialize, Deserialize)]
struct ObjectContainersRepr {
    placements: Vec<Placement>,
}

impl From<ObjectContainersRepr> for ObjectContainers {
    fn from(repr: ObjectContainersRepr) -> Self {
        let mut g = ObjectContainers {
            placements: repr.placements,
            by_sp_key: HashMap::new(),
            by_key: HashMap::new(),
            nest_owner: HashMap::new(),
            by_starmap: HashMap::new(),
            by_mission_crc: HashMap::new(),
            by_realized_socpak: HashMap::new(),
        };
        g.rebuild_indices();
        g
    }
}

impl ObjectContainers {
    /// Cook from a **live** p4k: scan every `objectcontainers/*.socpak`, index
    /// every placement entity. Returns an empty graph for a snapshot-backed
    /// [`AssetSource`] (entry enumeration is live-only).
    pub fn cook(assets: &AssetSource) -> Result<Self> {
        let socpaks: Vec<String> = assets
            .find(|n| {
                let l = n.to_ascii_lowercase().replace('\\', "/");
                l.ends_with(".socpak") && l.contains("objectcontainers")
            })
            .map(|e| e.name.to_string())
            .collect();
        if socpaks.is_empty() {
            warn!("no objectcontainers/*.socpak entries (snapshot-backed source?)");
            return Ok(Self::empty());
        }

        let mut g = Self::empty();
        for sp in &socpaks {
            let sp_norm = normalize_socpak_path(sp);
            // System nav-graph placements live in `.xml` members; decoding every
            // socpak's `.xml` would explode parse time, so gate on `/system/`.
            let allow_xml = sp
                .to_ascii_lowercase()
                .replace('\\', "/")
                .contains("/system/");
            let bytes = match assets.read(sp) {
                Ok(b) => b,
                Err(e) => {
                    warn!(socpak = %sp, error = %e, "skip unreadable socpak");
                    continue;
                }
            };
            let mut pak = match Socpak::open(bytes) {
                Ok(p) => p,
                Err(e) => {
                    warn!(socpak = %sp, error = %e, "skip unopenable socpak");
                    continue;
                }
            };
            for m in 0..pak.len() {
                let member = pak.name(m).unwrap_or_default().to_ascii_lowercase();
                let ok = member.ends_with(".soc")
                    || member.ends_with(".pla")
                    || member.ends_with(".entxml")
                    || (allow_xml && member.ends_with(".xml"));
                if !ok {
                    continue;
                }
                let Ok(b) = pak.read(m) else { continue };
                let Ok(Some(root)) = decode(&b) else { continue };
                g.harvest(&root, &sp_norm);
            }
        }
        g.rebuild_indices();
        Ok(g)
    }

    fn empty() -> Self {
        Self {
            placements: Vec::new(),
            by_sp_key: HashMap::new(),
            by_key: HashMap::new(),
            nest_owner: HashMap::new(),
            by_starmap: HashMap::new(),
            by_mission_crc: HashMap::new(),
            by_realized_socpak: HashMap::new(),
        }
    }

    /// Build every lookup index from `self.placements`. Called after a fresh cook
    /// and after deserialization (only `placements` is persisted; the indices are
    /// derived). Order matters: the containment maps (`by_sp_key` / `by_key` /
    /// `nest_owner`) must exist before the mission-CRC pass, which walks chains.
    fn rebuild_indices(&mut self) {
        self.by_sp_key.clear();
        self.by_key.clear();
        self.nest_owner.clear();
        self.by_starmap.clear();
        self.by_mission_crc.clear();
        self.by_realized_socpak.clear();

        for (i, p) in self.placements.iter().enumerate() {
            // containment (first-placement-wins, matching the harvest order).
            if let Some(oc) = &p.nests {
                self.nest_owner.entry(oc.clone()).or_insert(i);
            }
            self.by_sp_key.entry((p.socpak.clone(), p.key)).or_insert(i);
            self.by_key.entry(p.key).or_default().push(i);
            // starmap → placements, and place ↔ realized socpak.
            if let Some(g) = p.starmap_guid {
                self.by_starmap.entry(g).or_default().push(PlacementId(i));
                if let Some(oc) = &p.nests {
                    let bucket = self.by_realized_socpak.entry(oc.clone()).or_default();
                    if !bucket.contains(&g) {
                        bucket.push(g);
                    }
                }
            }
        }
        // mission CRC → leaf; a *named* leaf (has starmapRecord) is the better
        // representative when two leaves collide on a CRC.
        let mut by_crc: HashMap<u32, usize> = HashMap::new();
        for i in 0..self.placements.len() {
            if !self.placements[i].is_leaf {
                continue;
            }
            let Some(crc) = self.chain_crc(&self.chain(i)) else {
                continue;
            };
            match by_crc.entry(crc) {
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(i);
                }
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    if self.placements[*o.get()].starmap.is_none()
                        && self.placements[i].starmap.is_some()
                    {
                        *o.get_mut() = i;
                    }
                }
            }
        }
        self.by_mission_crc = by_crc
            .into_iter()
            .map(|(c, i)| (c, PlacementId(i)))
            .collect();
    }

    /// Index every placement entity in one decoded member tree.
    fn harvest(&mut self, root: &XmlNode, sp_norm: &str) {
        for e in root.find_all("Entity") {
            let class = e.attr("EntityClass").unwrap_or("");
            let Some(disp) = e.attr("EntityCryGUID") else {
                continue;
            };
            let disp = disp.to_ascii_lowercase();
            let Some(key) = cry_key(&disp) else { continue };

            // Optional identity facets — tag-agnostic, never gated on.
            let starmap = first_attr(e, "starmapRecord")
                .filter(|s| !s.starts_with("00000000-"))
                .map(str::to_string);
            let starmap_guid = starmap
                .as_deref()
                .and_then(|s| s.trim().trim_matches(['{', '}']).parse::<Guid>().ok());
            let has_mission = first_attr(e, "template").is_some()
                || e.find_all("MissionLocationParams").next().is_some();

            // Selection: object-container placements + any nav/mission leaf. This
            // is deliberately broad but excludes pure geometry (no OC, no facet).
            if !(class.contains("ObjectContainer") || starmap.is_some() || has_mission) {
                continue;
            }

            let parent = first_attr(e, "parentGUID").and_then(std_key);
            let nests = e
                .find_all("EntityComponentObjectContainer")
                .next()
                .and_then(|c| c.attr("objectContainer"))
                .map(normalize_socpak_path);
            let pos = e.attr("Pos").and_then(vec3);
            let orbit = e.find_all("EntityComponentOrbit").next().and_then(|o| {
                let r = o.attr("OrbitalRadius").and_then(|v| v.trim().parse().ok());
                let a = o.attr("OrbitalAngle").and_then(|v| v.trim().parse().ok());
                match (r, a) {
                    (Some(radius), Some(angle)) => Some(Orbit { radius, angle }),
                    _ => None,
                }
            });

            self.placements.push(Placement {
                key,
                disp,
                socpak: sp_norm.to_string(),
                parent,
                nests,
                is_leaf: starmap.is_some() || has_mission,
                starmap,
                starmap_guid,
                name: e.attr("Name").unwrap_or("").to_string(),
                pos,
                orbit,
            });
        }
    }

    /// Walk up from placement `start` to the system OOC, returning the chain of
    /// **placement indices** (root → leaf). Parent = `parentGUID` (preferring the
    /// same socpak), else the OOC whose `objectContainer` nests this socpak.
    pub(crate) fn chain(&self, start: usize) -> Vec<usize> {
        let mut chain = vec![start];
        let mut cur = start;
        let mut seen: HashSet<Key> = HashSet::new();
        seen.insert(self.placements[start].key);
        for _ in 0..24 {
            let e = &self.placements[cur];
            if let Some(pk) = e.parent {
                if seen.contains(&pk) {
                    break;
                }
                let pe = self
                    .by_sp_key
                    .get(&(e.socpak.clone(), pk))
                    .copied()
                    .or_else(|| self.by_key.get(&pk).and_then(|v| v.first().copied()));
                match pe {
                    Some(pe) => {
                        seen.insert(pk);
                        chain.push(pe);
                        cur = pe;
                    }
                    None => break, // parent is the runtime root — not a static entity
                }
            } else if let Some(oe) = self.nest_owner.get(&e.socpak).copied() {
                if seen.contains(&self.placements[oe].key) {
                    break;
                }
                seen.insert(self.placements[oe].key);
                chain.push(oe);
                cur = oe;
            } else {
                break;
            }
        }
        chain.reverse();
        chain
    }

    /// `crc32_ieee(",".join(EntityCryGUID chain))` — the mission DataSet id for a
    /// chain (root → leaf). Chains shorter than 2 have no id.
    pub(crate) fn chain_crc(&self, chain: &[usize]) -> Option<u32> {
        if chain.len() < 2 {
            return None;
        }
        let joined = chain
            .iter()
            .map(|&j| self.placements[j].disp.as_str())
            .collect::<Vec<_>>()
            .join(",");
        Some(crc32_ieee(joined.as_bytes()))
    }

    // ── public navigation (physical containment graph) ───────────────────

    /// Number of indexed placements.
    pub fn len(&self) -> usize {
        self.placements.len()
    }
    pub fn is_empty(&self) -> bool {
        self.placements.is_empty()
    }
    /// The placement behind a handle.
    ///
    /// A `PlacementId` is only meaningful for the `ObjectContainers` that issued
    /// it (via `iter` / `placements_of` / `by_mission_crc` / `chain_of`). Passing
    /// an id from a *different* instance — e.g. a freshly cooked graph vs. a
    /// deserialized snapshot — indexes the wrong slot or panics.
    pub fn get(&self, id: PlacementId) -> &Placement {
        &self.placements[id.0]
    }
    /// Every placement (with its handle).
    pub fn iter(&self) -> impl Iterator<Item = (PlacementId, &Placement)> + '_ {
        self.placements
            .iter()
            .enumerate()
            .map(|(i, p)| (PlacementId(i), p))
    }
    /// Placements realizing a `StarMapObject` (1:many; empty if none).
    pub fn placements_of(&self, starmap: Guid) -> &[PlacementId] {
        self.by_starmap
            .get(&starmap)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
    /// The leaf placement for a mission DataSet CRC.
    pub fn by_mission_crc(&self, crc: u32) -> Option<PlacementId> {
        self.by_mission_crc.get(&crc).copied()
    }
    /// Distinct realized socpaks (`objectContainer`s) that back ≥1 named location
    /// — the `place ↔ socpak` bridge (formerly `LocationContainers::containers`).
    pub fn realized_socpaks(&self) -> impl Iterator<Item = &str> + '_ {
        self.by_realized_socpak.keys().map(String::as_str)
    }
    /// The `StarMapObject`s a realized socpak backs (1:many for reused templates;
    /// formerly `LocationContainers::locations_in`).
    pub fn locations_in(&self, socpak: &str) -> &[Guid] {
        self.by_realized_socpak
            .get(socpak)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
    /// The mission DataSet CRC of a placement (its containment-chain id), if it
    /// has one.
    pub fn mission_crc_of(&self, id: PlacementId) -> Option<u32> {
        self.chain_crc(&self.chain(id.0))
    }
    /// Containment chain (root → leaf) as handles.
    pub fn chain_of(&self, id: PlacementId) -> Vec<PlacementId> {
        self.chain(id.0).into_iter().map(PlacementId).collect()
    }
    /// The immediate containment parent of a placement, if any.
    pub fn parent_of(&self, id: PlacementId) -> Option<PlacementId> {
        let c = self.chain(id.0); // root..=leaf(id)
        (c.len() >= 2).then(|| PlacementId(c[c.len() - 2]))
    }

    /// **System-global** position — accumulate `Pos` from the leaf up the
    /// containment chain, stopping at the first `*system.socpak` placement.
    ///
    /// A placement in a `*system.socpak` (a body, moon, Lagrange point) already
    /// carries a system-global `Pos`; one in a body/loc OC carries a `Pos` local
    /// to its parent's frame. So we add local offsets while walking up, then add
    /// the global base and stop — summing past it would double-count (a moon is
    /// both a `*system.socpak` global *and* a logical child of its planet). Per-
    /// level **rotation is ignored** — the error is bounded by a body radius
    /// (hundreds of km) against inter-body distances (billions of m), exact
    /// enough for ordering stops by distance, *not* for precise navigation.
    /// `None` if no level along the way has a `Pos`.
    ///
    /// **Caveat:** if the containment chain never reaches a `*system.socpak`
    /// base (parent unresolvable, or the defensive hop cap fired), the returned
    /// sum is local to the deepest *resolved* frame, not system-global. A
    /// consumer mixing these into one distance ordering should be aware the
    /// frames can differ on broken chains.
    pub fn global_position(&self, id: PlacementId) -> Option<[f64; 3]> {
        let mut acc = [0.0f64; 3];
        let mut any = false;
        for j in self.chain(id.0).into_iter().rev() {
            if let Some(p) = self.placements[j].pos {
                acc[0] += p[0];
                acc[1] += p[1];
                acc[2] += p[2];
                any = true;
            }
            // A *system.socpak placement's Pos is the global base — stop here.
            if self.placements[j].socpak.ends_with("system.socpak") {
                break;
            }
        }
        any.then_some(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Storage-byte canonicalization must agree for the two renderings of the SAME
    // guid: EntityCryGUID (CryGUID order) vs parentGUID (.NET order).
    #[test]
    fn cry_and_std_canonicalize_equal() {
        let cry = cry_key("4a269932-183e-362d-aacc-188e7debbea9").unwrap();
        let std = std_key("183e362d-9932-4a26-a9be-eb7d8e18ccaa").unwrap();
        assert_eq!(cry, std);
    }

    // CRC-32/IEEE check vector.
    #[test]
    fn crc32_check_vector() {
        assert_eq!(crc32_ieee(b"123456789"), 0xCBF4_3926);
    }
}
