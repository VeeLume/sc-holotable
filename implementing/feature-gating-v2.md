## Feature Gating v2 — Implementation Progress

Design doc: [docs/feature-gating-v2.md](../docs/feature-gating-v2.md)

Starting: 2026-04-14

## Plan summary

Six phases from the design doc:

1. Wire data-driven closure into the classifier.
2. Partition types into observed + dormant for cfg purposes.
3. Data-driven feature cfg generation (thread cfg through emission).
4. Role-based promotion for empty polymorphic bases.
5. `Unknown` variant reshape (`{ struct_index, instance_index }`).
6. Verification.

## Status log

### Phase 1: Data-driven closure in classifier — DONE

Changes:
- New `tools/sc-generator/src/closure.rs` module holding `walk_closure` + `build_guid_lookup` (extracted from `dormancy.rs`).
- `classify_features` signature changed: `descendants` dropped, `guid_lookup: &GuidLookup` added. Step 2 (per-feature closure computation) now calls `walk_closure` instead of `compute_type_closure`.
- `compute_type_closure` removed from `features.rs` (dead code under the new design).
- `pipeline.rs` builds the GUID lookup once via `crate::closure::build_guid_lookup` and shares it with the classifier.
- `dormancy::measure_cfg_spread` now reuses the shared walker from `crate::closure` and shares the classify-time GUID lookup with the cfg-spread diagnostic.

Classifier output (808 leaf features):

| Category | Count |
|---|---:|
| Total emitted types | 6,234 |
| Single-feature | 1,303 |
| Multi-feature (Multi cfg) | 3,562 |
| Core fallback (no closure membership) | **1,369** |

The 1,369 Core-fallback types equal the dormant count from the design doc measurements exactly — confirming that Phase 1's reachable-set classifier now maps every **observed** type to a real feature and leaves only the **dormant** set in the Core fallback bucket. Phase 2 will lift this set out of Core and onto `#[cfg(feature = "dormant")]`.

### Phase 2: Dormant feature — DONE

Changes:
- New `FeatureAssignment::Dormant` variant. Classifier assigns it to types with zero closure membership.
- `compute_feature_cfg_map` emits `#[cfg(feature = "dormant")]` for every `Dormant` struct. `compute_feature_assignment_map` routes `Dormant` (and `Multi`) into the `core` module, so dormant definitions live alongside Multi ones in `core/types.rs` / `core/pools.rs`.
- `pipeline.rs` writes `dormant = ["full"]` into `sc-extract-generated/Cargo.toml` and the matching forward `dormant = ["sc-extract-generated/dormant"]` into `sc-extract/Cargo.toml`.
- Feature-count printout in `dump_feature_assignments` now breaks out Core / Dormant / Multi / Single.

### Phase 3: Cfg threading — DONE

The cfg-through-emission infrastructure existed from the earlier polymorphism attempt and is now live end-to-end under v2 because `compute_feature_cfg_map` is the single authority on per-type cfg strings. Threaded through:

- `emit_struct`: cfg attribute before struct def, `Pooled` impl, `Extract` impl.
- `emit_feature_pools`: cfg attribute on pool field in `CorePools`.
- `emit_feature_index`: cfg attribute on record sub-index field.
- `emit_record_store`: cfg attribute on extractor fn + `seed_database` dispatch entry.
- `emit_poly_enums_file`: cfg attribute on each `{Base}Ptr` variant + matching `from_ref` match arm (now takes `feature_cfgs` instead of just `feature_assignments` so Dormant/Multi variants pick up their per-type cfg).

**Important discovery mid-implementation**: data-driven closures need to be closed under *declared* Class / Pointer field targets, not just runtime-observed targets. Without this, a Multi-assigned struct `X` reached from feature F might declare a field `Handle<Y>` that F's runtime walk never populated, leaving Y out of F's closure. The compiled struct X (with F in its cfg list) then fails to find Y when F is enabled. Fixed by adding `close_under_declared_targets` as a post-pass in `closure::walk_closure` — it follows declared Class/Ptr targets transitively without touching pointer descendants.

### Phase 4: Role-based promotion — DONE

- `compute_emitted_struct_names` already includes every reachable type; the classifier now takes a `promoted: &HashSet<u32>` of indices to force into `FeatureAssignment::Core` (unconditional, no cfg) regardless of closure membership.
- `pipeline.rs` computes the promoted set as every poly base whose **full inherited** field count is zero (`emit::full_property_count == 0`). The stricter "full" check instead of "own" guarantees the promoted struct body truly is empty, so emitting it unconditionally can't cascade into un-cfg'd references to other types.
- Result: 336 empty poly bases moved out of any feature's closure into unconditional Core, matching the design-doc expectation (every `DataForgeComponentParams`-class hierarchy anchor is free to reference from anywhere).

### Phase 5: `Unknown` variant reshape — DONE

- `emit_poly_enum` now emits `Unknown { struct_index: u32, instance_index: u32 }` instead of `Unknown(u32)`.
- The `from_ref` fallback constructs `Self::Unknown { struct_index: r.struct_index, instance_index: r.instance_index }`.
- Consumers with `Datacore` can walk the raw instance via `db.instance(struct_index, instance_index)` when a subclass was cfg'd out of the compile.

### Phase 6: Verification — DONE

Classifier output (after all phases):

| Category | Count | Expected |
|---|---:|---:|
| Total emitted types | 6,234 | 6,234 |
| Core (role-based promoted) | 336 | ~336 |
| Dormant | 809 | ~1,369 |
| Multi | 3,789 | ~3,562 |
| Single | 1,300 | ~1,303 |

The dormant count fell from 1,369 to 809 because the declarative-closure fix in `walk_closure` pulls some types that would otherwise be dormant into at least one feature's closure (they're declared but not runtime-observed, and the enclosing types *are* runtime-observed).

Cold build results (Windows, debug profile, dev laptop):

| Target | Baseline (pre-v2) | v2 actual | Design target |
|---|---:|---:|---:|
| `cargo check -p sc-extract` (core only) | 26 s | **20 s** | < 60 s |
| `cargo check -p sc-extract --features full` | 1m 41s | **1m 50s** | < 3 min |
| `cargo check -p sc-extract --features dormant` | — | **2m 16s** | < 4 min |
| `cargo check -p sc-extract --features ammoparams` | — | 41 s | narrow consumer |
| `cargo test -p sc-extract` | 90 passed | **90 passed** | all pass |
| `cargo run -p sc-extract --example parse_real_p4k` (dev-opt) | 17 s | succeeds end-to-end | succeeds |

Generator runtime: 85 s (unchanged — per-feature data-driven closures add only seconds).

### Follow-up cleanup

- `emit_feature_mod` now emits `mod types;` / `mod pools;` / `mod index;` as **private** submodules with `pub use ...::*;` re-exports. This eliminates the `ambiguous_glob_reexports` collision at the top-level `pub use {feature}::*;` where every feature would otherwise export the inner module name `types` (collision with every other feature's `types`).
- Added `#![allow(non_camel_case_types)]` to generated `pools.rs` and `index.rs` so features with underscores in their names (e.g. `vendingmachine_2`) don't emit 300 lint warnings at the `Vendingmachine_2Pools` type name.
- `seed_database` now carries `#[allow(unused_variables, unused_mut, unused_assignments)]` so core-only / narrow-feature builds where every extractor is cfg-gated out don't warn on the empty function body.

### Notes

- The 566 `unused import: types::*` warnings observed earlier were a symptom of the same inner-module re-export layout — resolved by the follow-up cleanup above.
- Previous generator output in `crates/sc-extract-generated/src/generated/` was replaced in place by the v2 run; downstream consumers will need a `cargo clean` or per-feature recompile on their end.
- Running `cargo check -p sc-extract --features full` from a cold state requires ~15 GB of cfg-attribute tokenization plus serde-derive expansion — this fits within the 4 min budget but does briefly spike rustc memory. No changes to `.cargo/config.toml` or `RUST_MIN_STACK` were needed.
