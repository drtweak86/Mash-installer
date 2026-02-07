# mash-installer

Idempotent mega-installer for Raspberry Pi 4 / Ubuntu 24.04 arm64 dev machines.
Also works on Ubuntu 22.04+ (amd64 or arm64).

## Overview

A two-layer installer:

1. **`bootstrap.sh`** – tiny bash script that installs minimal prerequisites,
   downloads the prebuilt `mash-setup` binary from GitHub Releases, and runs it.
2. **`mash-setup`** – Rust binary that performs the full idempotent installation.

## Quick start

```bash
# One-liner (downloads latest release and runs it):
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh | bash

# Or with options:
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer-/main/bootstrap.sh \
  | bash -s -- --profile dev --staging-dir /mnt/data/mash-installer
```

## Commands

```
mash-setup install [OPTIONS]
    --profile <dev|minimal|full>   Installation profile (default: dev)
    --staging-dir <PATH>           Override staging directory
    --dry-run                      Print what would happen without executing
    --interactive                  Enable interactive prompts
    --enable-ollama                Install Ollama (off by default on ARM)
    --enable-argon                 Install Argon One fan scripts
    --docker-data-root             Set Docker data-root to staging_dir/docker

mash-setup doctor
    Print diagnostic information about the system state.

mash-setup config init
    Write default config to ~/.config/mash-installer/config.toml

mash-setup config show
    Print the current configuration.
```

## Profiles

| Profile   | What it installs |
|-----------|-----------------|
| `minimal` | Core build tools, git, Rust toolchain |
| `dev`     | Everything in minimal + buildroot deps, Docker, zsh/starship, fonts, rclone, AI/scripting tools |
| `full`    | Everything in dev + Node.js/npm, flamegraph, optional extras |

## What gets installed

### All profiles
- **Build tools**: build-essential, pkg-config, clang, lld, cmake, ninja-build, gcc/g++, gdb, make
- **Rust**: rustup + stable toolchain, rustfmt, clippy, rust-src
- **Git**: git, git-lfs, gh (GitHub CLI), openssh-client

### Dev profile and above
- **Cargo tools**: cargo-edit, cargo-watch, cargo-audit, bacon, just, sccache
- **Buildroot deps**: bison, flex, gawk, texinfo, libncurses-dev, libssl-dev, bc, rsync, cpio, etc.
- **Docker**: docker-ce, docker-ce-cli, containerd.io, docker-buildx-plugin, docker-compose-plugin
- **Shell/UX**: zsh, oh-my-zsh (unattended), starship prompt
- **Fonts**: Terminus, Noto Color Emoji
- **AI/scripting tools**: python3 + venv + pip, ripgrep, fd-find, fzf, jq, yq
- **Terminal**: tmux, htop, btop, ncdu, neovim, bat, eza
- **rclone**: via apt or official script

### Full profile
- Node.js + npm
- flamegraph (cargo)

### Optional (flag-gated)
- **Ollama**: `--enable-ollama` (off by default on ARM)
- **Argon One**: `--enable-argon` (Raspberry Pi 4 Argon One case fan control)

## Configuration

Config file: `~/.config/mash-installer/config.toml`

```toml
staging_dir = "/var/tmp/mash-installer"

[agents]
larry = "/home/user/.config/mash-agents/larry"
moe = "/home/user/.config/mash-agents/moe"
claude = "/home/user/.config/mash-agents/claude"

[cache]
installer = "/home/user/.cache/mash-installer"
gh = "/home/user/.cache/gh"
cargo = "/home/user/.cache/cargo"
rustup = "/home/user/.cache/rustup"

[docker]
compose_plugin = true
# data_root = "/mnt/data/mash-installer/docker"  # optional

[git]
enforce_ssh = true
```

### Staging directory

The installer needs a staging area for downloads and temporary files. It will
refuse to stage on the root filesystem if free space is below 500 MiB.

Resolution order:
1. `--staging-dir` CLI flag
2. `staging_dir` from config file
3. Auto-detect: `/mnt/data/mash-installer` → `/data/mash-installer` → `/var/tmp/mash-installer`

### SSH-based GitHub auth

This installer enforces SSH-based GitHub authentication. It will **never**
convert your git remotes to HTTPS. Ensure you have an SSH key configured:

```bash
gh auth login  # select SSH when prompted
```

## Idempotency

Every phase checks before acting:
- `dpkg -s` to verify packages are installed
- `which` / file existence checks for binaries
- Config files are backed up before overwriting

Re-running `mash-setup install` is safe and will skip already-completed steps.

## Building from source

```bash
# Native build
cargo build --release

# Cross-compile for aarch64
cargo install cross --git https://github.com/cross-rs/cross
cross build --release --target aarch64-unknown-linux-gnu
```

## Project structure

```
├── bootstrap.sh              # Layer 1: bash bootstrap
├── Cargo.toml                # Rust project manifest
├── src/
│   ├── main.rs               # CLI wiring (clap)
│   ├── config.rs             # TOML config load/save
│   ├── platform.rs           # Distro/arch/Pi detection
│   ├── staging.rs            # Staging dir selection + space checks
│   ├── apt.rs                # apt wrapper with idempotent checks
│   ├── rust.rs               # rustup + cargo tools
│   ├── docker.rs             # Docker Engine install
│   ├── zsh.rs                # zsh + oh-my-zsh + starship
│   ├── fonts.rs              # Font installation
│   ├── github.rs             # Git, GitHub CLI, SSH
│   ├── buildroot.rs          # Buildroot dependencies
│   ├── rclone.rs             # rclone install
│   ├── argon.rs              # Argon One (optional)
│   └── doctor.rs             # System diagnostics
├── .github/
│   └── workflows/
│       ├── ci.yml            # Build + test + lint
│       └── release.yml       # Release artifacts on tags
└── README.md
```

## License

MIT
