//! Legacy font installation module (deprecated)
//!
//! This module has been replaced by the comprehensive fonts_all.rs module.
//! It is kept for backward compatibility and will be removed in future versions.

use crate::fonts_all;

/// Legacy font installation phase (now uses fonts_all)
pub fn install_phase(ctx: &mut crate::PhaseContext) -> anyhow::Result<()> {
    fonts_all::install_phase(ctx)
}
