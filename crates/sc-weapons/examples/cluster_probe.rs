//! Cluster missiles: body explosion plus submunitions, against spviewer.
//!
//! ```bash
//! cargo run -p sc-weapons --release --example cluster_probe
//! ```
//!
//! LIVE 4.10: the Rattler II reads 1,370 + 7 × 300 = 3,470 (spviewer) and
//! is the only cluster `Missile`. The Viper (3,700 + 7 × 300) also carries
//! cluster params but is a `GroundVehicleMissile`, outside `iter_missiles`.

use sc_extract::{AssetConfig, AssetData, AssetSource};
use sc_weapons::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());

    let mut clusters = 0;
    for m in iter_missiles(&datacore, &items) {
        let Some(c) = &m.cluster else { continue };
        clusters += 1;
        let body = m.damage.map_or(0.0, |d| d.total());
        let total = m.total_damage().map_or(0.0, |d| d.total());
        println!(
            "{} S{}: body {body} + {} submunitions = {total} (detach after {} s, every {} s)",
            m.record_name,
            m.size,
            c.submunitions.len(),
            c.launch_detach_time,
            c.detach_delay,
        );
        for s in &c.submunitions {
            println!("  {} {} {}", s.port, s.guid, s.damage.total());
        }
        if m.record_name == "MISL_S02_IR_NOVP_Rattler" {
            assert_eq!(total, 3470.0, "{}", m.record_name);
        }
    }
    assert_eq!(clusters, 1, "LIVE 4.10 has one cluster missile");
    Ok(())
}
