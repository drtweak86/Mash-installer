# MASH Installer

**Mythic Assembly & Sigil Heuristics** — a Rust-based system installer for Linux.

MASH provisions a complete development environment on Arch/Manjaro, Debian/Ubuntu, or Fedora —
with first-class support for Raspberry Pi 4B.

---

## What MASH Does

- Installs and configures development tools, shell polish, containers, and AI coding assistants
- Runs a full-screen cyberpunk TUI cockpit (Ratatui) with real-time telemetry
- Applies Raspberry Pi 4B-specific HDD/SSD kernel and mount optimisations
- Downloads curated wallpapers from Wallhaven, Pexels, and Pixabay
- Supports dry-run mode — see exactly what would happen before committing
- Ships as a single statically-linked binary for `aarch64` and `x86_64`

## One-Line Install

```bash
bash <(curl -L https://raw.githubusercontent.com/drtweak86/Mash-installer/main/install.sh)
```

## Key Subcommands

| Command                      | Purpose                                      |
|------------------------------|----------------------------------------------|
| `mash-setup`                 | Launch TUI installer                         |
| `mash-setup --dry-run`       | Simulate all phases (no writes)              |
| `mash-setup --no-tui`        | stdio fallback (SSH / headless)              |
| `mash-setup doctor`          | Full system diagnostic                       |
| `mash-setup status`          | Instant status overview (no network)         |
| `mash-setup catalog`         | Browse the software catalog                  |
| `mash-setup config init`     | Create default config file                   |
| `mash-setup config show`     | Print current config                         |

---

*Forged by the Drunken Dwarf Bard — Forge Tavern, Neon District.*
