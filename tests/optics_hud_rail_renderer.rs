#[path = "../src/optics.rs"]
mod optics;

use optics::{
    colorize_user_input, render_bottom_rail, render_pro_shell_layers, render_static_preview,
    render_top_rail, supported_device_labels, supported_device_profiles, OpticsDeviceProfile,
    OpticsRailState, USER_INPUT_BRIGHT_YELLOW,
};

#[test]
fn optics_renderer_static_preview_preserves_top_center_bottom_contract() {
    let preview = render_static_preview(OpticsDeviceProfile::Desktop);

    for required in [
        "OPTICS HUD RAIL RENDER",
        "status=static-render",
        "runtime=not-wired",
        "TOP product=Phase1 channel=edge profile=PRO",
        "ctx=root > nest:0/1 > portal:none > ghost:none",
        "integrity=not-checked",
        "crypto=chain-planned",
        "CENTER role=command-output chrome=none-permanent",
        "CENTER rule=center-remains-primary-workspace",
        "BOT color=bright-blue input=active mutation=none",
        "copy-safe=raw-command-preserved",
        "labels=no-color/ascii-visible",
    ] {
        assert!(
            preview.contains(required),
            "missing {required:?}: {preview}"
        );
    }
}

#[test]
fn optics_pro_shell_layers_keep_a_b_blank_c_d_order() {
    let mut state = OpticsRailState::pro_static(OpticsDeviceProfile::Terminal);
    state.mutation = "typing".to_string();
    state.command_family = "security".to_string();
    let frame = render_pro_shell_layers(&state, "security status", false);

    let top = frame.find("A TOP RAIL").expect("top rail");
    let command = frame.find("B COMMAND RAIL").expect("command rail");
    let status = frame.find("C STATUS HUD").expect("status hud");
    let bottom = frame.find("D BOTTOM HUD").expect("bottom hud");

    assert!(top < command, "{frame}");
    assert!(command < status, "{frame}");
    assert!(status < bottom, "{frame}");
    assert!(
        frame.contains("phase1://edge/root > security status\n\nC STATUS HUD"),
        "B and C must be separated by a blank line: {frame}"
    );
}

#[test]
fn optics_pro_shell_layers_color_code_without_reusing_bright_yellow() {
    let mut state = OpticsRailState::pro_static(OpticsDeviceProfile::Terminal);
    state.mutation = "typing".to_string();
    let frame = render_pro_shell_layers(&state, "optics status", true);

    let highlighted = format!("{USER_INPUT_BRIGHT_YELLOW}optics status");
    assert!(frame.contains(&highlighted), "typed text must be bright yellow: {frame:?}");
    assert_eq!(
        frame.matches(USER_INPUT_BRIGHT_YELLOW).count(),
        1,
        "bright yellow is reserved for typed/copied user input only: {frame:?}"
    );
    assert!(frame.contains("\x1b[36mA TOP RAIL"), "top rail should be cyan: {frame:?}");
    assert!(frame.contains("\x1b[34mB COMMAND RAIL"), "command rail label should be blue: {frame:?}");
    assert!(frame.contains("\x1b[32mC STATUS HUD"), "status hud should be green: {frame:?}");
    assert!(frame.contains("\x1b[35mD BOTTOM HUD"), "bottom hud should be magenta: {frame:?}");
}

#[test]
fn optics_user_input_highlight_reserves_bright_yellow() {
    let colored = colorize_user_input("rm scratch", true);
    assert!(colored.contains(USER_INPUT_BRIGHT_YELLOW));
    assert!(colored.contains("rm scratch"));
    assert_eq!(colored.matches(USER_INPUT_BRIGHT_YELLOW).count(), 1);

    let plain = colorize_user_input("rm scratch", false);
    assert_eq!(plain, "rm scratch");
}

#[test]
fn optics_renderer_exposes_supported_device_profiles_without_dead_code() {
    let profiles = supported_device_profiles();
    let labels = supported_device_labels();

    assert_eq!(profiles.len(), 4);
    assert!(profiles.contains(&OpticsDeviceProfile::Mobile));
    assert!(profiles.contains(&OpticsDeviceProfile::Laptop));
    assert!(profiles.contains(&OpticsDeviceProfile::Desktop));
    assert!(profiles.contains(&OpticsDeviceProfile::Terminal));
    assert_eq!(labels, "mobile,laptop,desktop,terminal");
}

#[test]
fn optics_renderer_keeps_rails_deterministic_and_ascii_safe() {
    let first = render_static_preview(OpticsDeviceProfile::Terminal);
    let second = render_static_preview(OpticsDeviceProfile::Terminal);

    assert_eq!(first, second, "static renderer must be deterministic");
    assert!(
        first.is_ascii(),
        "static renderer should be ASCII-safe: {first}"
    );
    assert!(
        !first.contains('\u{1b}'),
        "renderer should not emit ANSI escapes by default"
    );
    assert!(
        !first.contains('╭'),
        "renderer should not emit box drawing by default"
    );
}

#[test]
fn optics_renderer_adapts_device_density_without_changing_state_meaning() {
    let mobile = render_static_preview(OpticsDeviceProfile::Mobile);
    let laptop = render_static_preview(OpticsDeviceProfile::Laptop);
    let desktop = render_static_preview(OpticsDeviceProfile::Desktop);

    assert!(mobile.contains("BOT color=bright-blue input=active mutation=none result=ok"));
    assert!(
        !mobile.contains("BOT warning="),
        "mobile should stay one-line bottom rail: {mobile}"
    );

    assert!(laptop.contains("device=laptop"), "{laptop}");
    assert!(
        laptop.contains("BOT warning=none copy-safe=raw-command-preserved"),
        "{laptop}"
    );

    assert!(
        desktop.contains("device=desktop evidence=planned"),
        "{desktop}"
    );
    assert!(
        desktop.contains("labels=no-color/ascii-visible"),
        "{desktop}"
    );
}

#[test]
fn optics_renderer_custom_state_preserves_command_and_safety_labels() {
    let mut state = OpticsRailState::pro_static(OpticsDeviceProfile::Laptop);
    state.context = "root > portal:alpha > ghost:watch".to_string();
    state.integrity = "changed".to_string();
    state.crypto = "denied".to_string();
    state.mutation = "typing".to_string();
    state.command_family = "crypto".to_string();
    state.active_task = "verify".to_string();
    state.last_result = "warning".to_string();
    state.warning = "guarded-operation".to_string();

    let top = render_top_rail(&state);
    let bottom = render_bottom_rail(&state);

    assert!(
        top.contains("ctx=root > portal:alpha > ghost:watch"),
        "{top}"
    );
    assert!(top.contains("integrity=changed"), "{top}");
    assert!(top.contains("crypto=denied"), "{top}");
    assert!(bottom.contains("mutation=typing"), "{bottom}");
    assert!(bottom.contains("command=crypto"), "{bottom}");
    assert!(bottom.contains("task=verify"), "{bottom}");
    assert!(bottom.contains("result=warning"), "{bottom}");
    assert!(bottom.contains("warning=guarded-operation"), "{bottom}");
}

#[test]
fn optics_renderer_preserves_non_claims() {
    let preview = render_static_preview(OpticsDeviceProfile::Desktop);

    for required in [
        "not-compositor",
        "not-terminal-emulator",
        "not-sandbox",
        "not-security-boundary",
        "not-crypto-enforcement",
        "not-system-integrity-guarantee",
        "not-base1-boot-environment",
    ] {
        assert!(
            preview.contains(required),
            "missing {required:?}: {preview}"
        );
    }
}
