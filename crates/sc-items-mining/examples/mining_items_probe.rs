//! Re-proves the mining module and gadget values the crate reads, against the
//! LIVE data: the Surge module (x1.5 power on fire action 0, x1.0 on fire
//! action 1, resistance -15.5 %, instability +10 %, active for 15 s, 7 uses)
//! and the BoreMax gadget (instability -70 %). Then lists every module and
//! gadget.
//!
//! ```bash
//! cargo run -p sc-items-mining --release --example mining_items_probe
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_mining::{
    Activation, FloatModifier, MiningGadgets, MiningModifiers, MiningModules, ModifierKind,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let db = datacore.db();
    let name = |guid: &Guid| {
        db.record(guid)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_default()
    };

    let modules = MiningModules::build(&datacore, &items);
    let gadgets = MiningGadgets::build(&datacore, &items);
    println!("{} modules, {} gadgets", modules.len(), gadgets.len());

    let surge = modules
        .iter()
        .find(|(g, _)| name(g) == "Mining_Modules_Active_Surge")
        .map(|(_, m)| m)
        .ok_or("no Surge module")?;
    assert_eq!(surge.damage_multiplier(0), 1.5);
    assert_eq!(surge.damage_multiplier(1), 1.0);
    assert_eq!(pct(&surge.modifiers.resistance), Some(-15.5));
    assert_eq!(pct(&surge.modifiers.laser_instability), Some(10.0));
    assert_eq!(
        surge.activation,
        Activation::Active {
            duration_s: Some(15.0),
            uses: Some(7)
        }
    );

    let boremax = gadgets
        .iter()
        .find(|(g, _)| name(g) == "Mining_Gadget_THCN_BoreMax")
        .map(|(_, m)| m)
        .ok_or("no BoreMax gadget")?;
    assert_eq!(pct(&boremax.modifiers.laser_instability), Some(-70.0));
    println!("asserts passed: Surge, BoreMax\n");

    let mut rows: Vec<_> = modules.iter().map(|(g, m)| (name(g), m)).collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    for (n, m) in rows {
        let beams: Vec<String> = m
            .beam_multipliers
            .iter()
            .map(|b| format!("[{}]x{}", b.fire_action, b.damage_multiplier))
            .collect();
        let filter = m
            .inert_filter
            .map(|f| format!(" inert={}", f.value))
            .unwrap_or_default();
        println!(
            "{n:<44} {:<14} {:?}  {}{filter}",
            beams.join(" "),
            m.activation,
            show(&m.modifiers)
        );
    }
    println!();
    let mut rows: Vec<_> = gadgets.iter().map(|(g, m)| (name(g), m)).collect();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    for (n, m) in rows {
        println!("{n:<44} {}", show(&m.modifiers));
    }
    Ok(())
}

fn pct(m: &Option<FloatModifier>) -> Option<f32> {
    m.map(|m| m.value)
}

fn show(m: &MiningModifiers) -> String {
    [
        ("inst", m.laser_instability),
        ("win", m.optimal_charge_window_size),
        ("res", m.resistance),
        ("shatter", m.shatter_damage),
        ("cluster", m.cluster_factor),
        ("rate", m.optimal_charge_window_rate),
        ("cat", m.catastrophic_charge_window_rate),
    ]
    .iter()
    .filter_map(|(k, v)| {
        v.map(|v| {
            let add = if v.kind == ModifierKind::Additive {
                "(add)"
            } else {
                ""
            };
            format!("{k}={}{add}", v.value)
        })
    })
    .collect::<Vec<_>>()
    .join(" ")
}
