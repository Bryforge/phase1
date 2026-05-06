#![allow(clippy::assertions_on_constants)]

mod browser;
mod commands;
mod kernel;
mod man;
mod matrix;
mod ned;
mod network;
mod policy;
mod registry;
mod ui;

use commands::{dispatch, parse_line, Phase1Shell};
use kernel::VfsNode;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const PERSISTENT_STATE_PATH: &str = "phase1.state";

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
    shell.env.insert(
        "PHASE1_BOOT_PROFILE".to_string(),
        boot_config.profile_name().to_string(),
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
        "PHASE1_PERSISTENT_STATE".to_string(),
        if boot_config.persistent_state { "1" } else { "0" }.to_string(),
    );

    if boot_config.persistent_state {
        match load_persistent_state(&mut shell) {
            Ok(count) if count > 0 => {
                println!("persistent state: restored {count} entries from {PERSISTENT_STATE_PATH}")
            }
            Ok(_) => {
                println!("persistent state: enabled; no saved state found at {PERSISTENT_STATE_PATH}")
            }
            Err(err) => println!("persistent state: restore warning: {err}"),
        }
    }

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
                    "help" => ui::print_help(),
                    "accounts" => print!("{}", accounts_report(&shell)),
                    "security" => print!(
                        "{}",
                        policy::security_report(boot_config.persistent_state, "memory-only")
                    ),
                    "matrix" => matrix::run(args),
                    "bootcfg" => handle_bootcfg(boot_config, args),
                    _ => dispatch(&mut shell, cmd, args),
                }
                if boot_config.persistent_state {
                    if let Err(err) = save_persistent_state(&shell) {
                        eprintln!("persistent state save warning: {err}");
                    }
                }
            }
            Err(err) => eprintln!("parse error: {}", err),
        }
    }

    if boot_config.persistent_state {
        if let Err(err) = save_persistent_state(&shell) {
            eprintln!("persistent state save warning: {err}");
        }
    }
}

fn accounts_report(shell: &Phase1Shell) -> String {
    let mut out = String::from(
        "phase1 accounts // simulated Unix account database\nsource : /etc/passwd\nnote   : x means the credential hash is not stored in this file\nsafety : no real emails, tokens, host users, or account secrets are stored here\n\nUSER       UID   GID   HOME       SHELL\n",
    );

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
    println!("config file       : {}", ui::config_path());
    println!("state file        : {}", PERSISTENT_STATE_PATH);
    println!("color             : {}", if config.color { "on" } else { "off" });
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
    use super::{compact_path, decode_hex, encode_hex};
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
}
