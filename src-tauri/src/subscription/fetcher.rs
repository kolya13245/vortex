use crate::error::VortexError;
use crate::state::{CoreType, TrafficInfo};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};

/// Detected subscription format
#[derive(Debug, Clone, PartialEq)]
pub enum SubFormat {
    /// Xray JSON config (single object or array of outbounds)
    XrayJson,
    /// Clash/Mihomo YAML with proxies section
    ClashYaml,
    /// Base64-encoded list of proxy URIs
    Base64,
}

/// Result of fetching a subscription
#[derive(Debug)]
pub struct FetchResult {
    pub body: String,
    pub format: SubFormat,
    pub core_hint: Option<CoreType>,
    pub traffic: Option<TrafficInfo>,
    #[allow(dead_code)]
    pub update_interval: Option<u64>,
}

/// Fetch subscription from URL with HWID headers
pub async fn fetch_subscription(url: &str, hwid: &str) -> Result<FetchResult, VortexError> {
    let mut headers = HeaderMap::new();
    headers.insert("x-hwid", HeaderValue::from_str(hwid).unwrap_or(HeaderValue::from_static("")));
    headers.insert("x-device-os", HeaderValue::from_static(std::env::consts::OS));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Vortex/0.1.0"),
    );

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(false)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| VortexError::SubscriptionFetchFailed(e.to_string()))?;

    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| VortexError::SubscriptionFetchFailed(e.to_string()))?;

    if !resp.status().is_success() {
        return Err(VortexError::SubscriptionFetchFailed(format!(
            "HTTP {}",
            resp.status()
        )));
    }

    // Parse response headers for traffic info
    let traffic = parse_subscription_userinfo(resp.headers());
    let update_interval = resp
        .headers()
        .get("profile-update-interval")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok());

    // Detect core hint from response headers
    let core_hint = resp
        .headers()
        .get("x-subscription-core")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| match v.to_lowercase().as_str() {
            "xray" | "xray-core" => Some(CoreType::Xray),
            "mihomo" | "clash" | "clash-meta" => Some(CoreType::Mihomo),
            _ => None,
        });

    // Detect format from Content-Type
    let content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    let body = resp
        .text()
        .await
        .map_err(|e| VortexError::SubscriptionFetchFailed(e.to_string()))?;

    let format = detect_format(&body, &content_type);
    let core_hint = core_hint.or(match format {
        SubFormat::XrayJson => Some(CoreType::Xray),
        SubFormat::ClashYaml => Some(CoreType::Mihomo),
        SubFormat::Base64 => None,
    });

    Ok(FetchResult {
        body,
        format,
        core_hint,
        traffic,
        update_interval,
    })
}

/// Detect subscription format from body content and content-type header
fn detect_format(body: &str, content_type: &str) -> SubFormat {
    // Check content-type header first
    if content_type.contains("application/json") {
        return SubFormat::XrayJson;
    }
    if content_type.contains("application/yaml")
        || content_type.contains("application/x-yaml")
        || content_type.contains("text/yaml")
    {
        return SubFormat::ClashYaml;
    }

    // Heuristic: check body content
    let trimmed = body.trim();

    // JSON detection
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        // Verify it's actually valid JSON
        if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
            return SubFormat::XrayJson;
        }
    }

    // YAML detection: look for typical Clash config markers
    if trimmed.contains("proxies:")
        || trimmed.contains("proxy-groups:")
        || trimmed.starts_with("port:")
        || trimmed.starts_with("mixed-port:")
    {
        return SubFormat::ClashYaml;
    }

    // Default: assume Base64-encoded proxy URIs
    SubFormat::Base64
}

/// Parse subscription-userinfo header
/// Format: upload=X; download=Y; total=Z; expire=T
fn parse_subscription_userinfo(headers: &HeaderMap) -> Option<TrafficInfo> {
    let value = headers.get("subscription-userinfo")?.to_str().ok()?;

    let mut upload = 0u64;
    let mut download = 0u64;
    let mut total = 0u64;
    let mut expire: Option<String> = None;

    for part in value.split(';') {
        let part = part.trim();
        if let Some((key, val)) = part.split_once('=') {
            let key = key.trim();
            let val = val.trim();
            match key {
                "upload" => upload = val.parse().unwrap_or(0),
                "download" => download = val.parse().unwrap_or(0),
                "total" => total = val.parse().unwrap_or(0),
                "expire" => {
                    if let Ok(ts) = val.parse::<i64>() {
                        if ts > 0 {
                            expire = Some(
                                chrono::DateTime::from_timestamp(ts, 0)
                                    .map(|dt| dt.format("%Y-%m-%d").to_string())
                                    .unwrap_or_default(),
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Some(TrafficInfo {
        upload,
        download,
        total,
        expire,
    })
}
