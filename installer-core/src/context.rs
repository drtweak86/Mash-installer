use std::path::{Path, PathBuf};

use crate::backend::PkgBackend;
use crate::config::{self, ConfigError};
use crate::driver::DistroDriver;
use crate::dry_run::DryRunLog;
use crate::localization::Localization;
use crate::platform::PlatformInfo;
use crate::rollback::RollbackManager;
use crate::staging;
use anyhow::Result;

/// CLI-supplied options that guide the installation.
pub struct UserOptionsContext {
    pub profile: crate::ProfileLevel,
    pub staging_dir: PathBuf,
    pub dry_run: bool,
    pub interactive: bool,
    pub enable_argon: bool,
    pub enable_p10k: bool,
    pub docker_data_root: bool,
}

/// Options that override values in the persisted Mash config.
#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    pub staging_dir: Option<PathBuf>,
}

impl ConfigOverrides {
    pub fn staging_dir(&self) -> Option<&Path> {
        self.staging_dir.as_deref()
    }
}

/// Service that loads and exposes the Mash config along with any overrides.
pub struct ConfigService {
    config: config::MashConfig,
    overrides: ConfigOverrides,
    config_path: PathBuf,
}

impl ConfigService {
    pub fn load() -> std::result::Result<Self, ConfigError> {
        Self::load_with_overrides(ConfigOverrides::default())
    }

    pub fn load_with_overrides(
        overrides: ConfigOverrides,
    ) -> std::result::Result<Self, ConfigError> {
        let path = config::config_path();
        let config = config::load_or_default()?;
        Ok(Self {
            config,
            overrides,
            config_path: path,
        })
    }

    pub fn config(&self) -> &config::MashConfig {
        &self.config
    }

    pub fn overrides(&self) -> &ConfigOverrides {
        &self.overrides
    }

    pub fn staging_override(&self) -> Option<&Path> {
        self.overrides.staging_dir()
    }

    pub fn resolve_staging_dir(&self) -> Result<PathBuf> {
        staging::resolve(self.staging_override(), self.config())
    }

    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
}

/// Platform-specific data shared across phases.
pub struct PlatformContext {
    pub config_service: ConfigService,
    pub platform: PlatformInfo,
    pub driver_name: &'static str,
    pub driver: &'static dyn DistroDriver,
    pub pkg_backend: PkgBackend,
}

impl PlatformContext {
    pub fn config(&self) -> &config::MashConfig {
        self.config_service.config()
    }

    /// Return the detected board model string if available.
    pub fn pi_model(&self) -> Option<&str> {
        self.platform.pi_model.as_deref()
    }

    /// Is the detected device some variant of Raspberry Pi 4? We consider "Pi 4"/"Raspberry Pi 4" matches as 4B units.
    pub fn is_pi_4b(&self) -> bool {
        self.pi_model()
            .map(|model| model.contains("Raspberry Pi 4") || model.contains("Pi 4"))
            .unwrap_or(false)
    }

    /// Expose the raw platform metadata for phases that need additional probes.
    pub fn platform_info(&self) -> &PlatformInfo {
        &self.platform
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn overrides_default_to_none() {
        let overrides = ConfigOverrides::default();
        assert!(overrides.staging_dir().is_none());
    }

    #[test]
    fn staging_override_returns_specified_path() {
        let path = PathBuf::from("/tmp/custom-stage");
        let overrides = ConfigOverrides {
            staging_dir: Some(path.clone()),
        };
        assert_eq!(overrides.staging_dir(), Some(path.as_path()));
    }

    #[test]
    fn config_service_resolves_overrides_internally() {
        let config = config::MashConfig::default();
        let overrides = ConfigOverrides {
            staging_dir: Some(PathBuf::from("/tmp/custom-stage")),
        };
        let service = ConfigService {
            config: config.clone(),
            overrides,
            config_path: config::config_path(),
        };
        assert_eq!(service.config(), &config);
        assert!(service.staging_override().is_some());
    }
}

/// UI-related data that may become necessary for rendering progress or logging.
#[derive(Default)]
pub struct UIContext;

/// Combined contexts passed to individual phases.
pub struct PhaseContext<'a> {
    pub options: &'a UserOptionsContext,
    pub platform: &'a PlatformContext,
    pub ui: &'a UIContext,
    pub localization: &'a Localization,
    pub rollback: &'a RollbackManager,
    pub dry_run_log: &'a DryRunLog,
    actions_taken: Vec<String>,
    rollback_actions: Vec<String>,
}

impl<'a> PhaseContext<'a> {
    pub fn new(
        options: &'a UserOptionsContext,
        platform: &'a PlatformContext,
        ui: &'a UIContext,
        localization: &'a Localization,
        rollback: &'a RollbackManager,
        dry_run_log: &'a DryRunLog,
    ) -> Self {
        PhaseContext {
            options,
            platform,
            ui,
            localization,
            rollback,
            dry_run_log,
            actions_taken: Vec::new(),
            rollback_actions: Vec::new(),
        }
    }

    /// Register a rollback action associated with the provided label.
    pub fn register_rollback_action(
        &mut self,
        label: impl Into<String>,
        action: impl Fn() -> Result<()> + 'static,
    ) {
        let label = label.into();
        self.rollback_actions.push(label.clone());
        self.rollback.register_action(label, action);
    }

    /// Record an action that should be represented in `PhaseOutput`.
    pub fn record_action(&mut self, action: impl Into<String>) {
        self.actions_taken.push(action.into());
    }

    pub fn record_dry_run(
        &self,
        phase: impl Into<String>,
        action: impl Into<String>,
        detail: Option<String>,
    ) {
        if self.options.dry_run {
            self.dry_run_log.record(phase, action, detail);
        }
    }

    pub fn run_or_record<F>(
        &mut self,
        phase: impl Into<String>,
        action: impl Into<String>,
        detail: Option<String>,
        work: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut PhaseContext<'a>) -> Result<()>,
    {
        if self.options.dry_run {
            self.record_dry_run(phase, action, detail);
            Ok(())
        } else {
            work(self)
        }
    }

    pub fn take_metadata(&mut self) -> PhaseMetadata {
        PhaseMetadata {
            actions_taken: std::mem::take(&mut self.actions_taken),
            rollback_actions: std::mem::take(&mut self.rollback_actions),
            dry_run: self.options.dry_run,
        }
    }
}

/// Collected metadata that each phase can report to the runner.
pub struct PhaseMetadata {
    pub actions_taken: Vec<String>,
    pub rollback_actions: Vec<String>,
    pub dry_run: bool,
}
