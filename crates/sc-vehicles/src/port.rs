//! Item ports: what a port takes and whether it can be changed.

use sc_extract::generated::EItemSubType;
use sc_extract::{Guid, LocaleKey, PortType, VehiclePort};
use sc_items::Item;
use serde::{Deserialize, Serialize};

/// One item port, from a vehicle's hull XML or an entity's DCB port
/// container (the two are merged on a vehicle; items' child ports only come
/// from the DCB).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub name: String,
    pub display_name: Option<LocaleKey>,
    pub source: PortSource,
    /// What it takes; see [`fits`].
    pub types: Vec<PortType>,
    pub min_size: i32,
    pub max_size: i32,
    /// `flags`, split on spaces as authored (a leading `$` kept); see
    /// [`has_flag`].
    pub flags: Vec<String>,
    /// `portTags`: tags it offers the items below it.
    pub port_tags: Vec<String>,
    /// `requiredTags` / `RequiredPortTags`: tags an item must carry.
    pub required_tags: Vec<String>,
    /// `SItemPortDef.defaultItem`: the item it holds when a loadout names
    /// none (DCB ports only).
    pub default_item: Option<Guid>,
}

/// Where a [`Port`] is defined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortSource {
    /// The vehicle XML's `<Part class="ItemPort">`.
    Hull,
    /// The entity's `SItemPortContainerComponentParams.Ports`.
    Entity,
}

impl Port {
    pub(crate) fn from_hull(name: &str, p: &VehiclePort) -> Self {
        Self {
            name: name.to_string(),
            display_name: p
                .display_name
                .as_deref()
                .map(|k| LocaleKey::new(format!("@{k}"))),
            source: PortSource::Hull,
            types: p.types.clone(),
            min_size: p.min_size,
            max_size: p.max_size,
            flags: p.flags.clone(),
            port_tags: p.port_tags.clone(),
            required_tags: p.required_tags.clone(),
            default_item: None,
        }
    }
}

/// Why an item does not fit a port.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Misfit {
    /// Its `AttachDef.Type` / `SubType` is not among the port's types.
    Type,
    /// Its size is outside the port's range.
    Size,
    /// It lacks these of the port's required tags.
    PortTags(Vec<String>),
    /// These of its own required tags are offered by nothing above it.
    ItemTags(Vec<String>),
}

/// Whether `item` fits `port`, with `offered` the tags available from above
/// it: the port's own `portTags`, every ancestor port's and item's, the
/// vehicle XML's `itemPortTags` and the vehicle entity's tags (see
/// [`Vehicle::offered_tags`](crate::Vehicle::offered_tags)).
///
/// The rule is inferred from the data; the client check was not decompiled.
/// On LIVE 4.10's non-AI vehicles it rejects 1.3 % of the 13,493 stock
/// attachments on ports the player can change ([`is_editable`]), and 3.5 %
/// of all 49,495, the rest on locked ports, which the game fills by force
/// (`vehicles_probe`). The rule:
/// the item's type is one of the port's, and its sub-type one of that
/// type's listed sub-types (any when none is listed, and an `UNDEFINED`
/// sub-type passes; a port that lists no types at all is not judged on
/// type); its size lies within the port's range; it carries
/// every tag the port requires; and every tag it requires is offered.
/// Tags compare without a leading `$` and without case.
pub fn fits(port: &Port, item: &Item, offered: &[&str]) -> Result<(), Misfit> {
    let type_ok = port.types.is_empty()
        || port.types.iter().any(|t| {
            t.item_type == item.item_type
                && (t.sub_types.is_empty()
                    || item.item_sub_type == EItemSubType::UNDEFINED
                    || t.sub_types.contains(&item.item_sub_type))
        });
    if !type_ok {
        return Err(Misfit::Type);
    }
    if item.size < port.min_size || item.size > port.max_size {
        return Err(Misfit::Size);
    }
    let missing: Vec<String> = port
        .required_tags
        .iter()
        .filter(|t| !item.tags.iter().any(|i| same_tag(i, t)))
        .cloned()
        .collect();
    if !missing.is_empty() {
        return Err(Misfit::PortTags(missing));
    }
    let unmet: Vec<String> = item
        .required_tags
        .iter()
        .filter(|t| !offered.iter().any(|o| same_tag(o, t)))
        .cloned()
        .collect();
    if !unmet.is_empty() {
        return Err(Misfit::ItemTags(unmet));
    }
    Ok(())
}

fn same_tag(a: &str, b: &str) -> bool {
    a.trim_start_matches('$')
        .eq_ignore_ascii_case(b.trim_start_matches('$'))
}

/// Whether the last port of `chain` (outermost port first) has `flag`.
///
/// Client code (LIVE 4.10, `HasFlag`): flags compare without case and
/// without a leading `$`. A port without the flag inherits it from its
/// ancestors, except a flag written with `$`, which holds for its own port
/// only. So `uneditable` locks a whole subtree (thrusters), `$uneditable`
/// only its port (a gimbal whose gun can still be swapped).
pub fn has_flag(chain: &[&Port], flag: &str) -> bool {
    let Some((own, ancestors)) = chain.split_last() else {
        return false;
    };
    let matches = |f: &String| f.trim_start_matches('$').eq_ignore_ascii_case(flag);
    own.flags.iter().any(matches)
        || ancestors
            .iter()
            .any(|p| p.flags.iter().any(|f| !f.starts_with('$') && matches(f)))
}

/// Whether the last port of `chain` can be changed by the player: it is not
/// [`has_flag`] `uneditable`, and it lists at least one type.
///
/// A port that lists no types offers the player nothing to choose: on LIVE
/// 4.10's listed vehicles such ports hold fixed systems (life support,
/// relays, fuel controllers, the Skylark's lockers), erkul shows them
/// locked and the game does not let the player swap them, though some
/// carry an `editable` flag. Inferred from the data; the client's loadout
/// screen was not decompiled.
pub fn is_editable(chain: &[&Port]) -> bool {
    chain.last().is_some_and(|p| !p.types.is_empty()) && !has_flag(chain, "uneditable")
}

#[cfg(test)]
mod tests {
    use sc_extract::generated::EItemType;

    use super::*;

    fn port(flags: &[&str]) -> Port {
        Port {
            name: "p".into(),
            display_name: None,
            source: PortSource::Hull,
            types: vec![PortType {
                item_type: EItemType::WeaponGun,
                sub_types: vec![EItemSubType::Gun],
            }],
            min_size: 2,
            max_size: 3,
            flags: flags.iter().map(|s| s.to_string()).collect(),
            port_tags: vec![],
            required_tags: vec![],
            default_item: None,
        }
    }

    fn gun(size: i32, sub: EItemSubType) -> Item {
        Item {
            name_key: None,
            short_name_key: None,
            desc_key: None,
            item_type: EItemType::WeaponGun,
            item_sub_type: sub,
            size,
            grade: 1,
            tags: vec![],
            required_tags: vec![],
            mass: None,
            inventory: None,
        }
    }

    #[test]
    fn fit_rule() {
        let p = port(&[]);
        assert_eq!(fits(&p, &gun(3, EItemSubType::Gun), &[]), Ok(()));
        assert_eq!(fits(&p, &gun(4, EItemSubType::Gun), &[]), Err(Misfit::Size));
        assert_eq!(fits(&p, &gun(1, EItemSubType::Gun), &[]), Err(Misfit::Size));
        assert_eq!(
            fits(&p, &gun(2, EItemSubType::Rocket), &[]),
            Err(Misfit::Type)
        );
        // An UNDEFINED sub-type passes a sub-type list.
        assert_eq!(fits(&p, &gun(2, EItemSubType::UNDEFINED), &[]), Ok(()));
        let mut cooler = gun(2, EItemSubType::Gun);
        cooler.item_type = EItemType::Cooler;
        assert_eq!(fits(&p, &cooler, &[]), Err(Misfit::Type));
        // A port listing no types is not judged on type.
        let mut open = port(&[]);
        open.types.clear();
        assert_eq!(fits(&open, &cooler, &[]), Ok(()));

        // Tags compare without `$` and case.
        let mut locked = port(&[]);
        locked.required_tags = vec!["$Ship_Base".into()];
        let mut item = gun(2, EItemSubType::Gun);
        assert_eq!(
            fits(&locked, &item, &[]),
            Err(Misfit::PortTags(vec!["$Ship_Base".into()]))
        );
        item.tags = vec!["ship_base".into()];
        assert_eq!(fits(&locked, &item, &[]), Ok(()));
        item.required_tags = vec!["gimbalMount".into()];
        assert_eq!(
            fits(&locked, &item, &["Other"]),
            Err(Misfit::ItemTags(vec!["gimbalMount".into()]))
        );
        assert_eq!(fits(&locked, &item, &["$GimbalMount"]), Ok(()));
    }

    #[test]
    fn flag_inheritance() {
        let locked_tree = port(&["uneditable"]);
        let locked_port = port(&["$uneditable"]);
        let free = port(&["editable"]);
        // `uneditable` locks the subtree, `$uneditable` only its port.
        assert!(!is_editable(&[&locked_tree]));
        assert!(!is_editable(&[&locked_tree, &free]));
        assert!(!is_editable(&[&locked_port]));
        assert!(is_editable(&[&locked_port, &free]));
        assert!(is_editable(&[&free]));
        // Case and the `$` do not matter on the port itself.
        assert!(has_flag(&[&port(&["$UnEditable"])], "uneditable"));
        assert!(!has_flag(&[], "uneditable"));
    }

    #[test]
    fn typeless_port_is_locked() {
        // Centaurus `hardpoint_life_support`: no types, flagged `editable`.
        let mut life_support = port(&["editable"]);
        life_support.types.clear();
        assert!(!is_editable(&[&life_support]));
        assert!(!is_editable(&[]));
    }
}
