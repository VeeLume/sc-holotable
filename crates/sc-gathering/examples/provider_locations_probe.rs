//! Re-proves the provider ↔ place join against LIVE and prints each
//! provider's places. LIVE 4.10, 2026-10-16:
//!
//! - the walk from the solar systems the `MegaMap` records list (9 s, about
//!   0.8 GB over the DCB parse) finds a subset of what the same join over
//!   every object container finds (16 s). The only extras there come from
//!   content no live system includes: the orphaned `nyx/delamar` (8 Mining
//!   Claims), Arena Commander's `ea/jerichostation` (MIC L1-B) and `ellis3`
//!   (Green).
//! - Yela's ring (`HPP_Stanton2c_Belt`) shares Yela's socpak and maps to Yela;
//!   the Nyx belts' gas clouds sit in unnamed ring segments whose
//!   `parentGUID` is the Nyx star, so they map to Nyx (no placement names
//!   `StarMapObject.GlaciemRing` / `KeegerBelt`).
//! - six providers have no place: their socpaks (`akirocluster`,
//!   `miningasteroid_004` / `_006`, `gas_lrg_01`) are included by nothing
//!   and named by no DCB string, and two are in no object container at all.
//!   Taken as development leftovers (Valerie: not in the game today).
//!
//! ```bash
//! cargo run -p sc-gathering --release --example provider_locations_probe
//! ```

use std::collections::BTreeSet;
use std::time::Instant;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordPaths};
use sc_gathering::{ProviderLocations, Providers, RecordCollection};
use sc_locations::Locations;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("warn").init();
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let providers = Providers::build(datacore.records(), &RecordPaths::build(&datacore));
    let locations = Locations::build(datacore.records());
    let db = datacore.db();
    // Probe output names records; the join itself is by GUID.
    let record = |g: &Guid| {
        db.record(g)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_else(|| g.to_string())
    };
    let place = |g: &Guid| match locations.get(g) {
        Some(l) => l
            .display_name(&data.locale)
            .map_or_else(|| record(g), str::to_string),
        None => record(g),
    };

    let t = Instant::now();
    let walked = ProviderLocations::cook(&assets, datacore.records())?;
    println!(
        "walk from the solar systems: {:.1} s, {} places",
        t.elapsed().as_secs_f32(),
        walked.len()
    );
    let every: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak") && l.contains("objectcontainers")
        })
        .map(|e| e.name.to_string())
        .collect();
    let t = Instant::now();
    let full = ProviderLocations::cook_from(&assets, &every)?;
    println!(
        "every object container ({}): {:.1} s, {} places",
        every.len(),
        t.elapsed().as_secs_f32(),
        full.len()
    );

    let mut rows: Vec<(String, Guid)> = providers.guids().map(|g| (record(g), *g)).collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    let (mut same, mut unplaced) = (0, Vec::new());
    let places_of = |name: &str| -> BTreeSet<String> {
        rows.iter()
            .find(|(n, _)| n == name)
            .map(|(_, g)| walked.locations_of(g).iter().map(place).collect())
            .unwrap_or_default()
    };
    for (name, guid) in &rows {
        let a: BTreeSet<String> = walked.locations_of(guid).iter().map(place).collect();
        let b: BTreeSet<String> = full.locations_of(guid).iter().map(place).collect();
        assert!(
            a.is_subset(&b),
            "{name}: the walk found places the full join lacks"
        );
        if a.is_empty() {
            unplaced.push(name.clone());
        }
        let sample: Vec<&String> = a.iter().take(3).collect();
        println!("{name:34} {:>3} | {sample:?}", a.len());
        if a == b {
            same += 1;
        } else {
            println!(
                "    only in the full walk: {:?}",
                b.difference(&a).collect::<Vec<_>>()
            );
            println!(
                "    only in the solar-system walk: {:?}",
                a.difference(&b).collect::<Vec<_>>()
            );
        }
    }
    println!(
        "\n{same} / {} providers have the same places both ways; unplaced: {unplaced:?}",
        rows.len()
    );
    assert!(places_of("HPP_Stanton2c_Belt").contains("Yela"));
    assert!(places_of("HPP_Stanton2c").contains("Yela"));
    assert!(places_of("HPP_Nyx_GlaciemRing").contains("Nyx"));
    assert!(places_of("HPP_Nyx_KeegerBelt").contains("Nyx"));
    assert_eq!(
        unplaced,
        [
            "HPP_Lagrange_G",
            "HPP_Lagrange_Occupied",
            "HPP_Pyro_AkiroCluster",
            "HPP_ResourceRush_Gold",
            "HPP_ResourceRush_Gold_HighDensity",
            "HPP_SpaceDerelict_General",
        ]
    );
    println!("OK");
    Ok(())
}
