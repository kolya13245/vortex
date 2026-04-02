use crate::state::{ServerNode, UserSettings};
use serde_json::Value;
use std::path::Path;

/// Generate Mihomo YAML config from nodes and settings
pub fn generate(nodes: &[ServerNode], settings: &UserSettings, config_dir: &Path) -> String {
    let mut proxies = Vec::new();
    let mut proxy_names = Vec::new();

    for node in nodes {
        let proxy_yaml = node_to_mihomo_proxy(node);
        if !proxy_yaml.is_empty() {
            proxies.push(proxy_yaml);
            proxy_names.push(node.name.clone());
        }
    }

    let names_yaml: String = proxy_names
        .iter()
        .map(|n| format!("      - \"{}\"", n.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join("\n");

    let dns_servers: String = settings
        .dns_servers
        .iter()
        .map(|s| format!("    - {}", s))
        .collect::<Vec<_>>()
        .join("\n");

    let proxies_yaml = proxies.join("\n");

    format!(
        r#"# Vortex - Auto-generated Mihomo config
mixed-port: {mixed_port}
allow-lan: {allow_lan}
mode: rule
log-level: info
ipv6: false
external-controller: 127.0.0.1:{api_port}
secret: "{api_secret}"
geodata-mode: false

dns:
  enable: true
  listen: 0.0.0.0:53
  enhanced-mode: {dns_mode}
  fake-ip-range: 198.18.0.1/16
  default-nameserver:
    - 8.8.8.8
    - 1.1.1.1
  nameserver:
{dns_servers}

{tun_section}

proxies:
{proxies_yaml}

proxy-groups:
  - name: "Proxy"
    type: select
    proxies:
      - "Auto"
{names_yaml}
      - DIRECT

  - name: "Auto"
    type: url-test
    proxies:
{names_yaml}
    url: "http://www.gstatic.com/generate_204"
    interval: 300
    tolerance: 50

rules:
  - GEOIP,private,DIRECT,no-resolve
  - GEOSITE,category-ads-all,REJECT
  - GEOIP,cn,DIRECT
  - GEOSITE,cn,DIRECT
  - MATCH,Proxy
"#,
        mixed_port = settings.mixed_port,
        allow_lan = settings.allow_lan,
        api_port = settings.mihomo_api_port,
        api_secret = settings.mihomo_api_secret,
        dns_mode = settings.dns_mode,
        dns_servers = dns_servers,
        proxies_yaml = proxies_yaml,
        names_yaml = names_yaml,
        tun_section = if settings.tun_enabled {
            "tun:\n  enable: true\n  stack: mixed\n  auto-route: true\n  auto-detect-interface: true"
        } else {
            ""
        },
    )
}

fn node_to_mihomo_proxy(node: &ServerNode) -> String {
    let s = &node.settings;
    let name = node.name.replace('"', "\\\"");

    match node.protocol.as_str() {
        "vmess" => {
            let uuid = s.get("id").or(s.get("uuid"))
                .and_then(|v| v.as_str()).unwrap_or("");
            let aid = s.get("aid").and_then(|v| v.as_u64()).unwrap_or(0);
            let net = s.get("net").or(s.get("type"))
                .and_then(|v| v.as_str()).unwrap_or("tcp");
            let tls = s.get("tls").and_then(|v| v.as_str()).unwrap_or("");
            let sni = s.get("sni").and_then(|v| v.as_str()).unwrap_or("");
            let path = s.get("path").and_then(|v| v.as_str()).unwrap_or("");
            let host = s.get("host").and_then(|v| v.as_str()).unwrap_or("");

            let mut yaml = format!(
                "  - name: \"{name}\"\n    type: vmess\n    server: {}\n    port: {}\n    uuid: {}\n    alterId: {}\n    cipher: auto",
                node.address, node.port, uuid, aid
            );
            if net != "tcp" {
                yaml.push_str(&format!("\n    network: {}", net));
            }
            if tls == "tls" {
                yaml.push_str("\n    tls: true");
                if !sni.is_empty() {
                    yaml.push_str(&format!("\n    servername: {}", sni));
                }
            }
            if net == "ws" {
                yaml.push_str("\n    ws-opts:");
                if !path.is_empty() {
                    yaml.push_str(&format!("\n      path: {}", path));
                }
                if !host.is_empty() {
                    yaml.push_str(&format!("\n      headers:\n        Host: {}", host));
                }
            }
            yaml
        }
        "vless" => {
            let uuid = s.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
            let flow = s.get("flow").and_then(|v| v.as_str()).unwrap_or("");
            let security = s.get("security").and_then(|v| v.as_str()).unwrap_or("none");
            let sni = s.get("sni").and_then(|v| v.as_str()).unwrap_or("");
            let net_type = s.get("type").and_then(|v| v.as_str()).unwrap_or("tcp");
            let fp = s.get("fp").and_then(|v| v.as_str()).unwrap_or("");
            let pbk = s.get("pbk").and_then(|v| v.as_str()).unwrap_or("");
            let sid = s.get("sid").and_then(|v| v.as_str()).unwrap_or("");

            let mut yaml = format!(
                "  - name: \"{name}\"\n    type: vless\n    server: {}\n    port: {}\n    uuid: {}",
                node.address, node.port, uuid
            );
            if !flow.is_empty() {
                yaml.push_str(&format!("\n    flow: {}", flow));
            }
            if net_type != "tcp" {
                yaml.push_str(&format!("\n    network: {}", net_type));
            }
            if security == "tls" {
                yaml.push_str("\n    tls: true");
                if !sni.is_empty() {
                    yaml.push_str(&format!("\n    servername: {}", sni));
                }
                if !fp.is_empty() {
                    yaml.push_str(&format!("\n    client-fingerprint: {}", fp));
                }
            } else if security == "reality" {
                yaml.push_str("\n    tls: true");
                if !sni.is_empty() {
                    yaml.push_str(&format!("\n    servername: {}", sni));
                }
                yaml.push_str("\n    reality-opts:");
                if !pbk.is_empty() {
                    yaml.push_str(&format!("\n      public-key: {}", pbk));
                }
                if !sid.is_empty() {
                    yaml.push_str(&format!("\n      short-id: {}", sid));
                }
                if !fp.is_empty() {
                    yaml.push_str(&format!("\n    client-fingerprint: {}", fp));
                }
            }
            yaml
        }
        "trojan" => {
            let password = s.get("password").and_then(|v| v.as_str()).unwrap_or("");
            let sni = s.get("sni").and_then(|v| v.as_str()).unwrap_or("");

            let mut yaml = format!(
                "  - name: \"{name}\"\n    type: trojan\n    server: {}\n    port: {}\n    password: {}",
                node.address, node.port, password
            );
            if !sni.is_empty() {
                yaml.push_str(&format!("\n    sni: {}", sni));
            }
            yaml
        }
        "shadowsocks" | "ss" => {
            let method = s.get("method").or(s.get("cipher"))
                .and_then(|v| v.as_str()).unwrap_or("aes-256-gcm");
            let password = s.get("password").and_then(|v| v.as_str()).unwrap_or("");

            format!(
                "  - name: \"{name}\"\n    type: ss\n    server: {}\n    port: {}\n    cipher: {}\n    password: \"{}\"",
                node.address, node.port, method, password
            )
        }
        _ => String::new(),
    }
}
