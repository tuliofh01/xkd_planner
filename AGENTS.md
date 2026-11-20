# AGENTS.md

## Project snapshot
- `xkdPlanner` is a Rust + Tauri desktop app with a React/TypeScript frontend.
- The project is mid-refactor into an MVC-oriented workspace. Most backend/frontend logic is still skeleton code with `todo!()` bodies.
- Canonical layout: `model/` (data), `controller/` (handlers/services), `view/desktop/` (Tauri), `view/web/` (Vite/React), `scripts/`, `docs/`.

## Current state (2026-11-20)
- Reorg into Option C layout is in place and staged in git; no commit created.
- Path repairs completed: root `Cargo.toml`, `controller/Cargo.toml`, `view/desktop/Cargo.toml`, `view/desktop/tauri.conf.json`, `view/web/vite.config.ts`, `view/web/tsconfig.json`, `scripts/build_app.sh`, `scripts/init_crate.sh`.
- Added: `scripts/run_app.sh`, `view/desktop/.taurignore`, `.env.example`, expanded `.gitignore`, `.zed/*`, docs/snippets, frontend lint/format config.
- Local `.env` no longer contains `GH_TOKEN`; it should stay untracked.
- Crate package names remain `models`, `controllers`, `app` for now to reduce churn; folders are `model/`, `controller/`.

## Commands
- Frontend install: `npm --prefix view/web install`
- Frontend dev: `npm --prefix view/web run dev`
- Frontend build: `npm --prefix view/web run build`
- Frontend lint: `npm --prefix view/web run lint`
- Frontend format: `npm --prefix view/web run format`
- Workspace check: `cargo check --workspace`
- Workspace tests: `cargo test --workspace`
- Workspace lint: `cargo clippy --workspace --all-targets -- -D warnings` (after stubs/build are healthy)
- Release artifact: `./scripts/build_app.sh`
- Dev run/logs: `./scripts/run_app.sh`

## Notes
- Tauri command bodies and service implementations are mostly `todo!()` and will panic at runtime until implemented.
- `view/desktop/src/lib.rs` currently hardcodes `sqlite:xkdplanner.db`; wiring `DATABASE_URL`/dotenv is a follow-up improvement.
- `cargo check --workspace` and frontend lint may require dependency installation/tooling setup in this environment.
- Do not commit unless explicitly requested. Prefer a checkpoint commit only after validation is green.