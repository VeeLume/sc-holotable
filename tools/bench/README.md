# tools/bench

Benchmark script for sc-extract compile and runtime costs. Results feed
into [`docs/benchmarks.md`](../../docs/benchmarks.md).

## Quick start

```powershell
# From the workspace root. Full suite, ~30–60 minutes.
.\tools\bench\bench.ps1 -Mode all -KillRa
```

If PowerShell blocks the script due to execution policy, run it as:

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\bench\bench.ps1 -Mode all -KillRa
```

## What to benchmark

The script has three independent modes — use `-Mode` to pick one.

| Mode | Measures | Cost |
|---|---|---|
| `check` | Cold `cargo check -p sc-extract` for each feature set | ~10 min |
| `build` | Cold `cargo build -p sc-extract --profile dev-opt` for each feature set | ~30 min |
| `runtime` | `parse_real_p4k` against real `C:\Games\StarCitizen\LIVE\Data.p4k`, with and without the reference graph | ~5 min |
| `all` | Everything above | ~45 min |

## Feature sets

By default the script runs every feature set for compile benchmarks:

- `default` — no features enabled. Only core + multi_feature + always-compiled code.
- `ammoparams` — one narrow leaf feature (representative of a single domain crate consumer).
- `entities-scitem-ships` — one hub-ish leaf (pulls in a lot of polymorphic fan-out).
- `full` — every observed type.
- `dormant` — `full` + schema-reachable types never observed in real data.

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
| `s` | **Skip the current step.** Kills cargo (and any spawned rustc/link processes), records the step as `Skipped` with the elapsed time so far, and moves on to the next one. Useful when a build like `entities-scitem-ships --profile dev-opt` is taking 30+ minutes and you want to keep the rest of the matrix moving. |
| `a` | **Abort the run.** Kills cargo like skip, but then breaks out of the remaining loops. Partial results are persisted, summary prints, script exits cleanly. |
| `Ctrl+C` | **Nuclear.** Kills the PowerShell process. Whatever the last incremental save wrote is still on disk (see below), but no summary runs. |

Key presses are buffered between polls, so there's up to a ~500 ms
delay between pressing `s`/`a` and cargo actually getting killed.
Don't hammer the key — one press is enough.

**Verifying the controls work on your host:**

```powershell
.\tools\bench\bench.ps1 -TestControls
```

Runs a dummy polling loop with no cargo spawn. Presses are echoed
immediately — `s` prints `detected: skip`, `a` prints `detected: abort`
and exits, anything else prints `[key 'x' ignored …]`. A heartbeat
line prints every ~5 seconds so you know the loop is alive. This is
a ~10 second check that saves you from discovering a broken
keypress path 2 hours into a benchmark.

**Why the controls can break:** PowerShell reads key events from the
parent console's input buffer. If something else (a child process,
another console app in the same window) grabs the input handle, your
keystrokes go there instead of to PS. The script works around this
by redirecting cargo's stdin to an empty file so cargo can't claim
the console input, but if `-TestControls` doesn't see your keys,
something further up the stack (the terminal emulator, a remoting
layer, etc.) is stealing them. Fall back to `Ctrl+C` + manual
`-Features <name> -Mode <mode>` targeted re-runs.

## Incremental persistence

**Results are written to disk after every completed step**, not just
at the end. This means:

- If your 3-hour `full` dev-opt build finishes successfully, the
  result is already in `target/bench-results.json` and
  `docs/benchmarks.md` *before* the next step starts.
- If you `Ctrl+C` out of the script, everything that had completed
  by the time you killed it is still on disk — you don't lose the
  hours of compile time that already paid off.
- If you press `s` to skip a step, the skip is recorded as a row
  in the markdown (time = elapsed so far, status = `(skipped)`)
  and any previous completed steps are preserved.

This means it's safe to start a long `-Mode all` run, watch the first
few steps, and press `s` on the ones that are clearly going to take
hours — you still get a complete benchmark table, with the skipped
rows visible so you know to come back and fill them in later.

## rust-analyzer interference

**rust-analyzer is the biggest source of benchmark noise** in this
workspace. It runs its own `cargo check` on every `.rs` edit, using
`rust-analyzer.cargo.features = "all"` — which means it's essentially
running a full-features check concurrently with whatever the script is
measuring.

The script detects rust-analyzer on startup and refuses to run unless
you pick one of:

| Flag | Effect |
|---|---|
| _(none)_ | Error out with a list of options. Default — protects you from accidentally producing noisy numbers. |
| `-KillRa` | Force-kill `rust-analyzer.exe` + `rust-analyzer-proc-macro-srv.exe` before starting. VSCode will restart them when it next polls (usually on window focus). **Don't switch back to VSCode mid-run.** |
| `-IgnoreRa` | Proceed even though RA is running. Results will be noisy. Only use this if you're deliberately measuring day-to-day dev experience with RA in the background. |

Alternative: pause RA from inside VSCode before running the script, and
restart it afterwards. This is the least-disruptive option if you're
actively using the IDE:

1. `Ctrl+Shift+P` → "rust-analyzer: Stop server"
2. Run the script (no `-KillRa` / `-IgnoreRa` needed — it'll detect RA is quiet)
3. `Ctrl+Shift+P` → "rust-analyzer: Start server"

## What gets measured

### `-Mode check`

For each feature set:

1. `cargo clean -p sc-extract -p sc-extract-generated` — clears the two
   workspace crates that change between runs. External deps (svarog,
   serde, zstd, regex) stay cached; we measure only the part of the
   build that's affected by feature selection.
2. `cargo check -p sc-extract [--features …]` wall-clock timed.

### `-Mode build`

For each feature set:

1. `cargo clean --profile dev-opt` — whole-profile clean so deps get
   recompiled too. This is the slowest mode because it rebuilds svarog
   + serde + rmp-serde every time.
2. `cargo build -p sc-extract --profile dev-opt [--features …]`
   wall-clock timed.

### `-Mode runtime`

For each feature set (defaults to `default` and `full`):

1. Build the `parse_real_p4k` example (not cold-measured — this is a
   runtime benchmark, not a compile benchmark).
2. Run it twice: once without the reference graph (`DatacoreConfig::standard()`),
   once with (`DatacoreConfig::all()`, passed as `--all`).
3. Parse the example's stdout summary for: record count, locale entries,
   manufacturers, display names, tag nodes, graph edges, parse time,
   snapshot size on disk, save time, load time.

The example discovers the primary SC install via `sc_installs::discover_primary()`.
You need a working install at `C:\Games\StarCitizen\LIVE\` (or wherever
your discovered install lives). If the install isn't found, the example
fails and the script raises.

## Output

- **`target\bench-results.json`** — structured results for the run,
  including the full environment snapshot (rustc version, cargo version,
  CPU, RAM, whether RA was killed, peak memory per run). This is the
  machine-readable source of truth for future comparisons.
- **`docs/benchmarks.md`** — the script rewrites the region bracketed
  by `<!-- BENCH:RESULTS-START -->` / `<!-- BENCH:RESULTS-END -->`
  with the tables from this run. Methodology, interpretation, and the
  History section at the bottom of that file are preserved — only the
  "Latest results" block is auto-managed. Pass `-NoDocWrite` to
  suppress this.
- **Summary table on stdout** — same numbers, print-formatted, so you
  can eyeball them without opening the markdown file.

**Before re-running the script**, if you want to keep the previous run
as a trend datapoint, copy the "Latest results" tables from
`docs/benchmarks.md` into its History section with a dated heading.
The script does not merge old results; it overwrites.

### Peak memory

Every benchmarked cargo invocation reports a peak RAM number. The
script samples the working set of all `cargo`/`rustc`/`link`/
`lld-link`/`parse_real_p4k` processes every 500 ms and tracks the
peak summed value. That's what rustc + its children were holding at
the worst moment of the build.

Notes:

- **Under `-KillRa`** the sample is clean — no foreign rustc is
  running.
- **Under `-IgnoreRa`** the sample includes whatever rust-analyzer
  is doing in the background. Rhetorical value only; don't use those
  numbers to set OOM limits.
- Sampling has 500 ms granularity. Short link-phase spikes can be
  missed. For rustc-dominated workloads (typecheck + monomorphize)
  the plateau shape means the reported peak is accurate within
  ~5 %.

## Interpreting results

The numbers you care about depend on who's complaining:

- **"my edit–check cycle is too slow"** → warm incremental check. Not
  measured here because it's `<1s` on any well-cached workspace. If this
  is your problem, check that rust-analyzer isn't fighting with your
  terminal cargo.
- **"cold starts are too slow after `cargo clean`"** → `-Mode check`
  numbers. Default should be fastest, full should be slowest, dormant
  slightly above full.
- **"CI is too slow"** → `-Mode build` numbers (dev-opt is close to what
  CI does with `--release` minus LTO).
- **"parsing the DCB takes too long"** → `-Mode runtime` numbers.
  Parse time + load time are the user-visible cost. Snapshot size
  matters if you distribute the snapshot over the wire.

## Re-running a single benchmark

You can narrow the script to one feature set and one mode:

```powershell
# Just cold full build
.\tools\bench\bench.ps1 -Mode build -Features full -KillRa

# Just runtime for ammoparams (narrow consumer simulation)
.\tools\bench\bench.ps1 -Mode runtime -Features ammoparams -KillRa

# Cold full build but don't touch docs/benchmarks.md
.\tools\bench\bench.ps1 -Mode build -Features full -KillRa -NoDocWrite
```

Partial runs still overwrite the matching section of
`docs/benchmarks.md` — if you only ran `-Mode check`, the Build and
Runtime sections of the auto-generated block disappear from the doc
until you run those modes again. Pass `-NoDocWrite` if you don't want
the doc touched for a narrow re-measurement.

Results always overwrite `target/bench-results.json`. Copy it to a
dated filename if you want to keep a machine-readable history:

```powershell
Copy-Item target\bench-results.json "target\bench-results-$(Get-Date -Format yyyy-MM-dd).json"
```
