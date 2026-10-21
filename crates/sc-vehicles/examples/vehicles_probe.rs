//! Re-proves the vehicle model against spviewer (LIVE 4.10.197): the RSI
//! Constellation Mk V Centaurus's stock loadout resolves to every component
//! spviewer lists (S5 gimbals with CF-557s, remote turrets with four
//! CF-337 gimbals, S6 racks with 16 StrikeForce IIs, 5CA shields, Sedulity
//! power plants, Snowfall coolers, Khaos with the Excelsior jump drive,
//! Lephari radar, ComfortAir Plus, the flight blade), its gimbals are
//! locked while their guns can be swapped, and its thrusters are locked.
//! Then it checks [`fits`] on every stock attachment of the non-AI vehicles
//! and counts the misfits by kind. Every listed vehicle has a manufacturer
//! (an `SCItemManufacturer` record), career, role and description.
//!
//! ```bash
//! cargo run -p sc-vehicles --release --example vehicles_probe
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_vehicles::{Misfit, Mounted, Port, PortContainers, Vehicle, Vehicles, fits, is_editable};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let containers = PortContainers::build(&datacore, &assets);
    let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
    let names: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| Some((r.id(), short(r.name()?))))
        .collect();
    let name = |g: Option<Guid>| g.and_then(|g| names.get(&g)).map_or("-", String::as_str);
    println!(
        "{} vehicles, {} port containers",
        vehicles.len(),
        containers.len()
    );

    let (guid, ship) = vehicles
        .iter()
        .find(|(g, _)| names.get(g).map(String::as_str) == Some("RSI_Constellation_Mk5_Centaurus"))
        .ok_or("no Centaurus")?;
    assert_eq!(ship.hull_health(), 170_500.0);
    assert_eq!(ship.crew_size, 4);
    // Insurance terms (AI and unmanned records carry them too, so they do
    // not tell player ships apart).
    let ins = ship.insurance.as_ref().ok_or("no insurance")?;
    assert_eq!((ins.base_wait_minutes, ins.expediting_fee), (13.5, 6750));
    // erkul's 25.4k cross-section is the largest axis.
    assert_eq!(ship.cross_section, Some([8350.0, 11320.0, 25430.0]));
    let at = |path: &[&str]| -> Option<&Mounted> {
        let mut level = &ship.stock;
        let mut found = None;
        for p in path {
            let m = level.iter().find(|m| m.port.eq_ignore_ascii_case(p))?;
            level = &m.children;
            found = Some(m);
        }
        found
    };
    for (path, want) in [
        (&["hardpoint_gun_s5_front_left"][..], "Mount_Gimbal_S5"),
        (
            &["hardpoint_gun_s5_front_left", "hardpoint_class_2"][..],
            "KLWE_LaserRepeater_S5",
        ),
        (
            &["hardpoint_shield_generator_01"][..],
            "SHLD_BEHR_S03_5CA_SCItem",
        ),
        (
            &["hardpoint_power_plant_02"][..],
            "POWR_JUST_S02_Sedulity_SCItem",
        ),
        (
            &["hardpoint_cooler_01"][..],
            "COOL_JUST_S02_Snowfall_SCItem",
        ),
        (
            &["hardpoint_quantum_drive"][..],
            "QDRV_RSI_S02_Khaos_SCItem",
        ),
        (
            &["hardpoint_quantum_drive", "hardpoint_Jump_Drive"][..],
            "JDRV_TARS_S02_Excelsior_SCItem",
        ),
        (&["hardpoint_radar"][..], "RADR_WLOP_S03_Lephari"),
        (
            &["hardpoint_life_support"][..],
            "LFSP_TYDT_S02_ComfortAirPlus",
        ),
        (
            &["hardpoint_controller_flight"][..],
            "Controller_Flight_RSI_Constellation_Mk5",
        ),
    ] {
        let got = name(at(path).and_then(|m| m.item));
        assert_eq!(got, want, "{path:?}");
    }
    let turret = at(&["hardpoint_turret_upper"]).ok_or("upper turret")?;
    let guns: Vec<&str> = turret
        .children
        .iter()
        .filter_map(|g| g.children.first())
        .map(|m| name(m.item))
        .collect();
    assert_eq!(guns, ["KLWE_LaserRepeater_S3"; 4], "{turret:#?}");
    let rack = at(&["hardpoint_missiles_left"]).ok_or("rack")?;
    let missiles = rack
        .children
        .iter()
        .filter(|m| name(m.item) == "MISL_S02_CS_THCN_StrikeForce")
        .count();
    assert_eq!(missiles, 16);

    let gimbal_port = ship.port("hardpoint_gun_s5_front_left").ok_or("gun port")?;
    let gimbal = at(&["hardpoint_gun_s5_front_left"])
        .and_then(|m| m.item)
        .ok_or("gimbal")?;
    let gun_port = containers
        .get(&gimbal)
        .and_then(|c| c.port("hardpoint_class_2"))
        .ok_or("gimbal's gun port")?;
    assert!(!is_editable(&[gimbal_port]));
    assert!(is_editable(&[gimbal_port, gun_port]));
    let thruster = ship.port("hardpoint_thruster_main_left").or_else(|| {
        ship.ports
            .iter()
            .find(|p| p.name.starts_with("hardpoint_thruster_main"))
    });
    assert!(!is_editable(&[thruster.ok_or("thruster port")?]));
    // Life support lists no types (flagged `editable`): nothing to choose,
    // as erkul and the game show it.
    assert!(!is_editable(&[ship
        .port("hardpoint_life_support")
        .ok_or("life support port")?]));
    // Listing: rewards, base hulls (named by some insurance), editions
    // (own name, no AI crew, ready for inclusion), variants.
    let mut listing: BTreeMap<String, u32> = BTreeMap::new();
    for v in vehicles.values() {
        *listing.entry(format!("{:?}", v.listing)).or_default() += 1;
    }
    let distinct: std::collections::HashSet<_> =
        vehicles.listed().map(|(_, v)| v.name_key.clone()).collect();
    println!(
        "listing {listing:?}; listed {} records, {} distinct names",
        vehicles.listed().count(),
        distinct.len()
    );
    let listed: Vec<&sc_vehicles::Vehicle> = vehicles.listed().map(|(_, v)| v).collect();
    let ground = listed.iter().filter(|v| v.ground).count();
    let gravlev = listed.iter().filter(|v| v.gravlev).count();
    println!(
        "  {} ground, {gravlev} gravlev, {} rewards",
        ground,
        vehicles.values().filter(|v| v.reward).count()
    );
    // Every one of erkul's 222 (GUID-joined) plus 24 the data cannot tell
    // apart.
    assert_eq!(listed.len(), 246);
    assert_eq!(distinct.len(), 240);
    assert_eq!((ground, gravlev), (27, 11));
    assert_eq!(ship.listing, sc_vehicles::Listing::Edition);
    let skylark = vehicles
        .iter()
        .find(|(g, _)| names.get(g).is_some_and(|n| n == "RSI_Skylark"))
        .ok_or("no Skylark")?;
    assert_eq!(skylark.1.listing, sc_vehicles::Listing::Base);
    let ai: Vec<&str> = vehicles
        .listed()
        .filter_map(|(g, _)| names.get(g).map(String::as_str))
        .filter(|n| n.contains("_AI"))
        .collect();
    println!("listed with _AI in the record name: {ai:?}");

    // The ship header: manufacturer, career, role, description.
    let makers: HashMap<Guid, String> = datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("SCItemManufacturer"))
        .filter_map(|r| Some((r.id(), short(r.name()?))))
        .collect();
    let maker = ship.manufacturer.and_then(|g| makers.get(&g));
    println!(
        "Centaurus header: {maker:?}, career {:?}, role {:?}, description {:?}",
        ship.career_key, ship.role_key, ship.description_key
    );
    let header = |f: fn(&sc_vehicles::Vehicle) -> bool| listed.iter().filter(|v| f(v)).count();
    let with_maker = header(|v| v.manufacturer.is_some());
    let with_career = header(|v| v.career_key.is_some());
    let with_role = header(|v| v.role_key.is_some());
    let with_description = header(|v| v.description_key.is_some());
    println!(
        "listed with manufacturer {with_maker}, career {with_career}, role {with_role}, description {with_description}"
    );
    let unresolved: Vec<&str> = vehicles
        .listed()
        .filter(|(_, v)| v.manufacturer.is_some_and(|g| !makers.contains_key(&g)))
        .filter_map(|(g, _)| names.get(g).map(String::as_str))
        .collect();
    println!("listed with a manufacturer that is no SCItemManufacturer: {unresolved:?}");
    // LIVE 4.10: every listed vehicle has all four (the Centaurus carries
    // the Andromeda's description key, as the data ships it).
    assert!(maker.is_some_and(|m| m.starts_with("RSI")), "{maker:?}");
    assert!(unresolved.is_empty());
    assert_eq!(
        (with_maker, with_career, with_role, with_description),
        (246, 246, 246, 246)
    );

    println!("asserts passed: Centaurus stock tree and locks ({guid})");

    // fits() over every stock attachment of the non-AI vehicles.
    let mut total = 0u32;
    let (mut editable_total, mut editable_bad) = (0u32, 0u32);
    let mut misfits: BTreeMap<&str, u32> = BTreeMap::new();
    let mut samples: Vec<String> = Vec::new();
    for (guid, v) in vehicles.iter() {
        let vname = names.get(guid).map_or("?", String::as_str);
        if ["_AI", "Unmanned", "Template"]
            .iter()
            .any(|s| vname.contains(s))
        {
            continue;
        }
        check(
            v,
            &v.ports,
            &v.stock,
            &mut Vec::new(),
            &mut Vec::new(),
            &items,
            &vehicles,
            &containers,
            &mut |port, item, editable, result| {
                total += 1;
                editable_total += u32::from(editable);
                if result.is_err() && editable {
                    editable_bad += 1;
                }
                if let Err(m) = result {
                    let kind = match m {
                        Misfit::Type => "type",
                        Misfit::Size => "size",
                        Misfit::PortTags(_) => "port tags",
                        Misfit::ItemTags(_) => "item tags",
                    };
                    let n = misfits.entry(kind).or_default();
                    *n += 1;
                    if *n <= 4 {
                        samples.push(format!("{vname}: {port} <- {} ({m:?})", name(Some(item))));
                    }
                }
            },
        );
    }
    let bad: u32 = misfits.values().sum();
    println!(
        "fits(): {total} stock attachments, {bad} misfits ({:.1} %): {misfits:?}",
        100.0 * f64::from(bad) / f64::from(total.max(1))
    );
    println!(
        "  on ports the player can change: {editable_total} attachments, {editable_bad} misfits ({:.2} %)",
        100.0 * f64::from(editable_bad) / f64::from(editable_total.max(1))
    );
    for s in &samples {
        println!("  {s}");
    }
    Ok(())
}

/// Called with each attachment's port, item, whether the player can change
/// it, and its fit result.
type Seen<'s> = dyn FnMut(&str, Guid, bool, Result<(), Misfit>) + 's;

/// Walk a stock tree, calling `seen` for every attachment.
#[allow(clippy::too_many_arguments)]
fn check<'a>(
    v: &'a Vehicle,
    ports: &'a [Port],
    level: &'a [Mounted],
    chain: &mut Vec<&'a Port>,
    holders: &mut Vec<Guid>,
    items: &'a Items,
    vehicles: &'a Vehicles,
    containers: &'a PortContainers,
    seen: &mut Seen<'_>,
) {
    for m in level {
        let (Some(port), Some(item_guid)) = (
            ports.iter().find(|p| p.name.eq_ignore_ascii_case(&m.port)),
            m.item,
        ) else {
            continue;
        };
        let Some(item) = items.get(&item_guid) else {
            continue;
        };
        chain.push(port);
        let offered = v.offered_tags(items, chain, holders);
        seen(
            &port.name,
            item_guid,
            is_editable(chain),
            fits(port, item, &offered),
        );
        let child_ports: &[Port] = match vehicles.get(&item_guid) {
            Some(docked) => &docked.ports,
            None => containers.get(&item_guid).map_or(&[][..], |c| &c.ports[..]),
        };
        holders.push(item_guid);
        check(
            v,
            child_ports,
            &m.children,
            chain,
            holders,
            items,
            vehicles,
            containers,
            seen,
        );
        holders.pop();
        chain.pop();
    }
}

fn short(name: &str) -> String {
    name.rsplit('.').next().unwrap_or(name).to_string()
}
