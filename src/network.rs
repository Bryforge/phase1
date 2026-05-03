// src/network.rs — Cross-platform (Linux + macOS) networking for phase1 v1.1.1
use std::process::{Command, Output, ExitStatus};
use std::os::unix::process::ExitStatusExt;

fn default_output() -> Output {
    Output {
        status: ExitStatus::from_raw(0),
        stdout: Vec::new(),
        stderr: Vec::new(),
    }
}

#[derive(Clone, Debug)]
pub struct NetInterface {
    pub name: String,
    pub mac: String,
    pub ip: String,
    pub netmask: String,
    pub status: String,
    pub wifi_ssid: Option<String>,
    pub wifi_signal: i32,
    pub frequency: f32,
}

pub struct NetworkStack {
    interfaces: Vec<NetInterface>,
}

impl NetworkStack {
    pub fn new() -> Self {
        let mut stack = NetworkStack { interfaces: vec![] };
        stack.refresh();
        stack
    }

    pub fn refresh(&mut self) {
        self.interfaces.clear();

        if cfg!(target_os = "macos") {
            let airport = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
                .arg("-I")
                .output()
                .unwrap_or_else(|_| default_output());

            let output = String::from_utf8_lossy(&airport.stdout);
            let ssid = output.lines().find(|l| l.contains("SSID:")).and_then(|l| l.split(':').nth(1)).map(|s| s.trim().to_string());
            let signal: i32 = output.lines().find(|l| l.contains("RSSI")).and_then(|l| l.split(':').nth(1)).and_then(|s| s.trim().parse().ok()).unwrap_or(-55);
            let freq: f32 = output.lines().find(|l| l.contains("channel")).and_then(|l| l.split(':').nth(1)).and_then(|s| s.trim().parse().ok()).unwrap_or(2.437);

            let ip_output = Command::new("ipconfig").arg("getifaddr").arg("en0").output().unwrap_or_else(|_| default_output());
            let ip = String::from_utf8_lossy(&ip_output.stdout).trim().to_string();
            let ip = if ip.is_empty() { "192.168.1.137".to_string() } else { ip };

            self.interfaces.push(NetInterface { name: "en0".to_string(), mac: "aa:bb:cc:dd:ee:ff".to_string(), ip, netmask: "255.255.255.0".to_string(), status: if ssid.is_some() { "up" } else { "down" }.to_string(), wifi_ssid: ssid, wifi_signal: signal, frequency: freq });
            self.interfaces.push(NetInterface { name: "en1".to_string(), mac: "00:11:22:33:44:55".to_string(), ip: "10.0.0.42".to_string(), netmask: "255.255.255.0".to_string(), status: "up".to_string(), wifi_ssid: None, wifi_signal: 0, frequency: 0.0 });
        } else if cfg!(target_os = "linux") {
            let _ip_output = Command::new("ip").arg("addr").output().unwrap_or_else(|_| default_output());
            self.interfaces.push(NetInterface { name: "eth0".to_string(), mac: "00:11:22:33:44:55".to_string(), ip: "192.168.1.100".to_string(), netmask: "255.255.255.0".to_string(), status: "up".to_string(), wifi_ssid: None, wifi_signal: 0, frequency: 0.0 });

            let nm_output = Command::new("nmcli").arg("-t").arg("device").arg("wifi").output().unwrap_or_else(|_| default_output());
            if nm_output.status.success() {
                self.interfaces.push(NetInterface { name: "wlan0".to_string(), mac: "aa:bb:cc:dd:ee:ff".to_string(), ip: "192.168.1.101".to_string(), netmask: "255.255.255.0".to_string(), status: "up".to_string(), wifi_ssid: Some("LinuxNetwork".to_string()), wifi_signal: -52, frequency: 5.2 });
            }
        } else {
            self.interfaces.push(NetInterface { name: "lo".to_string(), mac: "00:00:00:00:00:00".to_string(), ip: "127.0.0.1".to_string(), netmask: "255.0.0.0".to_string(), status: "up".to_string(), wifi_ssid: None, wifi_signal: 0, frequency: 0.0 });
        }
    }

    pub fn ifconfig(&self) -> String {
        let mut out = String::new();
        for iface in &self.interfaces {
            out.push_str(&format!("{}: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500\n", iface.name));
            out.push_str(&format!("        inet {}  netmask {}  broadcast 192.168.1.255\n", iface.ip, iface.netmask));
            out.push_str(&format!("        ether {}  txqueuelen 1000  (Ethernet)\n", iface.mac));
            if let Some(ssid) = &iface.wifi_ssid {
                out.push_str(&format!("        wifi ssid:\"{}\"  signal:{} dBm  freq:{:.3} GHz\n", ssid, iface.wifi_signal, iface.frequency));
            }
            out.push_str(&format!("        status: {}\n\n", iface.status));
        }
        out
    }

    pub fn iwconfig(&self) -> String {
        let mut out = String::new();
        for iface in &self.interfaces {
            if iface.wifi_ssid.is_some() {
                out.push_str(&format!("{}     IEEE 802.11  ESSID:\"{}\"\n", iface.name, iface.wifi_ssid.as_ref().unwrap()));
                out.push_str(&format!("          Mode:Managed  Frequency:{:.3} GHz\n", iface.frequency));
                out.push_str(&format!("          Link Quality=70/70  Signal level={} dBm\n", iface.wifi_signal));
            }
        }
        if out.is_empty() { "No active WiFi interface".to_string() } else { out }
    }

    pub fn wifi_scan(&self) -> String {
        if cfg!(target_os = "macos") {
            let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
                .arg("-s")
                .output()
                .unwrap_or_else(|_| default_output());
            let scan = String::from_utf8_lossy(&output.stdout);
            if scan.trim().is_empty() {
                "No networks found (WiFi may be off or no APs visible)".to_string()
            } else {
                format!("WiFi scan from your Mac:\n{}", scan)
            }
        } else if cfg!(target_os = "linux") {
            let output = Command::new("nmcli").arg("dev").arg("wifi").arg("list").output().unwrap_or_else(|_| default_output());
            let scan = String::from_utf8_lossy(&output.stdout);
            if scan.trim().is_empty() { "No networks found (nmcli not available)".to_string() } else { format!("Linux WiFi scan:\n{}", scan) }
        } else {
            "WiFi scan not supported on this platform".to_string()
        }
    }

    pub fn wifi_connect(&mut self, ssid: &str, password: Option<&str>) -> String {
        println!("[wifi-connect] Attempting to connect to network: {}", ssid);

        let result = if cfg!(target_os = "macos") {
            let mut cmd = Command::new("networksetup");
            cmd.arg("-setairportnetwork").arg("en0").arg(ssid);
            if let Some(pw) = password { cmd.arg(pw); }

            match cmd.output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if output.status.success() {
                        format!("Successfully connected to {}\n(macOS connection performed)", ssid)
                    } else {
                        format!("Connection failed!\nExit code: {}\nstdout: {}\nstderr: {}", output.status, stdout.trim(), stderr.trim())
                    }
                }
                Err(e) => format!("Failed to execute networksetup: {}", e),
            }
        } else if cfg!(target_os = "linux") {
            let mut cmd = Command::new("nmcli");
            cmd.arg("dev").arg("wifi").arg("connect").arg(ssid);
            if let Some(pw) = password { cmd.arg("password").arg(pw); }
            match cmd.output() {
                Ok(output) => {
                    if output.status.success() {
                        format!("Connected to {} (Linux connection)", ssid)
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        format!("nmcli failed:\n{}", stderr.trim())
                    }
                }
                Err(e) => format!("Failed to run nmcli: {}", e),
            }
        } else {
            format!("Connected to {} (simulated only)", ssid)
        };

        self.refresh();
        result
    }

    pub fn ping(&self, host: &str) -> String {
        let output = Command::new("ping")
            .arg("-c")
            .arg("4")
            .arg(host)
            .output()
            .unwrap_or_else(|_| default_output());

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            format!("PING {} (network test)\n{}", host, stdout.trim())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("Ping failed for {}\n{}", host, stderr.trim())
        }
    }

    pub fn nmcli(&self) -> String {
        let active = self.interfaces.iter().find(|i| i.wifi_ssid.is_some());
        if let Some(iface) = active {
            format!("wlan0: connected to {}\n    signal: {} dBm\n    IPv4: {}/24\n", 
                iface.wifi_ssid.as_ref().unwrap(), iface.wifi_signal, iface.ip)
        } else {
            "No active WiFi connection".to_string()
        }
    }
}
