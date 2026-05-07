use crate::commands::Phase1Shell;

const RPI_ENV: &[(&str, &str)] = &[
    ("PHASE1_PLATFORM", "raspberry-pi-5"),
    ("PHASE1_DEVICE_MODE", "raspberry-pi"),
    ("PHASE1_RPI_COMPAT", "1"),
    ("PHASE1_TERMINAL_COMPAT", "1"),
    ("PHASE1_COOKED_INPUT", "1"),
    ("PHASE1_ASCII", "1"),
    ("PHASE1_FORCE_ASCII", "1"),
    ("PHASE1_NO_COLOR", "1"),
    ("PHASE1_COLOR_PACK", "raspberry-pi"),
    ("PHASE1_COLOR_DEPTH", "mono"),
    ("PHASE1_THEME", "mono"),
    ("PHASE1_QUICK_BOOT", "1"),
    ("PHASE1_SAFE_MODE", "1"),
    ("PHASE1_IDLE_ENTER_GUARD_SECONDS", "0"),
];

pub fn is_requested() -> bool {
    enabled("PHASE1_RPI_COMPAT")
        || enabled("PHASE1_PI_MODE")
        || platform_hint().contains("raspberry")
        || platform_hint().contains("raspi")
        || platform_hint().contains("rpi5")
}

pub fn apply_process_defaults() {
    if !is_requested() {
        return;
    }
    for (key, value) in RPI_ENV {
        std::env::set_var(key, value);
    }
    if std::env::var_os("TERM").is_none() {
        std::env::set_var("TERM", "xterm-256color");
    }
    if std::env::var_os("COLUMNS").is_none() {
        std::env::set_var("COLUMNS", "80");
    }
    if std::env::var_os("LANG").is_none() {
        std::env::set_var("LANG", "C.UTF-8");
    }
    if std::env::var_os("LC_ALL").is_none() {
        std::env::set_var("LC_ALL", "C.UTF-8");
    }
}

pub fn apply_shell(shell: &mut Phase1Shell) -> String {
    for (key, value) in RPI_ENV {
        std::env::set_var(key, value);
        shell.env.insert((*key).to_string(), (*value).to_string());
    }
    let columns = std::env::var("COLUMNS").unwrap_or_else(|_| "80".to_string());
    shell.env.insert("COLUMNS".to_string(), columns);
    report()
}

pub fn report() -> String {
    format!(
        "raspberry pi compatibility mode\nstatus   : {}\ninput    : cooked line mode\ndisplay  : ascii/mono terminal-safe output\ncolors   : disabled for maximum compatibility\nwidth    : {} columns\nsecurity : safe mode on; host tools remain guarded\nlauncher : bash scripts/phase1-rpi5.sh\n",
        if is_requested() { "active" } else { "available" },
        std::env::var("COLUMNS").unwrap_or_else(|_| "80".to_string())
    )
}

fn enabled(key: &str) -> bool {
    matches!(
        std::env::var(key).ok().as_deref(),
        Some("1" | "true" | "yes" | "on" | "pi" | "rpi" | "rpi5")
    )
}

fn platform_hint() -> String {
    ["PHASE1_PLATFORM", "PHASE1_DEVICE_MODE", "HOSTNAME"]
        .iter()
        .filter_map(|key| std::env::var(key).ok())
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{apply_process_defaults, is_requested};

    #[test]
    fn rpi_mode_can_be_requested_by_env() {
        std::env::set_var("PHASE1_RPI_COMPAT", "1");
        assert!(is_requested());
        apply_process_defaults();
        assert_eq!(std::env::var("PHASE1_COOKED_INPUT").ok().as_deref(), Some("1"));
        assert_eq!(std::env::var("PHASE1_NO_COLOR").ok().as_deref(), Some("1"));
        std::env::remove_var("PHASE1_RPI_COMPAT");
    }
}
