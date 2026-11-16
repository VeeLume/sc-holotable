//! Re-proves the `sc-law` data claims on the installed build and prints every
//! jurisdiction with its contraband and controlled substances.
//!
//! Asserts:
//! - every `prohibitedResources` and `controlledSubstanceClasses[].resources`
//!   reference resolves in `sc_resources::Resources` (consumers join there,
//!   with no raw fallback);
//! - every `prohibitedGoods` and `controlledSubstanceClasses[].commodities`
//!   reference is an `EntityClassDefinition` record;
//! - at least one jurisdiction outlaws something (an empty walk is a
//!   feature-gating bug, not "no laws").
//!
//! ```bash
//! cargo run -p sc-law --release --example jurisdictions
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordPaths};
use sc_law::{Jurisdictions, RecordCollection};
use sc_resources::Resources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;

    let jurisdictions = Jurisdictions::build(datacore.records());
    let resources = Resources::build(datacore.records());
    let paths = RecordPaths::build(&datacore);
    let short = |g: &Guid| {
        paths
            .get(g)
            .map_or_else(|| format!("<missing {g}>"), |p| p.short_name().to_string())
    };
    let type_of = |g: &Guid| {
        paths
            .get(g)
            .and_then(|p| paths.type_name(p.struct_index))
            .unwrap_or("<missing>")
            .to_string()
    };

    let mut resource_refs = 0;
    let mut unresolved = Vec::new();
    let mut good_refs = 0;
    let mut wrong_goods = Vec::new();
    let mut outlawing = 0;

    let mut sorted: Vec<_> = jurisdictions.values().collect();
    sorted.sort_by_key(|j| short(&j.guid));
    println!("jurisdictions: {}", sorted.len());
    for j in sorted {
        let name = j.name_key.as_ref().and_then(|k| data.locale.resolve(k));
        println!(
            "\n{} {:?} parent={:?} respects_parent_laws={} prison={} max_stolen_scu={}",
            short(&j.guid),
            name.unwrap_or("-"),
            j.parent.as_ref().map(short),
            j.respects_parent_laws,
            j.is_prison,
            j.max_stolen_goods_scu,
        );
        let mut check_resource = |g: &Guid| {
            resource_refs += 1;
            if resources.get(g).is_none() {
                unresolved.push(format!(
                    "{} -> {} ({})",
                    short(&j.guid),
                    short(g),
                    type_of(g)
                ));
            }
            short(g)
        };
        let contraband: Vec<_> = j
            .prohibited_resources
            .iter()
            .map(&mut check_resource)
            .collect();
        println!("  contraband: {}", contraband.join(", "));
        let mut classes = Vec::new();
        for c in &j.controlled_substances {
            let drugs: Vec<_> = c.resources.iter().map(&mut check_resource).collect();
            classes.push((c.max_possession_scu, drugs));
        }
        for (i, (max, drugs)) in classes.iter().enumerate() {
            println!("  controlled[{i}] max {max} SCU: {}", drugs.join(", "));
        }
        let goods = j
            .prohibited_goods
            .iter()
            .chain(j.controlled_substances.iter().flat_map(|c| &c.commodities));
        for g in goods {
            good_refs += 1;
            let ty = type_of(g);
            if ty != "EntityClassDefinition" {
                wrong_goods.push(format!("{} -> {} ({ty})", short(&j.guid), short(g)));
            }
        }
        if !j.prohibited_resources.is_empty()
            || j.controlled_substances
                .iter()
                .any(|c| !c.resources.is_empty())
        {
            outlawing += 1;
        }
    }

    println!(
        "\nresource refs: {resource_refs}, unresolved in Resources: {}",
        unresolved.len()
    );
    println!(
        "goods refs: {good_refs}, not EntityClassDefinition: {}",
        wrong_goods.len()
    );
    println!("jurisdictions outlawing something: {outlawing}");
    assert!(
        unresolved.is_empty(),
        "resource refs outside Resources: {unresolved:#?}"
    );
    assert!(
        wrong_goods.is_empty(),
        "goods refs of another type: {wrong_goods:#?}"
    );
    assert!(
        outlawing > 0,
        "no jurisdiction outlaws anything: check feature gating"
    );
    Ok(())
}
