use std::io;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const STEP_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    Plan,
    Run,
    Doctor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Suite {
    Quick,
    Full,
    Smoke,
    Bleeding,
    Game,
    Fmt,
    Check,
    Clippy,
}

impl Suite {
    fn label(self) -> &'static str {
        match self {
            Self::Quick => "quick",
            Self::Full => "full",
            Self::Smoke => "smoke",
            Self::Bleeding => "bleeding",
            Self::Game => "game",
            Self::Fmt => "fmt",
            Self::Check => "check",
            Self::Clippy => "clippy",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Request {
    action: Action,
    suite: Suite,
    trust_host: bool,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            action: Action::Plan,
            suite: Suite::Full,
            trust_host: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestStep {
    label: &'static str,
    command: Vec<&'static str>,
}

pub fn is_request(args: &[String]) -> bool {
    args.iter().any(|arg| {
        matches!(
            arg.as_str(),
            "test"
                | "tests"
                | "devtest"
                | "validate"
                | "verify"
                | "qa"
                | "quick"
                | "full"
                | "smoke"
                | "game"
                | "arena"
                | "fmt"
                | "format"
                | "cargo-check"
                | "clippy"
                | "lint"
                | "doctor"
        )
    })
}

pub fn run(args: &[String]) -> String {
    let request = match parse_args(args) {
        Ok(Some(request)) => request,
        Ok(None) => return help(),
        Err(err) => return format!("update test: {err}\n{}", help()),
    };

    match request.action {
        Action::Plan => plan(request.suite),
        Action::Run => run_suite(request),
        Action::Doctor => doctor(request),
    }
}

fn parse_args(args: &[String]) -> Result<Option<Request>, String> {
    let mut request = Request::default();
    for arg in args {
        match arg.as_str() {
            "help" | "--help" | "-h" => return Ok(None),
            "test" | "tests" | "devtest" | "validate" | "verify" | "qa" => {}
            "plan" | "--plan" => request.action = Action::Plan,
            "execute" | "run" | "--execute" => request.action = Action::Run,
            "doctor" | "status" | "--check" => request.action = Action::Doctor,
            "quick" => request.suite = Suite::Quick,
            "full" | "all" => request.suite = Suite::Full,
            "smoke" => request.suite = Suite::Smoke,
            "bleeding" | "edge" => request.suite = Suite::Bleeding,
            "game" | "arena" => request.suite = Suite::Game,
            "fmt" | "format" => request.suite = Suite::Fmt,
            "cargo-check" => request.suite = Suite::Check,
            "clippy" | "lint" => request.suite = Suite::Clippy,
            "--trust-host" | "trust-host" | "trust" => request.trust_host = true,
            "--build" | "build" | "--nocapture" | "--no-build" => {}
            other => return Err(format!("unknown option '{other}'")),
        }
    }
    Ok(Some(request))
}

fn plan(suite: Suite) -> String {
    let mut out = String::from("phase1 developer test kit\n");
    out.push_str(&format!("suite        : {}\n", suite.label()));
    out.push_str("safe default : plan only; no host commands run without --execute\n");
    out.push_str("host guard   : execution requires SHIELD off, boot TRUST HOST on, and explicit --trust-host\n");
    out.push_str("purpose      : fast Rust/code validation from inside phase1\n\n");
    out.push_str("planned commands:\n");
    for step in suite_steps(suite) {
        out.push_str("  ");
        out.push_str(&step.command.join(" "));
        out.push('\n');
    }
    out.push_str("\ninside phase1:\n");
    out.push_str("  boot selector: turn SHIELD off and TRUST HOST on first\n");
    out.push_str("  update test quick --trust-host --execute\n");
    out.push_str("  update test full --trust-host --execute\n");
    out.push_str("  update test doctor --trust-host\n");
    out
}

fn run_suite(request: Request) -> String {
    if !request.trust_host {
        return format!(
            "update test: explicit --trust-host is required for host test execution\n{}",
            plan(request.suite)
        );
    }
    if !crate::policy::host_tools_allowed() {
        return format!(
            "update test: {}\n{}",
            crate::policy::host_denial_message("update test"),
            plan(request.suite)
        );
    }

    let mut out = format!("phase1 developer test kit // run {}\n", request.suite.label());
    out.push_str("host tools : enabled by boot TRUST HOST + --trust-host\n");
    let mut all_ok = true;
    for step in suite_steps(request.suite) {
        let (step_out, ok) = run_step(&step);
        out.push_str(&step_out);
        all_ok &= ok;
        if !ok {
            break;
        }
    }
    out.push_str(if all_ok {
        "result     : ok\n"
    } else {
        "result     : failed\n"
    });
    out
}

fn doctor(request: Request) -> String {
    if !request.trust_host {
        return format!(
            "update test: explicit --trust-host is required for host doctor checks\n{}",
            plan(Suite::Quick)
        );
    }
    if !crate::policy::host_tools_allowed() {
        return format!(
            "update test: {}\n{}",
            crate::policy::host_denial_message("update test"),
            plan(Suite::Quick)
        );
    }

    let mut out = String::from("phase1 developer test kit // doctor\n");
    out.push_str("host tools : enabled by boot TRUST HOST + --trust-host\n");
    for step in [
        TestStep {
            label: "cargo version",
            command: vec!["cargo", "--version"],
        },
        TestStep {
            label: "rustc version",
            command: vec!["rustc", "--version"],
        },
        TestStep {
            label: "git status",
            command: vec![
                "git",
                "status",
                "--short",
                "--branch",
                "--untracked-files=no",
            ],
        },
    ] {
        let (step_out, _) = run_step(&step);
        out.push_str(&step_out);
    }
    out
}

fn suite_steps(suite: Suite) -> Vec<TestStep> {
    match suite {
        Suite::Quick => vec![fmt_step(), check_step(), all_tests_step()],
        Suite::Full => vec![
            fmt_step(),
            check_step(),
            clippy_step(),
            all_tests_step(),
            smoke_step(),
            bleeding_step(),
            game_step(),
        ],
        Suite::Smoke => vec![smoke_step()],
        Suite::Bleeding => vec![bleeding_step()],
        Suite::Game => vec![game_step()],
        Suite::Fmt => vec![fmt_step()],
        Suite::Check => vec![check_step()],
        Suite::Clippy => vec![clippy_step()],
    }
}

fn fmt_step() -> TestStep {
    TestStep {
        label: "cargo fmt",
        command: vec!["cargo", "fmt", "--all", "--", "--check"],
    }
}

fn check_step() -> TestStep {
    TestStep {
        label: "cargo check",
        command: vec!["cargo", "check", "--all-targets"],
    }
}

fn clippy_step() -> TestStep {
    TestStep {
        label: "cargo clippy",
        command: vec!["cargo", "clippy", "--all-targets", "--", "-D", "warnings"],
    }
}

fn all_tests_step() -> TestStep {
    TestStep {
        label: "cargo test all",
        command: vec!["cargo", "test", "--all-targets"],
    }
}

fn smoke_step() -> TestStep {
    TestStep {
        label: "smoke tests",
        command: vec!["cargo", "test", "--test", "smoke", "--", "--nocapture"],
    }
}

fn bleeding_step() -> TestStep {
    TestStep {
        label: "bleeding tests",
        command: vec!["cargo", "test", "--test", "bleeding", "--", "--nocapture"],
    }
}

fn game_step() -> TestStep {
    TestStep {
        label: "game tests",
        command: vec!["cargo", "test", "--test", "game", "--", "--nocapture"],
    }
}

fn run_step(step: &TestStep) -> (String, bool) {
    let Some((program, args)) = step.command.split_first() else {
        return (format!("{:<16} [failed]\n", step.label), false);
    };
    match run_command(program, args, STEP_TIMEOUT) {
        Ok(output) if output.status.success() => (format!("{:<16} [ok]\n", step.label), true),
        Ok(output) => {
            let mut out = format!("{:<16} [failed]\n", step.label);
            let details = sanitized_output(&output);
            if !details.trim().is_empty() {
                out.push_str("details: ");
                out.push_str(details.lines().next().unwrap_or("unavailable"));
                out.push('\n');
            }
            (out, false)
        }
        Err(err) => (format!("{:<16} [failed]: {err}\n", step.label), false),
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

fn sanitized_output(output: &Output) -> String {
    let mut raw = String::new();
    raw.push_str(&String::from_utf8_lossy(&output.stdout));
    raw.push_str(&String::from_utf8_lossy(&output.stderr));
    raw.split_whitespace()
        .map(redact_sensitive_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn redact_sensitive_token(token: &str) -> String {
    if token.contains("ghp_")
        || token.contains("gho_")
        || token.contains("ghu_")
        || token.contains("ghs_")
        || token.contains("ghr_")
        || token.contains("github_pat_")
    {
        return "[redacted-token]".to_string();
    }
    if let Some(proto_pos) = token.find("://") {
        let auth_start = proto_pos + 3;
        if let Some(at_offset) = token[auth_start..].find('@') {
            let at = auth_start + at_offset;
            let mut out = token.to_string();
            out.replace_range(auth_start..at, "[redacted-credential]");
            return out;
        }
    }
    token.to_string()
}

fn help() -> String {
    "usage: update test [plan|quick|full|smoke|bleeding|game|fmt|cargo-check|clippy|doctor] [--execute] [--trust-host]\n\nRuns or plans developer validation suites from inside phase1.\nDefault suite is full and default action is a safe plan.\nExecution requires SHIELD off, boot TRUST HOST on, plus --trust-host.\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::{is_request, redact_sensitive_token, run};

    #[test]
    fn detects_developer_test_requests() {
        assert!(is_request(&["test".to_string()]));
        assert!(is_request(&["validate".to_string(), "full".to_string()]));
        assert!(is_request(&["quick".to_string(), "--execute".to_string()]));
        assert!(is_request(&["doctor".to_string()]));
        assert!(!is_request(&["latest".to_string()]));
    }

    #[test]
    fn developer_test_plan_lists_full_suite() {
        let out = run(&["test".to_string(), "full".to_string()]);
        assert!(out.contains("phase1 developer test kit"));
        assert!(out.contains("suite        : full"));
        assert!(out.contains("cargo fmt --all -- --check"));
        assert!(out.contains("cargo clippy --all-targets -- -D warnings"));
        assert!(out.contains("cargo test --test bleeding -- --nocapture"));
        assert!(out.contains("cargo test --test game -- --nocapture"));
        assert!(out.contains("update test full --trust-host --execute"));
    }

    #[test]
    fn quick_plan_keeps_feedback_loop_small() {
        let out = run(&["quick".to_string()]);
        assert!(out.contains("suite        : quick"));
        assert!(out.contains("cargo check --all-targets"));
        assert!(!out.contains("cargo test --test game"));
    }

    #[test]
    fn developer_test_execution_is_guarded() {
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        let out = run(&[
            "test".to_string(),
            "quick".to_string(),
            "--execute".to_string(),
        ]);
        assert!(out.contains("explicit --trust-host"));
        assert!(out.contains("TRUST HOST"));
    }

    #[test]
    fn developer_test_trust_flag_does_not_open_boot_gate() {
        std::env::set_var("PHASE1_SAFE_MODE", "0");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        let out = run(&[
            "test".to_string(),
            "quick".to_string(),
            "--execute".to_string(),
            "--trust-host".to_string(),
        ]);
        assert!(out.contains("PHASE1_ALLOW_HOST_TOOLS"));
        std::env::remove_var("PHASE1_SAFE_MODE");
    }

    #[test]
    fn doctor_requires_explicit_command_trust() {
        std::env::set_var("PHASE1_SAFE_MODE", "0");
        std::env::set_var("PHASE1_ALLOW_HOST_TOOLS", "1");
        let out = run(&["doctor".to_string()]);
        assert!(out.contains("explicit --trust-host"));
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
    }

    #[test]
    fn sanitizer_redacts_tokens_and_url_credentials() {
        assert_eq!(redact_sensitive_token("ghp_secret"), "[redacted-token]");
        assert_eq!(
            redact_sensitive_token("https://user:pass@example.com/repo.git"),
            "https://[redacted-credential]@example.com/repo.git"
        );
    }
}
