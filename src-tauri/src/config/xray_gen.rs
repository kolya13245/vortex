use crate::state::{ServerNode, UserSettings};
use serde_json::{json, Value};

/// Generate Xray JSON config from nodes and settings
pub fn generate(nodes: &[ServerNode], settings: &UserSettings, raw_config: Option<&str>) -> String {
    // If we have a raw Xray config, use it with modified inbounds
    if let Some(raw) = raw_config {
        if let Ok(mut config) = serde_json::from_str::<Value>(raw) {
            inject_inbounds(&mut config, settings);
            return serde_json::to_string_pretty(&config).unwrap_or_default();
        }
    }

    // Build config from parsed nodes
    let first_node = nodes.first();
    let outbound = if let Some(node) = first_node {
        if let Some(raw) = &node.raw_outbound {
            raw.clone()
        } else {
            node_to_xray_outbound(node)
        }
    } else {
        json!({"protocol": "freedom", "tag": "direct"})
    };

    let config = json!({
        "log": {
            "loglevel": "warning"
        },
        "inbounds": [
            {
                "tag": "socks-in",
                "port": settings.mixed_port + 918,  // 10808 default
                "listen": "127.0.0.1",
                "protocol": "socks",
                "settings": {
                    "udp": true
                },
                "sniffing": {
                    "enabled": true,
                    "destOverride": ["http", "tls"]
                }
            },
            {
                "tag": "http-in",
                "port": settings.mixed_port,
                "listen": "127.0.0.1",
                "protocol": "http",
                "settings": {}
            }
        ],
        "outbounds": [
            outbound,
            {
                "protocol": "freedom",
                "tag": "direct"
            },
            {
                "protocol": "blackhole",
                "tag": "block"
            }
        ],
        "routing": {
            "domainStrategy": "IPIfNonMatch",
            "rules": [
                {
                    "type": "field",
                    "ip": ["geoip:private"],
                    "outboundTag": "direct"
                },
                {
                    "type": "field",
                    "domain": ["geosite:category-ads-all"],
                    "outboundTag": "block"
                }
            ]
        },
        "dns": {
            "servers": [
                "8.8.8.8",
                "1.1.1.1",
                {
                    "address": "114.114.114.114",
                    "domains": ["geosite:cn"]
                }
            ]
        }
    });

    serde_json::to_string_pretty(&config).unwrap_or_default()
}

fn inject_inbounds(config: &mut Value, settings: &UserSettings) {
    config["inbounds"] = json!([
        {
            "tag": "socks-in",
            "port": settings.mixed_port + 918,
            "listen": "127.0.0.1",
            "protocol": "socks",
            "settings": {"udp": true},
            "sniffing": {"enabled": true, "destOverride": ["http", "tls"]}
        },
        {
            "tag": "http-in",
            "port": settings.mixed_port,
            "listen": "127.0.0.1",
            "protocol": "http",
            "settings": {}
        }
    ]);
}

fn node_to_xray_outbound(node: &ServerNode) -> Value {
    let s = &node.settings;

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

            let mut outbound = json!({
                "tag": "proxy",
                "protocol": "vmess",
                "settings": {
                    "vnext": [{
                        "address": node.address,
                        "port": node.port,
                        "users": [{"id": uuid, "alterId": aid, "security": "auto"}]
                    }]
                }
            });

            let mut stream = json!({"network": net});
            if tls == "tls" {
                stream["security"] = json!("tls");
                stream["tlsSettings"] = json!({"serverName": sni});
            }
            if net == "ws" {
                stream["wsSettings"] = json!({"path": path, "headers": {"Host": host}});
            } else if net == "grpc" {
                let svc = s.get("serviceName").and_then(|v| v.as_str()).unwrap_or("");
                stream["grpcSettings"] = json!({"serviceName": svc});
            }
            outbound["streamSettings"] = stream;
            outbound
        }
        "vless" => {
            let uuid = s.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
            let flow = s.get("flow").and_then(|v| v.as_str()).unwrap_or("");
            let security = s.get("security").and_then(|v| v.as_str()).unwrap_or("none");
            let sni = s.get("sni").and_then(|v| v.as_str()).unwrap_or("");
            let net_type = s.get("type").and_then(|v| v.as_str()).unwrap_or("tcp");
            let fp = s.get("fp").and_then(|v| v.as_str()).unwrap_or("chrome");
            let pbk = s.get("pbk").and_then(|v| v.as_str()).unwrap_or("");
            let sid = s.get("sid").and_then(|v| v.as_str()).unwrap_or("");
            let path = s.get("path").and_then(|v| v.as_str()).unwrap_or("");
            let host = s.get("host").and_then(|v| v.as_str()).unwrap_or("");

            let mut user = json!({"id": uuid, "encryption": "none"});
            if !flow.is_empty() {
                user["flow"] = json!(flow);
            }

            let mut outbound = json!({
                "tag": "proxy",
                "protocol": "vless",
                "settings": {
                    "vnext": [{
                        "address": node.address,
                        "port": node.port,
                        "users": [user]
                    }]
                }
            });

            let mut stream = json!({"network": net_type});
            if security == "tls" {
                stream["security"] = json!("tls");
                stream["tlsSettings"] = json!({
                    "serverName": sni,
                    "fingerprint": fp,
                    "alpn": ["h2", "http/1.1"]
                });
            } else if security == "reality" {
                stream["security"] = json!("reality");
                stream["realitySettings"] = json!({
                    "serverName": sni,
                    "fingerprint": fp,
                    "publicKey": pbk,
                    "shortId": sid
                });
            }
            if net_type == "ws" {
                stream["wsSettings"] = json!({"path": path, "headers": {"Host": host}});
            } else if net_type == "grpc" {
                let svc = s.get("serviceName").and_then(|v| v.as_str()).unwrap_or("");
                stream["grpcSettings"] = json!({"serviceName": svc});
            } else if net_type == "xhttp" {
                stream["xhttpSettings"] = json!({"path": path, "host": host});
            }
            outbound["streamSettings"] = stream;
            outbound
        }
        "trojan" => {
            let password = s.get("password").and_then(|v| v.as_str()).unwrap_or("");
            let sni = s.get("sni").and_then(|v| v.as_str()).unwrap_or(&node.address);
            let fp = s.get("fp").and_then(|v| v.as_str()).unwrap_or("chrome");

            json!({
                "tag": "proxy",
                "protocol": "trojan",
                "settings": {
                    "servers": [{
                        "address": node.address,
                        "port": node.port,
                        "password": password
                    }]
                },
                "streamSettings": {
                    "network": "tcp",
                    "security": "tls",
                    "tlsSettings": {
                        "serverName": sni,
                        "fingerprint": fp
                    }
                }
            })
        }
        "shadowsocks" | "ss" => {
            let method = s.get("method").or(s.get("cipher"))
                .and_then(|v| v.as_str()).unwrap_or("aes-256-gcm");
            let password = s.get("password").and_then(|v| v.as_str()).unwrap_or("");

            json!({
                "tag": "proxy",
                "protocol": "shadowsocks",
                "settings": {
                    "servers": [{
                        "address": node.address,
                        "port": node.port,
                        "method": method,
                        "password": password
                    }]
                }
            })
        }
        _ => json!({"protocol": "freedom", "tag": "proxy"}),
    }
}
