use anyhow::{Context, Result};
use std::process::Command;

use crate::InstallContext;

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    if which::which("rclone").is_ok() {
        tracing::info!("rclone already installed");
        return Ok(());
    }

    // Try apt first
    if try_apt(ctx)? {
        return Ok(());
    }

    // Fall back to official install script
    install_via_script(ctx)?;
    Ok(())
}

fn try_apt(ctx: &InstallContext) -> Result<bool> {
    if crate::apt::is_installed("rclone") {
        return Ok(true);
    }

    tracing::info!("Attempting rclone install via apt");
    if ctx.dry_run {
        tracing::info!("[dry-run] would try apt install rclone");
        return Ok(true);
    }

    let status = Command::new("sudo")
        .args(["apt-get", "install", "-y", "--install-recommends", "rclone"])
        .env("DEBIAN_FRONTEND", "noninteractive")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    match status {
        Ok(s) if s.success() => {
            tracing::info!("Installed rclone via apt");
            Ok(true)
        }
        _ => {
            tracing::info!("rclone not in apt; will use official script");
            Ok(false)
        }
    }
}

fn install_via_script(ctx: &InstallContext) -> Result<()> {
    tracing::info!("Installing rclone via official install script");
    if ctx.dry_run {
        tracing::info!("[dry-run] would run rclone install script");
        return Ok(());
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://rclone.org/install.sh | sudo bash")
        .status()
        .context("running rclone install script")?;

    if !status.success() {
        tracing::warn!("rclone install script failed; continuing");
    }
    Ok(())
}
