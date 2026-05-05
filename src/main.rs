mod browser;
mod commands;
mod kernel;
mod man;
mod ned;
mod network;
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
    let mut shell = Phase1Shell::new();

    ui::print_boot(kernel::VERSION);
    shell.cmd_cd(Some("/home"));
    println!("phase1 {} ready. Type 'help' for commands.", kernel::VERSION);

    let stdin = io::stdin();
    let mut input = String::with_capacity(256);

    loop {
        let uptime_secs = shell.start_time.elapsed().as_secs();
        shell.kernel.tick(uptime_secs);

        let path = compact_path(&shell.kernel.vfs.cwd);
        print!("{}@phase1 {} › ", shell.user(), path);
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
                if tokens[0] == "help" {
                    ui::print_help();
                } else {
                    let cmd = &tokens[0];
                    let args = &tokens[1..];
                    dispatch(&mut shell, cmd, args);
                }
            }
            Err(err) => eprintln!("parse error: {}", err),
        }
    }
}
