mod apt;
mod argon;
mod buildroot;
mod config;
mod docker;
mod doctor;
mod fonts;
mod github;
mod platform;
mod rclone;
mod rust;
mod staging;
mod zsh;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tracing::info;

/// Mash Installer – idempotent mega-installer for Raspberry Pi / Ubuntu dev machines.
#[derive(Parser)]
#[command(name = "mash-setup", version, about)]
struct Cli {
    /// Enable verbose logging (RUST_LOG override)
    #[arg(long, short, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the full installation
    Install {
        /// Installation profile
        #[arg(long, default_value = "dev")]
        profile: Profile,

        /// Override staging directory
        #[arg(long)]
        staging_dir: Option<PathBuf>,

        /// Dry run – print what would happen without executing
        #[arg(long)]
        dry_run: bool,

        /// Enable interactive prompts (default is unattended)
        #[arg(long)]
        interactive: bool,

        /// Enable Ollama installation (off by default on ARM)
        #[arg(long)]
        enable_ollama: bool,

        /// Enable Argon One fan script installation
        #[arg(long)]
        enable_argon: bool,

        /// Set Docker data-root to staging_dir/docker
        #[arg(long)]
        docker_data_root: bool,
    },
    /// Diagnose the current system state
    Doctor,
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Write default config to ~/.config/mash-installer/config.toml
    Init,
    /// Print the current configuration
    Show,
}

#[derive(Clone, ValueEnum)]
enum Profile {
    /// Minimal: essential build tools + git only
    Minimal,
    /// Dev: full developer workstation (default)
    Dev,
    /// Full: everything including optional components
    Full,
}

/// Central context threaded through every install phase.
pub struct InstallContext {
    pub profile: ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_ollama: bool,
    pub enable_argon: bool,
    pub docker_data_root: bool,
    pub mp: MultiProgress,
    pub config: config::MashConfig,
    pub platform: platform::PlatformInfo,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProfileLevel {
    Minimal = 0,
    Dev = 1,
    Full = 2,
}

impl InstallContext {
    /// Create a spinner-style progress bar attached to the MultiProgress.
    pub fn phase_spinner(&self, msg: &str) -> ProgressBar {
        let pb = self.mp.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
        );
        pb.set_message(msg.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(120));
        pb
    }

    /// Finish a spinner with a checkmark.
    pub fn finish_phase(pb: &ProgressBar, msg: &str) {
        pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
        pb.set_prefix("✓");
        pb.finish_with_message(msg.to_string());
    }

    /// Finish a spinner indicating it was skipped.
    pub fn skip_phase(pb: &ProgressBar, msg: &str) {
        pb.set_style(ProgressStyle::with_template("{prefix} {msg}").unwrap());
        pb.set_prefix("–");
        pb.finish_with_message(msg.to_string());
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialise tracing
    let filter = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .without_time()
        .init();

    match cli.command {
        Commands::Install {
            profile,
            staging_dir,
            dry_run,
            interactive,
            enable_ollama,
            enable_argon,
            docker_data_root,
        } => {
            let profile_level = match profile {
                Profile::Minimal => ProfileLevel::Minimal,
                Profile::Dev => ProfileLevel::Dev,
                Profile::Full => ProfileLevel::Full,
            };

            run_install(
                profile_level,
                staging_dir,
                dry_run,
                interactive,
                enable_ollama,
                enable_argon,
                docker_data_root,
            )
        }
        Commands::Doctor => doctor::run_doctor(),
        Commands::Config { action } => match action {
            ConfigAction::Init => config::init_config(),
            ConfigAction::Show => config::show_config(),
        },
    }
}

fn run_install(
    profile: ProfileLevel,
    staging_dir_override: Option<PathBuf>,
    dry_run: bool,
    interactive: bool,
    enable_ollama: bool,
    enable_argon: bool,
    docker_data_root: bool,
) -> Result<()> {
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
    if let Some(ref model) = plat.pi_model {
        info!("Raspberry Pi model: {}", model);
    }

    let cfg = config::load_or_default()?;
    let staging = staging::resolve(staging_dir_override.as_deref(), &cfg)?;
    info!("Staging directory: {}", staging.display());

    let mp = MultiProgress::new();

    let ctx = InstallContext {
        profile,
        staging_dir: staging,
        dry_run,
        interactive,
        enable_ollama,
        enable_argon,
        docker_data_root,
        mp,
        config: cfg,
        platform: plat,
    };

    // ── Phase 1: APT core packages ──────────────────────────────
    let pb = ctx.phase_spinner("Phase 1/9 · APT core packages");
    apt::install_phase(&ctx)?;
    InstallContext::finish_phase(&pb, "APT core packages installed");

    // ── Phase 2: Rust toolchain ─────────────────────────────────
    let pb = ctx.phase_spinner("Phase 2/9 · Rust toolchain + cargo tools");
    rust::install_phase(&ctx)?;
    InstallContext::finish_phase(&pb, "Rust toolchain ready");

    // ── Phase 3: GitHub / Git / SSH ─────────────────────────────
    let pb = ctx.phase_spinner("Phase 3/9 · Git, GitHub CLI, SSH");
    github::install_phase(&ctx)?;
    InstallContext::finish_phase(&pb, "Git / GitHub CLI ready");

    // ── Phase 4: Buildroot deps ─────────────────────────────────
    if ctx.profile >= ProfileLevel::Dev {
        let pb = ctx.phase_spinner("Phase 4/9 · Buildroot dependencies");
        buildroot::install_phase(&ctx)?;
        InstallContext::finish_phase(&pb, "Buildroot dependencies ready");
    }

    // ── Phase 5: Docker ─────────────────────────────────────────
    if ctx.profile >= ProfileLevel::Dev {
        let pb = ctx.phase_spinner("Phase 5/9 · Docker Engine");
        docker::install_phase(&ctx)?;
        InstallContext::finish_phase(&pb, "Docker Engine ready");
    }

    // ── Phase 6: Shell / UX ─────────────────────────────────────
    if ctx.profile >= ProfileLevel::Dev {
        let pb = ctx.phase_spinner("Phase 6/9 · Shell & UX (zsh, starship)");
        zsh::install_phase(&ctx)?;
        InstallContext::finish_phase(&pb, "Shell & UX ready");
    }

    // ── Phase 7: Fonts ──────────────────────────────────────────
    if ctx.profile >= ProfileLevel::Dev {
        let pb = ctx.phase_spinner("Phase 7/9 · Fonts");
        fonts::install_phase(&ctx)?;
        InstallContext::finish_phase(&pb, "Fonts installed");
    }

    // ── Phase 8: rclone ─────────────────────────────────────────
    if ctx.profile >= ProfileLevel::Dev {
        let pb = ctx.phase_spinner("Phase 8/9 · rclone");
        rclone::install_phase(&ctx)?;
        InstallContext::finish_phase(&pb, "rclone ready");
    }

    // ── Phase 9: Argon One (optional) ───────────────────────────
    if ctx.enable_argon {
        let pb = ctx.phase_spinner("Phase 9/9 · Argon One fan script");
        argon::install_phase(&ctx)?;
        InstallContext::finish_phase(&pb, "Argon One installed");
    }

    // ── Summary ─────────────────────────────────────────────────
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       Installation complete!                  ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    if ctx.dry_run {
        println!("(dry-run mode – no changes were made)");
        println!();
    }

    // Post-install notes
    println!("Post-install notes:");
    println!("  - Log out and back in for docker group membership to take effect.");
    println!("  - Run `mash-setup doctor` to verify everything.");
    println!("  - Config lives at ~/.config/mash-installer/config.toml");
    if ctx.platform.arch == "aarch64" && !ctx.enable_ollama {
        println!("  - Ollama was skipped (ARM). Re-run with --enable-ollama to install.");
    }
    println!();

    Ok(())
}
