//! `sc-regen` — keeps the generated DataCore bindings in step with the game.
//!
//! A Windows scheduled task runs it every few hours. Each run:
//!
//! 1. Finds the newest Star Citizen build installed, by the changelist in
//!    its `build_manifest.id`. Only installs whose launcher label matches
//!    that changelist count: a folder renamed between channels (LIVE moved
//!    to PTU to seed a patch) is skipped until the launcher has patched it.
//!    So is an install the launcher is patching or left half-patched
//!    (`Installation::patch_state`); the check runs again once the generator
//!    has read `Data.p4k`, and a change in between fails the run.
//! 2. Stops if that build is no newer than the newest `datacore/*` tag, if
//!    a PR for it already exists (open, merged or closed), or if the game is
//!    running.
//! 3. Regenerates in its own worktree (`%LOCALAPPDATA%\sc-regen\worktree`,
//!    reset to `origin/main`), never in a development checkout: generator,
//!    fmt, clippy `--fix` on the generated crate, one commit on
//!    `regen/<label>`.
//! 4. Runs `cargo test --workspace` and the smoke examples, pushes the
//!    branch and opens a PR whose body reports what changed. The PR is a
//!    draft when a check failed or the build is not LIVE.
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

use report::{Bindings, BuildInfo, Check};

const GENERATED: &str = "crates/sc-extract-generated/src/generated";

struct Options {
    repo: PathBuf,
    now: bool,
    dry_run: bool,
    no_publish: bool,
    schedule: bool,
}

/// The newest installed build, as the regen will name it.
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

    fn branch(&self) -> String {
        format!("regen/{}", self.label)
    }

    fn commit_message(&self) -> String {
        format!("chore(datacore): regenerate bindings (SC {})", self.label)
    }
}

fn main() -> ExitCode {
    let options = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("sc-regen: {e}");
            eprintln!(
                "usage: sc-regen [schedule] [--repo <path>] [--now] [--dry-run] [--no-publish]"
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
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "schedule" => options.schedule = true,
            "--repo" => options.repo = args.next().ok_or("--repo requires a path")?.into(),
            "--now" => options.now = true,
            "--dry-run" => options.dry_run = true,
            "--no-publish" => options.no_publish = true,
            other => return Err(format!("unknown argument: {other}")),
        }
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
    let build = newest_build()?;
    info!(
        label = %build.label,
        channel = %build.channel,
        changelist = build.changelist,
        p4k = %build.p4k.display(),
        "newest installed build"
    );

    let repo = &options.repo;
    git(
        repo,
        &["fetch", "--quiet", "--prune", "--tags", "origin"],
        log,
    )?;
    let previous_tag = newest_datacore_tag(repo)?;
    if !options.now {
        if let Some((tag, changelist)) = &previous_tag
            && build.changelist <= *changelist
        {
            return Ok(format!(
                "{} is covered by {tag}; nothing to do",
                build.label
            ));
        }
        if let Some(pr) = find_pr(repo, &build.branch())? {
            if pr.open && pr.auto_merge && !options.dry_run {
                // Only the commit this machine pushed may be merged; the
                // worktree's branch still holds it from the interrupted run.
                let pushed = capture(
                    Command::new("git")
                        .args(["rev-parse", "--verify", "--quiet"])
                        .arg(format!("refs/heads/{}", build.branch()))
                        .current_dir(state.join("worktree")),
                );
                let Ok(pushed) = pushed else {
                    return Ok(format!(
                        "{} has {} but no local record of the pushed commit; left for review",
                        build.label, pr.url
                    ));
                };
                info!(pr = %pr.url, "resuming the merge of an open auto-merge PR");
                return land(repo, &pr.url, pushed.trim(), &build, log);
            }
            return Ok(format!(
                "{} already has {}; nothing to do",
                build.label, pr.url
            ));
        }
    }
    if options.dry_run {
        return Ok(format!(
            "dry run: would regenerate {} (previous tag {})",
            build.label,
            previous_tag.as_ref().map_or("none", |(t, _)| t.as_str())
        ));
    }

    let worktree = state.join("worktree");
    prepare_worktree(repo, &worktree, &build.branch(), log)?;
    let generated = worktree.join(GENERATED);
    let before = Bindings::read(&generated)?;

    info!("running sc-generator");
    let p4k = build.p4k.to_string_lossy().into_owned();
    cargo(
        &worktree,
        &[
            "run",
            "-p",
            "sc-generator",
            "--release",
            "--",
            "--p4k",
            &p4k,
        ],
        log,
    )?;
    // The launcher rewrites Data.p4k in place; a patch that started while the
    // generator read it leaves bindings from a torn file.
    match newest_build() {
        Ok(now) if now.label == build.label && now.p4k == build.p4k => {}
        _ => {
            return Err(format!(
                "{} changed while the generator read it (did the launcher start patching?); \
                 the next run retries",
                build.label
            ));
        }
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
        &["commit", "--quiet", "-m", &build.commit_message()],
        log,
    )?;

    let checks = [
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
    let all_passed = checks.iter().all(|c| c.passed);

    let info = BuildInfo {
        label: &build.label,
        channel: &build.channel.to_string(),
        changelist: build.changelist,
        previous_tag: previous_tag.as_ref().map(|(t, _)| t.as_str()),
    };
    let blockers = report::auto_merge_blockers(build.is_live(), &before, &after, &checks);
    let body = report::pr_body(&info, &before, &after, &checks, &blockers);
    let body_file = state.join("pr-body.md");
    std::fs::write(&body_file, &body).map_err(|e| format!("writing the PR body: {e}"))?;

    if options.no_publish {
        return Ok(format!(
            "regenerated {} locally (checks {}, {}); PR body in {}",
            build.label,
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

    let branch = build.branch();
    let mut push = vec!["push", "--quiet", "-u", "origin", branch.as_str()];
    if options.now {
        push.push("--force");
    }
    git(&worktree, &push, log)?;
    let title = build.commit_message();
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
    let draft = !all_passed || !build.is_live();
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
        return land(&worktree, &url, pushed.trim(), &build, log);
    }
    let summary = format!(
        "regen PR for {} opened for review ({}): {url}",
        build.label,
        blockers.join("; ")
    );
    windows::notify(&format!("Bindings for {}", build.label), &summary);
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
fn land(dir: &Path, url: &str, pushed: &str, build: &Build, log: &File) -> Result<String, String> {
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
            let summary = format!(
                "regen PR for {}: GitHub CI failed, left for review: {url}",
                build.label
            );
            windows::notify(&format!("Bindings for {}", build.label), &summary);
            return Ok(summary);
        }
        Ci::TimedOut => {
            return Ok(format!(
                "regen PR for {}: GitHub CI still running; the next run merges it: {url}",
                build.label
            ));
        }
    }
    let Some(pr) = view_pr(dir, url)? else {
        return Err(format!("{url} vanished while waiting for CI"));
    };
    if !(pr.open && pr.auto_merge && !pr.draft) {
        return Ok(format!(
            "regen PR for {} is no longer marked {AUTO_MERGE_LABEL}; left for review: {url}",
            build.label
        ));
    }
    if pr.fork || pr.head != pushed {
        return Ok(format!(
            "regen PR for {} no longer points at the pushed commit {pushed}; left for review: {url}",
            build.label
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
        &["push", "--quiet", "origin", "--delete", &build.branch()],
        log,
    ) {
        warn!("deleting the merged branch: {e}");
    }
    let summary = format!(
        "regen PR for {} merged; the release workflow tags and releases it: {url}",
        build.label
    );
    windows::notify(&format!("Bindings for {}", build.label), &summary);
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

/// The newest installed build that carries a launcher label for the build
/// actually on disk, and that the launcher is not patching or left
/// half-patched.
fn newest_build() -> Result<Build, String> {
    let installs = sc_discovery::discover().map_err(|e| format!("discovering installs: {e}"))?;
    let mut builds = Vec::new();
    for install in installs {
        match install.patch_state() {
            Ok(sc_discovery::PatchState::Settled) => {}
            Ok(state) => {
                warn!(
                    channel = %install.channel,
                    root = %install.root.display(),
                    ?state,
                    "skipping install: the launcher is patching it or left it half-patched"
                );
                continue;
            }
            Err(e) => {
                warn!(
                    channel = %install.channel,
                    root = %install.root.display(),
                    error = %e,
                    "skipping install: cannot rule out a launcher patch"
                );
                continue;
            }
        }
        let changelist = install
            .manifest
            .changelist
            .as_deref()
            .and_then(|c| c.parse::<u64>().ok());
        let (Some(changelist), Some(label)) = (changelist, install.launcher_version_label.clone())
        else {
            warn!(
                channel = %install.channel,
                root = %install.root.display(),
                "skipping install: no changelist, or no launcher label for the build on disk \
                 (the launcher has not finished patching it?)"
            );
            continue;
        };
        builds.push(Build {
            label,
            changelist,
            channel: install.channel,
            p4k: install.data_p4k(),
        });
    }
    builds
        .into_iter()
        .max_by_key(|b| b.changelist)
        .ok_or_else(|| "no install with a launcher label for the build on disk".into())
}

/// The `datacore/*` tag with the highest changelist (its trailing number).
fn newest_datacore_tag(repo: &Path) -> Result<Option<(String, u64)>, String> {
    let tags = capture(
        Command::new("git")
            .args(["tag", "--list", "datacore/*"])
            .current_dir(repo),
    )?;
    Ok(tags
        .lines()
        .filter_map(|t| {
            let changelist = t.rsplit_once('.')?.1.parse::<u64>().ok()?;
            Some((t.to_string(), changelist))
        })
        .max_by_key(|(_, cl)| *cl))
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

/// Create the worktree on first use, then reset it to `origin/main` on a
/// fresh `branch`. Ignored files (the `target/` dir) survive, so later runs
/// build incrementally.
fn prepare_worktree(repo: &Path, worktree: &Path, branch: &str, log: &File) -> Result<(), String> {
    let path = worktree.to_string_lossy().into_owned();
    if !worktree.join(".git").exists() {
        git(repo, &["worktree", "prune"], log)?;
        git(
            repo,
            &[
                "worktree",
                "add",
                "--quiet",
                "--detach",
                &path,
                "origin/main",
            ],
            log,
        )?;
    }
    git(
        worktree,
        &[
            "checkout",
            "--quiet",
            "--force",
            "-B",
            branch,
            "origin/main",
        ],
        log,
    )?;
    git(worktree, &["clean", "--quiet", "-fd"], log)?;
    Ok(())
}

fn git(dir: &Path, args: &[&str], log: &File) -> Result<(), String> {
    logged(Command::new("git").args(args).current_dir(dir), log)
}

fn cargo(dir: &Path, args: &[&str], log: &File) -> Result<(), String> {
    logged(Command::new("cargo").args(args).current_dir(dir), log)
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
    let result = Command::new("cargo").args(args).current_dir(dir).output();
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
