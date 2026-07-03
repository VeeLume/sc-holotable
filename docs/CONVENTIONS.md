# sc-holotable — API conventions

> **The contract every crate in this workspace follows.** Read this before
> adding a crate, a public type, or a public method. Most of the workspace was
> grown incrementally; without a single written contract, each addition drifts a
> little, and the drift compounds into the inconsistencies a consumer trips over.
> This file is the fixed point. When code and this file disagree, that is a bug
> in one of them — fix it, don't fork a third convention.
>
> CLAUDE.md is fast orientation; this is the normative reference. The
> [New-crate checklist](#new-crate-checklist) at the bottom is the TL;DR.

## 1. Crate taxonomy

Every library crate falls into exactly one of three roles. The role decides the
construction signature, the error policy, and whether the crate implements
[`RecordCollection`](#5-the-recordcollection-read-contract).

| Role | Crates | Builds from | Fallible? |
|---|---|---|---|
| **I/O-boundary** | `sc-discovery`, `sc-extract` | the filesystem / `Data.p4k` bytes | **yes** — owns a `thiserror` `Error` |
| **Foundational record-index** | `sc-items`, `sc-tags`, `sc-manufacturers`, `sc-resources`, `sc-locations`, `sc-gathering` | a parsed `&RecordStore` | no — infallible build, `Option` lookups |
| **Datacore-consuming domain** | `sc-crafting`, `sc-weapons`, `sc-missions`, `sc-items-armor`, `sc-items-fps-weapons`, `sc-items-ship-components`, `sc-items-ship-weapons` | a parsed `&Datacore` (+ `&Items`) | no — infallible build, `Option` lookups |

`sc-extract-generated` is workspace-internal codegen output and is exempt — it
follows the generator's own rules, not these. `sc-holotable` is the umbrella; it
re-exports and orchestrates, it doesn't define a new domain.

## 2. Naming

- **Crate prefix `sc-`.** Sub-domains hyphenate from the parent: `sc-items`,
  `sc-items-armor`.
- **A collection is the plural of its wrapper.** Wrapper `Item` → collection
  `Items`; `Location` → `Locations`; `Provider` → `Providers`. The collection is
  named after *what it holds*, never after the domain verb.
  - ❌ `Gathering` holding `Provider` (domain-named). ✅ `Providers`.
- **The wrapper, the collection, and the storage field share one noun.**
  `Mission` / `Missions` / `missions:` — not `Mission` / `Missions` /
  `contracts:`.
- **No `RawFoo` / `Foo` split.** The curated type takes the plain name; the raw
  layer is the generated DCB type or the svarog escape hatch. (See the
  wrapper-over-generated-type pattern in `docs/sc-weapons.md`.)

## 3. Construction

Three constructors, by role. The **verb is always `build`**; only the input
differs, and the input is fixed by the crate's role (§1).

```rust
// Foundational record-index crate:
impl Items {
    pub fn new() -> Self;                       // empty, == Default
    pub fn build(store: &RecordStore) -> Self;  // the one true builder
}

// Datacore-consuming domain crate:
impl Armor {
    pub fn build(dc: &Datacore, items: &Items) -> Self;
}

// I/O-boundary crate: build is fallible and named for what it does
impl Datacore {
    pub fn parse(/* … */) -> Result<Self>;
}
```

Rules:

- **`build` never takes both `&RecordStore` and `&Datacore`.** A `Datacore`
  hands out its `RecordStore` via `datacore.records()`; pick the narrower input
  your crate actually needs. Foundational crates take `&RecordStore`.
- **Domain crates take their dependencies explicitly, they don't rebuild them.**
  A domain crate that needs `Items` takes `&Items` as a parameter — it does not
  call `Items::build` internally. Build foundations once, share by reference.
  *(Exception: `sc-missions::Missions::build(&Datacore)` is a heavyweight,
  self-contained pipeline that builds its own `Items` + `Tags`. Forcing every
  standalone caller to pre-build those is worse ergonomics than the sharing is
  worth — it stays self-contained by design.)*
- **Every infallible collection has `new() -> Self` == `Default`.** "Empty
  collection" is `X::new()` everywhere, never `X::default()` at the call site.
- **A foundational crate also exposes a `XBuilder: RecordVisitor`** so it can ride
  the bundled walk (`BundledWalk` in `sc-extract`). `build` and the builder share
  one private `fn project_one(...)` so the two paths can't diverge. The umbrella's
  `build_foundations` fuses every builder into a single `all_records` pass.
  *(Outstanding: `sc-gathering` has no `RecordVisitor` impl — it builds post-walk
  because it needs `RecordPaths` for mode classification. A `ProvidersBuilder` to
  join the fused pass is open follow-up.)*

## 4. Lookup & accessors

| Concept | Signature | Notes |
|---|---|---|
| primary lookup | `fn get(&self, guid: &Guid) -> Option<&Item>` | **always `&Guid`, never by value.** |
| existence | `fn contains(&self, guid: &Guid) -> bool` | provided by the trait |
| size | `fn len(&self) -> usize` / `fn is_empty(&self) -> bool` | O(1) |
| iterate pairs | `fn iter(&self) -> impl Iterator<Item = (&Guid, &Item)>` | the canonical walk |
| iterate values | `fn values(&self) -> impl Iterator<Item = &Item>` | provided by the trait |
| secondary key | `fn by_<key>(&self, k: …) -> Option<&Item>` | single hit → `Option<&Item>` |
| multi-valued key | `fn by_<key>(&self, k: …) -> &[Guid]` | many hits → slice of GUIDs |
| class-CRC | `fn by_crc(&self, crc: u32) -> Option<&Item>` + `guid_by_crc` | only where CRC resolution is meaningful |

The first five rows are the [`RecordCollection`](#5-the-recordcollection-read-contract)
surface — they live on the trait, not as inherent methods, so bring the trait into
scope to call them (§5). The `by_*` / `by_crc` rows are inherent, collection-specific.

- **`get` takes `&Guid`.** (Every keyed collection now does, including
  `Missions`, `Blueprints`, and their sub-registries.)
- **"iterate everything" is `iter()`, returning `(&Guid, &Item)`.** Never `all()`,
  never a domain verb like `providers()`. A crate whose wrapper carries its own
  GUID still exposes `iter()` pairs (the key is mapped off the wrapper's GUID
  field); consumers that want values use `values()`.
- **A collection never exposes its backing `Vec`/`HashMap` as a public field.**
  Reads go through methods. *(Exception: `sc-weapons::Weapons` is a multi-family
  result bundle — three heterogeneous `Vec`s (`ships`/`fps`/`missiles`) plus
  `pools` — not a single GUID-keyed collection, so `RecordCollection` doesn't
  apply and the public fields stand. `sc-weapons` is legacy, superseded by the
  `sc-items-*` sheets for product stats.)*

## 5. The `RecordCollection` read contract

Every GUID-keyed collection implements `sc_extract::RecordCollection`. The trait
*is* the contract from §4 — implementing it makes a wrong `get` signature or a
missing `iter` a compile error, not a code-review nit.

```rust
pub trait RecordCollection {
    /// The wrapper value stored per record GUID.
    type Item;

    /// Look up the entry for a record GUID.
    fn get(&self, guid: &Guid) -> Option<&Self::Item>;

    /// Number of entries. O(1).
    fn len(&self) -> usize;

    /// Iterate `(guid, item)` pairs. Order is unspecified.
    fn iter(&self) -> impl Iterator<Item = (&Guid, &Self::Item)> + '_;

    // ── provided ───────────────────────────────────────────────
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn contains(&self, guid: &Guid) -> bool { self.get(guid).is_some() }
    fn values(&self) -> impl Iterator<Item = &Self::Item> + '_ {
        self.iter().map(|(_, v)| v)
    }
    fn guids(&self) -> impl Iterator<Item = &Guid> + '_ {
        self.iter().map(|(k, _)| k)
    }
}
```

Rules:

- **The canonical surface is trait-only — no inherent duplicates.** `get` / `iter`
  / `len` / `is_empty` / `contains` / `values` / `guids` live *only* on the trait
  impl; there are no inherent copies to drift out of sync (that duplication is
  exactly the smell this trait exists to remove). Consumers bring them into scope
  with `use <crate>::RecordCollection;` — every collection crate re-exports the
  trait next to its collection — or `use sc_extract::RecordCollection;`. This is
  the same idiom as `std::io::Write`; the umbrella's `sc_holotable::prelude` pulls
  it in for batteries-included consumers.
- **Only domain-specific methods stay inherent** — secondary-key and CRC lookups
  (`by_crc`, `by_name`, `by_code`, `refined_version_of`, `in_category`,
  `for_crafted_entity`, …) and the constructors (`new` / `build`). The trait covers
  the universal GUID surface; everything else is the collection's own.
- **`RecordCollection` is a read contract, not a construction one** — construction
  stays in inherent `build` (§3) because the input type is role-specific.

## 6. Errors

- **I/O-boundary crates own a `thiserror` enum** named `Error`, with
  `#[non_exhaustive]`, plus `pub type Result<T> = std::result::Result<T, Error>`.
  Variants describe the failure, not the line that raised it.
- **In-memory crates (foundational + domain) are infallible:** `build` returns
  `Self`, lookups return `Option`. They do **not** define an error type. A missing
  record is `None`, not an `Err`.
- **Don't define an error type you never return.** (The dead
  `sc-weapons::WeaponError` — defined but never returned — has been removed.)

## 7. Typed-value discipline

These keep the surface more type-safe than a raw schema reflection. They are not
optional.

- **Localization references are `LocaleKey`, never `String`.** Any field that
  holds a `@…` localization key is `LocaleKey` or `Option<LocaleKey>`. (Every
  curated wrapper now follows this, including
  `sc-manufacturers::Manufacturer::{name_key, description_key}`.)
- **Enum-choice fields are the generated Rust enum**, not a stringly value. Round-
  trip unknown values through `Unrecognized(String)` (never `Unknown`).
- **Cross-record links are `Option<CigGuid>`;** re-enter the typed surface with
  `Datacore::resolve::<T>(&guid)`, not raw `db().record()` field pokes.
- **No string-matching where a typed or data-derived alternative exists.** Substring
  checks on entity names / `debug_name`s are a symptom of missing structure. Use
  typed enums, tag references, and the reachable instance graph. If unavoidable,
  scope it tightly, comment *why* no typed path works, and alarm on pool-size
  regressions. (Cross-record joins resolve via typed GUID/`Reference`, never by
  name token — including record-name tokens.)

## 8. Dependencies, features, serde

- **`thiserror` for errors; `anyhow` is banned in lib code.**
- **`serde` with `derive`, always-on (no feature flag).** Curated wrapper types
  derive `Serialize`/`Deserialize`; the **generated** types derive nothing (going
  through serde there hits the monomorphization cliff — a crate that needs to
  serialize a generated enum writes a small `as_dcb_str`/`from_dcb_str` adapter,
  as `sc-items` does).
- **`specta` behind a `specta` feature flag**, only where a Tauri consumer needs
  typed bindings.
- **`tracing`, never `log`/`println!`.** `debug!` per-record, `info!` phase
  transitions, `warn!` recoverable, `error!` terminal.
- **A crate enables exactly the `sc-extract` feature closure it needs** and owns
  that closure (e.g. `sc-items` enables `item`, `sc-locations` enables `starmap`).
- **`tui` is a per-crate feature** gating an optional `tui` module. Keep it a
  single `tui.rs` or a `tui/` dir consistently — prefer `tui/` once it's more than
  one screen.

## Convergence status (2026-06-20)

This contract was applied to the existing workspace in a one-pass consistency
sweep. **Done:** `RecordCollection` defined in `sc-extract` and implemented by
every keyed collection (`Items`, `Tags`, `Manufacturers`, `Resources`,
`Providers`, `Locations`, `Blueprints`, `Missions`, and the four `sc-items-*`
sheets); `get` is `&Guid` everywhere; `all()`/`providers()` value-iterators
renamed to `values()` with `iter()` now yielding `(&Guid, &Item)` pairs;
`Gathering` → `Providers`; dead `sc-weapons::WeaponError` deleted;
`Manufacturer` name/description keys are `LocaleKey`. The canonical accessor
surface is **trait-only** — the inherent `get`/`iter`/`len`/`is_empty`/`values`
copies were removed, so each method has a single definition; every collection
crate re-exports `RecordCollection`, and `sc_holotable::prelude` includes it.

**Intentional exceptions** (documented above, not bugs): `Missions::build` stays
self-contained (§3); `Weapons` keeps its multi-family public bundle (§4).

**Outstanding follow-ups:** give `sc-gathering` a `ProvidersBuilder: RecordVisitor`
(§3); rename `Missions`' internal `contracts` field to `missions` (§2) — deferred
because it touches the public `index.contracts` accessor and the consumer guide.

## New-crate checklist

When you scaffold `sc-foo`:

1. **Pick the role** (§1). That fixes the next three answers.
2. **`Foo` wrapper, `Foos` collection, `foos:` field** — one noun (§2).
3. **`Foos::new()` + `Foos::build(<role input>)`** (§3). Foundational? add
   `FoosBuilder: RecordVisitor` and wire it into `build_foundations`.
4. **`impl RecordCollection for Foos`** — trait-only, no inherent
   `get`/`iter`/`len` copies (§4, §5). `get` takes `&Guid`.
5. **Errors:** I/O-boundary → `Error`/`Result` + thiserror + `#[non_exhaustive]`;
   otherwise none (§6).
6. **`LocaleKey` for every localization key; typed enums for choices** (§7).
7. **serde always-on; `tracing` for logs; own your `sc-extract` feature closure**
   (§8).
8. **Add the crate to `status.md`'s status table** (the single source for crate
   state — don't duplicate it into CLAUDE.md).
