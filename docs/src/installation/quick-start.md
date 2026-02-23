# Quick Start

## One-Line Install (Recommended)

```bash
bash <(curl -L https://raw.githubusercontent.com/drtweak86/Mash-installer/main/install.sh)
```

This downloads and runs the POSIX-compatible bootstrap script (`install.sh`), which:
1. Detects your distribution and architecture
2. Downloads the correct pre-built `mash-setup` binary for your system
3. Launches the TUI installer

## Manual Binary Download

If you prefer to download the binary directly:

```bash
# aarch64 (Raspberry Pi 4B, ARM servers)
curl -LO https://github.com/drtweak86/Mash-installer/releases/latest/download/mash-setup-aarch64

# x86_64 (most desktop/server Linux)
curl -LO https://github.com/drtweak86/Mash-installer/releases/latest/download/mash-setup-x86_64

chmod +x mash-setup-*
sudo mv mash-setup-* /usr/local/bin/mash-setup
mash-setup
```

## Headless / SSH Mode

For remote machines or CI environments without a TTY:

```bash
mash-setup --no-tui
```

Falls back to indicatif progress bars — all the same phases, no TUI.

## Non-Interactive Mode

Fully scripted install with zero prompts, using defaults (Developer profile):

```bash
mash-setup --non-interactive
```

Combine with `--profile` to override:

```bash
mash-setup --non-interactive --profile minimal
```

## Dry-Run First

Always recommended on a new system:

```bash
mash-setup --dry-run
```

Simulates all phases, logs what *would* happen, no packages installed.
