//! System Profile — The machine's true pedigree.
//!
//! This module defines the data structures for auto-detection and system profiling.
//! It serves as the single source of truth for the machine's hardware, OS, and storage landscape.

use serde::{Deserialize, Serialize};

/// The complete pedigree of the machine we are inhabiting.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemProfile {
    pub platform: PlatformInfo,
    pub distro: DistroInfo,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub session: SessionInfo,
    pub storage: StorageInfo,
    pub timestamp: u64,
}

/// Hardware platform classification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum PlatformType {
    #[default]
    Unknown,
    RaspberryPi,
    GenericArm,
    PC,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformInfo {
    pub platform_type: PlatformType,
    pub model: String,
    pub board_revision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DistroInfo {
    pub id: String,
    pub version: String,
    pub pretty_name: String,
    pub family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuInfo {
    pub model: String,
    pub arch: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub flags: Vec<String>,
    pub bogomips: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryInfo {
    pub ram_total_kb: u64,
    pub ram_avail_kb: u64,
    pub swap_total_kb: u64,
    pub zram_total_kb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionInfo {
    pub desktop_environment: String,
    pub window_manager: String,
    pub session_type: String, // x11, wayland, tty
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageInfo {
    pub devices: Vec<BlockDevice>,
    pub mounts: Vec<MountInfo>,
    pub btrfs_data: Option<BtrfsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlockDevice {
    pub name: String,
    pub type_name: String, // disk, part, etc.
    pub size_bytes: u64,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MountInfo {
    pub device: String,
    pub destination: String,
    pub fstype: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BtrfsData {
    pub has_subvolumes: bool,
    pub root_is_btrfs: bool,
    pub subvolumes: Vec<String>,
}

impl SystemProfile {
    /// Create a new skeleton profile.
    pub fn new() -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            ..Default::default()
        }
    }

    /// Stub for detection logic (Phase 2).
    pub fn detect(_system: &dyn crate::SystemOps) -> anyhow::Result<Self> {
        // This will be implemented in EX_S02 and EX_S03
        Ok(Self::new())
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_serialization() {
        let mut profile = SystemProfile::new();
        profile.platform.platform_type = PlatformType::RaspberryPi;
        profile.platform.model = "Raspberry Pi 4 Model B Rev 1.5".to_string();

        let json = profile.to_json().expect("Failed to serialize");
        assert!(json.contains("RaspberryPi"));

        let deserialized: SystemProfile =
            SystemProfile::from_json(&json).expect("Failed to deserialize");
        assert_eq!(
            deserialized.platform.platform_type,
            PlatformType::RaspberryPi
        );
        assert_eq!(
            deserialized.platform.model,
            "Raspberry Pi 4 Model B Rev 1.5"
        );
    }
}
