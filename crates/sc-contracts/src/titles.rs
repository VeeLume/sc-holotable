//! Contract title / description resolution.
//!
//! Contracts in the DCB carry their displayed text in `stringParamOverrides`
//! arrays at four possible levels:
//!
//! 1. **Sub-contract** — `SubContract.stringParamOverrides` (highest precedence)
//! 2. **Contract** — `Contract.paramOverrides.stringParamOverrides`
//! 3. **Handler** — `ContractGeneratorHandler_*.contractParams.stringParamOverrides`
//! 4. **Template** — `contract.template → record.stringParamOverrides` (fallback)
//!
//! For many common contract families (TarPits salvage, Adagio salvage, …)
//! the title + description live only on the handler or template — nothing
//! on the contract itself — and naive consumers that read only
//! `paramOverrides` see empty strings.
//!
//! This module owns the inheritance walk and the [`LocaleMap`] lookup so
//! downstream code sees a single `Option<String>` with the resolved
//! game-visible text.
//!
//! # Runtime substitution markers
//!
//! Many DCB strings contain `~mission(variable)` markers the game engine
//! substitutes at spawn time — e.g. `~mission(ship) clean up` fills
//! `~mission(ship)` from the mission's runtime state. We preserve these
//! markers verbatim; consumers can detect them and annotate accordingly
//! (the pool may be broader than any one substituted value — see the
//! TarPits salvage case in `docs/sc-contracts.md`).

use sc_extract::generated::{
    CareerContract, Contract, ContractLegacy, ContractParamOverrides, ContractStringParamType,
    DataPools, Handle, SubContract,
};
use sc_extract::svarog_datacore::{DataCoreDatabase, Value};
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap};

/// Resolved title + description text for a contract, combining the
/// inheritance chain (contract → handler → template) and a
/// [`LocaleMap`] lookup. `title` / `description` are `None` when no
/// text was found at any inheritance level OR the key was absent from
/// the locale. The paired `*_key` fields carry the raw (`@`-stripped)
/// INI key the text was resolved from, so consumers writing back to
/// `global.ini` (sc-langpatch, translation-extraction tooling) can
/// patch the original entry without re-walking the inheritance chain.
/// A `*_key` may be `Some` while the corresponding text is `None` —
/// the key existed in the DCB but was absent from the active
/// `LocaleMap`.
///
/// Text is returned verbatim — any `~mission(variable)` runtime
/// substitution markers are preserved. [`ResolvedText::has_runtime_substitution`]
/// reports whether the text has at least one such marker.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolvedText {
    pub title: Option<String>,
    pub title_key: Option<LocaleKey>,
    pub description: Option<String>,
    pub description_key: Option<LocaleKey>,
}

impl ResolvedText {
    /// True if either the title or the description contains a
    /// `~mission(...)` runtime substitution marker. Consumers that
    /// surface a ship pool alongside the title may want to mark it as
    /// non-guaranteed when this is true — the substituted value is
    /// engine-chosen at spawn time and may not align with any single
    /// entry in the pool.
    pub fn has_runtime_substitution(&self) -> bool {
        fn has(s: &Option<String>) -> bool {
            s.as_deref()
                .map(|t| t.contains("~mission("))
                .unwrap_or(false)
        }
        has(&self.title) || has(&self.description)
    }
}

/// Generic origin identifier for a contract — used to walk the right
/// `paramOverrides` handle. Exposed so the expansion pass (step 3 in
/// `docs/sc-contracts.md`) can build inheritance chains without
/// duplicating match logic.
#[derive(Clone, Copy)]
pub enum ContractAnchor<'p> {
    /// A `Contract` record with its own `paramOverrides` and an
    /// optional template reference.
    Contract(&'p Contract),
    /// A `ContractLegacy` record.
    ContractLegacy(&'p ContractLegacy),
    /// A `CareerContract` record.
    CareerContract(&'p CareerContract),
}

impl<'p> ContractAnchor<'p> {
    fn param_overrides(&self) -> Option<&'p Handle<ContractParamOverrides>> {
        match self {
            Self::Contract(c) => c.param_overrides.as_ref(),
            Self::ContractLegacy(c) => c.param_overrides.as_ref(),
            Self::CareerContract(c) => c.param_overrides.as_ref(),
        }
    }

    fn template_guid(&self) -> Option<Guid> {
        match self {
            Self::Contract(c) => c.template,
            Self::ContractLegacy(c) => c.template,
            Self::CareerContract(c) => c.template,
        }
    }
}

/// Resolve title + description for a contract.
///
/// Walks the four-level inheritance chain (sub-contract → contract →
/// handler → template), returning the first match found for each of
/// title and description independently. The two fields can come from
/// different levels (e.g. contract overrides the title, template
/// provides the description).
///
/// - `sub_contract`: set when the contract was walked via a
///   `SubContract` inside a parent. Its `string_param_overrides` takes
///   highest precedence.
/// - `anchor`: the parent Contract / ContractLegacy / CareerContract.
/// - `handler_params`: the owning handler's `contractParams`.
/// - `datacore` / `locale`: used to walk the template record (via
///   svarog raw) and to look up locale keys.
pub fn resolve_contract_text(
    sub_contract: Option<&SubContract>,
    anchor: ContractAnchor<'_>,
    handler_params: Option<&Handle<ContractParamOverrides>>,
    datacore: &Datacore,
    locale: &LocaleMap,
) -> ResolvedText {
    let pools = &datacore.records().pools;
    let db = datacore.db();

    let mut title_key: Option<LocaleKey> = None;
    let mut desc_key: Option<LocaleKey> = None;

    // Level 1 — sub-contract string params (flattened directly onto
    // SubContract rather than nested in a ParamOverrides handle).
    if let Some(sub) = sub_contract {
        pick_from_sub_contract(pools, sub, &mut title_key, &mut desc_key);
    }

    // Level 2 — contract-level paramOverrides.
    if title_key.is_none() || desc_key.is_none() {
        pick_from_param_overrides(
            pools,
            anchor.param_overrides(),
            &mut title_key,
            &mut desc_key,
        );
    }

    // Level 3 — handler's contractParams.
    if title_key.is_none() || desc_key.is_none() {
        pick_from_param_overrides(pools, handler_params, &mut title_key, &mut desc_key);
    }

    // Level 4 — template record (walked via svarog raw, since templates
    // come in multiple shapes and we don't have one typed accessor).
    if (title_key.is_none() || desc_key.is_none())
        && let Some(guid) = anchor.template_guid()
    {
        pick_from_template(db, &guid, &mut title_key, &mut desc_key);
    }

    let title = title_key
        .as_ref()
        .and_then(|k| locale.get(k.as_str()).map(String::from));
    let description = desc_key
        .as_ref()
        .and_then(|k| locale.get(k.as_str()).map(String::from));
    ResolvedText {
        title,
        title_key,
        description,
        description_key: desc_key,
    }
}

// ── Internal walkers ────────────────────────────────────────────────────────

fn pick_from_sub_contract(
    pools: &DataPools,
    sub: &SubContract,
    title: &mut Option<LocaleKey>,
    desc: &mut Option<LocaleKey>,
) {
    for h in &sub.string_param_overrides {
        let Some(param) = h.get(pools) else { continue };
        let key = param.value.stripped();
        if key.is_empty() {
            continue;
        }
        match param.param {
            ContractStringParamType::Title if title.is_none() => *title = Some(LocaleKey::new(key)),
            ContractStringParamType::Description if desc.is_none() => {
                *desc = Some(LocaleKey::new(key))
            }
            _ => {}
        }
    }
}

fn pick_from_param_overrides(
    pools: &DataPools,
    handle: Option<&Handle<ContractParamOverrides>>,
    title: &mut Option<LocaleKey>,
    desc: &mut Option<LocaleKey>,
) {
    let Some(h) = handle else { return };
    let Some(po) = h.get(pools) else { return };
    for ph in &po.string_param_overrides {
        let Some(param) = ph.get(pools) else { continue };
        let key = param.value.stripped();
        if key.is_empty() {
            continue;
        }
        match param.param {
            ContractStringParamType::Title if title.is_none() => *title = Some(LocaleKey::new(key)),
            ContractStringParamType::Description if desc.is_none() => {
                *desc = Some(LocaleKey::new(key))
            }
            _ => {}
        }
    }
}

fn pick_from_template(
    db: &DataCoreDatabase,
    template_guid: &Guid,
    title: &mut Option<LocaleKey>,
    desc: &mut Option<LocaleKey>,
) {
    let Some(record) = db.record(template_guid) else {
        return;
    };
    let inst = record.as_instance();

    // Templates appear in several shapes — some carry
    // `stringParamOverrides` directly at the top level, others nest
    // them under a `paramOverrides` field. Try both.
    walk_string_param_overrides(db, &inst, title, desc);

    if (title.is_none() || desc.is_none())
        && let Some(po) = inst.get_instance("paramOverrides")
    {
        walk_string_param_overrides(db, &po, title, desc);
    }
}

fn walk_string_param_overrides(
    db: &DataCoreDatabase,
    inst: &sc_extract::svarog_datacore::Instance<'_>,
    title: &mut Option<LocaleKey>,
    desc: &mut Option<LocaleKey>,
) {
    let Some(arr) = inst.get_array("stringParamOverrides") else {
        return;
    };
    for v in arr {
        let param_inst = match v {
            Value::Class { struct_index, data } => {
                sc_extract::svarog_datacore::Instance::from_inline_data(db, struct_index, data)
            }
            _ => continue,
        };
        let param_name = match param_inst.get("param") {
            Some(Value::Enum(e)) => e.to_string(),
            Some(Value::String(s)) => s.to_string(),
            _ => continue,
        };
        let param_key: Option<String> = match param_inst.get("value") {
            Some(Value::Locale(s)) => Some(s.strip_prefix('@').unwrap_or(s).to_string()),
            Some(Value::String(s)) => Some(s.strip_prefix('@').unwrap_or(s).to_string()),
            _ => None,
        };
        let Some(key) = param_key else { continue };
        if key.is_empty() {
            continue;
        }
        match param_name.as_str() {
            "Title" if title.is_none() => *title = Some(LocaleKey::new(key)),
            "Description" if desc.is_none() => *desc = Some(LocaleKey::new(key)),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_runtime_substitution_detects_marker() {
        let r = ResolvedText {
            title: Some("~mission(ship) clean up".into()),
            ..Default::default()
        };
        assert!(r.has_runtime_substitution());
    }

    #[test]
    fn has_runtime_substitution_false_for_plain_text() {
        let r = ResolvedText {
            title: Some("Destroy the target".into()),
            description: Some("Eliminate all hostiles.".into()),
            ..Default::default()
        };
        assert!(!r.has_runtime_substitution());
    }

    #[test]
    fn both_none_returns_false() {
        let r = ResolvedText::default();
        assert!(!r.has_runtime_substitution());
    }
}
