use anyhow::Result;

use crate::InstallContext;

pub fn install_phase(ctx: &InstallContext) -> Result<()> {
    let pkgs = [
        "fonts-terminus",
        "fonts-noto-color-emoji",
        // xfonts-terminus is the X11 bitmap version; fonts-terminus is the console one.
        // Both are small; install both.
        "xfonts-terminus",
    ];

    crate::apt::ensure_packages(&pkgs, ctx.dry_run)?;

    // Nerd Fonts note: Ubuntu apt repos don't carry Nerd Fonts.
    // Users should install manually from https://www.nerdfonts.com/ or via a
    // helper script if they want patched fonts for terminal emulators.
    tracing::info!(
        "Nerd Fonts are not available via apt. \
         See https://www.nerdfonts.com/ to install patched terminal fonts."
    );

    Ok(())
}
