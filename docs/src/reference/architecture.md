# Architecture

## Overview

MASH is a Rust workspace with 7 crates:

```
mash-installer/
├── installer-cli/       # Thin binary: arg parsing, TUI, stdio UI
├── installer-core/      # Library: phases, orchestration, all core logic
├── installer-arch/      # Arch/Manjaro distro driver
├── installer-debian/    # Debian/Ubuntu/Pi OS distro driver
├── installer-fedora/    # Fedora distro driver
├── wallpaper-downloader/# Standalone async wallpaper binary
└── xtask/              # Developer tooling (cargo xtask)
```

## Binary

The released binary is `mash-setup` — a single statically-linked executable.

- **x86_64**: built directly by CI on `ubuntu-latest`
- **aarch64**: cross-compiled via `cargo-zigbuild` (Zig linker for static musl)

## Core Abstractions

### PhaseContext

`PhaseContext` is the single object threaded through every installation phase. It carries:
- User options (profile, dry-run flag, interactive mode)
- Platform info (distro, arch, Pi detection)
- UI handle (for progress reporting)
- Interaction service (sudo prompts)
- Rollback manager

### PhaseRunner

`PhaseRunner` executes phases from the `PhaseRegistry` in order. Each phase is a `Phase`
trait object that returns `PhaseOutput` or a `PhaseRunError`.

### Dry-Run Gate

All side effects go through `PhaseContext::run_or_record()`. In dry-run mode, calls are
recorded to the `DryRunLog` instead of executed. This is the single seam for all simulation.

### DistroDriver

Each distro crate implements the `DistroDriver` trait, providing package manager commands
and distro-specific configuration. The CLI selects the correct driver at runtime.

## TUI Architecture

The TUI lives in `installer-cli/src/tui/` — 8 modules:

| Module          | Purpose                                |
|-----------------|----------------------------------------|
| `app.rs`        | Application state machine              |
| `render.rs`     | Ratatui rendering logic                |
| `menus.rs`      | Menu definitions and navigation        |
| `theme.rs`      | Color scheme and styling               |
| `bbs.rs`        | BBS message bank (68 messages)         |
| `sysinfo_poller.rs` | Background CPU/RAM/net telemetry   |
| `observer.rs`   | Phase event → TUI update bridge        |
| `mod.rs`        | Module entry point                     |

## Config

```
~/.config/mash/mash.toml
```

Managed via `installer-core::config`. The `ConfigService` loads config at startup; phases
read it through `PhaseContext`.

## Logging

`tracing` crate with `tracing-subscriber`. Log level controlled by `--verbose` flag or
`RUST_LOG` env var. Writes to `~/mash-install.log`.

## Rollback

`RollbackManager` tracks reversible operations. On fatal error, MASH attempts to undo
completed steps. Rollback scope is limited to file operations — package installs are
not reversed.
