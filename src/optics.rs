pub const USER_INPUT_BRIGHT_YELLOW: &str = "\x1b[93m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpticsDeviceProfile {
    Mobile,
    Laptop,
    Desktop,
    Terminal,
}

impl OpticsDeviceProfile {
    pub fn as_label(self) -> &'static str {
        match self {
            Self::Mobile => "mobile",
            Self::Laptop => "laptop",
            Self::Desktop => "desktop",
            Self::Terminal => "terminal",
        }
    }
}

pub fn supported_device_profiles() -> [OpticsDeviceProfile; 4] {
    [
        OpticsDeviceProfile::Mobile,
        OpticsDeviceProfile::Laptop,
        OpticsDeviceProfile::Desktop,
        OpticsDeviceProfile::Terminal,
    ]
}

pub fn supported_device_labels() -> String {
    supported_device_profiles()
        .into_iter()
        .map(OpticsDeviceProfile::as_label)
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpticsRailState {
    pub product: String,
    pub channel: String,
    pub profile: String,
    pub context: String,
    pub trust: String,
    pub security: String,
    pub integrity: String,
    pub crypto: String,
    pub base1: String,
    pub fyr: String,
    pub device: OpticsDeviceProfile,
    pub input: String,
    pub mutation: String,
    pub command_family: String,
    pub active_task: String,
    pub last_result: String,
    pub warning: String,
}

impl OpticsRailState {
    pub fn pro_static(device: OpticsDeviceProfile) -> Self {
        Self {
            product: "Phase1".to_string(),
            channel: "edge".to_string(),
            profile: "PRO".to_string(),
            context: "root > nest:0/1 > portal:none > ghost:none".to_string(),
            trust: "safe/armed".to_string(),
            security: "safe-mode host-gated".to_string(),
            integrity: "not-checked".to_string(),
            crypto: "chain-planned".to_string(),
            base1: "evidence-planned".to_string(),
            fyr: "idle".to_string(),
            device,
            input: "active".to_string(),
            mutation: "none".to_string(),
            command_family: "none".to_string(),
            active_task: "idle".to_string(),
            last_result: "ok".to_string(),
            warning: "none".to_string(),
        }
    }
}

pub fn colorize_user_input(input: &str, color: bool) -> String {
    if color && !input.is_empty() {
        format!("{BOLD}{USER_INPUT_BRIGHT_YELLOW}{input}{RESET}")
    } else {
        input.to_string()
    }
}

fn colorize_layer(text: &str, color: bool, ansi: &str) -> String {
    if color {
        format!("{BOLD}{ansi}{text}{RESET}")
    } else {
        text.to_string()
    }
}

pub fn render_pro_shell_layers(state: &OpticsRailState, typed_input: &str, color: bool) -> String {
    let top_label = colorize_layer("A TOP RAIL", color, CYAN);
    let command_label = colorize_layer("B COMMAND RAIL", color, BLUE);
    let status_label = colorize_layer("C STATUS HUD", color, GREEN);
    let bottom_label = colorize_layer("D BOTTOM HUD", color, MAGENTA);
    let typed = colorize_user_input(typed_input, color);
    let mutation = mutation_label_color(&state.mutation, color);
    let result = result_label_color(&state.last_result, color);

    format!(
        "{top_label}\nproduct={} channel={} profile={} ctx={} trust={} security={} device={}\n\
         {command_label}\nphase1://edge/root > {typed}\n\n\
         {status_label}\nresult={} mutation={} integrity={} crypto={} base1={} fyr={}\n\
         {bottom_label}\ninput={} command={} task={} warning={} copy-safe=raw-command-preserved\n",
        state.product,
        state.channel,
        state.profile,
        state.context,
        state.trust,
        state.security,
        state.device.as_label(),
        result,
        mutation,
        state.integrity,
        state.crypto,
        state.base1,
        state.fyr,
        state.input,
        state.command_family,
        state.active_task,
        state.warning
    )
}

fn mutation_label_color(value: &str, color: bool) -> String {
    let ansi = match value {
        "typing" | "command-family-detected" => CYAN,
        "guarded" | "confirmation" => MAGENTA,
        "denied" | "failed" | "unsafe" | "invalid" => RED,
        "complete" | "success" => GREEN,
        _ => BLUE,
    };
    colorize_layer(value, color, ansi)
}

fn result_label_color(value: &str, color: bool) -> String {
    let ansi = match value {
        "ok" | "success" => GREEN,
        "warning" | "guarded" => MAGENTA,
        "failed" | "denied" | "invalid" => RED,
        _ => BLUE,
    };
    colorize_layer(value, color, ansi)
}

pub fn render_top_rail(state: &OpticsRailState) -> String {
    match state.device {
        OpticsDeviceProfile::Mobile => format!(
            "TOP product={} channel={} profile={} ctx={} trust={} device={}\n",
            state.product,
            state.channel,
            state.profile,
            state.context,
            state.trust,
            state.device.as_label()
        ),
        OpticsDeviceProfile::Laptop | OpticsDeviceProfile::Terminal => format!(
            "TOP product={} channel={} profile={} ctx={} trust={} security={}\nTOP integrity={} crypto={} base1={} fyr={} device={}\n",
            state.product,
            state.channel,
            state.profile,
            state.context,
            state.trust,
            state.security,
            state.integrity,
            state.crypto,
            state.base1,
            state.fyr,
            state.device.as_label()
        ),
        OpticsDeviceProfile::Desktop => format!(
            "TOP product={} channel={} profile={} ctx={} trust={} security={}\nTOP integrity={} crypto={} base1={} fyr={} device={} evidence=planned\n",
            state.product,
            state.channel,
            state.profile,
            state.context,
            state.trust,
            state.security,
            state.integrity,
            state.crypto,
            state.base1,
            state.fyr,
            state.device.as_label()
        ),
    }
}

pub fn render_center_viewport(sample: &str) -> String {
    format!(
        "CENTER role=command-output chrome=none-permanent\nCENTER rule=center-remains-primary-workspace\nCENTER sample={}\n",
        sample
    )
}

pub fn render_bottom_rail(state: &OpticsRailState) -> String {
    match state.device {
        OpticsDeviceProfile::Mobile => format!(
            "BOT color=bright-blue input={} mutation={} result={}\n",
            state.input, state.mutation, state.last_result
        ),
        OpticsDeviceProfile::Laptop | OpticsDeviceProfile::Terminal => format!(
            "BOT color=bright-blue input={} mutation={} command={} task={} result={}\nBOT warning={} copy-safe=raw-command-preserved\n",
            state.input,
            state.mutation,
            state.command_family,
            state.active_task,
            state.last_result,
            state.warning
        ),
        OpticsDeviceProfile::Desktop => format!(
            "BOT color=bright-blue input={} mutation={} command={} task={} result={}\nBOT warning={} copy-safe=raw-command-preserved labels=no-color/ascii-visible\n",
            state.input,
            state.mutation,
            state.command_family,
            state.active_task,
            state.last_result,
            state.warning
        ),
    }
}

pub fn render_static_preview(device: OpticsDeviceProfile) -> String {
    let state = OpticsRailState::pro_static(device);
    let mut out = String::from("OPTICS HUD RAIL RENDER\nstatus=static-render\nruntime=not-wired\n");
    out.push_str(&render_top_rail(&state));
    out.push_str(&render_center_viewport(
        "phase1://edge/root > optics rails preview",
    ));
    out.push_str(&render_bottom_rail(&state));
    out.push_str("NON-CLAIMS not-compositor not-terminal-emulator not-sandbox not-security-boundary not-crypto-enforcement not-system-integrity-guarantee not-base1-boot-environment\n");
    out
}
