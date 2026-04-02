use crate::error::VortexError;
use crate::state::ServerNode;
use base64::Engine;
use serde_json::json;

/// Parse subscription body based on detected format
pub fn parse_base64(body: &str) -> Result<Vec<ServerNode>, VortexError> {
    let decoded = decode_base64(body)?;
    let mut nodes = Vec::new();

    for line in decoded.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(node) = parse_uri(line) {
            nodes.push(node);
        }
    }

    if nodes.is_empty() {
        return Err(VortexError::SubscriptionParseFailed(
            "No valid proxy URIs found".to_string(),
        ));
    }

    Ok(nodes)
}

/// Parse Clash/Mihomo YAML proxies
pub fn parse_clash_yaml(body: &str) -> Result<Vec<ServerNode>, VortexError> {
    let value: serde_yaml::Value =
        serde_yaml::from_str(body).map_err(|e| VortexError::SubscriptionParseFailed(e.to_string()))?;

    let proxies = value
        .get("proxies")
        .and_then(|v| v.as_sequence())
        .ok_or_else(|| VortexError::SubscriptionParseFailed("No 'proxies' section found".to_string()))?;

    let mut nodes = Vec::new();
    for proxy in proxies {
        let name = proxy.get("name").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
        let proto = proxy.get("type").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
        let server = proxy.get("server").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let port = proxy.get("port").and_then(|v| v.as_u64()).unwrap_or(0) as u16;

        if server.is_empty() || port == 0 {
            continue;
        }

        // Convert YAML proxy to JSON for storage
        let settings_json = yaml_to_json(proxy);

        nodes.push(ServerNode {
            name,
            protocol: proto,
            address: server,
            port,
            raw_outbound: None,
            settings: settings_json,
        });
    }

    Ok(nodes)
}

/// Parse Xray JSON config — array of outbound configs or single config with outbounds
pub fn parse_xray_json(body: &str) -> Result<Vec<ServerNode>, VortexError> {
    let value: serde_json::Value =
        serde_json::from_str(body).map_err(|e| VortexError::SubscriptionParseFailed(e.to_string()))?;

    let mut nodes = Vec::new();

    // Case 1: JSON array of outbound configs
    if let Some(arr) = value.as_array() {
        for (i, outbound) in arr.iter().enumerate() {
            if let Some(node) = xray_outbound_to_node(outbound, i) {
                nodes.push(node);
            }
        }
    }
    // Case 2: Single config with "outbounds" array
    else if let Some(outbounds) = value.get("outbounds").and_then(|v| v.as_array()) {
        for (i, outbound) in outbounds.iter().enumerate() {
            // Skip freedom, blackhole, dns outbounds
            let proto = outbound.get("protocol").and_then(|v| v.as_str()).unwrap_or("");
            if matches!(proto, "freedom" | "blackhole" | "dns" | "loopback") {
                continue;
            }
            if let Some(node) = xray_outbound_to_node(outbound, i) {
                nodes.push(node);
            }
        }
    }

    if nodes.is_empty() {
        return Err(VortexError::SubscriptionParseFailed(
            "No valid outbound configs found".to_string(),
        ));
    }

    Ok(nodes)
}

fn xray_outbound_to_node(outbound: &serde_json::Value, index: usize) -> Option<ServerNode> {
    let protocol = outbound.get("protocol")?.as_str()?.to_string();
    let tag = outbound
        .get("tag")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let settings = outbound.get("settings")?;

    // Extract server address and port from settings
    let (address, port) = extract_xray_address(settings, &protocol)?;

    let name = if tag.is_empty() {
        format!("{} #{}", protocol, index + 1)
    } else {
        tag
    };

    Some(ServerNode {
        name,
        protocol,
        address,
        port,
        raw_outbound: Some(outbound.clone()),
        settings: settings.clone(),
    })
}

fn extract_xray_address(settings: &serde_json::Value, protocol: &str) -> Option<(String, u16)> {
    match protocol {
        "vmess" | "vless" => {
            let vnext = settings.get("vnext")?.as_array()?.first()?;
            let addr = vnext.get("address")?.as_str()?.to_string();
            let port = vnext.get("port")?.as_u64()? as u16;
            Some((addr, port))
        }
        "trojan" | "shadowsocks" => {
            let servers = settings.get("servers")?.as_array()?.first()?;
            let addr = servers.get("address")?.as_str()?.to_string();
            let port = servers.get("port")?.as_u64()? as u16;
            Some((addr, port))
        }
        _ => {
            // Try generic address/port
            let addr = settings.get("address")?.as_str()?.to_string();
            let port = settings.get("port")?.as_u64()? as u16;
            Some((addr, port))
        }
    }
}

/// Parse a single proxy URI (vmess://, vless://, trojan://, ss://)
fn parse_uri(uri: &str) -> Option<ServerNode> {
    if uri.starts_with("vmess://") {
        parse_vmess_uri(uri)
    } else if uri.starts_with("vless://") {
        parse_vless_uri(uri)
    } else if uri.starts_with("trojan://") {
        parse_trojan_uri(uri)
    } else if uri.starts_with("ss://") {
        parse_ss_uri(uri)
    } else {
        None
    }
}

fn parse_vmess_uri(uri: &str) -> Option<ServerNode> {
    let b64 = uri.strip_prefix("vmess://")?;
    let decoded = decode_base64(b64).ok()?;
    let obj: serde_json::Value = serde_json::from_str(&decoded).ok()?;

    let address = obj.get("add")?.as_str()?.to_string();
    let port = obj
        .get("port")
        .and_then(|v| v.as_str().and_then(|s| s.parse().ok()).or_else(|| v.as_u64()))
        .unwrap_or(0) as u16;
    let name = obj
        .get("ps")
        .and_then(|v| v.as_str())
        .unwrap_or("VMess")
        .to_string();

    Some(ServerNode {
        name,
        protocol: "vmess".to_string(),
        address,
        port,
        raw_outbound: None,
        settings: obj,
    })
}

fn parse_vless_uri(uri: &str) -> Option<ServerNode> {
    let rest = uri.strip_prefix("vless://")?;
    let (userinfo_host, fragment) = rest.rsplit_once('#').unwrap_or((rest, ""));
    let name = urlencoding_decode(fragment);

    let (userinfo, host_port_params) = userinfo_host.split_once('@')?;
    let uuid = userinfo.to_string();

    let (host_port, params_str) = host_port_params.split_once('?').unwrap_or((host_port_params, ""));
    let (host, port_str) = parse_host_port(host_port)?;
    let port: u16 = port_str.parse().ok()?;

    let params = parse_query_params(params_str);

    Some(ServerNode {
        name: if name.is_empty() { format!("VLESS {}", host) } else { name },
        protocol: "vless".to_string(),
        address: host,
        port,
        raw_outbound: None,
        settings: json!({
            "uuid": uuid,
            "type": params.get("type").cloned().unwrap_or_default(),
            "security": params.get("security").cloned().unwrap_or_default(),
            "flow": params.get("flow").cloned().unwrap_or_default(),
            "sni": params.get("sni").cloned().unwrap_or_default(),
            "fp": params.get("fp").cloned().unwrap_or_default(),
            "pbk": params.get("pbk").cloned().unwrap_or_default(),
            "sid": params.get("sid").cloned().unwrap_or_default(),
            "path": params.get("path").cloned().unwrap_or_default(),
            "host": params.get("host").cloned().unwrap_or_default(),
            "headerType": params.get("headerType").cloned().unwrap_or_default(),
            "serviceName": params.get("serviceName").cloned().unwrap_or_default(),
        }),
    })
}

fn parse_trojan_uri(uri: &str) -> Option<ServerNode> {
    let rest = uri.strip_prefix("trojan://")?;
    let (main, fragment) = rest.rsplit_once('#').unwrap_or((rest, ""));
    let name = urlencoding_decode(fragment);

    let (password, host_port_params) = main.split_once('@')?;
    let (host_port, params_str) = host_port_params.split_once('?').unwrap_or((host_port_params, ""));
    let (host, port_str) = parse_host_port(host_port)?;
    let port: u16 = port_str.parse().ok()?;

    let params = parse_query_params(params_str);

    Some(ServerNode {
        name: if name.is_empty() { format!("Trojan {}", host) } else { name },
        protocol: "trojan".to_string(),
        address: host,
        port,
        raw_outbound: None,
        settings: json!({
            "password": password,
            "sni": params.get("sni").cloned().unwrap_or_default(),
            "type": params.get("type").cloned().unwrap_or_default(),
            "security": params.get("security").cloned().unwrap_or_default(),
            "fp": params.get("fp").cloned().unwrap_or_default(),
        }),
    })
}

fn parse_ss_uri(uri: &str) -> Option<ServerNode> {
    let rest = uri.strip_prefix("ss://")?;
    let (main, fragment) = rest.rsplit_once('#').unwrap_or((rest, ""));
    let name = urlencoding_decode(fragment);

    // ss://base64(method:password)@host:port or ss://base64(method:password@host:port)
    if let Some((userinfo_b64, host_port)) = main.split_once('@') {
        let decoded = decode_base64(userinfo_b64).ok()?;
        let (method, password) = decoded.split_once(':')?;
        let (host, port_str) = parse_host_port(host_port)?;
        let port: u16 = port_str.parse().ok()?;

        Some(ServerNode {
            name: if name.is_empty() { format!("SS {}", host) } else { name },
            protocol: "shadowsocks".to_string(),
            address: host,
            port,
            raw_outbound: None,
            settings: json!({
                "method": method,
                "password": password,
            }),
        })
    } else {
        // Entire thing is base64
        let decoded = decode_base64(main).ok()?;
        let (method_pass, host_port) = decoded.split_once('@')?;
        let (method, password) = method_pass.split_once(':')?;
        let (host, port_str) = parse_host_port(host_port)?;
        let port: u16 = port_str.parse().ok()?;

        Some(ServerNode {
            name: if name.is_empty() { format!("SS {}", host) } else { name },
            protocol: "shadowsocks".to_string(),
            address: host,
            port,
            raw_outbound: None,
            settings: json!({
                "method": method,
                "password": password,
            }),
        })
    }
}

// Helpers

fn decode_base64(input: &str) -> Result<String, VortexError> {
    let input = input.trim();
    // Try standard base64, then URL-safe
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(input)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(input))
        .or_else(|_| {
            // Try with padding
            let padded = match input.len() % 4 {
                2 => format!("{}==", input),
                3 => format!("{}=", input),
                _ => input.to_string(),
            };
            base64::engine::general_purpose::STANDARD
                .decode(&padded)
                .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(&padded))
        })
        .map_err(|e| VortexError::SubscriptionParseFailed(format!("Base64 decode failed: {}", e)))?;

    String::from_utf8(bytes)
        .map_err(|e| VortexError::SubscriptionParseFailed(format!("Invalid UTF-8: {}", e)))
}

fn parse_host_port(s: &str) -> Option<(String, &str)> {
    // Handle [ipv6]:port
    if s.starts_with('[') {
        let end = s.find(']')?;
        let host = s[1..end].to_string();
        let port = s.get(end + 2..)?; // skip "]:"
        Some((host, port))
    } else {
        let (host, port) = s.rsplit_once(':')?;
        Some((host.to_string(), port))
    }
}

fn parse_query_params(query: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for part in query.split('&') {
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.to_string(), urlencoding_decode(v));
        }
    }
    map
}

fn urlencoding_decode(s: &str) -> String {
    url::form_urlencoded::parse(s.as_bytes())
        .map(|(k, v)| {
            if v.is_empty() {
                k.to_string()
            } else {
                format!("{}={}", k, v)
            }
        })
        .collect::<Vec<_>>()
        .join("")
        .replace('=', "")
        // Simple fallback
        .to_string()
}

fn yaml_to_json(yaml: &serde_yaml::Value) -> serde_json::Value {
    match yaml {
        serde_yaml::Value::Null => serde_json::Value::Null,
        serde_yaml::Value::Bool(b) => serde_json::Value::Bool(*b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::json!(f)
            } else {
                serde_json::Value::Null
            }
        }
        serde_yaml::Value::String(s) => serde_json::Value::String(s.clone()),
        serde_yaml::Value::Sequence(seq) => {
            serde_json::Value::Array(seq.iter().map(yaml_to_json).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .filter_map(|(k, v)| {
                    let key = k.as_str()?.to_string();
                    Some((key, yaml_to_json(v)))
                })
                .collect();
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null,
    }
}
