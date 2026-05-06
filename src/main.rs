mod browser;
mod commands;
mod kernel;
mod man;
mod matrix;
mod ned;
mod network;
mod registry;
mod ui;

use commands::{dispatch, parse_line, Phase1Shell};
use std::io::{self, Write};
use std::path::Path;

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

fn main() {
    loop {
        match ui::configure_boot(kernel::VERSION) {
            ui::BootSelection::Boot(config) => {
                run_shell(config);
                break;
            }
            ui::BootSelection::Reboot => continue,
            ui::BootSelection::Quit => {
                println!("boot aborted: phase1 did not enter the main system");
                return;
            }
        }
    }
}

fn run_shell(boot_config: ui::BootConfig) {
    boot_config.apply();

    let mut shell = Phase1Shell::new();
    shell.env.insert("PHASE1_BOOT_PROFILE".to_string(), boot_config.profile_name().to_string());
    shell.env.insert("PHASE1_SAFE_MODE".to_string(), if boot_config.safe_mode { "1" } else { "0" }.to_string());
    shell.env.insert("PHASE1_MOBILE_MODE".to_string(), if boot_config.mobile_mode { "1" } else { "0" }.to_string());

    if boot_config.quick_boot {
        ui::print_quick_boot(kernel::VERSION, boot_config);
    } else {
        ui::print_boot(kernel::VERSION);
    }

    shell.cmd_cd(Some("/home"));
    println!("phase1 {} ready. Type 'help' for commands.", kernel::VERSION);

    let stdin = io::stdin();
    let mut input = String::with_capacity(256);

    loop {
        shell.kernel.tick();

        let path = compact_path(&shell.kernel.vfs.cwd);
        ui::print_prompt(shell.user(), &path);
        let _ = io::stdout().flush();

        input.clear();
        if stdin.read_line(&mut input).is_err() {
            println!();
            break;
        }

        let line = input.trim_end_matches(['\r', '\n']);
        if line.trim().is_empty() {
            continue;
        }

        shell.push_history(line);
        let expanded = shell.expand_env(line);
        match parse_line(&expanded) {
            Ok(tokens) if tokens.is_empty() => {}
            Ok(tokens) => {
                let cmd = &tokens[0];
                let args = &tokens[1..];
                match registry::canonical_name(cmd).unwrap_or(cmd) {
                    "matrix" => matrix::run(args),
                    "bootcfg" => handle_bootcfg(boot_config, args),
                    _ => dispatch(&mut shell, cmd, args),
                }
            }
            Err(err) => eprintln!("parse error: {}", err),
        }
    }
}

fn handle_bootcfg(config: ui::BootConfig, args: &[String]) {
    match args.first().map(String::as_str) {
        None | Some("show") => print_boot_config(config),
        Some("save") => match config.save() {
            Ok(()) => println!("bootcfg: saved active profile to {}", ui::config_path()),
            Err(err) => println!("bootcfg: save failed: {err}"),
        },
        Some("reset") | Some("defaults") => match ui::BootConfig::remove_saved() {
            Ok(()) => println!("bootcfg: removed {}; detected defaults will be used next launch", ui::config_path()),
            Err(err) => println!("bootcfg: reset failed: {err}"),
        },
        Some("path") => println!("{}", ui::config_path()),
        Some("help") | Some("-h") | Some("--help") => print_bootcfg_help(),
        Some(other) => {
            println!("bootcfg: unknown option '{other}'");
            print_bootcfg_help();
        }
    }
}

fn print_boot_config(config: ui::BootConfig) {
    println!("boot profile : {}", config.profile_name());
    println!("config file  : {}", ui::config_path());
    println!("color        : {}", if config.color { "on" } else { "off" });
    println!("ascii        : {}", if config.ascii_mode { "on" } else { "off" });
    println!("safe mode    : {}", if config.safe_mode { "on" } else { "off" });
    println!("quick boot   : {}", if config.quick_boot { "on" } else { "off" });
    println!("mobile mode  : {}", if config.mobile_mode { "on" } else { "off" });
}

fn print_bootcfg_help() {
    println!("usage: bootcfg [show|save|reset|path]");
    println!("  show   display the active boot profile");
    println!("  save   write the active profile to phase1.conf");
    println!("  reset  remove phase1.conf so detected defaults are used next launch");
    println!("  path   print the config file path");
}

#[cfg(test)]
mod tests {
    use super::compact_path;
    use std::path::Path;

    #[test]
    fn compact_path_shortens_home() {
        assert_eq!(compact_path(Path::new("/home")), "~");
        assert_eq!(compact_path(Path::new("/home/projects")), "~/projects");
        assert_eq!(compact_path(Path::new("/proc")), "/proc");
    }
}
