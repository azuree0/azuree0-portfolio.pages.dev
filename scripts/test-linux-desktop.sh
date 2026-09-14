#!/usr/bin/env bash
# Test Linux desktop installers (AppImage or .deb) on Ubuntu (WSL2 or VM).
# Run from repo root: bash scripts/test-linux-desktop.sh

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARTIFACTS="${ROOT}/linux-test-artifacts"

# Install runtime deps for Electron AppImage on Ubuntu.
install_deps() {
  if ! command -v apt-get >/dev/null 2>&1; then
    echo "apt-get not found; install libfuse2 manually for AppImage support."
    return
  fi
  sudo apt-get update -qq
  sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq libfuse2 fuse3 2>/dev/null || \
    sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq libfuse2
}

# Run the first AppImage found in linux-test-artifacts/.
test_appimage() {
  local appimage
  appimage="$(find "$ARTIFACTS" -maxdepth 1 -name '*.AppImage' -print -quit)"
  if [[ -z "$appimage" ]]; then
    return 1
  fi
  echo "Testing AppImage: $appimage"
  chmod +x "$appimage"
  "$appimage" --no-sandbox "$@"
}

# Install and launch the first .deb found in linux-test-artifacts/.
test_deb() {
  local deb
  deb="$(find "$ARTIFACTS" -maxdepth 1 -name '*.deb' -print -quit)"
  if [[ -z "$deb" ]]; then
    return 1
  fi
  echo "Installing .deb: $deb"
  sudo dpkg -i "$deb" || sudo apt-get install -y -f
  echo "Launch: azure-portfolio (or find binary in /usr/bin)"
  command -v azure-portfolio >/dev/null && azure-portfolio --no-sandbox "$@"
}

main() {
  if [[ ! -d "$ARTIFACTS" ]] || [[ -z "$(ls -A "$ARTIFACTS" 2>/dev/null)" ]]; then
    echo "No artifacts in $ARTIFACTS"
    echo "From Windows PowerShell: .\\scripts\\download-linux-desktop.ps1"
    echo "Or build in WSL: npm run make"
    exit 1
  fi

  install_deps

  if test_appimage "$@"; then
    exit 0
  fi

  if test_deb "$@"; then
    exit 0
  fi

  echo "No .AppImage or .deb found in $ARTIFACTS"
  ls -la "$ARTIFACTS"
  exit 1
}

main "$@"
