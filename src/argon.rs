use anyhow::{Context, Result};
use std::process::Command;

use crate::InstallContext;

/// Install Argon One fan control scripts.
///
/// This is entirely optional and only relevant for Raspberry Pi 4 with an
/// Argon One case.  The upstream script is:
///   curl https://download.argon40.com/argon1.sh | bash
pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    if which::which("argonone-config").is_ok() {
        tracing::info!("Argon One scripts already installed");
        return Ok(());
    }

    if ctx.platform.pi_model.is_none() {
        tracing::warn!("Not running on a Raspberry Pi; skipping Argon One");
        return Ok(());
    }

    tracing::info!("Installing Argon One fan control scripts (optional)");
    if ctx.dry_run {
        tracing::info!("[dry-run] would install Argon One scripts");
        return Ok(());
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://download.argon40.com/argon1.sh | bash")
        .status()
        .context("running Argon One install script")?;

    if !status.success() {
        tracing::warn!("Argon One install script failed; this is non-critical");
    }

    Ok(())
}
