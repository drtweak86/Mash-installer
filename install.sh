#!/bin/sh
# MASH Installer - The Gate & Guardian
# (C) 1984 Mythic Assembly & Sigil Heuristics
# Strictly POSIX-compliant for maximum compatibility across station nodes.

set -e

# ── Dependency Checks ─────────────────────────────────────────────────────────

check_dependencies() {
  for dep in curl uname mktemp chmod; do
    if ! command -v "$dep" >/dev/null 2>&1; then
      printf "HALTED: Missing essential rune: %s\n" "$dep" >&2
      exit 1
    fi
  done
}

# ── Architecture Detection ────────────────────────────────────────────────────

detect_arch() {
  raw_arch=$(uname -m)
  case "$raw_arch" in
    x86_64) echo "x86_64" ;;
    aarch64|arm64) echo "aarch64" ;;
    *)
      printf "HALTED: Unknown architecture sigil: %s\n" "$raw_arch" >&2
      exit 1
      ;;
  esac
}

# ── Main ──────────────────────────────────────────────────────────────────────

printf "COMMENCING INITIALIZATION...\n"
check_dependencies

target_arch=$(detect_arch)
printf "STATION_01: ARCHITECTURE_DETECTED: %s\n" "$target_arch"

# 15s Grace Period
printf "PRESS CTRL+C WITHIN 15s TO ABORT OR MANUAL SELECT...\n"
i=15
while [ "$i" -gt 0 ]; do
  printf "\rPROCEEDING IN %2d SECONDS... " "$i"
  sleep 1
  i=$((i - 1))
done
printf "\rPROCEEDING NOW.               \n"

file="mash-setup-${target_arch}-unknown-linux-gnu"
url="https://github.com/drtweak86/Mash-installer/releases/latest/download/${file}"

staging_dir=$(mktemp -d)
trap 'rm -rf "${staging_dir}"' EXIT INT TERM

printf "FETCHING RUNES FROM FORGE...\n"
curl -fsSL -o "${staging_dir}/${file}" "${url}"
chmod +x "${staging_dir}/${file}"

printf "BOOTING MASH OPERATING SYSTEM...\n"
exec "${staging_dir}/${file}" --arch "${target_arch}" "$@"
