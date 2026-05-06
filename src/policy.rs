pub fn security_report(persistent_state: bool, history_state: &str) -> String {
    format!(
        "security mode       : {}\nhost tools          : {}\nhost network changes: {}\npersistent state    : {}\nhistory             : {}\nprivacy             : no real emails, passwords, tokens, or account secrets are stored by phase1\n",
        if safe_mode_enabled() { "safe" } else { "host-capable" },
        if host_tools_enabled() { "enabled" } else { "disabled" },
        if host_network_changes_enabled() { "enabled" } else { "disabled" },
        if persistent_state { "on" } else { "off" },
        history_state
    )
}

pub fn safe_mode_enabled() -> bool {
    !matches!(
        std::env::var("PHASE1_SAFE_MODE").ok().as_deref(),
        Some("0" | "false" | "off" | "no")
    )
}

pub fn host_tools_enabled() -> bool {
    std::env::var("PHASE1_ALLOW_HOST_TOOLS").ok().as_deref() == Some("1")
}

pub fn host_network_changes_enabled() -> bool {
    std::env::var("PHASE1_ALLOW_HOST_NETWORK_CHANGES")
        .ok()
        .as_deref()
        == Some("1")
}

pub fn host_tools_allowed() -> bool {
    !safe_mode_enabled() && host_tools_enabled()
}

pub fn host_denial_message(command: &str) -> String {
    if safe_mode_enabled() {
        format!("{command}: disabled by safe boot profile")
    } else if !host_tools_enabled() {
        format!("{command}: disabled; set PHASE1_ALLOW_HOST_TOOLS=1 to enable trusted host tools")
    } else {
        format!("{command}: blocked by policy")
    }
}

#[cfg(test)]
mod tests {
    use super::{host_tools_allowed, host_tools_enabled, safe_mode_enabled};

    #[test]
    fn secure_defaults_block_host_tools() {
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        assert!(safe_mode_enabled());
        assert!(!host_tools_enabled());
        assert!(!host_tools_allowed());
    }

    #[test]
    fn host_tools_require_explicit_opt_in() {
        std::env::set_var("PHASE1_SAFE_MODE", "0");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        assert!(!host_tools_enabled());
        assert!(!host_tools_allowed());
        std::env::set_var("PHASE1_ALLOW_HOST_TOOLS", "1");
        assert!(host_tools_allowed());
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
    }
}
