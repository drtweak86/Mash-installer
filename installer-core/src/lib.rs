mod apt_repo;
mod argon;
mod backend;
mod buildroot;
mod config;
mod context;
mod distro;
mod docker;
mod doctor;
mod driver;
mod fonts;
mod github;
mod package_manager;
mod pkg;
mod platform;
mod rclone;
mod rust;
mod staging;
mod systemd;
mod zsh;

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tracing::{error, info};

pub use backend::PkgBackend;
use context::{OptionsContext, PlatformContext, UiContext};
pub use driver::{AptRepoConfig, DistroDriver, RepoKind, ServiceName};
pub use platform::{detect as detect_platform, PlatformInfo};

/// Options provided by the CLI that drive `run_with_driver`.
pub struct InstallOptions {
    pub profile: ProfileLevel,
    pub staging_dir: Option<PathBuf>,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub options: OptionsContext,
    pub platform: PlatformContext,
    pub ui: UiContext,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfileLevel {
    Minimal = 0,
    Dev = 1,
    Full = 2,
}

impl InstallContext {
    /// Create a spinner-style progress bar attached to the MultiProgress.
    pub fn phase_spinner(&self, msg: &str) -> ProgressBar {
        let pb = self
            .ui
            .mp
            .insert_before(&self.ui.overall, ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        pb.set_message(msg.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(120));
        pb
    }

    /// Finish a spinner with a checkmark and advance the overall bar.
    pub fn finish_phase(&self, pb: &ProgressBar, msg: &str) {
        pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
        pb.set_prefix("✓");
        pb.finish_with_message(msg.to_string());
        self.ui.overall.inc(1);
    }

    /// Finish a spinner indicating it was skipped and advance the overall bar.
    pub fn skip_phase(&self, pb: &ProgressBar, msg: &str) {
        pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
        pb.set_prefix("–");
        pb.finish_with_message(msg.to_string());
        self.ui.overall.inc(1);
    }
}

/// Run the installer using the supplied distro driver and CLI options.
pub fn run_with_driver(driver: &'static dyn DistroDriver, opts: InstallOptions) -> Result<()> {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       mash-setup · mega installer            ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    let plat = platform::detect()?;
    info!(
        "Platform: {} {} on {}",
        plat.distro, plat.distro_version, plat.arch
    );
    info!(
        "Using distro driver: {} ({})",
        driver.name(),
        driver.description()
    );
    if let Some(ref model) = plat.pi_model {
        info!("Raspberry Pi model: {}", model);
    }

    let cfg = config::load_or_default()?;
    let staging = staging::resolve(opts.staging_dir.as_deref(), &cfg)?;
    info!("Staging directory: {}", staging.display());

    let profile = opts.profile;
    let enable_argon = opts.enable_argon;
    let enable_p10k = opts.enable_p10k;
    let docker_data_root = opts.docker_data_root;

    let mut phases: Vec<Box<dyn Phase>> = vec![
        Box::new(FunctionPhase::new(
            "System packages",
            "System packages installed",
            pkg::install_phase,
        )),
        Box::new(FunctionPhase::new(
            "Rust toolchain + cargo tools",
            "Rust toolchain ready",
            rust::install_phase,
        )),
        Box::new(FunctionPhase::new(
            "Git, GitHub CLI, SSH",
            "Git / GitHub CLI ready",
            github::install_phase,
        )),
    ];

    if profile >= ProfileLevel::Dev {
        phases.push(Box::new(FunctionPhase::new(
            "Buildroot dependencies",
            "Buildroot dependencies ready",
            buildroot::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "Docker Engine",
            "Docker Engine ready",
            docker::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "Shell & UX (zsh, starship)",
            "Shell & UX ready",
            zsh::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "Fonts",
            "Fonts installed",
            fonts::install_phase,
        )));
        phases.push(Box::new(FunctionPhase::new(
            "rclone",
            "rclone ready",
            rclone::install_phase,
        )));
    }

    if enable_argon {
        phases.push(Box::new(FunctionPhase::new(
            "Argon One fan script",
            "Argon One installed",
            argon::install_phase,
        )));
    }

    let total = phases.len() as u64;

    let mp = MultiProgress::new();
    let overall = mp.add(ProgressBar::new(total));
    overall.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{bar:30.green/dim}] {pos}/{len} phases  {percent}%  ETA {eta}  [{elapsed}]",
        )
        .unwrap()
        .progress_chars("━╸─")
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    overall.enable_steady_tick(std::time::Duration::from_millis(200));

    let options = OptionsContext {
        profile,
        staging_dir: staging,
        dry_run: opts.dry_run,
        interactive: opts.interactive,
        enable_argon,
        enable_p10k,
        docker_data_root,
    };

    let platform_ctx = PlatformContext {
        config: cfg,
        platform: plat,
        driver_name: driver.name(),
        driver,
        pkg_backend: driver.pkg_backend(),
    };

    let ui = UiContext { mp, overall };

    let ctx = InstallContext {
        options,
        platform: platform_ctx,
        ui,
    };

    let mut completed_phases = Vec::new();
    for (i, phase) in phases.iter().enumerate() {
        let label = format!("Phase {}/{} · {}", i + 1, total, phase.label());
        let pb = ctx.phase_spinner(&label);
        match phase.execute(&ctx) {
            Ok(()) => {
                ctx.finish_phase(&pb, phase.done_msg());
                completed_phases.push(phase.label());
            }
            Err(e) => {
                pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
                pb.set_prefix("✗");
                pb.finish_with_message(format!("{} FAILED: {e:#}", phase.label()));
                ctx.ui.overall.inc(1);
                let completed = if completed_phases.is_empty() {
                    "none".to_string()
                } else {
                    completed_phases.join(", ")
                };
                error!(
                    "Installation aborted during {} (staging dir: {}). Completed phases: {}. \
                     Rerun `mash-setup doctor` or remove the staging directory before retrying.",
                    phase.label(),
                    ctx.options.staging_dir.display(),
                    completed
                );
                return Err(e);
            }
        }
    }

    ctx.ui.overall.finish_and_clear();

    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       Installation complete!                  ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    if ctx.options.dry_run {
        println!("(dry-run mode – no changes were made)");
        println!();
    }

    println!("Post-install notes:");
    println!("  - Log out and back in for docker group membership to take effect.");
    println!("  - Run `mash-setup doctor` to verify everything.");
    println!("  - Config lives at ~/.config/mash-installer/config.toml");
    println!();

    Ok(())
}

pub trait Phase {
    fn label(&self) -> &'static str;
    fn done_msg(&self) -> &'static str;
    fn should_run(&self, _: &OptionsContext) -> bool {
        true
    }
    fn execute(&self, ctx: &InstallContext) -> Result<()>;
}

pub struct FunctionPhase {
    label: &'static str,
    done_msg: &'static str,
    run: fn(&InstallContext) -> Result<()>,
}

impl Phase for FunctionPhase {
    fn label(&self) -> &'static str {
        self.label
    }

    fn done_msg(&self) -> &'static str {
        self.done_msg
    }

    fn execute(&self, ctx: &InstallContext) -> Result<()> {
        (self.run)(ctx)
    }
}

impl FunctionPhase {
    pub fn new(
        label: &'static str,
        done_msg: &'static str,
        run: fn(&InstallContext) -> Result<()>,
    ) -> Self {
        Self {
            label,
            done_msg,
            run,
        }
    }
}
