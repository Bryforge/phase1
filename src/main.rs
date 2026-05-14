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
                    "avim" | "emacs" | "repo" | "lang" | "fyr" | "portal" | "opslog"
                );
            match canonical {
                "help" => print!("{}", registry::help(args)),
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
                "portal" => print!("{}", portal_command(shell, args)),
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

fn portal_command(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") | Some("list") | Some("ls") => portal_status(shell),
        Some("open") => portal_open(shell, &args[1..]),
        Some("enter") => portal_enter(shell, &args[1..]),
        Some("leave") => portal_leave(shell),
        Some("close") | Some("rm") => portal_close(shell, &args[1..]),
        Some("network") | Some("net") => portal_network(shell, &args[1..]),
        Some("split") => portal_split(shell, &args[1..]),
        Some("snapshot") | Some("snap") => portal_snapshot(shell, &args[1..]),
        Some("restore") => portal_restore(shell, &args[1..]),
        Some("clone") => portal_clone(shell, &args[1..]),
        Some("link") | Some("local-link") => portal_link(shell, &args[1..]),
        Some("inspect") | Some("info") => portal_inspect(shell, &args[1..]),
        Some("help") | Some("-h") | Some("--help") => portal_help(),
        Some(other) => format!(
            "portal {other}\nstatus            : not-yet-implemented\nresult            : no-op\nhelp              : portal status\nclaim-boundary    : workspace-context-only\n"
        ),
    }
}

fn portal_names(shell: &Phase1Shell) -> Vec<String> {
    let mut names = vec!["root".to_string()];

    if let Some(raw) = shell.env.get("PHASE1_PORTALS") {
        for name in raw
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            if portal_name_is_valid(name) && !names.iter().any(|existing| existing == name) {
                names.push(name.to_string());
            }
        }
    }

    names
}

fn portal_store_names(shell: &mut Phase1Shell, names: &[String]) {
    shell
        .env
        .insert("PHASE1_PORTALS".to_string(), names.join(","));
}

fn portal_active(shell: &Phase1Shell, names: &[String]) -> String {
    shell
        .env
        .get("PHASE1_ACTIVE_PORTAL")
        .filter(|active| names.iter().any(|name| name == *active))
        .cloned()
        .unwrap_or_else(|| "root".to_string())
}

fn portal_name_is_valid(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 32
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn portal_status(shell: &Phase1Shell) -> String {
    let names = portal_names(shell);
    let active = portal_active(shell, &names);
    let network_mode = portal_network_mode(shell, &active);
    let split_mode = portal_split_mode(shell, &names);

    format!(
        "phase1 portals\n\
         mode              : read-only status\n\
         floor             : floor1\n\
         active-portal     : {active}\n\
         open-portals      : {}\n\
         portal-count      : {}\n\
         portal-layer      : workspace/session\n\
         split-mode        : {split_mode}\n\
         local-link        : planned-disabled\n\
         network-owner     : floor1\n\
         network-mode      : {network_mode}\n\
         network-default   : denied\n\
         brokered-egress   : planned-disabled\n\
         vfs-scope         : portal-context\n\
         history-scope     : portal-context\n\
         log-scope         : labelled\n\
         host-isolation    : not-claimed\n\
         process-isolation : not-claimed\n\
         network-isolation : not-claimed\n\
         network           : blocked\n\
         claim-boundary    : workspace-context-only\n",
        names.join(","),
        names.len()
    )
}

fn portal_open(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage             : portal open <name>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    if !portal_name_is_valid(name) || name == "root" {
        return format!(
            "portal open {name}\nstatus            : invalid-name\nresult            : no-op\nhelp              : portal open <name>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let mut names = portal_names(shell);
    let status = if names.iter().any(|existing| existing == name) {
        "already-open"
    } else {
        names.push(name.to_string());
        portal_store_names(shell, &names);
        "opened"
    };

    let active = portal_active(shell, &names);

    format!(
        "portal open {name}\n\
         status            : {status}\n\
         active-portal     : {active}\n\
         open-portals      : {}\n\
         network-mode      : denied\n\
         network-owner     : floor1\n\
         claim-boundary    : workspace-context-only\n",
        names.join(",")
    )
}

fn portal_enter(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage             : portal enter <name>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    let names = portal_names(shell);
    if !names.iter().any(|existing| existing == name) {
        return format!(
            "portal enter {name}\nstatus            : missing-portal\nresult            : no-op\nhelp              : portal list\nclaim-boundary    : workspace-context-only\n"
        );
    }

    shell
        .env
        .insert("PHASE1_ACTIVE_PORTAL".to_string(), name.to_string());

    let network_mode = portal_network_mode(shell, name);

    format!(
        "portal enter {name}\n\
         status            : entered\n\
         active-portal     : {name}\n\
         network-mode      : {network_mode}\n\
         network-owner     : floor1\n\
         claim-boundary    : workspace-context-only\n"
    )
}

fn portal_leave(shell: &mut Phase1Shell) -> String {
    shell
        .env
        .insert("PHASE1_ACTIVE_PORTAL".to_string(), "root".to_string());

    concat!(
        "portal leave\n",
        "status            : left\n",
        "active-portal     : root\n",
        "network-mode      : denied\n",
        "network-owner     : floor1\n",
        "claim-boundary    : workspace-context-only\n",
    )
    .to_string()
}

fn portal_close(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage             : portal close <name>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    if name == "root" || !portal_name_is_valid(name) {
        return format!(
            "portal close {name}\nstatus            : invalid-name\nresult            : no-op\nhelp              : portal close <name>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let mut names = portal_names(shell);
    if !names.iter().any(|existing| existing == name) {
        return format!(
            "portal close {name}\nstatus            : missing-portal\nresult            : no-op\nhelp              : portal list\nclaim-boundary    : workspace-context-only\n"
        );
    }

    names.retain(|existing| existing != name);
    portal_store_names(shell, &names);
    portal_store_network_mode(shell, name, "denied");

    let active = portal_active(shell, &names);
    let active = if active == name {
        shell
            .env
            .insert("PHASE1_ACTIVE_PORTAL".to_string(), "root".to_string());
        "root".to_string()
    } else {
        active
    };

    format!(
        "portal close {name}\n\
         status            : closed\n\
         active-portal     : {active}\n\
         open-portals      : {}\n\
         network-mode      : denied\n\
         network-owner     : floor1\n\
         claim-boundary    : workspace-context-only\n",
        names.join(",")
    )
}

fn portal_inspect(shell: &Phase1Shell, args: &[String]) -> String {
    let names = portal_names(shell);
    let active = portal_active(shell, &names);
    let name = args.first().map(String::as_str).unwrap_or(active.as_str());

    if !names.iter().any(|existing| existing == name) {
        return format!(
            "portal inspect {name}\nstatus            : missing-portal\nresult            : no-op\nhelp              : portal list\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let state = if name == active { "active" } else { "open" };
    let network_mode = portal_network_mode(shell, name);

    format!(
        "portal inspect {name}\n\
         status            : ok\n\
         state             : {state}\n\
         floor             : floor1\n\
         portal-layer      : workspace/session\n\
         network-owner     : floor1\n\
         network-mode      : {network_mode}\n\
         brokered-egress   : planned-disabled\n\
         host-isolation    : not-claimed\n\
         process-isolation : not-claimed\n\
         network-isolation : not-claimed\n\
         claim-boundary    : workspace-context-only\n"
    )
}

fn portal_snapshot(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage             : portal snapshot <name>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    let names = portal_names(shell);
    if !names.iter().any(|existing| existing == name) {
        return format!(
            "portal snapshot {name}\nstatus            : missing-portal\nresult            : no-op\nhelp              : portal list\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let active = portal_active(shell, &names);
    let network_mode = portal_network_mode(shell, name);
    let snapshot = format!("floor1-{name}-workspace-session");

    shell
        .env
        .insert(format!("PHASE1_PORTAL_SNAPSHOT_{name}"), snapshot.clone());

    format!(
        "portal snapshot {name}\n\
         status            : captured\n\
         portal            : {name}\n\
         snapshot          : {snapshot}\n\
         snapshot-scope    : workspace/session\n\
         active-portal     : {active}\n\
         network-owner     : floor1\n\
         network-mode      : {network_mode}\n\
         network           : blocked\n\
         result            : local-metadata-only\n\
         claim-boundary    : workspace-context-only\n"
    )
}

fn portal_restore(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage             : portal restore <name>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    if name == "root" || !portal_name_is_valid(name) {
        return format!(
            "portal restore {name}\nstatus            : invalid-name\nresult            : no-op\nhelp              : portal restore <name>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let snapshot_key = format!("PHASE1_PORTAL_SNAPSHOT_{name}");
    let Some(snapshot) = shell.env.get(&snapshot_key).cloned() else {
        return format!(
            "portal restore {name}\nstatus            : missing-snapshot\nresult            : no-op\nhelp              : portal snapshot <name>\nclaim-boundary    : workspace-context-only\n"
        );
    };

    let mut names = portal_names(shell);
    if !names.iter().any(|existing| existing == name) {
        names.push(name.to_string());
        portal_store_names(shell, &names);
    }

    shell
        .env
        .insert("PHASE1_ACTIVE_PORTAL".to_string(), name.to_string());

    let network_mode = portal_network_mode(shell, name);

    format!(
        "portal restore {name}\n\
         status            : restored\n\
         portal            : {name}\n\
         snapshot          : {snapshot}\n\
         snapshot-scope    : workspace/session\n\
         active-portal     : {name}\n\
         open-portals      : {}\n\
         network-owner     : floor1\n\
         network-mode      : {network_mode}\n\
         network           : blocked\n\
         result            : local-metadata-only\n\
         claim-boundary    : workspace-context-only\n",
        names.join(",")
    )
}

fn portal_clone(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(source) = args.first().map(String::as_str) else {
        return "usage             : portal clone <source> <name>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    let Some(target) = args.get(1).map(String::as_str) else {
        return "usage             : portal clone <source> <name>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    if source == target || target == "root" || !portal_name_is_valid(target) {
        return format!(
            "portal clone {source} {target}\nstatus            : invalid-clone\nresult            : no-op\nhelp              : portal clone <source> <name>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let mut names = portal_names(shell);
    if !names.iter().any(|existing| existing == source) {
        return format!(
            "portal clone {source} {target}\nstatus            : missing-source\nresult            : no-op\nhelp              : portal list\nclaim-boundary    : workspace-context-only\n"
        );
    }

    if names.iter().any(|existing| existing == target) {
        return format!(
            "portal clone {source} {target}\nstatus            : target-exists\nresult            : no-op\nhelp              : portal close <name>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let network_mode = portal_network_mode(shell, source);
    names.push(target.to_string());
    portal_store_names(shell, &names);
    portal_store_network_mode(shell, target, &network_mode);

    shell
        .env
        .insert("PHASE1_ACTIVE_PORTAL".to_string(), target.to_string());

    format!(
        "portal clone {source} {target}\n\
         status            : cloned\n\
         source            : {source}\n\
         portal            : {target}\n\
         active-portal     : {target}\n\
         open-portals      : {}\n\
         clone-scope       : workspace/session\n\
         network-owner     : floor1\n\
         network-mode      : {network_mode}\n\
         network           : blocked\n\
         result            : local-metadata-only\n\
         claim-boundary    : workspace-context-only\n",
        names.join(",")
    )
}

fn portal_link(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(left) = args.first().map(String::as_str) else {
        return "usage             : portal link <left> <right>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    let Some(right) = args.get(1).map(String::as_str) else {
        return "usage             : portal link <left> <right>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    if left == right || !portal_name_is_valid(left) || !portal_name_is_valid(right) {
        return format!(
            "portal link {left} {right}\nstatus            : invalid-link\nresult            : no-op\nhelp              : portal link <left> <right>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let names = portal_names(shell);
    if !names.iter().any(|existing| existing == left)
        || !names.iter().any(|existing| existing == right)
    {
        return format!(
            "portal link {left} {right}\nstatus            : missing-portal\nresult            : no-op\nhelp              : portal list\nclaim-boundary    : workspace-context-only\n"
        );
    }

    format!(
        "portal link {left} {right}\n\
         status            : planned-disabled\n\
         left              : {left}\n\
         right             : {right}\n\
         link-scope        : workspace/session\n\
         local-link        : planned-disabled\n\
         network-owner     : floor1\n\
         network-default   : denied\n\
         network           : blocked\n\
         result            : no-op\n\
         claim-boundary    : workspace-context-only\n"
    )
}

fn portal_split(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(left) = args.first().map(String::as_str) else {
        return "usage             : portal split <left> <right>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    let Some(right) = args.get(1).map(String::as_str) else {
        return "usage             : portal split <left> <right>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    if left == right
        || left == "root"
        || right == "root"
        || !portal_name_is_valid(left)
        || !portal_name_is_valid(right)
    {
        return format!(
            "portal split {left} {right}\nstatus            : invalid-split\nresult            : no-op\nhelp              : portal split <left> <right>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let mut names = portal_names(shell);
    let mut added = false;

    for portal in [left, right] {
        if !names.iter().any(|existing| existing == portal) {
            names.push(portal.to_string());
            added = true;
        }
    }

    portal_store_names(shell, &names);
    shell
        .env
        .insert("PHASE1_ACTIVE_PORTAL".to_string(), left.to_string());
    shell
        .env
        .insert("PHASE1_PORTAL_SPLIT".to_string(), format!("{left}|{right}"));

    let status = if added {
        "split-opened"
    } else {
        "split-selected"
    };

    format!(
        "portal split {left} {right}\n\
         status            : {status}\n\
         left              : {left}\n\
         right             : {right}\n\
         active-portal     : {left}\n\
         open-portals      : {}\n\
         split-mode        : two-pane-local\n\
         local-link        : planned-disabled\n\
         network-owner     : floor1\n\
         network-mode      : denied\n\
         network-default   : denied\n\
         network           : blocked\n\
         claim-boundary    : workspace-context-only\n",
        names.join(",")
    )
}

fn portal_split_mode(shell: &Phase1Shell, names: &[String]) -> String {
    shell
        .env
        .get("PHASE1_PORTAL_SPLIT")
        .and_then(|raw| raw.split_once('|'))
        .filter(|(left, right)| {
            names.iter().any(|name| name.as_str() == *left)
                && names.iter().any(|name| name.as_str() == *right)
        })
        .map(|_| "two-pane-local".to_string())
        .unwrap_or_else(|| "local-view".to_string())
}

fn portal_network(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage             : portal network <portal> <denied|local-only|brokered-egress>\nclaim-boundary    : workspace-context-only\n".to_string();
    };

    let names = portal_names(shell);
    if !names.iter().any(|existing| existing == name) {
        return format!(
            "portal network {name}\nstatus            : missing-portal\nresult            : no-op\nhelp              : portal list\nclaim-boundary    : workspace-context-only\n"
        );
    }

    let Some(mode) = args.get(1).map(String::as_str) else {
        let current = portal_network_mode(shell, name);
        return format!(
            "portal network {name}\nstatus            : current\nportal            : {name}\nnetwork-owner     : floor1\nnetwork-mode      : {current}\nnetwork-default   : denied\nnetwork           : blocked\nclaim-boundary    : workspace-context-only\n"
        );
    };

    if !matches!(mode, "denied" | "local-only" | "brokered-egress") {
        return format!(
            "portal network {name} {mode}\nstatus            : invalid-network-mode\nresult            : no-op\nhelp              : portal network <portal> <denied|local-only|brokered-egress>\nclaim-boundary    : workspace-context-only\n"
        );
    }

    if name == "root" && mode != "denied" {
        return format!(
            "portal network root {mode}\nstatus            : root-network-locked\nresult            : no-op\nnetwork-mode      : denied\nnetwork-owner     : floor1\nclaim-boundary    : workspace-context-only\n"
        );
    }

    portal_store_network_mode(shell, name, mode);

    format!(
        "portal network {name}\n\
         status            : updated\n\
         portal            : {name}\n\
         network-owner     : floor1\n\
         network-mode      : {mode}\n\
         network-default   : denied\n\
         local-link        : planned-disabled\n\
         brokered-egress   : planned-disabled\n\
         network           : blocked\n\
         result            : policy-state-only\n\
         claim-boundary    : workspace-context-only\n"
    )
}

fn portal_network_mode(shell: &Phase1Shell, name: &str) -> String {
    shell
        .env
        .get("PHASE1_PORTAL_NETWORKS")
        .and_then(|raw| {
            raw.split(',').find_map(|entry| {
                let (portal, mode) = entry.split_once('=')?;
                if portal == name && matches!(mode, "denied" | "local-only" | "brokered-egress") {
                    Some(mode.to_string())
                } else {
                    None
                }
            })
        })
        .unwrap_or_else(|| "denied".to_string())
}

fn portal_store_network_mode(shell: &mut Phase1Shell, name: &str, mode: &str) {
    let mut entries = Vec::new();

    if let Some(raw) = shell.env.get("PHASE1_PORTAL_NETWORKS") {
        for entry in raw
            .split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
        {
            let Some((portal, existing_mode)) = entry.split_once('=') else {
                continue;
            };
            if portal != name && matches!(existing_mode, "local-only" | "brokered-egress") {
                entries.push(format!("{portal}={existing_mode}"));
            }
        }
    }

    if matches!(mode, "local-only" | "brokered-egress") {
        entries.push(format!("{name}={mode}"));
    }

    if entries.is_empty() {
        shell.env.remove("PHASE1_PORTAL_NETWORKS");
    } else {
        shell
            .env
            .insert("PHASE1_PORTAL_NETWORKS".to_string(), entries.join(","));
    }
}

fn portal_help() -> String {
    concat!(
        "portal help\n",
        "usage             : portal <status|list|open|enter|leave|close|inspect|network|split|snapshot|restore|clone|link|help>\n",
        "local-state       : open, enter, leave, close, inspect, network, split, snapshot, restore, clone, link\n",
        "floor             : floor1\n",
        "network-default   : denied\n",
        "future-actions    : none\n",
        "claim-boundary    : workspace-context-only\n",
    )
    .to_string()
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

fn fyr_command(shell: &mut Phase1Shell, args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") => fyr_status(),
        Some("spec") => fyr_spec(),
        Some("new") => fyr_new(shell, &args[1..]),
        Some("init") => fyr_init(shell, &args[1..]),
        Some("cat") => fyr_cat(shell, &args[1..]),
        Some("color") | Some("highlight") => fyr_color(shell, &args[1..]),
        Some("check") => fyr_check(shell, &args[1..]),
        Some("build") => fyr_build(shell, &args[1..]),
        Some("test") => fyr_test(shell, &args[1..]),
        Some("self") => fyr_self(),
        Some("run") => fyr_run(shell, &args[1..]),
        Some("book") => fyr_book(&args[1..]),
        Some("learn") => fyr_learn(&args[1..]),
        Some("staged") => fyr_staged(&args[1..]),
        Some("help") | Some("-h") | Some("--help") => fyr_help(),
        Some(other) => format!("fyr: unknown action {other}\n{}", fyr_help()),
    }
}

fn fyr_book(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("list") => concat!(
            "fyr book\n",
            "status        : readable inside Phase1\n",
            "chapters      : 00 What is Fyr?\n",
            "read          : fyr book read 00\n",
            "runtime       : VFS-only\n",
            "host-tools    : blocked\n",
            "network       : blocked\n",
            "live-system   : untouched\n",
            "claim-boundary: book-contract-only\n",
        )
        .to_string(),
        Some("read") => match args.get(1).map(String::as_str) {
            None | Some("00" | "0") => {
                match fs::read_to_string("docs/fyr/book/00-what-is-fyr.txt") {
                    Ok(chapter) => chapter,
                    Err(err) => format!("fyr book: could not read chapter 00: {err}\n"),
                }
            }
            Some(chapter) => format!(
                "fyr book read {chapter}\nstatus        : missing chapter\nhelp          : fyr book list\nclaim-boundary: book-contract-only\n"
            ),
        },
        Some("help") | Some("-h") | Some("--help") => {
            "fyr book help\nusage         : fyr book <list|read 00>\nruntime       : VFS-only\nclaim-boundary: book-contract-only\n".to_string()
        }
        Some(other) => format!(
            "fyr book {other}\nstatus        : unknown book action\nresult        : no-op\nhelp          : fyr book list\nclaim-boundary: book-contract-only\n"
        ),
    }
}

fn fyr_learn(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("list") => concat!(
            "fyr learn\n",
            "status        : fixture-backed operator course\n",
            "lessons       : 001 orientation and safety\n",
            "run           : fyr learn run 001\n",
            "hint          : fyr learn hint 001\n",
            "runtime       : VFS-only\n",
            "host-tools    : blocked\n",
            "network       : blocked\n",
            "live-system   : untouched\n",
            "claim-boundary: lesson-contract-only\n",
        )
        .to_string(),
        Some("run") => match args.get(1).map(String::as_str) {
            None | Some("001" | "1") => {
                match fs::read_to_string("docs/fyr/fixtures/fyrlings-lesson-001-ok.txt") {
                    Ok(lesson) => lesson,
                    Err(err) => format!("fyr learn: could not read lesson 001: {err}\n"),
                }
            }
            Some(lesson) => format!(
                "fyr learn run {lesson}\nstatus        : missing lesson\nhelp          : fyr learn list\nclaim-boundary: lesson-contract-only\n"
            ),
        },
        Some("hint") => match args.get(1).map(String::as_str) {
            None | Some("001" | "1") => concat!(
                "fyr learn hint 001\n",
                "hint          : start with fyr status, fyr help, and fyr book read 00\n",
                "safety        : host-tools blocked; network blocked; live-system untouched\n",
                "claim-boundary: lesson-contract-only\n",
            )
            .to_string(),
            Some(lesson) => format!(
                "fyr learn hint {lesson}\nstatus        : missing lesson\nhelp          : fyr learn list\nclaim-boundary: lesson-contract-only\n"
            ),
        },
        Some("verify") => concat!(
            "fyr learn verify\n",
            "status        : not-yet-implemented\n",
            "result        : no-op\n",
            "help          : fyr learn list\n",
            "claim-boundary: lesson-contract-only\n",
        )
        .to_string(),
        Some("help") | Some("-h") | Some("--help") => {
            "fyr learn help\nusage         : fyr learn <list|run 001|hint 001|verify>\nruntime       : VFS-only\nclaim-boundary: lesson-contract-only\n".to_string()
        }
        Some(other) => format!(
            "fyr learn {other}\nstatus        : unknown learn action\nresult        : no-op\nhelp          : fyr learn list\nclaim-boundary: lesson-contract-only\n"
        ),
    }
}

fn fyr_staged(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") => fyr_staged_visual(),
        Some("help") | Some("-h") | Some("--help") => fyr_staged_help(),
        Some(other) => fyr_staged_unknown(other),
    }
}

fn fyr_staged_visual() -> String {
    concat!(
        "☠ FYR black_arts // STAGED CANDIDATE MODE\n",
        "[BLACK_ARTS] FYR staged candidate mode\n",
        "candidate     : phase1-base1-candidate\n",
        "workspace     : .phase1/staged-candidates/phase1-base1-candidate\n",
        "state         : fixture-backed\n",
        "live-system   : untouched\n",
        "promotion     : blocked-until-validation-and-approval\n",
        "evidence      : docs/fyr/fixtures/staged-lifecycle-example.txt\n",
        "boundary      : candidate-only | non-live | evidence-bound | claim-boundary\n",
        "commands      : status, plan, create, apply, validate, promote, discard\n",
        "implementation: pending\n",
        "claim-boundary: fixture-only\n",
    )
    .to_string()
}

fn fyr_staged_help() -> String {
    concat!(
        "fyr staged help\n",
        "codename      : black_arts\n",
        "status        : fixture-backed design help\n",
        "usage         : fyr staged <status|plan|create|apply|validate|promote|discard>\n",
        "commands      : status, plan, create, apply, validate, promote, discard\n",
        "workspace     : .phase1/staged-candidates\n",
        "boundaries    : candidate-only, non-live, evidence-bound, claim-boundary\n",
        "promotion     : validation-and-approval-required\n",
        "implementation: pending\n",
        "claim-boundary: fixture-only\n",
    )
    .to_string()
}

fn fyr_staged_unknown(action: &str) -> String {
    format!(
        "fyr staged {action}\n\
         codename      : black_arts\n\
         status        : unknown staged action\n\
         action        : {action}\n\
         live-system   : untouched\n\
         candidate     : none\n\
         result        : no-op\n\
         help          : fyr staged help\n\
         boundaries    : non-live, no-write, evidence-bound, claim-boundary\n\
         claim-boundary: fixture-only\n"
    )
}

fn fyr_status() -> String {
    "fyr native language\nname      : Fyr\nextension : .fyr\ncommand   : fyr\nstatus    : toolchain bootstrap active; VFS-only init/check/build and print-literal run\npurpose   : Phase1-owned language path for self-construction and VFS automation\n".to_string()
}

fn fyr_spec() -> String {
    match fs::read_to_string("docs/project/PHASE1_NATIVE_LANGUAGE.md") {
        Ok(spec) => spec,
        Err(err) => format!("fyr: could not read docs/project/PHASE1_NATIVE_LANGUAGE.md: {err}\n"),
    }
}

fn fyr_new(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(path) = args.first().and_then(|raw| fyr_file_name(raw)) else {
        return "usage: fyr new <name>\n".to_string();
    };

    if shell.kernel.sys_read(&path).is_ok() {
        return format!("fyr new: {path} already exists\n");
    }

    let source = "fn main() -> i32 { print(\"Hello, hacker!\"); return 0; }\n";
    match shell.kernel.sys_write(&path, source, false) {
        Ok(()) => format!("fyr new: created {path}\n"),
        Err(err) => format!("fyr new: {err}\n"),
    }
}

fn fyr_init(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(package) = args.first().and_then(|raw| fyr_package_name(raw)) else {
        return "usage: fyr init <package>\n".to_string();
    };

    let root = package;
    let src_dir = format!("{root}/src");
    let tests_dir = format!("{root}/tests");
    let manifest = format!("{root}/fyr.toml");
    let main_file = format!("{root}/src/main.fyr");
    let smoke_file = format!("{root}/tests/smoke.fyr");

    for dir in [&root, &src_dir, &tests_dir] {
        if let Err(err) = shell.kernel.vfs.mkdir(dir) {
            let msg = err.to_string();
            if !msg.contains("already exists") {
                return format!("fyr init: {msg}\n");
            }
        }
    }

    let manifest_source = format!(
        "name = \"{root}\"\nversion = \"0.1.0\"\nbackend = \"seed/interpreted\"\nhost = \"none\"\n"
    );
    let main_source = "fn main() -> i32 { print(\"Hello from Fyr package\"); return 0; }\n";
    let smoke_source = "fn main() -> i32 { print(\"fyr smoke ok\"); return 0; }\n";

    for (path, source) in [
        (&manifest, manifest_source.as_str()),
        (&main_file, main_source),
        (&smoke_file, smoke_source),
    ] {
        if shell.kernel.sys_read(path).is_err() {
            if let Err(err) = shell.kernel.sys_write(path, source, false) {
                return format!("fyr init: {err}\n");
            }
        }
    }

    format!(
        "fyr init: created package {root}\nmanifest: {manifest}\nmain    : {main_file}\ntests   : {smoke_file}\nhost    : none\n"
    )
}

fn fyr_check(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(target) = args.first().map(String::as_str) else {
        return "usage: fyr check <file.fyr|package>\n".to_string();
    };

    match fyr_resolve_target(shell, target) {
        Ok(resolved) => format!("fyr check: ok {}\n", resolved.source_path()),
        Err(err) => format!("fyr check: {err}\n"),
    }
}

fn fyr_build(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(target) = args.first().map(String::as_str) else {
        return "usage: fyr build <file.fyr|package>\n".to_string();
    };

    let resolved = match fyr_resolve_target(shell, target) {
        Ok(resolved) => resolved,
        Err(err) => return format!("fyr build: {err}\n"),
    };

    let summary = resolved.ast_summary();
    format!(
        "fyr build\npackage : {}\nsource  : {}\nast     : functions={} prints={} returns={}\nbackend : seed/interpreted\nhost    : none\nstatus  : dry-run artifact ready\n",
        resolved.package_name(),
        resolved.source_path(),
        summary.functions,
        summary.prints,
        summary.returns,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FyrSourceAst {
    functions: Vec<FyrFunctionAst>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FyrFunctionAst {
    name: String,
    statements: Vec<FyrStatementAst>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum FyrStatementAst {
    Print(String),
    Assert(FyrAssertionAst),
    AssertEq(i32, i32),
    Return(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FyrAssertionAst {
    value: bool,
    label: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FyrAstSummary {
    functions: usize,
    prints: usize,
    returns: usize,
}

enum FyrResolvedTarget {
    Single {
        source_path: String,
        ast: FyrSourceAst,
    },
    Package {
        package: String,
        main_path: String,
        ast: FyrSourceAst,
    },
}

impl FyrResolvedTarget {
    fn source_path(&self) -> &str {
        match self {
            Self::Single { source_path, .. } => source_path,
            Self::Package { main_path, .. } => main_path,
        }
    }

    fn package_name(&self) -> String {
        match self {
            Self::Single { source_path, .. } => source_path
                .rsplit('/')
                .next()
                .unwrap_or(source_path)
                .trim_end_matches(".fyr")
                .to_string(),
            Self::Package { package, .. } => package.clone(),
        }
    }

    fn ast_summary(&self) -> FyrAstSummary {
        match self {
            Self::Single { ast, .. } | Self::Package { ast, .. } => fyr_ast_summary(ast),
        }
    }
}

fn fyr_resolve_target(shell: &mut Phase1Shell, raw: &str) -> Result<FyrResolvedTarget, String> {
    let target = raw.trim().trim_end_matches('/');

    if target.ends_with(".fyr") {
        let source = shell
            .kernel
            .sys_read(target)
            .map_err(|err| err.to_string())?;
        let ast = fyr_parse_source_ast(&source).map_err(|err| format!("{target}: {err}"))?;
        return Ok(FyrResolvedTarget::Single {
            source_path: target.to_string(),
            ast,
        });
    }

    let manifest = format!("{target}/fyr.toml");
    if shell.kernel.sys_read(&manifest).is_err() {
        return Err(format!("{target}: missing package manifest {manifest}"));
    }

    let main_path = format!("{target}/src/main.fyr");
    let main_source = shell
        .kernel
        .sys_read(&main_path)
        .map_err(|_| format!("{target}: missing package main {main_path}"))?;
    let main_ast =
        fyr_parse_source_ast(&main_source).map_err(|err| format!("{main_path}: {err}"))?;

    let mut package_ast = FyrSourceAst {
        functions: main_ast.functions,
    };

    let src_dir = format!("{target}/src");
    let listing = shell.kernel.vfs.ls(Some(&src_dir), false);
    for name in listing
        .lines()
        .map(str::trim)
        .filter(|name| name.ends_with(".fyr") && *name != "main.fyr" && !name.contains(':'))
    {
        let path = format!("{src_dir}/{name}");
        let source = shell
            .kernel
            .sys_read(&path)
            .map_err(|err| format!("{path}: {err}"))?;
        let module_ast = fyr_parse_source_ast(&source).map_err(|err| format!("{path}: {err}"))?;
        package_ast.functions.extend(module_ast.functions);
    }

    let main_count = package_ast
        .functions
        .iter()
        .filter(|function| function.name == "main")
        .count();
    if main_count > 1 {
        return Err(format!("{target}: duplicate fn main"));
    }
    if main_count == 0 {
        return Err(format!("{target}: missing fn main entry point"));
    }

    Ok(FyrResolvedTarget::Package {
        package: target.to_string(),
        main_path,
        ast: package_ast,
    })
}

fn fyr_expand_let_bindings(source: &str) -> Result<String, &'static str> {
    let Some((body_start, body_end)) = fyr_main_body_bounds(source) else {
        return Ok(source.to_string());
    };

    let body = &source[body_start..body_end];

    if !body.contains("let ") && !body.contains("if ") {
        return Ok(source.to_string());
    }

    let mut bindings: Vec<(String, i32)> = Vec::new();
    let mut rewritten_body = String::new();
    let mut rest = body;

    loop {
        rest = rest.trim_start();

        if rest.is_empty() {
            break;
        }

        if let Some(after_let) = rest.strip_prefix("let ") {
            let Some(statement_end) = after_let.find(';') else {
                return Err("expected ';' after let binding");
            };

            let statement = &after_let[..statement_end];
            let Some((name, value)) = statement.split_once('=') else {
                return Err("expected '=' in let binding");
            };

            let name = name.trim();
            if !fyr_is_identifier(name) {
                return Err("invalid let binding name");
            }

            let value = match fyr_eval_integer_expression(value.trim(), &bindings) {
                Ok(value) => value,
                Err("division by zero") => return Err("division by zero"),
                Err("expected ')' in integer expression") => {
                    return Err("expected ')' in integer expression");
                }
                Err(_) => return Err("expected integer let binding value"),
            };

            bindings.push((name.to_string(), value));
            rest = &after_let[statement_end + 1..];
            continue;
        }

        if let Some(after_if) = rest.strip_prefix("if ") {
            let Some(condition_end) = after_if.find('{') else {
                return Err("expected '{' after if condition");
            };

            let condition =
                fyr_substitute_integer_bindings(after_if[..condition_end].trim(), &bindings);
            let after_open = &after_if[condition_end + 1..];

            let Some(body_end) = after_open.find('}') else {
                return Err("expected '}' after if body");
            };

            let if_body = after_open[..body_end].trim();
            let assertion =
                fyr_parse_assertion_ast(&condition).map_err(|_| "expected boolean if condition")?;

            let mut if_body_tail = if_body;
            loop {
                let trimmed = if_body_tail.trim_start();
                if trimmed.starts_with("return") {
                    if_body_tail = trimmed;
                    break;
                }

                let Some(statement_end) = trimmed.find(';') else {
                    if_body_tail = trimmed;
                    break;
                };

                let statement = trimmed[..statement_end].trim();
                if statement.starts_with("assert_eq(") || statement.starts_with("assert(") {
                    if_body_tail = &trimmed[statement_end + 1..];
                    continue;
                }

                if_body_tail = trimmed;
                break;
            }

            let Some(return_expr) = if_body_tail
                .strip_prefix("return")
                .and_then(|raw| raw.trim().strip_suffix(';'))
            else {
                return Err("expected return statement in if body");
            };

            let return_value = match fyr_eval_integer_expression(return_expr.trim(), &bindings) {
                Ok(value) => value,
                Err("division by zero") => return Err("division by zero"),
                Err("expected ')' in integer expression") => {
                    return Err("expected ')' in integer expression");
                }
                Err(_) => return Err("expected integer return value"),
            };

            if assertion.value {
                rewritten_body.push_str(&format!(" return {return_value};"));
            }

            rest = &after_open[body_end + 1..];
            continue;
        }

        let Some(statement_end) = rest.find(';') else {
            return Err("expected ';' after statement");
        };

        let statement = rest[..statement_end].trim();
        let statement = fyr_substitute_integer_bindings(statement, &bindings);

        rewritten_body.push(' ');
        rewritten_body.push_str(&statement);
        rewritten_body.push(';');

        rest = &rest[statement_end + 1..];
    }

    Ok(format!(
        "{}{}{}",
        &source[..body_start],
        rewritten_body,
        &source[body_end..]
    ))
}

fn fyr_substitute_integer_bindings(raw: &str, bindings: &[(String, i32)]) -> String {
    let mut out = String::new();
    let mut chars = raw.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some(ch) = chars.next() {
        if in_string {
            out.push(ch);

            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }

            continue;
        }

        if ch == '"' {
            in_string = true;
            out.push(ch);
            continue;
        }

        if ch == '_' || ch.is_ascii_alphabetic() {
            let mut ident = String::from(ch);

            while let Some(next) = chars.peek().copied() {
                if next == '_' || next.is_ascii_alphanumeric() {
                    ident.push(next);
                    chars.next();
                } else {
                    break;
                }
            }

            if let Some((_, value)) = bindings.iter().rev().find(|(name, _)| name == &ident) {
                out.push_str(&value.to_string());
            } else {
                out.push_str(&ident);
            }

            continue;
        }

        out.push(ch);
    }

    out
}

fn fyr_main_body_bounds(source: &str) -> Option<(usize, usize)> {
    let fn_start = source.find("fn main")?;
    let open = fn_start + source[fn_start..].find('{')?;
    let close = source.rfind('}')?;

    if close <= open {
        return None;
    }

    Some((open + 1, close))
}

#[allow(dead_code)]
fn fyr_split_seed_statements(body: &str) -> Result<Vec<String>, &'static str> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for ch in body.chars() {
        if in_string {
            current.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                current.push(ch);
            }
            ';' => {
                statements.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.trim().is_empty() {
        return Err("expected ';' after statement");
    }

    Ok(statements)
}

fn fyr_eval_integer_expression(raw: &str, bindings: &[(String, i32)]) -> Result<i32, &'static str> {
    let expr = raw.trim();
    if expr.is_empty() {
        return Err("expected integer expression");
    }

    if let Some((idx, op)) = fyr_find_top_level_operator(expr, &['+', '-'])? {
        let left = expr[..idx].trim();
        let right = expr[idx + op.len_utf8()..].trim();

        if left.is_empty() || right.is_empty() {
            return Err("expected integer expression");
        }

        let left = fyr_eval_integer_expression(left, bindings)?;
        let right = fyr_eval_integer_term(right, bindings)?;

        return match op {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            _ => unreachable!(),
        };
    }

    fyr_eval_integer_term(expr, bindings)
}

fn fyr_eval_integer_term(raw: &str, bindings: &[(String, i32)]) -> Result<i32, &'static str> {
    let term = raw.trim();
    if term.is_empty() {
        return Err("expected integer expression");
    }

    if let Some((idx, op)) = fyr_find_top_level_operator(term, &['*', '/'])? {
        let left = term[..idx].trim();
        let right = term[idx + op.len_utf8()..].trim();

        if left.is_empty() || right.is_empty() {
            return Err("expected integer expression");
        }

        let left = fyr_eval_integer_term(left, bindings)?;
        let right = fyr_eval_integer_factor(right, bindings)?;

        return match op {
            '*' => Ok(left * right),
            '/' => {
                if right == 0 {
                    Err("division by zero")
                } else {
                    Ok(left / right)
                }
            }
            _ => unreachable!(),
        };
    }

    fyr_eval_integer_factor(term, bindings)
}

fn fyr_find_top_level_operator(
    expr: &str,
    operators: &[char],
) -> Result<Option<(usize, char)>, &'static str> {
    let mut depth = 0i32;
    let mut found = None;

    for (idx, ch) in expr.char_indices() {
        match ch {
            '(' => {
                depth += 1;
                continue;
            }
            ')' => {
                if depth == 0 {
                    return Err("expected integer expression");
                }
                depth -= 1;
                continue;
            }
            _ => {}
        }

        if depth == 0 && idx > 0 && operators.contains(&ch) {
            let left = expr[..idx].trim();
            let right = expr[idx + ch.len_utf8()..].trim();

            if left.is_empty() || right.is_empty() {
                return Err("expected integer expression");
            }

            found = Some((idx, ch));
        }
    }

    if depth != 0 {
        return Err("expected ')' in integer expression");
    }

    Ok(found)
}

fn fyr_eval_integer_factor(raw: &str, bindings: &[(String, i32)]) -> Result<i32, &'static str> {
    let factor = raw.trim();

    if let Some(inner) = factor.strip_prefix('-') {
        let inner = inner.trim();
        if inner.is_empty() {
            return Err("expected integer expression");
        }
        return fyr_eval_integer_factor(inner, bindings).map(|value| -value);
    }

    if factor.starts_with('(') {
        let inner = fyr_parenthesized_inner(factor)?;
        return fyr_eval_integer_expression(inner, bindings);
    }

    if let Ok(value) = factor.parse::<i32>() {
        return Ok(value);
    }

    if !fyr_is_identifier(factor) {
        return Err("expected integer expression");
    }

    bindings
        .iter()
        .rev()
        .find_map(|(name, value)| (name == factor).then_some(*value))
        .ok_or("unknown let binding")
}

fn fyr_parenthesized_inner(raw: &str) -> Result<&str, &'static str> {
    let mut depth = 0i32;

    for (idx, ch) in raw.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    if idx + ch.len_utf8() == raw.len() {
                        return Ok(&raw[1..idx]);
                    }
                    return Err("expected integer expression");
                }
            }
            _ => {}
        }
    }

    Err("expected ')' in integer expression")
}

fn fyr_is_identifier(raw: &str) -> bool {
    let mut chars = raw.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }

    if !chars.all(fyr_identifier_char) {
        return false;
    }

    !matches!(
        raw,
        "fn" | "main" | "print" | "assert" | "assert_eq" | "return" | "let" | "i32"
    )
}

fn fyr_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[allow(dead_code)]
fn fyr_replace_identifier_outside_strings(input: &str, name: &str, value: &str) -> String {
    let mut out = String::new();
    let mut token = String::new();
    let mut in_string = false;
    let mut escaped = false;

    let flush_token = |token: &mut String, out: &mut String| {
        if token.is_empty() {
            return;
        }

        if token == name {
            out.push_str(value);
        } else {
            out.push_str(token);
        }

        token.clear();
    };

    for ch in input.chars() {
        if in_string {
            out.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            flush_token(&mut token, &mut out);
            in_string = true;
            out.push(ch);
        } else if fyr_identifier_char(ch) {
            token.push(ch);
        } else {
            flush_token(&mut token, &mut out);
            out.push(ch);
        }
    }

    flush_token(&mut token, &mut out);
    out
}

fn fyr_parse_source_ast(source: &str) -> Result<FyrSourceAst, &'static str> {
    let expanded_source = fyr_expand_let_bindings(source)?;
    let function = fyr_parse_main_function(&expanded_source)?;
    Ok(FyrSourceAst {
        functions: vec![function],
    })
}

fn fyr_ast_summary(ast: &FyrSourceAst) -> FyrAstSummary {
    let mut summary = FyrAstSummary {
        functions: ast.functions.len(),
        prints: 0,
        returns: 0,
    };

    for function in &ast.functions {
        for statement in &function.statements {
            match statement {
                FyrStatementAst::Print(_)
                | FyrStatementAst::Assert(_)
                | FyrStatementAst::AssertEq(_, _) => summary.prints += 1,
                FyrStatementAst::Return(_) => summary.returns += 1,
            }
        }
    }

    summary
}

fn fyr_parse_main_function(source: &str) -> Result<FyrFunctionAst, &'static str> {
    let body = fyr_main_body(source)?;
    let statements = fyr_parse_statements(body)?;
    Ok(FyrFunctionAst {
        name: "main".to_string(),
        statements,
    })
}

fn fyr_parse_statements(body: &str) -> Result<Vec<FyrStatementAst>, &'static str> {
    let mut statements = Vec::new();
    let mut rest = body.trim();

    while !rest.is_empty() {
        rest = rest.trim_start();

        if rest.starts_with("print") {
            let (statement, next) = fyr_parse_print_statement_ast(rest)?;
            statements.push(statement);
            rest = next.trim_start();
        } else if rest.starts_with("assert_eq") {
            let (statement, next) = fyr_parse_assert_eq_statement_ast(rest)?;
            statements.push(statement);
            rest = next.trim_start();
        } else if rest.starts_with("assert") {
            let (statement, next) = fyr_parse_assert_statement_ast(rest)?;
            statements.push(statement);
            rest = next.trim_start();
        } else if rest.starts_with("return") {
            let (statement, next) = fyr_parse_return_statement_ast(rest)?;
            statements.push(statement);
            rest = next.trim_start();
        } else {
            return Err("expected print, assert_eq, assert, or return statement");
        }
    }

    let has_observable_statement = statements.iter().any(|statement| {
        matches!(
            statement,
            FyrStatementAst::Print(_)
                | FyrStatementAst::Assert(_)
                | FyrStatementAst::AssertEq(_, _)
                | FyrStatementAst::Return(_)
        )
    });

    if !has_observable_statement {
        return Err("seed checker requires at least one executable statement");
    }

    if !statements
        .iter()
        .any(|statement| matches!(statement, FyrStatementAst::Return(_)))
    {
        return Err("missing return statement");
    }

    Ok(statements)
}

fn fyr_main_body(source: &str) -> Result<&str, &'static str> {
    let Some(fn_pos) = source.find("fn") else {
        return Err("missing fn main entry point");
    };

    let source = source[fn_pos..].trim_start();
    let Some(rest) = source.strip_prefix("fn main() -> i32") else {
        return Err("missing fn main entry point");
    };

    let rest = rest.trim_start();
    let Some(body) = rest.strip_prefix('{') else {
        return Err("expected '{' after fn main signature");
    };

    let Some(close) = body.rfind('}') else {
        return Err("expected '}' after fn main body");
    };

    Ok(&body[..close])
}

fn fyr_parse_print_statement_ast(statement: &str) -> Result<(FyrStatementAst, &str), &'static str> {
    let Some(rest) = statement.strip_prefix("print") else {
        return Err("expected print statement");
    };

    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix('(') else {
        return Err("expected '(' after print");
    };

    let (message, rest) = fyr_parse_string_literal_with_rest(rest)?;
    let rest = rest.trim_start();

    let Some(rest) = rest.strip_prefix(')') else {
        return Err("expected ')' after print literal");
    };

    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix(';') else {
        return Err("expected ';' after print statement");
    };

    Ok((FyrStatementAst::Print(message), rest))
}

fn fyr_parse_assert_statement_ast(
    statement: &str,
) -> Result<(FyrStatementAst, &str), &'static str> {
    let Some(rest) = statement.strip_prefix("assert(") else {
        return Err("expected assert statement");
    };

    let Some((assertion, rest)) = rest.split_once(");") else {
        return Err("expected ');' after assert statement");
    };

    Ok((
        FyrStatementAst::Assert(fyr_parse_assertion_ast(assertion)?),
        rest,
    ))
}

fn fyr_parse_assertion_ast(assertion: &str) -> Result<FyrAssertionAst, &'static str> {
    let assertion = fyr_strip_wrapping_assertion_parens(assertion.trim());

    if assertion.is_empty() {
        return Err("expected boolean assertion");
    }

    if let Some(parts) = fyr_split_assertion_chain(assertion, "||") {
        let mut labels = Vec::new();
        let mut value = false;

        for part in parts {
            let parsed = fyr_parse_assertion_ast(part)?;
            value |= parsed.value;
            labels.push(parsed.label);
        }

        return Ok(FyrAssertionAst {
            value,
            label: labels.join(" || "),
        });
    }

    if let Some(parts) = fyr_split_assertion_chain(assertion, "&&") {
        let mut labels = Vec::new();
        let mut value = true;

        for part in parts {
            let parsed = fyr_parse_assertion_ast(part)?;
            value &= parsed.value;
            labels.push(parsed.label);
        }

        return Ok(FyrAssertionAst {
            value,
            label: labels.join(" && "),
        });
    }

    if let Some(inner) = assertion.strip_prefix('!') {
        let parsed = fyr_parse_assertion_ast(inner)?;
        return Ok(FyrAssertionAst {
            value: !parsed.value,
            label: format!("!({})", parsed.label),
        });
    }

    fyr_parse_atomic_assertion_ast(assertion)
}

fn fyr_strip_wrapping_assertion_parens(assertion: &str) -> &str {
    let mut current = assertion.trim();

    loop {
        if !(current.starts_with('(') && current.ends_with(')')) {
            return current;
        }

        let mut depth = 0i32;
        let mut wraps_entire_expression = true;

        for (idx, ch) in current.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 && idx != current.len() - 1 {
                        wraps_entire_expression = false;
                        break;
                    }
                }
                _ => {}
            }

            if depth < 0 {
                wraps_entire_expression = false;
                break;
            }
        }

        if !wraps_entire_expression || depth != 0 {
            return current;
        }

        current = current[1..current.len() - 1].trim();
    }
}

fn fyr_split_assertion_chain<'a>(assertion: &'a str, op: &str) -> Option<Vec<&'a str>> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    let mut i = 0usize;

    while i < assertion.len() {
        let rest = &assertion[i..];
        let ch = rest.chars().next().expect("char");

        match ch {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            _ => {}
        }

        if depth == 0 && rest.starts_with(op) {
            parts.push(&assertion[start..i]);
            i += op.len();
            start = i;
            continue;
        }

        i += ch.len_utf8();
    }

    if parts.is_empty() {
        None
    } else {
        parts.push(&assertion[start..]);
        Some(parts)
    }
}

fn fyr_parse_atomic_assertion_ast(assertion: &str) -> Result<FyrAssertionAst, &'static str> {
    let assertion = assertion.trim();

    if assertion == "true" {
        return Ok(FyrAssertionAst {
            value: true,
            label: assertion.to_string(),
        });
    }

    if assertion == "false" {
        return Ok(FyrAssertionAst {
            value: false,
            label: assertion.to_string(),
        });
    }

    for op in [">=", "<=", "==", "!=", ">", "<"] {
        if let Some((left, right)) = assertion.split_once(op) {
            let left = fyr_eval_integer_expression(left.trim(), &[])
                .map_err(|_| "expected integer value")?;
            let right = fyr_eval_integer_expression(right.trim(), &[])
                .map_err(|_| "expected integer value")?;

            let value = match op {
                ">=" => left >= right,
                "<=" => left <= right,
                "==" => left == right,
                "!=" => left != right,
                ">" => left > right,
                "<" => left < right,
                _ => unreachable!(),
            };

            return Ok(FyrAssertionAst {
                value,
                label: format!("{left} {op} {right}"),
            });
        }
    }

    Err("expected boolean assertion")
}

fn fyr_parse_assert_eq_statement_ast(
    statement: &str,
) -> Result<(FyrStatementAst, &str), &'static str> {
    let Some(rest) = statement.strip_prefix("assert_eq") else {
        return Err("expected assert_eq statement");
    };

    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix('(') else {
        return Err("expected '(' after assert_eq");
    };

    let (left, rest) = fyr_parse_integer_with_rest(rest)?;
    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix(',') else {
        return Err("expected ',' in assert_eq");
    };

    let (right, rest) = fyr_parse_integer_with_rest(rest)?;
    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix(')') else {
        return Err("expected ')' after assert_eq values");
    };

    let rest = rest.trim_start();
    let Some(rest) = rest.strip_prefix(';') else {
        return Err("expected ';' after assert_eq statement");
    };

    Ok((FyrStatementAst::AssertEq(left, right), rest))
}

fn fyr_parse_integer_with_rest(text: &str) -> Result<(i32, &str), &'static str> {
    let raw = text.trim_start();
    let mut end = 0usize;
    let mut seen_digit = false;

    for (idx, ch) in raw.char_indices() {
        if idx == 0 && ch == '-' {
            end = ch.len_utf8();
            continue;
        }

        if ch.is_ascii_digit() {
            seen_digit = true;
            end = idx + ch.len_utf8();
            continue;
        }

        break;
    }

    if !seen_digit {
        return Err("expected integer value");
    }

    let value = raw[..end]
        .parse::<i32>()
        .map_err(|_| "expected integer value")?;

    Ok((value, &raw[end..]))
}

fn fyr_eval_test_ast(ast: &FyrSourceAst) -> Result<(), String> {
    for function in &ast.functions {
        for statement in &function.statements {
            match statement {
                FyrStatementAst::Assert(assertion) if !assertion.value => {
                    return Err(format!("assertion failed: {}", assertion.label));
                }
                FyrStatementAst::AssertEq(left, right) if left != right => {
                    return Err(format!("assertion failed: {left} != {right}"));
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn fyr_parse_return_statement_ast(
    statement: &str,
) -> Result<(FyrStatementAst, &str), &'static str> {
    let Some(rest) = statement.strip_prefix("return") else {
        return Err("expected return statement");
    };

    let Some((raw_value, rest)) = rest.split_once(';') else {
        return Err("expected ';' after return statement");
    };

    let value = fyr_eval_integer_expression(raw_value.trim(), &[])
        .map_err(|_| "expected integer return value")?;

    Ok((FyrStatementAst::Return(value), rest))
}

fn fyr_parse_string_literal_with_rest(text: &str) -> Result<(String, &str), &'static str> {
    let text = text.trim_start();
    let Some(inner) = text.strip_prefix('"') else {
        return Err("expected string literal");
    };

    let mut out = String::new();
    let mut escaped = false;

    for (idx, ch) in inner.char_indices() {
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
            '"' => {
                let rest = &inner[idx + ch.len_utf8()..];
                return Ok((out, rest));
            }
            other => out.push(other),
        }
    }

    Err("unterminated string literal")
}

fn fyr_package_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
    {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn fyr_color(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(target) = args.first().map(String::as_str) else {
        return "usage: fyr color <file.fyr>\n".to_string();
    };

    let target = target.trim();
    match shell.kernel.sys_read(target) {
        Ok(source) => fyr_colorize_source(&source),
        Err(_) => format!("fyr color: no such file: {target}\n"),
    }
}

fn fyr_colorize_source(source: &str) -> String {
    let mut out = String::new();

    for line in source.lines() {
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '"' {
                let mut literal = String::from("\"");
                let mut escaped = false;

                for next in chars.by_ref() {
                    literal.push(next);
                    if escaped {
                        escaped = false;
                    } else if next == '\\' {
                        escaped = true;
                    } else if next == '"' {
                        break;
                    }
                }

                out.push_str("\x1b[35m");
                out.push_str(&literal);
                out.push_str("\x1b[0m");
                continue;
            }

            if ch.is_ascii_digit() {
                let mut number = ch.to_string();
                while let Some(next) = chars.peek() {
                    if next.is_ascii_digit() {
                        number.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                out.push_str("\x1b[33m");
                out.push_str(&number);
                out.push_str("\x1b[0m");
                continue;
            }

            if ch.is_ascii_alphabetic() || ch == '_' {
                let mut word = ch.to_string();
                while let Some(next) = chars.peek() {
                    if next.is_ascii_alphanumeric() || *next == '_' {
                        word.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let color = match word.as_str() {
                    "fn" | "return" => Some("\x1b[36m"),
                    "print" | "assert" | "assert_eq" => Some("\x1b[32m"),
                    "true" | "false" | "i32" => Some("\x1b[34m"),
                    _ => None,
                };

                if let Some(color) = color {
                    out.push_str(color);
                    out.push_str(&word);
                    out.push_str("\x1b[0m");
                } else {
                    out.push_str(&word);
                }

                continue;
            }

            out.push(ch);
        }

        out.push('\n');
    }

    out
}

fn fyr_cat(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(path) = args.first().and_then(|raw| fyr_file_name(raw)) else {
        return "usage: fyr cat <file.fyr>\n".to_string();
    };

    match shell.kernel.sys_read(&path) {
        Ok(mut source) => {
            if !source.ends_with('\n') {
                source.push('\n');
            }
            source
        }
        Err(err) => format!("fyr cat: {err}\n"),
    }
}

fn fyr_test(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(package) = args.first().map(String::as_str) else {
        return "usage: fyr test <package>\n".to_string();
    };

    let package = package.trim().trim_end_matches('/');
    let manifest = format!("{package}/fyr.toml");
    if shell.kernel.sys_read(&manifest).is_err() {
        return format!("fyr test: {package}: missing package manifest {manifest}\n");
    }

    let tests_dir = format!("{package}/tests");
    let listing = shell.kernel.vfs.ls(Some(&tests_dir), false);

    let mut tests = 0usize;
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut out = format!("fyr test\npackage : {package}\n");

    for name in listing
        .lines()
        .map(str::trim)
        .filter(|name| name.ends_with(".fyr"))
    {
        tests += 1;
        let path = format!("{tests_dir}/{name}");

        match shell.kernel.sys_read(&path) {
            Ok(source) => match fyr_parse_source_ast(&source) {
                Ok(ast) => match fyr_eval_test_ast(&ast) {
                    Ok(()) => {
                        passed += 1;
                        out.push_str(&format!("test    : {path} ok\n"));
                    }
                    Err(err) => {
                        failed += 1;
                        out.push_str(&format!("test    : {path} failed: {err}\n"));
                    }
                },
                Err(err) => {
                    failed += 1;
                    out.push_str(&format!("test    : {path} failed: {err}\n"));
                }
            },
            Err(err) => {
                failed += 1;
                out.push_str(&format!("test    : {path} failed: {err}\n"));
            }
        }
    }

    let status = if failed == 0 { "ok" } else { "failed" };

    out.push_str(&format!(
        "tests   : {tests}\npassed  : {passed}\nfailed  : {failed}\nstatus  : {status}\n"
    ));
    out
}

fn fyr_self() -> String {
    "fyr self\nstatus : online\nvfs    : available\nrunner : print literal seed\nnext   : lexer, parser, VFS-safe standard library\n".to_string()
}

fn fyr_file_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let base = trimmed.strip_suffix(".fyr").unwrap_or(trimmed);
    if base
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
    {
        Some(format!("{base}.fyr"))
    } else {
        None
    }
}

fn fyr_run(shell: &mut Phase1Shell, args: &[String]) -> String {
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
    "phase1 fyr command\n\nusage:\n  fyr status\n  fyr spec\n  fyr new <name>\n  fyr init <package>\n  fyr cat <file.fyr>\n  fyr check <file.fyr|package>\n  fyr build <file.fyr|package>\n  fyr test <package>\n  fyr self\n  fyr run <file.fyr>\n\nexample:\n  echo 'fn main() -> i32 { print(\"Hello, hacker!\"); return 0; }' > hello_hacker.fyr\n  fyr run hello_hacker.fyr\n".to_string()
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
        Some("spawn") => nest_spawn(shell, &args[1..]),
        Some("list") | Some("ls") => nest_list(shell),
        Some("enter") => nest_enter(shell, &args[1..]),
        Some("destroy") | Some("rm") => nest_destroy(shell, &args[1..]),
        Some("inspect") | Some("info") => nest_inspect(shell, &args[1..]),
        Some("tree") => nest_tree(shell),
        Some("stack") => nest_stack_command(&args[1..]),
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
        Some("exit") if args.len() == 1 => nest_exit(shell),
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

fn nest_stack_command(args: &[String]) -> String {
    match args.first().map(String::as_str) {
        None | Some("status") | Some("list") | Some("ls") => nest_stack_status(),
        Some("push") | Some("pop") | Some("ghost") | Some("resume") | Some("prune")
        | Some("exit-all") => nest_stack_pending(args.first().map(String::as_str).unwrap_or("")),
        Some("help") | Some("-h") | Some("--help") => nest_stack_help(),
        Some(other) => format!(
            "nest stack {other}\nstatus        : unknown stack action\nresult        : no-op\nhelp          : nest stack status\nclaim-boundary: control-plane-only\n"
        ),
    }
}

fn nest_stack_status() -> String {
    let level = nested_level();
    let max = nested_max();
    let exit_all = if nest_exit_all_requested() {
        "requested"
    } else {
        "clear"
    };

    format!(
        "phase1 nest stack\n         mode          : read-only status\n         nest-level    : {level}/{max}\n         root          : active\n         current       : level-{level}\n         ghost-count   : 0\n         exit-all      : {exit_all}\n         safe-mode     : visible\n         trust         : visible\n         guardrail     : no host process spawn | no network | no isolation claim\n         claim-boundary: control-plane-only\n"
    )
}

fn nest_stack_pending(action: &str) -> String {
    format!(
        "nest stack {action}\n         status        : not-yet-implemented\n         result        : no-op\n         help          : nest stack status\n         claim-boundary: control-plane-only\n"
    )
}

fn nest_stack_help() -> String {
    "nest stack help\n     usage         : nest stack <status|list|push|pop|ghost|resume|prune|exit-all>\n     first-slice   : status, list\n     pending       : push, pop, ghost, resume, prune, exit-all\n     claim-boundary: control-plane-only\n"
        .to_string()
}

fn nest_spawn(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage: nest spawn <name>\n".to_string();
    };

    let level = nest_current_level(shell);
    let max = nested_max();

    if level >= max {
        return format!("nest spawn: max depth reached {level}/{max}\n");
    }

    if !nest_name_is_valid(name) {
        return "nest spawn: invalid nest name\n".to_string();
    }

    let mut children = nest_children(shell);
    if children.iter().any(|child| child == name) {
        return format!("nest spawn: {name} already exists\n");
    }

    let _ = shell.kernel.vfs.mkdir("/nest");
    let _ = shell.kernel.vfs.mkdir(&format!("/nest/{name}"));

    children.push(name.to_string());
    nest_store_children(shell, &children);

    format!(
        "nest spawn: created {name}\nlevel   : {}/{}\nmode    : isolated\nroot    : /nest/{name}\nhost    : inherited-safe-defaults\n",
        level + 1,
        max
    )
}

fn nest_list(shell: &Phase1Shell) -> String {
    let children = nest_children(shell);
    let level = nest_current_level(shell);
    let max = nested_max();

    if children.is_empty() {
        return "nest list\nchildren: none\n".to_string();
    }

    let mut out = "nest list\n".to_string();
    for child in children {
        out.push_str(&format!(
            "{child}\nlevel   : {}/{}\nmode    : isolated\nroot    : /nest/{child}\n",
            level + 1,
            max
        ));
    }
    out
}

fn nest_children(shell: &Phase1Shell) -> Vec<String> {
    shell
        .env
        .get("PHASE1_NEST_CHILDREN")
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn nest_store_children(shell: &mut Phase1Shell, children: &[String]) {
    shell
        .env
        .insert("PHASE1_NEST_CHILDREN".to_string(), children.join(","));
}

fn nest_name_is_valid(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 32
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn nest_current_level(shell: &Phase1Shell) -> u32 {
    shell
        .env
        .get("PHASE1_NEST_ACTIVE_LEVEL")
        .and_then(|raw| raw.trim().parse::<u32>().ok())
        .unwrap_or_else(nested_level)
}

fn nest_active_name(shell: &Phase1Shell) -> String {
    shell
        .env
        .get("PHASE1_NEST_ACTIVE")
        .filter(|name| !name.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| "root".to_string())
}

fn nest_active_path(shell: &Phase1Shell) -> String {
    let active = nest_active_name(shell);
    if active == "root" {
        "/".to_string()
    } else {
        format!("/nest/{active}")
    }
}

fn nest_tree(shell: &Phase1Shell) -> String {
    let children = nest_children(shell);
    let active = nest_active_name(shell);
    let level = nest_current_level(shell);
    let max = nested_max();

    let mut out = format!(
        "nest tree\nroot{}\nlevel   : {level}/{max}\n",
        if active == "root" { " *" } else { "" }
    );

    if children.is_empty() {
        out.push_str("children: none\n");
        return out;
    }

    out.push_str(&format!("active  : {active}\n"));

    for child in children {
        let marker = if active == child { " *" } else { "" };
        out.push_str(&format!(
            "|- {child}{marker}\n   level   : {}/{}\n   path    : /nest/{child}\n   mode    : isolated\n",
            level + 1,
            max
        ));
    }

    out
}

fn nest_inspect(shell: &Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage: nest inspect <name>\n".to_string();
    };

    if !nest_name_is_valid(name) {
        return "nest inspect: invalid nest name\n".to_string();
    }

    let children = nest_children(shell);
    if !children.iter().any(|child| child == name) {
        return format!("nest inspect: {name} not found\n");
    }

    let active = nest_active_name(shell) == name;
    format!(
        "nest inspect\nname    : {name}\nlevel   : {}/{}\nactive  : {}\npath    : /nest/{name}\nmode    : isolated\nhost    : inherited-safe-defaults\n",
        nested_level() + 1,
        nested_max(),
        if active { "yes" } else { "no" }
    )
}

fn nest_destroy(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage: nest destroy <name>\n".to_string();
    };

    if !nest_name_is_valid(name) {
        return "nest destroy: invalid nest name\n".to_string();
    }

    let mut children = nest_children(shell);
    let Some(index) = children.iter().position(|child| child == name) else {
        return format!("nest destroy: {name} not found\n");
    };

    children.remove(index);
    nest_store_children(shell, &children);

    let was_active = nest_active_name(shell) == name;
    if was_active {
        shell.env.remove("PHASE1_NEST_ACTIVE");
        shell.env.remove("PHASE1_NEST_ACTIVE_LEVEL");
        return format!(
            "nest destroy: removed {name}\nactive  : root\nlevel   : {}/{}\npath    : /\n",
            nested_level(),
            nested_max()
        );
    }

    format!("nest destroy: removed {name}\n")
}

fn nest_enter(shell: &mut Phase1Shell, args: &[String]) -> String {
    let Some(name) = args.first().map(String::as_str) else {
        return "usage: nest enter <name>\n".to_string();
    };

    if !nest_name_is_valid(name) {
        return "nest enter: invalid nest name\n".to_string();
    }

    let children = nest_children(shell);
    if !children.iter().any(|child| child == name) {
        return format!("nest enter: {name} not found\n");
    }

    let level = nest_current_level(shell);
    let max = nested_max();

    if level >= max {
        return format!("nest enter: max depth reached {level}/{max}\n");
    }

    shell
        .env
        .insert("PHASE1_NEST_ACTIVE".to_string(), name.to_string());
    shell.env.insert(
        "PHASE1_NEST_ACTIVE_LEVEL".to_string(),
        (level + 1).to_string(),
    );

    format!(
        "nest enter: {name}\nlevel   : {}/{}\nmode    : isolated\npath    : /nest/{name}\nhost    : inherited-safe-defaults\n",
        level + 1,
        max
    )
}

fn nest_exit(shell: &mut Phase1Shell) -> String {
    let level = nest_current_level(shell);

    if level == 0 {
        return "nest exit: already at root\n".to_string();
    }

    shell.env.remove("PHASE1_NEST_ACTIVE");
    shell.env.remove("PHASE1_NEST_ACTIVE_LEVEL");

    format!(
        "nest exit: returned to parent\nlevel   : {}/{}\nactive  : root\npath    : /\n",
        nested_level(),
        nested_max()
    )
}

fn nest_status(shell: &Phase1Shell) -> String {
    let level = nest_current_level(shell);
    let max = nested_max();
    let active = nest_active_name(shell);
    let path = nest_active_path(shell);

    format!(
        "nest status\nlevel   : {level}/{max}\nroot    : {}\nactive  : {active}\npath    : {path}\nmode    : isolated\nhost    : inherited-safe-defaults\n",
        if level == 0 { "yes" } else { "no" }
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
    "phase1 nest control\n\nusage:\n  nest status\n  nest spawn <name>\n  nest list\n  nest enter <name>\n  nest exit\n  nest destroy <name>\n  nest inspect <name>\n  nest tree\n  nest stack\n  nest target <self|parent|root|level>\n  nest exit self\n  nest exit all\n  nest exit-all\n  exit all\n\nnotes:\n  nest stack shows read-only ghost stack status\n  target is an operator context marker for nested workflows\n  nest exit-all is an alias for nest exit all\n  exit all writes a local Phase1 exit signal so parent shells unwind when they regain control\n".to_string()
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
