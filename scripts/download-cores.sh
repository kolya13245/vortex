#!/bin/bash
# Download Mihomo and Xray-core binaries for the current platform
set -e

# Configuration
MIHOMO_VERSION="v1.19.10"
XRAY_VERSION="v25.3.6"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  x86_64)  ARCH_MIHOMO="amd64"; ARCH_XRAY="64" ;;
  aarch64) ARCH_MIHOMO="arm64"; ARCH_XRAY="arm64-v8a" ;;
  *)       echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
  linux)  OS_MIHOMO="linux"; OS_XRAY="linux" ;;
  darwin) OS_MIHOMO="darwin"; OS_XRAY="macos" ;;
  *)      echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Determine output directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DATA_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/vortex/bin"
# Also copy to src-tauri/bin for development
DEV_DIR="$SCRIPT_DIR/../src-tauri/bin"

mkdir -p "$DATA_DIR" "$DEV_DIR"

echo "=== Downloading Mihomo ${MIHOMO_VERSION} ==="
MIHOMO_URL="https://github.com/MetaCubeX/mihomo/releases/download/${MIHOMO_VERSION}/mihomo-${OS_MIHOMO}-${ARCH_MIHOMO}-${MIHOMO_VERSION}.gz"
echo "URL: $MIHOMO_URL"
curl -L "$MIHOMO_URL" -o /tmp/mihomo.gz
gunzip -f /tmp/mihomo.gz
chmod +x /tmp/mihomo
cp /tmp/mihomo "$DATA_DIR/mihomo"
cp /tmp/mihomo "$DEV_DIR/mihomo"
echo "Mihomo installed to $DATA_DIR/mihomo and $DEV_DIR/mihomo"

echo ""
echo "=== Downloading Xray-core ${XRAY_VERSION} ==="
XRAY_URL="https://github.com/XTLS/Xray-core/releases/download/${XRAY_VERSION}/Xray-${OS_XRAY}-${ARCH_XRAY}.zip"
echo "URL: $XRAY_URL"
curl -L "$XRAY_URL" -o /tmp/xray.zip
cd /tmp && unzip -o xray.zip xray -d /tmp/xray-extract 2>/dev/null || unzip -o xray.zip -d /tmp/xray-extract
chmod +x /tmp/xray-extract/xray
cp /tmp/xray-extract/xray "$DATA_DIR/xray"
cp /tmp/xray-extract/xray "$DEV_DIR/xray"
echo "Xray installed to $DATA_DIR/xray and $DEV_DIR/xray"

echo ""
echo "=== Done ==="
echo "Mihomo: $(${DATA_DIR}/mihomo -v 2>&1 | head -1)"
echo "Xray:   $(${DATA_DIR}/xray version 2>&1 | head -1)"
echo ""
echo "Binaries installed to:"
echo "  Runtime: $DATA_DIR/"
echo "  Dev:     $DEV_DIR/"
