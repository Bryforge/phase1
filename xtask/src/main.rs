use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_string());

    let result = match command.as_str() {
        "fmt" => run("cargo", &["fmt", "--all", "--", "--check"]),
        "check" => run("cargo", &["check", "--workspace", "--all-targets"]),
        "test" => run("cargo", &["test", "--workspace", "--all-targets"]),
        "doc" | "docs" => run("cargo", &["doc", "--workspace", "--no-deps"]),
        "security" => security_review(),
        "validate" | "all" => validate(),
        "help" | "-h" | "--help" => {
            print_help();
            Ok(())
        }
        other => Err(format!("unknown xtask command: {other}")),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("xtask: {err}");
            ExitCode::FAILURE
        }
    }
}

fn validate() -> Result<(), String> {
    run("cargo", &["fmt", "--all", "--", "--check"])?;
    run("cargo", &["check", "--workspace", "--all-targets"])?;
    run("cargo", &["test", "--workspace", "--all-targets"])?;
    Ok(())
}

fn security_review() -> Result<(), String> {
    println!("phase1 security review checklist");
    println!("- inspect src/policy.rs for safe-mode and host-tool gates");
    println!("- inspect src/ops_log.rs and src/history.rs for credential redaction");
    println!("- inspect src/registry.rs for command capability metadata");
    println!("- inspect SECURITY.md and SECURITY_REVIEW.md for user-facing claims");
    println!("- confirm host network mutation paths require explicit opt-in");
    println!("- confirm VFS-only editors do not gain host shell escape paths");
    println!();
    run("cargo", &["test", "--workspace", "policy", "history", "ops_log"])
}

fn run(program: &str, args: &[&str]) -> Result<(), String> {
    println!("$ {} {}", program, args.join(" "));
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|err| format!("failed to start {program}: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed with status {status}: {program} {}", args.join(" ")))
    }
}

fn print_help() {
    println!("phase1 xtask");
    println!("usage: cargo xtask <command>");
    println!();
    println!("commands:");
    println!("  fmt       check formatting");
    println!("  check     cargo check --workspace --all-targets");
    println!("  test      cargo test --workspace --all-targets");
    println!("  docs      cargo doc --workspace --no-deps");
    println!("  security  print security checklist and run targeted tests");
    println!("  validate  run fmt, check, and test");
}
