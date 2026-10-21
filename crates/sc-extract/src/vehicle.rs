//! Vehicle XML: a ship's hull part tree and the item ports in it.
//!
//! The DCB holds the ship *entity* (`VehicleComponentParams`, its default
//! loadout, the ports its own components add). The hull lives outside the
//! DCB in a CryXmlB file that `VehicleComponentParams.vehicleDefinition`
//! names (`Scripts/Entities/Vehicles/Implementations/Xml/<ship>.xml`): the
//! part tree with each part's `damageMax` (the ship's hull HP is their sum)
//! and mass, and the `ItemPort` parts that are the ship's hardpoints.
//!
//! One XML serves many ship records. `VehicleComponentParams.modification`
//! names a `<Modification>` in it, applied by [`VehicleDefinition::read`]:
//! its `patchFile` (a file under `Modifications/` whose top-level elements
//! replace the base elements with the same `id`), then its `<Elem idRef
//! name value>` patches (attribute `name` set on the element whose `id` is
//! `idRef`). That order is inferred, not traced in the client. Attribute
//! names vary in case between files (`minSize` / `minsize`), so they are
//! matched without case.
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let ship = sc_extract::VehicleDefinition::read(
//!     &assets,
//!     "Scripts/Entities/Vehicles/Implementations/Xml/RSI_Constellation_Mk5.xml",
//!     "",
//! )?;
//! println!("{}: {} hull HP", ship.name, ship.hull_health());
//! for part in ship.parts.iter().filter(|p| p.port.is_some()) {
//!     println!("  port {}", part.name);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::generated::{EItemSubType, EItemType};
use crate::object_container::{self, XmlNode};
use crate::{AssetSource, Error, Result};

/// A vehicle XML with its modification applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleDefinition {
    /// `Vehicle@name`.
    pub name: String,
    /// `Vehicle@itemPortTags`: tags every port of the ship offers.
    pub item_port_tags: Vec<String>,
    /// `Vehicle@requiredItemTags`.
    pub required_item_tags: Vec<String>,
    /// Every `<Part>` in document order (pre-order), each with its parent.
    pub parts: Vec<VehiclePart>,
    /// `Damages/DamageMultipliers`: `(damageType, multiplier)`.
    pub damage_multipliers: Vec<(String, f32)>,
}

/// One `<Part>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehiclePart {
    pub name: String,
    /// `class`: `Animated`, `AnimatedJoint`, `ItemPort`, …
    pub class: String,
    /// Index into [`VehicleDefinition::parts`].
    pub parent: Option<usize>,
    /// `mass`, kg (set on the root part: the hull's mass).
    pub mass: Option<f32>,
    /// `damageMax`: this part's hit points (0 when absent).
    pub damage_max: f32,
    /// `detachRatio`.
    pub detach_ratio: Option<f32>,
    /// `detachProbability`.
    pub detach_probability: Option<f32>,
    /// Losing it destroys the vehicle: a `DamageBehavior class="Group"`
    /// fires a `DamagesGroup` whose behaviours include `Destroy` (the
    /// Gladius's `Body`, the Freelancer's nose, body and tail). Such parts
    /// can still set `detachRatio`.
    #[serde(default)]
    pub destroys_vehicle: bool,
    /// The hardpoint, for `class="ItemPort"` parts.
    pub port: Option<VehiclePort>,
}

/// A hardpoint: a part's `<ItemPort>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehiclePort {
    pub min_size: i32,
    pub max_size: i32,
    /// `flags`, split on spaces as authored (a leading `$` kept).
    pub flags: Vec<String>,
    /// `portTags`.
    pub port_tags: Vec<String>,
    /// `requiredTags`: tags an item must carry to fit.
    pub required_tags: Vec<String>,
    /// `display_name`, a localization key without the leading `@`.
    pub display_name: Option<String>,
    /// `defaultWeaponGroup`.
    pub default_weapon_group: Option<String>,
    /// `Types/Type`: the item types it takes.
    pub types: Vec<PortType>,
    /// `Pitch` limits, degrees.
    pub pitch: Option<[f32; 2]>,
    /// `Yaw` limits, degrees.
    pub yaw: Option<[f32; 2]>,
    /// `ControllerDef@controllableTags` (comma-separated): the tags a
    /// [`PriorityGroup`] names to claim the item at this port.
    #[serde(default)]
    pub controllable_tags: Vec<String>,
    /// `ControllerDef/UserDef`: what the occupant of a seat at this port
    /// controls.
    #[serde(default)]
    pub user_control: Vec<PriorityGroup>,
    /// `ControllerDef/UsableDef`: what the item at this port controls (a
    /// weapon controller its turrets; a seat, who may sit in it).
    #[serde(default)]
    pub usable_control: Vec<PriorityGroup>,
}

/// One `<PriorityGroup>` of a `ControllerDef` (the DCB's
/// `SCItemPriorityGroupParam`): how strongly it claims the items of
/// `item_type`, by default and for an item whose port carries one of the
/// tags as a controllable tag.
///
/// The chain from seat to turret runs through controller items: on the
/// Reclaimer the seat at `hardpoint_turret_console_01` claims the weapon
/// controller tagged `TurretConsole01`, which claims the turret tagged
/// `TurretConsole01_turret`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorityGroup {
    #[serde(with = "enum_serde::item_type")]
    pub item_type: EItemType,
    pub default: ControlPriority,
    pub tags: Vec<(String, ControlPriority)>,
}

impl PriorityGroup {
    /// The claim on an item of [`item_type`](Self::item_type) whose port
    /// carries `tags`: a tag's priority where one matches, else the default.
    pub fn claim(&self, tags: &[String]) -> ControlPriority {
        self.tags
            .iter()
            .filter(|(t, _)| tags.iter().any(|x| x.eq_ignore_ascii_case(t)))
            .map(|(_, p)| *p)
            .max_by_key(|p| p.rank())
            .unwrap_or(self.default)
    }
}

/// A control priority. Of two claims on one item the higher level wins: on
/// the Constellation Mk5 the pilot seat claims the flight controller at
/// 100, the copilot at 75; on the Idris 3 against 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlPriority {
    /// `no_control` (DCB 255).
    None,
    /// `observe_only` (DCB 254): on a screen, without control.
    Observe,
    /// No priority authored (vehicle XML only): a claim below every level.
    Unset,
    Level(u32),
    /// `exclusive_control` (DCB 253).
    Exclusive,
}

impl ControlPriority {
    /// A DCB `defaultPriority` / `priority`.
    pub fn from_dcb(v: u32) -> Self {
        match v {
            255 => Self::None,
            254 => Self::Observe,
            253 => Self::Exclusive,
            n => Self::Level(n),
        }
    }

    /// A vehicle XML `defaultPriority` / `Priority@value`. LIVE 4.10 ships
    /// three misspellings (`exlusive_control`, `nobserve_only`,
    /// `observe_onlyl`); what the game makes of them is not known, so they
    /// and any other unknown value claim nothing.
    pub fn from_xml(v: Option<&str>) -> Self {
        let Some(v) = v.map(str::trim) else {
            return Self::Unset;
        };
        match v {
            "no_control" => Self::None,
            "observe_only" => Self::Observe,
            "exclusive_control" => Self::Exclusive,
            _ => v.parse().map(Self::Level).unwrap_or_else(|_| {
                tracing::debug!(value = v, "unknown control priority");
                Self::None
            }),
        }
    }

    /// Its rank among claims that control; `None` for no claim and
    /// observing.
    pub fn rank(self) -> Option<u64> {
        match self {
            Self::None | Self::Observe => None,
            Self::Unset => Some(0),
            Self::Level(n) => Some(u64::from(n) + 1),
            Self::Exclusive => Some(u64::MAX),
        }
    }
}

/// One `<Type type subtypes>` of a port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortType {
    #[serde(with = "enum_serde::item_type")]
    pub item_type: EItemType,
    /// Empty when the port takes every sub-type.
    #[serde(with = "enum_serde::item_sub_types")]
    pub sub_types: Vec<EItemSubType>,
}

impl VehicleDefinition {
    /// Read `path` (with or without a `Data/` prefix) and apply the
    /// modification named `modification` (empty: none).
    pub fn read(assets: &AssetSource, path: &str, modification: &str) -> Result<Self> {
        let mut root = decode(&crate::mesh::read_data(assets, path)?)?;
        if !modification.is_empty() {
            let patch_file = modifications(&root, modification)
                .find_map(|m| attr(m, "patchFile").filter(|p| !p.is_empty()))
                .map(str::to_string);
            if let Some(patch) = patch_file {
                let dir = path.rsplit_once(['/', '\\']).map_or("", |(d, _)| d);
                let patch_path = format!("{dir}/{patch}.xml");
                // A few wreck records (LIVE 4.10: MISC_Starfarer_Wreck*) name
                // patch files the archive does not ship; keep the base.
                match crate::mesh::read_data(assets, &patch_path) {
                    Ok(bytes) => apply_patch_file(&mut root, &decode(&bytes)?),
                    Err(Error::FileNotInP4k(_)) => {
                        warn!(path, modification, patch_path, "vehicle patch file missing");
                    }
                    Err(e) => return Err(e),
                }
            }
            apply_elems(&mut root, modification);
        }
        Self::from_xml(&root)
    }

    /// Parse vehicle XML bytes (CryXmlB or plain XML) without a modification.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Self::from_xml(&decode(bytes)?)
    }

    fn from_xml(root: &XmlNode) -> Result<Self> {
        if root.tag != "Vehicle" {
            return Err(bad(&format!("root <{}>, not <Vehicle>", root.tag)));
        }
        // The damage groups that destroy the vehicle.
        let destroy: Vec<&str> = root
            .children_named("Damages")
            .flat_map(|d| d.children_named("DamagesGroups"))
            .flat_map(|g| g.children_named("DamagesGroup"))
            .filter(|g| {
                g.descendants()
                    .any(|b| b.tag == "DamageBehavior" && attr(b, "class") == Some("Destroy"))
            })
            .filter_map(|g| attr(g, "name"))
            .collect();
        let mut parts = Vec::new();
        for part_list in root.children_named("Parts") {
            collect_parts(part_list, None, &destroy, &mut parts);
        }
        let damage_multipliers = root
            .children_named("Damages")
            .flat_map(|d| d.children_named("DamageMultipliers"))
            .flat_map(|m| m.children_named("DamageMultiplier"))
            .filter_map(|m| Some((attr(m, "damageType")?.to_string(), float(m, "multiplier")?)))
            .collect();
        Ok(Self {
            name: attr(root, "name").unwrap_or_default().to_string(),
            item_port_tags: words(attr(root, "itemPortTags")),
            required_item_tags: words(attr(root, "requiredItemTags")),
            parts,
            damage_multipliers,
        })
    }

    /// The hull's hit points: every part's `damageMax`.
    pub fn hull_health(&self) -> f32 {
        self.parts.iter().map(|p| p.damage_max).sum()
    }

    /// The hull's mass, kg: every part's `mass` (the root body plus the
    /// detachable pieces that carry one); `None` when no part has one.
    pub fn hull_mass(&self) -> Option<f32> {
        self.parts
            .iter()
            .filter_map(|p| p.mass)
            .reduce(|a, b| a + b)
    }

    /// The part named `name`.
    pub fn part(&self, name: &str) -> Option<&VehiclePart> {
        self.parts
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(name))
    }

    /// The children of part `index`, in document order.
    pub fn children(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        self.parts
            .iter()
            .enumerate()
            .filter(move |(_, p)| p.parent == Some(index))
            .map(|(i, _)| i)
    }
}

fn collect_parts(
    list: &XmlNode,
    parent: Option<usize>,
    destroy: &[&str],
    out: &mut Vec<VehiclePart>,
) {
    for node in list.children_named("Part") {
        let index = out.len();
        out.push(VehiclePart::from_xml(node, parent, destroy));
        for sub in node.children_named("Parts") {
            collect_parts(sub, Some(index), destroy, out);
        }
    }
}

impl VehiclePart {
    fn from_xml(n: &XmlNode, parent: Option<usize>, destroy: &[&str]) -> Self {
        let class = attr(n, "class").unwrap_or_default().to_string();
        let port = class
            .eq_ignore_ascii_case("ItemPort")
            .then(|| n.children_named("ItemPort").next())
            .flatten()
            .map(VehiclePort::from_xml);
        Self {
            name: attr(n, "name").unwrap_or_default().to_string(),
            class,
            parent,
            mass: float(n, "mass"),
            damage_max: float(n, "damageMax").unwrap_or(0.0),
            detach_ratio: float(n, "detachRatio"),
            detach_probability: float(n, "detachProbability"),
            destroys_vehicle: n
                .children_named("DamageBehaviors")
                .flat_map(|b| b.children_named("DamageBehavior"))
                .filter(|b| attr(b, "class") == Some("Group"))
                .flat_map(|b| b.children_named("Group"))
                .any(|g| attr(g, "name").is_some_and(|name| destroy.contains(&name))),
            port,
        }
    }
}

impl VehiclePort {
    fn from_xml(n: &XmlNode) -> Self {
        let limits = |tag: &str| {
            let l = n
                .children
                .iter()
                .find(|c| c.tag.eq_ignore_ascii_case(tag))?;
            Some([float(l, "min")?, float(l, "max")?])
        };
        let types = n
            .children_named("Types")
            .flat_map(|t| t.children_named("Type"))
            .filter_map(|t| {
                let item_type = attr(t, "type").filter(|s| !s.is_empty())?;
                Some(PortType {
                    item_type: EItemType::from_dcb_str(item_type),
                    sub_types: attr(t, "subtypes")
                        .unwrap_or_default()
                        .split(',')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(EItemSubType::from_dcb_str)
                        .collect(),
                })
            })
            .collect();
        let text = |k: &str| attr(n, k).filter(|s| !s.is_empty()).map(str::to_string);
        let controller = n.children_named("ControllerDef").next();
        let groups = |def: &str| {
            controller
                .into_iter()
                .flat_map(|c| c.children_named(def))
                .flat_map(|d| d.children_named("PriorityGroups"))
                .flat_map(|g| g.children_named("PriorityGroup"))
                .filter_map(PriorityGroup::from_xml)
                .collect::<Vec<_>>()
        };
        Self {
            min_size: float(n, "minSize").map_or(0, |v| v as i32),
            max_size: float(n, "maxSize").map_or(0, |v| v as i32),
            flags: words(attr(n, "flags")),
            port_tags: words(attr(n, "portTags")),
            required_tags: words(attr(n, "requiredTags")),
            display_name: text("display_name").map(|s| s.trim_start_matches('@').to_string()),
            default_weapon_group: text("defaultWeaponGroup"),
            types,
            pitch: limits("Pitch"),
            yaw: limits("Yaw"),
            controllable_tags: controller
                .and_then(|c| attr(c, "controllableTags"))
                .map(tag_list)
                .unwrap_or_default(),
            user_control: groups("UserDef"),
            usable_control: groups("UsableDef"),
        }
    }
}

impl PriorityGroup {
    fn from_xml(n: &XmlNode) -> Option<Self> {
        let item_type = attr(n, "itemType").filter(|s| !s.is_empty())?;
        Some(Self {
            item_type: EItemType::from_dcb_str(item_type),
            default: ControlPriority::from_xml(attr(n, "defaultPriority")),
            tags: n
                .children_named("tags")
                .filter_map(|t| {
                    let tag = attr(t, "tag").filter(|s| !s.is_empty())?;
                    let value = t.children_named("Priority").next()?;
                    Some((
                        tag.to_string(),
                        ControlPriority::from_xml(attr(value, "value")),
                    ))
                })
                .collect(),
        })
    }
}

/// A comma-separated tag list (`controllableTags="copilotSeat, pilotSeat"`).
fn tag_list(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_string)
        .collect()
}

fn decode(bytes: &[u8]) -> Result<XmlNode> {
    object_container::decode(bytes)?.ok_or_else(|| bad("no XML document"))
}

/// Attribute `key`, matched without case (files differ: `minSize` / `minsize`).
fn attr<'a>(n: &'a XmlNode, key: &str) -> Option<&'a str> {
    n.attrs
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(key))
        .map(|(_, v)| v.as_str())
}

fn float(n: &XmlNode, key: &str) -> Option<f32> {
    attr(n, key)?.trim().parse().ok()
}

fn words(s: Option<&str>) -> Vec<String> {
    s.unwrap_or_default()
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

/// The `<Modification name=…>` elements named `name`.
fn modifications<'a>(root: &'a XmlNode, name: &'a str) -> impl Iterator<Item = &'a XmlNode> {
    root.children_named("Modifications")
        .flat_map(|m| m.children_named("Modification"))
        .filter(move |m| attr(m, "name") == Some(name))
}

/// Replace each base element whose `id` matches a top-level element of the
/// patch file's `<Modifications>` root with that element.
fn apply_patch_file(root: &mut XmlNode, patch: &XmlNode) {
    for replacement in &patch.children {
        if let Some(id) = attr(replacement, "id") {
            replace_by_id(root, id, replacement);
        }
    }
}

fn replace_by_id(node: &mut XmlNode, id: &str, replacement: &XmlNode) -> bool {
    for child in &mut node.children {
        if child.tag == "Modifications" {
            continue;
        }
        if attr(child, "id") == Some(id) {
            *child = replacement.clone();
            return true;
        }
        if replace_by_id(child, id, replacement) {
            return true;
        }
    }
    false
}

/// Apply the `<Elem idRef name value>` patches of the modification.
fn apply_elems(root: &mut XmlNode, modification: &str) {
    let elems: Vec<(String, String, String)> = modifications(root, modification)
        .flat_map(|m| m.descendants())
        .filter(|e| e.tag == "Elem")
        .filter_map(|e| {
            Some((
                attr(e, "idRef").filter(|s| !s.is_empty())?.to_string(),
                attr(e, "name").filter(|s| !s.is_empty())?.to_string(),
                attr(e, "value").unwrap_or_default().to_string(),
            ))
        })
        .collect();
    for (id, key, value) in &elems {
        set_by_id(root, id, key, value);
    }
}

fn set_by_id(node: &mut XmlNode, id: &str, key: &str, value: &str) {
    if node.tag == "Modifications" {
        return;
    }
    if attr(node, "id") == Some(id) {
        match node
            .attrs
            .iter_mut()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
        {
            Some(slot) => slot.1 = value.to_string(),
            None => node.attrs.push((key.to_string(), value.to_string())),
        }
    }
    for child in &mut node.children {
        set_by_id(child, id, key, value);
    }
}

fn bad(what: &str) -> Error {
    Error::VehicleXml(what.to_string())
}

/// `serde` adapters for the generated item enums (they derive nothing).
mod enum_serde {
    pub mod item_type {
        use crate::generated::EItemType;
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S: Serializer>(v: &EItemType, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemType, D::Error> {
            Ok(EItemType::from_dcb_str(&String::deserialize(d)?))
        }
    }

    pub mod item_sub_types {
        use crate::generated::EItemSubType;
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S: Serializer>(v: &[EItemSubType], s: S) -> Result<S::Ok, S::Error> {
            s.collect_seq(v.iter().map(EItemSubType::as_dcb_str))
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<EItemSubType>, D::Error> {
            Ok(Vec::<String>::deserialize(d)?
                .iter()
                .map(|s| EItemSubType::from_dcb_str(s))
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = r#"<Vehicle name="Test" itemPortTags="Base_Tag Other" requiredItemTags="">
  <Damages><DamageMultipliers>
    <DamageMultiplier damageType="melee" multiplier="0"/>
  </DamageMultipliers></Damages>
  <Parts id="xmlParts">
    <Part name="hull" class="Animated" mass="1000" id="modName">
      <Parts>
        <Part name="body" class="AnimatedJoint" damageMax="500">
          <Parts>
            <Part name="hardpoint_gun" class="ItemPort">
              <ItemPort minsize="1" maxSize="3" flags="$uneditable invisible" portTags="A B" display_name="@port_Gun" id="modGun">
                <Pitch min="-10" max="20"/>
                <Types><Type type="WeaponGun" subtypes="Gun,Rocket"/><Type type="Turret" subtypes=""/></Types>
              </ItemPort>
            </Part>
          </Parts>
        </Part>
        <Part name="wing" class="AnimatedJoint" damageMax="250"/>
      </Parts>
    </Part>
  </Parts>
  <Modifications>
    <Modification name="Heavy"><Elems>
      <Elem idRef="modName" name="mass" value="2000"/>
      <Elem idRef="modGun" name="maxSize" value="4"/>
    </Elems></Modification>
  </Modifications>
</Vehicle>"#;

    const PATCH: &str = r#"<Modifications>
  <Parts id="xmlParts">
    <Part name="variant_hull" class="Animated" mass="3000" id="modName">
      <Parts><Part name="core" class="AnimatedJoint" damageMax="900"/></Parts>
    </Part>
  </Parts>
</Modifications>"#;

    fn root() -> XmlNode {
        decode(BASE.as_bytes()).expect("xml")
    }

    #[test]
    fn parts_and_ports() {
        let def = VehicleDefinition::from_xml(&root()).expect("vehicle");
        assert_eq!(def.name, "Test");
        assert_eq!(def.item_port_tags, ["Base_Tag", "Other"]);
        assert!(def.required_item_tags.is_empty());
        assert_eq!(def.damage_multipliers, [("melee".to_string(), 0.0)]);
        assert_eq!(def.hull_health(), 750.0);
        assert_eq!(def.hull_mass(), Some(1000.0));
        let names: Vec<&str> = def.parts.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["hull", "body", "hardpoint_gun", "wing"]);
        assert_eq!(def.parts[2].parent, Some(1));
        assert_eq!(def.children(0).collect::<Vec<_>>(), [1, 3]);
        let port = def
            .part("HARDPOINT_GUN")
            .and_then(|p| p.port.as_ref())
            .expect("port");
        assert_eq!((port.min_size, port.max_size), (1, 3));
        assert_eq!(port.flags, ["$uneditable", "invisible"]);
        assert_eq!(port.port_tags, ["A", "B"]);
        assert_eq!(port.display_name.as_deref(), Some("port_Gun"));
        assert_eq!(port.pitch, Some([-10.0, 20.0]));
        assert_eq!(port.yaw, None);
        assert_eq!(port.types.len(), 2);
        assert_eq!(port.types[0].item_type, EItemType::WeaponGun);
        assert_eq!(port.types[0].sub_types.len(), 2);
        assert!(port.types[1].sub_types.is_empty());
        assert!(port.controllable_tags.is_empty() && port.user_control.is_empty());
    }

    #[test]
    fn vital_parts() {
        let xml = r#"<Vehicle name="V">
  <Damages><DamagesGroups>
    <DamagesGroup name="Destroy"><DamagesSubGroups><DamagesSubGroup><DamageBehaviors>
      <DamageBehavior class="Destroy"/>
    </DamageBehaviors></DamagesSubGroup></DamagesSubGroups></DamagesGroup>
    <DamagesGroup name="Smoke"><DamagesSubGroups><DamagesSubGroup><DamageBehaviors>
      <DamageBehavior class="Effect"/>
    </DamageBehaviors></DamagesSubGroup></DamagesSubGroups></DamagesGroup>
  </DamagesGroups></Damages>
  <Parts>
    <Part name="hull" class="Animated"><Parts>
      <Part name="body" class="AnimatedJoint" damageMax="100" detachRatio="1">
        <DamageBehaviors><DamageBehavior class="Group" damageRatioMin="1"><Group name="Destroy"/></DamageBehavior></DamageBehaviors>
      </Part>
      <Part name="wing" class="AnimatedJoint" damageMax="50">
        <DamageBehaviors><DamageBehavior class="Group" damageRatioMin="1"><Group name="Smoke"/></DamageBehavior></DamageBehaviors>
      </Part>
    </Parts></Part>
  </Parts>
</Vehicle>"#;
        let def = VehicleDefinition::parse(xml.as_bytes()).expect("vehicle");
        let vital: Vec<&str> = def
            .parts
            .iter()
            .filter(|p| p.destroys_vehicle)
            .map(|p| p.name.as_str())
            .collect();
        assert_eq!(vital, ["body"]);
    }

    #[test]
    fn controller_def() {
        let xml = r#"<ItemPort minSize="1" maxSize="1">
  <Types><Type type="Seat"/></Types>
  <ControllerDef controllableTags="copilotSeat, pilotSeat">
    <UserDef><PriorityGroups>
      <PriorityGroup itemType="FlightController" defaultPriority="75"/>
      <PriorityGroup itemType="Radar" defaultPriority="observe_only"/>
      <PriorityGroup itemType="WeaponController" defaultPriority="no_control">
        <tags tag="copilotSeat"><Priority value="exclusive_control"/></tags>
      </PriorityGroup>
      <PriorityGroup itemType="CapacitorAssignmentController"/>
    </PriorityGroups></UserDef>
    <UsableDef><PriorityGroups>
      <PriorityGroup itemType="Seat" defaultPriority="no_control"/>
    </PriorityGroups></UsableDef>
  </ControllerDef>
</ItemPort>"#;
        let port = VehiclePort::from_xml(&decode(xml.as_bytes()).expect("xml"));
        assert_eq!(port.controllable_tags, ["copilotSeat", "pilotSeat"]);
        let [flight, radar, weapons, capacitors] = &port.user_control[..] else {
            panic!("{:?}", port.user_control);
        };
        assert_eq!(flight.item_type, EItemType::FlightController);
        assert_eq!(flight.default, ControlPriority::Level(75));
        assert_eq!(radar.default.rank(), None);
        assert_eq!(capacitors.default, ControlPriority::Unset);
        assert_eq!(weapons.claim(&[]), ControlPriority::None);
        assert_eq!(
            weapons.claim(&["COPILOTSEAT".to_string()]),
            ControlPriority::Exclusive
        );
        assert_eq!(port.usable_control.len(), 1);
        assert!(ControlPriority::Exclusive.rank() > ControlPriority::Level(100).rank());
        assert!(ControlPriority::Level(0).rank() > ControlPriority::Unset.rank());
        assert_eq!(ControlPriority::from_dcb(254), ControlPriority::Observe);
        assert_eq!(
            ControlPriority::from_xml(Some("exlusive_control")),
            ControlPriority::None
        );
    }

    #[test]
    fn modification_elems_patch_attributes() {
        let mut root = root();
        apply_elems(&mut root, "Heavy");
        let def = VehicleDefinition::from_xml(&root).expect("vehicle");
        assert_eq!(def.hull_mass(), Some(2000.0));
        let port = def
            .part("hardpoint_gun")
            .and_then(|p| p.port.as_ref())
            .expect("port");
        // The patch names `maxSize`; the base wrote `maxSize`, and `minsize`
        // stays matched without case.
        assert_eq!((port.min_size, port.max_size), (1, 4));
        // An unknown modification changes nothing.
        let mut other = self::root();
        apply_elems(&mut other, "Nope");
        assert_eq!(
            VehicleDefinition::from_xml(&other)
                .expect("vehicle")
                .hull_mass(),
            Some(1000.0)
        );
    }

    #[test]
    fn patch_file_replaces_elements_by_id() {
        let mut root = root();
        apply_patch_file(&mut root, &decode(PATCH.as_bytes()).expect("patch"));
        let def = VehicleDefinition::from_xml(&root).expect("vehicle");
        let names: Vec<&str> = def.parts.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["variant_hull", "core"]);
        assert_eq!(def.hull_health(), 900.0);
        assert_eq!(def.hull_mass(), Some(3000.0));
        // The base's own elements outside the replaced one stay.
        assert_eq!(def.damage_multipliers.len(), 1);
    }
}
