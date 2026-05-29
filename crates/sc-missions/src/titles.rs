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
//! This module owns the inheritance walk, returning **keys only**. Per
//! the workspace localization rule
//! ([`docs/localization.md`](../../../docs/localization.md)) text
//! resolution against a [`LocaleMap`] happens at the call site through
//! [`crate::Mission::title`] / [`crate::Mission::description`].
//!
//! # Runtime substitution markers
//!
//! Many DCB strings contain `~mission(variable)` markers the game engine
//! substitutes at spawn time — e.g. `~mission(ship) clean up`. The
//! markers live in the resolved INI value, so detection is locale-aware
//! and runs at the call site too — see
//! [`crate::Mission::has_runtime_substitution`].

use sc_extract::generated::{
    CareerContract, Contract, ContractLegacy, ContractParamOverrides, ContractStringParamType,
    DataPools, Handle, SubContract,
};
use sc_extract::svarog_datacore::{DataCoreDatabase, Value};
use sc_extract::{Datacore, Guid, LocaleKey};

/// Resolved title + description **keys** for a contract, walked through
/// the four-level inheritance chain (sub-contract → contract
/// paramOverrides → handler contractParams → template).
///
/// Keys are raw `LocaleKey`s with the leading `@` preserved. Resolve
/// against a [`sc_extract::LocaleMap`] at the call site —
/// [`sc_extract::LocaleMap::resolve`] handles the `@` prefix
/// transparently.
///
/// A field is `Some(key)` whenever a key was found at any inheritance
/// level, regardless of whether the active `LocaleMap` carries a
/// translation. This lets sc-langpatch and similar consumers patch the
/// underlying INI entry even when no translation exists yet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolvedKeys {
    pub title_key: Option<LocaleKey>,
    pub description_key: Option<LocaleKey>,
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

/// Resolve title + description **keys** for a contract.
///
/// Walks the four-level inheritance chain (sub-contract → contract →
/// handler → template), returning the first match found for each of
/// title and description independently. The two fields can come from
/// different levels (e.g. contract overrides the title, template
/// provides the description).
///
/// Locale-independent — no [`sc_extract::LocaleMap`] is consulted. Pass
/// the resulting [`ResolvedKeys`] into a `LocaleMap`-aware accessor at
/// render time.
///
/// - `sub_contract`: set when the contract was walked via a
///   `SubContract` inside a parent. Its `string_param_overrides` takes
///   highest precedence.
/// - `anchor`: the parent Contract / ContractLegacy / CareerContract.
/// - `handler_params`: the owning handler's `contractParams`.
/// - `datacore`: used to walk the template record (via svarog raw).
pub fn resolve_contract_keys(
    sub_contract: Option<&SubContract>,
    anchor: ContractAnchor<'_>,
    handler_params: Option<&Handle<ContractParamOverrides>>,
    datacore: &Datacore,
) -> ResolvedKeys {
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

    ResolvedKeys {
        title_key,
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
        if param.value.is_empty() {
            continue;
        }
        match param.param {
            ContractStringParamType::Title if title.is_none() => *title = Some(param.value.clone()),
            ContractStringParamType::Description if desc.is_none() => {
                *desc = Some(param.value.clone())
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
        if param.value.is_empty() {
            continue;
        }
        match param.param {
            ContractStringParamType::Title if title.is_none() => *title = Some(param.value.clone()),
            ContractStringParamType::Description if desc.is_none() => {
                *desc = Some(param.value.clone())
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
        let raw = match param_inst.get("value") {
            Some(Value::Locale(s)) | Some(Value::String(s)) => s,
            _ => continue,
        };
        if raw.is_empty() {
            continue;
        }
        let key = LocaleKey::new(raw);
        match param_name.as_str() {
            "Title" if title.is_none() => *title = Some(key),
            "Description" if desc.is_none() => *desc = Some(key),
            _ => {}
        }
    }
}
