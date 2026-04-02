# Фаза 2: Backend (Rust)

## Структура модулей

```
src-tauri/src/
  main.rs              — Tauri entry, регистрация команд
  lib.rs               — mod declarations
  error.rs             — VortexError (thiserror)
  state.rs             — AppState (Arc<Mutex<>>)
  commands.rs          — #[tauri::command] функции
  tray.rs              — Системный трей
  core/
    mod.rs             — CoreType enum, CoreManager
    mihomo.rs          — Mihomo lifecycle + REST API клиент
    xray.rs            — Xray lifecycle (config rewrite + restart)
    process.rs         — Обёртка для child process с stderr capture
  config/
    mod.rs             — ConfigGenerator trait (strategy pattern)
    mihomo_gen.rs      — Генерация Mihomo YAML
    xray_gen.rs        — Генерация Xray JSON
    models.rs          — ServerNode, ProxyGroup, UserSettings и др.
  subscription/
    mod.rs             — SubscriptionManager: fetch, parse, store
    parser.rs          — Парсинг Base64, Clash YAML, SIP008, Xray JSON array
    hwid.rs            — HWID через machineid-rs
    headers.rs         — Парсинг response headers (traffic/expiry)
  system/
    mod.rs             — Platform detection
    tun.rs             — TUN mode (Wintun / ip route)
    proxy.rs           — Системный прокси (реестр Win / gsettings Linux)
```

---

## error.rs — Единый тип ошибок

```rust
#[derive(Debug, thiserror::Error)]
pub enum VortexError {
    #[error("Core start failed: {0}")]
    CoreStartFailed(String),
    #[error("Core not running")]
    CoreNotRunning,
    #[error("Config generation failed: {0}")]
    ConfigGenerationFailed(String),
    #[error("Subscription fetch failed: {0}")]
    SubscriptionFetchFailed(String),
    #[error("Subscription parse failed: {0}")]
    SubscriptionParseFailed(String),
    #[error("HWID generation failed: {0}")]
    HwidGenerationFailed(String),
    #[error("TUN setup failed: {0}")]
    TunSetupFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
// + impl Into<tauri::InvokeError>
```

---

## core/process.rs — Обёртка для процессов

```rust
pub struct CoreProcess {
    child: Option<Child>,
    config_path: PathBuf,
    binary_path: PathBuf,
    stderr_lines: Arc<Mutex<VecDeque<String>>>,  // ring buffer, 1000 строк
}
```

- `start()` → spawn с аргументами, tokio task для чтения stderr
- `stop()` → SIGTERM, wait 5s, SIGKILL
- `restart()` → stop + start
- `is_running()` → проверка child
- **Drop** → kill child (safety: no orphaned processes)

---

## core/mihomo.rs

```rust
pub struct MihomoCore {
    process: CoreProcess,
    api_base: String,       // http://127.0.0.1:9090
    api_secret: String,     // Bearer token
    client: reqwest::Client,
}
```

**REST API методы:**
- `get_proxies()` → GET /proxies
- `select_proxy(group, name)` → PUT /proxies/{group} body: {"name": "..."}
- `get_proxy_delay(name, url, timeout)` → GET /proxies/{name}/delay?url=...&timeout=...
- `get_traffic()` → GET /traffic (single) или WebSocket
- `get_connections()` → GET /connections
- `patch_config(patch)` → PATCH /configs
- `set_mode(mode)` → PATCH /configs {"mode": "rule"|"global"}
- `get_logs_ws()` → WebSocket /logs

**Запуск:** Записать конфиг → spawn `mihomo -d <config_dir>`

---

## core/xray.rs

```rust
pub struct XrayCore {
    process: CoreProcess,
}
```

- Нет REST API — все изменения через: сгенерировать JSON → atomic write → restart
- `apply_config(config)` → serialize → write temp → rename → restart
- **Запуск:** `xray run -config <path>`

---

## config/models.rs — Общие модели

```rust
pub enum ProxyProtocol {
    VMess, VLESS, Trojan, Shadowsocks, Hysteria2, TUIC, WireGuard
}

pub enum TransportType {
    Raw, WebSocket, GRPC, XHTTP, HTTP2, MKCP, HTTPUpgrade
}

pub struct ServerNode {
    pub name: String,
    pub protocol: ProxyProtocol,
    pub address: String,
    pub port: u16,
    pub uuid: Option<String>,        // VMess/VLESS
    pub password: Option<String>,    // Trojan/SS
    pub encryption: Option<String>,
    pub transport: TransportType,
    pub tls: Option<TlsSettings>,
    pub reality: Option<RealitySettings>,
    pub flow: Option<String>,        // VLESS XTLS
    // ... protocol-specific fields
}

pub struct Subscription {
    pub id: String,
    pub name: String,
    pub url: String,
    pub nodes: Vec<ServerNode>,
    pub traffic: Option<TrafficInfo>,
    pub last_updated: Option<DateTime<Utc>>,
    pub auto_update_interval: Option<u64>,
}

pub struct TrafficInfo {
    pub upload: u64,
    pub download: u64,
    pub total: u64,
    pub expire: Option<DateTime<Utc>>,
}
```

---

## config/mihomo_gen.rs — Генератор YAML

Генерирует:
- `mixed-port: 7890`
- `mode: rule` (или global)
- `external-controller: 127.0.0.1:9090` с секретом
- `dns`: nameserver, fallback, enhanced-mode: fake-ip
- `tun` (если включён): stack: mixed, auto-route: true
- `proxies`: массив из ServerNode → формат Mihomo
- `proxy-groups`: автогенерация Selector + URLTest
- `rules`: GEOIP, DOMAIN-SUFFIX, MATCH → proxy group

---

## config/xray_gen.rs — Генератор JSON

Генерирует:
- `inbounds`: SOCKS (10808) + HTTP (10809)
- `outbounds`: выбранный прокси + direct + block
- `routing`: правила (domain, ip, protocol)
- `dns`: серверы с domain routing
- Для Xray JSON подписки: элементы массива — готовые outbound конфиги, вставляются напрямую
- **XHTTP** транспорт: `streamSettings.network: "xhttp"` + `xhttpSettings`

---

## subscription/hwid.rs

```rust
pub fn generate_hwid() -> Result<String, VortexError> {
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::CPUCores);
    builder.build("vortex-vpn-client")
}
```

---

## subscription/parser.rs

**Определение формата:**
- Начинается с `[` или `{` → JSON (Xray array или SIP008)
- Содержит `proxies:` → Clash YAML
- Иначе → Base64 decode → split по `\n` → parse URI

**Парсинг URI:** vmess://, vless://, trojan://, ss://, hy2://, tuic://, wg://

**Xray JSON array:** `Vec<serde_json::Value>` → каждый элемент = outbound config → извлечь protocol/address/port/settings → ServerNode

---

## subscription/headers.rs

**Request headers:**
```
x-hwid: <sha256>
x-device-os: Windows|Linux
x-ver-os: <os_version>
x-device-model: <hostname>
user-agent: Vortex/0.1.0
```

**Response parsing:**
- `subscription-userinfo: upload=X; download=Y; total=Z; expire=T`
- `profile-update-interval: 3600`

---

## system/tun.rs

- **Windows:** Mihomo нативно поддерживает TUN через Wintun DLL. Для Xray — tun2socks
- **Linux:** Mihomo нативно; для Xray — ip route + tun interface
- Проверка привилегий перед включением

## system/proxy.rs

- **Windows:** Реестр `HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings`
- **Linux:** `gsettings` для GNOME, env vars для KDE

---

## commands.rs — Tauri команды

**Core:** `start_core`, `stop_core`, `restart_core`, `get_core_status`, `switch_core`
**Subscriptions:** `add_subscription`, `remove_subscription`, `update_subscription`, `update_all_subscriptions`, `get_subscriptions`
**Mihomo:** `get_proxy_groups`, `select_proxy`, `test_proxy_delay`, `get_connections`, `close_connection`, `close_all_connections`, `set_mode`
**Settings:** `get_settings`, `update_settings`, `toggle_tun`, `toggle_system_proxy`
**System:** `get_hwid`, `get_traffic_info`

---

## state.rs

```rust
pub struct AppState {
    pub core_manager: Arc<Mutex<CoreManager>>,
    pub subscriptions: Arc<Mutex<Vec<Subscription>>>,
    pub settings: Arc<Mutex<UserSettings>>,
    pub hwid: String,  // вычисляется один раз при старте
}
```

Persist: `settings.json` и `subscriptions.json` в app_data_dir.
