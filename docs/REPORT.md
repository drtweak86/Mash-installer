# mash-installer- audit report

## Overview
- Repo: `/work/drtweak86/Mash-installer-`
- Entry points:
  - `bootstrap.sh` downloads the latest GitHub release asset, optionally verifies a `.sha256`, installs to `/usr/local/bin/mash-setup`, then runs `mash-setup install`.
  - `src/main.rs` defines the CLI and orchestrates install phases via `pkg`, `rust`, `github`, `buildroot`, `docker`, `zsh`, `fonts`, `rclone`, and optional `argon`.
- Installer flow:
  1. Detect platform (distro family, arch, optional Pi model).
  2. Resolve staging dir (config/CLI), check space, create.
  3. Build phase list by profile/flags.
  4. Run phases sequentially (idempotent checks per phase).
  5. Print post-install notes.
- Safety guarantees stated: idempotent package installs, config backups, pacman `--needed`, staging space guard on root, dry-run support.

## Risks (High/Medium/Low)
- High
  - Bootstrap chain of trust is weak: downloads “latest” release and executes it without a mandatory checksum or signature. If `.sha256` is missing, it proceeds anyway. `bootstrap.sh` also uses unauthenticated GitHub API for latest tag (rate-limit risk).
  - Multiple phases execute unpinned remote scripts (`rustup`, `oh-my-zsh`, `starship`, `rclone`, Argon40 OEM script) or git clones. Any upstream change can alter behavior or break installs.
- Medium
  - Docker apt repo is hard-coded to the Ubuntu repo URL even when running on Debian; Debian users may get incompatible packages or failures.
  - `docker` data-root config overwrites existing `/etc/docker/daemon.json` if JSON parsing fails (e.g., comments). In that case existing settings are silently dropped.
  - `pacman -Sy` without a full upgrade can lead to partial-upgrade issues on Arch-based systems.
  - `--enable-ollama` flag is accepted but no install phase exists; users may assume it installed something when it did not.
- Low
  - `bootstrap.sh` installs into `/usr/local/bin` without checking for existing binaries; it always overwrites.
  - `zsh` / `oh-my-zsh` modifies `.zshrc`; backups are made for starship/p10k changes but not for the OEM `oh-my-zsh` script’s edits.

## Missing safety checks & idempotency gaps
- No mandatory signature or checksum verification for downloaded binaries or scripts. `.sha256` is optional and not enforced.
- No explicit check that `sudo` is available or that the user can authenticate non-interactively. Many commands will block on password prompts.
- Staging dir space check only runs if the parent exists; a user-provided `--staging-dir` under a non-existent mount (e.g., `/mnt/data/...` when `/mnt/data` is missing) will create directories on the root filesystem without the root-space safeguard.
- `docker` data-root changes are applied unconditionally when the flag is set, without verifying disk space on the target path.
- No rollbacks or transactional behavior; partial installs can leave the system in a mixed state if a later phase fails.

## Determinism issues
- `bootstrap.sh` pulls the latest GitHub release tag and the matching asset at runtime, which changes over time.
- `rustup update` runs on every install, and `cargo install` uses unpinned latest crates. Results are version-dependent.
- `oh-my-zsh`, `starship`, `rclone`, and Argon scripts are fetched from `master`/latest without pinning.
- `powerlevel10k` and `argononed` are git-cloned from HEAD.
- Package manager installs are unpinned; results vary with repo state.

## ARM/Raspberry Pi pitfalls
- `bootstrap.sh` only supports `aarch64/arm64` and `x86_64/amd64`. 32‑bit Pi OS (`armv7l`) is unsupported and will hard-fail.
- Docker repo setup assumes Ubuntu; many Pi images are Debian-based, increasing failure risk.
- Argon One installation assumes Pi hardware and systemd; non-systemd Pi distros will fail to enable services.
- `flamegraph` install is gated to full profile but not to x86_64; on Pi, perf-based tooling often requires extra kernel config.

## Docker/container assumptions
- Uses `systemctl` to enable and start services; this fails in containers, WSL, or minimal systems without systemd.
- Assumes `/etc/apt/keyrings` and `/etc/apt/sources.list.d` are present and writable.
- Assumes `sudo` is available; root containers without `sudo` will fail.

## Recommended changes
- Add mandatory integrity checks for the release binary:
  - Require `.sha256` and fail if missing, or
  - Provide a signed manifest (GPG/cosign) and verify it.
- Pin external scripts and git clones to specific versions or commits; offer a `--latest` override if desired.
- Add a Docker repo path selection based on distro (`ubuntu` vs `debian`), with a clear error if unsupported.
- Make staging space checks run even when parent directories don’t exist; resolve the mount point before creating.
- Respect `docker.data_root` from config, and validate available space before writing.
- Implement or remove the `--enable-ollama` flag to avoid misleading behavior.
- Add tests:
  - Unit tests for package translation, platform detection, and staging resolution.
  - Dry-run integration tests that simulate phase execution without system changes.
  - Shell tests for `bootstrap.sh` (e.g., with `bats` or `shellcheck` plus mocked `curl`).
