# Localization conventions

How `LocaleKey`s, resolved strings, and the entity-localization cache
fit together across the workspace.

> **The rule** — stored data carries `LocaleKey`s. Strings are resolved
> at the call site through whatever `LocaleMap` is current.

This doc is the canonical reference for that rule and the patterns
that flow from it. Any domain crate (`sc-weapons`, `sc-contracts`,
future `sc-ammo`, `bulkhead`'s UI layer, …) that surfaces in-game
text should follow it.

## Why

Pre-resolved strings break under locale overlays. The motivating case
is `sc-langpatch`:

1. `Datacore::parse` reads `Data/Localization/english/global.ini` from
   the P4K and builds a `LocaleMap` against base English.
2. `sc-langpatch` then overlays a community language pack on top of
   the INI, decodes the result, and rebuilds a *new* `LocaleMap`
   reflecting the overlay.
3. Any string a workspace crate resolved at parse time is locked to
   step 1 and silently disagrees with the player-visible text from
   step 2. Player sees German names; cross-record name resolution
   (blueprint pool → entity → name) stays English.

The fix isn't "rebuild the cache against the post-overlay locale" —
that's just moving the problem. The fix is to **never pre-resolve
in the first place**. The cache stores keys; the active `LocaleMap`
stores strings. Whichever `LocaleMap` is current at the call site
wins, automatically.

Same hazard exists for any future locale-switching scenario (live
language toggle, A/B testing translations, etc.). The rule keeps
those costless.

## `LocaleKey` is raw, always

Generated code emits `LocaleKey` for every `DataType::Locale` field,
preserving the leading `@` that the DCB carries (e.g.
`"@item_NameGATS_BallisticGatling_S1"`).

- **Storage**: keep the `@`. Don't strip it on the way into a struct
  field, a `HashMap` key, a domain pool, or a serialized snapshot.
- **Resolution**: `LocaleMap::resolve(&LocaleKey)` handles the `@`
  transparently. It returns `Option<&str>` — zero-allocation on the
  hot path.
- **INI write-back**: the rare call site that needs the bare key
  (writing back to `global.ini`) calls `LocaleKey::stripped()`.
  Localized to the write site, doesn't leak into stored data.

`LocaleMap::get` requires the bare key — prefer `resolve` unless
you're already past the strip step.

## `LocalizedItemCache` — entity localization, single-walk

Every `EntityClassDefinition` carries `Components →
SAttachableComponentParams.AttachDef.Localization` with three locale
fields: `Name`, `ShortName`, `Description`. The cache walks every
entity once at parse time and stores those keys, indexed by entity
GUID.

```rust
pub struct LocalizedItem {
    pub name_key: Option<LocaleKey>,        // raw, '@' kept
    pub short_name_key: Option<LocaleKey>,
    pub desc_key: Option<LocaleKey>,
}

pub struct LocalizedItemCache {
    by_record: HashMap<Guid, LocalizedItem>,
}
```

Lives at `datacore.snapshot().localized_items`. Locale-independent —
the same cache is correct regardless of which `LocaleMap` consumers
later resolve through.

### What's not in the cache

`SCItemLocalization.display_features:
Option<Handle<SCExtendedLocalizationLevelParams>>` — the in-game
"item card extras" payload (lore blurb, marketing callouts, hero
asset paths, hover audio cues). Reasons it's excluded:

- **Sparse**: most entities don't have it.
- **Heterogeneous**: mixes localized text, asset paths, sort
  priorities, and audio handles.
- **Domain-specific**: only relevant to detail-card UIs, not the
  bulk-resolution use case the cache exists for.

Reachable via the typed model
(`SItemDefinition.localization → display_features`) when a consumer
needs it. A sibling cache can land later if a third consumer needs
it bulked up.

## Domain crates: keys on the struct, methods for resolution

Domain types that surface in-game text expose **only the keys** as
fields. Resolution is a method that takes `&LocaleMap`:

```rust
pub struct ShipWeapon {
    // …
    pub name_key: Option<LocaleKey>,
    pub desc_key: Option<LocaleKey>,
}

impl ShipWeapon {
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.name_key.as_ref().and_then(|k| locale.resolve(k))
    }
    pub fn description<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.desc_key.as_ref().and_then(|k| locale.resolve(k))
    }
}
```

Methods discover well via IDE, read naturally at the call site, and
let the type author scope which keys are exposed for direct
resolution without forcing every consumer through the raw
`LocaleKey + LocaleMap` dance.

For composite renderings (e.g. `LocationRef::display` returning
`"Stanton / microTech"`), the method takes `&LocaleMap` and
formats at call time. Don't pre-build composite strings.

## Per-domain pools — `HashMap<LocaleKey, Vec<Guid>>`

Multiple entities can share a localization key (CIG ships variants
under one display name). When a consumer needs to dedup or group by
key, the domain crate exposes a pools struct mirroring
`sc_contracts::MissionPools`:

```rust
pub struct WeaponPools {
    pub name_key: HashMap<LocaleKey, Vec<Guid>>,
    pub desc_key: HashMap<LocaleKey, Vec<Guid>>,
}

pub fn weapon_pools(weapons: &[ShipWeapon]) -> WeaponPools { … }
```

- Keys are raw `LocaleKey` (with `@`), consistent with the rest of
  the rule.
- Build is O(n) over the materialized list. Cheap; consumers that
  don't need it don't pay.
- No collision-policy enum baked into the lib — collision policy is
  a UI concern. `sc-langpatch` picks "first entity per key";
  another consumer might render every variant. The pool exposes
  both possibilities.

## Sort order

Pre-resolved strings made it cheap to sort lists by display name at
build time. Without them:

- **Build-time sort uses a locale-independent key** — record name,
  debug name, size+guid, whatever's stable across locales.
- **Localized sort happens post-resolve at the consumer** — when a
  UI renders the list, it can re-sort by `display_name(&locale)`
  cheaply (one `LocaleMap::resolve` per row, all `&str`).

This isn't a regression; it relocates the sort to where the locale
is known. Build-time sort by guid is fine for any consumer that
doesn't need alphabetical UX (registries, indices, snapshots).

## Two patterns, one result shape

Two distinct ways `LocaleKey`s appear in domain types:

**Entity-localization** — single-walk per entity through
`SAttachableComponentParams`. Lives in `LocalizedItemCache`. Used by
`sc-weapons`, future `bulkhead` UI, anything pointing at an item
entity. Generic; one cache covers every consumer.

**Domain-text resolution** — multi-level inheritance walks. The
contract title/description chain is the canonical example: four
levels (sub-contract → contract paramOverrides → handler
contractParams → template), with per-key independence (title from
level 2, description from level 4 is allowed) and template-shape
ambiguity that requires raw-svarog walks. Stays per-domain
(`sc_contracts::resolve_contract_text`); no other crate would
reuse the chain.

Both patterns produce the same shape — `Option<LocaleKey>` on the
domain struct, `display_name(&locale)`-style methods for resolution
— and don't share machinery. New domain crates with their own
inheritance shape add their own resolver alongside their domain
types; they don't try to extend `LocalizedItemCache`.

## Checklist for a new domain crate

When adding a crate that surfaces in-game text:

1. **Field**: `pub name_key: Option<LocaleKey>` (raw `@`-prefixed)
   on the domain struct. Same for `desc_key`, `short_name_key` if
   relevant.
2. **Source**: populate from `LocalizedItemCache` via the entity
   GUID, or from a domain-specific resolver if the keys come
   through inheritance.
3. **Resolution**: method on the domain type taking `&LocaleMap`.
   Returns `Option<&str>` for simple lookups, `String` for composite
   formatting.
4. **Pools**: if collisions matter, `XxxPools { *_key:
   HashMap<LocaleKey, Vec<Guid>> }` built from the materialized list.
5. **Sort**: build-time sort uses a locale-independent key; UI
   re-sorts post-resolve if needed.
6. **Don't**: store resolved `String`s on the struct, strip `@`
   on the way in, bake `LocaleMap` into `Datacore` use sites,
   pre-format composite display strings.

## References

- `crates/sc-extract/src/locale.rs` — `LocaleMap`, `LocaleKey`
- `crates/sc-extract-generated/src/locale_key.rs` — newtype
- `crates/sc-extract/src/localized_items.rs` — cache (post-restructure;
  was `display_names.rs`)
- `crates/sc-contracts/src/titles.rs` — domain-text resolver
  reference shape (post-restructure; returns keys only)
- `crates/sc-contracts/src/pools.rs` — `MissionPools` reference
  shape for per-domain pools
- `docs/feature-request-sc-weapons-langpatch.md` — driving feature
  request that prompted the rule
