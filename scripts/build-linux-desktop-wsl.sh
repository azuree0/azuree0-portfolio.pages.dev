#!/usr/bin/env bash
# Build Linux desktop installers inside WSL Ubuntu (faster than waiting for CI).
# Run from repo root via WSL: bash scripts/build-linux-desktop-wsl.sh

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export NO_COLOR=false

echo "[1/4] System deps ..."
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  build-essential curl unzip squashfs-tools libfuse2

echo "[2/4] Rust wasm target ..."
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi
rustup target add wasm32-unknown-unknown

echo "[3/4] Trunk + wasm-bindgen ..."
export PATH="$HOME/.cargo/bin:$PATH"
if ! command -v trunk >/dev/null 2>&1; then
  cargo install trunk --locked
fi
if ! command -v wasm-bindgen >/dev/null 2>&1; then
  cargo install wasm-bindgen-cli --version 0.2.108 --locked
fi

echo "[4/4] npm make (Linux) ..."
if ! command -v npm >/dev/null 2>&1; then
  echo "Install Node.js in WSL: sudo apt install -y nodejs npm"
  exit 1
fi
npm ci
npm run make

echo ""
echo "Artifacts under out/make/. Test with:"
echo "  cp out/make/**/*.AppImage linux-test-artifacts/ 2>/dev/null || true"
echo "  bash scripts/test-linux-desktop.sh"
