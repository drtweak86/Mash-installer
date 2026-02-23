# Common Errors

## HALTED Status in TUI

If the TUI shows a **HALTED** status:

1. **READ** the `ERROR` line in the TUI log pane
2. **LOCATE** the full trace: `~/mash-install.log`
3. **FOLLOW** the `FIX` suggestion shown by the station
4. **RETRY** the phase with `--dry-run` first

## "No distro drivers available"

```
Error: No distro drivers available!
Recompile with at least one feature: arch, debian, or fedora
```

You are running a binary built without distro support. Download the correct binary from the
releases page for your distribution family.

## Permission Denied / sudo Failures

MASH prompts for sudo in the TUI. If sudo fails:

- Ensure your user is in the `sudo` (Debian/Ubuntu) or `wheel` (Arch/Fedora) group
- Run `sudo -v` first to cache credentials

## Package Manager Not Found

```
[FAIL] package manager — none of apt/pacman/dnf/yum found
```

Running on an unsupported distro. Check `mash-setup doctor` output for the full diagnosis.

## Docker Group Membership

After installation, Docker commands fail with "permission denied":

```bash
# Add yourself to the docker group (done by MASH, but requires re-login)
newgrp docker
# or log out and back in
```

## Wallpaper Download Failures

If wallpaper downloads fail silently:
- Check API keys: `mash-setup status`
- Verify connectivity: `mash-setup doctor`
- API rate limits may apply — wait and retry

## Config File Invalid

```
── Configuration ──
  Config:   ~/.config/mash/mash.toml [invalid]
```

The TOML is malformed. Fix it with a text editor or reset:

```bash
rm ~/.config/mash/mash.toml
mash-setup config init
```

## Installation Interrupted

If a run is interrupted mid-phase, phases already completed will not re-run (idempotent design).
Re-run `mash-setup` — it picks up where it left off.
