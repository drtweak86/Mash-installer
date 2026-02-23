# Status Subcommand

`mash-setup status` provides an instant overview of your system — no network calls, no delay.

```bash
mash-setup status              # pretty output
mash-setup status --format json  # JSON (CI-friendly)
```

## What It Reports

- **Platform**: distro name, architecture, Pi model (if applicable)
- **Configuration**: config file path + state (loaded / missing / invalid)
- **Wallpaper API keys**: PASS/WARN per provider (Wallhaven, Pexels, Pixabay)
- **Pre-flight summary**: pass/warn/fail counts from 7 fast checks

## Example Output

```
mash-setup status
=================

── Platform ──
  Distro:   Raspberry Pi OS GNU/Linux 12 (bookworm)
  Arch:     aarch64
  Pi:       Raspberry Pi 4 Model B Rev 1.5

── Configuration ──
  Config:   /home/user/.config/mash/mash.toml [loaded]

── Wallpaper API keys ──
  Wallhaven    PASS
  Pexels       WARN (not set)
  Pixabay      WARN (not set)

── Pre-flight summary (fast checks) ──
  pass: 5  warn: 2  fail: 0
```

## The 7 Fast Checks

| Check          | PASS condition           | FAIL / WARN condition         |
|----------------|--------------------------|-------------------------------|
| `curl`         | found in PATH            | not found → FAIL              |
| `git`          | found in PATH            | not found → FAIL              |
| `tar`          | found in PATH            | not found → FAIL              |
| Memory         | ≥ 3 GiB available        | < 2 GiB → FAIL, < 3 GiB → WARN |
| CPU cores      | ≥ 2 cores                | 1 core → WARN                 |
| Package mgr    | apt/pacman/dnf found     | none found → FAIL             |
| OS support     | recognised distro ID     | unknown → WARN                |

## JSON Output

```bash
mash-setup status --format json
```

```json
{
  "platform": { "distro": "...", "arch": "aarch64", "pi_model": "..." },
  "config": { "path": "~/.config/mash/mash.toml", "state": "loaded" },
  "wallpaper_keys": [
    { "provider": "Wallhaven", "env_var": "MASH_WALLHAVEN_KEY", "present": true }
  ],
  "preflight": { "pass": 5, "warn": 2, "fail": 0 }
}
```

Run `mash-setup doctor` for the full diagnostic report including connectivity checks.
