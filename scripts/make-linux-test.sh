#!/usr/bin/env bash
# Build Linux desktop installers for local test (.deb + .AppImage).
# Use on Ubuntu VM, WSL2, or Cursor Cloud Agent (Linux microVM).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update -qq
  sudo apt-get install -y -qq squashfs-tools
fi

export NO_COLOR=false

npm run build:electron
npm run build:wasm
npx electron-forge make \
  --targets='@electron-forge/maker-deb,@reforged/maker-appimage'

echo ""
echo "Linux test artifacts:"
find out/make -type f \( -name '*.deb' -o -name '*.AppImage' \) 2>/dev/null || true
