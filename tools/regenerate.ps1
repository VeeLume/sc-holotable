# Regenerate DataCore bindings after a Star Citizen patch.
#
# Pipeline:
#   1. Verify working tree is clean (refuse to mix regen with other work).
#   2. Run sc-generator (auto-discovers LIVE P4K via sc-installs unless -P4k is given).
#      Captures the SC version from the generator's machine-parseable stdout line.
#   3. cargo fmt --all
#   4. cargo clippy --fix on sc-extract-generated (the crate that holds the
#      generator output). Skipped with -SkipClippy.
#   5. Stage everything, commit with "Regenerate DataCore bindings (SC <version>)".
#   6. With -Publish: push current branch and create an annotated tag
#      "datacore/<sc_version>" pointing at the new commit, then push the tag.
#
# Destructive steps (push, tag) are opt-in behind -Publish. Default behaviour
# is regenerate-and-commit; the user inspects the commit and publishes manually
# if not using -Publish.
#
# ASCII only (no em-dashes, no smart quotes) so Windows PowerShell 5.1 with a
# non-English locale does not corrupt the script.

[CmdletBinding()]
param(
    # Opt into 'git push' + 'git tag -a' + 'git push --tags'. Off by default.
    [switch]$Publish,

    # Skip the clippy --fix pass (faster iteration; not recommended for final runs).
    [switch]$SkipClippy,

    # Override P4K auto-discovery with an explicit path.
    [string]$P4k,

    # Allow running even when the working tree is not clean.
    # Useful for recovering from a partially-failed previous run.
    [switch]$AllowDirty,

    # Skip the commit step entirely. Leaves all changes staged for manual review.
    [switch]$NoCommit
)

$ErrorActionPreference = 'Stop'

function Die([string]$msg) {
    Write-Host "regenerate.ps1: $msg" -ForegroundColor Red
    exit 1
}

function Step([string]$msg) {
    Write-Host ""
    Write-Host "==> $msg" -ForegroundColor Cyan
}

# Resolve repo root (parent of the tools/ directory containing this script).
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptDir '..')
Set-Location $repoRoot

# --- 1. Preflight ------------------------------------------------------------

Step "Preflight: check working tree"
# Only reject modifications to tracked files. Untracked files elsewhere
# in the tree (scratch notes, draft docs) are not swept into the regen
# commit by the scoped staging below, so they are safe to ignore here.
$status = git status --porcelain
if ($LASTEXITCODE -ne 0) { Die "git status failed" }
$dirtyTracked = $status | Where-Object { $_ -notmatch '^\?\?' }
if ($dirtyTracked -and -not $AllowDirty) {
    Write-Host ($dirtyTracked -join "`n")
    Die "Tracked files have uncommitted changes. Commit or stash first, or pass -AllowDirty."
}

# --- 2. Generator ------------------------------------------------------------

Step "Run sc-generator"
$generatorArgs = @('run', '-p', 'sc-generator', '--release', '--')
if ($P4k) { $generatorArgs += @('--p4k', $P4k) }

# Capture stdout line-by-line so we can fish out the identity line while
# still streaming cargo output to the console.
$identityLine = $null
$generatorProcess = & cargo @generatorArgs 2>&1 | ForEach-Object {
    $line = $_.ToString()
    if ($line -match '^sc-generator:\s+sc_version=(\S+)\s+channel=(\S+)\s+p4k=(.+)$') {
        $identityLine = $line
    }
    $line
}
if ($LASTEXITCODE -ne 0) { Die "sc-generator failed" }

if (-not $identityLine) {
    Die "sc-generator ran but did not emit an identity line (sc_version=...). Cannot determine version for commit/tag."
}
$null = $identityLine -match '^sc-generator:\s+sc_version=(\S+)\s+channel=(\S+)\s+p4k=(.+)$'
$scVersion = $Matches[1]
$scChannel = $Matches[2]
$scP4k = $Matches[3]
Write-Host "  sc_version: $scVersion"
Write-Host "  channel:    $scChannel"
Write-Host "  p4k:        $scP4k"

# --- 3. cargo fmt ------------------------------------------------------------

Step "cargo fmt --all"
cargo fmt --all
if ($LASTEXITCODE -ne 0) { Die "cargo fmt failed" }

# --- 4. cargo clippy --fix ---------------------------------------------------

if (-not $SkipClippy) {
    Step "cargo clippy --fix -p sc-extract-generated (with all features)"
    # Scoped to the generated crate: that's what the regen pipeline is
    # responsible for keeping clean. Lints in hand-written examples and other
    # crates are out of scope for a regen commit.
    cargo clippy -p sc-extract-generated --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Die "clippy reported unfixable lints in the generated crate. The generator emitter likely needs an update."
    }

    # Re-format in case clippy --fix introduced style drift.
    cargo fmt --all
    if ($LASTEXITCODE -ne 0) { Die "cargo fmt (post-clippy) failed" }
}

# --- 5. Commit ---------------------------------------------------------------

if ($NoCommit) {
    Step "Skipping commit (-NoCommit). All changes are unstaged."
    Write-Host "Review with 'git status' / 'git diff'."
    exit 0
}

Step "Stage + commit"
# Stage all tracked changes (generator output, fmt drift, clippy fixes),
# plus any new untracked files inside the generator output directory
# (e.g. a brand-new leaf feature directory from a game patch). Unrelated
# untracked files elsewhere in the tree (drafts, scratch docs) are left
# alone so they do not get swept into the regen commit.
git add -u
if ($LASTEXITCODE -ne 0) { Die "git add -u failed" }
git add crates/sc-extract-generated
if ($LASTEXITCODE -ne 0) { Die "git add crates/sc-extract-generated failed" }

# If nothing changed, bail out rather than create an empty commit.
$diffCached = git diff --cached --stat
if ($LASTEXITCODE -ne 0) { Die "git diff --cached failed" }
if (-not $diffCached) {
    Write-Host "No changes to commit. Generator produced identical output." -ForegroundColor Yellow
    exit 0
}

$commitMsg = "Regenerate DataCore bindings (SC $scVersion)"
git commit -m $commitMsg
if ($LASTEXITCODE -ne 0) { Die "git commit failed" }
Write-Host "Committed: $commitMsg"

# --- 6. Publish (optional) ---------------------------------------------------

if (-not $Publish) {
    Write-Host ""
    Write-Host "Done. To publish: rerun with -Publish, or push/tag manually." -ForegroundColor Green
    exit 0
}

$currentBranch = git rev-parse --abbrev-ref HEAD
if ($LASTEXITCODE -ne 0) { Die "git rev-parse failed" }

Step "Push branch '$currentBranch'"
git push origin $currentBranch
if ($LASTEXITCODE -ne 0) { Die "git push failed" }

$tagName = "datacore/$scVersion"
$tagMessage = "DataCore bindings regenerated from SC $scVersion ($scChannel)"
Step "Create tag '$tagName'"
git tag -a $tagName -m $tagMessage
if ($LASTEXITCODE -ne 0) { Die "git tag failed" }

Step "Push tag '$tagName'"
git push origin $tagName
if ($LASTEXITCODE -ne 0) { Die "git push tag failed" }

Write-Host ""
Write-Host "Done. Committed, pushed, tagged '$tagName'." -ForegroundColor Green
