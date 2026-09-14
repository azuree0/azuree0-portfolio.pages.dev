#!/usr/bin/env bash
# Launch the newest built AppImage (after scripts/make-linux-test.sh).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Electron AppImage runtime libs (Ubuntu / WSL guest).
if command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update -qq
  sudo apt-get install -y -qq libnss3 libnspr4 libatk1.0-0 libatk-bridge2.0-0 \
    libcups2 libdrm2 libgtk-3-0 libgbm1 libasound2t64 libxkbcommon0 2>/dev/null \
    || sudo apt-get install -y -qq libnss3 libnspr4 libatk1.0-0 libatk-bridge2.0-0 \
    libcups2 libdrm2 libgtk-3-0 libgbm1 libasound2 libxkbcommon0
fi

APPIMAGE="$(find out/make -type f -name '*.AppImage' | sort | tail -n 1)"

if [ -z "$APPIMAGE" ]; then
  echo "No AppImage found. Run: bash scripts/make-linux-test.sh"
  exit 1
fi

chmod +x "$APPIMAGE"
echo "Running: $APPIMAGE"

EXTRA_ARGS=()
if [ "$(id -u)" -eq 0 ]; then
  # WSL default user is often root; Chromium requires --no-sandbox in that case.
  EXTRA_ARGS+=(--no-sandbox)
fi

exec "$APPIMAGE" "${EXTRA_ARGS[@]}" "$@"
