//! Mining modules and gadgets — a T1 data crate.
//!
//! Covers **mining modules** (`EItemType::MiningModifier`, slotted into a
//! mining laser) and **mining gadgets** (`EItemType::Gadget` items that carry
//! a rock modifier and are attached to the rock). It also owns the shared
//! [`MiningModifiers`] / [`FloatModifier`] types: mining lasers carry the same
//! modifier block, and `sc-items-ship-weapons` reuses these types for it.
//!
//! # Where the values come from
//!
//! | value | path |
//! |---|---|
//! | modifiers | `EntityComponentAttachableModifierParams.modifiers[]` → `ItemMiningModifierParams` (modules) / `ItemMineableRockModifierParams` (gadgets) `.MiningLaserModifier` |
//! | damage multiplier | same list → `ItemWeaponModifiersParams` (one per laser fire action, `fireActionIndex`) `.weaponModifier.weaponStats.damageMultiplier` (modules) |
//! | inert filter | same list → `MiningFilterItemModifierParams.filterParams.filterModifier` (modules) |
//! | active duration | `ItemMiningModifierParams.modifierLifetime` (`ItemModifierTimedLife.lifetime`, s) |
//! | activation, uses | `EntityComponentAttachableModifierParams.activationMethod` / `.charges` |
//!
//! # How the client applies the modifiers
//!
//! Read from the client (LIVE 4.10): every laser, module and gadget acting on
//! a rock contributes one entry; multiplicative values combine as a product of
//! `1 + value/100`, additive ones as `(sum + value) / 100` per entry. See
//! [`FloatModifier`]. Only one item in the 4.10 data carries an additive
//! value (the ROC-DS vehicle module).
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
//! use sc_items::Items;
//! use sc_items_mining::{MiningGadgets, MiningModules};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let items = Items::build(datacore.records());
//! let modules = MiningModules::build(&datacore, &items);
//! for module in modules.values() {
//!     let window = module.modifiers.optimal_charge_window_size.map(|m| m.factor());
//!     println!("x{:.2} power, window factor {window:?}", module.damage_multiplier(0));
//! }
//! let gadgets = MiningGadgets::build(&datacore, &items);
//! println!("{} modules, {} gadgets", modules.len(), gadgets.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::HashMap;

use sc_extract::generated::EItemType;
use sc_extract::{DataCoreDatabase, Datacore, Guid, Instance, LocaleKey, Value};
use sc_items::Items;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collections.
pub use sc_extract::RecordCollection;

// ── shared modifier types ───────────────────────────────────────────────────

/// How a [`FloatModifier`] combines with others on the same rock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifierKind {
    /// `FloatModifierMultiplicative`: contributes a factor `1 + value/100`.
    Multiplicative,
    /// `FloatModifierAdditive`: contributes to the additive term. The client
    /// accumulates entries as `add = (add + value) / 100`, one entry at a time
    /// (LIVE 4.10), so with a single entry the term is `value / 100`.
    Additive,
}

/// One modifier value as the DCB stores it, in percent.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FloatModifier {
    pub kind: ModifierKind,
    /// Percent, e.g. `-35.0` for "-35 %".
    pub value: f32,
    /// `showInUI` — whether the game lists it on the item.
    pub show_in_ui: bool,
}

impl FloatModifier {
    /// The multiplicative factor this modifier contributes: `1 + value/100`
    /// for [`ModifierKind::Multiplicative`], `1.0` for additive ones.
    pub fn factor(&self) -> f32 {
        match self.kind {
            ModifierKind::Multiplicative => 1.0 + self.value / 100.0,
            ModifierKind::Additive => 1.0,
        }
    }

    /// Read a `FloatModifierMultiplicative` / `FloatModifierAdditive` instance;
    /// `None` for any other type.
    fn read(inst: &Instance<'_>) -> Option<Self> {
        let kind = match inst.type_name()? {
            "FloatModifierMultiplicative" => ModifierKind::Multiplicative,
            "FloatModifierAdditive" => ModifierKind::Additive,
            _ => return None,
        };
        Some(Self {
            kind,
            value: inst.get_f32("value").unwrap_or_default(),
            show_in_ui: inst.get_bool("showInUI").unwrap_or_default(),
        })
    }
}

/// The DCB `MiningLaserModifiers` block, shared by mining lasers, modules and
/// gadgets. Each field is `None` when the item leaves that stat alone.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct MiningModifiers {
    /// `laserInstability` — scales the rock's instability.
    pub laser_instability: Option<FloatModifier>,
    /// `optimalChargeWindowSizeModifier` — scales the optimal window's
    /// half-width around its centre.
    pub optimal_charge_window_size: Option<FloatModifier>,
    /// `resistanceModifier` — scales the rock's resistance.
    pub resistance: Option<FloatModifier>,
    /// `shatterdamageModifier`.
    pub shatter_damage: Option<FloatModifier>,
    /// `clusterFactorModifier`.
    pub cluster_factor: Option<FloatModifier>,
    /// `optimalChargeWindowRateModifier` — scales how fast the
    /// controlled-breaking pool fills inside the window.
    pub optimal_charge_window_rate: Option<FloatModifier>,
    /// `catastrophicChargeWindowRateModifier` — scales the fill rates above
    /// the window.
    pub catastrophic_charge_window_rate: Option<FloatModifier>,
    /// `isOptimalRateGood`.
    pub is_optimal_rate_good: bool,
}

impl MiningModifiers {
    /// Read a `MiningLaserModifiers` instance (raw layer). Used by this crate
    /// and by `sc-items-ship-weapons` for the laser's own block.
    pub fn read(inst: &Instance<'_>) -> Self {
        let m = |field: &str| {
            inst.get_instance(field)
                .as_ref()
                .and_then(FloatModifier::read)
        };
        Self {
            laser_instability: m("laserInstability"),
            optimal_charge_window_size: m("optimalChargeWindowSizeModifier"),
            resistance: m("resistanceModifier"),
            shatter_damage: m("shatterdamageModifier"),
            cluster_factor: m("clusterFactorModifier"),
            optimal_charge_window_rate: m("optimalChargeWindowRateModifier"),
            catastrophic_charge_window_rate: m("catastrophicChargeWindowRateModifier"),
            is_optimal_rate_good: inst.get_bool("isOptimalRateGood").unwrap_or_default(),
        }
    }

    /// Read the optional `MiningFilterParams.filterModifier` ("inert material
    /// level") a laser carries next to its modifier block.
    pub fn read_filter(filter_params: &Instance<'_>) -> Option<FloatModifier> {
        filter_params
            .get_instance("filterModifier")
            .as_ref()
            .and_then(FloatModifier::read)
    }
}

// ── modules ─────────────────────────────────────────────────────────────────

/// How a module is used.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Activation {
    /// `ActivateOnAttach` — always on while slotted.
    Passive,
    /// `ActivateOnDemand` — triggered by the player for `duration_s`, at most
    /// `uses` times.
    Active {
        duration_s: Option<f32>,
        uses: Option<i32>,
    },
}

/// One mining module. All values are base (unmodified by crafting).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiningModule {
    pub guid: Guid,
    pub name_key: Option<LocaleKey>,
    pub size: i32,
    pub grade: i32,
    pub activation: Activation,
    /// Power multipliers per laser fire action ("Mining Laser Power" on the
    /// item), from `SWeaponStats.damageMultiplier`. A module can scale the
    /// laser's beams differently (the Surge: x1.5 on fire action 0, x1.0 on
    /// fire action 1); see [`MiningModule::damage_multiplier`].
    pub beam_multipliers: Vec<BeamMultiplier>,
    pub modifiers: MiningModifiers,
    /// `MiningFilterParams.filterModifier` ("inert material level").
    pub inert_filter: Option<FloatModifier>,
}

/// A module's power multiplier for one of the laser's fire actions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BeamMultiplier {
    /// Index into the laser's `fireActions` (`fireActionIndex`).
    pub fire_action: i32,
    pub damage_multiplier: f32,
}

impl MiningModule {
    /// The power multiplier this module applies to the laser's fire action
    /// `fire_action`; `1.0` when it leaves that beam alone.
    pub fn damage_multiplier(&self, fire_action: i32) -> f32 {
        self.beam_multipliers
            .iter()
            .find(|b| b.fire_action == fire_action)
            .map_or(1.0, |b| b.damage_multiplier)
    }
}

/// Every mining module, keyed by `EntityClassDefinition` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MiningModules {
    by_guid: HashMap<Guid, MiningModule>,
}

impl MiningModules {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Keep the `MiningModifier` items of the [`Items`] envelope and read each
    /// one's modifier component (raw `db()` layer).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let db = datacore.db();
        let mut by_guid = HashMap::new();
        for (&guid, item) in items.iter() {
            if item.item_type != EItemType::MiningModifier {
                continue;
            }
            let Some(rec) = db.record(&guid) else {
                continue;
            };
            let entity = rec.as_instance();
            let Some(comp) = find_component(db, &entity, "EntityComponentAttachableModifierParams")
            else {
                continue;
            };
            let mut modifiers = MiningModifiers::default();
            let mut duration_s = None;
            let mut beam_multipliers = Vec::new();
            let mut inert_filter = None;
            for m in array(db, &comp, "modifiers") {
                match m.type_name() {
                    Some("ItemMiningModifierParams") => {
                        if let Some(block) = m.get_instance("MiningLaserModifier") {
                            modifiers = MiningModifiers::read(&block);
                        }
                        duration_s = m
                            .get_instance("modifierLifetime")
                            .filter(|l| l.type_name() == Some("ItemModifierTimedLife"))
                            .and_then(|l| l.get_f32("lifetime"));
                    }
                    Some("ItemWeaponModifiersParams") => {
                        if let Some(d) = m
                            .get_instance("weaponModifier")
                            .and_then(|w| w.get_instance("weaponStats"))
                            .and_then(|s| s.get_f32("damageMultiplier"))
                        {
                            beam_multipliers.push(BeamMultiplier {
                                fire_action: m.get_i32("fireActionIndex").unwrap_or_default(),
                                damage_multiplier: d,
                            });
                        }
                    }
                    Some("MiningFilterItemModifierParams") => {
                        inert_filter = m
                            .get_instance("filterParams")
                            .as_ref()
                            .and_then(MiningModifiers::read_filter);
                    }
                    _ => {}
                }
            }
            beam_multipliers.sort_by_key(|b| b.fire_action);
            let activation = match comp.get("activationMethod").and_then(|v| v.as_str()) {
                Some("ActivateOnDemand") => Activation::Active {
                    duration_s,
                    uses: comp.get_i32("charges"),
                },
                _ => Activation::Passive,
            };
            debug!(%guid, ?activation, ?beam_multipliers, "mining module");
            by_guid.insert(
                guid,
                MiningModule {
                    guid,
                    name_key: item.name_key.clone(),
                    size: item.size,
                    grade: item.grade,
                    activation,
                    beam_multipliers,
                    modifiers,
                    inert_filter,
                },
            );
        }
        Self { by_guid }
    }
}

impl sc_extract::RecordCollection for MiningModules {
    type Item = MiningModule;

    fn get(&self, guid: &Guid) -> Option<&MiningModule> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &MiningModule)> + '_ {
        self.by_guid.iter()
    }
}

// ── gadgets ─────────────────────────────────────────────────────────────────

/// One mining gadget: an item attached to the rock that modifies it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiningGadget {
    pub guid: Guid,
    pub name_key: Option<LocaleKey>,
    pub size: i32,
    pub grade: i32,
    pub modifiers: MiningModifiers,
}

/// Every mining gadget, keyed by `EntityClassDefinition` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MiningGadgets {
    by_guid: HashMap<Guid, MiningGadget>,
}

impl MiningGadgets {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Keep the `Gadget` items whose modifier list carries an
    /// `ItemMineableRockModifierParams` (other gadgets are not mining gear).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let db = datacore.db();
        let mut by_guid = HashMap::new();
        for (&guid, item) in items.iter() {
            if item.item_type != EItemType::Gadget {
                continue;
            }
            let Some(rec) = db.record(&guid) else {
                continue;
            };
            let entity = rec.as_instance();
            let Some(comp) = find_component(db, &entity, "EntityComponentAttachableModifierParams")
            else {
                continue;
            };
            let Some(block) = array(db, &comp, "modifiers")
                .into_iter()
                .find(|m| m.type_name() == Some("ItemMineableRockModifierParams"))
                .and_then(|m| m.get_instance("MiningLaserModifier"))
            else {
                continue;
            };
            debug!(%guid, "mining gadget");
            by_guid.insert(
                guid,
                MiningGadget {
                    guid,
                    name_key: item.name_key.clone(),
                    size: item.size,
                    grade: item.grade,
                    modifiers: MiningModifiers::read(&block),
                },
            );
        }
        Self { by_guid }
    }
}

impl sc_extract::RecordCollection for MiningGadgets {
    type Item = MiningGadget;

    fn get(&self, guid: &Guid) -> Option<&MiningGadget> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &MiningGadget)> + '_ {
        self.by_guid.iter()
    }
}

// ── extraction helpers (raw db layer) ───────────────────────────────────────

fn find_component<'a>(
    db: &'a DataCoreDatabase,
    entity: &Instance<'a>,
    type_name: &str,
) -> Option<Instance<'a>> {
    array(db, entity, "Components")
        .into_iter()
        .find(|c| c.type_name() == Some(type_name))
}

fn array<'a>(db: &'a DataCoreDatabase, inst: &Instance<'a>, field: &str) -> Vec<Instance<'a>> {
    inst.get_array(field)
        .map(|a| a.filter_map(|v| value_to_instance(db, &v)).collect())
        .unwrap_or_default()
}

fn value_to_instance<'a>(db: &'a DataCoreDatabase, v: &Value<'a>) -> Option<Instance<'a>> {
    match v {
        Value::Class { struct_index, data } => {
            Some(Instance::from_inline_data(db, *struct_index, data))
        }
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplicative_factor() {
        let m = FloatModifier {
            kind: ModifierKind::Multiplicative,
            value: -35.0,
            show_in_ui: true,
        };
        assert!((m.factor() - 0.65).abs() < 1e-6);
    }

    #[test]
    fn additive_has_no_factor() {
        let m = FloatModifier {
            kind: ModifierKind::Additive,
            value: 20.0,
            show_in_ui: true,
        };
        assert_eq!(m.factor(), 1.0);
    }
}
