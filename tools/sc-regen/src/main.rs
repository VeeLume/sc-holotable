//! `sc-regen` — keeps the generated DataCore bindings in step with the game.
//!
//! A Windows scheduled task runs it every few hours. Each run:
//!
//! 1. Picks the builds to cover ([`Window`]): the newest LIVE (or hotfix)
//!    build installed, plus the newest PTU or EPTU build when one is newer.
//!    The bindings are their union, so the release cut while a patch is on
//!    PTU already reads it, and the apps keep working on patch day whether
//!    or not this machine is on. Builds are ordered by the changelist in
//!    their `build_manifest.id`. Only installs whose launcher label matches
//!    that changelist count: a folder renamed between channels (LIVE moved
//!    to PTU to seed a patch) is skipped until the launcher has patched it.
//!    So is an install the launcher left half-patched
//!    (`Installation::patch_state`). While the launcher has a run open on
//!    any install, the run waits for it to end (up to two hours), so a patch
//!    started right after boot is regenerated as soon as it lands. The check
//!    runs again once the generator has read the `Data.p4k`s, and a change in
//!    between fails the run.
//! 2. Stops if the newest `datacore/*` tag already covers every build
//!    ([`Window::covered_by`]), if a PR for these builds already exists
//!    (open, merged or closed), or if the game is running (checked again
//!    after waiting out a patch). A stop is not a failure: the next run
//!    tries again, and only failures raise a toast.
//! 3. Regenerates in its own worktree (`%LOCALAPPDATA%\sc-regen\worktree`,
//!    reset to `origin/main`), never in a development checkout: generator
//!    (one `--p4k` per build, oldest first), fmt, clippy `--fix` on the
//!    generated crate, one commit on `regen/<label>`, where a union's label
//!    joins its builds' labels with `+`.
//! 4. Runs `cargo test --workspace`, the smoke examples and `parse_real_p4k`
//!    on every build, pushes the branch and opens a PR whose body reports
//!    what changed. The PR is a draft when a check failed or no LIVE build
//!    is covered.
//! 5. Merges the PR itself when the regen is safe
//!    ([`report::auto_merge_blockers`]): it gets the `auto-merge` label,
//!    `sc-regen` waits for GitHub CI and squash-merges. The push to `main`
//!    runs `.github/workflows/release.yml`, which tags `datacore/<label>` and
//!    cuts the `sc-holotable/v*` release. Removing the label stops the merge;
//!    a run interrupted while waiting picks the PR up again next time.
//!
//! ```text
//! cargo install --path tools/sc-regen
//! sc-regen schedule --repo E:\vscode\rust\sc-holotable   # register the task once
//! sc-regen --repo <path>             # what the task runs
//! sc-regen --repo <path> --now       # also when a tag or PR already covers the build
//! sc-regen --repo <path> --dry-run   # report the decision, change nothing
//! sc-regen --repo <path> --now --no-publish   # full local run, stop before push and PR
//! sc-regen --repo <path> --now --no-publish --base my-branch   # same, off an unpushed branch
//! ```
//!
//! The full output of the last run is in `%LOCALAPPDATA%\sc-regen\last-run.log`,
//! one line per run in `history.log`.

mod report;
mod windows;

use std::fs::File;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use sc_discovery::Channel;
use tracing::{info, warn};
use tracing_subscriber::fmt::writer::MakeWriterExt as _;

use report::{Bindings, BuildInfo, Check, GameBuild};

const GENERATED: &str = "crates/sc-extract-generated/src/generated";

struct Options {
    repo: PathBuf,
    now: bool,
    dry_run: bool,
    no_publish: bool,
    schedule: bool,
    /// What the worktree is reset to; anything but `origin/main` only with
    /// `--no-publish`, since the PR targets `main`.
    base: String,
}

const DEFAULT_BASE: &str = "origin/main";

/// One installed build, as the regen names it.
#[derive(Clone, Debug, PartialEq)]
struct Build {
    label: String,
    changelist: u64,
    channel: Channel,
    p4k: PathBuf,
}

impl Build {
    fn is_live(&self) -> bool {
        matches!(self.channel, Channel::Live | Channel::Hotfix)
    }

    /// A build on its way to LIVE. Not Tech Preview: those test server tech,
    /// and their schema need not ship, so their types would linger in the
    /// bindings without a LIVE build ever having them.
    fn is_preview(&self) -> bool {
        matches!(self.channel, Channel::Ptu | Channel::Eptu)
    }
}

/// The builds one regen covers, oldest first: the newest LIVE build, then
/// the newest preview build if it is newer. Without a LIVE install, the
/// newest build alone; such a regen is a draft and never tagged.
#[derive(Debug, PartialEq)]
struct Window {
    builds: Vec<Build>,
}

impl Window {
    fn from_builds(mut builds: Vec<Build>) -> Option<Window> {
        builds.sort_by_key(|b| b.changelist);
        let Some(live) = builds.iter().rposition(Build::is_live) else {
            return builds.pop().map(|b| Window { builds: vec![b] });
        };
        let live_changelist = builds[live].changelist;
        let preview = builds
            .iter()
            .rposition(|b| b.is_preview() && b.changelist > live_changelist)
            // Remove the later index first so `live` stays valid.
            .map(|i| builds.remove(i));
        let mut window = vec![builds.remove(live)];
        window.extend(preview);
        Some(Window { builds: window })
    }

    /// Covers a LIVE build, so it may merge itself and gets tagged.
    fn is_live(&self) -> bool {
        self.builds[0].is_live()
    }

    fn labels(&self) -> Vec<&str> {
        self.builds.iter().map(|b| b.label.as_str()).collect()
    }

    /// `4.10.3-live.13165946 + 5.0.0-ptu.13228337`; the release workflow
    /// reads this shape back from the commit subject.
    fn label(&self) -> String {
        self.labels().join(" + ")
    }

    fn branch(&self) -> String {
        format!("regen/{}", self.labels().join("+"))
    }

    fn commit_message(&self) -> String {
        format!("chore(datacore): regenerate bindings (SC {})", self.label())
    }

    /// Whether the newest tag's builds (`tagged`, as changelists) already
    /// cover every build here: each is one of them or older than all of
    /// them. Leaving one out is no reason to regenerate: an uninstalled PTU
    /// must not take its types out of the bindings again.
    fn covered_by(&self, tagged: &[u64]) -> bool {
        let oldest = tagged.iter().min();
        self.builds
            .iter()
            .all(|b| tagged.contains(&b.changelist) || oldest.is_some_and(|o| b.changelist < *o))
    }
}

fn main() -> ExitCode {
    let options = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("sc-regen: {e}");
            eprintln!(
                "usage: sc-regen [schedule] [--repo <path>] [--now] [--dry-run] [--no-publish] \
                 [--base <ref>]"
            );
            return ExitCode::from(2);
        }
    };
    let state = state_dir();
    if let Err(e) = std::fs::create_dir_all(&state) {
        eprintln!("sc-regen: creating {}: {e}", state.display());
        return ExitCode::FAILURE;
    }
    let log = match File::create(state.join("last-run.log")) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("sc-regen: creating the run log: {e}");
            return ExitCode::FAILURE;
        }
    };
    let Ok(log_for_tracing) = log.try_clone() else {
        eprintln!("sc-regen: cloning the run log handle");
        return ExitCode::FAILURE;
    };
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(std::io::stderr.and(Mutex::new(log_for_tracing)))
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    if options.schedule {
        return match schedule(&options.repo) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                tracing::error!("{e}");
                ExitCode::FAILURE
            }
        };
    }

    let Some(_lock) = Lock::acquire(&state.join("lock")) else {
        info!("another run holds the lock; exiting");
        return ExitCode::SUCCESS;
    };
    let outcome = run(&options, &state, &log);
    let line = match &outcome {
        Ok(msg) => msg.clone(),
        Err(e) => format!("FAILED: {e}"),
    };
    match &outcome {
        Ok(msg) => info!("{msg}"),
        Err(e) => tracing::error!("{e}"),
    }
    append_history(&state, &line);
    match outcome {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            windows::notify("sc-regen failed", &e);
            ExitCode::FAILURE
        }
    }
}

fn parse_args() -> Result<Options, String> {
    let mut options = Options {
        repo: std::env::current_dir().map_err(|e| e.to_string())?,
        now: false,
        dry_run: false,
        no_publish: false,
        schedule: false,
        base: DEFAULT_BASE.to_string(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "schedule" => options.schedule = true,
            "--repo" => options.repo = args.next().ok_or("--repo requires a path")?.into(),
            "--now" => options.now = true,
            "--dry-run" => options.dry_run = true,
            "--no-publish" => options.no_publish = true,
            "--base" => options.base = args.next().ok_or("--base requires a ref")?,
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    if options.base != DEFAULT_BASE && !options.no_publish {
        return Err("--base needs --no-publish: a published regen targets main".into());
    }
    // Not `canonicalize`: on Windows it yields a `\\?\` path, which then
    // lands in the scheduled task's command line.
    options.repo = std::path::absolute(&options.repo)
        .map_err(|e| format!("--repo {}: {e}", options.repo.display()))?;
    if !options.repo.join(".git").exists() {
        return Err(format!(
            "--repo {}: not a git checkout",
            options.repo.display()
        ));
    }
    Ok(options)
}

/// Returns a one-line summary of what happened, for `history.log`.
fn run(options: &Options, state: &Path, log: &File) -> Result<String, String> {
    if !options.now && !options.dry_run && windows::game_running() {
        return Ok("game running; skipped until the next run".into());
    }
    let window = match pick_window(options)? {
        Pick::Window(window) => window,
        Pick::Skip(why) => return Ok(why),
    };
    for build in &window.builds {
        info!(
            label = %build.label,
            channel = %build.channel,
            changelist = build.changelist,
            p4k = %build.p4k.display(),
            "covering build"
        );
    }
    let label = window.label();

    let repo = &options.repo;
    git(
        repo,
        &["fetch", "--quiet", "--prune", "--tags", "origin"],
        log,
    )?;
    let previous_tag = newest_datacore_tag(repo)?;
    if !options.now {
        if let Some((tag, tagged)) = &previous_tag
            && window.covered_by(tagged)
        {
            return Ok(format!("{label} is covered by {tag}; nothing to do"));
        }
        if let Some(pr) = find_pr(repo, &window.branch())? {
            if pr.open && pr.auto_merge && !options.dry_run {
                // Only the commit this machine pushed may be merged; the
                // worktree's branch still holds it from the interrupted run.
                let pushed = capture(
                    Command::new("git")
                        .args(["rev-parse", "--verify", "--quiet"])
                        .arg(format!("refs/heads/{}", window.branch()))
                        .current_dir(state.join("worktree")),
                );
                let Ok(pushed) = pushed else {
                    return Ok(format!(
                        "{label} has {} but no local record of the pushed commit; left for review",
                        pr.url
                    ));
                };
                info!(pr = %pr.url, "resuming the merge of an open auto-merge PR");
                return land(repo, &pr.url, pushed.trim(), &window, log);
            }
            return Ok(format!("{label} already has {}; nothing to do", pr.url));
        }
    }
    if options.dry_run {
        return Ok(format!(
            "dry run: would regenerate {label} (previous tag {})",
            previous_tag.as_ref().map_or("none", |(t, _)| t.as_str())
        ));
    }

    let worktree = state.join("worktree");
    prepare_worktree(repo, &worktree, &window.branch(), &options.base, log)?;
    let generated = worktree.join(GENERATED);
    let before = Bindings::read(&generated)?;

    info!("running sc-generator");
    let p4ks: Vec<String> = window
        .builds
        .iter()
        .map(|b| b.p4k.to_string_lossy().into_owned())
        .collect();
    let mut generator = vec!["run", "-p", "sc-generator", "--release", "--"];
    for p4k in &p4ks {
        generator.extend(["--p4k", p4k.as_str()]);
    }
    cargo(&worktree, &generator, log)?;
    // The launcher rewrites Data.p4k in place; a patch that started while the
    // generator read it leaves bindings from a torn file.
    // An install the launcher has a run open on drops out of the survey's
    // builds, so a patch on it shows as a different window.
    match Survey::take().map(|s| Window::from_builds(s.builds)) {
        Ok(Some(now)) if now == window => {}
        _ => {
            return Err(format!(
                "{label} changed while the generator read it (did the launcher start patching?); \
                 the next run retries"
            ));
        }
    }
    // A generator from before multi-build support takes only the last
    // `--p4k`; its bindings must not go out under a union's name and tag.
    let generated_builds = report::generated_builds(&generated)?;
    if generated_builds != window.labels() {
        return Err(format!(
            "the generator on {} wrote bindings for [{}], not [{}]: it predates \
             multi-build support or labels builds differently",
            options.base,
            generated_builds.join(", "),
            window.labels().join(", ")
        ));
    }
    cargo(&worktree, &["fmt", "--all"], log)?;
    info!("clippy --fix on the generated crate");
    cargo(
        &worktree,
        &[
            "clippy",
            "-p",
            "sc-extract-generated",
            "--all-targets",
            "--all-features",
            "--fix",
            "--allow-dirty",
            "--allow-staged",
            "--",
            "-D",
            "warnings",
        ],
        log,
    )
    .map_err(|e| format!("{e}: the generator emitter likely needs an update"))?;
    cargo(&worktree, &["fmt", "--all"], log)?;
    let after = Bindings::read(&generated)?;

    git(&worktree, &["add", "-u"], log)?;
    git(&worktree, &["add", "crates/sc-extract-generated"], log)?;
    git(
        &worktree,
        &["commit", "--quiet", "-m", &window.commit_message()],
        log,
    )?;

    let mut checks = vec![
        check(
            &worktree,
            "cargo test --workspace",
            &["test", "--workspace"],
            log,
        ),
        check(
            &worktree,
            "encounter_analytics",
            &[
                "run",
                "-p",
                "sc-missions",
                "--release",
                "--example",
                "encounter_analytics",
            ],
            log,
        ),
        check(
            &worktree,
            "product_stats",
            &[
                "run",
                "-p",
                "sc-crafting",
                "--release",
                "--example",
                "product_stats",
            ],
            log,
        ),
    ];
    // The smoke examples above read the LIVE install. This one parses each
    // covered build and fails on a record type the bindings don't know.
    for (build, p4k) in window.builds.iter().zip(&p4ks) {
        checks.push(check(
            &worktree,
            &format!("parse_real_p4k {}", build.label),
            &[
                "run",
                "-p",
                "sc-extract",
                "--release",
                "--features",
                "full",
                "--example",
                "parse_real_p4k",
                "--",
                p4k,
            ],
            log,
        ));
    }
    let all_passed = checks.iter().all(|c| c.passed);

    let info = BuildInfo {
        builds: window
            .builds
            .iter()
            .map(|b| GameBuild {
                label: &b.label,
                channel: b.channel.to_string(),
                changelist: b.changelist,
            })
            .collect(),
        previous_tag: previous_tag.as_ref().map(|(t, _)| t.as_str()),
    };
    let blockers = report::auto_merge_blockers(window.is_live(), &before, &after, &checks);
    let body = report::pr_body(&info, &before, &after, &checks, &blockers);
    let body_file = state.join("pr-body.md");
    std::fs::write(&body_file, &body).map_err(|e| format!("writing the PR body: {e}"))?;

    if options.no_publish {
        return Ok(format!(
            "regenerated {label} locally (checks {}, {}); PR body in {}",
            if all_passed { "passed" } else { "FAILED" },
            if blockers.is_empty() {
                "would auto-merge"
            } else {
                "would wait for review"
            },
            body_file.display()
        ));
    }
    let auto_merge = blockers.is_empty();
    if auto_merge {
        ensure_auto_merge_label(&worktree)?;
    }

    let branch = window.branch();
    let mut push = vec!["push", "--quiet", "-u", "origin", branch.as_str()];
    if options.now {
        push.push("--force");
    }
    // The repo's pre-push hook gates on clippy. A regen whose checks failed
    // is exactly the one that must still reach a draft PR for review, and
    // the hook would refuse it; its failures are already in the PR body and
    // CI repeats them. A passing regen keeps the hook.
    if !all_passed {
        push.push("--no-verify");
    }
    git(&worktree, &push, log)?;
    let title = window.commit_message();
    let body_path = body_file.to_string_lossy().into_owned();
    let mut pr = vec![
        "pr",
        "create",
        "--base",
        "main",
        "--head",
        &branch,
        "--title",
        &title,
        "--body-file",
        &body_path,
    ];
    let draft = !all_passed || !window.is_live();
    if draft {
        pr.push("--draft");
    }
    if auto_merge {
        pr.extend(["--label", AUTO_MERGE_LABEL]);
    }
    let url = capture(Command::new("gh").args(&pr).current_dir(&worktree))?;
    let url = url.trim().to_string();

    if auto_merge {
        let pushed = capture(
            Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&worktree),
        )?;
        return land(&worktree, &url, pushed.trim(), &window, log);
    }
    let summary = format!(
        "regen PR for {label} opened for review ({}): {url}",
        blockers.join("; ")
    );
    windows::notify(&format!("Bindings for {label}"), &summary);
    Ok(summary)
}

const AUTO_MERGE_LABEL: &str = "auto-merge";

/// How long to wait for GitHub CI on a regen PR. CI takes about ten
/// minutes; a run that gives up leaves the PR to the next run.
const CI_WAIT: Duration = Duration::from_secs(45 * 60);

/// The GitHub check that must pass before a regen merges itself. If CI is
/// renamed, auto-merge times out and the PR waits for review: fail closed.
const REQUIRED_CHECK: &str = "ci / check";

/// Wait for GitHub CI on an auto-merge PR, then squash-merge it. Merges
/// only `pushed`, the commit this run (or the interrupted one) pushed: a PR
/// whose head moved, or one from a fork, is left alone whatever its label.
/// The label is the off switch, so it is read again right before merging.
fn land(
    dir: &Path,
    url: &str,
    pushed: &str,
    window: &Window,
    log: &File,
) -> Result<String, String> {
    let label = window.label();
    info!(pr = url, "waiting for GitHub CI");
    match wait_for_ci(dir, url) {
        Ci::Passed => {}
        Ci::Failed => {
            // Drop the label so later runs don't wait on the same failure.
            let _ = capture(
                Command::new("gh")
                    .args(["pr", "edit", url, "--remove-label", AUTO_MERGE_LABEL])
                    .current_dir(dir),
            );
            let summary = format!("regen PR for {label}: GitHub CI failed, left for review: {url}");
            windows::notify(&format!("Bindings for {label}"), &summary);
            return Ok(summary);
        }
        Ci::TimedOut => {
            return Ok(format!(
                "regen PR for {label}: GitHub CI still running; the next run merges it: {url}"
            ));
        }
    }
    let Some(pr) = view_pr(dir, url)? else {
        return Err(format!("{url} vanished while waiting for CI"));
    };
    if !(pr.open && pr.auto_merge && !pr.draft) {
        return Ok(format!(
            "regen PR for {label} is no longer marked {AUTO_MERGE_LABEL}; left for review: {url}"
        ));
    }
    if pr.fork || pr.head != pushed {
        return Ok(format!(
            "regen PR for {label} no longer points at the pushed commit {pushed}; left for review: {url}"
        ));
    }
    // `--match-head-commit` makes GitHub refuse the merge if the head moved
    // after the check above.
    capture(
        Command::new("gh")
            .args([
                "pr",
                "merge",
                url,
                "--squash",
                "--match-head-commit",
                pushed,
            ])
            .current_dir(dir),
    )?;
    // Not `gh pr merge --delete-branch`: that also switches the local
    // checkout to `main`, which the development checkout holds.
    if let Err(e) = git(
        dir,
        &["push", "--quiet", "origin", "--delete", &window.branch()],
        log,
    ) {
        warn!("deleting the merged branch: {e}");
    }
    let summary =
        format!("regen PR for {label} merged; the release workflow tags and releases it: {url}");
    windows::notify(&format!("Bindings for {label}"), &summary);
    Ok(summary)
}

enum Ci {
    Passed,
    Failed,
    TimedOut,
}

/// Poll the PR's checks every 30 s until [`ci_verdict`] decides.
fn wait_for_ci(dir: &Path, url: &str) -> Ci {
    let deadline = Instant::now() + CI_WAIT;
    loop {
        // `gh pr checks` exits non-zero while checks are pending or failing,
        // so read the output whatever the status.
        let checks = Command::new("gh")
            .args(["pr", "checks", url, "--json", "bucket,name"])
            .args(["--jq", r#".[] | "\(.bucket) \(.name)""#])
            .current_dir(dir)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_default();
        if let Some(verdict) = ci_verdict(&checks) {
            return verdict;
        }
        if Instant::now() >= deadline {
            return Ci::TimedOut;
        }
        std::thread::sleep(Duration::from_secs(30));
    }
}

/// Decide from `gh pr checks` lines (`<bucket> <name>`): any failure fails;
/// a pass needs [`REQUIRED_CHECK`] to have passed and nothing pending. No
/// checks yet, or only skipped ones, is undecided (`None`), never a pass.
fn ci_verdict(checks: &str) -> Option<Ci> {
    let checks: Vec<(&str, &str)> = checks
        .lines()
        .filter_map(|l| l.trim().split_once(' '))
        .collect();
    if checks.iter().any(|(b, _)| matches!(*b, "fail" | "cancel")) {
        return Some(Ci::Failed);
    }
    let pending = checks.iter().any(|(b, _)| *b == "pending");
    let required = checks
        .iter()
        .any(|(b, n)| *b == "pass" && *n == REQUIRED_CHECK);
    (required && !pending).then_some(Ci::Passed)
}

/// Create the label if the repo lacks it (`--force` makes this idempotent).
fn ensure_auto_merge_label(dir: &Path) -> Result<(), String> {
    capture(
        Command::new("gh")
            .args(["label", "create", AUTO_MERGE_LABEL, "--force"])
            .args(["--color", "0E8A16"])
            .args(["--description", "sc-regen merges this once CI passes"])
            .current_dir(dir),
    )
    .map(drop)
}

fn schedule(repo: &Path) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("locating sc-regen.exe: {e}"))?;
    windows::register_task(&exe, repo)?;
    info!(
        task = windows::TASK_NAME,
        exe = %exe.display(),
        repo = %repo.display(),
        "scheduled task registered"
    );
    Ok(())
}

/// How long a run waits for an open launcher run to end. The common case is
/// a patch started right after boot, which the logon run catches midway; a
/// patch takes about an hour. The cap leaves the regen and the CI wait room
/// inside the task's four-hour execution limit.
const PATCH_WAIT: Duration = Duration::from_secs(2 * 60 * 60);

/// How often a waiting run asks the launcher log again.
const PATCH_POLL: Duration = Duration::from_secs(60);

/// What the installs look like right now.
struct Survey {
    /// Settled installs with a launcher label for the build on disk.
    builds: Vec<Build>,
    /// Installs the launcher has a run open on (possibly paused), as
    /// `"LIVE (4.10.3-live.13165946)"`.
    patching: Vec<String>,
    /// Every other install, with why it cannot be read.
    unusable: Vec<String>,
}

impl Survey {
    fn take() -> Result<Self, String> {
        let installs =
            sc_discovery::discover().map_err(|e| format!("discovering installs: {e}"))?;
        let mut survey = Survey {
            builds: Vec::new(),
            patching: Vec::new(),
            unusable: Vec::new(),
        };
        for install in installs {
            match install.patch_state() {
                Ok(sc_discovery::PatchState::Settled) => {}
                Ok(sc_discovery::PatchState::Patching(run)) => {
                    survey
                        .patching
                        .push(format!("{} ({})", install.channel, run.version_label));
                    continue;
                }
                Ok(state) => {
                    warn!(
                        channel = %install.channel,
                        root = %install.root.display(),
                        ?state,
                        "skipping install: the launcher left it half-patched"
                    );
                    survey
                        .unusable
                        .push(format!("{} left half-patched", install.channel));
                    continue;
                }
                Err(e) => {
                    warn!(
                        channel = %install.channel,
                        root = %install.root.display(),
                        error = %e,
                        "skipping install: cannot rule out a launcher patch"
                    );
                    survey.unusable.push(format!("{}: {e}", install.channel));
                    continue;
                }
            }
            let changelist = install
                .manifest
                .changelist
                .as_deref()
                .and_then(|c| c.parse::<u64>().ok());
            let (Some(changelist), Some(label)) =
                (changelist, install.launcher_version_label.clone())
            else {
                warn!(
                    channel = %install.channel,
                    root = %install.root.display(),
                    "skipping install: no changelist, or no launcher label for the build on disk \
                     (the launcher has not finished patching it?)"
                );
                survey.unusable.push(format!(
                    "{} has no launcher label for its build",
                    install.channel
                ));
                continue;
            };
            survey.builds.push(Build {
                label,
                changelist,
                channel: install.channel,
                p4k: install.data_p4k(),
            });
        }
        Ok(survey)
    }
}

/// Which builds a run regenerates from, or why it does nothing this time.
enum Pick {
    Window(Window),
    Skip(String),
}

/// The [`Window`] over the installs that carry a launcher label for the
/// build actually on disk, once no launcher run is open on any install.
///
/// An open run is waited out (up to [`PATCH_WAIT`]) rather than skipped:
/// the build it installs is one to regenerate. A run that ends with the
/// game started, a patch that outlasts the wait, or installs the launcher
/// left half-patched are skips, not failures; the next run tries again.
fn pick_window(options: &Options) -> Result<Pick, String> {
    let deadline = Instant::now() + PATCH_WAIT;
    let mut waited = false;
    let survey = loop {
        let survey = Survey::take()?;
        if survey.patching.is_empty() {
            break survey;
        }
        let patching = survey.patching.join(", ");
        if options.dry_run {
            return Ok(Pick::Skip(format!(
                "dry run: would wait for the launcher to finish patching {patching}"
            )));
        }
        if Instant::now() >= deadline {
            return Ok(Pick::Skip(format!(
                "{patching} still patching after {} min; skipped until the next run",
                PATCH_WAIT.as_secs() / 60
            )));
        }
        if !waited {
            info!(%patching, "waiting for the launcher to finish patching");
            waited = true;
        }
        std::thread::sleep(PATCH_POLL);
    };
    if waited {
        info!("launcher run ended");
        if !options.now && windows::game_running() {
            return Ok(Pick::Skip(
                "patch finished, game running; skipped until the next run".into(),
            ));
        }
    }
    if survey.builds.is_empty() && !survey.unusable.is_empty() {
        return Ok(Pick::Skip(format!(
            "no usable install ({}); skipped until the next run",
            survey.unusable.join(", ")
        )));
    }
    Window::from_builds(survey.builds)
        .map(Pick::Window)
        .ok_or_else(|| "no Star Citizen install found".into())
}

/// The `datacore/*` tag with the highest changelist, with the changelists
/// of every build it covers.
fn newest_datacore_tag(repo: &Path) -> Result<Option<(String, Vec<u64>)>, String> {
    let tags = capture(
        Command::new("git")
            .args(["tag", "--list", "datacore/*"])
            .current_dir(repo),
    )?;
    Ok(newest_tag(&tags))
}

/// The changelists a `datacore/*` tag covers, one per build label (the
/// number after the last `.`); a union tag joins its labels with `+`.
fn tag_changelists(tag: &str) -> Option<Vec<u64>> {
    tag.strip_prefix("datacore/")?
        .split('+')
        .map(|label| label.rsplit_once('.')?.1.parse().ok())
        .collect()
}

fn newest_tag(tags: &str) -> Option<(String, Vec<u64>)> {
    tags.lines()
        .filter_map(|t| {
            let changelists = tag_changelists(t.trim())?;
            Some((t.trim().to_string(), changelists))
        })
        .max_by_key(|(_, cls)| cls.iter().copied().max())
}

/// A regen PR as `gh` reports it.
struct Pr {
    url: String,
    open: bool,
    draft: bool,
    auto_merge: bool,
    /// The head commit.
    head: String,
    /// Opened from a fork rather than a branch of this repo.
    fork: bool,
}

const PR_FIELDS: &str = "url,state,isDraft,labels,headRefOid,isCrossRepository";

/// One PR as a line `url state isDraft has-label head fork`.
const PR_JQ: &str = r#""\(.url) \(.state) \(.isDraft) \(any(.labels[]; .name == "auto-merge")) \(.headRefOid) \(.isCrossRepository)""#;

impl Pr {
    fn parse(line: &str) -> Option<Pr> {
        let mut fields = line.split_whitespace();
        Some(Pr {
            url: fields.next()?.to_string(),
            open: fields.next()? == "OPEN",
            draft: fields.next()? == "true",
            auto_merge: fields.next()? == "true",
            head: fields.next()?.to_string(),
            fork: fields.next()? == "true",
        })
    }
}

/// Any PR, in any state, from this repo's `branch`. A closed one counts
/// too: rejecting a regen PR must not bring it back every three hours. PRs
/// from forks are ignored: anyone can name a fork branch `regen/<label>`.
fn find_pr(repo: &Path, branch: &str) -> Result<Option<Pr>, String> {
    let jq = format!("map(select(.isCrossRepository | not)) | .[0] | select(. != null) | {PR_JQ}");
    let out = capture(
        Command::new("gh")
            .args(["pr", "list", "--state", "all", "--head", branch])
            .args(["--json", PR_FIELDS, "--jq", &jq])
            .current_dir(repo),
    )?;
    Ok(Pr::parse(&out))
}

fn view_pr(dir: &Path, url: &str) -> Result<Option<Pr>, String> {
    let out = capture(
        Command::new("gh")
            .args(["pr", "view", url])
            .args(["--json", PR_FIELDS, "--jq", PR_JQ])
            .current_dir(dir),
    )?;
    Ok(Pr::parse(&out))
}

/// Create the worktree on first use, then reset it to `base` (normally
/// `origin/main`) on a fresh `branch`. Ignored files (the `target/` dir)
/// survive, so later runs build incrementally.
fn prepare_worktree(
    repo: &Path,
    worktree: &Path,
    branch: &str,
    base: &str,
    log: &File,
) -> Result<(), String> {
    let path = worktree.to_string_lossy().into_owned();
    if !worktree.join(".git").exists() {
        git(repo, &["worktree", "prune"], log)?;
        git(
            repo,
            &["worktree", "add", "--quiet", "--detach", &path, base],
            log,
        )?;
    }
    git(
        worktree,
        &["checkout", "--quiet", "--force", "-B", branch, base],
        log,
    )?;
    git(worktree, &["clean", "--quiet", "-fd"], log)?;
    Ok(())
}

fn git(dir: &Path, args: &[&str], log: &File) -> Result<(), String> {
    logged(Command::new("git").args(args).current_dir(dir), log)
}

fn cargo(dir: &Path, args: &[&str], log: &File) -> Result<(), String> {
    logged(cargo_command().args(args).current_dir(dir), log)
}

/// `cargo` tuned for a throwaway build. The generated crate is rewritten every
/// run, so incremental state never pays off, and debug info turned every
/// example into a ~250 MB PDB (tens of GB under `target/debug`). Panic
/// locations don't need debug info, so the check output loses nothing.
fn cargo_command() -> Command {
    let mut cmd = Command::new("cargo");
    cmd.env("CARGO_INCREMENTAL", "0")
        .env("CARGO_PROFILE_DEV_DEBUG", "0")
        .env("CARGO_PROFILE_TEST_DEBUG", "0");
    cmd
}

/// Run with stdout and stderr going to the run log.
fn logged(cmd: &mut Command, log: &File) -> Result<(), String> {
    let name = describe(cmd);
    let out = log.try_clone().map_err(|e| e.to_string())?;
    let err = log.try_clone().map_err(|e| e.to_string())?;
    let status = cmd
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .status()
        .map_err(|e| format!("starting `{name}`: {e}"))?;
    status
        .success()
        .then_some(())
        .ok_or_else(|| format!("`{name}` failed ({status}); see last-run.log"))
}

/// Run and return stdout; stderr goes into the error on failure.
fn capture(cmd: &mut Command) -> Result<String, String> {
    let name = describe(cmd);
    let out = cmd
        .output()
        .map_err(|e| format!("starting `{name}`: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "`{name}` failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Run a cargo check; a failure is a result to report, not an error.
fn check(dir: &Path, name: &str, args: &[&str], log: &File) -> Check {
    info!(check = name, "running");
    let result = cargo_command().args(args).current_dir(dir).output();
    let (passed, text) = match result {
        Ok(o) => {
            let mut text = String::from_utf8_lossy(&o.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&o.stderr));
            (o.status.success(), text)
        }
        Err(e) => (false, format!("starting cargo: {e}")),
    };
    let mut log = log;
    let _ = writeln!(log, "---- {name} ----\n{text}");
    if !passed {
        warn!(check = name, "failed");
    }
    let lines: Vec<&str> = text.lines().collect();
    let tail = lines[lines.len().saturating_sub(40)..].join("\n");
    Check {
        name: name.to_string(),
        passed,
        tail,
    }
}

fn describe(cmd: &Command) -> String {
    let mut s = cmd.get_program().to_string_lossy().into_owned();
    for arg in cmd.get_args() {
        s.push(' ');
        s.push_str(&arg.to_string_lossy());
    }
    s
}

fn state_dir() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("sc-regen")
}

fn append_history(state: &Path, line: &str) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(state.join("history.log"))
    {
        let _ = writeln!(f, "unix:{secs} {line}");
    }
}

/// A lock file held for the run. One left behind by a crashed run is
/// taken over once it is older than the task's four-hour time limit.
struct Lock(PathBuf);

impl Lock {
    fn acquire(path: &Path) -> Option<Lock> {
        let stale = std::fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.elapsed().ok())
            .is_some_and(|age| age.as_secs() > 4 * 3600);
        if stale {
            let _ = std::fs::remove_file(path);
        }
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .ok()
            .map(|_| Lock(path.to_path_buf()))
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_pr_line() {
        let pr = Pr::parse("https://github.com/o/r/pull/2 OPEN false true abc123 false\n").unwrap();
        assert_eq!(pr.url, "https://github.com/o/r/pull/2");
        assert_eq!(pr.head, "abc123");
        assert!(pr.open && !pr.draft && pr.auto_merge && !pr.fork);
        let fork = Pr::parse("https://github.com/o/r/pull/2 MERGED false false abc true").unwrap();
        assert!(!fork.open && !fork.auto_merge && fork.fork);
        assert!(Pr::parse("").is_none());
        assert!(Pr::parse("https://github.com/o/r/pull/2 OPEN false true").is_none());
    }

    fn build(channel: Channel, changelist: u64) -> Build {
        let name = format!("{channel}").to_lowercase();
        Build {
            label: format!("1.0.0-{name}.{changelist}"),
            changelist,
            channel,
            p4k: PathBuf::from(format!("C:/SC/{name}/Data.p4k")),
        }
    }

    fn changelists(window: &Window) -> Vec<u64> {
        window.builds.iter().map(|b| b.changelist).collect()
    }

    #[test]
    fn the_window_is_live_plus_a_newer_preview() {
        use Channel::*;
        let window = |builds| Window::from_builds(builds).unwrap();
        // PTU ahead of LIVE: both, oldest first.
        let w = window(vec![build(Ptu, 20), build(Live, 10)]);
        assert_eq!(changelists(&w), [10, 20]);
        assert!(w.is_live());
        // A stale PTU behind LIVE drops out.
        assert_eq!(
            changelists(&window(vec![build(Ptu, 5), build(Live, 10)])),
            [10]
        );
        // The newest preview of several; Tech Preview never counts.
        let w = window(vec![
            build(Live, 10),
            build(Ptu, 20),
            build(Eptu, 30),
            build(TechPreview, 40),
        ]);
        assert_eq!(changelists(&w), [10, 30]);
        // A hotfix newer than LIVE is the LIVE side.
        assert_eq!(
            changelists(&window(vec![build(Live, 10), build(Hotfix, 12)])),
            [12]
        );
        // No LIVE install: the newest build alone, not live.
        let w = window(vec![build(Ptu, 20), build(TechPreview, 40)]);
        assert_eq!(changelists(&w), [40]);
        assert!(!w.is_live());
        assert!(Window::from_builds(Vec::new()).is_none());
    }

    #[test]
    fn a_union_names_every_build() {
        let w =
            Window::from_builds(vec![build(Channel::Live, 10), build(Channel::Ptu, 20)]).unwrap();
        assert_eq!(w.label(), "1.0.0-live.10 + 1.0.0-ptu.20");
        assert_eq!(w.branch(), "regen/1.0.0-live.10+1.0.0-ptu.20");
        assert_eq!(
            w.commit_message(),
            "chore(datacore): regenerate bindings (SC 1.0.0-live.10 + 1.0.0-ptu.20)"
        );
    }

    #[test]
    fn reads_union_tags() {
        assert_eq!(
            tag_changelists("datacore/4.10.3-live.13165946+5.0.0-ptu.13228337"),
            Some(vec![13165946, 13228337])
        );
        assert_eq!(
            tag_changelists("datacore/4.9.0-live.12344265"),
            Some(vec![12344265])
        );
        assert_eq!(tag_changelists("datacore/garbage"), None);
        let tags = "datacore/4.10.3-live.30\ndatacore/4.10.3-live.30+5.0.0-ptu.40\ndatacore/4.9.0-live.20\n";
        assert_eq!(
            newest_tag(tags),
            Some(("datacore/4.10.3-live.30+5.0.0-ptu.40".into(), vec![30, 40]))
        );
    }

    #[test]
    fn coverage_follows_the_tagged_builds() {
        use Channel::*;
        let window = |builds| Window::from_builds(builds).unwrap();
        let tagged = [30, 40];
        // Exactly the tagged builds, or the PTU uninstalled since: covered.
        assert!(window(vec![build(Live, 30), build(Ptu, 40)]).covered_by(&tagged));
        assert!(window(vec![build(Live, 30)]).covered_by(&tagged));
        // PTU promoted to LIVE under the same changelist: covered.
        assert!(window(vec![build(Live, 40)]).covered_by(&tagged));
        // A new PTU build, a LIVE hotfix, or LIVE moving on: not covered.
        assert!(!window(vec![build(Live, 30), build(Ptu, 45)]).covered_by(&tagged));
        assert!(!window(vec![build(Live, 35), build(Ptu, 40)]).covered_by(&tagged));
        assert!(!window(vec![build(Live, 50)]).covered_by(&tagged));
        // Older than every tagged build (tagged elsewhere): covered.
        assert!(window(vec![build(Live, 20)]).covered_by(&tagged));
        // The plain single-build tag keeps its old meaning.
        assert!(window(vec![build(Live, 30)]).covered_by(&[30]));
        assert!(!window(vec![build(Live, 30), build(Ptu, 40)]).covered_by(&[30]));
    }

    #[test]
    fn ci_passes_only_on_the_required_check() {
        let pass = |s| matches!(ci_verdict(s), Some(Ci::Passed));
        let fail = |s| matches!(ci_verdict(s), Some(Ci::Failed));
        assert!(pass("pass ci / check\n"));
        assert!(pass("pass ci / check\nskipping other\n"));
        // Nothing yet, only skipped, or another check alone: undecided.
        assert!(ci_verdict("").is_none());
        assert!(ci_verdict("skipping ci / check\n").is_none());
        assert!(ci_verdict("pass lint\n").is_none());
        assert!(ci_verdict("pass ci / check\npending other\n").is_none());
        assert!(fail("pass ci / check\nfail other\n"));
        assert!(fail("cancel ci / check\n"));
    }
}
