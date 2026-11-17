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
//!
//! # Sites and their markers
//!
//! [`ProviderLocations::sites_of`] lists every placement that carries a
//! provider (a [`Site`]): where it is and the place it resolves to. A ring
//! or belt field in an unnamed segment resolves to its system's **star**,
//! which no quantum travel targets. For such a site, [`Site::markers`] names
//! the places in the same segment: on LIVE 4.10 the Glaciem Ring's asteroid
//! clusters and mining bases, the Keeger Belt's breaker stations. Those are
//! where a run in the ring starts.
//!
//! A ring field that is one entity with its own shape (`AsteroidRing`, its
//! `SAsteroidRingComponentParams`) carries a [`Ring`], and its markers are
//! also the named places inside that ring. The Aaron Halo sits directly
//! under Stanton's star, outside any segment; its ring (19.60 to 21.39 Gm,
//! 10 Mm deep) holds the Wikelo Emporium Selo Station, the halo's one
//! quantum marker on LIVE 4.10, where players start a halo run.

use std::collections::HashMap;

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
    /// provider GUID → every placement that carries it.
    #[serde(default)]
    sites: HashMap<Guid, Vec<Site>>,
}

/// One placement that carries a provider: a field, a body, an asteroid base.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Site {
    /// The place it resolves to (the [`ProviderLocations::providers_at`] key).
    pub place: Guid,
    /// System-global position of the placement, m (the field's centre, or
    /// the body's).
    pub position: Option<[f64; 3]>,
    /// When `place` is the system's star (a ring or belt field in an
    /// unnamed segment): the named places in that segment, and for a field
    /// with a [`Ring`] the named places inside it, in GUID order. Intersect
    /// with `sc_locations::Waypoints` for the quantum targets a run starts
    /// from. A segment can hold none (one Glaciem segment holds only a QV
    /// Logistics Station), so a route to a provider takes the union over its
    /// sites. Empty otherwise.
    pub markers: Vec<Guid>,
    /// The field's own ring shape, when it is an `AsteroidRing` entity,
    /// centred on `position` (the system's origin when that is `None`).
    #[serde(default)]
    pub ring: Option<Ring>,
    /// The named places up its containment chain, nearest first: `place`,
    /// then what holds it, up to the star (a mining claim: the claim, `ARC
    /// L3-A`, `ARC L3`, Stanton). The first one in `sc_locations::Waypoints`
    /// is where quantum travel gets closest; the rest of the way to
    /// `position` is flown. On LIVE 4.10, 203 places with providers are no
    /// quantum target themselves, most of them private mining claims.
    #[serde(default)]
    pub anchors: Vec<Guid>,
}

/// An asteroid ring's shape (`SAsteroidRingComponentParams`), m: the band
/// between two radii around its centre, `depth` thick about its plane.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ring {
    /// `innerRadiusKm`, in m.
    pub inner_radius: f64,
    /// `outerRadiusKm`, in m.
    pub outer_radius: f64,
    /// `depthKm`, in m: the full thickness.
    pub depth: f64,
}

impl Ring {
    /// `offset` (from the ring's centre, in its frame) lies inside the band.
    pub fn contains(&self, offset: [f64; 3]) -> bool {
        let r = offset[0].hypot(offset[1]);
        r >= self.inner_radius && r <= self.outer_radius && offset[2].abs() <= self.depth / 2.0
    }
}

/// What the cook's walk finds in one socpak member.
enum Found {
    Provider(Guid),
    Ring(Ring),
}

impl ProviderLocations {
    /// Bump when the cooked layout changes (for `ProcessedSnapshot` gating).
    pub const COOK_SCHEMA_VERSION: u32 = 4;

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
        let (containers, found) = ObjectContainers::walk(assets, roots, |s, m, root| {
            let mut out: Vec<Found> = provider_presets(s, m, root)
                .into_iter()
                .map(Found::Provider)
                .collect();
            out.extend(ring_shape(m, root).map(Found::Ring));
            out
        })?;
        let mut providers: HashMap<String, Vec<Guid>> = HashMap::new();
        let mut rings: HashMap<String, Ring> = HashMap::new();
        for (socpak, list) in found {
            for f in list {
                match f {
                    Found::Provider(g) => providers.entry(socpak.clone()).or_default().push(g),
                    Found::Ring(r) => {
                        rings.entry(socpak.clone()).or_insert(r);
                    }
                }
            }
        }
        let mut pl = Self::compose(&containers, &providers);
        pl.sites = sites(&containers, &providers, &rings);
        Ok(pl)
    }

    /// Pure join: providers per socpak (normalized path) ⊕ the placement graph
    /// ⇒ place ↔ provider. Sites carry no [`Ring`]s ([`cook_from`](Self::cook_from)
    /// reads them).
    pub fn compose(
        containers: &ObjectContainers,
        socpak_providers: &HashMap<String, Vec<Guid>>,
    ) -> Self {
        let mut pl = Self::default();
        for (socpak, providers) in socpak_providers {
            for (place, _) in containers.places_of(socpak) {
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
        pl.sites = sites(containers, socpak_providers, &HashMap::new());
        pl
    }

    /// Every placement that carries `provider` ([`Site`]); empty for an
    /// unknown one.
    pub fn sites_of(&self, provider: &Guid) -> &[Site] {
        self.sites.get(provider).map(Vec::as_slice).unwrap_or(&[])
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

/// The [`Site`]s of every provider: each placement including a provider
/// socpak, the place it resolves to (the nearest named placement up its
/// containment chain), and, when that is the chain's root (the star), the
/// named places under the same top-level segment and inside the socpak's
/// [`Ring`].
fn sites(
    containers: &ObjectContainers,
    socpak_providers: &HashMap<String, Vec<Guid>>,
    rings: &HashMap<String, Ring>,
) -> HashMap<Guid, Vec<Site>> {
    // Named placements by their top-level segment (the chain's second link),
    // and by their root with their position.
    let mut by_segment: HashMap<PlacementId, Vec<Guid>> = HashMap::new();
    let mut by_root: HashMap<PlacementId, Vec<(Guid, [f64; 3])>> = HashMap::new();
    for (id, p) in containers.iter() {
        let Some(g) = p.starmap_guid() else { continue };
        let chain = containers.chain_of(id);
        if let Some(&segment) = chain.get(1) {
            let named = by_segment.entry(segment).or_default();
            if !named.contains(&g) {
                named.push(g);
            }
        }
        if !rings.is_empty()
            && let (Some(&root), Some(pos)) = (chain.first(), containers.global_position(id))
        {
            by_root.entry(root).or_default().push((g, pos));
        }
    }
    for named in by_segment.values_mut() {
        named.sort_by_key(Guid::to_string);
    }
    let mut out: HashMap<Guid, Vec<Site>> = HashMap::new();
    for (socpak, providers) in socpak_providers {
        let ring = rings.get(socpak).copied();
        for &id in containers.nesting(socpak) {
            let chain = containers.chain_of(id);
            let Some(named) = chain
                .iter()
                .rposition(|&c| containers.get(c).starmap_guid().is_some())
            else {
                continue;
            };
            let place = containers.get(chain[named]).starmap_guid().expect("named");
            let position = containers.global_position(id);
            let mut markers = match (named, chain.get(1)) {
                (0, Some(segment)) => by_segment.get(segment).cloned().unwrap_or_default(),
                _ => Vec::new(),
            };
            // A ring at the star: the named places inside it. A placement
            // with no `Pos` up its chain sits at the system's origin.
            if let (0, Some(r)) = (named, ring) {
                let centre = position.unwrap_or([0.0; 3]);
                for (g, p) in by_root.get(&chain[0]).into_iter().flatten() {
                    let offset = [p[0] - centre[0], p[1] - centre[1], p[2] - centre[2]];
                    if *g != place && r.contains(offset) && !markers.contains(g) {
                        markers.push(*g);
                    }
                }
                markers.sort_by_key(Guid::to_string);
            }
            let mut anchors: Vec<Guid> = Vec::new();
            for g in chain[..=named]
                .iter()
                .rev()
                .filter_map(|&c| containers.get(c).starmap_guid())
            {
                if !anchors.contains(&g) {
                    anchors.push(g);
                }
            }
            let site = Site {
                place,
                position,
                markers,
                ring,
                anchors,
            };
            for &provider in providers {
                out.entry(provider).or_default().push(site.clone());
            }
        }
    }
    for list in out.values_mut() {
        list.sort_by(|a, b| {
            a.place.to_string().cmp(&b.place.to_string()).then(
                a.position
                    .partial_cmp(&b.position)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
        });
    }
    out
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

/// The first `SAsteroidRingComponentParams` in a socpak member (an
/// `AsteroidRing` entity's field shape), km converted to m.
fn ring_shape(member: &str, root: &XmlNode) -> Option<Ring> {
    let member = member.to_ascii_lowercase();
    if !(member.ends_with("pivot.entxml") || member.ends_with(".soc") || member.ends_with(".pla")) {
        return None;
    }
    let c = root
        .descendants()
        .find(|n| n.attr("__type") == Some("SAsteroidRingComponentParams"))?;
    let km = |key: &str| -> Option<f64> { Some(c.attr(key)?.trim().parse::<f64>().ok()? * 1000.0) };
    Some(Ring {
        inner_radius: km("innerRadiusKm")?,
        outer_radius: km("outerRadiusKm")?,
        depth: km("depthKm")?,
    })
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

    /// The Aaron Halo's shape takes the Wikelo Selo Station's offset (20.196
    /// Gm, in the plane) and leaves out Crusader's (19.149 Gm) and a point
    /// above the band.
    #[test]
    fn ring_contains_the_band_only() {
        let halo = Ring {
            inner_radius: 19_597_600_000.0,
            outer_radius: 21_392_800_000.0,
            depth: 10_000_000.0,
        };
        assert!(halo.contains([14_281e6, 14_281e6, 0.0])); // ~20.196 Gm
        assert!(!halo.contains([19_149e6, 0.0, 0.0]));
        assert!(!halo.contains([20_196e6, 0.0, 6e6]));
    }

    const STAR: &str = "66666666-6666-6666-6666-666666666666";
    const BASE: &str = "77777777-7777-7777-7777-777777777777";
    const BODY: &str = "88888888-8888-8888-8888-888888888888";

    /// A cloud in an unnamed segment resolves to the star and gets the named
    /// places in its segment as markers; a field on a named body resolves to
    /// the body and needs none.
    #[test]
    fn sites_mark_star_fields_with_their_segment() {
        let g = graph(serde_json::json!([
            placement(1, "sys.socpak", "star.socpak", Some(STAR), None),
            placement(2, "sys.socpak", "segment.socpak", None, Some(1)),
            placement(3, "segment.socpak", "cloud.socpak", None, None),
            placement(4, "segment.socpak", "base.socpak", Some(BASE), None),
            placement(5, "sys.socpak", "body.socpak", Some(BODY), Some(1)),
            placement(6, "body.socpak", "field.socpak", None, None),
        ]));
        let g_ = |s: &str| s.parse::<Guid>().unwrap();
        let providers = HashMap::from([
            ("cloud.socpak".to_string(), vec![g_(P_CLOUD)]),
            ("field.socpak".to_string(), vec![g_(P_BODY)]),
        ]);
        let pl = ProviderLocations::compose(&g, &providers);
        let cloud = pl.sites_of(&g_(P_CLOUD));
        assert_eq!(cloud.len(), 1);
        assert_eq!(
            (cloud[0].place, cloud[0].markers.as_slice()),
            (g_(STAR), [g_(BASE)].as_slice())
        );
        assert_eq!(cloud[0].anchors, [g_(STAR)]);
        let field = pl.sites_of(&g_(P_BODY));
        assert_eq!((field[0].place, field[0].markers.len()), (g_(BODY), 0));
        assert_eq!(field[0].anchors, [g_(BODY), g_(STAR)]);
        assert!(pl.sites_of(&g_(BASE)).is_empty());
    }
}
