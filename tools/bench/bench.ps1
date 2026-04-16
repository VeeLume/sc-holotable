<#
.SYNOPSIS
    Repeatable compile-time and runtime benchmarks for sc-extract.

.DESCRIPTION
    Runs cold `cargo check` / `cargo build` / runtime `sc-bench`
    benchmarks against a configurable set of Cargo feature configurations
    and release profiles. Writes results to target/bench-results.json,
    saves timestamped history to target/bench-history/, and updates
    docs/benchmarks.md (auto-archiving the previous run into the
    History section).

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
      build      Cold `cargo build` per profile + feature set
      runtime    Runtime `sc-bench` per profile + feature set
      all        Everything (check + build + runtime, chained)

    Defaults to 'all'. When 'all' is selected, build and runtime are
    chained per profile x feature: cold build -> runtime -> runtime +graph,
    so the freshly-built binary is reused without rebuilding.

.PARAMETER Features
    Which feature sets to benchmark. Any of:
        default, ammoparams, entities-scitem-ships, full, dormant
    Omit to use the full set for check/build, and default+full for runtime.

.PARAMETER Profiles
    Which Cargo profiles to benchmark for build and runtime modes. Each
    must be a [profile.XXX] section in Cargo.toml. Defaults to 'release'.
    Example: -Profiles release,bench-thin,bench-cu1

.PARAMETER KillRa
    Stop rust-analyzer.exe and rust-analyzer-proc-macro-srv.exe before
    measuring. VSCode will restart them when it next polls, so do NOT
    switch back to VSCode until the script is done.

.PARAMETER IgnoreRa
    Don't error out if rust-analyzer is running. Measurements will be
    noisy. Used mostly for smoke-testing the script itself.

.PARAMETER Quiet
    Suppress live cargo stdout/stderr; show only timing results.

.PARAMETER TestControls
    Run a dummy polling loop to verify interactive skip/abort keypresses
    work on your terminal. Does not run any cargo commands.

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

.PARAMETER HistoryDir
    Directory for timestamped JSON history files. Defaults to
    target/bench-history under the workspace root.

.PARAMETER NoDocWrite
    Skip updating the benchmarks markdown document. Results still go
    to target/bench-results.json, history, and stdout.

.PARAMETER NoAutoArchive
    Skip auto-archiving the previous run's results into the History
    section of benchmarks.md. Use when re-running the same config and
    you don't want duplicate history entries.

.EXAMPLE
    # Full suite, clean measurement (stops RA)
    .\tools\bench\bench.ps1 -Mode all -KillRa

.EXAMPLE
    # Compare two profiles on one feature set
    .\tools\bench\bench.ps1 -Mode build -Profiles release,bench-thin -Features entities-scitem-ships -KillRa

.EXAMPLE
    # Runtime parse benchmarks for a single feature set
    .\tools\bench\bench.ps1 -Mode runtime -Features full -KillRa

.EXAMPLE
    # Test interactive controls
    .\tools\bench\bench.ps1 -TestControls

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

    [string[]]$Profiles = @('release'),

    [switch]$KillRa,
    [switch]$IgnoreRa,
    [switch]$Quiet,
    [switch]$TestControls,

    [string]$OutJson = '',
    [string]$BenchDoc = '',
    [string]$HistoryDir = '',
    [switch]$NoDocWrite,
    [switch]$NoAutoArchive
)

$ErrorActionPreference = 'Stop'

# Force invariant / en-US number formatting so "1879" prints as
# "1,879 MB" / "1.84 GB", not "1.879 MB" / "1,84 GB" on de-DE hosts.
[System.Threading.Thread]::CurrentThread.CurrentCulture = 'en-US'

# Script-level abort flag set by the 'a' key handler.
$script:Aborted = $false

# ── Test controls mode ─────────────────────────────────────────────────

if ($TestControls) {
    Write-Host ""
    Write-Host "Interactive controls test" -ForegroundColor Cyan
    Write-Host "Press 's' to test skip, 'a' to test abort and exit."
    Write-Host "Heartbeat prints every ~5 seconds. Ctrl+C to force quit."
    Write-Host ""
    $canReadKey = $true
    try { [void][Console]::KeyAvailable } catch { $canReadKey = $false }
    if (-not $canReadKey) {
        Write-Host "WARNING: [Console]::KeyAvailable not supported on this host." -ForegroundColor Yellow
        Write-Host "Interactive skip/abort will not work. Use Ctrl+C instead." -ForegroundColor Yellow
        exit 0
    }
    $heartbeat = 0
    while ($true) {
        if ([Console]::KeyAvailable) {
            $key = [Console]::ReadKey($true)
            if ($key.KeyChar -eq 's') {
                Write-Host "  detected: skip" -ForegroundColor Green
            } elseif ($key.KeyChar -eq 'a') {
                Write-Host "  detected: abort — exiting" -ForegroundColor Yellow
                exit 0
            } else {
                Write-Host ("  [key '{0}' ignored — press 's' or 'a']" -f $key.KeyChar) -ForegroundColor DarkGray
            }
        }
        Start-Sleep -Milliseconds 500
        $heartbeat++
        if ($heartbeat % 10 -eq 0) {
            Write-Host ("  heartbeat: {0}s" -f ($heartbeat / 2)) -ForegroundColor DarkGray
        }
    }
}

# ── Workspace root ──────────────────────────────────────────────────────

$WorkspaceRoot = (Resolve-Path "$PSScriptRoot\..\..").Path
Set-Location $WorkspaceRoot

if (-not $OutJson) {
    $OutJson = Join-Path $WorkspaceRoot 'target\bench-results.json'
}
if (-not $BenchDoc) {
    $BenchDoc = Join-Path $WorkspaceRoot 'docs\benchmarks.md'
}
if (-not $HistoryDir) {
    $HistoryDir = Join-Path $WorkspaceRoot 'target\bench-history'
}

Write-Host ""
Write-Host "sc-holotable benchmark" -ForegroundColor Cyan
Write-Host "  workspace: $WorkspaceRoot"
Write-Host "  mode     : $Mode"
Write-Host "  profiles : $($Profiles -join ', ')"

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

# Runtime benchmarks are expensive (require full release build); default
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

# Detect whether [Console]::KeyAvailable works on this host.
$script:CanReadKey = $true
try { [void][Console]::KeyAvailable } catch { $script:CanReadKey = $false }
if (-not $script:CanReadKey) {
    Write-Host ""
    Write-Host "NOTE: interactive skip/abort not available on this terminal." -ForegroundColor DarkYellow
    Write-Host "      Use Ctrl+C to interrupt. Results are saved incrementally." -ForegroundColor DarkYellow
}

# Processes that count toward a cargo invocation's peak memory.
$ToolchainProcessNames = @(
    'cargo',
    'rustc',
    'rustdoc',
    'link',
    'lld-link',
    'ld',
    'ld.lld',
    'sc-bench'
)

function Invoke-CargoTimedWithMemory {
    param(
        [string[]]$Arguments,
        [switch]$CaptureOutput
    )

    # Spawn cargo as a System.Diagnostics.Process and poll in a loop for:
    # - peak memory sampling (every 500ms)
    # - interactive keypress (skip / abort)
    # Falls back to synchronous execution if Process spawning fails.

    $cargoExe = (Get-Command cargo).Path
    $peakBytes = [long]0
    $status = 'Completed'

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $cargoExe
    $psi.WorkingDirectory = $WorkspaceRoot
    $psi.Arguments = ($Arguments | ForEach-Object {
        if ($_ -match '\s') { "`"$_`"" } else { $_ }
    }) -join ' '
    $psi.UseShellExecute = $false
    $psi.RedirectStandardInput = $true
    if ($CaptureOutput -or $Quiet) {
        $psi.RedirectStandardOutput = $true
        $psi.RedirectStandardError = $true
    }

    $proc = New-Object System.Diagnostics.Process
    $proc.StartInfo = $psi

    $outputLines = [System.Collections.Generic.List[string]]::new()
    $errorLines = [System.Collections.Generic.List[string]]::new()

    if ($CaptureOutput -or $Quiet) {
        # Register async handlers to avoid deadlocks on large output.
        $outHandler = {
            param($sender, $e)
            if ($e.Data -ne $null) { $outputLines.Add($e.Data) }
        }
        $errHandler = {
            param($sender, $e)
            if ($e.Data -ne $null) { $errorLines.Add($e.Data) }
        }
        $proc.add_OutputDataReceived($outHandler)
        $proc.add_ErrorDataReceived($errHandler)
    }

    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $proc.Start() | Out-Null
    # Close stdin so cargo doesn't try to read from it.
    $proc.StandardInput.Close()

    if ($CaptureOutput -or $Quiet) {
        $proc.BeginOutputReadLine()
        $proc.BeginErrorReadLine()
    }

    # Polling loop: memory sampling + interactive controls.
    while (-not $proc.HasExited) {
        # Memory sampling — sum WorkingSet64 across all toolchain procs.
        $total = [long]0
        foreach ($n in $ToolchainProcessNames) {
            try {
                $procs = Get-Process -Name $n -ErrorAction SilentlyContinue
                if ($procs) {
                    $sum = ($procs | Measure-Object -Property WorkingSet64 -Sum).Sum
                    if ($sum) { $total += $sum }
                }
            } catch {}
        }
        if ($total -gt $peakBytes) { $peakBytes = $total }

        # Interactive controls.
        if ($script:CanReadKey) {
            try {
                while ([Console]::KeyAvailable) {
                    $key = [Console]::ReadKey($true)
                    if ($key.KeyChar -eq 's') {
                        Write-Host "  [skip requested — killing cargo]" -ForegroundColor Yellow
                        try {
                            $proc.Kill($true)
                        } catch {
                            try { $proc.Kill() } catch {}
                        }
                        $status = 'Skipped'
                        break
                    } elseif ($key.KeyChar -eq 'a') {
                        Write-Host "  [abort requested — killing cargo]" -ForegroundColor Yellow
                        try {
                            $proc.Kill($true)
                        } catch {
                            try { $proc.Kill() } catch {}
                        }
                        $status = 'Aborted'
                        $script:Aborted = $true
                        break
                    }
                }
            } catch {}
        }

        if ($status -ne 'Completed') { break }
        Start-Sleep -Milliseconds 500
    }

    $sw.Stop()

    # Wait for the process to fully exit (collect exit code).
    if (-not $proc.HasExited) {
        try { $proc.WaitForExit(5000) | Out-Null } catch {}
    }
    $exitCode = if ($proc.HasExited) { $proc.ExitCode } else { -1 }

    if ($CaptureOutput -or $Quiet) {
        # Give async handlers a moment to flush.
        try { $proc.WaitForExit() } catch {}
    }

    $proc.Dispose()

    $peakMb = [math]::Round($peakBytes / 1MB, 0)

    $output = ($outputLines -join "`n")
    if ($Quiet -and -not $CaptureOutput) {
        # Show only Finished / error lines.
        $outputLines | Where-Object { $_ -match 'Finished|error\[|^error:' } | ForEach-Object { Write-Host $_ }
        $errorLines | Where-Object { $_ -match 'Finished|error\[|^error:' } | ForEach-Object { Write-Host $_ }
    }

    return @{
        ExitCode       = $exitCode
        PeakMemoryMb   = $peakMb
        ElapsedSeconds = $sw.Elapsed.TotalSeconds
        Output         = $output
        Status         = $status
    }
}

function Write-StepOutcome {
    param(
        [double]$ElapsedSeconds,
        [double]$PeakMemoryMb,
        [string]$Status = 'Completed'
    )
    $suffix = if ($Status -ne 'Completed') { " ($Status)" } else { '' }
    Write-Host ("  -> {0}, peak {1}{2}" -f (Format-Duration $ElapsedSeconds), (Format-Mb $PeakMemoryMb), $suffix) -ForegroundColor Green
}

# ── Benchmark steps ────────────────────────────────────────────────────

function Invoke-ColdCheck {
    param([PSCustomObject]$FeatureSet)

    Write-Host ""
    Write-Host "-- Cold check: $($FeatureSet.Name) --" -ForegroundColor Cyan

    & { $ErrorActionPreference = 'Continue'; & cargo clean -p sc-extract -p sc-extract-generated 2>&1 | Out-Null }

    $result = Invoke-CargoTimedWithMemory -Arguments (@('check', '-p', 'sc-extract') + $FeatureSet.Args)

    if ($result.Status -eq 'Completed' -and $result.ExitCode -ne 0) {
        throw "cargo check failed for $($FeatureSet.Name) (exit $($result.ExitCode))"
    }

    Write-StepOutcome -ElapsedSeconds $result.ElapsedSeconds -PeakMemoryMb $result.PeakMemoryMb -Status $result.Status

    return [PSCustomObject]@{
        Name           = $FeatureSet.Name
        Profile        = 'dev'
        ElapsedSeconds = $result.ElapsedSeconds
        PeakMemoryMb   = $result.PeakMemoryMb
        Status         = $result.Status
    }
}

function Invoke-ColdBuild {
    param(
        [PSCustomObject]$FeatureSet,
        [string]$ProfileName = 'release'
    )

    Write-Host ""
    Write-Host "-- Cold build ($ProfileName): $($FeatureSet.Name) --" -ForegroundColor Cyan

    & { $ErrorActionPreference = 'Continue'; & cargo clean --profile $ProfileName 2>&1 | Out-Null }

    $result = Invoke-CargoTimedWithMemory -Arguments (@('build', '-p', 'sc-bench', '--profile', $ProfileName) + $FeatureSet.Args)

    if ($result.Status -eq 'Completed' -and $result.ExitCode -ne 0) {
        throw "cargo build failed for $($FeatureSet.Name) / $ProfileName (exit $($result.ExitCode))"
    }

    Write-StepOutcome -ElapsedSeconds $result.ElapsedSeconds -PeakMemoryMb $result.PeakMemoryMb -Status $result.Status

    return [PSCustomObject]@{
        Name           = $FeatureSet.Name
        Profile        = $ProfileName
        ElapsedSeconds = $result.ElapsedSeconds
        PeakMemoryMb   = $result.PeakMemoryMb
        Status         = $result.Status
    }
}

function Invoke-RuntimeBench {
    param(
        [PSCustomObject]$FeatureSet,
        [string]$ProfileName = 'release',
        [switch]$WithGraph,
        [switch]$SkipBuild
    )

    $label = if ($WithGraph) { "$($FeatureSet.Name) + graph" } else { $FeatureSet.Name }
    Write-Host ""
    Write-Host "-- Runtime ($ProfileName): $label --" -ForegroundColor Cyan

    # Build if not chained from a cold build.
    if (-not $SkipBuild) {
        $buildArgs = @('build', '-p', 'sc-bench', '--profile', $ProfileName) + $FeatureSet.Args
        $bresult = Invoke-CargoTimedWithMemory -Arguments $buildArgs
        if ($bresult.ExitCode -ne 0) {
            throw "sc-bench build failed for $($FeatureSet.Name) / $ProfileName"
        }
    }

    $runArgs = @('run', '-p', 'sc-bench', '--profile', $ProfileName) + $FeatureSet.Args
    if ($WithGraph) {
        $runArgs += '--'
        $runArgs += '--all'
    }

    $rresult = Invoke-CargoTimedWithMemory -Arguments $runArgs -CaptureOutput

    if ($rresult.Status -ne 'Completed') {
        Write-StepOutcome -ElapsedSeconds $rresult.ElapsedSeconds -PeakMemoryMb $rresult.PeakMemoryMb -Status $rresult.Status
        return [PSCustomObject]@{
            Name               = $label
            FeatureSet         = $FeatureSet.Name
            Profile            = $ProfileName
            WithGraph          = [bool]$WithGraph
            WallClockSeconds   = $rresult.ElapsedSeconds
            PeakMemoryMb       = $rresult.PeakMemoryMb
            Status             = $rresult.Status
            Records            = 0
            LocaleEntries      = 0
            Manufacturers      = 0
            DisplayNames       = 0
            TagNodes           = 0
            GraphEdges         = 0
            ParseSeconds       = 0
            SnapshotSizeMb     = 0
            SaveSeconds        = 0
            LoadSeconds        = 0
        }
    }

    if ($rresult.ExitCode -ne 0) {
        Write-Host $rresult.Output
        throw "sc-bench run failed for $($FeatureSet.Name) / $ProfileName"
    }

    # Parse JSON output from sc-bench.
    $json = $null
    try {
        $json = $rresult.Output | ConvertFrom-Json
    } catch {
        Write-Host "WARNING: failed to parse sc-bench JSON output" -ForegroundColor Yellow
        Write-Host $rresult.Output
        throw "sc-bench JSON parse failed for $($FeatureSet.Name) / $ProfileName"
    }

    $result = [PSCustomObject]@{
        Name               = $label
        FeatureSet         = $FeatureSet.Name
        Profile            = $ProfileName
        WithGraph          = [bool]$WithGraph
        WallClockSeconds   = $rresult.ElapsedSeconds
        PeakMemoryMb       = $rresult.PeakMemoryMb
        Status             = 'Completed'
        Records            = if ($json.records) { [int]$json.records } else { 0 }
        LocaleEntries      = if ($json.locale_entries) { [int]$json.locale_entries } else { 0 }
        Manufacturers      = if ($json.manufacturers) { [int]$json.manufacturers } else { 0 }
        DisplayNames       = if ($json.display_names) { [int]$json.display_names } else { 0 }
        TagNodes           = if ($json.tag_nodes) { [int]$json.tag_nodes } else { 0 }
        GraphEdges         = if ($json.graph_edges) { [int]$json.graph_edges } else { 0 }
        ParseSeconds       = if ($json.parse_time) { [double]$json.parse_time } else { 0 }
        SnapshotSizeMb     = if ($json.snapshot_size_mb) { [double]$json.snapshot_size_mb } else { 0 }
        SaveSeconds        = if ($json.snapshot_save_time) { [double]$json.snapshot_save_time } else { 0 }
        LoadSeconds        = if ($json.snapshot_load_time) { [double]$json.snapshot_load_time } else { 0 }
    }

    Write-Host ("  records={0:N0} parse={1:F2}s snap={2:F2} MB load={3:F2}s peak={4}" -f `
        $result.Records, $result.ParseSeconds, $result.SnapshotSizeMb, $result.LoadSeconds, (Format-Mb $result.PeakMemoryMb)) -ForegroundColor Green

    return $result
}

# ── Persistence helpers ────────────────────────────────────────────────

function Write-JsonResults {
    param([PSCustomObject]$R, [string]$Path)
    $dir = Split-Path -Parent $Path
    if ($dir -and -not (Test-Path $dir)) {
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
    }
    $R | ConvertTo-Json -Depth 6 | Out-File -FilePath $Path -Encoding utf8
}

function Write-HistoryEntry {
    param([PSCustomObject]$R, [string]$Dir)
    if (-not (Test-Path $Dir)) {
        New-Item -ItemType Directory -Force -Path $Dir | Out-Null
    }
    $ts = (Get-Date).ToString('yyyy-MM-ddTHH-mm-ss')
    $dest = Join-Path $Dir "$ts.json"
    $R | ConvertTo-Json -Depth 6 | Out-File -FilePath $dest -Encoding utf8
}

function Build-ResultsMarkdown {
    param([PSCustomObject]$R)

    $multiProfile = ($R.Profiles.Count -gt 1)

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
    $profileList = ($R.Profiles -join ', ')
    [void]$sb.AppendLine(('*Last run: {0}. Mode: `{1}`. Profiles: `{2}`. rust-analyzer: {3}.*' -f $R.Timestamp, $Mode, $profileList, $raNote))
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
        if ($multiProfile) {
            [void]$sb.AppendLine('### Cold `cargo build`')
            [void]$sb.AppendLine()
            [void]$sb.AppendLine('| Profile | Features | Cold build | Peak RAM |')
            [void]$sb.AppendLine('|---|---|---:|---:|')
        } else {
            [void]$sb.AppendLine(('### Cold `cargo build --profile {0}`' -f $R.Profiles[0]))
            [void]$sb.AppendLine()
            [void]$sb.AppendLine('| Features | Cold build | Peak RAM |')
            [void]$sb.AppendLine('|---|---:|---:|')
        }
        foreach ($row in $R.BuildTimes) {
            $suffix = if ($row.Status -and $row.Status -ne 'Completed') { (" _({0})_" -f $row.Status.ToLower()) } else { '' }
            if ($multiProfile) {
                [void]$sb.AppendLine(('| `{0}` | `{1}` | {2}{3} | {4} |' -f `
                    $row.Profile, $row.Name, (Format-DurationCell $row.ElapsedSeconds), $suffix, (Format-Mb $row.PeakMemoryMb)))
            } else {
                [void]$sb.AppendLine(('| `{0}` | {1}{2} | {3} |' -f `
                    $row.Name, (Format-DurationCell $row.ElapsedSeconds), $suffix, (Format-Mb $row.PeakMemoryMb)))
            }
        }
        [void]$sb.AppendLine()
    }

    if ($R.Runtime.Count -gt 0) {
        $standard = $R.Runtime | Where-Object { -not $_.WithGraph }
        $withGraph = $R.Runtime | Where-Object { $_.WithGraph }

        if ($standard.Count -gt 0) {
            [void]$sb.AppendLine('### Runtime `sc-bench` — standard')
            [void]$sb.AppendLine()
            if ($multiProfile) {
                [void]$sb.AppendLine('| Profile | Features | Records | Locale | Display names | Parse | Snapshot | Save | Load | Peak RAM |')
                [void]$sb.AppendLine('|---|---|---:|---:|---:|---:|---:|---:|---:|---:|')
            } else {
                [void]$sb.AppendLine('| Features | Records | Locale | Display names | Parse | Snapshot | Save | Load | Peak RAM |')
                [void]$sb.AppendLine('|---|---:|---:|---:|---:|---:|---:|---:|---:|')
            }
            foreach ($row in $standard) {
                if ($row.Status -and $row.Status -ne 'Completed') {
                    if ($multiProfile) {
                        [void]$sb.AppendLine(('| `{0}` | `{1}` | — | — | — | — _({2})_ | — | — | — | {3} |' -f `
                            $row.Profile, $row.FeatureSet, $row.Status.ToLower(), (Format-Mb $row.PeakMemoryMb)))
                    } else {
                        [void]$sb.AppendLine(('| `{0}` | — | — | — | — _({1})_ | — | — | — | {2} |' -f `
                            $row.FeatureSet, $row.Status.ToLower(), (Format-Mb $row.PeakMemoryMb)))
                    }
                } else {
                    if ($multiProfile) {
                        [void]$sb.AppendLine(('| `{0}` | `{1}` | {2:N0} | {3:N0} | {4:N0} | {5} | {6:F2} MB | {7} | {8} | {9} |' -f `
                            $row.Profile, $row.FeatureSet, $row.Records, $row.LocaleEntries, $row.DisplayNames, `
                            (Format-DurationCell $row.ParseSeconds), $row.SnapshotSizeMb, `
                            (Format-DurationCell $row.SaveSeconds), (Format-DurationCell $row.LoadSeconds), `
                            (Format-Mb $row.PeakMemoryMb)))
                    } else {
                        [void]$sb.AppendLine(('| `{0}` | {1:N0} | {2:N0} | {3:N0} | {4} | {5:F2} MB | {6} | {7} | {8} |' -f `
                            $row.FeatureSet, $row.Records, $row.LocaleEntries, $row.DisplayNames, `
                            (Format-DurationCell $row.ParseSeconds), $row.SnapshotSizeMb, `
                            (Format-DurationCell $row.SaveSeconds), (Format-DurationCell $row.LoadSeconds), `
                            (Format-Mb $row.PeakMemoryMb)))
                    }
                }
            }
            [void]$sb.AppendLine()
        }

        if ($withGraph.Count -gt 0) {
            [void]$sb.AppendLine('### Runtime `sc-bench` — with reference graph')
            [void]$sb.AppendLine()
            if ($multiProfile) {
                [void]$sb.AppendLine('| Profile | Features | Records | Graph edges | Parse | Snapshot | Save | Load | Peak RAM |')
                [void]$sb.AppendLine('|---|---|---:|---:|---:|---:|---:|---:|---:|')
            } else {
                [void]$sb.AppendLine('| Features | Records | Graph edges | Parse | Snapshot | Save | Load | Peak RAM |')
                [void]$sb.AppendLine('|---|---:|---:|---:|---:|---:|---:|---:|')
            }
            foreach ($row in $withGraph) {
                if ($row.Status -and $row.Status -ne 'Completed') {
                    if ($multiProfile) {
                        [void]$sb.AppendLine(('| `{0}` | `{1}` | — | — | — _({2})_ | — | — | — | {3} |' -f `
                            $row.Profile, $row.FeatureSet, $row.Status.ToLower(), (Format-Mb $row.PeakMemoryMb)))
                    } else {
                        [void]$sb.AppendLine(('| `{0}` | — | — | — _({1})_ | — | — | — | {2} |' -f `
                            $row.FeatureSet, $row.Status.ToLower(), (Format-Mb $row.PeakMemoryMb)))
                    }
                } else {
                    if ($multiProfile) {
                        [void]$sb.AppendLine(('| `{0}` | `{1}` | {2:N0} | {3:N0} | {4} | {5:F2} MB | {6} | {7} | {8} |' -f `
                            $row.Profile, $row.FeatureSet, $row.Records, $row.GraphEdges, `
                            (Format-DurationCell $row.ParseSeconds), $row.SnapshotSizeMb, `
                            (Format-DurationCell $row.SaveSeconds), (Format-DurationCell $row.LoadSeconds), `
                            (Format-Mb $row.PeakMemoryMb)))
                    } else {
                        [void]$sb.AppendLine(('| `{0}` | {1:N0} | {2:N0} | {3} | {4:F2} MB | {5} | {6} | {7} |' -f `
                            $row.FeatureSet, $row.Records, $row.GraphEdges, `
                            (Format-DurationCell $row.ParseSeconds), $row.SnapshotSizeMb, `
                            (Format-DurationCell $row.SaveSeconds), (Format-DurationCell $row.LoadSeconds), `
                            (Format-Mb $row.PeakMemoryMb)))
                    }
                }
            }
            [void]$sb.AppendLine()
        }
    }

    [void]$sb.AppendLine('<!-- BENCH:RESULTS-END -->')
    return $sb.ToString()
}

function Archive-PreviousResults {
    param([string]$Path)

    if (-not (Test-Path $Path)) { return }

    $docBody = Get-Content -Raw -Path $Path
    $startMarker = '<!-- BENCH:RESULTS-START -->'
    $endMarker   = '<!-- BENCH:RESULTS-END -->'

    $startIdx = $docBody.IndexOf($startMarker)
    $endIdx   = $docBody.IndexOf($endMarker)

    if ($startIdx -lt 0 -or $endIdx -lt 0 -or $endIdx -le $startIdx) { return }

    # Extract the old results block.
    $endIdx += $endMarker.Length
    $oldBlock = $docBody.Substring($startIdx, $endIdx - $startIdx)

    # Extract the timestamp from the old block.
    $tsMatch = [regex]::Match($oldBlock, '\*Last run:\s*([^.]+)\.')
    if (-not $tsMatch.Success) { return }
    $oldTimestamp = $tsMatch.Groups[1].Value.Trim()

    # Strip the BENCH markers and the "## Latest results" heading from the
    # old block — we're inserting it as a history subsection, not as the
    # latest results.
    $archiveBody = $oldBlock `
        -replace '<!-- BENCH:RESULTS-START -->', '' `
        -replace '<!-- BENCH:RESULTS-END -->', '' `
        -replace '## Latest results', '' `
        -replace '\*Last run:[^\*]+\*', ''
    $archiveBody = $archiveBody.Trim()

    if ([string]::IsNullOrWhiteSpace($archiveBody)) { return }

    # Find the "## History" heading and insert after it.
    $historyIdx = $docBody.IndexOf('## History')
    if ($historyIdx -lt 0) { return }

    # Find the end of the "## History" line.
    $nlIdx = $docBody.IndexOf("`n", $historyIdx)
    if ($nlIdx -lt 0) { $nlIdx = $docBody.Length }

    # Build the archive section.
    $archiveSection = @"


### $oldTimestamp (auto-archived)

$archiveBody

"@

    $newDoc = $docBody.Substring(0, $nlIdx + 1) + $archiveSection + $docBody.Substring($nlIdx + 1)
    Set-Content -Path $Path -Value $newDoc -Encoding utf8 -NoNewline
}

function Update-BenchmarkDoc {
    param([PSCustomObject]$R, [string]$Path)

    if (-not (Test-Path $Path)) {
        Write-Host ""
        Write-Host "WARNING: benchmark document not found: $Path" -ForegroundColor Yellow
        Write-Host "         create it with the BENCH:RESULTS-START/END markers to enable auto-update." -ForegroundColor Yellow
        return
    }

    # Auto-archive previous results before overwriting.
    if (-not $NoAutoArchive) {
        Archive-PreviousResults -Path $Path
    }

    $resultsBlock = Build-ResultsMarkdown -R $R
    # Re-read in case Archive-PreviousResults modified the file.
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
    Write-HistoryEntry -R $R -Dir $HistoryDir
    if (-not $NoDocWrite) {
        Update-BenchmarkDoc -R $R -Path $BenchDoc
    }
}

# ── Run benchmarks ──────────────────────────────────────────────────────

$Results = [PSCustomObject]@{
    Timestamp    = (Get-Date).ToString('o')
    Profiles     = $Profiles
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
if ($script:CanReadKey) {
    Write-Host "Press 's' to skip a step, 'a' to abort the run." -ForegroundColor DarkCyan
}

$doCheck   = $Mode -in @('all', 'check')
$doBuild   = $Mode -in @('all', 'build')
$doRuntime = $Mode -in @('all', 'runtime')
$doChained = $Mode -eq 'all'

# ── Check mode: per feature set (profiles don't apply) ─────────────────

if ($doCheck) {
    Write-Host ""
    Write-Host "== Cold cargo check ======================" -ForegroundColor Cyan
    foreach ($fs in $FeatureSets) {
        if ($script:Aborted) { break }
        $Results.CheckTimes += Invoke-ColdCheck -FeatureSet $fs
        Persist-Results -R $Results
    }
}

# ── Chained mode: per profile x feature -> cold build -> runtime ───────

if ($doChained -and -not $script:Aborted) {
    Write-Host ""
    Write-Host "== Cold build + runtime (chained) ========" -ForegroundColor Cyan
    foreach ($prof in $Profiles) {
        if ($script:Aborted) { break }
        foreach ($fs in $FeatureSets) {
            if ($script:Aborted) { break }

            # Cold build (timed).
            $buildResult = Invoke-ColdBuild -FeatureSet $fs -ProfileName $prof
            $Results.BuildTimes += $buildResult
            Persist-Results -R $Results

            if ($buildResult.Status -ne 'Completed') { continue }

            # Only run runtime for the feature sets in $RuntimeFeatureSets.
            $isRuntimeFeature = ($RuntimeFeatureSets | Where-Object { $_.Name -eq $fs.Name }).Count -gt 0
            if (-not $isRuntimeFeature) { continue }

            # Runtime standard (reuses the binary just built).
            if (-not $script:Aborted) {
                $Results.Runtime += Invoke-RuntimeBench -FeatureSet $fs -ProfileName $prof -SkipBuild
                Persist-Results -R $Results
            }
            # Runtime +graph.
            if (-not $script:Aborted) {
                $Results.Runtime += Invoke-RuntimeBench -FeatureSet $fs -ProfileName $prof -WithGraph -SkipBuild
                Persist-Results -R $Results
            }
        }
    }
}

# ── Build-only mode (no chaining) ──────────────────────────────────────

if ($doBuild -and -not $doChained -and -not $script:Aborted) {
    Write-Host ""
    Write-Host "== Cold build ============================" -ForegroundColor Cyan
    foreach ($prof in $Profiles) {
        if ($script:Aborted) { break }
        foreach ($fs in $FeatureSets) {
            if ($script:Aborted) { break }
            $Results.BuildTimes += Invoke-ColdBuild -FeatureSet $fs -ProfileName $prof
            Persist-Results -R $Results
        }
    }
}

# ── Runtime-only mode (no chaining) ────────────────────────────────────

if ($doRuntime -and -not $doChained -and -not $script:Aborted) {
    Write-Host ""
    Write-Host "== Runtime sc-bench ======================" -ForegroundColor Cyan
    foreach ($prof in $Profiles) {
        if ($script:Aborted) { break }
        foreach ($fs in $RuntimeFeatureSets) {
            if ($script:Aborted) { break }
            $Results.Runtime += Invoke-RuntimeBench -FeatureSet $fs -ProfileName $prof
            Persist-Results -R $Results
            if (-not $script:Aborted) {
                $Results.Runtime += Invoke-RuntimeBench -FeatureSet $fs -ProfileName $prof -WithGraph
                Persist-Results -R $Results
            }
        }
    }
}

# ── Final persistence + summary ────────────────────────────────────────

Persist-Results -R $Results

Write-Host ""
Write-Host "== Summary ================================" -ForegroundColor Cyan

function Write-StepRow {
    param([PSCustomObject]$R)
    $profStr = if ($Profiles.Count -gt 1) { " [$($R.Profile)]" } else { '' }
    '  {0,-24}{1}  {2,10}  peak {3,10}' -f `
        $R.Name, $profStr, (Format-Duration $R.ElapsedSeconds), (Format-Mb $R.PeakMemoryMb) | Write-Host
}

if ($Results.CheckTimes.Count -gt 0) {
    Write-Host ""
    Write-Host "Cold cargo check:" -ForegroundColor Yellow
    foreach ($r in $Results.CheckTimes) { Write-StepRow -R $r }
}

if ($Results.BuildTimes.Count -gt 0) {
    Write-Host ""
    Write-Host "Cold build:" -ForegroundColor Yellow
    foreach ($r in $Results.BuildTimes) { Write-StepRow -R $r }
}

if ($Results.Runtime.Count -gt 0) {
    Write-Host ""
    Write-Host "Runtime sc-bench:" -ForegroundColor Yellow
    foreach ($r in $Results.Runtime) {
        $profStr = if ($Profiles.Count -gt 1) { " [$($r.Profile)]" } else { '' }
        '  {0,-24}{1}  records={2,7:N0} parse={3,7:F2}s snap={4,7:F2} MB load={5,6:F2}s peak={6,10}' -f `
            $r.Name, $profStr, $r.Records, $r.ParseSeconds, $r.SnapshotSizeMb, $r.LoadSeconds, (Format-Mb $r.PeakMemoryMb) | Write-Host
    }
}

Write-Host ""
if ($script:Aborted) {
    Write-Host "Aborted by operator." -ForegroundColor Yellow
} else {
    Write-Host "Done." -ForegroundColor Cyan
}
Write-Host ("  raw JSON : {0}" -f $OutJson) -ForegroundColor DarkGray
Write-Host ("  history  : {0}" -f $HistoryDir) -ForegroundColor DarkGray
if (-not $NoDocWrite) {
    Write-Host ("  markdown : {0}" -f $BenchDoc) -ForegroundColor DarkGray
}
Write-Host ""
