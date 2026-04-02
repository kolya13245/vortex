use crate::error::VortexError;
use log::{info, warn};
use std::process::Command;

/// Set system HTTP proxy to 127.0.0.1:port
pub fn set_system_proxy(port: u16) -> Result<(), VortexError> {
    let addr = format!("127.0.0.1:{}", port);
    info!("Setting system proxy to {}", addr);

    #[cfg(target_os = "linux")]
    {
        set_proxy_linux(&addr)?;
    }

    #[cfg(target_os = "windows")]
    {
        set_proxy_windows(&addr)?;
    }

    Ok(())
}

/// Unset system proxy
pub fn unset_system_proxy() -> Result<(), VortexError> {
    info!("Unsetting system proxy");

    #[cfg(target_os = "linux")]
    {
        unset_proxy_linux()?;
    }

    #[cfg(target_os = "windows")]
    {
        unset_proxy_windows()?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn set_proxy_linux(addr: &str) -> Result<(), VortexError> {
    let port_str = addr.rsplit_once(':').map(|(_, p)| p).unwrap_or("7890").to_string();

    let cmds: Vec<Vec<&str>> = vec![
        vec!["gsettings", "set", "org.gnome.system.proxy", "mode", "manual"],
        vec!["gsettings", "set", "org.gnome.system.proxy.http", "host", "127.0.0.1"],
    ];

    for cmd in &cmds {
        let result = Command::new(cmd[0]).args(&cmd[1..]).output();
        if let Err(e) = result {
            warn!("gsettings command failed: {} — maybe not GNOME", e);
            return Ok(());
        }
    }

    // Set ports separately to avoid borrow issues
    for schema in &["http", "https", "socks"] {
        let key = format!("org.gnome.system.proxy.{}", schema);
        let _ = Command::new("gsettings").args(["set", &key, "host", "127.0.0.1"]).output();
        let _ = Command::new("gsettings").args(["set", &key, "port", &port_str]).output();
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn unset_proxy_linux() -> Result<(), VortexError> {
    let _ = Command::new("gsettings")
        .args(["set", "org.gnome.system.proxy", "mode", "none"])
        .output();
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_proxy_windows(addr: &str) -> Result<(), VortexError> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyEnable",
            "/t", "REG_DWORD",
            "/d", "1",
            "/f",
        ])
        .output();

    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyServer",
            "/t", "REG_SZ",
            "/d", addr,
            "/f",
        ])
        .output();

    Ok(())
}

#[cfg(target_os = "windows")]
fn unset_proxy_windows() -> Result<(), VortexError> {
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyEnable",
            "/t", "REG_DWORD",
            "/d", "0",
            "/f",
        ])
        .output();

    Ok(())
}
