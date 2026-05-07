use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const REFRESH_CACHE_TTL: Duration = Duration::from_secs(2);
const SHORT_CMD_TIMEOUT: Duration = Duration::from_secs(3);
const WIFI_SCAN_TIMEOUT: Duration = Duration::from_secs(8);
const MUTATION_TIMEOUT: Duration = Duration::from_secs(15);
const PING_TIMEOUT: Duration = Duration::from_secs(8);
const MAX_COMMAND_OUTPUT: usize = 16_384;

#[derive(Clone, Debug)]
pub struct NetInterface {
    pub name: String,
    pub mac: String,
    pub ip: String,
    pub netmask: String,
    pub status: String,
    pub wifi_ssid: Option<String>,
    pub wifi_signal: Option<i32>,
    pub frequency: Option<f32>,
}

pub struct NetworkStack {
    interfaces: Vec<NetInterface>,
    last_refresh: Option<Instant>,
}

impl NetworkStack {
    pub fn new() -> Self {
        let mut stack = Self {
            interfaces: Vec::new(),
            last_refresh: None,
        };
        stack.force_refresh();
        stack
    }

    pub fn refresh(&mut self) {
        if self
            .last_refresh
            .is_some_and(|last| last.elapsed() < REFRESH_CACHE_TTL)
        {
            return;
        }
        self.force_refresh();
    }

    fn force_refresh(&mut self) {
        self.interfaces.clear();
        if host_tools_blocked() {
            self.interfaces.push(loopback());
            self.last_refresh = Some(Instant::now());
            return;
        }

        if cfg!(target_os = "linux") {
            self.refresh_linux();
        } else if cfg!(target_os = "macos") {
            self.refresh_macos();
        }

        if self.interfaces.is_empty() {
            self.interfaces.push(loopback());
        }
        self.interfaces.sort_by(|a, b| a.name.cmp(&b.name));
        self.last_refresh = Some(Instant::now());
    }

    pub fn ifconfig(&self) -> String {
        let mut out = String::new();
        for iface in &self.interfaces {
            out.push_str(&format!(
                "{}: flags=<{}> mtu 1500\n",
                iface.name,
                iface.status.to_ascii_uppercase()
            ));
            out.push_str(&format!(
                "    inet {} netmask {}\n",
                iface.ip, iface.netmask
            ));
            out.push_str(&format!("    ether {}\n", iface.mac));
            if let Some(ssid) = &iface.wifi_ssid {
                out.push_str(&format!("    wifi ssid=\"{}\"", sanitize_inline(ssid)));
                if let Some(signal) = iface.wifi_signal {
                    out.push_str(&format!(" signal={}dBm", signal));
                }
                if let Some(freq) = iface.frequency {
                    out.push_str(&format!(" freq={freq:.3}GHz"));
                }
                out.push('\n');
            }
            out.push('\n');
        }
        if host_tools_blocked() {
            out.push_str("safe-mode: host network inspection disabled\n");
        }
        out
    }

    pub fn iwconfig(&self) -> String {
        if host_tools_blocked() {
            return "safe-mode: host WiFi inspection disabled\n".to_string();
        }
        let mut out = String::new();
        for iface in &self.interfaces {
            if let Some(ssid) = &iface.wifi_ssid {
                out.push_str(&format!(
                    "{} IEEE 802.11 ESSID=\"{}\"\n",
                    iface.name,
                    sanitize_inline(ssid)
                ));
                if let Some(freq) = iface.frequency {
                    out.push_str(&format!("    Mode:Managed Frequency={freq:.3} GHz\n"));
                }
                if let Some(signal) = iface.wifi_signal {
                    out.push_str(&format!("    Signal level={signal} dBm\n"));
                }
            }
        }
        if out.is_empty() {
            "no active WiFi interface\n".to_string()
        } else {
            out
        }
    }

    pub fn wifi_scan(&self) -> String {
        if host_tools_blocked() {
            return "wifi-scan: disabled by safe boot profile\n".to_string();
        }
        if cfg!(target_os = "macos") {
            return macos_wifi_scan();
        }
        if cfg!(target_os = "linux") {
            if command_exists("nmcli") {
                let mut cmd = Command::new("nmcli");
                cmd.args([
                    "--terse",
                    "--fields",
                    "SSID,SECURITY,SIGNAL,FREQ,CHAN,IN-USE",
                    "dev",
                    "wifi",
                    "list",
                ]);
                return command_text(cmd, WIFI_SCAN_TIMEOUT, "nmcli WiFi scan unavailable");
            }
            if command_exists("iw") {
                let mut cmd = Command::new("iw");
                cmd.arg("dev");
                return command_text(cmd, WIFI_SCAN_TIMEOUT, "iw WiFi scan unavailable");
            }
        }
        "wifi-scan: unsupported platform or scanner unavailable\n".to_string()
    }

    pub fn wifi_connect(&mut self, ssid: &str, password: Option<&str>) -> String {
        if host_tools_blocked() {
            return "wifi-connect: disabled by safe boot profile".to_string();
        }
        if ssid.trim().is_empty() {
            return "usage: wifi-connect <ssid> [password]".to_string();
        }
        if !safe_ssid(ssid) {
            return "wifi-connect: invalid SSID".to_string();
        }
        if !crate::policy::host_network_changes_enabled() {
            return format!("dry-run: would connect to '{}'. Set PHASE1_ALLOW_HOST_NETWORK_CHANGES=1 to allow host network changes.", sanitize_inline(ssid));
        }

        let result = if cfg!(target_os = "macos") {
            let device = macos_wifi_device().unwrap_or_else(|| "en0".to_string());
            let mut cmd = Command::new("networksetup");
            cmd.arg("-setairportnetwork").arg(device).arg(ssid);
            if let Some(password) = password {
                cmd.arg(password);
            }
            command_text(cmd, MUTATION_TIMEOUT, "networksetup unavailable")
        } else if cfg!(target_os = "linux") {
            let mut cmd = Command::new("nmcli");
            cmd.args(["dev", "wifi", "connect", ssid]);
            if let Some(password) = password {
                cmd.arg("password").arg(password);
            }
            command_text(cmd, MUTATION_TIMEOUT, "nmcli unavailable")
        } else {
            format!("connected to {} (simulated)\n", sanitize_inline(ssid))
        };
        self.force_refresh();
        result
    }

    pub fn ping(&self, host: &str) -> String {
        if host_tools_blocked() {
            return "ping: disabled by safe boot profile\n".to_string();
        }
        if !safe_host(host) {
            return "ping: invalid host\n".to_string();
        }
        let mut cmd = Command::new("ping");
        if cfg!(target_os = "macos") {
            cmd.args(["-c", "4", "-W", "2000", host]);
        } else {
            cmd.args(["-c", "4", "-W", "2", host]);
        }
        command_text(cmd, PING_TIMEOUT, "ping unavailable")
    }

    pub fn nmcli(&self) -> String {
        if host_tools_blocked() {
            return "safe-mode: host network manager inspection disabled\n".to_string();
        }
        if cfg!(target_os = "linux") {
            let mut cmd = Command::new("nmcli");
            cmd.args(["-t", "-f", "NAME,TYPE,DEVICE,STATE", "connection", "show", "--active"]);
            command_text(cmd, SHORT_CMD_TIMEOUT, "nmcli unavailable")
        } else {
            self.iwconfig()
        }
    }

    fn refresh_linux(&mut self) {
        let mut by_name: BTreeMap<String, NetInterface> = BTreeMap::new();
        let mut cmd = Command::new("ip");
        cmd.args(["-o", "addr", "show"]);
        let Ok(output) = run_with_timeout(cmd, SHORT_CMD_TIMEOUT) else {
            return;
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() < 4 || !matches!(parts[2], "inet" | "inet6") {
                continue;
            }
            let name = parts[1].trim_end_matches(':').to_string();
            let entry = by_name.entry(name.clone()).or_insert_with(|| NetInterface {
                mac: read_linux_mac(&name),
                name: name.clone(),
                ip: "0.0.0.0".to_string(),
                netmask: "0.0.0.0".to_string(),
                status: linux_operstate(&name),
                wifi_ssid: None,
                wifi_signal: None,
                frequency: None,
            });

            if parts[2] == "inet" && entry.ip == "0.0.0.0" {
                let (ip, prefix) = parts[3].split_once('/').unwrap_or((parts[3], "24"));
                entry.ip = ip.to_string();
                entry.netmask = prefix_to_netmask(prefix.parse().unwrap_or(24));
            }
        }

        if let Some((device, ssid, signal, frequency)) = linux_wifi_info() {
            if let Some(iface) = by_name.get_mut(&device) {
                iface.wifi_ssid = Some(ssid);
                iface.wifi_signal = signal;
                iface.frequency = frequency;
            } else if let Some(iface) = by_name
                .values_mut()
                .find(|iface| iface.name.starts_with("wl") || iface.name.starts_with("wlan"))
            {
                iface.wifi_ssid = Some(ssid);
                iface.wifi_signal = signal;
                iface.frequency = frequency;
            }
        }

        self.interfaces.extend(by_name.into_values());
    }

    fn refresh_macos(&mut self) {
        let cmd = Command::new("ifconfig");
        let Ok(output) = run_with_timeout(cmd, SHORT_CMD_TIMEOUT) else {
            return;
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let wifi_device = macos_wifi_device();
        let wifi_ssid = wifi_device.as_ref().and_then(|device| macos_current_ssid(device));
        let mut current: Option<NetInterface> = None;

        for line in stdout.lines() {
            if !line.starts_with('\t') && line.contains(':') {
                if let Some(iface) = current.take() {
                    self.interfaces.push(iface);
                }
                let name = line.split(':').next().unwrap_or("unknown").to_string();
                current = Some(NetInterface {
                    name,
                    mac: "unknown".to_string(),
                    ip: "0.0.0.0".to_string(),
                    netmask: "0.0.0.0".to_string(),
                    status: if line.contains("UP") { "up" } else { "down" }.to_string(),
                    wifi_ssid: None,
                    wifi_signal: None,
                    frequency: None,
                });
            } else if let Some(iface) = current.as_mut() {
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields.first() == Some(&"inet") && fields.len() > 1 {
                    iface.ip = fields[1].to_string();
                    if let Some(idx) = fields.iter().position(|field| *field == "netmask") {
                        if let Some(raw_mask) = fields.get(idx + 1) {
                            iface.netmask = macos_hex_netmask(raw_mask);
                        }
                    }
                } else if fields.first() == Some(&"ether") && fields.len() > 1 {
                    iface.mac = fields[1].to_string();
                }
            }
        }
        if let Some(iface) = current.take() {
            self.interfaces.push(iface);
        }

        if let (Some(device), Some(ssid)) = (wifi_device, wifi_ssid) {
            if let Some(iface) = self.interfaces.iter_mut().find(|iface| iface.name == device) {
                iface.wifi_ssid = Some(ssid);
            }
        }
    }
}

impl Default for NetworkStack {
    fn default() -> Self {
        Self::new()
    }
}

fn loopback() -> NetInterface {
    NetInterface {
        name: "lo".to_string(),
        mac: "00:00:00:00:00:00".to_string(),
        ip: "127.0.0.1".to_string(),
        netmask: "255.0.0.0".to_string(),
        status: "up".to_string(),
        wifi_ssid: None,
        wifi_signal: None,
        frequency: None,
    }
}

fn read_linux_mac(name: &str) -> String {
    fs::read_to_string(format!("/sys/class/net/{name}/address"))
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

fn linux_operstate(name: &str) -> String {
    fs::read_to_string(format!("/sys/class/net/{name}/operstate"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "up".to_string())
}

fn linux_wifi_info() -> Option<(String, String, Option<i32>, Option<f32>)> {
    if command_exists("nmcli") {
        let mut cmd = Command::new("nmcli");
        cmd.args(["-t", "-f", "DEVICE,TYPE,STATE,CONNECTION", "dev", "status"]);
        if let Ok(output) = run_with_timeout(cmd, SHORT_CMD_TIMEOUT) {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let fields = split_nmcli_line(line);
                if fields.len() >= 4 && fields[1] == "wifi" && fields[2] == "connected" {
                    let ssid = fields[3].trim().to_string();
                    if !ssid.is_empty() && ssid != "--" {
                        return Some((fields[0].to_string(), ssid, None, None));
                    }
                }
            }
        }
    }

    let mut cmd = Command::new("iwgetid");
    cmd.arg("-r");
    run_with_timeout(cmd, Duration::from_secs(2))
        .ok()
        .and_then(|output| {
            let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            (!ssid.is_empty()).then_some(("".to_string(), ssid, None, None))
        })
}

fn split_nmcli_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut escaped = false;
    for ch in line.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == ':' {
            fields.push(std::mem::take(&mut current));
        } else {
            current.push(ch);
        }
    }
    fields.push(current);
    fields
}

fn macos_wifi_scan() -> String {
    let airport =
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport";
    if Path::new(airport).exists() {
        let mut cmd = Command::new(airport);
        cmd.arg("-s");
        let output = command_text(cmd, WIFI_SCAN_TIMEOUT, "airport scan unavailable");
        if !output.trim().is_empty() && !output.contains("unavailable") {
            return output;
        }
    }

    let mut out = String::from("wifi-scan: macOS nearby scan backend unavailable\n");
    out.push_str(
        "note: Apple removed or restricts the legacy airport scanner on newer macOS releases.\n\n",
    );

    if Path::new("/usr/bin/wdutil").exists() {
        let mut cmd = Command::new("/usr/bin/wdutil");
        cmd.arg("info");
        let wdutil = command_text(cmd, SHORT_CMD_TIMEOUT, "wdutil info unavailable");
        if !wdutil.trim().is_empty() {
            out.push_str("[wdutil info]\n");
            out.push_str(&wdutil);
            if !wdutil.ends_with('\n') {
                out.push('\n');
            }
            out.push('\n');
        }
    }

    let device = macos_wifi_device().unwrap_or_else(|| "en0".to_string());
    let mut current = Command::new("networksetup");
    current.arg("-getairportnetwork").arg(&device);
    out.push_str(&format!("[current network: {device}]\n"));
    out.push_str(&command_text(
        current,
        SHORT_CMD_TIMEOUT,
        "current WiFi network unavailable",
    ));
    out.push('\n');

    let mut preferred = Command::new("networksetup");
    preferred.arg("-listpreferredwirelessnetworks").arg(&device);
    out.push_str(&format!("[saved networks: {device}]\n"));
    out.push_str(&command_text(
        preferred,
        SHORT_CMD_TIMEOUT,
        "saved WiFi networks unavailable",
    ));
    out
}

fn macos_wifi_device() -> Option<String> {
    let mut cmd = Command::new("networksetup");
    cmd.arg("-listallhardwareports");
    let output = run_with_timeout(cmd, SHORT_CMD_TIMEOUT).ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let mut saw_wifi = false;
    for line in text.lines().map(str::trim) {
        if line.starts_with("Hardware Port:") {
            saw_wifi = line.contains("Wi-Fi") || line.contains("AirPort");
            continue;
        }
        if saw_wifi && line.starts_with("Device:") {
            return line
                .split_once(':')
                .map(|(_, value)| value.trim().to_string());
        }
    }
    None
}

fn macos_current_ssid(device: &str) -> Option<String> {
    let mut cmd = Command::new("networksetup");
    cmd.arg("-getairportnetwork").arg(device);
    let output = run_with_timeout(cmd, SHORT_CMD_TIMEOUT).ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    text.split_once(':')
        .map(|(_, ssid)| ssid.trim().to_string())
        .filter(|ssid| !ssid.is_empty() && !ssid.contains("not associated"))
}

fn macos_hex_netmask(raw: &str) -> String {
    let raw = raw.trim_start_matches("0x");
    u32::from_str_radix(raw, 16)
        .map(|mask| {
            format!(
                "{}.{}.{}.{}",
                (mask >> 24) & 0xff,
                (mask >> 16) & 0xff,
                (mask >> 8) & 0xff,
                mask & 0xff
            )
        })
        .unwrap_or_else(|_| raw.to_string())
}

fn prefix_to_netmask(prefix: u8) -> String {
    let prefix = prefix.min(32);
    let mask = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };
    format!(
        "{}.{}.{}.{}",
        (mask >> 24) & 0xff,
        (mask >> 16) & 0xff,
        (mask >> 8) & 0xff,
        mask & 0xff
    )
}

fn safe_host(host: &str) -> bool {
    let host = host.trim();
    if host.is_empty()
        || host.len() > 255
        || host.starts_with('-')
        || host.contains("..")
        || host.chars().any(char::is_whitespace)
    {
        return false;
    }
    if host
        .chars()
        .all(|ch| ch.is_ascii_hexdigit() || matches!(ch, ':' | '.'))
        && host.contains(':')
    {
        return true;
    }
    host.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
    })
}

fn safe_ssid(ssid: &str) -> bool {
    let bytes = ssid.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 32
        && !ssid.chars().any(|ch| ch.is_control() && ch != '\t')
}

fn sanitize_inline(text: &str) -> String {
    text.chars()
        .filter(|ch| !ch.is_control())
        .take(96)
        .collect::<String>()
}

fn host_tools_blocked() -> bool {
    !crate::policy::host_tools_allowed()
}

fn command_exists(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn command_text(cmd: Command, timeout: Duration, fallback: &str) -> String {
    match run_with_timeout(cmd, timeout) {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() {
                format!("{fallback}\n")
            } else {
                bounded_output(&stdout)
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.trim().is_empty() {
                format!("{fallback}\n")
            } else {
                bounded_output(&stderr)
            }
        }
        Err(err) => format!("{fallback}: {err}\n"),
    }
}

fn bounded_output(raw: &str) -> String {
    if raw.len() <= MAX_COMMAND_OUTPUT {
        return raw.to_string();
    }
    let mut out = raw.chars().take(MAX_COMMAND_OUTPUT).collect::<String>();
    out.push_str("\n...[truncated by phase1 network]\n");
    out
}

fn run_with_timeout(mut cmd: Command, timeout: Duration) -> io::Result<Output> {
    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let start = Instant::now();
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::new(io::ErrorKind::TimedOut, "command timed out"));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::{prefix_to_netmask, safe_host, split_nmcli_line, NetworkStack};

    #[test]
    fn prefix_to_netmask_handles_common_prefixes() {
        assert_eq!(prefix_to_netmask(24), "255.255.255.0");
        assert_eq!(prefix_to_netmask(0), "0.0.0.0");
    }

    #[test]
    fn safe_host_rejects_shell_metacharacters() {
        assert!(safe_host("example.com"));
        assert!(safe_host("localhost"));
        assert!(safe_host("2001:db8::1"));
        assert!(!safe_host("example.com;rm-rf"));
        assert!(!safe_host("-bad.example"));
        assert!(!safe_host("bad..example"));
    }

    #[test]
    fn nmcli_split_handles_escaped_colons() {
        let fields = split_nmcli_line("wlan0:wifi:connected:Lab\\:Network");
        assert_eq!(fields[0], "wlan0");
        assert_eq!(fields[3], "Lab:Network");
    }

    #[test]
    fn safe_mode_uses_loopback_only() {
        std::env::set_var("PHASE1_SAFE_MODE", "1");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        let network = NetworkStack::new();
        let output = network.ifconfig();
        assert!(output.contains("lo:"));
        assert!(output.contains("safe-mode: host network inspection disabled"));
        std::env::remove_var("PHASE1_SAFE_MODE");
    }

    #[test]
    fn safe_off_still_requires_host_tools_opt_in() {
        std::env::set_var("PHASE1_SAFE_MODE", "0");
        std::env::remove_var("PHASE1_ALLOW_HOST_TOOLS");
        let network = NetworkStack::new();
        let output = network.ifconfig();
        assert!(output.contains("lo:"));
        assert!(output.contains("host network inspection disabled"));
        std::env::remove_var("PHASE1_SAFE_MODE");
    }
}
