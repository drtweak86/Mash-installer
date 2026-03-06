use installer_core::{DistroDriver, PkgBackend, PlatformInfo};

pub struct FedoraDriver;

impl DistroDriver for FedoraDriver {
    fn name(&self) -> &'static str {
        "Fedora/RHEL"
    }

    fn description(&self) -> &'static str {
        "Fedora/RHEL/CentOS/Rocky/AlmaLinux with dnf backend"
    }

    fn matches(&self, info: &PlatformInfo) -> bool {
        info.distro_family == "fedora"
    }

    fn pkg_backend(&self) -> PkgBackend {
        PkgBackend::Dnf
    }

    fn translate_package(&self, canonical: &str) -> Option<String> {
        match canonical {
            "software-properties-common" | "apt-transport-https" | "lsb-release" => None,
            "g++" => Some("gcc-c++".to_string()),
            "xz-utils" => Some("xz".to_string()),
            "python3-pip" => Some("python3-pip".to_string()),
            "borgbackup" => Some("borgbackup".to_string()),
            "wireguard" => Some("wireguard-tools".to_string()),
            "fd-find" => Some("fd-find".to_string()),
            "libncurses-dev" => Some("ncurses-devel".to_string()),
            "libssl-dev" => Some("openssl-devel".to_string()),
            "openssh-client" => Some("openssh-clients".to_string()),
            "fonts-terminus" => Some("terminus-fonts".to_string()),
            "fonts-noto-color-emoji" => Some("google-noto-emoji-color-fonts".to_string()),
            "docker-ce" => Some("docker".to_string()),
            "docker-ce-cli" => None,
            "containerd.io" => Some("containerd".to_string()),
            "docker-buildx-plugin" => Some("docker-buildx".to_string()),
            "docker-compose-plugin" => Some("docker-compose".to_string()),
            "gh" => Some("gh".to_string()),
            _ => Some(canonical.to_string()),
        }
    }

    fn configure_local_mirror(&self, ctx: &mut installer_core::PhaseContext) -> anyhow::Result<()> {
        // Fedora DNF Mirror Heuristic
        let mut cmd = std::process::Command::new("curl");
        cmd.args([
            "-I",
            "-s",
            "--connect-timeout",
            "1",
            "http://localhost:3142",
        ]);

        if let Ok(output) = ctx.platform.system.command_output(&mut cmd) {
            if output.status.success() {
                ctx.record_action(
                    "Mirror Heuristics: Local dnf-compatible proxy detected at localhost:3142",
                );

                // For Fedora, we can add 'proxy=http://localhost:3142' to dnf.conf
                let proxy_line = "proxy=http://localhost:3142";
                let conf_path = "/etc/dnf/dnf.conf";

                if ctx.options.dry_run {
                    ctx.record_dry_run(
                        "mirror_heuristics",
                        "Would configure dnf proxy",
                        Some(conf_path.to_string()),
                    );
                } else {
                    let mut append_cmd = std::process::Command::new("sh");
                    append_cmd
                        .arg("-c")
                        .arg(format!("echo '{}' | sudo tee -a {}", proxy_line, conf_path));
                    ctx.platform.system.command_output(&mut append_cmd)?;
                    ctx.record_tweaked("Configured local dnf proxy in /etc/dnf/dnf.conf");
                }
            }
        }
        Ok(())
    }
}

pub static FEDORA_DRIVER: FedoraDriver = FedoraDriver;

pub fn driver() -> &'static dyn DistroDriver {
    &FEDORA_DRIVER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpp_translates_to_gcc_cxx() {
        assert_eq!(
            FEDORA_DRIVER.translate_package("g++"),
            Some("gcc-c++".to_string())
        );
    }

    #[test]
    fn optional_packages_translate_to_none() {
        assert!(FEDORA_DRIVER.translate_package("docker-ce-cli").is_none());
    }
}
