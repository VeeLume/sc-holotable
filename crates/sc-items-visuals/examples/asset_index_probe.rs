//! Proves the cooked asset index on the real archive: every recipe listed,
//! a sample of files read through it byte for byte as a full open reads them,
//! and the archive's file list left unloaded.
//!
//! `cargo run -p sc-items-visuals --release --example asset_index_probe`

use std::time::Instant;

use sc_extract::{AssetConfig, AssetData, AssetIndex, AssetSource, Datacore, MeshGeometry};
use sc_items::Items;
use sc_items_visuals::Visuals;

/// Files a host reads by a known path, beside what recipes need.
const HOST_FILES: [&str; 2] = [
    "localization/english/global.ini",
    "ui/fonts/shared/fonts_en.gfx",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let p4k = install.data_p4k();

    let t = Instant::now();
    let live = AssetSource::open(&p4k)?;
    println!("full open: {:?}", t.elapsed());
    let archive = live.archive().expect("live");
    let beyond_game_levels = archive
        .iter()
        .filter(|e| {
            let name = e.name.to_ascii_lowercase();
            (6..=9).any(|n| name.contains(&format!("_lod{n}.")))
        })
        .count();
    println!("files named above level 5: {beyond_game_levels}");
    for file in HOST_FILES {
        let wanted = format!("data\\{}", file.replace('/', "\\"));
        match archive
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case(&wanted))
        {
            Some(e) => println!(
                "{file}: encrypted {}, {:?}, {} bytes",
                e.is_encrypted, e.compression_method, e.uncompressed_size
            ),
            None => println!("{file}: NOT IN THE ARCHIVE"),
        }
    }

    let data = AssetData::extract(&live, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&live, &data)?;
    let items = Items::build(datacore.records());
    let visuals = Visuals::build(&datacore, &items);

    let t = Instant::now();
    let mut builder = AssetIndex::builder(&live)?;
    visuals.add_files(&mut builder);
    for file in HOST_FILES {
        builder.add(file);
    }
    let index = builder.finish();
    let bytes = index.to_bytes();
    println!(
        "cook: {:?}, {} paths listed, {} KB",
        t.elapsed(),
        index.len(),
        bytes.len() / 1000
    );

    let dir = std::env::temp_dir().join("sc-asset-index-probe");
    std::fs::create_dir_all(&dir)?;
    let index_file = dir.join("assets.index");
    std::fs::write(&index_file, &bytes)?;

    let t = Instant::now();
    let indexed = AssetSource::open_indexed(&p4k, &index_file)?;
    println!("indexed open: {:?}", t.elapsed());
    assert!(!indexed.holds_file_list(), "the index was not used");

    let mut models: Vec<String> = sc_extract::RecordCollection::iter(&visuals)
        .filter_map(|(_, v)| v.geometry.as_ref()?.geometry.clone())
        .filter(|p| !p.to_ascii_lowercase().ends_with(".cdf"))
        .collect();
    models.sort();
    models.dedup();
    let sample: Vec<&String> = models.iter().step_by(models.len().div_ceil(800)).collect();

    let t = Instant::now();
    let mut through_index = Vec::with_capacity(sample.len());
    for path in &sample {
        through_index.push((
            indexed.read_data(path).ok(),
            MeshGeometry::read_shape_level(&indexed, path, 2).ok(),
        ));
    }
    let indexed_time = t.elapsed();
    assert!(
        !indexed.holds_file_list(),
        "a listed read opened the whole archive"
    );

    let t = Instant::now();
    for (path, (bytes, mesh)) in sample.iter().zip(&through_index) {
        assert_eq!(*bytes, live.read_data(path).ok(), "{path}");
        let full = MeshGeometry::read_shape_level(&live, path, 2).ok();
        assert_eq!(
            mesh.as_ref()
                .map(|g| (g.level, g.positions.len(), g.indices.len())),
            full.as_ref()
                .map(|g| (g.level, g.positions.len(), g.indices.len())),
            "{path}"
        );
    }
    println!(
        "{} models of {}: same bytes and meshes; indexed {:?}, full {:?}",
        sample.len(),
        models.len(),
        indexed_time,
        t.elapsed()
    );

    for file in HOST_FILES {
        let wanted = indexed.read_data(file)?;
        assert_eq!(wanted, live.read_data(file)?, "{file}");
        println!("{file}: {} bytes through the index", wanted.len());
    }
    assert!(
        !indexed.holds_file_list(),
        "a host file opened the whole archive"
    );

    let t = Instant::now();
    assert!(indexed.try_read("nothing/listed/here.bin")?.is_none());
    assert!(indexed.holds_file_list());
    println!(
        "unlisted path, whole archive opened once: {:?}",
        t.elapsed()
    );

    std::fs::remove_dir_all(&dir)?;
    Ok(())
}
