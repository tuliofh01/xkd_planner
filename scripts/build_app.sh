#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STAMP="$(date +%s%3N)"
DIST_ROOT="$ROOT_DIR/dist"
WEB_ROOT="$DIST_ROOT/web"
WEB_DIR="$ROOT_DIR/view/web"
OUT_DIR="$DIST_ROOT/$STAMP"

echo ">> workspace : $ROOT_DIR"
echo ">> frontend  : $WEB_DIR"
echo ">> output    : $OUT_DIR"

mkdir -p "$OUT_DIR"

if ! command -v npm >/dev/null 2>&1; then
  printf 'Error: npm is required to build the frontend.\n' >&2
  exit 1
fi

if [[ ! -f "$WEB_DIR/package.json" ]]; then
  printf 'Error: missing %s\n' "$WEB_DIR/package.json" >&2
  exit 1
fi

echo ">> building React + TypeScript frontend..."
npm --prefix "$WEB_DIR" run build

if [[ ! -f "$WEB_ROOT/index.html" ]]; then
  printf 'Error: %s is missing. Vite must emit the frontend before building.\n' "$WEB_ROOT/index.html" >&2
  exit 1
fi

echo ">> checking workspace..."
cargo check --workspace --manifest-path "$ROOT_DIR/Cargo.toml"

echo ">> building release binary..."
cargo build --release --manifest-path "$ROOT_DIR/view/desktop/Cargo.toml"

BIN_SRC="$ROOT_DIR/target/release/app"
if [[ ! -f "$BIN_SRC" ]]; then
  printf 'Error: expected binary at %s\n' "$BIN_SRC" >&2
  exit 1
fi

cp "$BIN_SRC" "$OUT_DIR/xkdPlanner"

for entry in "$WEB_ROOT"/* "$WEB_ROOT"/.[!.]*; do
  [[ -e "$entry" ]] || continue
  cp -R "$entry" "$OUT_DIR/"
done

cat > "$OUT_DIR/BUILD_INFO.txt" <<EOF
xkdPlanner build
unix_ms : $STAMP
built   : $(date -u +%Y-%m-%dT%H:%M:%SZ)
binary  : xkdPlanner
frontend: Vite + React + TypeScript
node    : $(node --version 2>/dev/null || echo "unknown")
npm     : $(npm --version 2>/dev/null || echo "unknown")
rust    : $(rustc --version 2>/dev/null || echo "unknown")
EOF

echo ">> done: $OUT_DIR/xkdPlanner"
