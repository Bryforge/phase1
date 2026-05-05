use std::fs;
use std::io;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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
}

impl NetworkStack {
    pub fn new() -> Self {
        let mut stack = Self { interfaces: Vec::new() };
        stack.refresh();
        stack
    }

    pub fn refresh(&mut self) {
        self.interfaces.clear();
        if cfg!(target_os = "linux") {
            self.refresh_linux();
        } else if cfg!(target_os = "macos") {
            self.refresh_macos();
        }
        if self.interfaces.is_empty() {
            self.interfaces.push(loopback());
        }
    }

    pub fn ifconfig(&self) -> String {
        let mut out = String::new();
        for iface in &self.interfaces {
            out.push_str(&format!("{}: flags=<{}> mtu 1500\n", iface.name, iface.status.to_ascii_uppercase()));
            out.push_str(&format!("    inet {} netmask {}\n", iface.ip, iface.netmask));
            out.push_str(&format!("    ether {}\n", iface.mac));
            if let Some(ssid) = &iface.wifi_ssid {
                out.push_str(&format!("    wifi ssid=\"{}\"", ssid));
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
        out
    }

    pub fn iwconfig(&self) -> String {
        let mut out = String::new();
        for iface in &self.interfaces {
            if let Some(ssid) = &iface.wifi_ssid {
                out.push_str(&format!("{} IEEE 802.11 ESSID=\"{}\"\n", iface.name, ssid));
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
        if cfg!(target_os = "macos") {
            let mut cmd = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport");
            cmd.arg("-s");
            return command_text(cmd, Duration::from_secs(8), "airport scan unavailable");
        }
        if cfg!(target_os = "linux") {
            let mut cmd = Command::new("nmcli");
            cmd.args(["dev", "wifi", "list"]);
            return command_text(cmd, Duration::from_secs(8), "nmcli WiFi scan unavailable");
        }
        "wifi-scan: unsupported platform\n".to_string()
    }

    pub fn wifi_connect(&mut self, ssid: &str, password: Option<&str>) -> String {
        if ssid.trim().is_empty() {
            return "usage: wifi-connect <ssid> [password]".to_string();
        }
        if std::env::var("PHASE1_ALLOW_HOST_NETWORK_CHANGES").ok().as_deref() != Some("1") {
            return format!("dry-run: would connect to '{ssid}'. Set PHASE1_ALLOW_HOST_NETWORK_CHANGES=1 to allow host network changes.");
        }

        let result = if cfg!(target_os = "macos") {
            let mut cmd = Command::new("networksetup");
            cmd.arg("-setairportnetwork").arg("en0").arg(ssid);
            if let Some(password) = password {
                cmd.arg(password);
            }
            command_text(cmd, Duration::from_secs(15), "networksetup unavailable")
        } else if cfg!(target_os = "linux") {
            let mut cmd = Command::new("nmcli");
            cmd.args(["dev", "wifi", "connect", ssid]);
            if let Some(password) = password {
                cmd.arg("password").arg(password);
            }
            command_text(cmd, Duration::from_secs(15), "nmcli unavailable")
        } else {
            format!("connected to {ssid} (simulated)\n")
        };
        self.refresh();
        result
    }

    pub fn ping(&self, host: &str) -> String {
        if !safe_host(host) {
            return "ping: invalid host\n".to_string();
        }
        let mut cmd = Command::new("ping");
        cmd.args(["-c", "4", host]);
        command_text(cmd, Duration::from_secs(8), "ping unavailable")
    }

    pub fn nmcli(&self) -> String {
        if cfg!(target_os = "linux") {
            let mut cmd = Command::new("nmcli");
            cmd.args(["-t", "connection", "show", "--active"]);
            command_text(cmd, Duration::from_secs(5), "nmcli unavailable")
        } else {
            self.iwconfig()
        }
    }

    fn refresh_linux(&mut self) {
        let mut cmd = Command::new("ip");
        cmd.args(["-o", "-4", "addr", "show"]);
        let Ok(output) = run_with_timeout(cmd, Duration::from_secs(3)) else {
            return;
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() < 4 || parts[2] != "inet" {
                continue;
            }
            let name = parts[1].trim_end_matches(':').to_string();
            let (ip, prefix) = parts[3].split_once('/').unwrap_or((parts[3], "24"));
            self.interfaces.push(NetInterface {
                mac: read_linux_mac(&name),
                name,
                ip: ip.to_string(),
                netmask: prefix_to_netmask(prefix.parse().unwrap_or(24)),
                status: "up".to_string(),
                wifi_ssid: None,
                wifi_signal: None,
                frequency: None,
            });
        }
        if let Some(ssid) = linux_wifi_ssid() {
            if let Some(iface) = self.interfaces.iter_mut().find(|iface| iface.name.starts_with("wl")) {
                iface.wifi_ssid = Some(ssid);
            }
        }
    }

    fn refresh_macos(&mut self) {
        let mut cmd = Command::new("ifconfig");
        let Ok(output) = run_with_timeout(cmd, Duration::from_secs(3)) else {
            return;
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
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
                } else if fields.first() == Some(&"ether") && fields.len() > 1 {
                    iface.mac = fields[1].to_string();
                }
            }
        }
        if let Some(iface) = current.take() {
            self.interfaces.push(iface);
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
    fs::read_to_string(format!("/sys/class/net/{name}/address")).map(|value| value.trim().to_string()).unwrap_or_else(|_| "unknown".to_string())
}

fn linux_wifi_ssid() -> Option<String> {
    let mut cmd = Command::new("iwgetid");
    cmd.arg("-r");
    run_with_timeout(cmd, Duration::from_secs(2)).ok().and_then(|output| {
        let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        (!ssid.is_empty()).then_some(ssid)
    })
}

fn prefix_to_netmask(prefix: u8) -> String {
    let prefix = prefix.min(32);
    let mask = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix) };
    format!("{}.{}.{}.{}", (mask >> 24) & 0xff, (mask >> 16) & 0xff, (mask >> 8) & 0xff, mask & 0xff)
}

fn safe_host(host: &str) -> bool {
    !host.is_empty() && host.len() <= 255 && host.chars().all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | ':' | '_'))
}

fn command_text(cmd: Command, timeout: Duration, fallback: &str) -> String {
    match run_with_timeout(cmd, timeout) {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.trim().is_empty() { format!("{fallback}\n") } else { stdout.to_string() }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.trim().is_empty() { format!("{fallback}\n") } else { stderr.to_string() }
        }
        Err(err) => format!("{fallback}: {err}\n"),
    }
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
        thread::sleep(Duration::from_millis(25));
    }
}
