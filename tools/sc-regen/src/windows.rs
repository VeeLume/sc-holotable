//! The Windows side: the scheduled task, the toast, and whether the game is
//! running. Everything goes through Windows PowerShell 5.1 (`powershell.exe`),
//! which ships with Windows and, unlike `pwsh`, can load WinRT types for the
//! toast. Values reach the scripts through environment variables, never by
//! splicing them into script text, so paths need no quoting.

use std::path::Path;
use std::process::Command;

pub const TASK_NAME: &str = "sc-regen";

/// Register (or replace) the scheduled task for the current user: every
/// three hours from 09:00, plus ten minutes after logon. It runs only while
/// the user is logged on, so `git` and `gh` find their stored credentials,
/// below normal priority, and through `conhost --headless` so a run that
/// compiles for half an hour doesn't hold a console window open.
pub fn register_task(exe: &Path, repo: &Path) -> Result<(), String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$arguments = '--headless "' + $env:SC_REGEN_EXE + '" --repo "' + $env:SC_REGEN_REPO + '"'
$action = New-ScheduledTaskAction -Execute 'conhost.exe' -Argument $arguments
$daily = New-ScheduledTaskTrigger -Daily -At 09:00
$daily.Repetition = (New-ScheduledTaskTrigger -Once -At 09:00 -RepetitionInterval (New-TimeSpan -Hours 3) -RepetitionDuration (New-TimeSpan -Hours 15)).Repetition
$logon = New-ScheduledTaskTrigger -AtLogOn -User $env:USERNAME
$logon.Delay = 'PT10M'
$settings = New-ScheduledTaskSettingsSet -StartWhenAvailable -RunOnlyIfNetworkAvailable -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -MultipleInstances IgnoreNew -Priority 7 -ExecutionTimeLimit (New-TimeSpan -Hours 4)
Register-ScheduledTask -TaskName $env:SC_REGEN_TASK -Action $action -Trigger @($daily, $logon) -Settings $settings -Description 'Regenerates the sc-holotable DataCore bindings when Star Citizen patches and opens a PR.' -Force | Out-Null
"#;
    powershell(script)
        .env("SC_REGEN_EXE", exe)
        .env("SC_REGEN_REPO", repo)
        .env("SC_REGEN_TASK", TASK_NAME)
        .status()
        .map_err(|e| format!("starting powershell.exe: {e}"))?
        .success()
        .then_some(())
        .ok_or_else(|| "Register-ScheduledTask failed (output above)".into())
}

/// Best effort: a toast under Windows PowerShell's app id, which is
/// registered on every Windows install (an unregistered id shows nothing).
pub fn notify(title: &str, body: &str) {
    let script = r#"
[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
$xml = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02)
$text = $xml.GetElementsByTagName('text')
$text.Item(0).AppendChild($xml.CreateTextNode($env:SC_REGEN_TITLE)) | Out-Null
$text.Item(1).AppendChild($xml.CreateTextNode($env:SC_REGEN_BODY)) | Out-Null
$app = '{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe'
[Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier($app).Show([Windows.UI.Notifications.ToastNotification]::new($xml))
"#;
    let result = powershell(script)
        .env("SC_REGEN_TITLE", title)
        .env("SC_REGEN_BODY", body)
        .output();
    if !matches!(&result, Ok(o) if o.status.success()) {
        tracing::warn!("toast notification failed");
    }
}

/// Whether `StarCitizen.exe` is running: a regen compiles for a long time,
/// and patch day is when the game is being played.
pub fn game_running() -> bool {
    Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq StarCitizen.exe", "/NH"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("StarCitizen.exe"))
        .unwrap_or(false)
}

fn powershell(script: &str) -> Command {
    let mut cmd = Command::new("powershell.exe");
    cmd.args(["-NoProfile", "-NonInteractive", "-Command", script]);
    cmd
}
