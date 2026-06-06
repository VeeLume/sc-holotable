//! Runtime substitution markers — filling `~mission(Token|Fmt)` placeholders.
//!
//! Many DCB title/description strings carry `~mission(...)` markers the engine
//! substitutes at spawn time (see [`Mission::has_runtime_substitution`]). Most
//! are runtime-only — a name generator, a per-spawn location pick — and can't
//! be filled from static data. A few, though, bind to data the contract
//! already carries. This module fills those (currently the reputation-rank
//! token, from the mission's own rep gate) and renders the rest as readable
//! `[Token]` placeholders, so a static view stays honest about what is a real
//! value versus an engine-filled blank.
//!
//! Marker grammar: `~mission(Token|Fmt|Fmt2)`. The token before the first `|`
//! is the variable; each `|Fmt` is an engine render directive (`|Address`,
//! `|Last`, `|ListAll`, …). Resolution needs the [`Missions`] registries (rep
//! standings, …), so the substitution API lives on [`Missions`], mirroring the
//! locale-at-call-site convention of [`Mission::title`].

use sc_extract::LocaleMap;

use crate::{Mission, MissionVar, Missions, PrereqView};

const MARK: &str = "~mission(";

impl Missions {
    /// Resolve a mission's title with known `~mission(...)` markers substituted
    /// in (e.g. the required reputation rank). Unresolved / runtime-only
    /// markers render as `[Token]`. `None` when no title key resolves.
    pub fn title_text(&self, m: &Mission, locale: &LocaleMap) -> Option<String> {
        m.title(locale).map(|t| self.substitute(m, t, locale))
    }

    /// Resolve a mission's description with markers substituted. Same shape as
    /// [`Self::title_text`].
    pub fn description_text(&self, m: &Mission, locale: &LocaleMap) -> Option<String> {
        m.description(locale).map(|t| self.substitute(m, t, locale))
    }

    /// The marker token names that remain UNRESOLVED across this mission's
    /// title + description — the leftover `[Token]` placeholders (the name
    /// before any `|format`), deduped + sorted. Lets a consumer surface what
    /// the engine fills at runtime that a static view can't.
    pub fn unresolved_markers(&self, m: &Mission, locale: &LocaleMap) -> Vec<String> {
        let mut seen = std::collections::BTreeSet::new();
        for text in [m.title(locale), m.description(locale)]
            .into_iter()
            .flatten()
        {
            for inner in markers(text) {
                if self.resolve_marker(m, inner, locale).is_none() {
                    let name = inner.split('|').next().unwrap_or(inner).trim();
                    if !name.is_empty() {
                        seen.insert(name.to_string());
                    }
                }
            }
        }
        seen.into_iter().collect()
    }

    /// Replace every `~mission(...)` marker, resolving nested markers a resolved
    /// value may itself inject (e.g. pluralization helpers whose text contains
    /// further `~mission(...)`). Bounded so a self-referential value can't loop.
    fn substitute(&self, m: &Mission, text: &str, locale: &LocaleMap) -> String {
        let mut out = self.substitute_once(m, text, locale);
        for _ in 0..3 {
            if !out.contains(MARK) {
                break;
            }
            out = self.substitute_once(m, &out, locale);
        }
        out
    }

    /// One substitution pass: known tokens → their resolved value, unknown /
    /// runtime-only tokens → a readable `[Inner]` (the full inner string,
    /// matching the engine marker minus the `~mission`).
    fn substitute_once(&self, m: &Mission, text: &str, locale: &LocaleMap) -> String {
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(pos) = rest.find(MARK) {
            out.push_str(&rest[..pos]);
            let after = &rest[pos + MARK.len()..];
            let Some(end) = after.find(')') else {
                // Unterminated marker — emit the remainder verbatim.
                out.push_str(&rest[pos..]);
                return out;
            };
            let inner = &after[..end];
            match self.resolve_marker(m, inner, locale) {
                // Sure value — render plain.
                Some(Resolved::Exact(v)) => out.push_str(&v),
                // Best-effort value — bracket it to flag it isn't guaranteed.
                Some(Resolved::Partial(v)) => {
                    out.push('[');
                    out.push_str(&v);
                    out.push(']');
                }
                // Unresolved — bracket the raw token (the full inner string).
                None => {
                    out.push('[');
                    out.push_str(inner);
                    out.push(']');
                }
            }
            rest = &after[end + 1..];
        }
        out.push_str(rest);
        out
    }

    /// Resolve one marker's inner token (`Token` or `Token|Fmt…`) to a value,
    /// or `None` when it's runtime-only / unmodelled. Reputation rank comes from
    /// the mission's rep gate; cargo-grade / quantity tokens come from the
    /// resolved [`Mission::variables`] map. Name generators (`TargetName`,
    /// `Ship`) and the location-query expansion are still a later pass — those
    /// fall back to the `[Token]` form.
    fn resolve_marker(&self, m: &Mission, inner: &str, locale: &LocaleMap) -> Option<Resolved> {
        let name = inner.split('|').next().unwrap_or(inner).trim();
        match name {
            // The rep gate is an exact source (the required rank) — render plain.
            // Fall back to the variable map (a gated StringHash → Partial) when a
            // mission carries the token but no gate.
            "ReputationRank" => self
                .rep_rank(m, locale)
                .map(Resolved::Exact)
                .or_else(|| m.variables.get(name).and_then(|v| resolve_var(v, locale))),
            _ => resolve_var(m.variables.get(name)?, locale),
        }
    }

    /// The mission's required reputation rank — the `min_standing` (else
    /// `max_standing`) of its reputation gate, resolved to the tier's display
    /// name (e.g. "Senior"). `None` when the mission carries no rep gate.
    fn rep_rank(&self, m: &Mission, locale: &LocaleMap) -> Option<String> {
        let standing = m.prerequisites.iter().find_map(|p| match p {
            PrereqView::Reputation {
                min_standing,
                max_standing,
                ..
            } => (*min_standing).or(*max_standing),
            _ => None,
        })?;
        let s = self.rep_standings.get(&standing)?;
        locale.resolve(&s.display_name_key).map(str::to_owned)
    }
}

/// The outcome of resolving one `~mission(...)` marker token.
enum Resolved {
    /// A value we're confident is exactly correct — rendered plain (no brackets).
    Exact(String),
    /// A best-effort value the engine finalizes at spawn (a location *query
    /// scope*, or a gated / multi-option pick) — rendered in `[brackets]` to
    /// flag that it isn't a guaranteed final value.
    Partial(String),
}

/// Resolve a [`MissionVar`] to a [`Resolved`] display value. A value is
/// [`Resolved::Exact`] only when the contract pins it (a single ungated
/// option); anything the engine finalizes at spawn — a gated / multi-option
/// choice, or a location query scope — is [`Resolved::Partial`]. `None` when
/// nothing resolves to a non-empty value.
fn resolve_var(var: &MissionVar, locale: &LocaleMap) -> Option<Resolved> {
    match var {
        MissionVar::Choice(opts) => {
            let mut labels: Vec<String> = Vec::new();
            for o in opts {
                if let Some(l) = locale.resolve(&o.label_key)
                    && !l.is_empty()
                    && !labels.iter().any(|p| p == l)
                {
                    labels.push(l.to_owned());
                }
            }
            if labels.is_empty() {
                return None;
            }
            let value = labels.join(" / ");
            // Sure only when the contract pins a single ungated option.
            if opts.len() == 1 && !opts[0].gated {
                Some(Resolved::Exact(value))
            } else {
                Some(Resolved::Partial(value))
            }
        }
        MissionVar::Number(nums) => {
            let mut distinct: Vec<String> = Vec::new();
            for n in nums {
                let s = n.to_string();
                if !distinct.contains(&s) {
                    distinct.push(s);
                }
            }
            match distinct.len() {
                0 => None,
                1 => Some(Resolved::Exact(distinct.remove(0))),
                _ => Some(Resolved::Partial(distinct.join(" / "))),
            }
        }
        // The query's system(s) then setting(s) ("Nyx · Space") — always
        // Partial: it's the scope, not the concrete spawn place.
        MissionVar::Location { systems, settings } => {
            let mut parts: Vec<String> = Vec::new();
            if !systems.is_empty() {
                parts.push(systems.join(" / "));
            }
            if !settings.is_empty() {
                parts.push(settings.join(" / "));
            }
            (!parts.is_empty()).then(|| Resolved::Partial(parts.join(" · ")))
        }
    }
}

/// Iterate the inner strings of every `~mission(Inner)` marker in `text`.
fn markers(text: &str) -> impl Iterator<Item = &str> {
    let mut rest = text;
    std::iter::from_fn(move || {
        let pos = rest.find(MARK)?;
        let after = &rest[pos + MARK.len()..];
        let end = after.find(')')?;
        let inner = &after[..end];
        rest = &after[end + 1..];
        Some(inner)
    })
}
