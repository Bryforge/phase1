mod browser;
mod commands;
mod kernel;
mod man;
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
    let boot_config = ui::configure_boot(kernel::VERSION);
    boot_config.apply();

    let mut shell = Phase1Shell::new();
    shell.env.insert("PHASE1_BOOT_PROFILE".to_string(), boot_config.profile_name().to_string());
    shell.env.insert("PHASE1_SAFE_MODE".to_string(), if boot_config.safe_mode { "1" } else { "0" }.to_string());

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
                dispatch(&mut shell, cmd, args);
            }
            Err(err) => eprintln!("parse error: {}", err),
        }
    }
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
