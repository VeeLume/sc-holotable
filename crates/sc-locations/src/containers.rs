//! `StarMapObject` ↔ object-container binding.
//!
//! A [`crate::Location`] is the navigable *place*; the 3D world it loads is an
//! **object container** (`.socpak`). The link between them is **not** in the
//! DCB — it is authored in each system's top-level object container, where every
//! body/field/moon/lagrange/jump-point/asteroid-base is placed by an
//! `OrbitingObjectContainer` entity carrying both halves on one entity:
//!
//! ```xml
//! <Entity EntityClass="OrbitingObjectContainer" …>
//!   <EntityComponentObjectContainer objectContainer="…/stanton/stanton4b.socpak"/>
//!   <EntityComponentObjectMetadata>
//!     <SNavPointObjectMetadataParams starmapRecord="2a21d86f-…"/>
//! ```
//!
//! [`LocationContainers`] harvests those `(starmapRecord, objectContainer)` pairs
//! from the system socpaks. It needs a live [`AssetSource`] (the binding lives in
//! the socpaks, not the DCB), so it is cooked once and persisted on its own
//! (`ProcessedSnapshot<LocationContainers>`) rather than folded into [`Locations`],
//! which stays DCB-pure. See `docs/sc-locations.md` → "Object-container binding".
//!
//! **Scope (v1):** system-OC only. Sub-locations placed *inside* a body OC
//! (e.g. outposts) and the bridge entity's orbital data (`Pos` / `OrbitalRadius`
//! / `parentGUID`) are deferred — see the design spec.

use std::collections::HashMap;

use sc_extract::object_container::{Socpak, decode};
use sc_extract::{AssetSource, Guid, Result, XmlNode};
use serde::{Deserialize, Serialize};
use tracing::warn;

/// StarMapObject ↔ object-container binding, harvested from the system OCs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationContainers {
    /// `StarMapObject` GUID → the socpak that realizes it (1:1).
    by_location: HashMap<Guid, String>,
    /// socpak path → `StarMapObject`s sharing it (1:many — reused asteroid bases).
    by_container: HashMap<String, Vec<Guid>>,
}

impl LocationContainers {
    /// Bump when the cooked layout changes (for `ProcessedSnapshot` gating).
    pub const COOK_SCHEMA_VERSION: u32 = 1;

    /// Cook from a **live** p4k: parse every `*system.socpak` for
    /// `OrbitingObjectContainer` `(starmapRecord, objectContainer)` pairs.
    ///
    /// Returns an empty index for a snapshot-backed [`AssetSource`] (entry
    /// enumeration is live-only) — the binding is a p4k-time cook by design.
    pub fn cook(assets: &AssetSource) -> Result<Self> {
        let mut idx = Self::default();

        // System object containers — stanton/pyro/nyx today; no universe root.
        let socpaks: Vec<String> = assets
            .find(|name| name.to_ascii_lowercase().ends_with("system.socpak"))
            .map(|e| e.name.to_string())
            .collect();
        if socpaks.is_empty() {
            warn!("no *system.socpak entries found (snapshot-backed source?)");
        }

        for socpak in socpaks {
            let bytes = assets.read(&socpak)?;
            let mut pak = match Socpak::open(bytes) {
                Ok(pak) => pak,
                Err(e) => {
                    warn!(socpak = %socpak, error = %e, "skip unreadable system socpak");
                    continue;
                }
            };
            // Placement entities live in the socpak's `entdata/*.entxml` members
            // (CryXmlB); the `.soc` scene graph references them by id.
            for i in 0..pak.len() {
                let Some(member) = pak.name(i) else { continue };
                if !member.to_ascii_lowercase().ends_with(".entxml") {
                    continue;
                }
                let bytes = match pak.read(i) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        warn!(member = %member, error = %e, "skip unreadable socpak member");
                        continue;
                    }
                };
                match decode(&bytes) {
                    Ok(Some(root)) => idx.harvest(&root),
                    Ok(None) => {}
                    Err(e) => warn!(member = %member, error = %e, "skip undecodable member"),
                }
            }
        }
        Ok(idx)
    }

    /// Pull every `OrbitingObjectContainer` `(starmapRecord, objectContainer)`
    /// pair out of a decoded entity tree.
    fn harvest(&mut self, root: &XmlNode) {
        for entity in root
            .find_all("Entity")
            .filter(|e| e.attr("EntityClass") == Some("OrbitingObjectContainer"))
        {
            let Some(container) = entity
                .find_all("EntityComponentObjectContainer")
                .next()
                .and_then(|c| c.attr("objectContainer"))
            else {
                continue;
            };
            let Some(starmap) = entity
                .find_all("SNavPointObjectMetadataParams")
                .next()
                .and_then(|c| c.attr("starmapRecord"))
            else {
                continue;
            };
            // Placements without a nav entry (skybox, scattered derelicts) carry
            // an empty or null GUID — not locations.
            if starmap.is_empty() || starmap.starts_with("00000000-") {
                continue;
            }
            let Ok(guid) = starmap.parse::<Guid>() else {
                warn!(starmap, "unparseable starmapRecord GUID");
                continue;
            };
            let path = normalize_oc_path(container);
            self.by_location.insert(guid, path.clone());
            let bucket = self.by_container.entry(path).or_default();
            if !bucket.contains(&guid) {
                bucket.push(guid);
            }
        }
    }

    /// The socpak that realizes a location (`StarMapObject` GUID).
    pub fn container_of(&self, location: &Guid) -> Option<&str> {
        self.by_location.get(location).map(String::as_str)
    }

    /// Every location sharing a socpak (1:many for reused asteroid-base templates).
    pub fn locations_in(&self, container: &str) -> &[Guid] {
        self.by_container
            .get(container)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// All `(StarMapObject GUID, socpak)` bindings.
    pub fn iter(&self) -> impl Iterator<Item = (&Guid, &str)> + '_ {
        self.by_location.iter().map(|(g, s)| (g, s.as_str()))
    }

    /// Distinct socpak paths that back at least one location.
    pub fn containers(&self) -> impl Iterator<Item = &str> + '_ {
        self.by_container.keys().map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.by_location.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_location.is_empty()
    }
}

/// Normalize a socpak path for stable keying: backslash→`/`, lowercase, and drop
/// a leading `data/` prefix. Live data mixes `objectcontainers/…`,
/// `ObjectContainers/PU/…`, and `Data\ObjectContainers\…`.
fn normalize_oc_path(raw: &str) -> String {
    let s = raw.replace('\\', "/").to_ascii_lowercase();
    s.strip_prefix("data/").unwrap_or(&s).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entity(class: &str, container: &str, starmap: &str) -> Vec<u8> {
        format!(
            r#"<Entity EntityClass="{class}">
                 <PropertiesDataCore>
                   <EntityComponentObjectContainer objectContainer="{container}"/>
                   <EntityComponentObjectMetadata>
                     <SNavPointObjectMetadataParams starmapRecord="{starmap}"/>
                   </EntityComponentObjectMetadata>
                 </PropertiesDataCore>
               </Entity>"#
        )
        .into_bytes()
    }

    fn harvest_one(bytes: &[u8]) -> LocationContainers {
        let root = decode(bytes).unwrap().unwrap();
        let mut lc = LocationContainers::default();
        lc.harvest(&root);
        lc
    }

    #[test]
    fn binds_and_normalizes_path() {
        let g = "2a21d86f-ebf0-4052-a134-c414c9998592";
        let lc = harvest_one(&entity(
            "OrbitingObjectContainer",
            r"Data\ObjectContainers\PU\system\stanton\stanton4b.socpak",
            g,
        ));
        assert_eq!(
            lc.container_of(&g.parse().unwrap()),
            Some("objectcontainers/pu/system/stanton/stanton4b.socpak")
        );
    }

    #[test]
    fn one_socpak_many_locations() {
        let shared =
            "ObjectContainers/PU/system/stanton/asteroidbase/ab_mine_stanton_cloud_med_001.socpak";
        let mut lc = harvest_one(&entity(
            "OrbitingObjectContainer",
            shared,
            "02069d4a-e37a-474c-9229-691ce013bdc1",
        ));
        lc.harvest(
            &decode(&entity(
                "OrbitingObjectContainer",
                shared,
                "032fa15d-33a1-42a9-afb9-744811d1335d",
            ))
            .unwrap()
            .unwrap(),
        );
        let key =
            "objectcontainers/pu/system/stanton/asteroidbase/ab_mine_stanton_cloud_med_001.socpak";
        assert_eq!(
            lc.locations_in(key).len(),
            2,
            "1:many asteroid-base binding"
        );
        assert_eq!(lc.len(), 2);
    }

    #[test]
    fn skips_null_and_non_orbiting() {
        // Null starmapRecord (scattered derelict / skybox) → not a location.
        let lc = harvest_one(&entity(
            "OrbitingObjectContainer",
            "x.socpak",
            "00000000-0000-0000-0000-000000000000",
        ));
        assert!(lc.is_empty());
        // Non-OrbitingObjectContainer entity → ignored.
        let lc = harvest_one(&entity(
            "ProceduralEntity",
            "y.socpak",
            "2a21d86f-ebf0-4052-a134-c414c9998592",
        ));
        assert!(lc.is_empty());
    }
}
