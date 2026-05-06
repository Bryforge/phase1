pub const PROTOCOL_FILE: &str = "UPDATE_PROTOCOL.md";
pub const VERSION_FORMAT: &str = "MAJOR.MINOR.PATCH[-dev]";
pub const CURRENT_EDGE_VERSION: &str = "3.7.1-dev";

pub fn report() -> String {
    let mut out = String::from("phase1 update protocol\n");
    out.push_str(&format!("reference file : {PROTOCOL_FILE}\n"));
    out.push_str(&format!("version format : {VERSION_FORMAT}\n"));
    out.push_str(&format!("edge version   : {CURRENT_EDGE_VERSION}\n"));
    out.push_str("\nupdate rules\n");
    out.push_str("  - keep stable releases on exact semantic versions such as 3.6.0\n");
    out.push_str("  - keep master/bleeding edge on semantic prereleases such as 3.7.1-dev\n");
    out.push_str("  - use the third number as PATCH for every safe fix, docs, protocol, and incremental feature update\n");
    out.push_str("  - bump MINOR only when a roadmap track gains a meaningful new capability set\n");
    out.push_str("  - bump MAJOR only for deliberate compatibility-breaking behavior\n");
    out.push_str("\nsafety gates\n");
    out.push_str("  - update without --execute is always a dry-run plan\n");
    out.push_str("  - update --execute requires safe mode off and PHASE1_ALLOW_HOST_TOOLS=1\n");
    out.push_str("  - tracked local changes block execution instead of being overwritten\n");
    out.push_str("  - updater output is sanitized for tokens, URL credentials, and account secrets\n");
    out
}

#[cfg(test)]
mod tests {
    use super::{report, CURRENT_EDGE_VERSION, PROTOCOL_FILE, VERSION_FORMAT};

    #[test]
    fn protocol_report_documents_patch_versioning() {
        let out = report();
        assert!(out.contains(PROTOCOL_FILE));
        assert!(out.contains(VERSION_FORMAT));
        assert!(out.contains(CURRENT_EDGE_VERSION));
        assert!(out.contains("third number as PATCH"));
        assert!(out.contains("PHASE1_ALLOW_HOST_TOOLS"));
    }
}
