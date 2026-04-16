# tools/bench

Benchmark script for sc-extract compile and runtime costs. Results feed
into [`docs/benchmarks.md`](../../docs/benchmarks.md).

## Quick start

```powershell
# From the workspace root. Full suite, ~30-60 minutes.
.\tools\bench\bench.ps1 -Mode all -KillRa
```

If PowerShell blocks the script due to execution policy, run it as:

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\bench\bench.ps1 -Mode all -KillRa
```

## What to benchmark

The script has three independent modes -- use `-Mode` to pick one.

| Mode | Measures | Cost |
|---|---|---|
| `check` | Cold `cargo check -p sc-extract` for each feature set | ~10 min |
| `build` | Cold `cargo build -p sc-bench --profile X` for each profile x feature set | ~30 min |
| `runtime` | `sc-bench` against real `Data.p4k`, with and without the reference graph | ~5 min |
| `all` | Everything above, chained (build -> runtime per profile x feature) | ~45 min |

When `all` is selected, cold build and runtime are **chained** per
profile x feature combination: the cold build measurement feeds directly
into the runtime benchmark, avoiding a redundant rebuild.

## Multi-profile support

The script accepts `-Profiles` to benchmark multiple Cargo profiles in
a single run. Each profile must be a `[profile.XXX]` section in
`Cargo.toml`. This is the primary way to compare release config
alternatives (LTO modes, codegen-units, opt-level) side by side.

```powershell
# Compare three profiles on one feature set
.\tools\bench\bench.ps1 -Mode build -Profiles release,bench-thin,bench-cu1 -Features entities-scitem-ships -KillRa
```

Profiles default to `release` if not specified.

## Feature sets

By default the script runs every feature set for compile benchmarks:

- `default` -- no features enabled. Only core + multi_feature + always-compiled code.
- `ammoparams` -- one narrow leaf feature (representative of a single domain crate consumer).
- `entities-scitem-ships` -- one hub-ish leaf (pulls in a lot of polymorphic fan-out).
- `full` -- every observed type.
- `dormant` -- `full` + schema-reachable types never observed in real data.

Runtime benchmarks default to `default` and `full` only (to keep the
total runtime manageable). Pass `-Features <list>` to narrow or widen
the set. Example:

```powershell
.\tools\bench\bench.ps1 -Mode check -Features default,full -KillRa
```

## Controls during a running step

The script polls the console input buffer every 500 ms while a cargo
invocation is running. You can press:

| Key | Effect |
|---|---|
| `s` | **Skip the current step.** Kills cargo (and any spawned rustc/link processes), records the step as `Skipped` with the elapsed time so far, and moves on to the next one. |
| `a` | **Abort the run.** Kills cargo like skip, but then breaks out of the remaining loops. Partial results are persisted. |
| `Ctrl+C` | **Nuclear.** Kills the PowerShell process. Whatever the last incremental save wrote is still on disk. |

**Verifying the controls work on your host:**

```powershell
.\tools\bench\bench.ps1 -TestControls
```

Runs a dummy polling loop with no cargo spawn. Presses are echoed
immediately. This is a ~10 second check that saves you from discovering
a broken keypress path 2 hours into a benchmark.

If `[Console]::KeyAvailable` is not supported on your terminal
(remoting layers, some terminal emulators), the script falls back to
synchronous blocking and prints a warning. Use `Ctrl+C` in that case.

## Non-destructive history

Results are **never overwritten**. The script:

1. Writes `target/bench-results.json` (always, overwritten with latest).
2. Copies to `target/bench-history/YYYY-MM-DDTHH-mm-ss.json` (timestamped, append-only).
3. Before updating `docs/benchmarks.md`, auto-archives the previous
   run's "Latest results" tables into the History section.

No manual "copy before re-running" step needed. Pass `-NoAutoArchive` to
suppress the markdown auto-archive (e.g., when re-running the same config
and you don't want duplicate history entries).

## Incremental persistence

**Results are written to disk after every completed step**, not just
at the end. If you `Ctrl+C` out of the script, everything that had
completed is already on disk.

## The runtime binary: sc-bench

The script uses `tools/sc-bench` (a workspace member crate) as the
runtime benchmark binary. It exercises the full sc-extract API surface:

- Parse DCB with `Datacore::parse`
- Tag tree traversal (roots, ancestors, descendants, depth)
- Manufacturer lookups (by code, iterate all)
- Display name iteration and counting
- Locale lookups (resolve, contains_key)
- Reference graph queries (outgoing, incoming, 2-hop reachability)
- Filter predicates (`is_playable_weapon`, `is_playable_ship` on real data)
- Raw svarog escape hatch (`db().records_by_type()`)
- Snapshot round-trip (capture, save, load, hydrate)

Each phase is independently timed. The binary outputs structured JSON
on stdout, which the bench script parses via `ConvertFrom-Json`.

The existing `parse_real_p4k` example is kept as a quick smoke test
but is no longer used for benchmarking.

## rust-analyzer interference

**rust-analyzer is the biggest source of benchmark noise** in this
workspace. The script detects it on startup and refuses to run unless
you pick one of:

| Flag | Effect |
|---|---|
| _(none)_ | Error out. Default -- protects from noisy numbers. |
| `-KillRa` | Force-kill `rust-analyzer.exe`. VSCode will restart it on window focus. |
| `-IgnoreRa` | Proceed anyway. Results will be noisy. |

Alternative: pause RA from inside VSCode:

1. `Ctrl+Shift+P` -> "rust-analyzer: Stop server"
2. Run the script
3. `Ctrl+Shift+P` -> "rust-analyzer: Start server"

## Output

- **`target/bench-results.json`** -- structured results for the latest run.
- **`target/bench-history/`** -- timestamped JSON files, one per run.
- **`docs/benchmarks.md`** -- auto-updated "Latest results" section.
  Previous runs are auto-archived into the History section.
- **Summary table on stdout**.

### Peak memory

Every benchmarked cargo invocation reports a peak RAM number. The
script samples the working set of all `cargo`/`rustc`/`link`/
`lld-link`/`sc-bench` processes every 500 ms and tracks the peak.

- **Under `-KillRa`** the sample is clean.
- **Under `-IgnoreRa`** the sample includes rust-analyzer's background activity.
- Sampling has 500 ms granularity. Short link-phase spikes can be missed.

## Re-running a single benchmark

```powershell
# Just cold full build
.\tools\bench\bench.ps1 -Mode build -Features full -KillRa

# Compare two profiles on one feature
.\tools\bench\bench.ps1 -Mode build -Profiles release,bench-thin -Features entities-scitem-ships -KillRa

# Just runtime for full (narrow consumer simulation)
.\tools\bench\bench.ps1 -Mode runtime -Features full -KillRa

# Skip markdown update
.\tools\bench\bench.ps1 -Mode build -Features full -KillRa -NoDocWrite
```
