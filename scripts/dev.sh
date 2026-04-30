#!/usr/bin/env bash
# Quick dev launcher: brings up Postgres, then runs RAKIT in watch mode.
set -euo pipefail

docker compose up -d
export $(grep -v '^#' .env | xargs)

if command -v cargo-watch >/dev/null 2>&1; then
  cargo watch -x run
else
  echo "Tip: install cargo-watch for hot reload: cargo install cargo-watch"
  cargo run
fi
