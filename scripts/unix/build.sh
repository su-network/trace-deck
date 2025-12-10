#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust not found. Installing via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

echo "Building trace-deck..."
cargo build --release

echo ""
echo "Build complete. Binary: $REPO_ROOT/target/release/trace-deck"
echo "Examples:"
echo "  $REPO_ROOT/target/release/trace-deck info"
echo "  $REPO_ROOT/target/release/trace-deck process sample.pdf --timing"
