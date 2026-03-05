use installer_core::{DistroDriver, PkgBackend, PlatformInfo};

pub struct ArchDriver;

impl DistroDriver for ArchDriver {
    fn name(&self) -> &'static str {
        "Arch/Manjaro"
    }

    fn description(&self) -> &'static str {
        "Arch-based with pacman backend"
    }

    fn matches(&self, info: &PlatformInfo) -> bool {
        info.distro_family == "arch"
    }

    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Pacman
    }

    fn translate_package(&self, canonical: &str) -> Option<String> {
        match canonical {
            "software-properties-common" | "apt-transport-https" => None,
            "lsb-release" => None,
            "python3-venv" => None,
            "build-essential" => Some("base-devel".to_string()),
            "pkg-config" => Some("pkgconf".to_string()),
            "ninja-build" => Some("ninja".to_string()),
            "g++" => None,
            "xz-utils" => Some("xz".to_string()),
            "python3" => Some("python".to_string()),
            "python3-pip" => Some("python-pip".to_string()),
            "i3" => Some("i3-wm".to_string()),
            "borgbackup" => Some("borg".to_string()),
            "wireguard" => Some("wireguard-tools".to_string()),
            "fd-find" => Some("fd".to_string()),
            "libncurses-dev" => Some("ncurses".to_string()),
            "libssl-dev" => Some("openssl".to_string()),
            "openssh-client" => Some("openssh".to_string()),
            "fonts-terminus" => Some("terminus-font".to_string()),
            "fonts-noto-color-emoji" => Some("noto-fonts-emoji".to_string()),
            "xfonts-terminus" => None,
            "docker-ce" => Some("docker".to_string()),
            "docker-ce-cli" => None,
            "containerd.io" => None,
            "docker-buildx-plugin" => Some("docker-buildx".to_string()),
            "docker-compose-plugin" => Some("docker-compose".to_string()),
            "gh" => Some("github-cli".to_string()),
            _ => Some(canonical.to_string()),
        }
    }

    fn configure_local_mirror(&self, ctx: &mut installer_core::PhaseContext) -> anyhow::Result<()> {
        // Arch Mirror Heuristic: Check for local proxy or cache
        // If pacman.conf has XferCommand or if we can detect a local proxy
        let mut cmd = std::process::Command::new("curl");
        cmd.args(["-I", "-s", "--connect-timeout", "1", "http://localhost:3142"]);
        
        if let Ok(output) = ctx.platform.system.command_output(&mut cmd) {
            if output.status.success() {
                ctx.record_action("Mirror Heuristics: Local pacman-compatible proxy detected at localhost:3142");
                
                // For Arch, we'd typically use 'XferCommand' in pacman.conf or a proxy env
                // We'll record the intent here as Arch mirror management is usually more manual
                ctx.record_tweaked("Mirror Heuristics: Local proxy detected. Manual pacman.conf 'XferCommand' optimization recommended.");
            }
        }
        Ok(())
    }
}

pub static ARCH_DRIVER: ArchDriver = ArchDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &ARCH_DRIVER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_translate_to_base_devel() {
        assert_eq!(
            ARCH_DRIVER.translate_package("build-essential"),
            Some("base-devel".to_string())
        );
    }

    #[test]
    fn docker_package_maps_to_docker() {
        assert_eq!(
            ARCH_DRIVER.translate_package("docker-ce"),
            Some("docker".to_string())
        );
    }

    #[test]
    fn unsupported_package_returns_none() {
        assert!(ARCH_DRIVER
            .translate_package("apt-transport-https")
            .is_none());
    }
}
