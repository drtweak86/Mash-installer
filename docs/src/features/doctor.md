# Doctor Mode

`mash-setup doctor` runs a full system diagnostic and pre-flight check.

```bash
mash-setup doctor
```

## What It Checks

- **Pre-flight checks**: required commands, disk space, memory, internet connectivity
- **System info**: OS, kernel version, Pi model (if applicable)
- **Package manager**: detected backend (apt/pacman/dnf)
- **Installed tools**: which tools are present and their versions
- **Cargo tools**: rustup, cargo, clippy status
- **Wallpaper API keys**: PASS/WARN per provider with setup URLs
- **SSH keys**: inventory of keys in `~/.ssh/`
- **Config file**: path and state (loaded / missing / invalid)

## Example Output

```
mash-setup doctor
=================

── Pre-flight ──
  [PASS] curl — found
  [PASS] git — found
  [PASS] internet — reachable
  [PASS] disk space — 12.4 GiB free
  [WARN] memory — 2.1 GiB available (recommended: 3 GiB+)

── System ──
  OS:       Raspberry Pi OS (Debian 12)
  Arch:     aarch64
  Pi:       Raspberry Pi 4 Model B Rev 1.5
  Kernel:   6.12.62+rpt-rpi-v8

── Wallpaper API keys ──
  Wallhaven    PASS
  Pexels       WARN — set MASH_PEXELS_KEY (https://www.pexels.com/api/new/)
  Pixabay      WARN — set MASH_PIXABAY_KEY (https://pixabay.com/api/docs/#api_key)
```

## JSON Output

For CI or scripted parsing:

```bash
mash-setup doctor --format json
```

Returns a structured JSON document with all the same fields.

## vs Status

| Command              | Speed   | Network? | Detail |
|----------------------|---------|----------|--------|
| `mash-setup status`  | Instant | No       | Summary overview |
| `mash-setup doctor`  | ~15s    | Yes      | Full diagnostic  |

Use `status` for a quick sanity check; use `doctor` before a fresh install.
