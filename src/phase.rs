pub fn run(args: &[String]) -> String {
    let action = args.first().map(String::as_str).unwrap_or("whereami");

    match action {
        "whereami" | "compass" | "path" | "map" | "status" => phase_compass_status(),
        "help" | "--help" | "-h" => phase_help(),
        other => unknown_phase_action(other),
    }
}

fn phase_help() -> String {
    let mut out = String::new();
    out.push_str("PHASE HELP\n");
    out.push_str("mode=status-only\n");
    out.push_str("commands=whereami compass path map status help\n");
    out.push_str(
        "boundary=no-live-movement no-origin-mutation no-host-effect no-external-effect\n",
    );
    out.push_str("examples:\n");
    out.push_str("  phase whereami\n");
    out.push_str("  phase compass\n");
    out.push_str("  phase path\n");
    out.push_str("  phase map\n");
    out
}

fn unknown_phase_action(action: &str) -> String {
    format!(
        "PHASE COMPASS\nmode=status-only\nstatus=unknown-action\naction={action}\nhint=phase whereami | phase compass | phase path | phase map | phase help\nmutation=disabled\nhost-effect=none\nexternal-effect=none\n"
    )
}

fn phase_compass_status() -> String {
    let mut out = String::new();
    out.push_str("PHASE COMPASS\n");
    out.push_str("mode=status-only\n");
    out.push_str("runtime=source-native\n");
    out.push_str("mutation=disabled\n");
    out.push_str("origin=0/0\n");
    out.push_str("root-anchor=ROOT\n");
    out.push_str("current-route=ROOT\n");
    out.push_str("current-axis=ROOT\n");
    out.push_str("path=ROOT>0/0\n");
    out.push_str("breadcrumb=ROOT\n");
    out.push_str("trace-id=trace-preview\n");
    out.push_str("safe-portal=planned\n");
    out.push_str("rollback-target=available\n");
    out.push_str("health=nominal\n");
    out.push_str("risk=low\n");
    out.push_str("lock-state=open\n");
    out.push_str("dark_phase=off\n");
    out.push_str("host-effect=none\n");
    out.push_str("external-effect=none\n");
    out.push_str("operator-intent=explicit\n");
    out.push_str("claim-boundary=phase-compass-status-only\n");
    out.push('\n');
    out.push_str("SUPPORTED ROUTES\n");
    out.push_str("phase whereami\n");
    out.push_str("phase compass\n");
    out.push_str("phase path\n");
    out.push_str("phase map\n");
    out.push('\n');
    out.push_str("ROOT DIRECTION MAP\n");
    out.push_str("              u/NUM\n");
    out.push_str("                ^\n");
    out.push_str("                |\n");
    out.push_str("L/NUM  <----  ROOT  ---->  R/NUM\n");
    out.push_str("                |\n");
    out.push_str("                v\n");
    out.push_str("              d/NUM\n");
    out.push_str("rule=root-remains-anchor\n");
    out.push_str("external-link=none\n");
    out
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn phase_whereami_reports_status_only_native_compass() {
        let output = run(&["whereami".to_string()]);

        for required in [
            "PHASE COMPASS",
            "mode=status-only",
            "runtime=source-native",
            "mutation=disabled",
            "origin=0/0",
            "root-anchor=ROOT",
            "current-route=ROOT",
            "current-axis=ROOT",
            "path=ROOT>0/0",
            "breadcrumb=ROOT",
            "trace-id=trace-preview",
            "safe-portal=planned",
            "rollback-target=available",
            "operator-intent=explicit",
            "claim-boundary=phase-compass-status-only",
            "L/NUM  <----  ROOT  ---->  R/NUM",
        ] {
            assert!(output.contains(required), "missing {required}:\n{output}");
        }
    }

    #[test]
    fn phase_aliases_share_status_surface() {
        for action in ["compass", "path", "map", "status"] {
            let output = run(&[action.to_string()]);
            assert!(output.contains("PHASE COMPASS"), "{output}");
            assert!(output.contains("runtime=source-native"), "{output}");
            assert!(output.contains("mutation=disabled"), "{output}");
        }
    }

    #[test]
    fn phase_unknown_action_stays_status_only() {
        let output = run(&["shift".to_string()]);
        assert!(output.contains("status=unknown-action"), "{output}");
        assert!(output.contains("mutation=disabled"), "{output}");
        assert!(output.contains("host-effect=none"), "{output}");
        assert!(output.contains("external-effect=none"), "{output}");
    }
}
