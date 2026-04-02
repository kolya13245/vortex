use tauri::State;

use crate::config::{mihomo_gen, xray_gen};
use crate::core;
use crate::error::VortexError;
use crate::state::*;
use crate::subscription::{fetcher, parser};
use crate::system::proxy;

// ─── Core Status ────────────────────────────────────────────────

#[tauri::command]
pub fn get_core_status(state: State<AppState>) -> CoreStatus {
    let settings = state.settings.lock().unwrap();
    let status = *state.connection_status.lock().unwrap();
    CoreStatus {
        core_type: settings.core_type,
        connection_status: status,
        uptime_secs: 0,
        current_proxy: None,
    }
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> UserSettings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_settings(state: State<AppState>, settings: UserSettings) -> Result<(), VortexError> {
    *state.settings.lock().unwrap() = settings;
    Ok(())
}

#[tauri::command]
pub fn get_hwid(state: State<AppState>) -> String {
    state.hwid.clone()
}

#[tauri::command]
pub fn get_platform() -> String {
    std::env::consts::OS.to_string()
}

#[tauri::command]
pub fn switch_core(state: State<AppState>, core_type: CoreType) -> Result<(), VortexError> {
    state.settings.lock().unwrap().core_type = core_type;
    Ok(())
}

// ─── Subscriptions ──────────────────────────────────────────────

#[tauri::command]
pub fn get_subscriptions(state: State<AppState>) -> Vec<Subscription> {
    state.subscriptions.lock().unwrap().clone()
}

#[tauri::command]
pub async fn add_subscription(
    state: State<'_, AppState>,
    name: String,
    url: String,
) -> Result<Subscription, VortexError> {
    let hwid = state.hwid.clone();

    // Fetch subscription
    let result = fetcher::fetch_subscription(&url, &hwid).await?;

    // Parse nodes based on format
    let nodes = match result.format {
        fetcher::SubFormat::XrayJson => parser::parse_xray_json(&result.body)?,
        fetcher::SubFormat::ClashYaml => parser::parse_clash_yaml(&result.body)?,
        fetcher::SubFormat::Base64 => parser::parse_base64(&result.body)?,
    };

    let core_type = result.core_hint.unwrap_or_else(|| {
        // Default: if YAML -> Mihomo, if JSON/Base64 -> Xray
        match result.format {
            fetcher::SubFormat::ClashYaml => CoreType::Mihomo,
            _ => CoreType::Xray,
        }
    });

    let (traffic_used, traffic_total, expire) = match &result.traffic {
        Some(t) => (
            Some(t.upload + t.download),
            Some(t.total),
            t.expire.clone(),
        ),
        None => (None, None, None),
    };

    let raw_config = if result.format == fetcher::SubFormat::XrayJson {
        Some(result.body.clone())
    } else {
        None
    };

    let sub = Subscription {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        url,
        core_type,
        node_count: nodes.len(),
        nodes,
        traffic_used,
        traffic_total,
        expire,
        last_updated: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string()),
        raw_config,
    };

    state.subscriptions.lock().unwrap().push(sub.clone());

    // Auto-switch core type based on subscription
    state.settings.lock().unwrap().core_type = core_type;

    Ok(sub)
}

#[tauri::command]
pub fn remove_subscription(state: State<AppState>, id: String) -> Result<(), VortexError> {
    state.subscriptions.lock().unwrap().retain(|s| s.id != id);
    Ok(())
}

#[tauri::command]
pub async fn update_subscription(
    state: State<'_, AppState>,
    id: String,
) -> Result<Subscription, VortexError> {
    let (url, hwid) = {
        let subs = state.subscriptions.lock().unwrap();
        let sub = subs
            .iter()
            .find(|s| s.id == id)
            .ok_or(VortexError::Other("Subscription not found".into()))?;
        (sub.url.clone(), state.hwid.clone())
    };

    let result = fetcher::fetch_subscription(&url, &hwid).await?;
    let nodes = match result.format {
        fetcher::SubFormat::XrayJson => parser::parse_xray_json(&result.body)?,
        fetcher::SubFormat::ClashYaml => parser::parse_clash_yaml(&result.body)?,
        fetcher::SubFormat::Base64 => parser::parse_base64(&result.body)?,
    };

    let mut subs = state.subscriptions.lock().unwrap();
    let sub = subs
        .iter_mut()
        .find(|s| s.id == id)
        .ok_or(VortexError::Other("Subscription not found".into()))?;

    sub.node_count = nodes.len();
    sub.nodes = nodes;
    sub.last_updated = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string());

    if let Some(t) = &result.traffic {
        sub.traffic_used = Some(t.upload + t.download);
        sub.traffic_total = Some(t.total);
        sub.expire = t.expire.clone();
    }

    if result.format == fetcher::SubFormat::XrayJson {
        sub.raw_config = Some(result.body);
    }

    Ok(sub.clone())
}

// ─── Connect / Disconnect ───────────────────────────────────────

#[tauri::command]
pub fn connect(state: State<AppState>) -> Result<(), VortexError> {
    let settings = state.settings.lock().unwrap().clone();
    let subs = state.subscriptions.lock().unwrap().clone();

    // Collect all nodes from subscriptions matching active core type
    let mut all_nodes: Vec<ServerNode> = Vec::new();
    let mut raw_config: Option<String> = None;

    for sub in &subs {
        if sub.core_type == settings.core_type {
            all_nodes.extend(sub.nodes.clone());
            if raw_config.is_none() {
                raw_config = sub.raw_config.clone();
            }
        }
    }

    // Fallback: use all nodes regardless of core type
    if all_nodes.is_empty() {
        for sub in &subs {
            all_nodes.extend(sub.nodes.clone());
            if raw_config.is_none() {
                raw_config = sub.raw_config.clone();
            }
        }
    }

    if all_nodes.is_empty() {
        return Err(VortexError::Other(
            "No proxy nodes available. Add a subscription first.".into(),
        ));
    }

    // Generate config
    let config_dir = state.data_dir.join("configs");
    let config_path = match settings.core_type {
        CoreType::Mihomo => {
            let content = mihomo_gen::generate(&all_nodes, &settings, &config_dir);
            let path = config_dir.join("config.yaml");
            std::fs::write(&path, &content)?;
            path
        }
        CoreType::Xray => {
            let content = xray_gen::generate(&all_nodes, &settings, raw_config.as_deref());
            let path = config_dir.join("config.json");
            std::fs::write(&path, &content)?;
            path
        }
    };

    // Find binary
    let binary = core::find_binary(settings.core_type, &state.data_dir).ok_or_else(|| {
        VortexError::CoreStartFailed(format!(
            "{:?} binary not found. Place it in {:?}/bin/",
            settings.core_type, state.data_dir
        ))
    })?;

    // Stop existing core if running
    {
        let mut proc = state.core_process.lock().unwrap();
        if let Some(ref mut child) = *proc {
            core::stop_core(child);
        }
        *proc = None;
    }

    // Start core
    let child = core::start_core(settings.core_type, &binary, &config_path)?;
    *state.core_process.lock().unwrap() = Some(child);

    // Set system proxy if enabled
    if settings.system_proxy {
        proxy::set_system_proxy(settings.mixed_port).ok();
    }

    *state.connection_status.lock().unwrap() = ConnectionStatus::Connected;

    Ok(())
}

#[tauri::command]
pub fn disconnect(state: State<AppState>) -> Result<(), VortexError> {
    // Stop core process
    {
        let mut proc = state.core_process.lock().unwrap();
        if let Some(ref mut child) = *proc {
            core::stop_core(child);
        }
        *proc = None;
    }

    // Unset system proxy
    proxy::unset_system_proxy().ok();

    *state.connection_status.lock().unwrap() = ConnectionStatus::Disconnected;

    Ok(())
}

// ─── Traffic & Logs ─────────────────────────────────────────────

#[tauri::command]
pub fn get_traffic() -> TrafficStats {
    // TODO: read from Mihomo API or Xray stats
    TrafficStats {
        download_speed: 0,
        upload_speed: 0,
        download_total: 0,
        upload_total: 0,
    }
}

#[tauri::command]
pub fn get_logs() -> Vec<String> {
    vec![
        "[INFO] Vortex started".to_string(),
        "[INFO] Waiting for connection...".to_string(),
    ]
}
