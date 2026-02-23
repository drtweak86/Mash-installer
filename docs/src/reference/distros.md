# Supported Distributions

## Distribution Matrix

| Distro           | Backend | Status       | Notes                          |
|------------------|---------|-------------|--------------------------------|
| Arch Linux       | pacman  | ✅ Supported |                                |
| Manjaro          | pacman  | ✅ Supported |                                |
| Debian 11+       | apt     | ✅ Supported | Incl. Debian 13 (trixie)       |
| Ubuntu 22.04+    | apt     | ✅ Supported |                                |
| Raspberry Pi OS  | apt     | ✅ Supported | Debian base + Pi 4B tuning     |
| Fedora 38+       | dnf     | ✅ Supported |                                |

## Architectures

| Architecture  | Support |
|---------------|---------|
| `x86_64`      | ✅ Full |
| `aarch64`     | ✅ Full (Raspberry Pi 4B primary target) |

The `aarch64` binary is cross-compiled with `cargo-zigbuild` on `x86_64` CI runners and
verified on real Raspberry Pi hardware.

## Distribution Detection

MASH reads `/etc/os-release` to detect the distro at runtime and selects the appropriate
package manager backend (`apt`, `pacman`, or `dnf`).

```bash
mash-setup doctor
# ── System ──
#   OS: Raspberry Pi OS GNU/Linux 12 (bookworm)
```

## Package Manager Backends

### apt (Debian/Ubuntu/Raspberry Pi OS)

- Uses `apt-get` for package operations
- Adds external repositories via `add-apt-repository` where needed (Docker, Rust)
- Runs `apt-get update` before package installs

### pacman (Arch/Manjaro)

- Uses `pacman -S --noconfirm` for package installs
- Syncs `--refresh` before operations

### dnf (Fedora)

- Uses `dnf install -y`
- Handles copr repositories for packages not in the default repos

## Unsupported Distributions

On unsupported distros, `mash-setup doctor` shows:

```
── Pre-flight ──
  [WARN] OS — unrecognised distro ID (MASH may not work correctly)
```

MASH will still attempt to run but no guarantees are made. Contributions for additional
distros are welcome.
