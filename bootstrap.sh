#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────
#  mash-installer – Slim Bootstrap (Binary Download Mode) 🚀
# ─────────────────────────────────────────────────────────────────
set -euo pipefail

# Detect architecture and map to target triple
ARCH=$(uname -m)
case "$ARCH" in
    x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
    armv7l) TARGET="armv7-unknown-linux-gnueabihf" ;;
    *) echo "Error: Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

# Download pre-built binary from latest GitHub Release
RELEASE_URL="https://github.com/drtweak86/Mash-installer-/releases/latest/download"
BINARY_URL="$RELEASE_URL/mash-setup-$TARGET"
CHECKSUM_URL="$RELEASE_URL/mash-setup-$TARGET.sha256"

# Download and verify binary
echo "Downloading mash-setup binary for $TARGET..."
TMP_BIN=$(mktemp)
TMP_CHECKSUM=$(mktemp)

curl -sSfL "$BINARY_URL" -o "$TMP_BIN"
curl -sSfL "$CHECKSUM_URL" -o "$TMP_CHECKSUM"

# Verify checksum
EXPECTED_CHECKSUM=$(cut -d' ' -f1 "$TMP_CHECKSUM")
ACTUAL_CHECKSUM=$(sha256sum "$TMP_BIN" | cut -d' ' -f1)

if [ "$EXPECTED_CHECKSUM" != "$ACTUAL_CHECKSUM" ]; then
    echo "Error: Checksum verification failed!" >&2
    rm -f "$TMP_BIN" "$TMP_CHECKSUM"
    exit 1
fi

# Make executable and run
chmod +x "$TMP_BIN"
exec "$TMP_BIN" "$@"
