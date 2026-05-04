mod network;
mod kernel;
mod browser;
mod man;
mod ned;
mod commands;

use std::io::{self, Write};

use commands::Phase1Shell;

fn main() {
    let mut shell = Phase1Shell::new();
    Phase1Shell::print_boot();
    shell.cmd_cd(Some("/home"));
    println!("\x1b[32mphase1 v3.3.1 ready. Type 'help' for commands.\x1b[0m");

    let mut input = String::with_capacity(256);

    loop {
        let uptime_secs = shell.start_time.elapsed().as_secs();
        shell.kernel.tick(uptime_secs);

        print!("\x1b[36m@phase1\x1b[0m:\x1b[34m{}\x1b[0m$ ", shell.kernel.vfs.cwd.display());
        let _ = io::stdout().flush();

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        shell.history.push_back(line.to_string());
        if shell.history.len() > 300 {
            shell.history.pop_front();
        }

        let expanded = shell.expand_env(line);
        let parts: Vec<&str> = expanded.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0];
        let args = &parts[1..];

        commands::dispatch(&mut shell, cmd, args);
    }
}
