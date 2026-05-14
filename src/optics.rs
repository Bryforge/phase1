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

pub fn render_top_rail(state: &OpticsRailState) -> String {
    match state.device {
        OpticsDeviceProfile::Mobile => format!(
            "TOP product={} channel={} profile={} ctx={} trust={}\n",
            state.product, state.channel, state.profile, state.context, state.trust
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
    out.push_str(&render_center_viewport("phase1://edge/root > optics rails preview"));
    out.push_str(&render_bottom_rail(&state));
    out.push_str("NON-CLAIMS not-compositor not-terminal-emulator not-sandbox not-security-boundary not-crypto-enforcement not-system-integrity-guarantee not-base1-boot-environment\n");
    out
}
