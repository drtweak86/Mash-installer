use anyhow::{bail, Result};
use std::fs;

/// Information about the host we are running on.
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub arch: String,
    pub distro: String,
    pub distro_version: String,
    pub distro_codename: String,
    pub pi_model: Option<String>,
}

/// Detect the current platform.  We require Ubuntu 22.04+.
pub fn detect() -> Result<PlatformInfo> {
    let arch = std::env::consts::ARCH.to_string();

    // Read /etc/os-release
    let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let distro = parse_os_field(&os_release, "ID").unwrap_or_else(|| "unknown".into());
    let distro_version = parse_os_field(&os_release, "VERSION_ID").unwrap_or_else(|| "0".into());
    let distro_codename = parse_os_field(&os_release, "VERSION_CODENAME").unwrap_or_default();

    // Sanity check
    if distro != "ubuntu" && distro != "debian" {
        tracing::warn!(
            "Detected distro '{}' – this installer targets Ubuntu but may work on Debian-based distros.",
            distro
        );
    }

    let ver_major: u32 = distro_version
        .split('.')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if distro == "ubuntu" && ver_major < 22 {
        bail!("Ubuntu {} is too old; 22.04+ is required.", distro_version);
    }

    // Pi model detection
    let pi_model = detect_pi_model();

    Ok(PlatformInfo {
        arch,
        distro,
        distro_version,
        distro_codename,
        pi_model,
    })
}

fn parse_os_field(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix(&format!("{key}=")) {
            return Some(rest.trim_matches('"').to_string());
        }
    }
    None
}

fn detect_pi_model() -> Option<String> {
    // /proc/device-tree/model on Raspberry Pi
    if let Ok(model) = fs::read_to_string("/proc/device-tree/model") {
        let model = model.trim_end_matches('\0').trim().to_string();
        if !model.is_empty() {
            return Some(model);
        }
    }
    // fallback: /proc/cpuinfo
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("Model") || line.starts_with("Hardware") {
                if let Some((_k, v)) = line.split_once(':') {
                    return Some(v.trim().to_string());
                }
            }
        }
    }
    None
}
