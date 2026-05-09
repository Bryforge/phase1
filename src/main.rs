#![allow(clippy::assertions_on_constants)]

mod autocomplete;
mod avim;
mod browser;
mod commands;
mod fastfetch;
mod history;
mod kernel;
mod languages;
mod learn;
mod line_editor;
mod linux_colors;
mod man;
mod matrix;
mod ned;
mod network;
mod operator;
mod ops_log;
mod policy;
mod registry;
mod text;
#[path = "boot_ui_static.rs"]
mod ui;
mod updater;
mod wasm;

use commands::{dispatch, parse_line, Phase1Shell};
use kernel::VfsNode;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

const PERSISTENT_STATE_PATH: &str = "phase1.state";
const NEST_EXIT_ALL_PATH: &str = "phase1.nest.exit-all";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const GRAY: &str = "\x1b[90m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ShellExit {
    Shutdown,
    Reboot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChainOp {
    Always,
    And,
    Or,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ChainSegment {
    op: ChainOp,
    command: String,
}

fn compact_path(path: &Path) -> String {
    let raw = path.display().to_string();
    if raw == "/home" {
        "~".to_string()
    } else if raw.starts_with("/home/") {
        raw.replacen("/home", "~", 1)
    } else {
        raw
    }
}

fn nested_level() -> u32 {
    std::env::var("PHASE1_NESTED_LEVEL")
        .ok()
        .and_then(|raw| raw.trim().parse::<u32>().ok())
        .unwrap_or(0)
}

fn nested_max() -> u32 {
    std::env::var("PHASE1_NESTED_MAX")
        .ok()
        .and_then(|raw| raw.trim().parse::<u32>().ok())
        .unwrap_or(1)
}

fn nest_exit_all_requested() -> bool {
    Path::new(NEST_EXIT_ALL_PATH).exists()
}

fn clear_nest_exit_all_if_root() {
    if nested_level() == 0 {
        let _ = fs::remove_file(NEST_EXIT_ALL_PATH);
    }
}

fn main() {
    ops_log::install_panic_hook();
    ops_log::log_event(
        "process.start",
        &format!("phase1 {}", env!("CARGO_PKG_VERSION")),
    );
    loop {
        match ui::configure_boot(kernel::VERSION) {
            ui::BootSelection::Boot(config) => match run_shell(config) {
                ShellExit::Shutdown => break,
                ShellExit::Reboot => continue,
            },
            ui::BootSelection::StorageTools(config) => {
                run_storage_boot_option(config);
                continue;
            }
            ui::BootSelection::Reboot => continue,
            ui::BootSelection::Quit => {
                ops_log::log_event("boot.abort", "phase1 did not enter main system");
                println!("boot aborted: phase1 did not enter the main system");
                return;
            }
        }
    }
    ops_log::log_event("process.stop", "phase1 exited");
}

fn run_storage_boot_option(boot_config: ui::BootConfig) {
    boot_config.apply();
    ops_log::log_event(
        "storage.boot_option",
        "opened read-only storage helper status",
    );

    println!("phase1 storage helper // boot option");
    println!("mode      : read-only status");
    println!(
        "safe mode : {}",
        if boot_config.safe_mode { "on" } else { "off" }
    );
    println!("hint      : mutating Git/Rust actions still require safe mode off and PHASE1_ALLOW_HOST_TOOLS=1");
    println!();

    let helper_name = if cfg!(windows) {
        "phase1-storage.exe"
    } else {
        "phase1-storage"
    };

    let helper_path = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|dir| dir.join(helper_name)));

    match helper_path {
        Some(path) if path.exists() => {
            match Command::new(&path)
                .arg("storage")
                .arg("status")
                .stdin(Stdio::null())
                .output()
            {
                Ok(output) => {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.trim().is_empty() {
                        ops_log::log_error("storage.stderr", stderr.trim());
                        eprintln!("{stderr}");
                    }
                }
                Err(err) => {
                    ops_log::log_error("storage.run", &err.to_string());
                    println!("storage helper could not run: {err}");
                }
            }
        }
        _ => {
            ops_log::log_event("storage.helper", "binary not built");
            println!("storage helper binary is not built yet.");
            println!("build matching profile first:");
            println!("  dev     : cargo build --bins");
            println!("  release : cargo build --release --bins");
        }
    }

    println!();
    println!("manual commands:");
    println!("  cargo run --bin phase1-storage -- storage status");
    println!("  cargo run --bin phase1-storage -- storage doctor");
    println!("  cargo run --bin phase1-storage -- git list");
    println!("  cargo run --bin phase1-storage -- lang roadmap");
    println!();
    println!("Press Enter to return to boot.");
    let _ = io::stdout().flush();
    let mut ignored = String::new();
    let _ = io::stdin().read_line(&mut ignored);
}

fn run_shell(boot_config: ui::BootConfig) -> ShellExit {
    boot_config.apply();
    clear_nest_exit_all_if_root();

    let mut shell = Phase1Shell::new();
    let history_store = history::HistoryStore::from_env(boot_config.persistent_state);
    let display_version = ui::display_version(kernel::VERSION, boot_config);
    let channel = if boot_config.bleeding_edge {
        "bleeding-edge"
    } else {
        "release"
    };
    ops_log::log_event(
        "shell.start",
        &format!(
            "version={display_version} channel={channel} profile={}",
            boot_config.profile_name()
        ),
    );

    shell.env.insert(
        "PHASE1_BOOT_PROFILE".to_string(),
        boot_config.profile_name().to_string(),
    );
    shell
        .env
        .insert("PHASE1_CHANNEL".to_string(), channel.to_string());
    shell.env.insert(
        "PHASE1_DISPLAY_VERSION".to_string(),
        display_version.clone(),
    );
    shell.env.insert(
        "PHASE1_SAFE_MODE".to_string(),
        if boot_config.safe_mode { "1" } else { "0" }.to_string(),
    );
    shell.env.insert(
        "PHASE1_MOBILE_MODE".to_string(),
        if boot_config.mobile_mode { "1" } else { "0" }.to_string(),
    );
    shell.env.insert(
        "PHASE1_DEVICE_MODE".to_string(),
        std::env::var("PHASE1_DEVICE_MODE").unwrap_or_else(|_| {
            if boot_config.mobile_mode {
                "mobile"
            } else {
                "desktop"
            }
            .to_string()
        }),
    );
    shell.env.insert(
        "PHASE1_PERSISTENT_STATE".to_string(),
        if boot_config.persistent_state {
            "1"
        } else {
            "0"
        }
        .to_string(),
    );
    shell.env.insert(
        "PHASE1_BLEEDING_EDGE".to_string(),
        if boot_config.bleeding_edge { "1" } else { "0" }.to_string(),
    );
    shell
        .env
        .insert("PHASE1_HISTORY".to_string(), history_store.describe());

    if boot_config.persistent_state {
        match load_persistent_state(&mut shell) {
            Ok(count) if count > 0 => {
                println!("persistent state: restored {count} entries from {PERSISTENT_STATE_PATH}")
            }
            Ok(_) => println!(
                "persistent state: enabled; no saved state found at {PERSISTENT_STATE_PATH}"
            ),
            Err(err) => {
                ops_log::log_error("state.restore", &err.to_string());
                println!("persistent state: restore warning: {err}");
            }
        }
    }

    match history_store.load(&mut shell.history) {
        Ok(count) if count > 0 => println!(
            "persistent history: restored {count} entries from {}",
            history_store.describe()
        ),
        Ok(_) => {}
        Err(err) => {
            ops_log::log_error("history.restore", &err.to_string());
            println!("persistent history: restore warning: {err}");
        }
    }

    if boot_config.quick_boot {
        ui::print_quick_boot(kernel::VERSION, boot_config);
    } else {
        ui::print_boot(kernel::VERSION);
    }

    shell.cmd_cd(Some("/home"));
    println!(
        "phase1 {} ready. Type 'help' for commands.",
        display_version
    );

    let mut shell_exit = ShellExit::Shutdown;
    loop {
        if nest_exit_all_requested() {
            println!(
                "nest: exit-all signal observed at level {}/{}",
                nested_level(),
                nested_max()
            );
            clear_nest_exit_all_if_root();
            break;
        }

        shell.kernel.tick();

        let path = compact_path(&shell.kernel.vfs.cwd);
        ui::print_prompt(shell.user(), &path);
        let _ = io::stdout().flush();
        let editor_prompt = editor_prompt_text(shell.user(), &path);

        let line = match line_editor::read_shell_line(&editor_prompt) {
            Ok(Some(line)) => line,
            Ok(None) => {
                println!();
                break;
            }
            Err(err) => {
                ops_log::log_error("input", &err.to_string());
                eprintln!("input error: {err}");
                break;
            }
        };
        if line.trim().is_empty() {
            continue;
        }

        for command_line in pasted_command_lines(&line) {
            ops_log::log_command(&command_line);
            history::push_bounded(&mut shell.history, &command_line);
            match execute_chain(&mut shell, boot_config, &history_store, &command_line) {
                Ok(_) => {
                    if boot_config.persistent_state {
                        if let Err(err) = save_persistent_state(&shell) {
                            ops_log::log_error("state.save", &err.to_string());
                            eprintln!("persistent state save warning: {err}");
                        }
                    }
                    if let Err(err) = history_store.save(&shell.history) {
                        ops_log::log_error("history.save", &err.to_string());
                        eprintln!("persistent history save warning: {err}");
                    }
                }
                Err(err) => {
                    ops_log::log_error("parse", &err);
                    eprintln!("parse error: {err}");
                }
            }

            if shell
                .env
                .get("PHASE1_NEST_EXIT_REQUESTED")
                .is_some_and(|value| value == "1")
                || nest_exit_all_requested()
                || shell
                    .env
                    .get("PHASE1_REBOOT_REQUESTED")
                    .is_some_and(|value| value == "1")
            {
                break;
            }
        }

        if shell
            .env
            .get("PHASE1_NEST_EXIT_REQUESTED")
            .is_some_and(|value| value == "1")
        {
            break;
        }

        if nest_exit_all_requested() {
            println!(
                "nest: exit-all signal observed at level {}/{}",
                nested_level(),
                nested_max()
            );
            clear_nest_exit_all_if_root();
            break;
        }

        if shell
            .env
            .get("PHASE1_REBOOT_REQUESTED")
            .is_some_and(|value| value == "1")
        {
            shell_exit = ShellExit::Reboot;
            break;
        }
    }

    if boot_config.persistent_state {
        if let Err(err) = save_persistent_state(&shell) {
            ops_log::log_error("state.save", &err.to_string());
            eprintln!("persistent state save warning: {err}");
        }
    }
    if let Err(err) = history_store.save(&shell.history) {
        ops_log::log_error("history.save", &err.to_string());
        eprintln!("persistent history save warning: {err}");
    }
    ops_log::log_event(
        "shell.stop",
        match shell_exit {
            ShellExit::Shutdown => "shutdown",
            ShellExit::Reboot => "reboot",
        },
    );
    shell_exit
}

fn pasted_command_lines(input: &str) -> Vec<String> {
    let normalized = input
        .replace("\x1b[200~", "")
        .replace("\x1b[201~", "")
        .replace("\r\n", "\n")
        .replace('\r', "\n");

    normalized
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn execute_chain(
    shell: &mut Phase1Shell,
    boot_config: ui::BootConfig,
    history_store: &history::HistoryStore,
    line: &str,
) -> Result<bool, String> {
    let chain = parse_chain(line)?;
    let mut last_status = true;

    for segment in chain {
        let should_run = match segment.op {
            ChainOp::Always => true,
            ChainOp::And => last_status,
            ChainOp::Or => !last_status,
        };
        if should_run {
            last_status = execute_one(shell, boot_config, history_store, &segment.command)?;
            learn::auto_observe(&segment.command, last_status);
        }
        if shell
            .env
            .get("PHASE1_REBOOT_REQUESTED")
            .is_some_and(|value| value == "1")
        {
            break;
        }
    }

    Ok(last_status)
}

fn execute_one(
    shell: &mut Phase1Shell,
    boot_config: ui::BootConfig,
    history_store: &history::HistoryStore,
    line: &str,
) -> Result<bool, String> {
    let expanded = shell.expand_env(line);
    match parse_line(&expanded) {
        Ok(tokens) if tokens.is_empty() => Ok(true),
        Ok(tokens) => {
            let cmd = &tokens[0];
            let args = &tokens[1..];
            let canonical = registry::canonical_name(cmd).unwrap_or(cmd);
            let known = registry::lookup(cmd).is_some()
                || plugin_exists(shell, canonical)
                || matches!(
                    canonical,
                    "avim" | "emacs" | "repo" | "lang" | "fyr" | "opslog"
                );
            match canonical {
                "help" => ui::print_help(),
                "accounts" => print!("{}", accounts_report(shell)),
                "security" => print!(
                    "{}",
                    policy::security_report(boot_config.persistent_state, "memory-only")
                ),
                "sysinfo" => print!("{}", operator::sysinfo(shell, boot_config)),
                "fastfetch" => print!("{}", fastfetch::run(shell, boot_config)),
                "dash" => print!("{}", operator::dashboard(shell, boot_config, args)),
                "theme" => print!("{}", theme_command(shell, args)),
                "banner" => print!("{}", operator::banner(boot_config, args)),
                "tips" => print!("{}", operator::tips(shell)),
                "update" => print!("{}", updater::run(args)),
                "history" => print!("{}", history::run(shell, history_store, args)),
                "grep" => print!("{}", text::grep(&shell.kernel.vfs, args)),
                "wc" => print!("{}", text::wc(&shell.kernel.vfs, args)),
                "head" => print!("{}", text::head(&shell.kernel.vfs, args)),
                "tail" => print!("{}", text::tail(&shell.kernel.vfs, args)),
                "find" => print!("{}", text::find(&shell.kernel.vfs, args)),
                "matrix" => matrix::run(args),
                "avim" => avim::edit(&mut shell.kernel.vfs, args),
                "emacs" => avim::edit(&mut shell.kernel.vfs, args),
                "lang" => print!("{}", languages::run(shell, args)),
                "fyr" => print!("{}", fyr_command(shell, args)),
                "opslog" => print!("{}", ops_log::run(args)),
                "bootcfg" => handle_bootcfg(boot_config, args),
                "nest" => print!("{}", nest_command(shell, args)),
                "repo" => print!("{}", repo_command(args)),
                "exit"
                    if args
                        .first()
                        .is_some_and(|arg| matches!(arg.as_str(), "all" | "--all")) =>
                {
                    print!("{}", request_nest_exit_all(shell))
                }
                "reboot" => request_reboot(shell),
                _ => dispatch(shell, cmd, args),
            }
            Ok(known)
        }
        Err(err) => Err(err),
    }
}

fn theme_command(shell: &mut Phase1Shell, args: &[String]) -> String {
    if args
        .first()
        .is_some_and(|arg| linux_colors::is_linux_alias(arg))
    {
        return linux_colors::theme(shell, &args[1..]);
    }
    let mut out = operator::theme(shell, args);
    if matches!(args.first().map(String::as_str), Some("list" | "ls")) {
        out.push_str("- linux        Linux host color pack with truecolor/256/ANSI fallback\n");
        out.push_str("- linux apply  detect and apply the best Linux-safe color depth\n");
        out.push_str("- linux preview show RGB/xterm/ANSI swatches without changing settings\n");
    }
    if matches!(args.first().map(String::as_str), None | Some("show")) {
        out.push_str(&format!("pack   : {}\n", linux_colors::summary(shell)));
    }
    out
}

fn fyr_command(shell: &Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") => fyr_status(),
        Some("spec") => fyr_spec(),
        Some("run") => fyr_run(shell, &args[1..]),
        Some("help") | Some("-h") | Some("--help") => fyr_help(),
        Some(other) => format!("fyr: unknown action {other}\n{}", fyr_help()),
    }
}

fn fyr_status() -> String {
    "fyr native language\nname      : Fyr\nextension : .fyr\ncommand   : fyr\nstatus    : command stub active; interpreter seed supports print literals\npurpose   : Phase1-owned language path for self-construction and VFS automation\n".to_string()
}

fn fyr_spec() -> String {
    match fs::read_to_string("PHASE1_NATIVE_LANGUAGE.md") {
        Ok(spec) => spec,
        Err(err) => format!("fyr: could not read PHASE1_NATIVE_LANGUAGE.md: {err}\n"),
    }
}

fn fyr_run(shell: &Phase1Shell, args: &[String]) -> String {
    let Some(path) = args.first() else {
        return "usage: fyr run <file.fyr>\n".to_string();
    };

    if !path.ends_with(".fyr") {
        return "fyr: expected a .fyr file\n".to_string();
    }

    let source = match shell.kernel.sys_read(path) {
        Ok(source) => source,
        Err(err) => return format!("fyr: {err}\n"),
    };

    let output = fyr_print_output(&source);
    if output.is_empty() {
        "fyr: interpreter planned; no printable output found\n".to_string()
    } else {
        output
    }
}

fn fyr_print_output(source: &str) -> String {
    let mut out = String::new();
    let mut rest = source;

    while let Some(pos) = rest.find("print(") {
        rest = &rest[pos + "print(".len()..];

        if let Some(message) = parse_fyr_string_literal(rest) {
            out.push_str(&message);
            out.push('\n');
        }

        let Some(close) = rest.find(')') else {
            break;
        };
        rest = &rest[close + 1..];
    }

    out
}

fn parse_fyr_string_literal(text: &str) -> Option<String> {
    let start = text.find('"')?;
    let mut out = String::new();
    let mut escaped = false;

    for ch in text[start + 1..].chars() {
        if escaped {
            match ch {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                other => out.push(other),
            }
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => return Some(out),
            other => out.push(other),
        }
    }

    None
}

fn fyr_help() -> String {
    "phase1 fyr command\n\nusage:\n  fyr status\n  fyr spec\n  fyr run <file.fyr>\n\nexample:\n  echo 'fn main() -> i32 { print(\"Hello, hacker!\"); return 0; }' > hello_hacker.fyr\n  fyr run hello_hacker.fyr\n".to_string()
}

fn repo_command(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") => repo_status(),
        Some("base") => repo_base(),
        Some("edge") | Some("stable") => repo_edge(),
        Some("checkpoint") | Some("checkpoints") => repo_checkpoint(),
        Some("help") | Some("-h") | Some("--help") => repo_help(),
        Some(other) => format!("repo: unknown action {other}\n{}", repo_help()),
    }
}

fn repo_status() -> String {
    "phase1 repo doctrine\nbase       : base/v4.2.0 frozen stable base\nedge       : edge/stable active default development path\ncheckpoint : checkpoint/* verified milestone snapshots\nfeature    : feature/* working branches targeting edge/stable\nrule       : stable base stays boring; edge/stable carries current tested work\ncommands   : repo base | repo edge | repo checkpoint | repo help\n".to_string()
}

fn repo_base() -> String {
    "phase1 repo base\nbranch : base/v4.2.0\nrole   : frozen stable base\npolicy : no feature work; only emergency metadata or archival fixes\nreason : gives Phase1 a known-good recovery and comparison point\n".to_string()
}

fn repo_edge() -> String {
    "phase1 repo edge\nbranch : edge/stable\nrole   : active default development path\npolicy : new tested work lands here through PRs\nreason : keeps forward motion separate from the frozen 4.2.0 base\n".to_string()
}

fn repo_checkpoint() -> String {
    "phase1 repo checkpoints\npattern : checkpoint/*\nrole    : verified milestone snapshots\npolicy  : cut checkpoints after test-passing milestones\nexample : checkpoint/edge-stable-4.3.0-dev\n".to_string()
}

fn repo_help() -> String {
    "phase1 repo command\n\nusage:\n  repo status\n  repo base\n  repo edge\n  repo checkpoint\n\nmodel:\n  base/v4.2.0  frozen stable base\n  edge/stable  active tested development path\n  checkpoint/* verified milestone snapshots\n  feature/*    working branches into edge/stable\n".to_string()
}

fn nest_command(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") => nest_status(shell),
        Some("target") | Some("use") => {
            let Some(raw) = args.get(1).map(String::as_str) else {
                return "usage: nest target <self|parent|root|level>\n".to_string();
            };
            match parse_nest_target(raw) {
                Some(level) => {
                    shell
                        .env
                        .insert("PHASE1_NEST_TARGET".to_string(), level.to_string());
                    format!("nest: target set to level {level}/{}\n", nested_max())
                }
                None => format!("nest: invalid target '{raw}'\n"),
            }
        }
        Some("exit") => match args.get(1).map(String::as_str) {
            None | Some("self") => request_nest_exit_self(shell),
            Some("all") | Some("--all") => request_nest_exit_all(shell),
            Some("target") => {
                let target = shell
                    .env
                    .get("PHASE1_NEST_TARGET")
                    .and_then(|raw| raw.parse::<u32>().ok())
                    .unwrap_or_else(nested_level);
                if target == nested_level() {
                    request_nest_exit_self(shell)
                } else if target == 0 {
                    request_nest_exit_all(shell)
                } else {
                    format!(
                        "nest: target level {target} is not directly addressable from level {}; use `nest exit all` to unwind safely\n",
                        nested_level()
                    )
                }
            }
            Some(other) => format!("nest: unknown exit target '{other}'\n"),
        },
        Some("help") | Some("-h") | Some("--help") => nest_help(),
        Some(other) => format!("nest: unknown action '{other}'\n{}", nest_help()),
    }
}

fn nest_status(shell: &Phase1Shell) -> String {
    let target = shell
        .env
        .get("PHASE1_NEST_TARGET")
        .cloned()
        .unwrap_or_else(|| nested_level().to_string());
    format!(
        "phase1 nest status\nlevel     : {}/{}\ntarget    : {}\nexit-all  : {}\ncommands  : nest target <self|parent|root|level> | nest exit self | nest exit all | exit all\n",
        nested_level(),
        nested_max(),
        target,
        if nest_exit_all_requested() { "armed" } else { "clear" }
    )
}

fn parse_nest_target(raw: &str) -> Option<u32> {
    match raw {
        "self" | "." => Some(nested_level()),
        "parent" | ".." => Some(nested_level().saturating_sub(1)),
        "root" => Some(0),
        _ => raw
            .parse::<u32>()
            .ok()
            .filter(|level| *level <= nested_max()),
    }
}

fn request_nest_exit_self(shell: &mut Phase1Shell) -> String {
    shell
        .env
        .insert("PHASE1_NEST_EXIT_REQUESTED".to_string(), "1".to_string());
    format!(
        "nest: exiting current level {}/{}\n",
        nested_level(),
        nested_max()
    )
}

fn request_nest_exit_all(shell: &mut Phase1Shell) -> String {
    shell
        .env
        .insert("PHASE1_NEST_EXIT_REQUESTED".to_string(), "1".to_string());
    let payload = format!(
        "requested_by_level={}\nmax={}\n",
        nested_level(),
        nested_max()
    );
    match fs::write(NEST_EXIT_ALL_PATH, payload) {
        Ok(()) => format!(
            "nest: exit-all armed from level {}/{}; parent levels will exit as they regain control\n",
            nested_level(),
            nested_max()
        ),
        Err(err) => format!("nest: could not arm exit-all signal: {err}\n"),
    }
}

fn nest_help() -> String {
    "phase1 nest control\n\nusage:\n  nest status\n  nest target <self|parent|root|level>\n  nest exit self\n  nest exit all\n  exit all\n\nnotes:\n  target is an operator context marker for nested workflows\n  exit all writes a local Phase1 exit signal so parent shells unwind when they regain control\n"
        .to_string()
}

fn request_reboot(shell: &mut Phase1Shell) {
    ops_log::log_event("shell.reboot", "returning to boot selector");
    shell
        .env
        .insert("PHASE1_REBOOT_REQUESTED".to_string(), "1".to_string());
    println!("reboot: returning to boot configuration screen");
}

fn editor_prompt_text(user: &str, path: &str) -> String {
    if std::env::var("PHASE1_ASCII").ok().as_deref() == Some("1")
        || std::env::var_os("NO_COLOR").is_some()
        || std::env::var("PHASE1_NO_COLOR").ok().as_deref() == Some("1")
    {
        return format!("phase1://{} {} > ", user, path);
    }

    let (title, user_color, path_color) = match active_theme_name().as_str() {
        "matrix" => (GREEN, GREEN, GREEN),
        "cyber" => (CYAN, MAGENTA, CYAN),
        "amber" => (YELLOW, YELLOW, YELLOW),
        "ice" => (BLUE, CYAN, BLUE),
        "synthwave" | "synth" => (MAGENTA, CYAN, MAGENTA),
        "crimson" => (RED, RED, YELLOW),
        "bleeding-edge" | "bleeding" | "edge" => (MAGENTA, MAGENTA, CYAN),
        _ => (GREEN, CYAN, BLUE),
    };

    format!(
        "{}{}phase1{}{}://{}{}{}{} {}{}{} ❯ ",
        BOLD, title, RESET, GRAY, RESET, user_color, user, RESET, path_color, path, RESET
    )
}

fn active_theme_name() -> String {
    std::env::var("PHASE1_THEME").unwrap_or_else(|_| {
        if std::env::var("PHASE1_BLEEDING_EDGE").ok().as_deref() == Some("1") {
            "bleeding-edge".to_string()
        } else {
            "rainbow".to_string()
        }
    })
}

fn parse_chain(line: &str) -> Result<Vec<ChainSegment>, String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut op = ChainOp::Always;
    let chars: Vec<char> = line.chars().collect();
    let mut idx = 0;

    while idx < chars.len() {
        let ch = chars[idx];
        if escaped {
            current.push(ch);
            escaped = false;
            idx += 1;
            continue;
        }
        if ch == '\\' {
            current.push(ch);
            escaped = true;
            idx += 1;
            continue;
        }
        if let Some(q) = quote {
            current.push(ch);
            if ch == q {
                quote = None;
            }
            idx += 1;
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            current.push(ch);
            idx += 1;
            continue;
        }

        let next = chars.get(idx + 1).copied();
        match (ch, next) {
            ('&', Some('&')) => {
                push_segment(&mut segments, op, &mut current)?;
                op = ChainOp::And;
                idx += 2;
            }
            ('|', Some('|')) => {
                push_segment(&mut segments, op, &mut current)?;
                op = ChainOp::Or;
                idx += 2;
            }
            (';', _) => {
                push_segment(&mut segments, op, &mut current)?;
                op = ChainOp::Always;
                idx += 1;
            }
            _ => {
                current.push(ch);
                idx += 1;
            }
        }
    }

    if quote.is_some() {
        return Err("unterminated quote in command chain".to_string());
    }
    push_segment(&mut segments, op, &mut current)?;
    Ok(segments)
}

fn push_segment(
    segments: &mut Vec<ChainSegment>,
    op: ChainOp,
    current: &mut String,
) -> Result<(), String> {
    let command = current.trim();
    if command.is_empty() {
        return Err("empty command in chain".to_string());
    }
    segments.push(ChainSegment {
        op,
        command: command.to_string(),
    });
    current.clear();
    Ok(())
}

fn plugin_exists(shell: &Phase1Shell, name: &str) -> bool {
    if name.is_empty()
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
    {
        return false;
    }
    shell.plugins_dir.join(format!("{name}.py")).exists()
        || shell.plugins_dir.join(format!("{name}.wasm")).exists()
}

fn accounts_report(shell: &Phase1Shell) -> String {
    let mut out = String::from("phase1 accounts // simulated Unix account database\nsource : /etc/passwd\nnote   : x means the credential hash is not stored in this file\nsafety : no real emails, tokens, host users, or account secrets are stored here\n\nUSER       UID   GID   HOME       SHELL\n");

    match shell.kernel.vfs.cat("/etc/passwd") {
        Ok(raw) => {
            for line in raw.lines().filter(|line| !line.trim().is_empty()) {
                let fields: Vec<_> = line.split(':').collect();
                if fields.len() >= 7 {
                    out.push_str(&format!(
                        "{:<10} {:<5} {:<5} {:<10} {}\n",
                        fields[0], fields[2], fields[3], fields[5], fields[6]
                    ));
                }
            }
        }
        Err(err) => out.push_str(&format!("accounts: could not read /etc/passwd: {err}\n")),
    }

    out
}

fn handle_bootcfg(config: ui::BootConfig, args: &[String]) {
    match args.first().map(String::as_str) {
        None | Some("show") => print_boot_config(config),
        Some("save") => match config.save() {
            Ok(()) => println!("bootcfg: saved active profile to {}", ui::config_path()),
            Err(err) => println!("bootcfg: save failed: {err}"),
        },
        Some("reset") | Some("defaults") => match ui::BootConfig::remove_saved() {
            Ok(()) => println!(
                "bootcfg: removed {}; detected defaults will be used next launch",
                ui::config_path()
            ),
            Err(err) => println!("bootcfg: reset failed: {err}"),
        },
        Some("path") => println!("{}", ui::config_path()),
        Some("state") => println!("{}", PERSISTENT_STATE_PATH),
        Some("help") | Some("-h") | Some("--help") => print_bootcfg_help(),
        Some(other) => {
            println!("bootcfg: unknown option '{other}'");
            print_bootcfg_help();
        }
    }
}

fn print_boot_config(config: ui::BootConfig) {
    println!("boot profile      : {}", config.profile_name());
    println!(
        "channel           : {}",
        if config.bleeding_edge {
            "bleeding-edge"
        } else {
            "release"
        }
    );
    println!(
        "display version   : {}",
        ui::display_version(kernel::VERSION, config)
    );
    println!("config file       : {}", ui::config_path());
    println!("state file        : {}", PERSISTENT_STATE_PATH);
    println!("ops log           : {}", ops_log::LOG_PATH);
    println!(
        "color             : {}",
        if config.color { "on" } else { "off" }
    );
    println!(
        "ascii             : {}",
        if config.ascii_mode { "on" } else { "off" }
    );
    println!(
        "safe mode         : {}",
        if config.safe_mode { "on" } else { "off" }
    );
    println!(
        "quick boot        : {}",
        if config.quick_boot { "on" } else { "off" }
    );
    println!(
        "mobile mode       : {}",
        if config.mobile_mode { "on" } else { "off" }
    );
    println!(
        "device mode       : {}",
        std::env::var("PHASE1_DEVICE_MODE").unwrap_or_else(|_| if config.mobile_mode {
            "mobile"
        } else {
            "desktop"
        }
        .to_string())
    );
    println!(
        "host tools        : {}",
        if config.host_tools { "trusted" } else { "off" }
    );
    println!(
        "bleeding edge     : {}",
        if config.bleeding_edge { "on" } else { "off" }
    );
    println!(
        "persistent state  : {}",
        if config.persistent_state { "on" } else { "off" }
    );
}

fn print_bootcfg_help() {
    println!("usage: bootcfg [show|save|reset|path|state]");
    println!("  show   display the active boot profile");
    println!("  save   write the active profile to phase1.conf");
    println!("  reset  remove phase1.conf so detected defaults are used next launch");
    println!("  path   print the config file path");
    println!("  state  print the persistent state file path");
}

fn load_persistent_state(shell: &mut Phase1Shell) -> io::Result<usize> {
    let Ok(raw) = fs::read_to_string(PERSISTENT_STATE_PATH) else {
        return Ok(0);
    };

    let mut restored = 0;
    for line in raw.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.splitn(3, '\t');
        match (parts.next(), parts.next(), parts.next()) {
            (Some("D"), Some(path), _) if is_persisted_path(path) => {
                let _ = shell.kernel.vfs.mkdir(path);
                restored += 1;
            }
            (Some("F"), Some(path), Some(encoded)) if is_persisted_path(path) => {
                let bytes = decode_hex(encoded)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
                let content = String::from_utf8(bytes)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
                shell
                    .kernel
                    .vfs
                    .write_file(path, &content, false)
                    .map_err(io::Error::other)?;
                restored += 1;
            }
            _ => {}
        }
    }
    Ok(restored)
}

fn save_persistent_state(shell: &Phase1Shell) -> io::Result<usize> {
    let mut entries = Vec::new();
    if let Some(node) = shell.kernel.vfs.get_node(Path::new("/home")) {
        collect_persistent_entries(Path::new("/home"), node, &mut entries);
    }

    let mut out = String::from("# phase1 persistent state v1\n");
    for entry in &entries {
        out.push_str(entry);
        out.push('\n');
    }
    fs::write(PERSISTENT_STATE_PATH, out)?;
    Ok(entries.len())
}

fn collect_persistent_entries(path: &Path, node: &VfsNode, out: &mut Vec<String>) {
    let path_text = path.display().to_string();
    match node {
        VfsNode::Dir { children, .. } => {
            if path_text != "/home" {
                out.push(format!("D\t{path_text}"));
            }
            let mut names: Vec<_> = children.keys().collect();
            names.sort();
            for name in names {
                collect_persistent_entries(&path.join(name), &children[name], out);
            }
        }
        VfsNode::File { content, .. } => out.push(format!(
            "F\t{path_text}\t{}",
            encode_hex(content.as_bytes())
        )),
    }
}

fn is_persisted_path(path: &str) -> bool {
    path == "/home" || path.starts_with("/home/")
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn decode_hex(raw: &str) -> Result<Vec<u8>, String> {
    if !raw.len().is_multiple_of(2) {
        return Err("hex payload has odd length".to_string());
    }
    let mut bytes = Vec::with_capacity(raw.len() / 2);
    let chars: Vec<_> = raw.bytes().collect();
    for pair in chars.chunks(2) {
        let high = hex_value(pair[0]).ok_or_else(|| "invalid hex payload".to_string())?;
        let low = hex_value(pair[1]).ok_or_else(|| "invalid hex payload".to_string())?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{compact_path, decode_hex, editor_prompt_text, encode_hex, parse_chain, ChainOp};
    use std::path::Path;

    #[test]
    fn compact_path_shortens_home() {
        assert_eq!(compact_path(Path::new("/home")), "~");
        assert_eq!(compact_path(Path::new("/home/projects")), "~/projects");
        assert_eq!(compact_path(Path::new("/proc")), "/proc");
    }

    #[test]
    fn state_hex_round_trip() {
        let encoded = encode_hex("hello phase1".as_bytes());
        assert_eq!(decode_hex(&encoded).unwrap(), b"hello phase1");
    }

    #[test]
    fn pasted_command_lines_split_shell_blocks() {
        let lines = super::pasted_command_lines(
            "git status\ngh pr list\ncargo test --workspace --all-targets\nexit all\n",
        );

        assert_eq!(
            lines,
            vec![
                "git status",
                "gh pr list",
                "cargo test --workspace --all-targets",
                "exit all",
            ]
        );
    }

    #[test]
    fn pasted_command_lines_strip_bracketed_paste_markers() {
        let lines = super::pasted_command_lines(
            "\x1b[200~cat demo_copy.go\nlang run go demo_copy.go\x1b[201~",
        );

        assert_eq!(lines, vec!["cat demo_copy.go", "lang run go demo_copy.go"]);
    }

    #[test]
    fn command_chain_respects_quotes_and_operators() {
        let chain = parse_chain("echo 'a;b' ; unknown && echo no || echo yes").unwrap();
        assert_eq!(chain.len(), 4);
        assert_eq!(chain[0].command, "echo 'a;b'");
        assert_eq!(chain[1].op, ChainOp::Always);
        assert_eq!(chain[2].op, ChainOp::And);
        assert_eq!(chain[3].op, ChainOp::Or);
    }

    #[test]
    fn editor_prompt_preserves_theme_escape_codes() {
        std::env::remove_var("PHASE1_ASCII");
        std::env::remove_var("PHASE1_NO_COLOR");
        std::env::set_var("PHASE1_THEME", "bleeding-edge");
        let prompt = editor_prompt_text("root", "~");
        assert!(prompt.contains("\x1b["));
        assert!(prompt.contains("phase1"));
        assert!(prompt.contains("root"));
        std::env::remove_var("PHASE1_THEME");
    }
}
