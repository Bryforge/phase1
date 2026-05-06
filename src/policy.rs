pub fn security_report(persistent_state: bool, history_state: &str) -> String {
    format!(
        "security mode       : {}\nhost tools          : {}\nhost network changes: {}\ncapability metadata : {}\npersistent state    : {}\nhistory             : {}\nprivacy             : no real emails, passwords, tokens, or account secrets are stored by phase1\n",
        if safe_mode_enabled() { "safe" } else { "host-capable" },
        if host_tools_enabled() { "enabled" } else { "disabled" },
        if host_network_changes_enabled() { "enabled" } else { "disabled" },
        capability_metadata_status(),
        if persistent_state { "on" } else { "off" },
        history_state
    )
}

pub fn safe_mode_enabled() -> bool {
    safe_mode_from_value(std::env::var("PHASE1_SAFE_MODE").ok().as_deref())
}

pub fn host_tools_enabled() -> bool {
    host_tools_from_value(std::env::var("PHASE1_ALLOW_HOST_TOOLS").ok().as_deref())
}

pub fn host_network_changes_enabled() -> bool {
    host_tools_from_value(
        std::env::var("PHASE1_ALLOW_HOST_NETWORK_CHANGES")
            .ok()
            .as_deref(),
    )
}

pub fn host_tools_allowed() -> bool {
    host_tools_allowed_from_values(
        std::env::var("PHASE1_SAFE_MODE").ok().as_deref(),
        std::env::var("PHASE1_ALLOW_HOST_TOOLS").ok().as_deref(),
    )
}

pub fn capability_denial_message(command: &str, capability: &str) -> Option<String> {
    capability_denial_message_from_values(
        command,
        capability,
        std::env::var("PHASE1_SAFE_MODE").ok().as_deref(),
        std::env::var("PHASE1_ALLOW_HOST_TOOLS").ok().as_deref(),
    )
}

pub fn host_denial_message(command: &str) -> String {
    host_denial_message_from_values(command, safe_mode_enabled(), host_tools_enabled())
}

pub fn capability_metadata_status() -> &'static str {
    let _ = capability_denial_message("python", "host.exec");
    "enforced"
}

fn safe_mode_from_value(value: Option<&str>) -> bool {
    !matches!(value, Some("0" | "false" | "off" | "no"))
}

fn host_tools_from_value(value: Option<&str>) -> bool {
    value == Some("1")
}

fn host_tools_allowed_from_values(safe_mode: Option<&str>, host_tools: Option<&str>) -> bool {
    !safe_mode_from_value(safe_mode) && host_tools_from_value(host_tools)
}

fn capability_denial_message_from_values(
    command: &str,
    capability: &str,
    safe_mode: Option<&str>,
    host_tools: Option<&str>,
) -> Option<String> {
    match capability {
        "host.exec" | "host.net" if command != "update" => {
            (!host_tools_allowed_from_values(safe_mode, host_tools)).then(|| {
                host_denial_message_from_values(
                    command,
                    safe_mode_from_value(safe_mode),
                    host_tools_from_value(host_tools),
                )
            })
        }
        "net.admin" => (!host_tools_allowed_from_values(safe_mode, host_tools)).then(|| {
            host_denial_message_from_values(
                command,
                safe_mode_from_value(safe_mode),
                host_tools_from_value(host_tools),
            )
        }),
        _ => None,
    }
}

fn host_denial_message_from_values(
    command: &str,
    safe_mode: bool,
    host_tools_enabled: bool,
) -> String {
    if safe_mode {
        format!(
            "{command}: disabled by safe boot profile; for Python/language runtimes reboot and press 4 then t, or use the new r/runtimes boot shortcut when available"
        )
    } else if !host_tools_enabled {
        format!(
            "{command}: disabled; set PHASE1_ALLOW_HOST_TOOLS=1 to enable trusted host tools, or reboot and press t / r for runtimes"
        )
    } else {
        format!("{command}: blocked by policy")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        capability_denial_message_from_values, host_tools_allowed_from_values,
        host_tools_from_value, safe_mode_from_value, security_report,
    };

    #[test]
    fn secure_defaults_block_host_tools() {
        assert!(safe_mode_from_value(None));
        assert!(!host_tools_from_value(None));
        assert!(!host_tools_allowed_from_values(None, None));
    }

    #[test]
    fn host_tools_require_explicit_opt_in() {
        assert!(!host_tools_allowed_from_values(Some("0"), None));
        assert!(!host_tools_allowed_from_values(None, Some("1")));
        assert!(host_tools_allowed_from_values(Some("0"), Some("1")));
    }

    #[test]
    fn command_metadata_blocks_guarded_capabilities() {
        let safe =
            capability_denial_message_from_values("python", "host.exec", None, None).unwrap();
        assert!(safe.contains("disabled by safe boot profile"));

        let host =
            capability_denial_message_from_values("browser", "host.net", Some("0"), None).unwrap();
        assert!(host.contains("PHASE1_ALLOW_HOST_TOOLS"));

        assert!(
            capability_denial_message_from_values("python", "host.exec", Some("0"), Some("1"))
                .is_none()
        );
        assert!(capability_denial_message_from_values(
            "wifi-connect",
            "net.admin",
            Some("0"),
            Some("1")
        )
        .is_none());
    }

    #[test]
    fn update_plan_is_not_blocked_by_host_exec_metadata() {
        assert!(capability_denial_message_from_values("update", "host.exec", None, None).is_none());
    }

    #[test]
    fn security_report_mentions_metadata_enforcement() {
        let out = security_report(false, "memory-only");
        assert!(out.contains("capability metadata : enforced"));
    }
}
