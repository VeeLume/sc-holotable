# sc-holotable

Shared Rust utility workspace for Star Citizen tooling: install discovery, DataCore (`Game2.dcb`) extraction through generated typed bindings, localization, object-container reading, and cooked domain indices that several consumer apps share instead of re-implementing.

## Where documentation lives

**There is no `docs/` directory, on purpose.** Long prose docs went unread and went stale. Every kind of knowledge has exactly one home:

| Kind | Home |
|---|---|
| Project state, decisions + rationale, open work, known issues | **Firefly** (MCP server `firefly`), project slug `sc-holotable`. `describe` once, then `list` per kind with `{"project_id": …}`. Update the existing record (with its version); don't add a parallel one. |
| Game-data findings (where data lives, verified field paths, reference numbers) | Firefly **notes** — plus a committed probe under `crates/*/examples/` that re-proves the claim |
| API contracts, usage, behaviour, format notes | **rustdoc**, next to the code, with `no_run` doctests so drift is a compile error |
| Release history of the public surface | `CHANGELOG.md` |
| Orientation + rules for working in this repo | this file |
| Public overview | `README.md` |

Do not create markdown docs. Do not write derivable facts anywhere by hand (type counts, test counts, crate tables with status, consumer pins) — they are the first thing to rot. If a doc comment needs more than a screen, it is two comments or it is code.

## Design principles

Load-bearing; deviating is a deliberate decision, not an accident. Rationale for each is a Firefly decision.

1. **Go slow.** Verify game-mechanics assumptions against real DCB data before encoding them as types; reproduce a trusted reference (SCMDB, scmdb's crafter, starmap.space, in-game ground truth) to the digit. "We don't model this yet" beats a wrong model. A *negative* finding ("not in the data") needs the same evidence as a positive one — web sources about SC data are leads, never evidence.
2. **Real utility lib, not app-shaped.** When the lib and a consumer disagree, change the consumer.
3. **One canonical model per domain.** The most demanding consumer drives correctness; others read a subset.
4. **Layering is on data source, not format.** `sc-extract` owns DCB and non-DCB access alike.
5. **No string matching where a typed / data-derived alternative exists.** Cross-record joins are typed GUID / `Reference` / poly-subclass / instance-graph — never an entity name, debug name, display name or record-name token. CRC is for gRPC runtime data, never an in-p4k join. If a string match is genuinely unavoidable: scope it tightly, comment *why* no typed path works, and `warn!` on drift.

## Workspace layout

`Cargo.toml` `members` is the authoritative crate list. By role:

- **I/O-boundary** (fallible, own an `Error`): `sc-discovery` (standalone, no svarog), `sc-extract`; `sc-extract-generated` is workspace-internal generator output.
- **Foundational** (build from `&RecordStore`): `sc-items`, `sc-tags`, `sc-manufacturers`, `sc-resources`, `sc-locations`, `sc-gathering`.
- **Domain** (build from `&Datacore` + indices): `sc-crafting`, `sc-items-{fps-weapons,armor,ship-components,ship-weapons}`, `sc-missions`, `sc-weapons` (legacy; still the only home of DPS maths).
- **Umbrella**: `sc-holotable` — the recommended consumer dependency (features, `prelude`, `build_foundations`, `HolotableSnapshot`).
- **Tools**: `sc-generator` (DCB schema → Rust), `sc-bench`, `sc-explorer` (TUI), `sc-cargo-viewer` (3D cargo-grid viewer), `regenerate.ps1`, `release.ps1`, `bench/bench.ps1`.

**Never depend on `sc-extract-generated` directly** from another crate — go through `sc-extract`. Consumers never name svarog.

Consumers live outside the repo (`starlume`, `hearth`, `sc-cargo-planner`, `sc-fleetsync`, `sc-langpatch`, `bulkhead`); who pins what is a Firefly note.

## API conventions (the contract every crate follows)

The role fixes the construction signature and the error policy.

- **Naming.** Crate prefix `sc-`; sub-domains hyphenate (`sc-items-armor`). Wrapper, collection and storage field share **one noun**: `Item` / `Items` / `items`. A collection is the plural of what it holds — never a domain verb (`Gathering`), never a structural suffix (`Cache`, `Registry`, `Index`, `Tree`). No `RawFoo` / `Foo` split: the curated type takes the plain name.
- **Construction.** The verb is always `build`. Foundational: `Items::build(&RecordStore)`. Domain: `Armor::build(&Datacore, &Items)` — dependencies are taken by reference, never rebuilt internally. Never both `&RecordStore` and `&Datacore`. `X::new()` is the empty collection (== `Default`). A foundational crate also exposes `XBuilder: RecordVisitor` sharing one private `project_one` with `build`, and is wired into `build_foundations`.
- **Reads.** Every GUID-keyed collection implements `sc_extract::RecordCollection` — `get(&Guid)`, `len`, `iter() -> (&Guid, &Item)`, provided `is_empty` / `contains` / `values` / `guids`. **Trait-only:** no inherent copies. Secondary keys stay inherent: `by_<key>` → `Option<&Item>` (single) or `&[Guid]` (many); `by_crc` / `guid_by_crc` only where class-CRC resolution is meaningful. No public backing `Vec` / `HashMap` field.
- **Errors.** I/O-boundary crates own a `thiserror` enum `Error` (`#[non_exhaustive]`) + `Result` alias. In-memory crates are infallible and define **no** error type — a missing record is `None`. Never define an error you don't return. `anyhow` is banned in lib code.
- **Typed values.** Localization references are `LocaleKey`, never `String`, stored raw with the leading `@`; text is resolved at the call site via a method taking `&LocaleMap` (rule + rationale: `LocaleMap` rustdoc). Enum choices are the generated enum; unknown values round-trip as `Unrecognized(String)`. Cross-record links are `Option<CigGuid>`; re-enter the typed surface with `Datacore::resolve::<T>(&guid)`, not raw `db().record()` pokes. Foreign refs stay GUIDs — never flatten a resolved name into a struct. Curated polymorphic enums get `Other { type_name, struct_index }`.
- **Escape hatches.** Every typed record stays one step from the raw layer (entity handle, `raw` module re-exporting svarog types).
- **Deps.** `serde` derive always-on for curated types; generated types derive nothing (a crate serializing a generated enum writes an `as_dcb_str` / `from_dcb_str` adapter). `specta` behind a `specta` feature, pinned `=2.0.0-rc.21` (`^2` resolves to a nightly-only rc). `tracing` only — `debug!` per record, `info!` phase transitions, `warn!` recoverable, `error!` terminal. Each crate enables exactly the `sc-extract` leaf features it needs. `tui` is a per-crate feature.
- **Intentional exceptions:** `Missions::build(&Datacore)` is self-contained (builds its own `Items` + `Tags`); `sc-weapons::Weapons` keeps its public multi-family bundle.

New crate: pick the role → `Foo` / `Foos` / `foos` → `new` + `build` (+ `FoosBuilder`) → `impl RecordCollection` → errors per role → `LocaleKey` + typed enums → add to workspace `members` + `[workspace.dependencies]` + the umbrella (optional dep, feature, module, prelude).

## Generated code and feature gating

`tools/sc-generator` runs offline; its output is committed under `crates/sc-extract-generated/src/generated/` and never hand-edited. Mapping rules: `sc-generator` crate docs; classifier: `features.rs` module docs.

Closures are **data-driven** (walked from real record instances, not the schema). Types land in four buckets: `core/` (empty polymorphic bases, unconditional), `multi_feature/` (per-type `#[cfg(any(feature = …))]`), `dormant/` (never observed; behind `dormant`, which implies `full`), and one directory per leaf feature. **`multi_feature` is a module name, NOT a Cargo feature.**

```toml
sc-extract = { features = ["entities-scitem-ships"] }  # one leaf
sc-extract = { features = ["entities"] }               # a parent
sc-extract = { features = ["full"] }                   # every observed type
sc-extract = { features = ["dormant"] }                # every schema-reachable type
```

A missing feature is **not an error** — pools are empty and lookups return `None`. Default features parse 0 records.

After a regen, compare bucket sizes with the previous one: `core/` ballooning, cfg unions with hundreds of entries, or `dormant/` collapsing mean the classifier drifted toward schema-static.

## svarog

Git dep on `https://github.com/19h/Svarog.git`; all four crates (`svarog-common`, `-datacore`, `-p4k`, `-cryxml`) pinned by `rev` in the root `Cargo.toml`, members use `{ workspace = true }` — bump in one place. No `[patch]` section. `svarog-common` needs its `serde` feature (forgetting it yields ~1000 `CigGuid: Deserialize` errors). `RecordRef` has no serde; `Reference` fields are `Option<CigGuid>`. `Instance` hides its database — anything materializing array-of-Class elements goes through the `Builder`.

## Regenerating bindings and releasing

```powershell
pwsh tools/regenerate.ps1            # regen + fmt + clippy-fix + one local commit
pwsh tools/regenerate.ps1 -Publish   # + push + annotated tag datacore/<sc_version>
pwsh tools/release.ps1 -Version vX.Y.Z -Publish   # after editing CHANGELOG [Unreleased]
```

Two immutable, monotonic tag families: `datacore/<sc_version>` (bindings for a game build) and `sc-holotable/vX.Y.Z` (library API; pre-1.0: `0.X.0` for any public-surface change, `0.x.Y` for fixes). **Never move or recreate a tag** — Cargo lockfiles keep the old SHA; ship a library fix as a new `sc-holotable/v*` tag. Commit messages must be Conventional Commits (the `commit-msg` hook rejects others). The full patch-day runbook is a Firefly note.

Run a **debug** build after a game patch: `seed_database` panics with the list of record types the bindings don't know. Release builds skip unknown records silently.

## Build / test / lint

```bash
cargo check -p <crate>                       # warm incremental ~1s; keep features narrow
cargo test --workspace                       # the real test count
cargo clippy -p <crate> --all-targets -- -D warnings
just check                                   # what CI runs
just doc-check                               # rustdoc with broken intra-doc links as errors

# Smoke runs — always --release (a debug DCB parse is tens of times slower)
cargo run -p sc-extract --release --features full --example parse_real_p4k
cargo run -p sc-missions --release --example encounter_analytics   # encounter-graph regression baseline
cargo run -p sc-crafting --release --example product_stats         # scmdb reference reproduction
cargo run -p sc-explorer --release
```

`cargo check --all-targets` at the root uses the umbrella's **empty** default features, so umbrella targets needing features never compile there.

## Gotchas that bite

- **Never verify a cargo gate through a pipe** (`cargo test | tail`) — the pipe masks the exit code.
- **`tools/bench/bench.ps1` edits must be ASCII** — PowerShell 5.1 under the German locale corrupts non-ASCII script source.
- **No `install` / `setup` / `update` / `patch` in crate or binary names** — Windows installer detection demands UAC elevation and the test binary can't launch (`sc-installs` → `sc-discovery`).
- **Stacks.** `Datacore::parse` runs on its own 64 MB thread (deeply nested records like `GameMode.SC_Default`). `.cargo/config.toml` links Windows binaries with `/STACK:8388608`. If *rustc itself* overflows, derive load crept back into the generated crate — find it; don't re-add `RUST_MIN_STACK`.
- **No recursion through `Value` nesting** — `Builder::drain` and `ReferenceGraph::from_database` are worklists; the recursive versions overflowed on real data. `ReferenceGraph` stores `Reference` edges only.
- **Generated structs derive nothing** (no `Debug` / `Clone` / serde) — read through `handle.get(&pools)`. One generic `Handle<T>`; don't emit per-type id newtypes.
- **`seed_database` dispatch is name-based**, so reordered struct indices don't drop records.
- **Fallback shapes differ on purpose:** generated enums → `Unrecognized(String)` (DCB enums contain a literal `Unknown`); poly enums → `Unknown { struct_index, instance_index }` (the raw-layer handle — don't collapse it).
- **`LocaleKey` lives in `sc-extract-generated`** (generated fields reference it; elsewhere would be a dep cycle).
- **Locale formats:** `LocaleMap::parse` reads UTF-16 LE (as shipped in `Data.p4k`); `parse_utf8_bom` / `serialize` use UTF-8 + BOM (what sc-langpatch writes). Keys are normalized once at construction (`,P` suffix stripped).
- **Launcher log has two formats**, sometimes mixed in one file — one regex handles both; don't split the parser.
- **Plain tree walks don't see `Reference` targets** — follow the reference or use `ReferenceGraph`. Some links exist only in loose XML outside the DCB.
- Rust edition 2024, `rust-version` in the root manifest; let-chains are fine.

## Reference material (read-only local clones — do not modify)

- `E:\repros\Svarog` — svarog source + CLI (`target/release/svarog.exe`; rebuild after a DCB format bump).
- `E:\repros\StarBreaker` (MIT) — CryEngine geometry / chunk-file reference.
- `E:\repros\SuperLightTUI` — TUI lib under `tools/sc-explorer` (`docs/WIDGETS.md`, `examples/demo_cli.rs`).
- `E:\vscode\rust\sc-damage-calculator\src\extract\hull.rs` — working vehicle-XML parser, the port source if that phase is revived.
- `D:\Obsidian\Star Citizen\Game Files\` — binary format specs (P4K, DataCore, CryXmlB, chunk files, SOCPAK).
- `C:\Games\StarCitizen\LIVE\Data.p4k` — what the generator and every probe read.
