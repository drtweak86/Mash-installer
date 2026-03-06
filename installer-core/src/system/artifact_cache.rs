//! Artifact Cache — The Forge's Hoard.
//!
//! This module provides a local artifact caching service to minimize external network
//! dependencies. It stores downloaded files (wallpapers, themes, scripts) and ensures
//! that if we've already fetched an artifact, we reuse the local copy.
//!
//! **Principles**:
//! - **Hash-based verification**: Ensure cached artifacts match their expected pedigree.
//! - **Zero-HTTP preference**: Always check the hoard before reaching for the net.
//! - **Staging integration**: Leverages the staging directory as the base for the cache.

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

pub struct ArtifactCache {
    base_dir: PathBuf,
}

impl ArtifactCache {
    /// Create a new cache anchored in the provided staging directory.
    pub fn new(staging_dir: &Path) -> Self {
        let base_dir = staging_dir.join("cache/artifacts");
        Self { base_dir }
    }

    /// Ensure the cache directory exists.
    pub fn init(&self) -> Result<()> {
        if !self.base_dir.exists() {
            fs::create_dir_all(&self.base_dir).with_context(|| {
                format!("creating artifact cache at {}", self.base_dir.display())
            })?;
        }
        Ok(())
    }

    /// Resolve the path for a specific artifact key (e.g., "wallpapers/retro_pack.zip").
    pub fn resolve_path(&self, key: &str) -> PathBuf {
        self.base_dir.join(key)
    }

    /// Check if an artifact exists and matches the provided SHA-256 hash.
    /// If hash is None, only existence is checked.
    pub fn exists(&self, key: &str, expected_hash: Option<&str>) -> bool {
        let path = self.resolve_path(key);
        if !path.exists() {
            return false;
        }

        if let Some(expected) = expected_hash {
            match self.verify_hash(&path, expected) {
                Ok(matches) => matches,
                Err(e) => {
                    debug!("Hash verification failed for {}: {}", key, e);
                    false
                }
            }
        } else {
            true
        }
    }

    /// Put a file into the cache.
    pub fn put(&self, key: &str, source: &Path) -> Result<PathBuf> {
        let dest = self.resolve_path(key);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(source, &dest).with_context(|| {
            format!(
                "caching artifact from {} to {}",
                source.display(),
                dest.display()
            )
        })?;

        info!("Artifact cached: {}", key);
        Ok(dest)
    }

    /// Verify the SHA-256 hash of a file.
    fn verify_hash(&self, path: &Path, expected: &str) -> Result<bool> {
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let hash = format!("{:x}", hasher.finalize());
        Ok(hash == expected)
    }

    /// Clear the entire hoard.
    pub fn clear(&self) -> Result<()> {
        if self.base_dir.exists() {
            fs::remove_dir_all(&self.base_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_artifact_cache_flow() -> Result<()> {
        let staging = tempdir()?;
        let cache = ArtifactCache::new(staging.path());
        cache.init()?;

        let source_file = staging.path().join("source.txt");
        fs::write(&source_file, "the forge is hot")?;

        let key = "docs/motto.txt";
        cache.put(key, &source_file)?;

        assert!(cache.exists(key, None));

        // Expected SHA-256 of "the forge is hot"
        let expected = "4c57d1fcee6b9074f5b82a82bfd726d319b5222040595cf9d7ab1ab457185274";
        assert!(cache.exists(key, Some(expected)));

        // Wrong hash
        assert!(!cache.exists(key, Some("wrong")));

        Ok(())
    }
}
