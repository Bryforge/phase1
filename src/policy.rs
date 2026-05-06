#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub audited: bool,
}

impl PolicyDecision {
    pub fn allowed(&self) -> bool {
        self.result == PolicyResult::Allow
    }

    pub fn audit_event(&self, user: &str) -> String {
        format!(
            "user={user} action=policy.check object={} result={} reason={} capability={}",
            self.command,
            match self.result {
                PolicyResult::Allow => "allow",
                PolicyResult::Deny => "deny",
            },
            self.reason,
            self.capability
        )
    }
}

pub fn check(command: &str, capability: &str, safe_mode: bool) -> PolicyDecision {
    let guarded = guarded_by_safe_mode(command, capability);
    let audited = guarded || capability.starts_with("host.") || capability == "net.admin" || capability == "plugin.exec";

    if safe_mode && guarded {
        PolicyDecision {
            command: command.to_string(),
            capability: capability.to_string(),
            result: PolicyResult::Deny,
            reason: "safe-mode",
            audited: true,
        }
    } else {
        PolicyDecision {
            command: command.to_string(),
            capability: capability.to_string(),
            result: PolicyResult::Allow,
            reason: if audited { "policy-ok" } else { "open" },
            audited,
        }
    }
}

pub fn check_plugin(name: &str, safe_mode: bool) -> PolicyDecision {
    check(&format!("plugin:{name}"), "plugin.exec", safe_mode)
}

fn guarded_by_safe_mode(command: &str, capability: &str) -> bool {
    matches!(
        command,
        "browser" | "ping" | "wifi-scan" | "wifi-connect" | "python" | "gcc"
    ) || matches!(capability, "host.exec" | "host.net" | "net.admin" | "plugin.exec")
}

#[cfg(test)]
mod tests {
    use super::{check, PolicyResult};

    #[test]
    fn safe_mode_denies_host_commands() {
        let decision = check("browser", "host.net", true);
        assert_eq!(decision.result, PolicyResult::Deny);
        assert_eq!(decision.reason, "safe-mode");
    }

    #[test]
    fn normal_mode_allows_host_commands_but_audits_them() {
        let decision = check("browser", "host.net", false);
        assert_eq!(decision.result, PolicyResult::Allow);
        assert!(decision.audited);
    }
}
