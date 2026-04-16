# Feature Gating v2 — Data-Driven Scoping + Polymorphism

> **Status**: **Implemented** as of commit `bc9f899` (2026-04-15).
> Captures the considerations and decisions from the design session on
> 2026-04-14, and supersedes `docs/feature-gating.md` (v1 kept for the
> compile-time-problem framing but the architecture there is obsolete).
>
> The shipped implementation differs from this draft in one place:
> dormant types live in their **own physical `generated/dormant/` module**
> (whole-module `#[cfg(feature = "dormant")]`), rather than being mixed
> into `core/types.rs` with per-type cfgs as originally described in
> Decision 5. This keeps the core-module grep clean and lets the whole
> dormant module drop out of parsing with a single cfg decision. The
> observable behaviour (default → no dormant; `--features dormant` → all
> dormant types + their references via the `full` forward) is unchanged.
>
> Similarly, the multi-feature types live in their own physical
> `generated/multi_feature/` module — always compiled, each type carrying
> its own `#[cfg(any(...))]`. `multi-feature` is a module name, not a
> Cargo feature; only `core`, `dormant`, `full`, and the 245 leaf
> features appear in the manifest.
>
> Diagnostic flags (`--check-polymorphism`, `--analyze-poly-bases`,
> `--measure-dormancy`, `--feature-closure`, `--measure-cfg-spread`) were
> removed from the shipped generator once the design was locked in.
> Reintroduce them on a branch if future measurement is needed.

## Context

The original feature-gating design in `docs/feature-gating.md` computed
per-feature type closures by walking the DCB **schema** — `compute_type_closure`
followed declared `Class` / `StrongPointer` / `WeakPointer` targets. That
worked fine until polymorphism handling was added to `sc-extract-generated`.

When the generator started emitting tagged enums for polymorphic pointer
fields, it had to pull in every possible subclass of each declared base
so the enum variants could reference real types. This expanded the emitted
type count from ~1,935 to ~6,234 and produced two cascading failures:

1. **Feature closures over-fan-out.** Each feature's closure transitively
   included every subclass of every polymorphic base it touched. Since a
   common base like `DataForgeComponentParams` has 899 subclasses and is
   referenced from many features, every feature's closure grew to contain
   most of the polymorphic type graph. Feature assignment collapsed
   because most types ended up in most features.

2. **Cfg text exploded.** With `Multi` cfg assignments driven by the
   over-fan-out closures, a single type's `#[cfg(any(feature = "…"))]`
   attribute grew to list hundreds or thousands of features. At one point
   `core/types.rs` reached ~208 MB (the broken state during experimentation),
   and `cargo check` took 12+ minutes before failing with ~1,200 cross-feature
   reference errors.

The **root cause**: static schema closures treat every theoretically-possible
subclass as if it were present in real data, even though the DCB instance
graph only uses a fraction of them. The schema lies about what's reachable
from each feature in practice.

This document describes a replacement design grounded in **what's actually
in the DCB**, not what the schema says is possible.

## The Three API Levels

Consumer-facing layering (decided earlier in the session and unchanged):

| Level | Crate | Shape | Audience |
|---|---|---|---|
| **Raw** | `svarog-datacore` (external dep) | Untyped instance walking, zero-copy | Generator / explorers |
| **Low** | `sc-extract` + `sc-extract-generated` | Concrete typed structs, 1:1 with DCB schema | Domain crates |
| **High** | `sc-weapons`, `sc-ammo`, future `sc-vehicles` | Hand-curated, ergonomic domain APIs | App consumers |

**This document is about the low level.** The goal for that layer:
*correctly-typed concrete structs with low compile time (dev-opt builds)
AND low parse time (user runtime)*.

## Problem Statement

Design a generator output for `sc-extract-generated` that:

1. **Preserves polymorphism correctness**: every concrete subclass actually
   stored in the DCB must be accessible through a typed handle.
2. **Scales compile time reasonably**: cold `cargo check` for a narrow
   consumer (one feature + core) should stay close to the pre-polymorphism
   baseline (~26 s). Cold check for `--features full` should stay under
   ~3 minutes.
3. **Scales parse time reasonably**: end-to-end `Datacore::parse` should stay
   in the 15–20 s range against real `Data.p4k`.
4. **Keeps polymorphism "on by default"**: no opt-in feature that a
   consumer must enable to get correct subclass data. Polymorphism is a
   correctness property of the low-level API, not a configuration.
5. **Keeps the feature set Cargo-tractable**: the generated manifest should
   be readable by `cargo metadata` and reviewable in a `git diff`.

## Key Empirical Findings

All numbers come from measurement flags built into `sc-generator` during
the session (see [Tooling](#tooling) at the bottom). Dataset is the
current Star Citizen 4.x Data.p4k (~6,545 struct types, ~760 enums,
~112k records, ~305 MB DCB).

### Polymorphism reality (`--check-polymorphism`)

At every Class/Pointer field site in every record:

| Kind | Observations | Monomorphic | Polymorphic |
|---|---:|---:|---:|
| Inline `Class` | 9,150,090 | 9,150,090 (100%) | **0** |
| `StrongPointer` | 2,575,037 | 637,694 (25%) | 1,937,343 (**75%**) |
| `WeakPointer` | 950,622 | 720,562 (76%) | 230,060 (**24%**) |

**Takeaway**: inline `Class` is monomorphic by wire-format construction
(confirmed theoretically and empirically — there is no discriminator byte
in the DCB, and any polymorphism would desync the reader). Only pointer
fields can store subclasses.

### Polymorphic base analysis (`--analyze-poly-bases`)

Of 492 polymorphic bases (types that are the declared target of ≥1
pointer field with ≥1 emitted descendant):

- **336 (68.3%) have zero own fields.** Pure abstract markers.
- **p50 = 0 own fields, p95 = 6, max = 57.**
- **63 bases have 100% dormant subclasses** (test/scripting infrastructure).

Top polymorphism fan-out bases:

| Base | Own fields | Descendants |
|---|---:|---:|
| `DataForgeComponentParams` | 0 | **899** |
| `BuildingBlocks_Node` | 0 | 116 |
| `InteractionConditionParams` | 1 | 84 |
| `BuildingBlocks_WidgetBase` | 57 | 79 |
| `BuildingBlocks_DisplayWidget` | 57 | 68 |
| `ControlHintCondition` | 0 | 66 |
| `SBaseInteractionGameplayTrigger` | 0 | 65 |

**Takeaway**: the biggest polymorphism offenders are overwhelmingly empty
(zero fields). The single most fan-out base, `DataForgeComponentParams`, is
empty — promoting it to unconditional compilation costs essentially nothing.

### Subclass shape

Across 3,410 emitted polymorphic subclasses, counting each subclass's
**own** fields (not inherited):

- **15.5%** (530 subclasses) are empty — they declare no fields of
  their own, inheriting everything from the parent.
- 57.1% have 1–4 own fields.
- 14.2% have 5–9.
- 5.8% have 10–19.
- 2.6% have 20–49.
- 4.7% have 50–99.
- Median = 2, mean = 6.8, max = 116.

**Empty subclasses exist but they're never the dominant pattern in a
contentful-base hierarchy.** Specifically (measured at
`poly bases where base_own_fields > 0 AND ≥50% of subclasses are empty`):

| Hierarchy shape | Count (out of 492 bases) |
|---|---:|
| Every emitted subclass is empty | 8 |
| Base has fields AND every subclass is empty | **0** |
| Base has fields AND ≥50% of subclasses are empty | **0** |

So the "fat base + lots of marker subclasses" pattern doesn't exist in
the DCB. The 530 empty subclasses are either scattered minorities inside
hierarchies whose *other* subclasses carry data, or children of empty
bases (where the whole hierarchy is degenerate — 8 cases). A base that
has its own fields always has subclasses that *extend* it with more
fields; subclasses are never used as pure type markers on top of a
contentful base.

**Why this matters for the generator design**: we considered an
optimization that would emit "marker subclasses" as unit variants in
poly enums (instead of full structs) to save compile cost. The
measurement shows there's no such optimization target in the data.
The handful of fully-empty hierarchies (8 bases) already get covered
by the "empty poly base" promotion rule, so a separate unit-variant
path would add complexity with no additional payoff.

### Dormancy (`--measure-dormancy`)

Walking every record's instance graph and collecting observed runtime
struct indices:

| Set | Total | Observed | Dormant |
|---|---:|---:|---:|
| Full schema closure (includes descendants) | 6,234 | 4,865 (78.0%) | **1,369 (22.0%)** |
| Polymorphism-added types only (subset) | 4,299 | 3,176 (73.9%) | **1,123 (26.1%)** |
| Per-base subclasses | 3,410 | 2,575 (75.5%) | **835 (24.5%)** |

**Takeaway**: ~22% of types the generator currently emits are defined in
the schema but never actually stored in DCB data. Dropping them is pure
dead-weight elimination. The worst offenders include `MobiGlasAppDataBase`
(92% dormant), `UserVariableComputeValueBase` (70%), `SecurityClearanceTokenData`
(64%) — scripting/mission infrastructure that exists in the schema but
isn't populated in shipped data.

### Feature closure for a realistic consumer (`--feature-closure`)

Walking only records under `libs/foundry/records/entities/scitem/ships/weapons/`
(629 records) with `Reference`-following:

| Metric | Value |
|---|---:|
| Seed records | 629 |
| Records visited via `Reference` follow | 506 (beyond seeds) |
| **Observed types (closure)** | **467** |
| From monomorphic schema | 144 |
| From polymorphism (subclasses actually stored) | 323 |
| Global emitted set | 6,234 |
| **Reduction** | **93%** |

**Takeaway**: a narrow feature needs only 467 types, not 6,234. The 93%
reduction is why data-driven scoping is the primary lever.

### Cfg spread under data-driven scoping (`--measure-cfg-spread`)

Running per-feature closures for every leaf feature (808 total), inverting
into `type → features-whose-closure-contains-it`, we get the distribution
that literally *is* the size of every type's `#[cfg(any(...))]` attribute
under the data-driven model:

| Cfg width | Types | % of all |
|---|---:|---:|
| 1 (Single) | 1,303 | 26.8% |
| 2 | 593 | 12.2% |
| 3–4 | 636 | 13.1% |
| 5–9 | 398 | 8.2% |
| 10–19 | 617 | 12.7% |
| 20–49 | 523 | 10.8% |
| 50–99 | 432 | 8.9% |
| 100–199 | 296 | 6.1% |
| 200–499 | 65 | 1.3% |
| 500+ | 2 | 0.0% |

**Percentiles**: p50 = 4 · p75 = 22 · p90 = 93 · p95 = 127 · p99 = 205 ·
max = 528 · mean = 26.1.

Top widest cfgs are universal structural helpers:

| Type | Width |
|---|---:|
| `RGB` | 528 |
| `EntityClassDefinition` | 509 |
| `Vec3` | 454 |
| `Vec2` | 406 |
| `TagList` | 374 |
| `GlobalResourceMaterial` | 372 |
| `GlobalResourceGeometry` | 367 |
| `TintPaletteRef` | 366 |
| `GlobalResourceAudio` | 362 |
| `SGeometryDataParams` | 348 |

Estimated cfg text in `core/types.rs` under this scheme: **~15.7 MB**
(~3,562 Multi types × 3 emission sites × average cfg byte count).

### Forwarding-list size if we used per-type Cargo features instead of `any()` cfgs (`--measure-cfg-spread` leaf-forwarding section)

For comparison only — NOT the chosen design. If every shared type got
its own Cargo feature and leaves forwarded to it:

| Percentile | Forwarding list size per leaf |
|---|---:|
| p50 | 10 |
| p75 | 53 |
| p90 | 376 |
| p95 | 429 |
| p99 | 761 |
| max | **960** |

Top 20 leaves (`actor-actors`, `contracts`, `entities-spaceships`,
`entitlementpolicies`, `ui-buildingblocks`, `aiwavecollection`, …) would
each have 600–960 forwarding entries. Total forwarding edges across all
leaves: **~67,656**. Projected Cargo.toml size: ~70k lines.

**Takeaway**: the per-type-feature model shifts complexity from rustc
source to Cargo.toml. It's bimodal: most leaves are clean, but the top
~20 "hub" leaves explode because their closures transitively touch most
of the DCB. This is not a cfg-encoding problem — it's a property of the
hub features themselves.

## Design Decisions

### Decision 1: Data-driven scoping is the primary mechanism

**The generator computes per-feature closures by walking actual record
instance data, not by static schema reachability.**

For each leaf feature, walk the seed records whose `file_name` starts
with any of the feature's path prefixes. From each seed, follow:

- **Inline `Class`** fields → the declared target type (monomorphic, as
  confirmed by `--check-polymorphism`).
- **`StrongPointer` / `WeakPointer`** fields → the *runtime* `struct_index`
  at each value site. This captures the actual polymorphic subclass
  stored, not every schema-possible subclass.
- **`Reference`** fields → resolve the GUID to the target record's instance,
  then continue walking from there.

Types observed during the walk are in the feature's closure. Types never
observed in any record's walk are **not emitted at all** (they're dormant).

**Rationale**: The 93% reduction measured for `ships/weapons/` and the
22% global dormancy finding show that schema-based closures overcount by
a large factor. Data-driven closures directly reflect what the DCB contains,
not what it could theoretically contain. No typed-access capability is lost
because never-observed subclasses would produce `Unknown` variants even
if emitted.

### Decision 2: Feature closures "own" the types they reach, including Reference targets

**A feature closure includes everything reachable from its seed records
through Class / Pointer / Reference edges, and every type in that closure
is co-gated with the feature.**

This means a weapon feature whose records reference ammoparams records
will include `AmmoParams` types in its closure — not because ammoparams
is the natural home of that type, but because the weapon feature *needs*
`AmmoParams` available at runtime to resolve the `Reference` field.

Consequently:

- Cfg-gating is driven by the **union of features whose closure contains
  each type**. A type appearing in features `{A, B, C}` gets
  `#[cfg(any(feature = "A", feature = "B", feature = "C"))]`.
- Record extraction for cross-feature types is cfg-gated on the same
  union. Enabling `A` alone activates the `AmmoParams` extractor, populates
  `store.records.ammoparams.ammo_params`, and makes the weapon-side
  `Reference` resolve.
- The sub-index fields inside `RecordIndex.ammoparams` are also gated on
  the union, so the field exists whenever any of its users is enabled.
- The sub-index module itself (`generated/ammoparams/`) is gated on the
  union of all its fields' gates — it exists whenever any of its records
  is needed by some enabled feature.

**Rationale**: This is option (b) from the design discussion. The
alternative (Cargo feature forwarding: `ships-weapons = ["ammoparams"]`)
was rejected because it fails to express "ships-weapons needs only part
of ammoparams". Each feature specifies exactly what it uses, no more.
Everything co-gates on the union.

### Decision 3: Cfg encoding is explicit `#[cfg(any(feature = "…"))]` lists

**Every Multi type gets an inline `any(...)` cfg attribute listing all
features whose closure contains it. No per-type synthetic Cargo features,
no cluster grouping features.**

A type shared by 5 features gets a 5-entry `any(...)`; a type shared by
200 features gets a 200-entry `any(...)`. The cfg attribute is emitted
before the struct definition, the `Pooled` impl, the `Extract` impl, the
pool field in `CorePools`, the record extractor function, the
`seed_database` dispatch entry, and any sub-index field for record types.

**Rationale for choosing this over the alternatives**:

| Approach | Cfg text | Cargo.toml | Simplicity |
|---|---|---|---|
| **Explicit `any(...)` lists (chosen)** | ~15 MB | Flat, ~1k lines, small | Simplest |
| Per-type Cargo features | ~200 KB | ~70k lines, big hub forwarding | Complex |
| Cluster grouping features | ~200 KB | ~70k lines, cluster naming scheme | Most complex |

The numbers favor the cluster grouping approach on rustc-parse time,
but: (a) `cargo check`'s absolute cost drop is small (the 15 MB of cfg
text tokenization is maybe 30–60 s of the full compile budget), (b)
Cargo.toml complexity is a one-time hit on generator design but a
recurring cost on every diff review, (c) explicit cfgs are immediately
readable without cross-referencing a Cargo.toml, and (d) the cfg-parse
bottleneck is not the biggest cost anyway — type count and serde-derive
expansion dominate.

**Explicit `any(...)` wins on simplicity at an acceptable compile-time
cost. If this turns out to be a real compile-time problem in practice,
we revisit with concrete measurements and consider the per-type-feature
or cluster-grouping alternatives.**

### Decision 4: Role-based unconditional promotion for empty polymorphic bases

**The 336 polymorphic base types with zero own fields (`attribute_count == 0`)
are emitted unconditionally in the `core` module, regardless of feature
closure membership.**

These types are abstract markers whose only purpose is to anchor a
polymorphism hierarchy. They have no fields to compile, no transitive
dependencies, and no cfg text. Promoting them to unconditional is
literally free.

**Rationale**: The analysis showed these bases include the biggest
polymorphism fan-out offenders (`DataForgeComponentParams` alone has 899
subclasses and zero fields). Making the base unconditional decouples
base visibility from referring-type visibility, so every pointer field
targeting `DataForgeComponentParams` can be gated by its own (narrower)
feature set without worrying about whether the base is in scope.

### Decision 5: Dormant types are emitted under an opt-in `dormant` feature

**Types in the full schema-with-descendants closure that are never
observed during any record's instance walk are still written to the
generated source files, but every emission site carries
`#[cfg(feature = "dormant")]` so they are only compiled when the
`dormant` Cargo feature is enabled.**

Two separate phases to keep distinct:

- **Generator phase** (offline, runs when a new SC patch lands):
  walks the DCB, identifies the observed set + the dormant set, and
  writes every type from **both sets** into
  `crates/sc-extract-generated/src/generated/`. Dormant types are
  written with `#[cfg(feature = "dormant")]` attributes on every
  emission site (struct def, `Extract` impl, `Pooled` impl, pool field,
  extractor fn, sub-index field, sub-index module gate).

- **Compile phase** (each `cargo check`/`cargo build`):
  - **Default (no `dormant` feature)**: rustc's cfg resolution strips
    every dormant item from the AST before type checking. Dormant
    types contribute no compile cost — they're just inert text in the
    source files. The effective compile surface drops by ~1,369 types.
  - **`dormant` enabled**: rustc keeps the dormant items. Those
    ~1,369 types plus their transitive field references compile as
    usual.

So the generator always produces the same output regardless of what
consumers later enable. The `dormant` feature is a runtime cargo knob
on top of an always-present, unchanging set of generated files.

The `dormant` feature declares a dependency on `full`:

```toml
[features]
dormant = ["full"]
```

Enabling `dormant` therefore implicitly enables every observed feature
too, guaranteeing that any reference from a dormant type to a
non-dormant one is in scope. The user gets **every type the schema
declares**, not just the subset the current patch populates.

**Rationale**: ~22% of schema-reachable types (1,369 types) are never
populated in real DCB data — they're test scaffolding, unused scripting
infrastructure, or deprecated engine features. Emitting them by default
wastes compile time with zero runtime benefit for most consumers. But
some consumers have legitimate reasons to want the full schema:

1. **CI validation**: `cargo check --features dormant` catches generator
   bugs on the entire schema surface, not just what the current patch
   exercises.
2. **Forward-compat before regeneration**: if a new SC patch starts
   populating a previously-dormant subclass, a consumer who had
   `dormant` enabled already has the typed struct compiled and ready.
   They get one-patch-ahead coverage in exchange for a larger compile.
3. **Schema exploration tooling**: a DCB explorer built on `sc-extract`
   may want access to every type the schema declares, even the empty
   ones, for inspection purposes.
4. **Exhaustive pattern matching**: a consumer who wants to handle every
   schema-possible subclass at the match arm level (rather than relying
   on `Unknown` fallbacks) enables `dormant` to guarantee every subclass
   has a typed variant.

Default consumers (domain crates like `sc-weapons`) do **not** enable
`dormant`. They accept that rare-or-new subclasses fall through to
`Unknown { struct_index, instance_index }` at runtime, which gives them
a raw-layer escape hatch via `Datacore::db()`.

### Decision 5a: Three-level feature ladder

The combination of data-driven scoping (Decision 1) and the `dormant`
opt-in (Decision 5) gives consumers a three-step ladder of compile-cost
vs schema-coverage trade-offs:

| Level | Enable | Types compiled | Use case |
|---|---|---:|---|
| **Narrow** | One or more specific leaf features (e.g. `entities-scitem-ships-weapons`) | ~100–500 per feature closure | Domain-focused consumers who want the smallest possible compile surface. Covered by data-driven scoping. |
| **Full observed** | `full` (aliases all leaf features) | ~4,865 (every observed type) | Consumers who want typed access to everything populated in current DCB data. The baseline "enable everything real". |
| **Full schema** | `dormant` (forwards to `full` + enables dormant types) | ~6,234 (every schema-reachable type) | CI validation, forward-compat, schema-exploration tooling. |

All three levels use the same generated code — what changes is which
cfg gates are active. The ladder is strictly additive: enabling
`dormant` doesn't turn off anything, it only adds types on top of
`full`'s coverage.

**Implementation note**: Dormant types live alongside Multi types in
the core module file (or in their own dedicated file if it turns out
cleaner — to be decided in Phase 5 of the implementation plan). They
carry the single-feature cfg `#[cfg(feature = "dormant")]`, not a
multi-feature union, because by definition no observed feature's
closure reaches them. Their sub-index fields and extractor functions
share that same cfg.

### Decision 6: Representation stays eager (owned structs, pool-based)

**The current `Extract` / `Pooled` / `Builder` / `Handle<T>` architecture
is preserved.** Each emitted type gets an owned struct, materialized at
parse time into `Vec<Option<T>>` pools, referenced by `Handle<T>` indices.

**Rationale**: An earlier idea was lazy wrappers over `svarog::Instance<'a>`,
which would defer parsing until field access. This was rejected because:

- Parse time is dominated by **instance count**, not type count. Data-driven
  scoping doesn't change instance count; it only reduces which types get
  materialized. Expected parse time stays in the 15–20 s range.
- Eager owned structs give consumers thread-safe, lifetime-free handles
  that can be serialized, cached, and stored in the snapshot. Lazy
  wrappers would carry a `'db` lifetime bound to the live DCB and can't
  be serialized.
- The pool footprint shrinks proportionally to the smaller emitted type
  set, so memory cost drops under data-driven scoping naturally.

The parse-time cost of dormancy-elimination is marginal but positive:
smaller pool allocations, fewer type slots in `seed_database`'s
name-to-index lookup, less monomorphization pressure.

### Decision 7: `Unknown` variant in poly enums carries `{struct_index, instance_index}`

**The fallback variant in every generated `{Base}Ptr` enum is:**

```rust
Unknown { struct_index: u32, instance_index: u32 }
```

**not** the current single-`u32` shape, and **not** an opaque `Unknown`
with no payload.

**Rationale**: When a consumer's enabled feature set doesn't cover a
subclass that shows up at runtime (because that subclass is dormant in
this build, or was cfg-gated out, or is a newly-added type after a patch),
the consumer needs a way to drop back to the raw layer. With both
`struct_index` and `instance_index`, a consumer holding `Datacore` can do:

```rust
match weapon.components[i] {
    DataForgeComponentParamsPtr::Unknown { struct_index, instance_index } => {
        let inst = datacore.db().instance(struct_index, instance_index);
        let type_name = datacore.db().struct_name(struct_index as usize);
        // Walk inst.properties() at the raw svarog layer for whatever
        // fields the consumer needs.
    }
    // ...known variants...
}
```

This is the **raw-layer escape hatch** from the three-level API: the
typed layer has a value it doesn't know how to type, and the consumer
can always fall back to svarog's `Instance` API via `datacore.db()`.
Without both halves of the InstanceRef, consumers would have only the
struct name, not the data.

## Deferred / Open Questions

The following were considered and deliberately deferred. Each can be
revisited if the initial implementation hits problems.

### Width-based unconditional promotion

We considered unconditionally promoting types with cfg width above some
threshold (e.g. ≥ 100 features) to avoid long `any(...)` lists. Numbers:

| Threshold | Types promoted | Cfg text saved |
|---|---:|---:|
| ≥ 200 | 67 | ~1.5 MB |
| ≥ 100 | 363 | ~5 MB |
| ≥ 50 | 795 | ~9 MB |
| ≥ 20 | 1,318 | ~12 MB |

**Deferred because**: we're committing to explicit `any(...)` for
simplicity. If compile time turns out to be too slow with explicit cfgs,
width-based promotion is the cheapest mitigation and can be added as a
generator knob without changing the user-facing design.

**Gotcha to remember if we revisit**: unconditional promotion cascades.
Promoting type `X` forces every type `X`'s fields reference to also be
unconditional, transitively. The cascade's realistic size is unmeasured.

### Cluster grouping for Multi types

Instead of per-type `any(...)` cfgs, treat each distinct feature set as
a named cluster and emit one synthetic Cargo feature per cluster. We
measured 1,466 distinct Multi feature sets — 2.4× amortization over
per-type features. Cfg text would drop from ~15 MB to ~200 KB, at the
cost of a ~70k-line Cargo.toml.

**Deferred because**: cfg text in rustc source turned out to be
tolerable compared to the alternative complexity. If generator output
becomes painful to parse at cold compile time, this is the next tool
to reach for after width-based promotion.

### Per-type Cargo features (the original user proposal)

Give every shared type its own `_typename` Cargo feature, forwarded
from each leaf feature that needs it. Same trade-off as cluster grouping:
cfg text drops to near-zero, Cargo.toml balloons to ~70k lines, and the
top 20 hub leaves end up with 600–960 forwarding entries each.

**Deferred because**: same reason as cluster grouping. Simpler to commit
to explicit cfgs first and revisit only if measurements force it.

### Hub-leaf problem

20 leaf features (`actor-actors`, `contracts`, `entities-spaceships`, etc.)
have closures that transitively touch 600–960 "cluster-worth" of types.
These leaves correspond to hub records (NPC entities, ships, missions)
whose type graph spans most of the DCB.

Under any cfg encoding, a consumer enabling one of these hub leaves
compiles most of sc-extract-generated. That's a property of the data,
not the encoding. Possible future mitigations:

- **Convert hub leaves into parent features**: replace `actor-actors`
  with `actor-actors = ["actor-actors-combat", "actor-actors-dialogue", …]`
  and push the actual type ownership to the children. Requires the
  classifier to split deeper at these specific prefixes.
- **Accept them as "almost-full" features**: document that enabling
  `actor-actors` is close to enabling `full` and advise narrow consumers
  to use the narrower sibling features.

**Deferred because**: we don't need to solve it to ship v2. The 20 hub
leaves are identifiable and can be documented; consumers pick narrower
features if they want to compile less.

### Feature-closure seed-path classification

The current path-based classifier (`walk_tree` in
`tools/sc-generator/src/features.rs`) splits on record path depth to
produce 808 leaf features. Under the new data-driven closure, the split
decisions are made against `compute_base_closure` (monomorphic cost)
while the actual closures are built with the data-driven walker.

This already works in the current generator state. No changes to the
splitter itself are part of v2; we just feed its output into a different
closure computation.

**Deferred**: if the hub-leaf problem becomes acute, revisit the
splitter to produce different leaves at hub paths.

### Promotion cascade measurement

For any future revisit of width-based unconditional promotion, we'd want
to know the transitive closure of promotions. Not measured yet; the
generator flag for this would be a straightforward extension of
`--measure-cfg-spread`.

**Deferred**: only relevant if we decide to enable promotion.

## Implementation

**Completed** as of commit `bc9f899` (2026-04-15). All six phases
(data-driven closure, observed/dormant partition, cfg generation,
role-based promotion, `Unknown` variant reshape, verification) landed
in a single generator commit. See `docs/codegen.md` for the current
generator architecture.

The diagnostic flags used during the design session (`--check-polymorphism`,
`--analyze-poly-bases`, `--measure-dormancy`, `--feature-closure`,
`--measure-cfg-spread`) were removed from the shipped generator CLI.
Reintroduce them on a temporary branch if future measurement is needed.

## Outcome

Measured results after implementation (SC 4.7 branch):

| Metric | Pre-poly baseline | v2 actual |
|---|---:|---:|
| Emitted types (default) | 1,935 | 5,425 (core + multi_feature + leaf) |
| Emitted types (`--features dormant`) | — | 6,234 |
| `core/` types | 214 | 336 (empty poly bases promoted) |
| `multi_feature/` types | — | 3,789 |
| `dormant/` types | — | 809 |
| Leaf feature dirs | 227 | 245 |
| Cold `cargo check --features full` | 1m 41s | 4m 15s |
| Polymorphism correctness | no | **yes** |

Cold check is higher than projected (~2-3 min) because the polymorphism
support pulls in more types than expected. The derive-drop optimization
(commit `06e0d2e`, post-v2) brought cold full check from 4m 15s down to
~37s, making v2's absolute cost acceptable. See `docs/benchmarks.md` for
current numbers.
