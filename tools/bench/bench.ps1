<#
.SYNOPSIS
    Repeatable compile-time and runtime benchmarks for sc-extract.

.DESCRIPTION
    Runs cold `cargo check` / `cargo build` / runtime `parse_real_p4k`
    benchmarks against a configurable set of Cargo feature configurations.
    Writes results to target/bench-results.json and prints a summary table.

    RUST-ANALYZER CONTAMINATION
    ---------------------------
    rust-analyzer runs its own `cargo check` in the background on every
    file change. If it's active during a benchmark run it competes for
    CPU and disk with the cargo process being measured, inflating times
    and adding variance. The script detects rust-analyzer and errors out
    by default. Options:

      -KillRa      Force-kill rust-analyzer.exe before measuring. VSCode
                   will restart it automatically when it detects the
                   loss, but not during this script's runtime. Cleanest
                   for benchmark accuracy, disruptive if you're actively
                   using the IDE.

      -IgnoreRa    Proceed even though rust-analyzer is running. Results
                   will be noisy. Use when you explicitly want to measure
                   "day-to-day dev experience with RA running".

    Alternative: pause RA from inside VSCode before running:
      Ctrl+Shift+P -> "rust-analyzer: Stop server"
    Then run the script with neither flag and restart RA afterward:
      Ctrl+Shift+P -> "rust-analyzer: Start server"

.PARAMETER Mode
    Which benchmark categories to run.

      check      Cold `cargo check` per feature set
      build      Cold `cargo build --profile dev-opt` per feature set
      runtime    Runtime `parse_real_p4k` per feature set (default + full
                 unless -Features overrides)
      all        Everything (check + build + runtime)

    Defaults to 'all'.

.PARAMETER Features
    Which feature sets to benchmark. Any of:
        default, ammoparams, entities-scitem-ships, full, dormant
    Omit to use the full set for check/build, and default+full for runtime.

.PARAMETER KillRa
    Stop rust-analyzer.exe and rust-analyzer-proc-macro-srv.exe before
    measuring. VSCode will restart them when it next polls, so do NOT
    switch back to VSCode until the script is done.

.PARAMETER IgnoreRa
    Don't error out if rust-analyzer is running. Measurements will be
    noisy. Used mostly for smoke-testing the script itself.

.PARAMETER Quiet
    Suppress live cargo stdout/stderr; show only timing results.

.PARAMETER OutJson
    Path to write structured results. Defaults to target/bench-results.json
    under the workspace root.

.PARAMETER BenchDoc
    Path to the benchmarks markdown document to update. Defaults to
    docs/benchmarks.md under the workspace root. The script replaces
    the region between the <!-- BENCH:RESULTS-START --> and
    <!-- BENCH:RESULTS-END --> markers with the tables from this run;
    everything outside those markers (methodology, history,
    interpretation notes) is preserved.

.PARAMETER NoDocWrite
    Skip updating the benchmarks markdown document. Results still go
    to target/bench-results.json and stdout.

.EXAMPLE
    # Full suite, clean measurement (stops RA, takes ~45 min)
    .\tools\bench\bench.ps1 -Mode all -KillRa

.EXAMPLE
    # Just cold check times for all features, quiet output
    .\tools\bench\bench.ps1 -Mode check -KillRa -Quiet

.EXAMPLE
    # Runtime parse benchmarks for a single feature set
    .\tools\bench\bench.ps1 -Mode runtime -Features full -KillRa

.EXAMPLE
    # Day-to-day noisy measurement, don't touch RA
    .\tools\bench\bench.ps1 -Mode check -IgnoreRa

.NOTES
    Execution policy: if PowerShell refuses to run the script, use
        powershell -ExecutionPolicy Bypass -File .\tools\bench\bench.ps1 -Mode all -KillRa
#>

[CmdletBinding()]
param(
    [ValidateSet('check', 'build', 'runtime', 'all')]
    [string]$Mode = 'all',

    [ValidateSet('default', 'ammoparams', 'entities-scitem-ships', 'full', 'dormant')]
    [string[]]$Features = @(),

    [switch]$KillRa,
    [switch]$IgnoreRa,
    [switch]$Quiet,

    [string]$OutJson = '',
    [string]$BenchDoc = '',
    [switch]$NoDocWrite
)

$ErrorActionPreference = 'Stop'

# Force invariant / en-US number formatting so "1879" prints as
# "1,879 MB" / "1.84 GB", not "1.879 MB" / "1,84 GB" on de-DE hosts.
[System.Threading.Thread]::CurrentThread.CurrentCulture = 'en-US'

# ── Workspace root ──────────────────────────────────────────────────────

$WorkspaceRoot = (Resolve-Path "$PSScriptRoot\..\..").Path
Set-Location $WorkspaceRoot

if (-not $OutJson) {
    $OutJson = Join-Path $WorkspaceRoot 'target\bench-results.json'
}
if (-not $BenchDoc) {
    $BenchDoc = Join-Path $WorkspaceRoot 'docs\benchmarks.md'
}

Write-Host ""
Write-Host "sc-holotable benchmark" -ForegroundColor Cyan
Write-Host "  workspace: $WorkspaceRoot"
Write-Host "  mode     : $Mode"

# ── Rust-analyzer detection ─────────────────────────────────────────────

function Get-RaProcesses {
    @(
        Get-Process -Name 'rust-analyzer' -ErrorAction SilentlyContinue
        Get-Process -Name 'rust-analyzer-proc-macro-srv' -ErrorAction SilentlyContinue
    ) | Where-Object { $_ -ne $null }
}

$RaProcesses = Get-RaProcesses

if ($RaProcesses.Count -gt 0) {
    if ($KillRa) {
        Write-Host ""
        Write-Host "Stopping rust-analyzer ($($RaProcesses.Count) process(es))..." -ForegroundColor Yellow
        $RaProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 1

        # Also nuke any rustc rust-analyzer had spawned. Our own cargo
        # invocations haven't started yet, so any running rustc right now
        # belongs to rust-analyzer.
        $StrayRustc = Get-Process -Name 'rustc' -ErrorAction SilentlyContinue
        if ($StrayRustc) {
            Write-Host "Stopping $($StrayRustc.Count) orphaned rustc process(es)..." -ForegroundColor Yellow
            $StrayRustc | Stop-Process -Force -ErrorAction SilentlyContinue
            Start-Sleep -Seconds 1
        }

        $StillAlive = Get-RaProcesses
        if ($StillAlive.Count -gt 0) {
            Write-Host "  warning: $($StillAlive.Count) rust-analyzer process(es) survived the kill" -ForegroundColor Red
        }
    } elseif ($IgnoreRa) {
        Write-Host ""
        Write-Host "WARNING: rust-analyzer is running. Measurements WILL be noisy." -ForegroundColor Yellow
        Write-Host "         $($RaProcesses.Count) process(es) active. Proceeding because -IgnoreRa was set." -ForegroundColor Yellow
    } else {
        Write-Host ""
        Write-Host "ERROR: rust-analyzer is running — measurements would be contaminated." -ForegroundColor Red
        Write-Host "       Choose one:" -ForegroundColor Red
        Write-Host "         (a) Pause it in VSCode: Ctrl+Shift+P -> 'rust-analyzer: Stop server'" -ForegroundColor Red
        Write-Host "             and re-run the script." -ForegroundColor Red
        Write-Host "         (b) Re-run with -KillRa to stop rust-analyzer automatically." -ForegroundColor Red
        Write-Host "         (c) Re-run with -IgnoreRa to proceed anyway (noisy results)." -ForegroundColor Red
        exit 1
    }
}

# ── Environment capture ────────────────────────────────────────────────

$RustcVersion = (& rustc --version).Trim()
$CargoVersion = (& cargo --version).Trim()
$Cpu = (Get-CimInstance Win32_Processor | Select-Object -First 1).Name
$TotalRamGb = [math]::Round(((Get-CimInstance Win32_PhysicalMemory | Measure-Object -Property Capacity -Sum).Sum / 1GB), 0)
$OsVersion = (Get-CimInstance Win32_OperatingSystem).Caption

Write-Host ""
Write-Host "Environment:" -ForegroundColor Cyan
Write-Host "  os       : $OsVersion"
Write-Host "  cpu      : $Cpu"
Write-Host "  ram      : ${TotalRamGb} GB"
Write-Host "  rustc    : $RustcVersion"
Write-Host "  cargo    : $CargoVersion"

# ── Feature sets ────────────────────────────────────────────────────────

$AllFeatureSets = @(
    [PSCustomObject]@{ Name = 'default';                Args = @() }
    [PSCustomObject]@{ Name = 'ammoparams';             Args = @('--features', 'ammoparams') }
    [PSCustomObject]@{ Name = 'entities-scitem-ships';  Args = @('--features', 'entities-scitem-ships') }
    [PSCustomObject]@{ Name = 'full';                   Args = @('--features', 'full') }
    [PSCustomObject]@{ Name = 'dormant';                Args = @('--features', 'dormant') }
)

if ($Features.Count -gt 0) {
    $FeatureSets = $AllFeatureSets | Where-Object { $Features -contains $_.Name }
} else {
    $FeatureSets = $AllFeatureSets
}

# Runtime benchmarks are expensive (require full dev-opt build); default
# to a narrower set unless -Features was passed.
if ($Features.Count -gt 0) {
    $RuntimeFeatureSets = $FeatureSets
} else {
    $RuntimeFeatureSets = $AllFeatureSets | Where-Object { $_.Name -in @('default', 'full') }
}

# ── Helpers ─────────────────────────────────────────────────────────────

function Format-Duration {
    param([double]$Seconds)
    if ($Seconds -ge 60) {
        $Minutes = [math]::Floor($Seconds / 60)
        $Secs = $Seconds - ($Minutes * 60)
        return ('{0}m {1:F1}s' -f $Minutes, $Secs)
    } else {
        return ('{0:F2}s' -f $Seconds)
    }
}

function Format-DurationCell {
    param([double]$Seconds)
    if ($Seconds -le 0) { return '—' }
    return (Format-Duration $Seconds)
}

function Format-Mb {
    param([double]$Mb)
    if ($Mb -le 0) { return '—' }
    if ($Mb -ge 1024) {
        return ('{0:F2} GB' -f ($Mb / 1024))
    }
    return ('{0:N0} MB' -f $Mb)
}

function Invoke-Cargo {
    param(
        [string[]]$Arguments,
        [switch]$CaptureOutput
    )

    if ($CaptureOutput) {
        $output = & cargo @Arguments 2>&1 | Out-String
        return @{ Output = $output; ExitCode = $LASTEXITCODE; PeakMemoryMb = 0 }
    }

    if ($Quiet) {
        $output = & cargo @Arguments 2>&1
        # Show only Finished / error lines
        $output | Where-Object { $_ -match 'Finished|error\[|^error:' }
        return @{ ExitCode = $LASTEXITCODE; PeakMemoryMb = 0 }
    }

    & cargo @Arguments
    return @{ ExitCode = $LASTEXITCODE; PeakMemoryMb = 0 }
}

# Processes that count toward a cargo invocation's peak memory. Under
# -KillRa we've guaranteed there are no other rust-analyzer rustc
# processes competing, so this name-based snapshot accurately reflects
# the build we're measuring. Under -IgnoreRa it includes whatever
# rust-analyzer is also running — which matches the "noisy" contract.
#
# `parse_real_p4k` is the sc-extract example binary — included so
# runtime benchmarks catch its peak RSS too (the whole point of
# runtime memory sampling). If you add more examples or smoke-test
# binaries, add their names here.
$ToolchainProcessNames = @(
    'cargo',
    'rustc',
    'rustdoc',
    'link',
    'lld-link',
    'ld',
    'ld.lld',
    'parse_real_p4k'
)

function Invoke-CargoTimedWithMemory {
    param(
        [string[]]$Arguments,
        [switch]$CaptureOutput
    )

    # Runs cargo synchronously in the foreground and samples memory in
    # a background runspace. When the foreground call returns, cargo
    # has definitively exited — there's no process object to poll and
    # no liveness to track. This is deliberately dumber than earlier
    # revisions, which tried to spawn cargo asynchronously and track
    # its exit through .NET Process objects / Get-Process / etc. and
    # kept getting bitten by PS host quirks. Foreground blocking is
    # the only way we've found that's 100% reliable across PS 5.1 and
    # PS 7, Windows Terminal and conhost, bash-spawned and
    # interactive.
    #
    # The background runspace samples working set of all toolchain
    # processes (cargo, rustc, link, etc.) every 500 ms and records
    # the peak in a synchronized hashtable shared with the main
    # thread.
    $cargoExe = (Get-Command cargo).Path

    # Shared state for the background sampler. Synchronized hashtable
    # is the simplest way to pass mutable state between runspaces.
    $shared = [hashtable]::Synchronized(@{
        PeakBytes = [long]0
        Stop      = $false
    })

    # Start the background memory sampler. It reads toolchain process
    # names via a snapshot passed in as a variable. If creating the
    # runspace fails for any reason (unusual host restrictions, etc.)
    # we fall back to running cargo without memory measurement.
    $rs = $null
    $sampler = $null
    $samplerHandle = $null
    try {
        $rs = [runspacefactory]::CreateRunspace()
        $rs.Open()
        $rs.SessionStateProxy.SetVariable('shared', $shared)
        $rs.SessionStateProxy.SetVariable('names', $ToolchainProcessNames)

        $sampler = [powershell]::Create()
        $sampler.Runspace = $rs
        $null = $sampler.AddScript({
            $ErrorActionPreference = 'Continue'
            while (-not $shared.Stop) {
                $total = 0
                # Query each process name individually. PS 5.1's
                # multi-name `Get-Process -Name @('a','b',...)` silently
                # drops results when any name has no matches, which made
                # the reported peak come in at ~188 MB (cargo alone)
                # instead of ~1.8 GB (cargo + rustc) on some hosts.
                foreach ($n in $names) {
                    try {
                        $procs = Get-Process -Name $n -ErrorAction SilentlyContinue
                        if ($procs) {
                            $sum = ($procs | Measure-Object -Property WorkingSet64 -Sum).Sum
                            if ($sum) { $total += $sum }
                        }
                    } catch {}
                }
                if ($total -gt $shared.PeakBytes) { $shared.PeakBytes = $total }
                Start-Sleep -Milliseconds 500
            }
        })
        $samplerHandle = $sampler.BeginInvoke()
    } catch {
        Write-Host "warning: memory sampler failed to start, peak will be 0: $($_.Exception.Message)" -ForegroundColor Yellow
    }

    $output = ''
    $exitCode = -1
    $sw = [System.Diagnostics.Stopwatch]::StartNew()

    try {
        # Run cargo synchronously in the foreground. The subshell
        # locally sets $ErrorActionPreference = 'Continue' so cargo's
        # stderr output (status lines like "Removed N files", warning
        # lines, etc.) doesn't trip the script-level Stop preference.
        # Genuine PS throws and generic errors still propagate via the
        # subshell's return value.
        if ($CaptureOutput -or $Quiet) {
            # Capture full output into a single string for later parsing
            # (CaptureOutput) or filtering (Quiet).
            $output = & {
                $ErrorActionPreference = 'Continue'
                & $cargoExe @Arguments 2>&1 | Out-String
            }
            $exitCode = $LASTEXITCODE

            if ($Quiet -and -not $CaptureOutput) {
                # Show only the Finished line and any errors.
                ($output -split "`r?`n") | Where-Object { $_ -match 'Finished|error\[|^error:' } | ForEach-Object { Write-Host $_ }
            }
        } else {
            # Live passthrough — cargo's output streams directly to
            # the PS host as it happens. No capture, no buffering.
            & {
                $ErrorActionPreference = 'Continue'
                & $cargoExe @Arguments 2>&1
            }
            $exitCode = $LASTEXITCODE
        }
    } finally {
        $sw.Stop()
        # Stop the sampler and wait for it to finish. EndInvoke returns
        # once the script block has exited its loop; it should be
        # within one 500 ms tick of setting Stop = $true.
        if ($shared) { $shared.Stop = $true }
        if ($sampler -and $samplerHandle) {
            try { $null = $sampler.EndInvoke($samplerHandle) } catch {}
        }
        if ($sampler) { try { $sampler.Dispose() } catch {} }
        if ($rs) {
            try { $rs.Close() } catch {}
            try { $rs.Dispose() } catch {}
        }
    }

    $peakMb = [math]::Round($shared.PeakBytes / 1MB, 0)

    return @{
        ExitCode       = $exitCode
        PeakMemoryMb   = $peakMb
        ElapsedSeconds = $sw.Elapsed.TotalSeconds
        Output         = $output
    }
}

function Write-StepOutcome {
    param(
        [double]$ElapsedSeconds,
        [double]$PeakMemoryMb
    )
    Write-Host ("  → {0}, peak {1}" -f (Format-Duration $ElapsedSeconds), (Format-Mb $PeakMemoryMb)) -ForegroundColor Green
}

function Invoke-ColdCheck {
    param([PSCustomObject]$FeatureSet)

    Write-Host ""
    Write-Host "── Cold check: $($FeatureSet.Name) ──" -ForegroundColor Cyan

    # Clean only the two workspace crates that hold hand-written +
    # generated code. External deps (svarog, serde, zstd, regex) stay
    # cached between runs so we measure the part of the build that
    # actually changes with each feature set.
    #
    # Subshell so native-command stderr (cargo's "Removed N files"
    # status line) doesn't trip $ErrorActionPreference = 'Stop' on PS 5.1.
    & { $ErrorActionPreference = 'Continue'; & cargo clean -p sc-extract -p sc-extract-generated 2>&1 | Out-Null }

    $result = Invoke-CargoTimedWithMemory -Arguments (@('check', '-p', 'sc-extract') + $FeatureSet.Args)

    if ($result.ExitCode -ne 0) {
        throw "cargo check failed for $($FeatureSet.Name) (exit $($result.ExitCode))"
    }

    Write-StepOutcome -ElapsedSeconds $result.ElapsedSeconds -PeakMemoryMb $result.PeakMemoryMb

    return [PSCustomObject]@{
        Name           = $FeatureSet.Name
        ElapsedSeconds = $result.ElapsedSeconds
        PeakMemoryMb   = $result.PeakMemoryMb
    }
}

function Invoke-ColdBuild {
    param([PSCustomObject]$FeatureSet)

    Write-Host ""
    Write-Host "── Cold dev-opt build: $($FeatureSet.Name) ──" -ForegroundColor Cyan

    # Whole-profile clean so we get a true cold build, including the
    # sc-extract-generated artifacts sitting in target/dev-opt/ from
    # earlier sessions. `cargo clean -p` alone is not enough — the
    # dev-opt profile has its own subdirectory that persists across
    # dev-profile cleans.
    #
    # Subshell so native-command stderr doesn't trip $ErrorActionPreference.
    & { $ErrorActionPreference = 'Continue'; & cargo clean --profile dev-opt 2>&1 | Out-Null }

    $result = Invoke-CargoTimedWithMemory -Arguments (@('build', '-p', 'sc-extract', '--profile', 'dev-opt') + $FeatureSet.Args)

    if ($result.ExitCode -ne 0) {
        throw "cargo build failed for $($FeatureSet.Name) (exit $($result.ExitCode))"
    }

    Write-StepOutcome -ElapsedSeconds $result.ElapsedSeconds -PeakMemoryMb $result.PeakMemoryMb

    return [PSCustomObject]@{
        Name           = $FeatureSet.Name
        ElapsedSeconds = $result.ElapsedSeconds
        PeakMemoryMb   = $result.PeakMemoryMb
    }
}

function Invoke-RuntimeBench {
    param(
        [PSCustomObject]$FeatureSet,
        [switch]$WithGraph
    )

    $label = if ($WithGraph) { "$($FeatureSet.Name) + graph" } else { $FeatureSet.Name }
    Write-Host ""
    Write-Host "── Runtime: $label ──" -ForegroundColor Cyan

    # Ensure the example binary is built (dev-opt for fast iteration).
    # Not cold-measured — this is runtime benchmarking, not compile.
    $buildArgs = @('build', '-p', 'sc-extract', '--profile', 'dev-opt', '--example', 'parse_real_p4k') + $FeatureSet.Args
    $bresult = Invoke-Cargo -Arguments $buildArgs
    if ($bresult.ExitCode -ne 0) {
        throw "example build failed for $($FeatureSet.Name)"
    }

    $runArgs = @('run', '-p', 'sc-extract', '--profile', 'dev-opt', '--example', 'parse_real_p4k') + $FeatureSet.Args
    if ($WithGraph) {
        $runArgs += '--'
        $runArgs += '--all'
    }

    # Runtime path captures stdout to parse the summary and samples
    # memory to get peak runtime RSS of the example binary.
    $rresult = Invoke-CargoTimedWithMemory -Arguments $runArgs -CaptureOutput

    if ($rresult.ExitCode -ne 0) {
        Write-Host $rresult.Output
        throw "example run failed for $($FeatureSet.Name)"
    }

    $output = $rresult.Output

    # Parse the summary block from parse_real_p4k's stdout.
    function Match-One([string]$pattern) {
        $m = [regex]::Match($output, $pattern)
        if ($m.Success) { return $m.Groups[1].Value } else { return '' }
    }

    $records = Match-One 'records\s*:\s*(\d+)'
    $locale  = Match-One 'locale entries\s*:\s*(\d+)'
    $mans    = Match-One 'manufacturers\s*:\s*(\d+)'
    $names   = Match-One 'display names\s*:\s*(\d+)'
    $tags    = Match-One 'tag nodes\s*:\s*(\d+)'
    $edges   = Match-One 'graph edges\s*:\s*(\d+)'
    $pTime   = Match-One 'parse time\s*:\s*([\d.]+)s'
    $sSizeMb = Match-One 'size on disk\s*:\s*([\d.]+)'
    $sTime   = Match-One 'save time\s*:\s*([\d.]+)s'
    $lTime   = Match-One 'load time\s*:\s*([\d.]+)s'

    $result = [PSCustomObject]@{
        Name               = $label
        FeatureSet         = $FeatureSet.Name
        WithGraph          = [bool]$WithGraph
        WallClockSeconds   = $rresult.ElapsedSeconds
        PeakMemoryMb       = $rresult.PeakMemoryMb
        Records            = if ($records) { [int]$records } else { 0 }
        LocaleEntries      = if ($locale)  { [int]$locale  } else { 0 }
        Manufacturers      = if ($mans)    { [int]$mans    } else { 0 }
        DisplayNames       = if ($names)   { [int]$names   } else { 0 }
        TagNodes           = if ($tags)    { [int]$tags    } else { 0 }
        GraphEdges         = if ($edges)   { [int]$edges   } else { 0 }
        ParseSeconds       = if ($pTime)   { [double]$pTime } else { 0 }
        SnapshotSizeMb     = if ($sSizeMb) { [double]$sSizeMb } else { 0 }
        SaveSeconds        = if ($sTime)   { [double]$sTime } else { 0 }
        LoadSeconds        = if ($lTime)   { [double]$lTime } else { 0 }
    }

    Write-Host ("  records={0:N0} parse={1:F2}s snap={2:F2} MB load={3:F2}s peak={4}" -f `
        $result.Records, $result.ParseSeconds, $result.SnapshotSizeMb, $result.LoadSeconds, (Format-Mb $result.PeakMemoryMb)) -ForegroundColor Green

    return $result
}

# ── Persistence helpers (called after every step) ──────────────────────

function Write-JsonResults {
    param([PSCustomObject]$R, [string]$Path)
    $dir = Split-Path -Parent $Path
    if ($dir -and -not (Test-Path $dir)) {
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
    }
    $R | ConvertTo-Json -Depth 6 | Out-File -FilePath $Path -Encoding utf8
}

function Build-ResultsMarkdown {
    param([PSCustomObject]$R)

    $sb = [System.Text.StringBuilder]::new()
    [void]$sb.AppendLine('<!-- BENCH:RESULTS-START -->')
    [void]$sb.AppendLine('<!-- This block is auto-generated by tools/bench/bench.ps1. Edit the script, not the markdown, if you want to change the format. -->')
    [void]$sb.AppendLine()
    [void]$sb.AppendLine('## Latest results')
    [void]$sb.AppendLine()

    $raNote = if ($R.Environment.RaKilled) {
        'killed by script (-KillRa)'
    } elseif ($R.Environment.RaWasRunning) {
        '**was running** (-IgnoreRa — noisy)'
    } else {
        'not running'
    }
    [void]$sb.AppendLine(('*Last run: {0}. Mode: `{1}`. rust-analyzer: {2}.*' -f $R.Timestamp, $Mode, $raNote))
    [void]$sb.AppendLine()
    [void]$sb.AppendLine('Rows marked `(skipped)` or `(aborted)` were cut short by the operator. Re-run with `-Features <name> -Mode <mode>` to fill them in.')
    [void]$sb.AppendLine()

    [void]$sb.AppendLine('### Environment')
    [void]$sb.AppendLine()
    [void]$sb.AppendLine('| Field | Value |')
    [void]$sb.AppendLine('|---|---|')
    [void]$sb.AppendLine(('| os | {0} |' -f $R.Environment.Os))
    [void]$sb.AppendLine(('| cpu | {0} |' -f $R.Environment.Cpu))
    [void]$sb.AppendLine(('| ram | {0} GB |' -f $R.Environment.RamGb))
    [void]$sb.AppendLine(('| rustc | {0} |' -f $R.Environment.Rustc))
    [void]$sb.AppendLine(('| cargo | {0} |' -f $R.Environment.Cargo))
    [void]$sb.AppendLine()

    if ($R.CheckTimes.Count -gt 0) {
        [void]$sb.AppendLine('### Cold `cargo check`')
        [void]$sb.AppendLine()
        [void]$sb.AppendLine('| Features | Cold check | Peak RAM |')
        [void]$sb.AppendLine('|---|---:|---:|')
        foreach ($row in $R.CheckTimes) {
            $suffix = if ($row.Status -and $row.Status -ne 'Completed') { (" _({0})_" -f $row.Status.ToLower()) } else { '' }
            [void]$sb.AppendLine(('| `{0}` | {1}{2} | {3} |' -f `
                $row.Name, (Format-DurationCell $row.ElapsedSeconds), $suffix, (Format-Mb $row.PeakMemoryMb)))
        }
        [void]$sb.AppendLine()
    }

    if ($R.BuildTimes.Count -gt 0) {
        [void]$sb.AppendLine('### Cold `cargo build --profile dev-opt`')
        [void]$sb.AppendLine()
        [void]$sb.AppendLine('| Features | Cold build | Peak RAM |')
        [void]$sb.AppendLine('|---|---:|---:|')
        foreach ($row in $R.BuildTimes) {
            $suffix = if ($row.Status -and $row.Status -ne 'Completed') { (" _({0})_" -f $row.Status.ToLower()) } else { '' }
            [void]$sb.AppendLine(('| `{0}` | {1}{2} | {3} |' -f `
                $row.Name, (Format-DurationCell $row.ElapsedSeconds), $suffix, (Format-Mb $row.PeakMemoryMb)))
        }
        [void]$sb.AppendLine()
    }

    if ($R.Runtime.Count -gt 0) {
        $standard = $R.Runtime | Where-Object { -not $_.WithGraph }
        $withGraph = $R.Runtime | Where-Object { $_.WithGraph }

        if ($standard.Count -gt 0) {
            [void]$sb.AppendLine('### Runtime `parse_real_p4k` — `DatacoreConfig::standard()`')
            [void]$sb.AppendLine()
            [void]$sb.AppendLine('| Features | Records | Locale | Display names | Parse | Snapshot | Save | Load | Peak RAM |')
            [void]$sb.AppendLine('|---|---:|---:|---:|---:|---:|---:|---:|---:|')
            foreach ($row in $standard) {
                if ($row.Status -and $row.Status -ne 'Completed') {
                    [void]$sb.AppendLine(('| `{0}` | — | — | — | — _({1})_ | — | — | — | {2} |' -f `
                        $row.FeatureSet, $row.Status.ToLower(), (Format-Mb $row.PeakMemoryMb)))
                } else {
                    [void]$sb.AppendLine(('| `{0}` | {1:N0} | {2:N0} | {3:N0} | {4} | {5:F2} MB | {6} | {7} | {8} |' -f `
                        $row.FeatureSet, $row.Records, $row.LocaleEntries, $row.DisplayNames, `
                        (Format-DurationCell $row.ParseSeconds), $row.SnapshotSizeMb, `
                        (Format-DurationCell $row.SaveSeconds), (Format-DurationCell $row.LoadSeconds), `
                        (Format-Mb $row.PeakMemoryMb)))
                }
            }
            [void]$sb.AppendLine()
        }

        if ($withGraph.Count -gt 0) {
            [void]$sb.AppendLine('### Runtime `parse_real_p4k` — `DatacoreConfig::all()` (reference graph on)')
            [void]$sb.AppendLine()
            [void]$sb.AppendLine('| Features | Records | Graph edges | Parse | Snapshot | Save | Load | Peak RAM |')
            [void]$sb.AppendLine('|---|---:|---:|---:|---:|---:|---:|---:|')
            foreach ($row in $withGraph) {
                if ($row.Status -and $row.Status -ne 'Completed') {
                    [void]$sb.AppendLine(('| `{0}` | — | — | — _({1})_ | — | — | — | {2} |' -f `
                        $row.FeatureSet, $row.Status.ToLower(), (Format-Mb $row.PeakMemoryMb)))
                } else {
                    [void]$sb.AppendLine(('| `{0}` | {1:N0} | {2:N0} | {3} | {4:F2} MB | {5} | {6} | {7} |' -f `
                        $row.FeatureSet, $row.Records, $row.GraphEdges, `
                        (Format-DurationCell $row.ParseSeconds), $row.SnapshotSizeMb, `
                        (Format-DurationCell $row.SaveSeconds), (Format-DurationCell $row.LoadSeconds), `
                        (Format-Mb $row.PeakMemoryMb)))
                }
            }
            [void]$sb.AppendLine()
        }
    }

    [void]$sb.AppendLine('<!-- BENCH:RESULTS-END -->')
    return $sb.ToString()
}

function Update-BenchmarkDoc {
    param([PSCustomObject]$R, [string]$Path)

    if (-not (Test-Path $Path)) {
        Write-Host ""
        Write-Host "WARNING: benchmark document not found: $Path" -ForegroundColor Yellow
        Write-Host "         create it with the BENCH:RESULTS-START/END markers to enable auto-update." -ForegroundColor Yellow
        return
    }

    $resultsBlock = Build-ResultsMarkdown -R $R
    $docBody = Get-Content -Raw -Path $Path

    $startMarker = '<!-- BENCH:RESULTS-START -->'
    $endMarker   = '<!-- BENCH:RESULTS-END -->'

    $startIdx = $docBody.IndexOf($startMarker)
    $endIdx   = $docBody.IndexOf($endMarker)

    if ($startIdx -lt 0 -or $endIdx -lt 0 -or $endIdx -le $startIdx) {
        Write-Host ""
        Write-Host "WARNING: could not find BENCH:RESULTS-START/END markers in $Path" -ForegroundColor Yellow
        Write-Host "         skipping markdown update." -ForegroundColor Yellow
        return
    }

    $endIdx += $endMarker.Length
    $newDoc = $docBody.Substring(0, $startIdx) + $resultsBlock.TrimEnd() + $docBody.Substring($endIdx)
    Set-Content -Path $Path -Value $newDoc -Encoding utf8 -NoNewline
}

function Persist-Results {
    param([PSCustomObject]$R)
    Write-JsonResults -R $R -Path $OutJson
    if (-not $NoDocWrite) {
        Update-BenchmarkDoc -R $R -Path $BenchDoc
    }
}

# ── Run benchmarks ──────────────────────────────────────────────────────

$Results = [PSCustomObject]@{
    Timestamp    = (Get-Date).ToString('o')
    Environment  = [PSCustomObject]@{
        Os           = $OsVersion
        Cpu          = $Cpu
        RamGb        = $TotalRamGb
        Rustc        = $RustcVersion
        Cargo        = $CargoVersion
        RaWasRunning = ($RaProcesses.Count -gt 0)
        RaKilled     = ($RaProcesses.Count -gt 0 -and $KillRa)
    }
    CheckTimes   = @()
    BuildTimes   = @()
    Runtime      = @()
}

Write-Host ""
Write-Host "Results are written incrementally after every step, so if" -ForegroundColor DarkCyan
Write-Host "the script is killed mid-run, whatever steps had completed" -ForegroundColor DarkCyan
Write-Host "are already on disk. Ctrl+C is safe." -ForegroundColor DarkCyan

$doCheck   = $Mode -in @('all', 'check')
$doBuild   = $Mode -in @('all', 'build')
$doRuntime = $Mode -in @('all', 'runtime')

if ($doCheck) {
    Write-Host ""
    Write-Host "══ Cold cargo check ═══════════════════════════" -ForegroundColor Cyan
    foreach ($fs in $FeatureSets) {
        $Results.CheckTimes += Invoke-ColdCheck -FeatureSet $fs
        Persist-Results -R $Results
    }
}

if ($doBuild) {
    Write-Host ""
    Write-Host "══ Cold dev-opt build ═════════════════════════" -ForegroundColor Cyan
    foreach ($fs in $FeatureSets) {
        $Results.BuildTimes += Invoke-ColdBuild -FeatureSet $fs
        Persist-Results -R $Results
    }
}

if ($doRuntime) {
    Write-Host ""
    Write-Host "══ Runtime parse_real_p4k ═════════════════════" -ForegroundColor Cyan
    foreach ($fs in $RuntimeFeatureSets) {
        $Results.Runtime += Invoke-RuntimeBench -FeatureSet $fs
        Persist-Results -R $Results
        $Results.Runtime += Invoke-RuntimeBench -FeatureSet $fs -WithGraph
        Persist-Results -R $Results
    }
}

# ── Final persistence + summary ────────────────────────────────────────

# The loops above already call Persist-Results after each step, so this
# is redundant on a clean exit — but it's cheap insurance against a
# future bug that skips incremental persistence on the last step.
Persist-Results -R $Results

Write-Host ""
Write-Host "══ Summary ════════════════════════════════════" -ForegroundColor Cyan

function Write-StepRow {
    param([PSCustomObject]$R)
    '  {0,-24}  {1,10}  peak {2,10}' -f `
        $R.Name, (Format-Duration $R.ElapsedSeconds), (Format-Mb $R.PeakMemoryMb) | Write-Host
}

if ($Results.CheckTimes.Count -gt 0) {
    Write-Host ""
    Write-Host "Cold cargo check:" -ForegroundColor Yellow
    foreach ($r in $Results.CheckTimes) { Write-StepRow -R $r }
}

if ($Results.BuildTimes.Count -gt 0) {
    Write-Host ""
    Write-Host "Cold dev-opt build:" -ForegroundColor Yellow
    foreach ($r in $Results.BuildTimes) { Write-StepRow -R $r }
}

if ($Results.Runtime.Count -gt 0) {
    Write-Host ""
    Write-Host "Runtime parse_real_p4k:" -ForegroundColor Yellow
    foreach ($r in $Results.Runtime) {
        '  {0,-24}  records={1,7:N0} parse={2,7:F2}s snap={3,7:F2} MB load={4,6:F2}s peak={5,10}' -f `
            $r.Name, $r.Records, $r.ParseSeconds, $r.SnapshotSizeMb, $r.LoadSeconds, (Format-Mb $r.PeakMemoryMb) | Write-Host
    }
}

Write-Host ""
Write-Host "Done." -ForegroundColor Cyan
Write-Host ("  raw JSON : {0}" -f $OutJson) -ForegroundColor DarkGray
if (-not $NoDocWrite) {
    Write-Host ("  markdown : {0}" -f $BenchDoc) -ForegroundColor DarkGray
}
Write-Host ""
