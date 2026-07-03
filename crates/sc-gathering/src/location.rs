//! Location join — `StarMapObject` ↔ provider.
//!
//! Composed from two halves:
//! - `sc_locations::ObjectContainers` — place ↔ socpak (the placement graph's
//!   realized-socpak index).
//! - each body/field socpak's `HarvestableProviderComponent.preset` — socpak ↔ provider.
//!
//! The result keys the provider-GUID-keyed [`crate::Providers`] by actual place:
//! `StarMapObject → provider → resources`, and the inverse
//! `provider → StarMapObjects` (the 1:many asteroid-base reuse falls out, since
//! many places share one socpak/provider).
//!
//! [`ProviderLocations::cook`] needs a **live** p4k (it opens body socpaks). The
//! join ([`ProviderLocations::compose`]) and the per-socpak extraction are
//! unit-tested; the full cook is a p4k-time step (system socpaks aren't in the
//! offline probe corpus).

use std::collections::HashMap;

use sc_extract::object_container::{Socpak, decode};
use sc_extract::{AssetSource, Guid, Result};
use sc_locations::{ObjectContainers, normalize_socpak_path};
use serde::{Deserialize, Serialize};

/// `StarMapObject` ↔ `HarvestableProviderPreset` GUID, both directions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderLocations {
    /// `StarMapObject` GUID → its provider (1:1).
    by_location: HashMap<Guid, Guid>,
    /// provider GUID → the places it applies to (1:many — reused socpaks).
    by_provider: HashMap<Guid, Vec<Guid>>,
}

impl ProviderLocations {
    /// Bump when the cooked layout changes (for `ProcessedSnapshot` gating).
    pub const COOK_SCHEMA_VERSION: u32 = 1;

    /// Cook from a live p4k + an [`ObjectContainers`] graph: open each distinct
    /// body socpak once, read its provider preset, and join to the places that
    /// socpak realizes. Empty for a snapshot-backed source (socpaks aren't there).
    pub fn cook(assets: &AssetSource, containers: &ObjectContainers) -> Result<Self> {
        let mut socpak_provider: HashMap<String, Guid> = HashMap::new();
        for socpak in containers.realized_socpaks() {
            let Some((_, bytes)) =
                assets.find_and_read(|name| normalize_socpak_path(name) == socpak)?
            else {
                continue;
            };
            if let Some(provider) = provider_in_socpak(bytes)? {
                socpak_provider.insert(socpak.to_string(), provider);
            }
        }
        Ok(Self::compose(containers, &socpak_provider))
    }

    /// Pure join: `socpak → provider` ⊕ `place ↔ socpak` ⇒ `place ↔ provider`.
    pub fn compose(containers: &ObjectContainers, socpak_provider: &HashMap<String, Guid>) -> Self {
        let mut pl = Self::default();
        for socpak in containers.realized_socpaks() {
            let Some(&provider) = socpak_provider.get(socpak) else {
                continue;
            };
            for &loc in containers.locations_in(socpak) {
                pl.by_location.insert(loc, provider);
                let bucket = pl.by_provider.entry(provider).or_default();
                if !bucket.contains(&loc) {
                    bucket.push(loc);
                }
            }
        }
        pl
    }

    /// The provider that applies at a place (`StarMapObject` GUID).
    pub fn provider_at(&self, location: &Guid) -> Option<Guid> {
        self.by_location.get(location).copied()
    }

    /// Every place a provider applies to (1:many for reused asteroid bases).
    pub fn locations_of(&self, provider: &Guid) -> &[Guid] {
        self.by_provider
            .get(provider)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn len(&self) -> usize {
        self.by_location.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_location.is_empty()
    }
}

/// The first non-null `HarvestableProviderComponent.preset` in a socpak. The
/// provider sits on the body's root entity (`pivot.entxml`) or the field's
/// `.soc`/`.pla`; the deep `entdata/*` child entities are skipped.
fn provider_in_socpak(bytes: Vec<u8>) -> Result<Option<Guid>> {
    let mut pak = Socpak::open(bytes)?;
    for i in 0..pak.len() {
        let Some(name) = pak.name(i) else { continue };
        let lname = name.to_ascii_lowercase();
        if !(lname.ends_with("pivot.entxml") || lname.ends_with(".soc") || lname.ends_with(".pla"))
        {
            continue;
        }
        let Ok(Some(root)) = decode(&pak.read(i)?) else {
            continue;
        };
        for comp in root.find_all("HarvestableProviderComponent") {
            if let Some(preset) = comp.attr("preset")
                && !preset.is_empty()
                && !preset.starts_with("00000000-")
                && let Ok(g) = preset.parse::<Guid>()
            {
                return Ok(Some(g));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// A one-member socpak whose pivot.entxml carries a provider component.
    fn socpak_with(preset: &str) -> Vec<u8> {
        let mut zw = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zw.start_file("body/pivot.entxml", opts).unwrap();
        let xml = format!(
            r#"<Entity EntityClass="ProceduralEntity"><PropertiesDataCore>
                 <HarvestableProviderComponent __type="HarvestableProviderParams" preset="{preset}"/>
               </PropertiesDataCore></Entity>"#
        );
        zw.write_all(xml.as_bytes()).unwrap();
        zw.finish().unwrap().into_inner()
    }

    #[test]
    fn extracts_provider_from_socpak() {
        let g = "703a18ca-7f7c-4489-a64a-cd0cd359b8fe";
        assert_eq!(
            provider_in_socpak(socpak_with(g)).unwrap(),
            Some(g.parse().unwrap())
        );
    }

    #[test]
    fn null_preset_yields_none() {
        assert_eq!(
            provider_in_socpak(socpak_with("00000000-0000-0000-0000-000000000000")).unwrap(),
            None
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
        assert_eq!(
            provider_in_socpak(bytes).unwrap(),
            Some("703a18ca-7f7c-4489-a64a-cd0cd359b8fe".parse().unwrap())
        );
    }
}
