use std::io;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const DEFAULT_REMOTE: &str = "origin";
const BLEEDING_BRANCH: &str = "master";
const STABLE_BRANCH: &str = "stable";
const COMMAND_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Target {
    Bleeding,
    Stable,
}

impl Target {
    fn branch(self) -> &'static str {
        match self {
            Self::Bleeding => BLEEDING_BRANCH,
            Self::Stable => STABLE_BRANCH,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Bleeding => "bleeding edge",
            Self::Stable => "stable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    Plan,
    Check,
    Execute,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UpdateRequest {
    action: Action,
    target: Target,
    build: bool,
}

impl Default for UpdateRequest {
    fn default() -> Self {
        Self {
            action: Action::Plan,
            target: Target::Bleeding,
            build: false,
        }
    }
}

pub fn run(args: &[String]) -> String {
    let request = match parse_args(args) {
        Ok(Some(request)) => request,
        Ok(None) => return help(),
        Err(err) => return format!("update: {err}\n{}", help()),
    };

    match request.action {
        Action::Plan => plan(request.target, request.build),
        Action::Check => guarded_check(request.target),
        Action::Execute => guarded_execute(request.target, request.build),
    }
}

fn guarded_check(target: Target) -> String {
    if !crate::policy::host_tools_allowed() {
        return format!(
            "update: {}\n{}",
            crate::policy::host_denial_message("update"),
            plan(target, false)
        );
    }

    let mut out = format!("phase1 updater // check {}\n", target.label());
    out.push_str("host tools : enabled\n");
    out.push_str("privacy    : command output is sanitized; credentials and tokens are redacted\n");
    out.push_str(&run_git_summary(&["rev-parse", "--is-inside-work-tree"], "git repo"));
    out.push_str(&run_git_summary(&["rev-parse", "--abbrev-ref", "HEAD"], "branch"));
    out.push_str(&run_git_summary(&["log", "-1", "--oneline"], "head"));
    out.push_str(&run_git_summary(
        &["status", "--short", "--branch", "--untracked-files=no"],
        "status",
    ));
    out.push_str(&format!(
        "target     : {}/{}\n",
        DEFAULT_REMOTE,
        target.branch()
    ));
    out
}

fn guarded_execute(target: Target, build: bool) -> String {
    if !crate::policy::host_tools_allowed() {
        return format!(
            "update: {}\n{}",
            crate::policy::host_denial_message("update"),
            plan(target, build)
        );
    }

    let mut out = format!("phase1 updater // execute {}\n", target.label());
    out.push_str("mode       : guarded host git update\n");
    out.push_str("privacy    : command output is sanitized; credentials and tokens are redacted\n");

    if let Err(err) = ensure_git_repo() {
        out.push_str(&format!("update: {err}\n"));
        return out;
    }

    match dirty_status() {
        Ok(Some(status)) => {
            out.push_str("update: blocked because tracked local changes exist\n");
            out.push_str("action    : commit/stash your changes, then run update again\n");
            out.push_str("status    :\n");
            out.push_str(&status);
            return out;
        }
        Ok(None) => out.push_str("worktree  : clean tracked files\n"),
        Err(err) => {
            out.push_str(&format!("update: could not inspect worktree: {err}\n"));
            return out;
        }
    }

    for step in update_steps(target) {
        let (step_out, ok) = run_step(step.0, &step.1);
        out.push_str(&step_out);
        if !ok {
            return out;
        }
    }

    if build {
        let (step_out, ok) = run_step("cargo build --release", &["cargo", "build", "--release"]);
        out.push_str(&step_out);
        if !ok {
            return out;
        }
    }

    out.push_str("update: complete; restart phase1 to run the updated code\n");
    out
}

fn plan(target: Target, build: bool) -> String {
    let branch = target.branch();
    let mut out = format!("phase1 updater // plan {}\n\n", target.label());
    out.push_str("safe default : this command does not modify files unless --execute is provided\n");
    out.push_str("guard        : --execute requires PHASE1_SAFE_MODE=0 and PHASE1_ALLOW_HOST_TOOLS=1\n");
    out.push_str("privacy      : updater never asks for passwords, tokens, email credentials, cookies, or keys\n");
    out.push_str("local safety : tracked local changes block the update instead of being overwritten\n\n");
    out.push_str("manual commands:\n");
    out.push_str("  git status --short --branch --untracked-files=no\n");
    out.push_str(&format!("  git fetch --prune {DEFAULT_REMOTE} {branch}\n"));
    out.push_str(&format!("  git checkout {branch}\n"));
    out.push_str(&format!("  git pull --ff-only {DEFAULT_REMOTE} {branch}\n"));
    if build {
        out.push_str("  cargo build --release\n");
    }
    out.push_str("\ninside phase1:\n");
    out.push_str(&format!("  update {} --check\n", target_arg(target)));
    out.push_str(&format!(
        "  update {} --execute{}\n",
        target_arg(target),
        if build { " --build" } else { "" }
    ));
    out
}

fn parse_args(args: &[String]) -> Result<Option<UpdateRequest>, String> {
    let mut request = UpdateRequest::default();
    for arg in args {
        match arg.as_str() {
            "help" | "--help" | "-h" => return Ok(None),
            "plan" | "--plan" => request.action = Action::Plan,
            "check" | "--check" | "status" => request.action = Action::Check,
            "execute" | "--execute" | "apply" => request.action = Action::Execute,
            "bleeding" | "edge" | "master" | "main" => request.target = Target::Bleeding,
            "stable" | "release" => request.target = Target::Stable,
            "--build" | "build" => request.build = true,
            "--no-build" => request.build = false,
            other => return Err(format!("unknown option '{other}'")),
        }
    }
    Ok(Some(request))
}

fn update_steps(target: Target) -> Vec<(&'static str, Vec<&'static str>)> {
    let branch = target.branch();
    vec![
        (
            "git fetch",
            vec!["git", "fetch", "--prune", DEFAULT_REMOTE, branch],
        ),
        ("git checkout", vec!["git", "checkout", branch]),
        (
            "git pull --ff-only",
            vec!["git", "pull", "--ff-only", DEFAULT_REMOTE, branch],
        ),
    ]
}

fn ensure_git_repo() -> Result<(), String> {
    let output = run_command("git", &["rev-parse", "--is-inside-work-tree"], COMMAND_TIMEOUT)
        .map_err(|err| format!("git repo check failed: {err}"))?;
    if output.status.success() && sanitize_output(&output.stdout).trim() == "true" {
        Ok(())
    } else {
        Err("current directory is not a Git working tree".to_string())
    }
}

fn dirty_status() -> Result<Option<String>, String> {
    let output = run_command(
        "git",
        &["status", "--porcelain", "--untracked-files=no"],
        COMMAND_TIMEOUT,
    )
    .map_err(|err| err.to_string())?;
    if !output.status.success() {
        return Err(sanitize_combined(&output));
    }
    let status = sanitize_output(&output.stdout);
    if status.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(status))
    }
}

fn run_git_summary(args: &[&str], label: &str) -> String {
    match run_command("git", args, COMMAND_TIMEOUT) {
        Ok(output) if output.status.success() => {
            let value = sanitize_combined(&output);
            let value = value.trim();
            if value.is_empty() {
                format!("{label:<11}: ok\n")
            } else {
                format!("{label:<11}: {}\n", value.lines().next().unwrap_or("ok"))
            }
        }
        Ok(output) => format!(
            "{label:<11}: unavailable ({})\n",
            sanitize_combined(&output).trim()
        ),
        Err(err) => format!("{label:<11}: unavailable ({err})\n"),
    }
}

fn run_step(label: &str, command: &[&str]) -> (String, bool) {
    let Some((program, args)) = command.split_first() else {
        return (format!("{label:<19} [failed]\n"), false);
    };
    match run_command(program, args, COMMAND_TIMEOUT) {
        Ok(output) if output.status.success() => (format!("{label:<19} [ok]\n"), true),
        Ok(output) => {
            let mut out = format!("{label:<19} [failed]\n");
            let details = sanitize_combined(&output);
            let details = details.trim();
            if !details.is_empty() {
                out.push_str("details:\n");
                for line in details.lines().take(10) {
                    out.push_str("  ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
            (out, false)
        }
        Err(err) => (
            format!("{label:<19} [failed]\ndetails:\n  {err}\n"),
            false,
        ),
    }
}

fn run_command(program: &str, args: &[&str], timeout: Duration) -> io::Result<Output> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let start = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
        }
        thread::sleep(Duration::from_millis(25));
    }
}

fn sanitize_combined(output: &Output) -> String {
    let mut raw = String::new();
    raw.push_str(&String::from_utf8_lossy(&output.stdout));
    raw.push_str(&String::from_utf8_lossy(&output.stderr));
    sanitize_output(raw.as_bytes())
}

fn sanitize_output(bytes: &[u8]) -> String {
    let raw = String::from_utf8_lossy(bytes);
    raw.split_whitespace()
        .map(sanitize_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn sanitize_token(token: &str) -> String {
    let mut out = token.to_string();
    for prefix in ["ghp_", "gho_", "ghu_", "ghs_", "ghr_", "github_pat_"] {
        if out.contains(prefix) {
            return "[redacted-token]".to_string();
        }
    }
    if let Some(proto_pos) = out.find("://") {
        let auth_start = proto_pos + 3;
        if let Some(at_offset) = out[auth_start..].find('@') {
            let at = auth_start + at_offset;
            out.replace_range(auth_start..at, "[redacted-credential]");
        }
    }
    out
}

fn target_arg(target: Target) -> &'static str {
    match target {
        Target::Bleeding => "bleeding",
        Target::Stable => "stable",
    }
}

fn help() -> String {
    "usage: update [plan|check|--execute] [bleeding|stable] [--build]\n\nupdate defaults to a safe dry-run plan.\n--execute runs guarded git fetch/checkout/pull and requires PHASE1_SAFE_MODE=0 plus PHASE1_ALLOW_HOST_TOOLS=1.\n--build also runs cargo build --release after a successful update.\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::{run, sanitize_token};

    #[test]
    fn update_defaults_to_safe_plan() {
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        let out = run(&[]);
        assert!(out.contains("phase1 updater // plan bleeding edge"));
        assert!(out.contains("update bleeding --execute"));
        assert!(out.contains("never asks for passwords"));
    }

    #[test]
    fn update_execute_is_guarded() {
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        let out = run(&["bleeding".to_string(), "--execute".to_string()]);
        assert!(out.contains("disabled by safe boot profile"));
        assert!(out.contains("PHASE1_ALLOW_HOST_TOOLS"));
    }

    #[test]
    fn sanitizer_redacts_tokens_and_url_credentials() {
        assert_eq!(sanitize_token("ghp_abc123"), "[redacted-token]");
        assert_eq!(
            sanitize_token("https://user:secret@example.com/repo.git"),
            "https://[redacted-credential]@example.com/repo.git"
        );
    }
}
