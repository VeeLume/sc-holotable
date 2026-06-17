# DCB / object-container exploration guide

> A reusable playbook for figuring out **where some piece of Star Citizen game
> data lives** and **how to extract it** — the method behind
> [`resource-gathering.md`](resource-gathering.md). The resource dig is the
> worked example throughout, but the technique generalizes to any domain
> (weapons, missions, locations, economy, …).

## When to use this

Reach for this when a question is *"is X in the data, and if so, where?"* and
the answer isn't already obvious from the generated bindings — especially when:

- the data might span the **DCB** *and* the **object containers** (placement,
  per-location, per-instance data), or
- you suspect the typed bindings are incomplete/misleading and need to check
  against the **real, live** records, or
- a previous answer was a confident *"it's not in the data"* that you want to
  disprove (the resource dig started exactly there).

## The mental model: two data layers

Star Citizen ships data in two layers, and **the join between them is where most
"where is X" questions actually live**:

| Layer | What it is | Format | How to read it |
|---|---|---|---|
| **DataCore** (`Data\Game2.dcb`) | ~115 k typed records — the *templates / definitions* (recipes, item stats, presets, entity *classes*) | one binary blob, typed schema | `sc-extract` typed bindings, or svarog → XML corpus |
| **Object containers** (`.socpak` → `.soc` / `.entxml`) | the *world instances* — which entity is placed where, with which per-instance component values | per-zone chunked binaries (`CrChF`, `CryXmlB`) | svarog `--expand-socpak`, `cryxml-convert`, byte-carving `.soc` |

The DCB tells you *what a Clio mining rock is*; the object container tells you
*that Clio has a mining-resource provider and which preset it points at*. The
resource dig's breakthrough was finding that the per-location join lived **only**
in the `.soc` layer — no DCB record referenced it.

## Phase 1 — Orient in the schema (generated bindings)

Start from `crates/sc-extract-generated/src/generated/` — it *is* the DCB
schema, already decoded. This is fast and free (no parse).

1. **Grep for candidate record/struct names** by domain vocabulary:
   ```
   Grep "pub struct \w*(Mineable|Harvest|Deposit|Resource|Salvage)\w*"
        path=crates/sc-extract-generated/src/generated  output_mode=content
   ```
2. **Find the seeded (top-level, queryable) record types** in
   `generated/record_store.rs` — these have `alloc_record` + `RecordLookup`
   impls, i.e. you can look them up by GUID and they appear in
   `records_by_type`. Grep it for your domain terms.
3. **Read the struct bodies**, but skip the giant `#[cfg(any(...))]` unions —
   read the few lines right after `pub struct Foo {`. Note which fields are
   `Reference` (cross-record GUID link, `Option<CigGuid>`), `StrongPointer`
   /`Class` (owned, materialized into pools), or `Locale` (`LocaleKey`).
4. **Follow the type, not your assumptions.** A field's *declared* type often
   points at a polymorphic base; the real subclasses are in `poly_enums.rs`.

> The leaf feature directory a type lives in (`harvestable/`, `entities_*/`,
> `multi_feature/`) hints at how it's used and which Cargo `--features` you'd
> need to decode it with the typed API.

## Phase 2 — Verify what's actually populated (don't trust the schema)

The schema says a type *can* exist; it doesn't say the live game *has* any. The
resource dig's first wrong turn was concluding `MineableComposition` records
were "dormant templates" — a parse proved there were **249** of them.

**Probe the live DCB via the raw svarog escape hatch** — this needs no feature
gates (the raw layer decodes every record regardless of what the typed bindings
were compiled with). Write a throwaway example under
`crates/sc-extract/examples/`:

```rust
// examples/probe_counts.rs  —  cargo run -p sc-extract --release --example probe_counts
use sc_extract::{AssetConfig, AssetData, AssetSource, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();                      // raw DataCoreDatabase
    let loc = &asset_data.locale;

    for ty in ["MineableComposition", "MineableElement", "ResourceType"] {
        println!("{ty:<28} {}", db.records_by_type(ty).count());
    }
    // sample a few records: record name + a resolved Locale field
    for r in db.records_by_type("ResourceType").take(20) {
        let dn = r.get_str("displayName").and_then(|k| loc.resolve(k)).unwrap_or("");
        println!("  {:<40} {dn}", r.name().unwrap_or("?"));
    }
    Ok(())
}
```

Raw-layer API worth knowing (all on `datacore.db()`):

- `records_by_type("Foo")` / `records_by_type_containing("Foo")` → iterator of
  `Record`.
- `Record`: `.name()`, `.id()` (GUID), `.type_name()`, `.get(field) -> Value`,
  `.get_str(field)`, `.get_array(field)`, `.as_instance()`.
- `asset_data.locale.resolve("@key" | "key")` → display string.
- `assets.archive()` → `P4kArchive`; `.iter()` yields entries with `.name` and
  `.uncompressed_size` (fields, not methods).

**Build with `--release`** — debug DCB parse is tens of times slower. No feature
flags needed for raw queries. Delete the probe when done (keep the tree clean);
the workspace convention is committed `examples/*_dump.rs` only for durable digs.

## Phase 3 — Build the corpus (when grep-the-whole-DCB beats typed queries)

For broad "where does this pattern appear" sweeps, a flat XML corpus you can
`grep -r` is far faster to iterate on than recompiling typed probes. Use the
svarog CLI (`E:\repros\Svarog\target\release\svarog.exe`).

**Export the entire DCB to per-record XML (~60 k files):**
```bash
svarog.exe p4k-extract -p "C:\Games\StarCitizen\LIVE\Data.p4k" \
  -o target/probe-resources/dcbraw --filter "*Game2.dcb"
# (--extract-dcb is the explicit flag for the same effect)
```

**Expand object containers for a system (recursive socpak unpack):**
```bash
svarog.exe p4k-extract -p "C:\Games\StarCitizen\LIVE\Data.p4k" \
  -o target/probe-resources/oc --regex \
  --filter 'ObjectContainers[/\\]PU[/\\]system[/\\](stanton|nyx)[/\\].*socpak$' \
  --expand-socpak
```

**Dump the full p4k entry listing** (1.3 M entries) to grep for asset paths —
do this from a tiny `archive.iter()` probe (Phase 2 API) writing
`name<TAB>size`, or use `p4k-list`.

Keep all of this under `target/` (gitignored).

> **Two gotchas that will bite (both hit the resource dig):**
>
> 1. **Stale svarog binary → `unsupported DataCore version: N (expected 5
>    through 8)`.** The local `E:\repros\Svarog` clone may predate a DCB format
>    bump. Rebuild at the workspace's pinned rev: `cargo build --release -p
>    svarog`. Also: `dcb-extract -i <p4k>` mis-parsed the archive (reported
>    "version 45"); the reliable path is `p4k-extract --filter "*Game2.dcb"`,
>    which extracts `Game2.dcb` then auto-runs the DCB→XML export.
> 2. **The XML corpus collapses records that share a DCB file path.** All 206
>    `ResourceType.*` records export to a single
>    `resourcetypedatabase/resourcetypedatabase.xml` (only one survives on disk).
>    When you need every record of a path-colliding type individually, dump via
>    `dcb-extract -f <pathfragment>` or a typed probe instead. The corpus has
>    60,309 files for ~115 k records — expect collisions.

## Phase 4 — Trace chains in the corpus

Now grep the corpus to follow references. DCB XML uses
`<field><ReferencedFile>file://…/foo.xml</ReferencedFile></field>` for links and
`<field><RecordId>guid</RecordId>…` for record references — both human-readable.

- **Find where a type is used:** `grep -rl "SAsteroidGasCloudComponentParams"
  dcbraw/libs/foundry/records/`.
- **Read a record and its references**, then `cat` the referenced files to walk
  the chain (the resource dig walked
  `HarvestableProviderPreset → HarvestablePreset → entityClass →
  MineableParams.composition → MineableComposition`).
- **Object-container files have mixed formats** — check magic bytes first
  (`head -c 8 file | od -c`):
  - `<ObjectC…` → plain XML, grep directly.
  - `CryXmlB\0` → binary; convert with
    `svarog.exe cryxml-convert -i <in> -o <out.xml>` (`.entxml`, `.rmxml`).
  - `CrChF` → `.soc` chunk container; **svarog has no parser** — string-carve
    (below).

## Phase 5 — Find cross-layer joins (the GUID-intersection trick)

This is the technique that cracked the location join. When you suspect layer A
(DCB records) is referenced by layer B (object containers) but don't know which
field carries it, **intersect their GUID sets**:

```bash
# 1. GUIDs of candidate DCB records (e.g. dumped to guids.txt: guid<TAB>type<TAB>name)
cut -f1 guids.txt | sort -u > /tmp/dcb_guids.txt

# 2. every GUID that appears as a string anywhere in the OC tree
find oc/.../nyx -type f \( -name '*.soc' -o -name '*.entxml' -o -name '*.xml' \) \
  | while read f; do strings -n 30 "$f"; done \
  | grep -oE '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}' \
  | sort -u > /tmp/oc_guids.txt

# 3. the intersection is your join
comm -12 /tmp/dcb_guids.txt /tmp/oc_guids.txt
```

In the dig this returned three GUIDs — all `HarvestableProviderPreset`s
(`HPP_Nyx_GlaciemRing`, …) — instantly revealing both the join record type and
the field (`HarvestableProviderComponent.preset`) without reading a schema.

**Reading values out of a `CrChF` `.soc`** (no parser available): the serialized
property table is plain strings interleaved with binary. `strings -n 6 file.soc
| grep -iE 'composition|Harvestable|preset'` surfaces component names and
adjacent GUIDs; for a specific entity, find the offset of its name and read the
property names/values that follow. This is good enough for *discovery*; a robust
extractor needs a real CrChF parser (the open engineering item).

## Phase 6 — Verify against ground truth

Don't stop at "the field exists" — **compute the answer from the data and check
it against a known-good reference** (here, the SCMDB/Rocks-Syndicate UI numbers).
The resource dig confirmed `groupProbability` normalization by reproducing Clio's
13.8 / 28.7 / 57.5 % exactly, and signatures by matching all 25 ores' `×1000`
values. Exact reproduction is what turns "plausible" into "proven."

For wide or high-stakes verification, fan out a **multi-agent workflow** (the dig
used 6 trace + 6 adversarial-verify agents over the corpora). The pattern that
worked:

- **One trace agent per sub-domain** (mining / plants / salvage / signal+quality
  / composition tables / one per location), each handed the corpus paths, the
  known schema, the reference numbers to reproduce, and a structured output
  schema (`verdict` / `chain[file,evidence]` / `computable[value,bool,explain]`
  / `gaps`).
- **An adversarial verifier per trace** that re-opens the cited files and tries
  to *refute* the load-bearing claims (GUID misresolutions, probability
  misreadings, missed alternative paths).
- Give agents the file-format rules (CryXmlB vs CrChF), the `cryxml-convert`
  command, and permission to extract more from the p4k themselves.

Read every agent result; treat "computable=NO" and "gaps" as the real output —
they're the boundary of what the data actually contains.

## File-format quick reference

| Ext / magic | What | Read with |
|---|---|---|
| `Data\Game2.dcb` | the DataCore blob | `sc-extract` typed API, or svarog `p4k-extract --filter "*Game2.dcb"` → XML |
| `.socpak` | zip of one object container | svarog `--expand-socpak`, or `Expand-Archive` |
| `.soc` (`CrChF`) | OC chunk container — entity instances, per-instance component values | **no parser**; string-carve (Phase 5) |
| `.entxml`, `.rmxml` (`CryXmlB\0`) | binary CryXML entity/region data | `svarog.exe cryxml-convert` |
| `.xml` (`<…`) | plain-text record/OC XML | grep / read directly |
| `.eco`, `.opr`, `.cgf*`, `.dds` | ecosystem / object-preset / mesh / texture | mostly not data-bearing for "where is X" |

## Tooling reference

**svarog CLI** (`E:\repros\Svarog\target\release\svarog.exe`; rebuild with
`cargo build --release -p svarog` if it errors on DCB version):

- `p4k-list -p <p4k>` — list archive entries.
- `p4k-extract -p <p4k> -o <out> [--regex] --filter <pat> [--expand-socpak] [--extract-dcb]`
- `dcb-extract -i <dcb|p4k> -o <out> [-f <pathfragment>]` — per-record XML (use
  `-f` to dodge the path-collapse gotcha).
- `cryxml-convert -i <in> -o <out.xml>` — CryXmlB → XML.

**Typed / raw escape hatch** (in a `crates/sc-extract/examples/*.rs` probe, run
`--release`):

- `sc_discovery::discover_primary()` → install; `install.data_p4k()` → p4k path.
- `AssetSource::from_install(&install)` / `AssetSource::open(&p4k_path)`.
- `AssetData::extract(&assets, &AssetConfig::standard())` → locale etc.
- `sc_extract::Datacore::parse(&assets, &asset_data)`; `.db()` for raw queries,
  `.records()` for the typed store; `Datacore::resolve::<T>(&guid)` to re-enter
  the typed surface from a `Reference` GUID.

## Recipe (condensed)

1. **Schema scan** — grep generated bindings + `record_store.rs` for candidate
   record types and their fields. (Phase 1)
2. **Reality check** — `--release` probe with `db.records_by_type(...).count()`;
   confirm the records actually exist and sample a few. (Phase 2)
3. **Corpus** — `p4k-extract --filter "*Game2.dcb"` (DCB→XML) and
   `--expand-socpak` for the relevant system(s); dump the p4k listing. Mind the
   svarog-version and path-collapse gotchas. (Phase 3)
4. **Trace** — grep the corpus, walk `ReferencedFile`/`RecordId` links; convert
   CryXmlB as needed. (Phase 4)
5. **Cross-layer join** — intersect DCB GUIDs with GUIDs string-carved from the
   OC tree (`comm -12`). (Phase 5)
6. **Prove it** — compute from the data, match a known-good reference to the
   digit; fan out trace + adversarial-verify agents for breadth. (Phase 6)
7. **Write it up** — record the chain with `file:line` / record-path citations,
   live counts, and the boundary of what's *not* in the data (runtime-only).
   Save a [[memory]] for the non-obvious verdict.
