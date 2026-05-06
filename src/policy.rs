#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyResult {
    Allow,
    Deny,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDecision {
    pub command: String,
    pub capability: String,
    pub result: PolicyResult,
    pub reason: &'static str,
    pub host_backed: bool,
}

impl PolicyDecision {
    pub fn allowed(&self) -> bool {
        self.result == PolicyResult::Allow
    }

    pub fn audit_event(&self, user: &str) -> String {
        format!(
            "user={user} action=policy.check object={} capability={} result={} reason={} host_backed={}",
            self.command,
            self.capability,
            match self.result {
                PolicyResult::Allow => "allow",
                PolicyResult::Deny => "deny",
            },
            self.reason,
            self.host_backed
        )
    }
}

pub fn check(command: &str, capability: &str) -> PolicyDecision {
    let host_backed = is_host_backed(command, capability);
    if host_backed && safe_mode_enabled() {
        return decision(command, capability, host_backed, PolicyResult::Deny, "safe-mode");
    }
    if host_backed && !host_tools_enabled() {
        return decision(
            command,
            capability,
            host_backed,
            PolicyResult::Deny,
            "host-tools-disabled",
        );
    }
    decision(command, capability, host_backed, PolicyResult::Allow, "policy-ok")
}

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

fn decision(
    command: &str,
    capability: &str,
    host_backed: bool,
    result: PolicyResult,
    reason: &'static str,
) -> PolicyDecision {
    PolicyDecision {
        command: command.to_string(),
        capability: capability.to_string(),
        result,
        reason,
        host_backed,
    }
}

fn is_host_backed(command: &str, capability: &str) -> bool {
    matches!(capability, "host.exec" | "host.net" | "net.admin")
        || matches!(command, "ping" | "wifi-scan" | "nmcli")
}

#[cfg(test)]
mod tests {
    use super::{check, host_tools_allowed, host_tools_enabled, PolicyResult};

    #[test]
    fn safe_mode_denies_host_backed_commands() {
        std::env::set_var("PHASE1_SAFE_MODE", "1");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        let decision = check("python", "host.exec");
        assert_eq!(decision.result, PolicyResult::Deny);
        assert_eq!(decision.reason, "safe-mode");
        assert!(!host_tools_allowed());
        std::env::remove_var("PHASE1_SAFE_MODE");
    }

    #[test]
    fn host_tools_require_explicit_opt_in() {
        std::env::set_var("PHASE1_SAFE_MODE", "0");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        assert!(!host_tools_enabled());
        assert!(!host_tools_allowed());
        let decision = check("python", "host.exec");
        assert_eq!(decision.result, PolicyResult::Deny);
        assert_eq!(decision.reason, "host-tools-disabled");
        std::env::set_var("PHASE1_ALLOW_HOST_TOOLS", "1");
        assert!(host_tools_allowed());
        std::env::remove_var("PHASE1_SAFE_MODE");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
    }
}
