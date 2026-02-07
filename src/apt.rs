use anyhow::{Context, Result};
use std::process::Command;

use crate::InstallContext;

/// Check whether a package is already installed via dpkg.
pub fn is_installed(pkg: &str) -> bool {
    Command::new("dpkg")
        .args(["-s", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Run `apt-get update` once.
pub fn update(dry_run: bool) -> Result<()> {
    if dry_run {
        tracing::info!("[dry-run] apt-get update");
        return Ok(());
    }
    let status = Command::new("sudo")
        .args(["apt-get", "update", "-qq"])
        .status()
        .context("running apt-get update")?;
    if !status.success() {
        anyhow::bail!("apt-get update failed");
    }
    Ok(())
}

/// Install a list of packages idempotently.  Uses --install-recommends.
pub fn ensure_packages(pkgs: &[&str], dry_run: bool) -> Result<()> {
    let missing: Vec<&str> = pkgs.iter().copied().filter(|p| !is_installed(p)).collect();
    if missing.is_empty() {
        tracing::info!("All packages already installed");
        return Ok(());
    }
    tracing::info!(
        "Installing {} packages: {}",
        missing.len(),
        missing.join(", ")
    );

    if dry_run {
        tracing::info!("[dry-run] would install: {}", missing.join(" "));
        return Ok(());
    }

    let mut cmd = Command::new("sudo");
    cmd.args(["apt-get", "install", "-y", "--install-recommends"]);
    for p in &missing {
        cmd.arg(p);
    }

    let status = cmd
        .env("DEBIAN_FRONTEND", "noninteractive")
        .status()
        .context("running apt-get install")?;

    if !status.success() {
        anyhow::bail!("apt-get install failed for: {}", missing.join(" "));
    }
    Ok(())
}

/// Mega-install phase: core packages for every profile.
pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    update(ctx.dry_run)?;

    // Always-needed core packages
    let mut pkgs: Vec<&str> = vec![
        "ca-certificates",
        "curl",
        "wget",
        "xz-utils",
        "tar",
        "coreutils",
        "jq",
        "git",
        "software-properties-common",
        "gnupg",
        "lsb-release",
        "apt-transport-https",
    ];

    // Build essentials (all profiles)
    pkgs.extend_from_slice(&[
        "build-essential",
        "pkg-config",
        "clang",
        "lld",
        "cmake",
        "ninja-build",
        "gcc",
        "g++",
        "gdb",
        "make",
    ]);

    // lldb – may not exist on all Ubuntu versions
    if !is_installed("lldb") {
        // Try to install; won't fail the whole phase if unavailable
        pkgs.push("lldb");
    }

    // Dev+ packages
    if ctx.profile >= crate::ProfileLevel::Dev {
        pkgs.extend_from_slice(&[
            // AI / scripting
            "python3",
            "python3-pip",
            "python3-venv",
            "ripgrep",
            "fd-find",
            "fzf",
            // terminal basics
            "tmux",
            "htop",
            "ncdu",
            "neovim",
        ]);

        // btop may not be in older repos; best-effort
        pkgs.push("btop");
        // bat
        pkgs.push("bat");
        // eza may not be in apt on 22.04; added best-effort
        pkgs.push("eza");
    }

    // Full profile extras
    if ctx.profile >= crate::ProfileLevel::Full {
        pkgs.extend_from_slice(&["nodejs", "npm"]);
    }

    // yq – may not be in apt; skip gracefully
    if ctx.profile >= crate::ProfileLevel::Dev {
        pkgs.push("yq");
    }

    // Install, but don't abort the whole process for optional packages that
    // may not exist in this Ubuntu version (btop, eza, yq, lldb, bat).
    let optional = ["btop", "eza", "yq", "lldb", "bat"];
    let required: Vec<&str> = pkgs
        .iter()
        .copied()
        .filter(|p| !optional.contains(p))
        .collect();
    let optional_present: Vec<&str> = pkgs
        .iter()
        .copied()
        .filter(|p| optional.contains(p))
        .collect();

    ensure_packages(&required, ctx.dry_run)?;

    // Best-effort install for optional packages (one at a time)
    for pkg in &optional_present {
        if is_installed(pkg) {
            continue;
        }
        if ctx.dry_run {
            tracing::info!("[dry-run] would attempt optional: {pkg}");
            continue;
        }
        let status = Command::new("sudo")
            .args(["apt-get", "install", "-y", "--install-recommends", pkg])
            .env("DEBIAN_FRONTEND", "noninteractive")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        match status {
            Ok(s) if s.success() => tracing::info!("Installed optional package: {pkg}"),
            _ => tracing::warn!("Optional package '{pkg}' not available; skipping"),
        }
    }

    Ok(())
}
