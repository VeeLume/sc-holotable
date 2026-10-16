//! Location join — `StarMapObject` ↔ provider.
//!
//! Composed from two halves:
//! - `sc_locations::ObjectContainers` — the placement graph: which placement
//!   includes which socpak, and which placements name a `StarMapObject`.
//! - each socpak's `HarvestableProviderComponent.preset`s — socpak ↔ providers.
//!
//! A provider applies to the place its socpak belongs to: from every placement
//! that includes the socpak, up through `parentGUID` and the placements
//! including each socpak on the way, to the nearest placement that names a
//! `StarMapObject`. Body socpaks are named directly; fields nested in unnamed
//! segments (the Nyx belts' gas clouds, Pyro's Lagrange child clouds) get the
//! place above them. One socpak can carry several providers (Yela and its
//! ring), and one place several providers (a mining claim realized by two
//! asteroid templates).
//!
//! [`ProviderLocations::cook`] needs a **live** p4k. The join
//! ([`ProviderLocations::compose`]) and the per-member extraction are
//! unit-tested; the full cook is a p4k-time step.

use std::collections::{HashMap, HashSet};

use sc_extract::object_container::XmlNode;
use sc_extract::{AssetSource, Guid, RecordStore, Result};
use sc_locations::{ObjectContainers, PlacementId, normalize_socpak_path};
use serde::{Deserialize, Serialize};

/// `StarMapObject` ↔ `HarvestableProviderPreset` GUID, both directions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderLocations {
    /// `StarMapObject` GUID → the providers that apply there.
    by_location: HashMap<Guid, Vec<Guid>>,
    /// provider GUID → the places it applies to (1:many — reused socpaks).
    by_provider: HashMap<Guid, Vec<Guid>>,
}

impl ProviderLocations {
    /// Bump when the cooked layout changes (for `ProcessedSnapshot` gating).
    pub const COOK_SCHEMA_VERSION: u32 = 2;

    /// Cook from a live p4k: walk the object containers of every solar system
    /// the `MegaMap` records list (only what they include, read in parallel)
    /// and read each socpak's provider presets in the same pass. Empty for a
    /// snapshot-backed source (socpaks aren't there).
    pub fn cook(assets: &AssetSource, store: &RecordStore) -> Result<Self> {
        Self::cook_from(assets, &solar_system_roots(store))
    }

    /// [`cook`](Self::cook) from other root socpaks (any path rendering): one
    /// system, or every object container to check the walk misses nothing.
    pub fn cook_from(assets: &AssetSource, roots: &[String]) -> Result<Self> {
        let (containers, providers) = ObjectContainers::walk(assets, roots, provider_presets)?;
        Ok(Self::compose(&containers, &providers))
    }

    /// Pure join: providers per socpak (normalized path) ⊕ the placement graph
    /// ⇒ place ↔ provider.
    pub fn compose(
        containers: &ObjectContainers,
        socpak_providers: &HashMap<String, Vec<Guid>>,
    ) -> Self {
        let mut pl = Self::default();
        for (socpak, providers) in socpak_providers {
            for place in named_places(containers, socpak) {
                for &provider in providers {
                    let at = pl.by_location.entry(place).or_default();
                    if !at.contains(&provider) {
                        at.push(provider);
                    }
                    let of = pl.by_provider.entry(provider).or_default();
                    if !of.contains(&place) {
                        of.push(place);
                    }
                }
            }
        }
        // A stable order whatever order the graph was harvested in.
        for list in pl
            .by_location
            .values_mut()
            .chain(pl.by_provider.values_mut())
        {
            list.sort_by_key(Guid::to_string);
        }
        pl
    }

    /// The providers that apply at a place (`StarMapObject` GUID).
    pub fn providers_at(&self, location: &Guid) -> &[Guid] {
        self.by_location
            .get(location)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Every place a provider applies to (1:many for reused asteroid bases).
    pub fn locations_of(&self, provider: &Guid) -> &[Guid] {
        self.by_provider
            .get(provider)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Number of places with at least one provider.
    pub fn len(&self) -> usize {
        self.by_location.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_location.is_empty()
    }
}

/// The object containers of every solar system the `MegaMap` records list
/// (entries with a solar-system record: Stanton, Pyro, Nyx). Test, template and
/// Arena Commander maps list theirs without one.
fn solar_system_roots(store: &RecordStore) -> Vec<String> {
    let pools = &store.pools;
    let mut roots = Vec::new();
    for map in store.records.megamap.mega_map.values() {
        let Some(map) = map.get(pools) else { continue };
        for system in &map.solar_systems {
            let Some(system) = system.get(pools) else {
                continue;
            };
            if system.record.is_none() {
                continue;
            }
            for socpak in &system.object_containers {
                let socpak = normalize_socpak_path(socpak);
                if !roots.contains(&socpak) {
                    roots.push(socpak);
                }
            }
        }
    }
    roots.sort();
    roots
}

/// The `StarMapObject`s a socpak's content belongs to: from each placement
/// including it, up through `parentGUID` (else every placement including the
/// socpak it sits in) to the nearest placement that names one.
fn named_places(containers: &ObjectContainers, socpak: &str) -> Vec<Guid> {
    let mut places = Vec::new();
    let mut seen: HashSet<PlacementId> = HashSet::new();
    let mut stack: Vec<PlacementId> = containers.nesting(socpak).to_vec();
    while let Some(id) = stack.pop() {
        if !seen.insert(id) {
            continue;
        }
        let placement = containers.get(id);
        if let Some(place) = placement.starmap_guid() {
            if !places.contains(&place) {
                places.push(place);
            }
            continue;
        }
        match containers.parent_entity(id) {
            Some(parent) => stack.push(parent),
            None => stack.extend_from_slice(containers.nesting(placement.socpak())),
        }
    }
    places
}

/// Every non-null `HarvestableProviderComponent.preset` in a socpak member.
/// Providers sit on the body's root entity (`pivot.entxml`) or the field's
/// `.soc`/`.pla`; the deep `entdata/*` child entities are skipped.
fn provider_presets(_socpak: &str, member: &str, root: &XmlNode) -> Vec<Guid> {
    let member = member.to_ascii_lowercase();
    if !(member.ends_with("pivot.entxml") || member.ends_with(".soc") || member.ends_with(".pla")) {
        return Vec::new();
    }
    root.find_all("HarvestableProviderComponent")
        .filter_map(|c| c.attr("preset"))
        .filter(|p| !p.is_empty() && !p.starts_with("00000000-"))
        .filter_map(|p| p.parse::<Guid>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_extract::object_container::{Socpak, decode};
    use std::io::Write;

    /// A socpak with the given members, each carrying one provider component.
    fn socpak_with(members: &[(&str, &str)]) -> Vec<u8> {
        let mut zw = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (name, preset) in members {
            zw.start_file(*name, opts).unwrap();
            let xml = format!(
                r#"<Entity EntityClass="ProceduralEntity"><PropertiesDataCore>
                     <HarvestableProviderComponent __type="HarvestableProviderParams" preset="{preset}"/>
                   </PropertiesDataCore></Entity>"#
            );
            zw.write_all(xml.as_bytes()).unwrap();
        }
        zw.finish().unwrap().into_inner()
    }

    /// What the cook's visitor reads from a whole socpak.
    fn presets_in(bytes: Vec<u8>) -> Vec<Guid> {
        let mut pak = Socpak::open(bytes).unwrap();
        let mut out = Vec::new();
        for i in 0..pak.len() {
            let name = pak.name(i).unwrap();
            if let Ok(Some(root)) = decode(&pak.read(i).unwrap()) {
                out.extend(provider_presets("test.socpak", &name, &root));
            }
        }
        out
    }

    const CLIO: &str = "703a18ca-7f7c-4489-a64a-cd0cd359b8fe";
    const YELA: &str = "a2ed2e64-1a85-4f5c-8a0e-0b8f3f4f7f01";
    const RING: &str = "b3fe3f75-2b96-4061-9b1f-1c904050a012";

    #[test]
    fn extracts_provider_from_socpak() {
        assert_eq!(
            presets_in(socpak_with(&[("body/pivot.entxml", CLIO)])),
            [CLIO.parse::<Guid>().unwrap()]
        );
    }

    #[test]
    fn null_preset_yields_none() {
        let null = "00000000-0000-0000-0000-000000000000";
        assert!(presets_in(socpak_with(&[("body/pivot.entxml", null)])).is_empty());
    }

    /// A body and its ring in one socpak: both providers, not the first.
    #[test]
    fn extracts_every_provider() {
        let found = presets_in(socpak_with(&[
            ("stanton2c/pivot.entxml", YELA),
            ("stanton2c.soc", RING),
        ]));
        assert_eq!(
            found,
            [YELA.parse::<Guid>().unwrap(), RING.parse::<Guid>().unwrap()]
        );
    }

    /// Real-data check: validate against the probe's Clio socpak when present
    /// (skipped in CI, where the gitignored probe corpus is absent).
    #[test]
    fn extracts_real_clio_provider() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/probe-resources/stanton4b_clio.zip");
        let Ok(bytes) = std::fs::read(&path) else {
            eprintln!("skip: no probe socpak at {}", path.display());
            return;
        };
        assert!(presets_in(bytes).contains(&CLIO.parse().unwrap()));
    }

    /// Deep child entities don't count.
    #[test]
    fn skips_entdata_members() {
        assert!(presets_in(socpak_with(&[("body/entdata/rock.entxml", CLIO)])).is_empty());
    }

    /// A placement graph as the cook would harvest it, built through serde.
    fn graph(placements: serde_json::Value) -> ObjectContainers {
        serde_json::from_value(serde_json::json!({ "placements": placements })).unwrap()
    }

    fn placement(
        key: u8,
        socpak: &str,
        nests: &str,
        starmap: Option<&str>,
        parent: Option<u8>,
    ) -> serde_json::Value {
        let k = |b: u8| {
            let mut a = [0u8; 16];
            a[0] = b;
            a
        };
        serde_json::json!({
            "key": k(key),
            "disp": format!("{key:08x}-0000-0000-0000-000000000000"),
            "socpak": socpak,
            "parent": parent.map(k),
            "nests": nests,
            "starmap": starmap,
            "starmap_guid": starmap,
            "name": "",
            "pos": null,
            "orbit": null,
            "is_leaf": starmap.is_some(),
        })
    }

    const BELT: &str = "11111111-1111-1111-1111-111111111111";
    const CLAIM_A: &str = "22222222-2222-2222-2222-222222222222";
    const CLAIM_B: &str = "33333333-3333-3333-3333-333333333333";
    const P_BODY: &str = "44444444-4444-4444-4444-444444444444";
    const P_CLOUD: &str = "55555555-5555-5555-5555-555555555555";

    /// A named body socpak maps directly; a cloud nested in an unnamed segment
    /// gets the place above it; a template nested by two claims maps to both.
    #[test]
    fn compose_attributes_to_the_nearest_named_placement() {
        let g = graph(serde_json::json!([
            placement(1, "sys.socpak", "body.socpak", Some(BELT), None),
            placement(2, "sys.socpak", "segment.socpak", None, Some(1)),
            placement(3, "segment.socpak", "cloud.socpak", None, None),
            placement(4, "body.socpak", "asteroid.socpak", Some(CLAIM_A), None),
            placement(5, "body.socpak", "asteroid.socpak", Some(CLAIM_B), None),
        ]));
        let g_ = |s: &str| s.parse::<Guid>().unwrap();
        let providers = HashMap::from([
            ("body.socpak".to_string(), vec![g_(P_BODY)]),
            ("cloud.socpak".to_string(), vec![g_(P_CLOUD)]),
            ("asteroid.socpak".to_string(), vec![g_(CLIO)]),
        ]);
        let pl = ProviderLocations::compose(&g, &providers);
        // GUIDs here are chosen so their string order is the listed order.
        assert_eq!(pl.providers_at(&g_(BELT)), [g_(P_BODY), g_(P_CLOUD)]);
        assert_eq!(pl.locations_of(&g_(CLIO)), [g_(CLAIM_A), g_(CLAIM_B)]);
        assert_eq!(pl.len(), 3);
    }
}
