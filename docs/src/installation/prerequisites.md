# Prerequisites

## Supported Operating Systems

| Distro           | Status       | Notes                         |
|------------------|-------------|-------------------------------|
| Arch / Manjaro   | ✅ Supported | pacman backend                |
| Debian / Ubuntu  | ✅ Supported | apt backend, incl. Debian 13  |
| Fedora           | ✅ Supported | dnf backend                   |
| Raspberry Pi OS  | ✅ Supported | Debian base + Pi tuning        |

## Architectures

| Architecture | Status       |
|--------------|-------------|
| `x86_64`     | ✅ Supported |
| `aarch64`    | ✅ Supported |

## Minimum Requirements

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| RAM      | 2 GiB   | 3 GiB+      |
| CPU cores| 1       | 2+          |
| Disk     | 4 GiB free | 10 GiB+  |

## Required Tools (pre-installed on most distros)

- `curl` — for downloading the install script and wallpapers
- `git` — for cloning repos during phase installation
- `tar` — for archive extraction

Run `mash-setup doctor` to verify all requirements are met before installing.

## Connectivity

An internet connection is required during installation. Doctor mode checks connectivity;
`mash-setup status` provides an instant overview without network checks.
