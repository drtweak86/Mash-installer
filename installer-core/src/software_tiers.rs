//! Software tier data model and installation phase.
//!
//! This module owns two concerns that belong in the **core** crate:
//! - **Data model**: [`SoftwareTierPlan`] and [`ThemePlan`] — the user's selections,
//!   constructed by the UI but used by the engine.
//! - **Installation logic**: [`install_phase`], which executes the actual
//!   package manager calls and theme application steps.

use anyhow::Result;
use std::collections::BTreeSet;

use crate::context::PhaseContext;
use crate::model::options::EnvironmentTag;
use crate::model::software::{SoftwareTierPlan, Tier};
use crate::package_manager;
use crate::PhaseResult;

/// Software installation phase — the primary payload of the installer.
pub fn install_phase(ctx: &mut PhaseContext<'_>) -> Result<PhaseResult> {
    let plan = ctx.options.software_plan.clone();

    if plan.is_empty() {
        return Ok(PhaseResult::Success);
    }

    ctx.record_action(format!(
        "Executing software installation for tier: {:?}",
        plan.target_tier.unwrap_or(Tier::S)
    ));

    // 1. Collect all packages to install
    let mut required = BTreeSet::new();
    let optional = BTreeSet::new();

    // Add explicit selections
    for programs in plan.selections.values() {
        for prog_id in programs {
            required.insert(prog_id.clone());
        }
    }

    // 2. Resolve Tier dependencies if targeted
    if let Some(tier) = plan.target_tier {
        let catalog = if tier == Tier::S {
            crate::catalog::Catalog::load_s_tier().unwrap_or_default()
        } else {
            crate::catalog::Catalog::load_full().unwrap_or_default()
        };

        let tiers_to_include = tier.resolve();
        for cat in &catalog.categories {
            for sub in &cat.subcategories {
                for prog in &sub.programs {
                    if tiers_to_include.contains(&prog.tier) {
                        required.insert(prog.id.clone());
                    }
                }
            }
        }
    }

    // 3. Apply dynamic heuristics (Bard's Recommendations)
    apply_heuristics(ctx, &mut required)?;

    // 4. Mirror Heuristics (Zero-HTTP strategy)
    ctx.platform.driver.configure_local_mirror(ctx)?;

    install_packages(ctx, &required, &optional)?;

    // 5. Apply Theme Plan
    if plan.theme_plan != crate::model::software::ThemePlan::None {
        let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/root"));
        crate::theme::install_retro_theme(&home, ctx.options.dry_run)?;
    }

    // Post-install configurations (Auth, etc.)
    if ctx.options.interactive {
        handle_interactive_auth(ctx, &plan)?;
    }

    Ok(PhaseResult::Success)
}

fn apply_heuristics(ctx: &mut PhaseContext, required: &mut BTreeSet<String>) -> Result<()> {
    let Some(_profile) = &ctx.options.system_profile else {
        tracing::debug!("No system profile available for heuristics");
        return Ok(());
    };

    let distro_family = &ctx.platform.platform.distro_family;

    // 1. Pi 4B Optimizations
    if ctx.platform.is_pi_4b() {
        // Enforce ZRAM for memory constrained Pi
        if distro_family == "debian" {
            if !required.contains("zram-tools") {
                required.insert("zram-tools".to_string());
                ctx.record_tweaked("Heuristics: Added 'zram-tools' for Pi 4B memory optimization");
            }
        } else if distro_family == "arch" || distro_family == "fedora" {
            // Arch/Fedora usually use zram-generator
            if !required.contains("zram-generator") {
                required.insert("zram-generator".to_string());
                ctx.record_tweaked(
                    "Heuristics: Added 'zram-generator' for Pi 4B memory optimization",
                );
            }
        }
    }

    // 2. Low RAM Warnings (Only for recommended mode to avoid nagging manual users)
    if ctx.options.software_plan.target_tier.is_some() {
        let ram_gb = _profile.memory.ram_total_kb as f32 / (1024.0 * 1024.0);
        if ram_gb < 7.5 {
            let heavy_apps = ["vscode", "discord", "slack", "teams"];
            for app_id in ctx.options.software_plan.selections.values().flatten() {
                if heavy_apps.contains(&app_id.as_str()) {
                    ctx.record_warning(format!(
                        "Heuristics: '{}' identified on low-RAM system ({:.1}GB). Performance may be throttled.",
                        app_id, ram_gb
                    ));
                }
            }
        }
    }

    // 3. Network Awareness
    let net = &_profile.network;
    if !net.online {
        ctx.record_warning(
            "Heuristics: System appears to be OFFLINE. Network-heavy tasks may fail.",
        );
    } else if let Some(latency) = net.latency_ms {
        if latency > 200.0 {
            ctx.record_warning(format!(
                "Heuristics: High network latency detected ({:.0}ms). Downloads may be slow.",
                latency
            ));
        }
    }

    // 4. Environment-aware Heuristics (Roaming Agent feature)
    if ctx.options.environment == EnvironmentTag::Traveling {
        ctx.record_warning(
            "Heuristics: 'Traveling' environment identified. Postponing heavy background harvests.",
        );
    }

    Ok(())
}

fn handle_interactive_auth(ctx: &mut PhaseContext, plan: &SoftwareTierPlan) -> Result<()> {
    let all_selected_ids: BTreeSet<_> = plan.selections.values().flatten().collect();

    let has_borg = all_selected_ids.contains(&"borgbackup".to_string());
    if has_borg
        && ctx.interaction.confirm(
            "borg_init",
            "Would you like to initialize a Borg backup repository now?",
            true,
            || {
                Ok(ctx
                    .observer
                    .confirm("Would you like to initialize a Borg backup repository now?"))
            },
        )?
    {
        ctx.record_action("Initializing Borg backup repository...");
        // Trigger actual borg init logic here
    }

    Ok(())
}

fn install_packages(
    ctx: &mut PhaseContext,
    required: &BTreeSet<String>,
    optional: &BTreeSet<String>,
) -> Result<()> {
    if required.is_empty() && optional.is_empty() {
        return Ok(());
    }

    let req_refs: Vec<&str> = required.iter().map(String::as_str).collect();
    package_manager::ensure_packages(ctx.platform.driver, &req_refs, ctx.options.dry_run)?;

    for opt in optional {
        package_manager::try_optional(ctx.platform.driver, opt, ctx.options.dry_run);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::PlatformContext;
    use crate::profile::SystemProfile;
    use crate::system::dry_run::DryRunLog;
    use crate::{
        InstallContext, InstallOptions, Localization, PhaseObserver, RollbackManager, UIContext,
        UserOptionsContext,
    };
    use std::collections::BTreeSet;

    struct TestObserver;
    impl PhaseObserver for TestObserver {
        fn on_event(&mut self, _event: crate::PhaseEvent) {}
    }

    struct TestDriver;
    impl crate::DistroDriver for TestDriver {
        fn name(&self) -> &'static str {
            "test"
        }
        fn description(&self) -> &'static str {
            "test"
        }
        fn matches(&self, _: &crate::PlatformInfo) -> bool {
            true
        }
        fn pkg_backend(&self) -> crate::PkgBackend {
            crate::PkgBackend::Apt
        }
    }
    static TEST_DRIVER: TestDriver = TestDriver;

    fn mock_context(pi_model: Option<&str>, distro_family: &str) -> (InstallContext, TestObserver) {
        let platform = crate::platform::PlatformInfo {
            arch: "aarch64".into(),
            distro: "test".into(),
            distro_version: "0".into(),
            distro_codename: "test".into(),
            distro_family: distro_family.into(),
            pi_model: pi_model.map(|s| s.to_string()),
            cpu_model: "test".into(),
            cpu_cores: 4,
            ram_total_gb: 8.0,
        };
        let opts = InstallOptions {
            system_profile: Some(SystemProfile::default()),
            ..InstallOptions::default()
        };

        let platform_ctx = PlatformContext {
            config_service: crate::context::ConfigService::load().unwrap(),
            platform,
            driver_name: "test",
            driver: &TEST_DRIVER,
            pkg_backend: crate::PkgBackend::Apt,
            system: &crate::sys_ops::REAL_SYSTEM,
        };

        let staging_dir = std::path::PathBuf::from("/tmp/mash-test");
        let cache = crate::ArtifactCache::new(&staging_dir);

        let ctx = InstallContext {
            options: UserOptionsContext::from_options(&opts),
            platform: platform_ctx,
            ui: UIContext,
            interaction: crate::interaction::InteractionService::new(false, Default::default()),
            localization: Localization::load_default().unwrap(),
            rollback: RollbackManager::new(),
            dry_run_log: DryRunLog::new(),
            cache,
        };
        (ctx, TestObserver)
    }

    #[test]
    fn test_pi4b_heuristics_adds_zram_debian() {
        let (ctx, mut observer) = mock_context(Some("Raspberry Pi 4 Model B"), "debian");
        let mut p_ctx = ctx.phase_context(&mut observer);
        let mut required = BTreeSet::new();

        apply_heuristics(&mut p_ctx, &mut required).unwrap();

        assert!(required.contains("zram-tools"));
    }

    #[test]
    fn test_pi4b_heuristics_adds_zram_arch() {
        let (ctx, mut observer) = mock_context(Some("Raspberry Pi 4 Model B"), "arch");
        let mut p_ctx = ctx.phase_context(&mut observer);
        let mut required = BTreeSet::new();

        apply_heuristics(&mut p_ctx, &mut required).unwrap();

        assert!(required.contains("zram-generator"));
    }

    #[test]
    fn test_non_pi_heuristics_does_nothing() {
        let (ctx, mut observer) = mock_context(None, "debian");
        let mut p_ctx = ctx.phase_context(&mut observer);
        let mut required = BTreeSet::new();

        apply_heuristics(&mut p_ctx, &mut required).unwrap();

        assert!(required.is_empty());
    }

    #[test]
    fn test_offline_heuristics_records_warning() {
        let (mut ctx, mut observer) = mock_context(None, "debian");
        if let Some(profile) = &mut ctx.options.system_profile {
            profile.network.online = false;
        }
        let mut p_ctx = ctx.phase_context(&mut observer);
        let mut required = BTreeSet::new();

        apply_heuristics(&mut p_ctx, &mut required).unwrap();

        let metadata = p_ctx.take_metadata();
        assert!(metadata.warnings.iter().any(|w| w.contains("OFFLINE")));
    }

    #[test]
    fn test_high_latency_heuristics_records_warning() {
        let (mut ctx, mut observer) = mock_context(None, "debian");
        if let Some(profile) = &mut ctx.options.system_profile {
            profile.network.online = true;
            profile.network.latency_ms = Some(500.0);
        }
        let mut p_ctx = ctx.phase_context(&mut observer);
        let mut required = BTreeSet::new();

        apply_heuristics(&mut p_ctx, &mut required).unwrap();

        let metadata = p_ctx.take_metadata();
        assert!(metadata
            .warnings
            .iter()
            .any(|w| w.contains("High network latency")));
    }
}
