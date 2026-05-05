mod browser;
mod commands;
mod kernel;
mod man;
mod ned;
mod network;

use commands::{dispatch, parse_line, Phase1Shell};
use std::io::{self, Write};

fn main() {
    let mut shell = Phase1Shell::new();

    Phase1Shell::print_boot();
    shell.cmd_cd(Some("/home"));
    println!("phase1 {} ready. Type 'help' for commands.", kernel::VERSION);

    let stdin = io::stdin();
    let mut input = String::with_capacity(256);

    loop {
        let uptime_secs = shell.start_time.elapsed().as_secs();
        shell.kernel.tick(uptime_secs);

        print!(
            "{}@phase1:{}$ ",
            shell.user(),
            shell.kernel.vfs.cwd.display()
        );
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
                dispatch(&mut shell, cmd, args);
            }
            Err(err) => eprintln!("parse error: {}", err),
        }
    }
}
