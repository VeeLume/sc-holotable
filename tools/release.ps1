# Cut an sc-holotable library release.
#
# Pipeline:
#   1. Verify working tree is clean.
#   2. Validate -Version argument (vX.Y.Z form, strictly greater than the
#      highest existing 'sc-holotable/v*' tag).
#   3. Read CHANGELOG.md. Refuse if the '## [Unreleased]' section is
#      missing or empty (forces the author to have written release notes).
#   4. Rewrite CHANGELOG: rename '## [Unreleased]' to '## [vX.Y.Z] - YYYY-MM-DD'
#      and insert a fresh empty '## [Unreleased]' above it.
#   5. Commit the CHANGELOG rewrite as "Release sc-holotable/vX.Y.Z".
#   6. Create annotated tag 'sc-holotable/vX.Y.Z' whose message is the
#      released section's body.
#   7. With -Publish: push branch + tag. Otherwise leave both local for review.
#
# Semver convention (pre-1.0):
#   - 0.x.Y  patch: bugfixes, internal refactors, doc-only changes.
#   - 0.X.0  minor: ANY public-surface change, additive or breaking.
#            Pre-1.0 lets minor bumps break; don't overthink it.
#   - 1.0.0  reserved for the first stable workspace API.
#
# ASCII only (no em-dashes, no smart quotes) so Windows PowerShell 5.1 with a
# non-English locale does not corrupt the script.

[CmdletBinding()]
param(
    # Required. Target version in 'vX.Y.Z' form (the leading 'v' is optional).
    [Parameter(Mandatory = $true)]
    [string]$Version,

    # Opt into 'git push' of the branch and the new tag. Off by default.
    [switch]$Publish,

    # Allow running even when the working tree is not clean.
    [switch]$AllowDirty,

    # Skip the commit + tag steps. Leaves the CHANGELOG rewrite staged
    # for manual review.
    [switch]$NoCommit
)

$ErrorActionPreference = 'Stop'

function Die([string]$msg) {
    Write-Host "release.ps1: $msg" -ForegroundColor Red
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

# --- 1. Validate version argument -------------------------------------------

Step "Validate version"

# Strip optional leading 'v', then require a strict X.Y.Z triple.
$versionCore = $Version -replace '^v', ''
if ($versionCore -notmatch '^(\d+)\.(\d+)\.(\d+)$') {
    Die "Version '$Version' is not in vX.Y.Z form."
}
$major = [int]$Matches[1]
$minor = [int]$Matches[2]
$patch = [int]$Matches[3]
$normalizedVersion = "v$major.$minor.$patch"
$tagName = "sc-holotable/$normalizedVersion"

Write-Host "  Target tag: $tagName"

# --- 2. Preflight: clean tree, no existing tag, monotonic ordering ----------

Step "Preflight: check working tree"
$status = git status --porcelain
if ($LASTEXITCODE -ne 0) { Die "git status failed" }
$dirtyTracked = $status | Where-Object { $_ -notmatch '^\?\?' }
if ($dirtyTracked -and -not $AllowDirty) {
    Write-Host ($dirtyTracked -join "`n")
    Die "Tracked files have uncommitted changes. Commit or stash first, or pass -AllowDirty."
}

Step "Preflight: tag uniqueness + monotonic ordering"
$existing = git tag -l $tagName
if ($LASTEXITCODE -ne 0) { Die "git tag -l failed" }
if ($existing) {
    Die "Tag '$tagName' already exists. Pick a higher version."
}

$tagPattern = 'refs/tags/sc-holotable/v*'
$allTags = git for-each-ref --format='%(refname:short)' $tagPattern 2>$null
if ($LASTEXITCODE -ne 0) { $allTags = @() }

$maxTriple = @(0, 0, 0)
$maxTag = $null
foreach ($t in $allTags) {
    if ($t -match '^sc-holotable/v(\d+)\.(\d+)\.(\d+)$') {
        $triple = @([int]$Matches[1], [int]$Matches[2], [int]$Matches[3])
        $greater = $false
        for ($i = 0; $i -lt 3; $i++) {
            if ($triple[$i] -gt $maxTriple[$i]) { $greater = $true; break }
            if ($triple[$i] -lt $maxTriple[$i]) { break }
        }
        if ($greater) {
            $maxTriple = $triple
            $maxTag = $t
        }
    }
}

$newTriple = @($major, $minor, $patch)
$isGreater = $false
for ($i = 0; $i -lt 3; $i++) {
    if ($newTriple[$i] -gt $maxTriple[$i]) { $isGreater = $true; break }
    if ($newTriple[$i] -lt $maxTriple[$i]) { break }
}
if ($maxTag -and -not $isGreater) {
    Die "Version $normalizedVersion is not greater than the highest existing tag '$maxTag'."
}
if ($maxTag) {
    Write-Host "  Previous tag: $maxTag"
} else {
    Write-Host "  Previous tag: (none -- first release)"
}

# --- 3. Read + validate CHANGELOG -------------------------------------------

Step "Read CHANGELOG.md"
$changelogPath = Join-Path $repoRoot 'CHANGELOG.md'
if (-not (Test-Path $changelogPath)) {
    Die "CHANGELOG.md not found at repo root."
}
$changelog = Get-Content -LiteralPath $changelogPath -Raw -Encoding UTF8

# Locate the '## [Unreleased]' heading and the next '## ' heading (or EOF).
# The section body is everything between them, excluding both headings.
$unreleasedPattern = '(?ms)^## \[Unreleased\][^\r\n]*\r?\n(?<body>.*?)(?=^## \[|\z)'
$match = [regex]::Match($changelog, $unreleasedPattern)
if (-not $match.Success) {
    Die "CHANGELOG.md has no '## [Unreleased]' section."
}
$body = $match.Groups['body'].Value
# A section is 'empty' if it has no non-whitespace content beyond the heading.
$trimmedBody = $body.Trim()
if (-not $trimmedBody) {
    Die "The '## [Unreleased]' section is empty. Add release notes before cutting a tag."
}

# --- 4. Rewrite CHANGELOG ---------------------------------------------------

Step "Rewrite CHANGELOG.md"
$releaseDate = (Get-Date -Format 'yyyy-MM-dd')
$releasedHeading = "## [$normalizedVersion] - $releaseDate"
$freshUnreleased = "## [Unreleased]`r`n`r`n"
# Replace the single '## [Unreleased]' line with:
#   ## [Unreleased]    (fresh, empty)
#
#   ## [vX.Y.Z] - YYYY-MM-DD
# and keep the previous body where it was.
$unreleasedHeadingPattern = '(?m)^## \[Unreleased\][^\r\n]*\r?\n'
$replacement = $freshUnreleased + $releasedHeading + "`r`n"
$newChangelog = [regex]::Replace($changelog, $unreleasedHeadingPattern, $replacement, 1)
# Use UTF-8 without BOM for cross-platform friendliness.
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllText($changelogPath, $newChangelog, $utf8NoBom)
Write-Host "  Moved Unreleased notes into $releasedHeading"

# --- 5. Commit --------------------------------------------------------------

if ($NoCommit) {
    Step "Skipping commit (-NoCommit). CHANGELOG.md is modified, unstaged."
    Write-Host "Review with 'git diff CHANGELOG.md'."
    exit 0
}

Step "Stage + commit"
git add CHANGELOG.md
if ($LASTEXITCODE -ne 0) { Die "git add CHANGELOG.md failed" }

$commitMsg = "Release sc-holotable/$normalizedVersion"
git commit -m $commitMsg
if ($LASTEXITCODE -ne 0) { Die "git commit failed" }
Write-Host "Committed: $commitMsg"

# --- 6. Tag -----------------------------------------------------------------

Step "Create tag '$tagName'"
# Tag message = the released section body. Write to a temp file so
# bullet points / multi-line notes round-trip faithfully (git tag -m
# mangles newlines across shells).
$tagBody = "sc-holotable $normalizedVersion`r`n`r`n" + $trimmedBody + "`r`n"
$tagMsgFile = [System.IO.Path]::GetTempFileName()
try {
    [System.IO.File]::WriteAllText($tagMsgFile, $tagBody, $utf8NoBom)
    git tag -a $tagName -F $tagMsgFile
    if ($LASTEXITCODE -ne 0) { Die "git tag failed" }
} finally {
    Remove-Item -LiteralPath $tagMsgFile -Force -ErrorAction SilentlyContinue
}

# --- 7. Publish (optional) --------------------------------------------------

if (-not $Publish) {
    Write-Host ""
    Write-Host "Done. Local commit + tag created. To publish: rerun with -Publish, or push/tag manually." -ForegroundColor Green
    exit 0
}

$currentBranch = git rev-parse --abbrev-ref HEAD
if ($LASTEXITCODE -ne 0) { Die "git rev-parse failed" }

Step "Push branch '$currentBranch'"
git push origin $currentBranch
if ($LASTEXITCODE -ne 0) { Die "git push failed" }

Step "Push tag '$tagName'"
git push origin $tagName
if ($LASTEXITCODE -ne 0) { Die "git push tag failed" }

Write-Host ""
Write-Host "Done. Committed, pushed, tagged '$tagName'." -ForegroundColor Green
