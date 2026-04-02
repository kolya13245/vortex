use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoreType {
    Mihomo,
    Xray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub core_type: CoreType,
    pub tun_enabled: bool,
    pub system_proxy: bool,
    pub mixed_port: u16,
    pub allow_lan: bool,
    pub theme: String,
    pub language: String,
    pub mihomo_api_port: u16,
    pub mihomo_api_secret: String,
    pub dns_mode: String,
    pub dns_servers: Vec<String>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            core_type: CoreType::Mihomo,
            tun_enabled: false,
            system_proxy: false,
            mixed_port: 7890,
            allow_lan: false,
            theme: "dark".to_string(),
            language: "en".to_string(),
            mihomo_api_port: 9090,
            mihomo_api_secret: uuid::Uuid::new_v4().to_string(),
            dns_mode: "fake-ip".to_string(),
            dns_servers: vec![
                "8.8.8.8".to_string(),
                "1.1.1.1".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub url: String,
    pub core_type: CoreType,
    pub node_count: usize,
    pub nodes: Vec<ServerNode>,
    pub traffic_used: Option<u64>,
    pub traffic_total: Option<u64>,
    pub expire: Option<String>,
    pub last_updated: Option<String>,
    /// Raw body for Xray JSON configs that should be used as-is
    pub raw_config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficInfo {
    pub upload: u64,
    pub download: u64,
    pub total: u64,
    pub expire: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNode {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: u16,
    /// Full JSON for Xray outbound config (used as-is)
    pub raw_outbound: Option<serde_json::Value>,
    /// Protocol-specific settings as generic JSON
    pub settings: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoreStatus {
    pub core_type: CoreType,
    pub connection_status: ConnectionStatus,
    pub uptime_secs: u64,
    pub current_proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrafficStats {
    pub download_speed: u64,
    pub upload_speed: u64,
    pub download_total: u64,
    pub upload_total: u64,
}

pub struct AppState {
    pub settings: Mutex<UserSettings>,
    pub subscriptions: Mutex<Vec<Subscription>>,
    pub connection_status: Mutex<ConnectionStatus>,
    pub core_process: Mutex<Option<std::process::Child>>,
    pub hwid: String,
    pub data_dir: std::path::PathBuf,
}

impl AppState {
    pub fn new() -> Self {
        let hwid = generate_hwid();
        let data_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("vortex");
        std::fs::create_dir_all(&data_dir).ok();
        std::fs::create_dir_all(data_dir.join("configs")).ok();
        std::fs::create_dir_all(data_dir.join("bin")).ok();

        Self {
            settings: Mutex::new(UserSettings::default()),
            subscriptions: Mutex::new(Vec::new()),
            connection_status: Mutex::new(ConnectionStatus::Disconnected),
            core_process: Mutex::new(None),
            hwid,
            data_dir,
        }
    }
}

fn generate_hwid() -> String {
    use sha2::{Digest, Sha256};
    use std::env;

    let mut hasher = Sha256::new();

    // Machine-specific data that persists across reinstalls
    if let Ok(hostname) = env::var("HOSTNAME") {
        hasher.update(hostname.as_bytes());
    }
    if let Ok(user) = env::var("USER").or_else(|_| env::var("USERNAME")) {
        hasher.update(user.as_bytes());
    }

    // Linux: machine-id is stable across reinstalls
    #[cfg(target_os = "linux")]
    {
        if let Ok(id) = std::fs::read_to_string("/etc/machine-id") {
            hasher.update(id.trim().as_bytes());
        }
    }

    // Windows: MachineGuid from registry
    #[cfg(target_os = "windows")]
    {
        // Fallback: use computer name + OS info
        if let Ok(name) = env::var("COMPUTERNAME") {
            hasher.update(name.as_bytes());
        }
    }

    hasher.update(b"vortex-vpn-client-salt");

    let result = hasher.finalize();
    hex::encode(&result[..16]) // 32-char hex string
}
