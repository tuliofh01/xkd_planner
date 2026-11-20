#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="$ROOT_DIR/logs"
STAMP="$(date +%Y%m%d-%H%M%S)"
LOG_FILE="$LOG_DIR/dev-$STAMP.log"

mkdir -p "$LOG_DIR"

if [[ -f "$ROOT_DIR/.env" ]]; then
  set -a
  # shellcheck disable=SC1091
  . "$ROOT_DIR/.env"
  set +a
fi

export RUST_LOG="${RUST_LOG:-info}"
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"
export DATABASE_URL="${DATABASE_URL:-sqlite:xkdplanner.db}"

echo ">> workspace : $ROOT_DIR"
echo ">> logs      : $LOG_FILE"
echo ">> database  : $DATABASE_URL"

cd "$ROOT_DIR"

cleanup() {
  if [[ -n "${VITE_PID:-}" ]] && kill -0 "$VITE_PID" >/dev/null 2>&1; then
    kill "$VITE_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT INT TERM

if command -v cargo-tauri >/dev/null 2>&1 || cargo tauri --version >/dev/null 2>&1; then
  echo ">> starting Tauri dev (cargo tauri dev)"
  cargo tauri dev --config "$ROOT_DIR/view/desktop/tauri.conf.json" 2>&1 | tee "$LOG_FILE"
  exit ${PIPESTATUS[0]}
fi

echo ">> cargo-tauri not found; starting Vite and cargo run separately" >&2
if command -v npm >/dev/null 2>&1; then
  npm --prefix "$ROOT_DIR/view/web" run dev 2>&1 | tee "$LOG_DIR/vite-$STAMP.log" &
  VITE_PID=$!
else
  echo "Error: npm is required for frontend dev." >&2
  exit 1
fi

cargo run --manifest-path "$ROOT_DIR/view/desktop/Cargo.toml" 2>&1 | tee "$LOG_FILE"
exit ${PIPESTATUS[0]}
