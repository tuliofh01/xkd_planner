#!/usr/bin/env bash
set -euo pipefail

usage() {
  printf 'Usage: %s <crate-name>\n\n' "$0"
  printf 'Scaffolds a library crate at the repository root and prints import instructions.\n'
}

if [[ $# -ne 1 || "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  [[ $# -eq 1 ]] && exit 0 || exit 1
fi

CRATE_NAME="$1"

if [[ ! "$CRATE_NAME" =~ ^[a-z][a-z0-9_-]*$ ]]; then
  printf 'Error: invalid crate name %q (use lowercase letters, digits, - or _)\n' "$CRATE_NAME" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CRATE_DIR="$ROOT_DIR/$CRATE_NAME"

if [[ -e "$CRATE_DIR" ]]; then
  printf 'Error: path already exists at %s\n' "$CRATE_DIR" >&2
  exit 1
fi

mkdir -p "$CRATE_DIR/src"

cat > "$CRATE_DIR/Cargo.toml" <<EOF
[package]
name = "$CRATE_NAME"
version.workspace = true
edition.workspace = true

[dependencies]
EOF

: > "$CRATE_DIR/src/lib.rs"

printf 'Scaffolded library crate at %s\n\n' "$CRATE_DIR"
printf 'Next steps:\n'
printf '  1. Add "%s" to the root Cargo.toml workspace members.\n' "$CRATE_NAME"
printf '  2. Add this to any dependent crate Cargo.toml:\n\n'
printf '       %s = { path = "../%s" }\n\n' "$CRATE_NAME" "$CRATE_NAME"
printf '  3. Export items from %s/src/lib.rs, then import them with:\n\n' "$CRATE_NAME"
printf '       use %s::<public_item>;\n' "$CRATE_NAME"
