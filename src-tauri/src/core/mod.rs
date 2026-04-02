use crate::error::VortexError;
use crate::state::CoreType;
use log::{error, info};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// Find core binary path
pub fn find_binary(core_type: CoreType, data_dir: &Path) -> Option<PathBuf> {
    let name = match core_type {
        CoreType::Mihomo => "mihomo",
        CoreType::Xray => "xray",
    };

    #[cfg(target_os = "windows")]
    let name = format!("{}.exe", name);

    // Check data_dir/bin/ first
    let bin_dir = data_dir.join("bin");
    let path = bin_dir.join(&name);
    if path.exists() {
        return Some(path);
    }

    // Check PATH
    if let Ok(output) = Command::new("which").arg(&name).output() {
        if output.status.success() {
            let p = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !p.is_empty() {
                return Some(PathBuf::from(p));
            }
        }
    }

    None
}

/// Start core process with generated config
pub fn start_core(
    core_type: CoreType,
    binary_path: &Path,
    config_path: &Path,
) -> Result<Child, VortexError> {
    info!(
        "Starting {:?} core: binary={:?}, config={:?}",
        core_type, binary_path, config_path
    );

    let child = match core_type {
        CoreType::Mihomo => {
            let config_dir = config_path.parent().unwrap_or(config_path);
            Command::new(binary_path)
                .arg("-d")
                .arg(config_dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
        }
        CoreType::Xray => Command::new(binary_path)
            .arg("run")
            .arg("-config")
            .arg(config_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn(),
    };

    child.map_err(|e| {
        error!("Failed to start core: {}", e);
        VortexError::CoreStartFailed(e.to_string())
    })
}

/// Stop core process
pub fn stop_core(child: &mut Child) {
    info!("Stopping core process (pid: {:?})", child.id());

    // Try graceful kill first
    let _ = child.kill();
    let _ = child.wait();
}
