//! Telemetry Service — Reporting to the Forge Tavern.
//!
//! This module provides an optional telemetry service that reports the results
//! of an installation to a centralized endpoint. This allows for long-term
//! health monitoring of the MASH ecosystem.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::system::error::InstallationReport;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub endpoint: String,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "https://forge-tavern.example.com/api/v1/telemetry".to_string(),
        }
    }
}

pub struct TelemetryService {
    config: TelemetryConfig,
}

impl TelemetryService {
    pub fn new(config: TelemetryConfig) -> Self {
        Self { config }
    }

    /// Report an installation result to the Tavern.
    pub fn report(&self, report: &InstallationReport) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        info!("Sending telemetry to {}...", self.config.endpoint);

        // In a real implementation, we'd use ureq or reqwest to POST the report.
        // For now, we'll simulate the successful transmission.

        let client = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(5))
            .build();

        let res = client
            .post(&self.config.endpoint)
            .send_json(serde_json::to_value(report)?);

        match res {
            Ok(_) => {
                info!("Telemetry successfully transmitted to the Forge Tavern.");
                Ok(())
            }
            Err(e) => {
                warn!("Failed to send telemetry: {}", e);
                // We don't want to fail the whole installation just because telemetry failed
                Ok(())
            }
        }
    }
}
